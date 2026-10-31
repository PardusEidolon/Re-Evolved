//! The runtime on small modules written here, and on game.wasm when it has
//! been built (tools/engine_wasm.py; the test skips itself without it).

use std::path::PathBuf;

use opence_runtime::{answers, Error, Runtime, Val, Value};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// a cache folder of the test's own, empty
fn cache(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("opence_runtime_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    folder
}

const CALLER: &str = r#"
(module
  (import "env" "halo_platform_tick" (func $tick (param i32 i64) (result i32)))
  (import "env" "platform_log" (func $log (param f32 f64)))
  (memory (export "memory") 1)
  (func (export "run") (result i32)
    (call $log (f32.const 1.5) (f64.const -2.25))
    (drop (call $tick (i32.const 7) (i64.const 9)))
    (call $tick (i32.const -1) (i64.const 0x100000000))))
"#;

#[test]
fn the_stubs_record_each_call_with_its_arguments_and_return_zero() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(CALLER))?;
    assert_eq!(game.call_i32("run")?, 0);
    let calls: Vec<_> = game
        .calls()
        .iter()
        .map(|call| (game.import_name(call.import), call.args.clone()))
        .collect();
    assert_eq!(
        calls,
        vec![
            (
                "env.platform_log".to_string(),
                vec![
                    Value::F32(1.5f32.to_bits()),
                    Value::F64((-2.25f64).to_bits())
                ]
            ),
            (
                "env.halo_platform_tick".to_string(),
                vec![Value::I32(7), Value::I64(9)]
            ),
            (
                "env.halo_platform_tick".to_string(),
                vec![Value::I32(-1), Value::I64(0x1_0000_0000)]
            ),
        ]
    );
    assert_eq!(game.call_count("env.halo_platform_tick"), 2);
    assert_eq!(game.call_count("env.platform_log"), 1);
    assert_eq!(game.call_count("env.missing"), 0);
    Ok(())
}

#[test]
fn the_record_keeps_a_bounded_number_of_calls_but_counts_them_all() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "env" "f" (func $f))
          (func (export "run") (local $i i32)
            (loop $again
              (call $f)
              (local.set $i (i32.add (local.get $i) (i32.const 1)))
              (br_if $again (i32.lt_u (local.get $i) (i32.const 5000))))))"#,
    ))?;
    game.set_record_limit(100);
    game.call("run")?;
    assert_eq!(game.calls().len(), 100);
    assert_eq!(game.call_count("env.f"), 5000);
    Ok(())
}

#[test]
fn a_call_budget_ends_a_game_that_never_returns() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "env" "f" (func $f))
          (func (export "run") (loop $again (call $f) (br $again))))"#,
    ))?;
    game.set_call_budget(Some(1000));
    assert!(matches!(game.call("run"), Err(Error::Budget(1000))));
    assert_eq!(game.call_count("env.f"), 1000);
    Ok(())
}

#[test]
fn a_deadline_ends_a_game_that_never_returns() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "env" "f" (func $f))
          (func (export "run") (loop $again (call $f) (br $again))))"#,
    ))?;
    let start = std::time::Instant::now();
    game.set_deadline(Some(start + std::time::Duration::from_millis(50)));
    assert!(matches!(game.call("run"), Err(Error::Deadline)));
    assert!(start.elapsed() >= std::time::Duration::from_millis(50));
    assert!(game.call_count("env.f") > 0);
    Ok(())
}

#[test]
fn proc_exit_ends_the_call_with_its_code() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
          (func (export "run") (call $exit (i32.const 3)) (unreachable)))"#,
    ))?;
    match game.call("run") {
        Err(Error::Exit(3)) => Ok(()),
        other => panic!("expected an exit with code 3, got {other:?}"),
    }
}

#[test]
fn a_trap_in_the_game_is_an_error_not_a_panic() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(r#"(module (func (export "run") (unreachable)))"#))?;
    assert!(matches!(game.call("run"), Err(Error::Trap(_))));
    Ok(())
}

#[test]
fn a_missing_export_and_a_malformed_module_are_errors() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module("(module)"))?;
    assert!(matches!(game.call("run"), Err(Error::NoExport(_))));
    assert!(matches!(
        runtime.load(b"\0asm\x01\0\0\0\x01\x10"),
        Err(Error::Compile(_))
    ));
    Ok(())
}

