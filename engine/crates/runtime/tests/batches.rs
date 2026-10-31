//! The Metal device's render batches (`opence_runtime::batches`): the
//! game's device (`port/wasmtime/metal/d3d8_metal_device.c`, compiled into
//! game.wasm) writes each batch into its memory and rings
//! `host_render_submit`; the platform copies it out, checks it
//! (`opence_render`), hands what passed to the app's renderer with a
//! read-only view of the memory its uploads name, and returns how many of
//! its commands were refused, or -1 for a batch refused whole. The device
//! asks the drawable's size with `host_drawable_width` and
//! `host_drawable_height`.

use std::sync::{Arc, Mutex, PoisonError};

use opence_render::{op, Command, DecodedBatch, MAGIC, VERSION};
use opence_runtime::plugins::View;
use opence_runtime::{answers, payload, readback_fits, Error, Readback, RenderSink, Runtime, Val};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// the module: the three imports re-exported as functions the test calls
const DEVICE: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (import "env" "host_drawable_width" (func $width (result i32)))
  (import "env" "host_drawable_height" (func $height (result i32)))
  (memory (export "memory") 1)
  (func (export "submit") (param i32 i32) (result i32) (call $submit (local.get 0) (local.get 1)))
  (func (export "width") (result i32) (call $width))
  (func (export "height") (result i32) (call $height)))
"#;

/// where the test writes its batches
const BATCH: u32 = 0x100;

/// a batch of `commands`, each its op and fields, with its header
fn batch(commands: &[(u32, &[u32])]) -> Vec<u8> {
    let mut words = vec![MAGIC, VERSION, 0, 0];
    for (code, fields) in commands {
        let length = u32::try_from(fields.len() + 1).unwrap_or(0);
        words.push(code | (length << 16));
        words.extend_from_slice(fields);
    }
    let size = u32::try_from(words.len() * 4).unwrap_or(0);
    words[2] = size;
    words[3] = u32::try_from(commands.len()).unwrap_or(0);
    words.iter().flat_map(|word| word.to_le_bytes()).collect()
}

/// what the sink was handed: each batch, and the first word of the memory
/// at [`BATCH`] as its view read it
type Handed = Arc<Mutex<Vec<(DecodedBatch, Option<u32>)>>>;

struct Sink(Handed);

impl RenderSink for Sink {
    fn submit(&mut self, batch: &DecodedBatch, memory: &View<'_>) -> Vec<Readback> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push((batch.clone(), memory.u32(BATCH)));
        Vec::new()
    }
}

/// a sink that answers every batch with `readbacks`
struct Answers(Vec<Readback>);

impl RenderSink for Answers {
    fn submit(&mut self, _: &DecodedBatch, _: &View<'_>) -> Vec<Readback> {
        self.0.clone()
    }
}

/// a texture read back: `length` bytes to `address`
fn read_texture(address: u32, length: u32) -> Command {
    Command::ReadTexture {
        id: 1,
        level: 0,
        slice: 0,
        x: 0,
        y: 0,
        width: 1,
        height: 1,
        address,
        bytes_per_row: length,
        length,
    }
}

fn first(results: &[Val]) -> Option<i32> {
    results.first().and_then(Val::i32)
}

fn submit(game: &mut opence_runtime::Game, bytes: &[u8]) -> Result<Option<i32>, Error> {
    game.write(u64::from(BATCH), bytes)?;
    let size = i32::try_from(bytes.len()).unwrap_or(0);
    Ok(first(&game.call_with(
        "submit",
        &[Val::I32(BATCH.cast_signed()), Val::I32(size)],
    )?))
}

