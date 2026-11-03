//! The app's hold on the keyboard and mouse while the game runs
//! (`opence_runtime::Controls`): `halo_game_main` keeps the [`Game`] busy
//! for the whole run, so the app hands the keys and the mouse to a handle
//! it took before, from its own thread. The keys come as macOS's virtual
//! key codes and are given to the game as Windows' (`virtual_key`), and
//! nothing but the keyboard and the mouse is ever the game's controller.

use opence_runtime::{keystroke_flags, virtual_key, Error, Game, Runtime, Val};

/// the module: the debug keyboard's queue and the first controller's state
const MODULE: &str = r#"
(module
  (import "env" "XInputOpen" (func $open (param i32 i32 i32 i32) (result i32)))
  (import "env" "XInputGetState" (func $state (param i32 i32) (result i32)))
  (import "env" "XInputDebugGetKeystroke" (func $keystroke (param i32) (result i32)))
  (import "env" "halo_linux_mouse_look" (func $look (param i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (global (export "XDEVICE_TYPE_GAMEPAD_TABLE") i32 (i32.const 0x400))
  (global (export "XDEVICE_TYPE_DEBUG_KEYBOARD_TABLE") i32 (i32.const 0x410))
  (func (export "console_is_active") (result i32) (i32.const 1))
  (func (export "open") (param i32 i32 i32 i32) (result i32)
    (call $open (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "state") (param i32 i32) (result i32) (call $state (local.get 0) (local.get 1)))
  (func (export "keystroke") (param i32) (result i32) (call $keystroke (local.get 0)))
  (func (export "look") (param i32 i32 i32) (result i32)
    (call $look (local.get 0) (local.get 1) (local.get 2))))
"#;

const GAMEPAD: u32 = 0x400;
const OUT: u32 = 0x100;
const OTHER: u32 = 0x104;
const STATE: u32 = 0x200;
const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;

fn game() -> Result<Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

fn call(game: &mut Game, name: &str, args: &[u32]) -> Result<u32, Error> {
    let args: Vec<Val> = args.iter().map(|arg| Val::I32(arg.cast_signed())).collect();
    Ok(match game.call_with(name, &args)?.as_slice() {
        [Val::I32(value)] => value.cast_unsigned(),
        _ => 0,
    })
}

fn bytes<const N: usize>(game: &Game, address: u32) -> Result<[u8; N], Error> {
    let read = game.read(u64::from(address), N as u64)?;
    let mut bytes = [0; N];
    bytes.copy_from_slice(&read);
    Ok(bytes)
}

#[test]
fn the_controls_reach_the_game_from_another_thread() -> Result<(), Error> {
    let mut game = game()?;
    let controls = game.controls();
    let pad = call(&mut game, "open", &[GAMEPAD, 0, 0, 0])?;
    let app = std::thread::spawn(move || {
        controls.key(0xC0, b'`', 0, true);
        controls.mouse_motion(10.0, 0.0);
        controls.mouse_button(1, true);
    });
    assert!(app.join().is_ok());

    assert_eq!(call(&mut game, "keystroke", &[OUT])?, 0);
    assert_eq!(bytes::<3>(&game, OUT)?, [0xC0, b'`', 0]);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 1);
    Ok(())
}

#[test]
fn a_clone_of_the_controls_is_the_same_keyboard() -> Result<(), Error> {
    let mut game = game()?;
    let controls = game.controls();
    controls.clone().key(b'W', b'w', 0, true);
    assert_eq!(call(&mut game, "keystroke", &[OUT])?, 0);
    assert_eq!(bytes::<3>(&game, OUT)?, [b'W', b'w', 0]);
    Ok(())
}

#[test]
fn only_the_keyboard_and_mouse_are_a_controller() -> Result<(), Error> {
    let mut game = game()?;
    assert_ne!(call(&mut game, "open", &[GAMEPAD, 0, 0, 0])?, 0);
    for port in 1..4 {
        assert_eq!(call(&mut game, "open", &[GAMEPAD, port, 0, 0])?, 0);
    }
    let pad = call(&mut game, "open", &[GAMEPAD, 0, 0, 0])?;
    // (a handle that is not the keyboard's has no state)
    assert_eq!(
        call(&mut game, "state", &[pad + 1, STATE])?,
        ERROR_DEVICE_NOT_CONNECTED
    );
    Ok(())
}

#[test]
fn the_mac_s_keys_are_windows_virtual_keys() {
    // (macOS's kVK_ codes, Carbon's Events.h)
    let pairs: [(u16, u8); 24] = [
        (0x00, b'A'),
        (0x0D, b'W'),
        (0x01, b'S'),
        (0x02, b'D'),
        (0x2E, b'M'),
        (0x12, b'1'),
        (0x1D, b'0'),
        (0x31, 0x20),
        (0x24, 0x0D),
        (0x4C, 0x0D),
        (0x30, 0x09),
        (0x35, 0x1B),
        (0x33, 0x08),
        (0x75, 0x2E),
        (0x32, 0xC0),
        (0x7A, 0x70),
        (0x6F, 0x7B),
        (0x7B, 0x25),
        (0x7E, 0x26),
        (0x38, 0xA0),
        (0x3C, 0xA1),
        (0x3B, 0xA2),
        (0x3A, 0xA4),
        (0x52, 0x60),
    ];
    for (mac, windows) in pairs {
        assert_eq!(virtual_key(mac), Some(windows), "kVK 0x{mac:02X}");
    }
    // (the command keys stay the Mac's, and codes past the table are none)
    assert_eq!(virtual_key(0x37), None);
    assert_eq!(virtual_key(0x36), None);
    assert_eq!(virtual_key(0x80), None);
    assert_eq!(virtual_key(u16::MAX), None);
}

#[test]
fn the_modifiers_are_the_keystroke_s_flags() {
    assert_eq!(keystroke_flags(false, false, false), 0);
    assert_eq!(keystroke_flags(true, false, false), 0x01);
    assert_eq!(keystroke_flags(false, true, false), 0x02);
    assert_eq!(keystroke_flags(false, false, true), 0x04);
    assert_eq!(keystroke_flags(true, true, true), 0x07);
}