#[test]
fn imports_other_than_functions_are_refused() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let result = runtime.load(&module(r#"(module (import "env" "memory" (memory 1)))"#));
    assert!(matches!(result, Err(Error::Import(_))));
    Ok(())
}

#[test]
fn the_memory_is_read_and_written_with_bounds_checks() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module (memory (export "memory") 1)
             (func (export "run") (i32.store (i32.const 16) (i32.const 0x04030201))))"#,
    ))?;
    game.call("run")?;
    assert_eq!(game.memory_size(), 65536);
    assert_eq!(game.read(16, 4)?, vec![1, 2, 3, 4]);
    game.write(65532, &[9, 9, 9, 9])?;
    assert!(matches!(
        game.read(65533, 4),
        Err(Error::OutOfBounds { .. })
    ));
    assert!(matches!(
        game.write(u64::MAX, &[0]),
        Err(Error::OutOfBounds { .. })
    ));
    Ok(())
}

#[test]
fn the_compiled_module_is_cached_and_used_again() -> Result<(), Error> {
    let folder = cache("cached");
    let wasm = module(CALLER);
    let first = Runtime::new(Some(&folder))?;
    first.load(&wasm)?;
    let files = |folder: &PathBuf| -> usize {
        fn count(path: &std::path::Path) -> usize {
            std::fs::read_dir(path).map_or(0, |entries| {
                entries
                    .flatten()
                    .map(|entry| {
                        if entry.path().is_dir() {
                            count(&entry.path())
                        } else {
                            1
                        }
                    })
                    .sum()
            })
        }
        count(folder)
    };
    assert!(
        files(&folder) > 0,
        "nothing was cached in {}",
        folder.display()
    );
    let second = Runtime::new(Some(&folder))?;
    let mut game = second.load(&wasm)?;
    assert_eq!(game.call_i32("run")?, 0);
    assert!(second.cache_hits() >= 1, "the second load compiled again");
    let _ = std::fs::remove_dir_all(&folder);
    Ok(())
}

#[test]
fn wasmtime_keeps_to_a_folder_of_its_own_in_the_cache() -> Result<(), Error> {
    // (its cleanup deletes files it does not know, such as the bakes and
    // the binary archive the host keeps beside it)
    let folder = cache("own");
    Runtime::new(Some(&folder))?.load(&module(CALLER))?;
    let names: Vec<String> = std::fs::read_dir(&folder)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(names, [opence_runtime::COMPILED_FOLDER]);
    let _ = std::fs::remove_dir_all(&folder);
    Ok(())
}

/// game.wasm, where tools/engine_wasm.py builds it
fn game_wasm() -> Option<Vec<u8>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    std::fs::read(root.join("build/engine/wasm/game.wasm")).ok()
}

#[test]
fn game_wasm_loads_with_a_stub_for_every_import() -> Result<(), Error> {
    let Some(wasm) = game_wasm() else {
        eprintln!("skipped: no build/engine/wasm/game.wasm (python tools/engine_wasm.py)");
        return Ok(());
    };
    let runtime = Runtime::new(None)?;
    let game = runtime.load(&wasm)?;
    // the C heap's first region, below the Xbox's window
    assert!(game.memory_size() >= 0x8000_0000);
    // every import the reviewed list names (tools/engine_wasm_imports.txt)
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let listed =
        std::fs::read_to_string(root.join("tools/engine_wasm_imports.txt")).unwrap_or_default();
    assert_eq!(
        game.import_count(),
        listed
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count()
    );
    for export in ["halo_game_main", "malloc", "free", "_initialize"] {
        assert!(game.has_export(export), "game.wasm has no {export}");
    }
    // the data the UI plugin reads (source/interface/ui_widget.c's port:)
    assert!(
        game.data_export(opence_runtime::widgets::WIDGET_GLOBALS_EXPORT)
            .is_some_and(|address| address != 0),
        "game.wasm does not export the widget globals"
    );
    // its threads are its own: the platform starts each at wasi_thread_start
    // through the game's pthread_create, and answers what they need
    for export in ["pthread_create", "pthread_detach", "wasi_thread_start"] {
        assert!(game.has_export(export), "game.wasm has no {export}");
    }
    let names: Vec<String> = (0..game.import_count())
        .map(|index| game.import_name(index))
        .collect();
    for import in [
        "wasi.thread-spawn",
        "wasi_snapshot_preview1.sched_yield",
        "wasi_snapshot_preview1.clock_time_get",
        "env.CreateThread",
    ] {
        assert!(
            names.iter().any(|name| name == import),
            "game.wasm does not import {import}"
        );
        assert!(answers(import), "the platform does not answer {import}");
    }
    // source/ never jumps out of a function (no setjmp or longjmp), so the
    // module needs neither wasm exceptions nor a host to unwind it
    for index in 0..game.import_count() {
        let name = game.import_name(index);
        assert!(
            !name.contains("setjmp") && !name.contains("longjmp"),
            "game.wasm imports {name}"
        );
    }
    Ok(())
}

