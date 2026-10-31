//! The game run as game.wasm for the app (`opence_ffi::game`): its render
//! batches handed to the app's renderer ([`RenderTarget`]) as frames, each
//! its commands and the bytes its uploads name, one after another; the
//! textures the renderer reads back written where the game asked.

use std::sync::{Arc, Mutex, PoisonError};

use opence_ffi::game::{
    render_sink, GameEnd, RenderFrame, RenderReadback, RenderTarget, WasmGame, WasmGameError,
    WasmGameOptions,
};
use opence_render::{op, Command, PixelFormat, TextureKind, MAGIC, USAGE_RENDER_TARGET, VERSION};
use opence_runtime::{Error, Runtime, Val};

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

/// a module that submits the batch it is given
const DEVICE: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "submit") (param i32 i32) (result i32) (call $submit (local.get 0) (local.get 1))))
"#;

/// A renderer that keeps each frame and answers with `readbacks`.
#[derive(Default)]
struct Target {
    frames: Mutex<Vec<RenderFrame>>,
    readbacks: Vec<RenderReadback>,
}

impl RenderTarget for Target {
    fn submit(&self, frame: RenderFrame) -> Vec<RenderReadback> {
        self.frames
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(frame);
        self.readbacks.clone()
    }
}

fn frames(target: &Target) -> Vec<RenderFrame> {
    std::mem::take(&mut *target.frames.lock().unwrap_or_else(PoisonError::into_inner))
}

fn submit(game: &mut opence_runtime::Game, bytes: &[u8]) -> Result<Option<i32>, Error> {
    game.write(u64::from(BATCH), bytes)?;
    let size = i32::try_from(bytes.len()).unwrap_or(0);
    let results = game.call_with("submit", &[Val::I32(BATCH.cast_signed()), Val::I32(size)])?;
    Ok(results.first().and_then(Val::i32))
}

#[test]
fn a_frame_is_the_batch_s_commands_and_the_bytes_its_uploads_name() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&wat::parse_str(DEVICE).unwrap_or_default())?;
    let target = Arc::new(Target::default());
    game.set_render_sink(render_sink(Arc::clone(&target) as Arc<dyn RenderTarget>));
    game.write(0x800, b"abcdefgh")?;
    game.write(0x900, b"xy")?;
    let bytes = batch(&[
        (op::CREATE_BUFFER, &[1, 8]),
        (op::UPLOAD_BUFFER, &[1, 0, 0x800, 8]),
        (op::WRITE_BUFFER, &[1, 2, 0x900, 2]),
    ]);
    assert_eq!(submit(&mut game, &bytes)?, Some(0));
    let frames = frames(&target);
    let [frame] = frames.as_slice() else {
        panic!("the renderer was handed {} frames", frames.len());
    };
    assert_eq!(frame.commands.len(), 3);
    assert_eq!(
        frame.commands.get(1),
        Some(&Command::UploadBuffer {
            id: 1,
            offset: 0,
            address: 0x800,
            length: 8
        })
    );
    assert_eq!(frame.data, b"abcdefghxy");
    Ok(())
}

#[test]
fn a_texture_read_back_is_written_where_its_command_names() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&wat::parse_str(DEVICE).unwrap_or_default())?;
    let target = Arc::new(Target {
        frames: Mutex::default(),
        readbacks: vec![
            RenderReadback {
                address: 0xa00,
                bytes: vec![1, 2, 3, 4],
            },
            // (no command reads back here: refused)
            RenderReadback {
                address: 0xb00,
                bytes: vec![9],
            },
        ],
    });
    game.set_render_sink(render_sink(Arc::clone(&target) as Arc<dyn RenderTarget>));
    let plane = TextureKind::Plane.word();
    let rgba8 = PixelFormat::Rgba8.word();
    let bytes = batch(&[
        (
            op::CREATE_TEXTURE,
            &[1, plane, rgba8, 1, 1, 1, 1, USAGE_RENDER_TARGET, 1],
        ),
        (op::READ_TEXTURE, &[1, 0, 0, 0, 0, 1, 1, 0xa00, 4, 4]),
    ]);
    assert_eq!(submit(&mut game, &bytes)?, Some(0));
    assert_eq!(game.read(0xa00, 4)?, vec![1, 2, 3, 4]);
    assert_eq!(game.read(0xb00, 1)?, vec![0]);
    assert_eq!(game.render_refusals().len(), 1);
    Ok(())
}

