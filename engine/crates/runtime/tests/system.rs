//! The debug monitor's imports the platform answers
//! (`opence_runtime::system`): with no monitor, every walk of the loaded
//! modules and their sections is already at its end; and the title's
//! language and how it was launched.

use opence_runtime::{answers, Error, Runtime, Val};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// the module: each import re-exported as a function the test calls
const MONITOR: &str = r#"
(module
  (import "env" "DmWalkLoadedModules" (func $modules (param i32 i32) (result i32)))
  (import "env" "DmWalkModuleSections" (func $sections (param i32 i32 i32) (result i32)))
  (import "env" "DmCloseModuleSections" (func $close (param i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "modules") (param i32 i32) (result i32) (call $modules (local.get 0) (local.get 1)))
  (func (export "sections") (param i32 i32 i32) (result i32)
    (call $sections (local.get 0) (local.get 1) (local.get 2)))
  (func (export "close") (param i32) (result i32) (call $close (local.get 0))))
"#;

/// XBDM_ENDOFLIST and XBDM_NOERR (`port/include/xdk/xdk_xbdm.h`), as the
/// game reads them back: signed 32-bit HRESULTs
const END_OF_LIST: i32 = 0x82DB_0104_u32 as i32;
const NO_ERROR: i32 = 0x02DB_0000;

fn first(results: &[Val]) -> Option<i32> {
    results.first().and_then(Val::i32)
}

#[test]
fn the_monitor_s_walks_end_at_once_and_closing_succeeds() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(MONITOR))?;
    let walk = Val::I32(16);
    let module = Val::I32(32);
    assert_eq!(
        first(&game.call_with("modules", &[walk, module])?),
        Some(END_OF_LIST)
    );
    assert_eq!(
        first(&game.call_with("sections", &[walk, Val::I32(48), Val::I32(64)])?),
        Some(END_OF_LIST)
    );
    assert_eq!(first(&game.call_with("close", &[walk])?), Some(NO_ERROR));
    // nothing was written where the game would read a module or a section
    assert_eq!(game.read(16, 64)?, vec![0; 64]);
    assert_eq!(game.call_count("env.DmWalkLoadedModules"), 1);
    Ok(())
}

#[test]
fn the_platform_answers_the_monitor_s_imports() {
    for name in [
        "env.DmWalkLoadedModules",
        "env.DmWalkModuleSections",
        "env.DmCloseModuleSections",
    ] {
        assert!(answers(name), "{name}");
    }
    for name in ["env.XGetLanguage", "env.XGetLaunchInfo"] {
        assert!(answers(name), "{name}");
    }
    assert!(!answers("env.DmGetModuleLongName"));
}

/// the module: the language and launch imports, and `config_write` to
/// choose the language
const TITLE: &str = r#"
(module
  (import "env" "XGetLanguage" (func $language (result i32)))
  (import "env" "XGetLaunchInfo" (func $launch (param i32 i32) (result i32)))
  (import "env" "config_write" (func $write (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "language") (result i32) (call $language))
  (func (export "launch") (param i32 i32) (result i32) (call $launch (local.get 0) (local.get 1)))
  (func (export "write") (param i32 i32) (result i32) (call $write (local.get 0) (local.get 1))))
"#;

/// XC_LANGUAGE_ENGLISH, _GERMAN and _JAPANESE, and LDT_FROM_DASHBOARD
/// (`port/include/xdk/xdk_xbox.h`)
const ENGLISH: i32 = 1;
const JAPANESE: i32 = 2;
const GERMAN: i32 = 3;
const FROM_DASHBOARD: u8 = 2;
/// the size of LAUNCH_DATA (`port/include/xdk/xdk_pdb.h`)
const LAUNCH_DATA: usize = 3072;

/// `game.language` set to `language` (written as C strings at 256 and 512)
fn choose(game: &mut opence_runtime::Game, language: &str) -> Result<(), Error> {
    game.write(256, b"game.language\0")?;
    let mut value = language.as_bytes().to_vec();
    value.push(0);
    game.write(512, &value)?;
    assert_eq!(
        first(&game.call_with("write", &[Val::I32(256), Val::I32(512)])?),
        Some(1)
    );
    Ok(())
}

#[test]
fn the_language_is_the_setting_s_and_english_by_default() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(TITLE))?;
    assert_eq!(first(&game.call_with("language", &[])?), Some(ENGLISH));
    for (language, expected) in [
        ("de", GERMAN),
        ("ja_JP", JAPANESE),
        ("en", ENGLISH),
        ("nl", ENGLISH),
    ] {
        choose(&mut game, language)?;
        assert_eq!(
            first(&game.call_with("language", &[])?),
            Some(expected),
            "{language}"
        );
    }
    Ok(())
}

#[test]
fn the_game_was_launched_from_the_dashboard_with_no_data() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(TITLE))?;
    game.write(1024, &[0xAA; 4 + LAUNCH_DATA + 4])?;
    let launch = [Val::I32(1024), Val::I32(1028)];
    assert_eq!(first(&game.call_with("launch", &launch)?), Some(0));
    let written = game.read(1024, 4 + 3072 + 4)?;
    assert_eq!(written.get(..4), Some(&[FROM_DASHBOARD, 0, 0, 0][..]));
    assert!(written
        .get(4..4 + LAUNCH_DATA)
        .is_some_and(|data| data.iter().all(|&byte| byte == 0)));
    // nothing past the data is touched
    assert_eq!(written.get(4 + LAUNCH_DATA..), Some(&[0xAA; 4][..]));
    // either pointer may be null
    assert_eq!(
        first(&game.call_with("launch", &[Val::I32(0), Val::I32(0)])?),
        Some(0)
    );
    Ok(())
}

#[test]
fn quitting_from_the_menu_ends_the_game_as_a_clean_exit() -> Result<(), Error> {
    assert!(answers("env.platform_request_quit"));
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "env" "platform_request_quit" (func $quit))
          (func (export "_start") (call $quit) (unreachable)))"#,
    ))?;
    // the Linux port's exit(EXIT_SUCCESS): the run ends there, cleanly
    assert_eq!(game.run_command()?, 0);
    Ok(())
}