/// adds 1 to the word at 0, atomically, `n` times; reads it atomically
const ADDER: &str = r#"
(module
  (import "env" "memory" (memory 1 1 shared))
  (func (export "get") (result i32) (i32.atomic.load (i32.const 0)))
  (func (export "add") (param $n i32)
    (loop $again
      (drop (i32.atomic.rmw.add (i32.const 0) (i32.const 1)))
      (local.tee $n (i32.sub (local.get $n) (i32.const 1)))
      (br_if $again))))
"#;

#[test]
fn two_threads_share_a_memory_and_its_atomics() -> Result<(), Box<dyn std::error::Error>> {
    use wasmtime::{Instance, MemoryType, Module, SharedMemory, Store};
    let runtime = Runtime::new(None)?;
    let engine = runtime.engine();
    let module = Module::new(engine, module(ADDER))?;
    let memory = SharedMemory::new(engine, MemoryType::shared(1, 1))?;
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let (engine, module, memory) = (engine.clone(), module.clone(), memory.clone());
            std::thread::spawn(move || -> wasmtime::Result<()> {
                let mut store = Store::new(&engine, ());
                let instance = Instance::new(&mut store, &module, &[memory.into()])?;
                let add = instance.get_typed_func::<i32, ()>(&mut store, "add")?;
                add.call(&mut store, 100_000)
            })
        })
        .collect();
    for thread in threads {
        thread.join().map_err(|_| "a thread panicked")??;
    }
    let mut store = Store::new(engine, ());
    let instance = Instance::new(&mut store, &module, &[memory.into()])?;
    let get = instance.get_typed_func::<(), i32>(&mut store, "get")?;
    assert_eq!(get.call(&mut store, ())?, 200_000);
    Ok(())
}

#[test]
fn a_nan_has_the_same_bits_on_every_machine() -> Result<(), Error> {
    // a NaN with a payload, plus one: ARM keeps the payload, x86 too, but
    // the runtime canonicalises it, as the simulation's determinism needs
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (func (export "run") (result i32)
            (i32.reinterpret_f32
              (f32.add (f32.reinterpret_i32 (i32.const 0x7fa00001)) (f32.const 1)))))"#,
    ))?;
    assert_eq!(game.call_i32("run")?.cast_unsigned(), 0x7fc0_0000);
    Ok(())
}

#[test]
fn relaxed_simd_whose_results_differ_by_machine_is_refused() -> Result<(), Error> {
    // relaxed_madd may or may not fuse the multiply and add; the game is
    // built with -ffp-contract=off and no relaxed SIMD, so the runtime
    // refuses it rather than let a module differ between machines
    let runtime = Runtime::new(None)?;
    let result = runtime.load(&module(
        r#"(module
          (func (export "run") (param v128 v128 v128) (result v128)
            (f32x4.relaxed_madd (local.get 0) (local.get 1) (local.get 2))))"#,
    ));
    assert!(
        matches!(result, Err(Error::Compile(_))),
        "{:?}",
        result.err()
    );
    Ok(())
}

/// the Xbox's contiguous window, as the desktop sizes need it
/// (`port/linux/src/platform.h`): 0x80000000 to 0xA0000000
const WINDOW: std::ops::Range<u64> = 0x8000_0000..0xA000_0000;

#[test]
fn the_c_heap_never_hands_out_the_xbox_window() -> Result<(), Error> {
    let Some(wasm) = game_wasm() else {
        eprintln!("skipped: no build/engine/wasm/game.wasm (python tools/engine_wasm.py)");
        return Ok(());
    };
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&wasm)?;
    // the module starts with the heap's 2 GB, and the platform grows it
    // past the window before the game runs
    assert_eq!(game.memory_size(), WINDOW.start);
    game.grow_memory_to(WINDOW.end)?;
    assert_eq!(game.memory_size(), WINDOW.end);
    game.call("_initialize")?;
    // the window's pages are the game's: the game state and the tag cache
    // are placed at fixed addresses in it, so the C heap (malloc, and
    // GlobalAlloc over it) must lie wholly outside. 2 GB of blocks fill the
    // first region and make it grow.
    let sizes = [16, 4096, 1 << 20, 64 << 20]
        .into_iter()
        .chain([256 << 20; 8]);
    for size in sizes {
        let block = match game.call_with("malloc", &[Val::I32(size)])?.as_slice() {
            [Val::I32(block)] => u64::from(block.cast_unsigned()),
            other => panic!("malloc returned {other:?}"),
        };
        assert_ne!(block, 0, "malloc({size:#x}) failed");
        let end = block + u64::from(size.cast_unsigned());
        assert!(
            end <= WINDOW.start || block >= WINDOW.end,
            "malloc({size:#x}) gave {block:#x}..{end:#x}, inside the window"
        );
    }
    Ok(())
}