/// a game whose main submits one batch, already in its memory, and returns
const MAIN: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0x100) "BATCH")
  (func (export "halo_game_main") (drop (call $submit (i32.const 0x100) (i32.const SIZE)))))
"#;

#[test]
fn the_game_runs_on_its_own_thread_and_draws_to_the_target() {
    let bytes = batch(&[(op::CREATE_BUFFER, &[1, 64])]);
    let escaped: String = bytes.iter().map(|byte| format!("\\{byte:02x}")).collect();
    let text = MAIN
        .replace("BATCH", &escaped)
        .replace("SIZE", &bytes.len().to_string());
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("wasm_game");
    let _ = std::fs::create_dir_all(folder.join("saves"));
    let wasm = folder.join("game.wasm");
    let _ = std::fs::write(&wasm, wat::parse_str(&text).unwrap_or_default());
    let display = |path: &std::path::Path| path.display().to_string();
    let game = WasmGame::new(WasmGameOptions {
        wasm: display(&wasm),
        data: display(&folder),
        saves: display(&folder.join("saves")),
        cache: None,
        width: 1280,
        height: 720,
        map: None,
        seconds: None,
        settings: Vec::new(),
        audit: None,
    });
    let game = match game {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    let target = Arc::new(Target::default());
    assert!(game
        .start(Arc::clone(&target) as Arc<dyn RenderTarget>)
        .is_ok());
    // a second start is refused: the game is running
    assert!(game
        .start(Arc::clone(&target) as Arc<dyn RenderTarget>)
        .is_err());
    let end = game.wait();
    assert_eq!(end.error, None);
    assert_eq!((end.batches, end.commands, end.refused), (1, 1, 0));
    assert_eq!(
        frames(&target)
            .iter()
            .map(|frame| frame.commands.clone())
            .collect::<Vec<_>>(),
        [vec![Command::CreateBuffer { id: 1, length: 64 }]]
    );
}

/// a game whose main never returns (it submits nothing, over and over)
const FOREVER: &str = r#"
(module
  (import "env" "host_render_submit" (func $submit (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "halo_game_main") (loop $again (drop (call $submit (i32.const 0) (i32.const 0))) (br $again))))
"#;

#[test]
fn a_game_given_seconds_ends_when_they_are_up() {
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("wasm_game_seconds");
    let _ = std::fs::create_dir_all(folder.join("saves"));
    let wasm = folder.join("game.wasm");
    let _ = std::fs::write(&wasm, wat::parse_str(FOREVER).unwrap_or_default());
    let display = |path: &std::path::Path| path.display().to_string();
    let game = match WasmGame::new(WasmGameOptions {
        wasm: display(&wasm),
        data: display(&folder),
        saves: display(&folder.join("saves")),
        cache: None,
        width: 640,
        height: 480,
        map: None,
        seconds: Some(1),
        settings: Vec::new(),
        audit: None,
    }) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    let started = std::time::Instant::now();
    assert!(game
        .start(Arc::new(Target::default()) as Arc<dyn RenderTarget>)
        .is_ok());
    let end = game.wait();
    // (its time running out is how it was meant to end, not an error)
    assert_eq!(end.error, None);
    assert!(end.timed_out);
    assert!(started.elapsed() < std::time::Duration::from_secs(10));
}

/// options for the never-ending game in `folder`, with `settings`
fn forever(folder: &str, settings: &[&str]) -> WasmGameOptions {
    let folder = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(folder);
    let _ = std::fs::create_dir_all(folder.join("saves"));
    let wasm = folder.join("game.wasm");
    let _ = std::fs::write(&wasm, wat::parse_str(FOREVER).unwrap_or_default());
    let display = |path: &std::path::Path| path.display().to_string();
    WasmGameOptions {
        wasm: display(&wasm),
        data: display(&folder),
        saves: display(&folder.join("saves")),
        cache: None,
        width: 640,
        height: 480,
        map: None,
        seconds: Some(1),
        settings: settings.iter().map(|&setting| setting.to_owned()).collect(),
        audit: None,
    }
}

#[test]
fn the_game_is_given_the_settings_it_names() {
    assert!(WasmGame::new(forever("wasm_game_settings", &["debug.frame_clock=true"])).is_ok());
    // a setting the game does not have, or one not written name=value, is refused
    for wrong in ["debug.no_such_setting=1", "debug.frame_clock"] {
        assert!(matches!(
            WasmGame::new(forever("wasm_game_settings", &[wrong])),
            Err(WasmGameError::Load { .. })
        ));
    }
}

#[test]
fn a_game_without_its_console_answers_nothing() {
    let game = match WasmGame::new(forever("wasm_game_console", &[])) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    assert_eq!(game.ask("debug_camera_load".to_owned(), 100), None);
}

#[test]
fn the_game_s_control_socket_hands_its_lines_to_the_console() {
    use std::io::{BufRead, BufReader, Write};
    let game = match WasmGame::new(forever("wasm_game_control", &[])) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    // (a short path: a socket's is at most 104 bytes)
    let path = std::env::temp_dir().join(format!("opence-wasm-{}.sock", std::process::id()));
    assert_eq!(game.listen(path.display().to_string()), Ok(()));
    let reply = (|| -> Option<String> {
        let mut stream = std::os::unix::net::UnixStream::connect(&path).ok()?;
        stream.write_all(b"\"game_speed 2\"\n").ok()?;
        let mut reply = String::new();
        BufReader::new(stream).read_line(&mut reply).ok()?;
        Some(reply)
    })();
    let _ = std::fs::remove_file(&path);
    // (this game has no console, so the line is refused as the shell shows it)
    assert_eq!(
        reply.as_deref(),
        Some("{\"error\":\"the game's console is not listening yet\",\"ok\":false}\n")
    );
    // a path that cannot be bound is the caller's error
    assert!(matches!(
        game.listen("/no/such/folder/opence.sock".to_owned()),
        Err(WasmGameError::Control { .. })
    ));
}

/// a game whose main ends three ticks (the section a tick ends with)
const TICKING: &str = r#"
(module
  (import "env" "halo_profile_exit" (func $exit (param i32)))
  (memory (export "memory") 1)
  (data (i32.const 0x100) "game_tick.player_effect\00")
  (func (export "halo_game_main")
    (call $exit (i32.const 0x100)) (call $exit (i32.const 0x100)) (call $exit (i32.const 0x100))))
"#;

#[test]
fn the_game_s_ticks_are_counted_from_the_first() {
    // (the never-ending game's folder, its game.wasm replaced by the ticking one)
    let options = forever("wasm_game_ticks", &[]);
    let _ = std::fs::write(&options.wasm, wat::parse_str(TICKING).unwrap_or_default());
    let game = match WasmGame::new(options) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    // (loading, or at the menus, the game has not ticked)
    assert_eq!(game.ticks(), 0);
    assert!(game
        .start(Arc::new(Target::default()) as Arc<dyn RenderTarget>)
        .is_ok());
    let end = game.wait();
    assert_eq!(end.error, None);
    assert!(!end.timed_out);
    assert_eq!(game.ticks(), 3);
}

/// a game whose main runs three frames, each its input and a tick
const FRAMES: &str = r#"
(module
  (import "env" "halo_profile_enter" (func $enter (param i32)))
  (import "env" "halo_profile_exit" (func $exit (param i32)))
  (memory (export "memory") 1)
  (data (i32.const 0x100) "input\00")
  (data (i32.const 0x110) "game_tick.cheats_enforce\00")
  (data (i32.const 0x130) "game_tick.player_effect\00")
  (func $section (param i32) (call $enter (local.get 0)) (call $exit (local.get 0)))
  (func $frame (call $section (i32.const 0x100)) (call $section (i32.const 0x110)) (call $section (i32.const 0x130)))
  (func (export "halo_game_main") (call $frame) (call $frame) (call $frame)))
"#;

/// the frames game run to its end, profiled or not
fn frames_run(folder: &str, profiled: bool) -> GameEnd {
    let options = forever(folder, &[]);
    let _ = std::fs::write(&options.wasm, wat::parse_str(FRAMES).unwrap_or_default());
    let game = match WasmGame::new(options) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    if profiled {
        assert!(game.profile());
    }
    assert!(game
        .start(Arc::new(Target::default()) as Arc<dyn RenderTarget>)
        .is_ok());
    // (too late once it has started)
    assert!(!game.profile());
    game.wait()
}

#[test]
fn a_profiled_game_s_frames_are_timed_from_its_first_tick() {
    let end = frames_run("wasm_game_frames", true);
    assert_eq!(end.error, None);
    // (three inputs: two frames, each to the next input)
    assert_eq!(end.frame_ms.len(), 2);
    assert!(end.frame_ms.iter().all(|ms| (0.0..1000.0).contains(ms)));
    assert!(frames_run("wasm_game_unprofiled", false)
        .frame_ms
        .is_empty());
}

/// a game whose player picks Quit at once
const QUITS: &str = r#"
(module
  (import "env" "platform_request_quit" (func $quit))
  (memory (export "memory") 1)
  (func (export "halo_game_main") (call $quit) (unreachable)))
"#;

#[test]
fn quitting_from_the_menu_ends_the_game_without_an_error() {
    let options = forever("wasm_game_quits", &[]);
    let _ = std::fs::write(&options.wasm, wat::parse_str(QUITS).unwrap_or_default());
    let game = match WasmGame::new(options) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    assert!(game
        .start(Arc::new(Target::default()) as Arc<dyn RenderTarget>)
        .is_ok());
    let end = game.wait();
    assert_eq!(end.error, None);
    assert!(!end.timed_out);
}

/// A renderer that notes whether the game made its text atlas (text_tags.c's
/// 2048 by 2048 glyph texture) and drew from it.
#[derive(Default)]
struct AtlasWatch {
    atlas: Mutex<Option<u32>>,
}

impl RenderTarget for AtlasWatch {
    fn submit(&self, frame: RenderFrame) -> Vec<RenderReadback> {
        for command in &frame.commands {
            if let Command::CreateTexture {
                id,
                format: PixelFormat::Rgba8,
                width: 2048,
                height: 2048,
                render_target: false,
                ..
            } = *command
            {
                *self.atlas.lock().unwrap_or_else(PoisonError::into_inner) = Some(id);
            }
        }
        Vec::new()
    }
}

/// whether the app's game made the text atlas on the main menu at 4K, with
/// `settings`; None (skipped) without game.wasm and ui.map
fn menu_atlas_at_4k(settings: &[&str]) -> Option<bool> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm = root.join("build/engine/wasm/game.wasm");
    if !wasm.is_file() || !root.join("assets/maps/ui.map").is_file() {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return None;
    }
    let saves = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("wasm_game_atlas");
    let _ = std::fs::create_dir_all(&saves);
    let display = |path: &std::path::Path| path.display().to_string();
    let game = match WasmGame::new(WasmGameOptions {
        wasm: display(&wasm),
        data: display(&root.join("assets")),
        saves: display(&saves),
        cache: Some(display(&root.join("build/engine/wasm/cache"))),
        width: 3840,
        height: 2160,
        map: Some("ui".to_owned()),
        seconds: Some(20),
        settings: settings.iter().map(|&setting| setting.to_owned()).collect(),
        audit: Some(display(&saves.join("audit"))),
    }) {
        Ok(game) => game,
        Err(error) => panic!("{error}"),
    };
    let watch = Arc::new(AtlasWatch::default());
    assert!(game
        .start(Arc::clone(&watch) as Arc<dyn RenderTarget>)
        .is_ok());
    let end = game.wait();
    assert_eq!(end.error, None);
    let made = watch
        .atlas
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .is_some();
    Some(made)
}

#[test]
fn the_app_s_main_menu_draws_its_text_from_the_atlas_unless_the_setting_is_off() {
    // (the game's own text: the Halo UI plugin's redraw, which draws the
    // menus in its place, switched off)
    let Some(on) = menu_atlas_at_4k(&["display.redraw_menus=false"]) else {
        return;
    };
    assert!(on, "the main menu made no text atlas at 4K");
    assert_eq!(
        menu_atlas_at_4k(&["display.redraw_menus=false", "display.high_res_text=false"]),
        Some(false)
    );
}