#[test]
fn a_batch_is_checked_and_what_passed_handed_to_the_sink() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    let handed: Handed = Arc::default();
    game.set_render_sink(Box::new(Sink(Arc::clone(&handed))));
    // a buffer made, and a pass ended that was never begun
    let bytes = batch(&[(op::CREATE_BUFFER, &[1, 64]), (op::END_PASS, &[])]);
    assert_eq!(submit(&mut game, &bytes)?, Some(1));
    let handed = std::mem::take(&mut *handed.lock().unwrap_or_else(PoisonError::into_inner));
    let [(batch, word)] = handed.as_slice() else {
        panic!("the sink was handed {} batches", handed.len());
    };
    assert_eq!(
        batch.commands,
        [Command::CreateBuffer { id: 1, length: 64 }]
    );
    assert_eq!(batch.rejected.len(), 1);
    assert_eq!(
        batch.rejected.first().map(|refused| refused.op),
        Some(op::END_PASS)
    );
    // the view is the game's memory
    assert_eq!(*word, Some(MAGIC));
    let counts = game.render_counts();
    assert_eq!((counts.batches, counts.commands, counts.refused), (1, 1, 1));
    Ok(())
}

#[test]
fn a_batch_that_is_not_one_is_refused_whole() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    let handed: Handed = Arc::default();
    game.set_render_sink(Box::new(Sink(Arc::clone(&handed))));
    let mut bytes = batch(&[(op::CREATE_BUFFER, &[1, 64])]);
    // not OCRB
    bytes[0] ^= 0xff;
    assert_eq!(submit(&mut game, &bytes)?, Some(-1));
    // a batch that runs past the end of the memory
    let past = game.call_with("submit", &[Val::I32(0xfff0), Val::I32(64)])?;
    assert_eq!(first(&past), Some(-1));
    // six bytes are not whole words
    assert_eq!(submit(&mut game, &bytes[..6])?, Some(-1));
    assert!(handed
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .is_empty());
    let counts = game.render_counts();
    assert_eq!((counts.batches, counts.refused_batches), (0, 3));
    assert_eq!(game.render_refusals().len(), 3);
    Ok(())
}

#[test]
fn without_a_sink_a_batch_is_checked_and_dropped() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    let bytes = batch(&[(op::CREATE_BUFFER, &[1, 64]), (op::END_PASS, &[])]);
    assert_eq!(submit(&mut game, &bytes)?, Some(1));
    assert_eq!(game.render_counts().batches, 1);
    Ok(())
}

#[test]
fn the_validator_keeps_the_objects_of_earlier_batches() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    assert_eq!(
        submit(&mut game, &batch(&[(op::CREATE_BUFFER, &[1, 64])]))?,
        Some(0)
    );
    // the same buffer made again is refused: the first is still there
    assert_eq!(
        submit(&mut game, &batch(&[(op::CREATE_BUFFER, &[1, 64])]))?,
        Some(1)
    );
    Ok(())
}

#[test]
fn the_drawable_is_the_xbox_s_until_the_app_sets_it() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    assert_eq!(first(&game.call_with("width", &[])?), Some(640));
    assert_eq!(first(&game.call_with("height", &[])?), Some(480));
    game.set_drawable_size(3840, 2160);
    assert_eq!(first(&game.call_with("width", &[])?), Some(3840));
    assert_eq!(first(&game.call_with("height", &[])?), Some(2160));
    Ok(())
}

#[test]
fn a_resize_from_the_app_s_thread_reaches_the_game() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    let controls = game.controls();
    std::thread::spawn(move || controls.set_drawable_size(2560, 1600))
        .join()
        .ok();
    assert_eq!(first(&game.call_with("width", &[])?), Some(2560));
    assert_eq!(first(&game.call_with("height", &[])?), Some(1600));
    Ok(())
}

#[test]
fn the_platform_answers_the_device_s_imports() {
    for name in [
        "env.host_render_submit",
        "env.host_drawable_width",
        "env.host_drawable_height",
    ] {
        assert!(answers(name), "{name}");
    }
}

