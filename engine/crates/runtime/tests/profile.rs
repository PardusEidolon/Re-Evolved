//! The game's profile sections (`opence_runtime::profile`): game.wasm's
//! `profile_sections.h` (`port/wasmtime/include/`) makes each section's
//! entry and exit an import, and the runtime keeps the spans between them
//! while profiling is on, so a benchmark times the game's own steps.

use opence_runtime::{answers, Error, Runtime, Val};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// the module: two sections' names in its memory, and the imports re-exported
const SECTIONS: &str = r#"
(module
  (import "env" "halo_profile_enter" (func $enter (param i32)))
  (import "env" "halo_profile_exit" (func $exit (param i32)))
  (import "env" "Sleep" (func $sleep (param i32)))
  (memory (export "memory") 1)
  (data (i32.const 16) "game_tick\00")
  (data (i32.const 32) "present\00")
  (data (i32.const 65534) "ab")
  (func (export "enter") (param i32) (call $enter (local.get 0)))
  (func (export "exit") (param i32) (call $exit (local.get 0)))
  (func (export "sleep") (param i32) (call $sleep (local.get 0))))
"#;

const GAME_TICK: Val = Val::I32(16);
const PRESENT: Val = Val::I32(32);

#[test]
fn the_platform_answers_the_profile_imports() {
    assert!(answers("env.halo_profile_enter"));
    assert!(answers("env.halo_profile_exit"));
}

#[test]
fn sections_are_kept_only_while_profiling_is_on() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(SECTIONS))?;
    game.call_with("enter", &[GAME_TICK])?;
    game.call_with("exit", &[GAME_TICK])?;
    assert!(game.take_profile_spans().is_empty());

    game.set_profiling(true);
    game.call_with("enter", &[GAME_TICK])?;
    game.call_with("enter", &[PRESENT])?;
    game.call_with("exit", &[PRESENT])?;
    game.call_with("exit", &[GAME_TICK])?;
    let spans = game.take_profile_spans();
    // in the order they end: the inner one first
    let names: Vec<&str> = spans.iter().map(|span| span.name.as_str()).collect();
    assert_eq!(names, ["present", "game_tick"]);
    let (inner, outer) = (&spans[0], &spans[1]);
    assert!(outer.start_ns <= inner.start_ns && inner.end_ns <= outer.end_ns);
    // taken, they are gone
    assert!(game.take_profile_spans().is_empty());
    Ok(())
}

#[test]
fn a_section_left_unopened_or_named_outside_the_memory_is_skipped() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(SECTIONS))?;
    game.set_profiling(true);
    // an exit with no entry
    game.call_with("exit", &[PRESENT])?;
    // a name that runs past the memory's end, and one outside it: no
    // trap, no span
    let outside = Val::I32(0x0001_0000 - 2);
    game.call_with("enter", &[outside])?;
    game.call_with("exit", &[outside])?;
    game.call_with("enter", &[Val::I32(-16)])?;
    game.call_with("exit", &[Val::I32(-16)])?;
    assert!(game.take_profile_spans().is_empty());
    Ok(())
}

#[test]
fn a_section_entered_again_before_it_exits_nests() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(SECTIONS))?;
    game.set_profiling(true);
    game.call_with("enter", &[GAME_TICK])?;
    game.call_with("enter", &[GAME_TICK])?;
    game.call_with("exit", &[GAME_TICK])?;
    game.call_with("exit", &[GAME_TICK])?;
    let spans = game.take_profile_spans();
    assert_eq!(spans.len(), 2);
    assert!(spans[1].start_ns <= spans[0].start_ns && spans[0].end_ns <= spans[1].end_ns);
    Ok(())
}

#[test]
fn the_first_thread_s_sleeps_are_kept_while_profiling_is_on() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(SECTIONS))?;
    game.call_with("sleep", &[Val::I32(1)])?;
    assert!(game.take_profile_spans().is_empty());
    game.set_profiling(true);
    game.call_with("sleep", &[Val::I32(5)])?;
    let spans = game.take_profile_spans();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, opence_runtime::SLEEP);
    assert!(spans[0].end_ns - spans[0].start_ns >= 5_000_000);
    Ok(())
}