#[test]
fn preparing_the_game_grows_its_memory_past_the_window() -> Result<(), Error> {
    let Some(wasm) = game_wasm() else {
        eprintln!("skipped: no build/engine/wasm/game.wasm (python tools/engine_wasm.py)");
        return Ok(());
    };
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&wasm)?;
    let saves = std::env::temp_dir().join("opence_runtime_prepare");
    game.prepare(std::path::Path::new("assets"), &saves, (3840, 2160))?;
    assert_eq!(game.memory_size(), WINDOW.end);
    // a second time changes nothing
    game.prepare(std::path::Path::new("assets"), &saves, (3840, 2160))?;
    assert_eq!(game.memory_size(), WINDOW.end);
    Ok(())
}

const TONED: &str = r#"
(module
  (memory (export "memory") 1)
  (global (export "compressed_monk_tone") i32 (i32.const 1024)))
"#;

#[test]
fn the_chant_takes_the_players_tone_or_is_silent() -> Result<(), Error> {
    use opence_runtime::{TONE_BYTES, TONE_EXPORT};

    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(TONED))?;
    assert_eq!(game.data_export(TONE_EXPORT), Some(1024));
    let tone: [u8; TONE_BYTES] = std::array::from_fn(|index| (index % 251) as u8 + 1);
    assert!(game.set_tone(Some(&tone))?);
    assert_eq!(game.read(1024, TONE_BYTES as u64)?, tone);
    assert!(game.set_tone(None)?);
    assert!(game
        .read(1024, TONE_BYTES as u64)?
        .iter()
        .all(|&byte| byte == 0));
    // a module without the chant has nothing to give it
    let mut plain = runtime.load(&module(CALLER))?;
    assert!(!plain.set_tone(Some(&tone))?);
    Ok(())
}

#[test]
fn game_wasm_holds_no_tone_of_its_own() -> Result<(), Error> {
    let Some(wasm) = game_wasm() else {
        eprintln!("skipped: no build/engine/wasm/game.wasm (python tools/engine_wasm.py)");
        return Ok(());
    };
    let runtime = Runtime::new(None)?;
    let game = runtime.load(&wasm)?;
    let address = game.data_export(opence_runtime::TONE_EXPORT);
    assert!(
        address.is_some_and(|address| address != 0),
        "game.wasm does not export the chant's tone"
    );
    let tone = game.read(
        u64::from(address.unwrap_or(0)),
        opence_runtime::TONE_BYTES as u64,
    )?;
    // source/interface/progress_bar.c's port: the tone is the player's xbe's
    assert!(
        tone.iter().all(|&byte| byte == 0),
        "game.wasm carries a tone"
    );
    Ok(())
}

#[test]
fn every_import_is_answered_or_listed_as_stubbed() {
    // the zero stub answers whatever has no handler, so a missing handler is
    // silent: every import game.wasm makes (tools/engine_wasm_imports.txt)
    // has a handler or is in the reviewed list of deliberate stubs
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let read = |file: &str| std::fs::read_to_string(root.join(file)).unwrap_or_default();
    let imports = read("tools/engine_wasm_imports.txt");
    let stubbed = read("tools/engine_wasm_stubbed.txt");
    let stubbed: Vec<&str> = stubbed
        .lines()
        .filter_map(|line| line.split('#').next())
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    let names: Vec<&str> = imports
        .lines()
        .filter_map(|line| line.split(':').next())
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .collect();
    let unanswered: Vec<&&str> = names
        .iter()
        .filter(|name| !answers(name) && !stubbed.contains(name))
        .collect();
    assert!(
        unanswered.is_empty(),
        "answered by the zero stub: {unanswered:?}"
    );
    // and a stub that gained a handler, or left the game, leaves the list
    let stale: Vec<&&str> = stubbed
        .iter()
        .filter(|name| answers(name) || !names.contains(name))
        .collect();
    assert!(
        stale.is_empty(),
        "listed as stubbed but answered or not imported: {stale:?}"
    );
}