#[test]
fn the_payload_is_the_bytes_each_reading_command_names_in_order() {
    let mut memory: Vec<u8> = (0..=255).collect();
    let batch = DecodedBatch {
        commands: vec![
            Command::CreateBuffer { id: 1, length: 4 },
            Command::UploadBuffer {
                id: 1,
                offset: 0,
                address: 0x10,
                length: 3,
            },
            Command::CreateLibrary {
                id: 2,
                address: 0x40,
                length: 2,
            },
            // (read back by the renderer, so not in the payload)
            read_texture(0x80, 4),
            Command::WriteBuffer {
                id: 1,
                offset: 1,
                address: 0x20,
                length: 1,
            },
        ],
        rejected: Vec::new(),
    };
    let bytes = View::over(&mut memory, |view| payload(&batch, view));
    assert_eq!(bytes, Some(vec![0x10, 0x11, 0x12, 0x40, 0x41, 0x20]));
    // a command naming bytes outside the memory: no payload at all
    let outside = DecodedBatch {
        commands: vec![Command::CreateLibrary {
            id: 2,
            address: 0xff,
            length: 2,
        }],
        rejected: Vec::new(),
    };
    assert_eq!(
        View::over(&mut memory, |view| payload(&outside, view)),
        None
    );
}

#[test]
fn a_readback_fits_only_the_range_a_read_texture_names() {
    let batch = DecodedBatch {
        commands: vec![read_texture(0x80, 16)],
        rejected: Vec::new(),
    };
    let fits = |address, length| {
        readback_fits(
            &batch,
            &Readback {
                address,
                bytes: vec![0; length],
            },
        )
    };
    assert!(fits(0x80, 16));
    assert!(fits(0x84, 4));
    assert!(!fits(0x80, 17));
    assert!(!fits(0x7f, 1));
    assert!(!fits(0x90, 1));
    assert!(!fits(u32::MAX, 2));
}

#[test]
fn a_readback_is_written_into_the_memory_or_refused() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    game.set_render_sink(Box::new(Answers(vec![
        Readback {
            address: 0x800,
            bytes: vec![1, 2, 3, 4],
        },
        // (no command read back to 0x900)
        Readback {
            address: 0x900,
            bytes: vec![5],
        },
    ])));
    // the renderer's answers are checked against the batch the validator
    // passed, so a batch whose ReadTexture passed is what the sink sees:
    // here no command passed, and both are refused
    assert_eq!(submit(&mut game, &batch(&[(op::END_PASS, &[])]))?, Some(1));
    assert_eq!(game.read(0x800, 4)?, vec![0; 4]);
    assert_eq!(game.read(0x900, 1)?, vec![0]);
    let refusals = game.render_refusals();
    assert_eq!(
        refusals
            .iter()
            .filter(|reason| reason.contains("read back"))
            .count(),
        2,
        "{refusals:?}"
    );
    Ok(())
}

#[test]
fn a_texture_read_back_is_written_where_its_command_names() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    game.set_render_sink(Box::new(Answers(vec![Readback {
        address: 0x800,
        bytes: vec![1, 2, 3, 4],
    }])));
    // a 1 by 1 RGBA8 render target, read back to 0x800
    let plane = opence_render::TextureKind::Plane.word();
    let rgba8 = opence_render::PixelFormat::Rgba8.word();
    let target = opence_render::USAGE_RENDER_TARGET;
    let bytes = batch(&[
        (
            op::CREATE_TEXTURE,
            &[1, plane, rgba8, 1, 1, 1, 1, target, 1],
        ),
        (op::READ_TEXTURE, &[1, 0, 0, 0, 0, 1, 1, 0x800, 4, 4]),
    ]);
    assert_eq!(submit(&mut game, &bytes)?, Some(0));
    assert_eq!(game.read(0x800, 4)?, vec![1, 2, 3, 4]);
    Ok(())
}

#[test]
fn a_buffer_read_back_is_written_where_its_command_names() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(DEVICE))?;
    game.set_render_sink(Box::new(Answers(vec![Readback {
        address: 0x800,
        bytes: vec![9, 8, 7, 6, 5, 4, 3, 2],
    }])));
    // (a visibility buffer's first count, read back to 0x800)
    let bytes = batch(&[
        (op::CREATE_BUFFER, &[1, 64]),
        (op::READ_BUFFER, &[1, 0, 0x800, 8]),
    ]);
    assert_eq!(submit(&mut game, &bytes)?, Some(0));
    assert_eq!(game.read(0x800, 8)?, vec![9, 8, 7, 6, 5, 4, 3, 2]);
    Ok(())
}

/// a device that also reads the game's clock (QueryPerformanceCounter, in
/// nanoseconds here) and the monotonic one
const CLOCKED: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (import "env" "QueryPerformanceCounter" (func $counter (param i32) (result i32)))
  (import "wasi_snapshot_preview1" "clock_time_get" (func $clock (param i32 i64 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "submit") (param i32 i32) (result i32) (call $submit (local.get 0) (local.get 1)))
  (func (export "clock") (result i64) (drop (call $counter (i32.const 0x80))) (i64.mul (i64.load (i32.const 0x80)) (i64.const 100)))
  (func (export "monotonic") (result i64) (drop (call $clock (i32.const 1) (i64.const 1) (i32.const 0x80))) (i64.load (i32.const 0x80))))
"#;

fn read_clock(game: &mut opence_runtime::Game, name: &str) -> Result<u64, Error> {
    let results = game.call_with(name, &[])?;
    Ok(match results.first() {
        Some(Val::I64(value)) => value.cast_unsigned(),
        _ => 0,
    })
}

fn clock(game: &mut opence_runtime::Game) -> Result<u64, Error> {
    read_clock(game, "clock")
}

/// `a - b` within the counter's 100 ns
fn near(a: u64, b: u64) -> bool {
    a.abs_diff(b) < 100
}

/// a 60th of a second, as the frame clock steps
const STEP: u64 = 16_666_667;

#[test]
fn with_the_frame_clock_each_frame_shown_is_a_60th_of_a_second() -> Result<(), Error> {
    let mut game = Runtime::new(None)?.load(&module(CLOCKED))?;
    assert!(game.set_setting("debug.frame_clock", "true"));
    let present = batch(&[(op::PRESENT, &[])]);
    // (the first frame shown starts the clock)
    submit(&mut game, &present)?;
    let start = clock(&mut game)?;
    for _ in 0..30 {
        submit(&mut game, &present)?;
    }
    assert!(near(clock(&mut game)? - start, 30 * STEP));
    // a batch that shows nothing does not step it
    submit(&mut game, &batch(&[(op::CREATE_BUFFER, &[1, 64])]))?;
    assert!(near(clock(&mut game)? - start, 30 * STEP));
    // in the dark for more than half a second, it steps with the real time
    // (bounded by the time measured, as a loaded machine sleeps longer)
    let before = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(700));
    let dark = clock(&mut game)? - start - 30 * STEP;
    let slept = u64::try_from(before.elapsed().as_nanos()).unwrap_or(u64::MAX);
    assert!(
        dark >= 5 * STEP && dark <= slept + STEP,
        "{dark} after {slept}"
    );
    Ok(())
}

#[test]
fn with_the_frame_clock_the_monotonic_clock_is_still_the_real_time() -> Result<(), Error> {
    // (the vertical blanks are paced by it: were they paced by the frame
    // clock, which steps only when a frame is shown, a game that waits for
    // a blank before it shows its frame would wait for the dark steps)
    let mut game = Runtime::new(None)?.load(&module(CLOCKED))?;
    assert!(game.set_setting("debug.frame_clock", "true"));
    let present = batch(&[(op::PRESENT, &[])]);
    submit(&mut game, &present)?;
    let start = read_clock(&mut game, "monotonic")?;
    let before = std::time::Instant::now();
    std::thread::sleep(std::time::Duration::from_millis(50));
    let moved = read_clock(&mut game, "monotonic")? - start;
    let slept = u64::try_from(before.elapsed().as_nanos()).unwrap_or(u64::MAX);
    assert!(
        moved >= 50_000_000 && moved <= slept + 1_000_000,
        "{moved} after {slept}"
    );
    Ok(())
}

#[test]
fn without_the_frame_clock_the_clock_is_the_real_time() -> Result<(), Error> {
    let mut game = Runtime::new(None)?.load(&module(CLOCKED))?;
    let present = batch(&[(op::PRESENT, &[])]);
    submit(&mut game, &present)?;
    let start = clock(&mut game)?;
    for _ in 0..30 {
        submit(&mut game, &present)?;
    }
    assert!(clock(&mut game)? - start < 30 * STEP);
    Ok(())
}
