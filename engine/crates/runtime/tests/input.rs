//! The keyboard and mouse as the platform answers the game's input imports
//! (`opence_runtime`'s input module), as the Linux port does
//! (`port/linux/src/xinput_sdl.c`): the keyboard and mouse are the first
//! controller, its debug keyboard queues the keystrokes the console and
//! typing take, and the mouse turns the first player's view.

use opence_runtime::{answers, Error, Runtime, Val};

/// the module: each import re-exported, the device tables, and the
/// console's and chat's states, which the test sets
const MODULE: &str = r#"
(module
  (import "env" "XGetDeviceChanges" (func $changes (param i32 i32 i32) (result i32)))
  (import "env" "XInitDevices" (func $init (param i32 i32)))
  (import "env" "XInputOpen" (func $open (param i32 i32 i32 i32) (result i32)))
  (import "env" "XInputClose" (func $close (param i32)))
  (import "env" "XInputGetState" (func $state (param i32 i32) (result i32)))
  (import "env" "XInputSetState" (func $set_state (param i32 i32) (result i32)))
  (import "env" "XInputDebugInitKeyboardQueue" (func $queue (param i32) (result i32)))
  (import "env" "XInputDebugGetKeystroke" (func $keystroke (param i32) (result i32)))
  (import "env" "halo_keyboard_actions" (func $actions (param i32) (result i32)))
  (import "env" "halo_linux_mouse_look" (func $look (param i32 i32 i32) (result i32)))
  (import "env" "halo_linux_mouse_aiming" (func $aiming (param i32) (result i32)))
  (import "env" "halo_linux_touch_look" (func $touch_look (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "halo_linux_touch_move" (func $touch_move (param i32 i32 i32) (result i32)))
  (import "env" "halo_linux_touch_aiming" (func $touch_aiming (param i32) (result i32)))
  (import "env" "halo_push_to_talk_held" (func $talk (result i32)))
  (import "env" "halo_ui_pointer_update" (func $pointer (param i32 i32) (result i32)))
  (import "env" "host_menus_active" (func $menus (param i32)))
  (import "env" "host_ui_pointer_read" (func $read_pointer (param i32) (result i32)))
  (import "env" "platform_text_typing" (func $typing (param i32)))
  (import "env" "host_ui_loading_held" (func $held (result i32)))
  (import "env" "GetLastError" (func $last_error (result i32)))
  (memory (export "memory") 1)
  (global (export "XDEVICE_TYPE_GAMEPAD_TABLE") i32 (i32.const 0x400))
  (global (export "XDEVICE_TYPE_DEBUG_KEYBOARD_TABLE") i32 (i32.const 0x410))
  (global $console (mut i32) (i32.const 0))
  (func (export "console_is_active") (result i32) (global.get $console))
  (func (export "set_console") (param i32) (global.set $console (local.get 0)))
  (func (export "changes") (param i32 i32 i32) (result i32)
    (call $changes (local.get 0) (local.get 1) (local.get 2)))
  (func (export "init") (param i32 i32) (call $init (local.get 0) (local.get 1)))
  (func (export "open") (param i32 i32 i32 i32) (result i32)
    (call $open (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "close") (param i32) (call $close (local.get 0)))
  (func (export "state") (param i32 i32) (result i32) (call $state (local.get 0) (local.get 1)))
  (func (export "set_state") (param i32 i32) (result i32) (call $set_state (local.get 0) (local.get 1)))
  (func (export "queue") (param i32) (result i32) (call $queue (local.get 0)))
  (func (export "keystroke") (param i32) (result i32) (call $keystroke (local.get 0)))
  (func (export "actions") (param i32) (result i32) (call $actions (local.get 0)))
  (func (export "look") (param i32 i32 i32) (result i32)
    (call $look (local.get 0) (local.get 1) (local.get 2)))
  (func (export "aiming") (param i32) (result i32) (call $aiming (local.get 0)))
  (func (export "touch_look") (param i32 i32 i32 i32 i32) (result i32)
    (call $touch_look (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "touch_move") (param i32 i32 i32) (result i32)
    (call $touch_move (local.get 0) (local.get 1) (local.get 2)))
  (func (export "touch_aiming") (param i32) (result i32) (call $touch_aiming (local.get 0)))
  (func (export "talk") (result i32) (call $talk))
  (func (export "pointer") (param i32 i32) (result i32) (call $pointer (local.get 0) (local.get 1)))
  (func (export "menus") (param i32) (call $menus (local.get 0)))
  (func (export "read_pointer") (param i32) (result i32) (call $read_pointer (local.get 0)))
  (func (export "typing") (param i32) (call $typing (local.get 0)))
  (func (export "held") (result i32) (call $held))
  (func (export "last_error") (result i32) (call $last_error)))
"#;

/// the device tables the module exports
const GAMEPAD: u32 = 0x400;
const DEBUG_KEYBOARD: u32 = 0x410;
/// where the test has the imports write
const OUT: u32 = 0x100;
const OTHER: u32 = 0x104;
const STATE: u32 = 0x200;

/// `XINPUT_STATE`'s gamepad: its buttons, analog buttons and sticks
const BUTTONS: u32 = STATE + 4;
const ANALOG: u32 = STATE + 6;
const THUMB_LX: u32 = STATE + 14;
const THUMB_LY: u32 = STATE + 16;
const DPAD_UP: u16 = 0x1;
const START: u16 = 0x10;
const BACK: u16 = 0x20;
const ANALOG_A: u32 = 0;
const ANALOG_B: u32 = 1;

const ERROR_HANDLE_EOF: u32 = 38;
const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;
const KEYUP: u8 = 0x40;

/// Windows' virtual keys the tests press
const VK_W: u8 = b'W';
const VK_D: u8 = b'D';
const VK_SPACE: u8 = 0x20;
const VK_ESCAPE: u8 = 0x1B;
const VK_TAB: u8 = 0x09;
const VK_UP: u8 = 0x26;
const VK_BACKQUOTE: u8 = 0xC0;
const VK_V: u8 = b'V';

/// halo_keyboard_actions' bits for the default bindings tried
const MOVE_FORWARD: u32 = 1 << 0;
const STRAFE_RIGHT: u32 = 1 << 3;
const JUMP: u32 = 1 << 4;
const FIRE: u32 = 1 << 6;

fn game() -> Result<opence_runtime::Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

/// calls `name` with these unsigned arguments; its one result, or 0
fn call(game: &mut opence_runtime::Game, name: &str, args: &[u32]) -> Result<u32, Error> {
    let args: Vec<Val> = args.iter().map(|arg| Val::I32(arg.cast_signed())).collect();
    Ok(match game.call_with(name, &args)?.as_slice() {
        [Val::I32(value)] => value.cast_unsigned(),
        [] => 0,
        other => panic!("{name} returned {other:?}"),
    })
}

fn bytes<const N: usize>(game: &opence_runtime::Game, address: u32) -> Result<[u8; N], Error> {
    let read = game.read(u64::from(address), N as u64)?;
    let mut bytes = [0; N];
    bytes.copy_from_slice(&read);
    Ok(bytes)
}

fn word(game: &opence_runtime::Game, address: u32) -> Result<u32, Error> {
    bytes(game, address).map(u32::from_le_bytes)
}

fn half(game: &opence_runtime::Game, address: u32) -> Result<u16, Error> {
    bytes(game, address).map(u16::from_le_bytes)
}

fn real(game: &opence_runtime::Game, address: u32) -> Result<f32, Error> {
    bytes(game, address).map(f32::from_le_bytes)
}

/// the first controller, opened
fn first_controller(game: &mut opence_runtime::Game) -> Result<u32, Error> {
    let handle = call(game, "open", &[GAMEPAD, 0, 0, 0])?;
    assert_ne!(handle, 0);
    Ok(handle)
}

#[test]
fn the_platform_answers_the_input_imports() {
    for name in [
        "env.XGetDeviceChanges",
        "env.XInitDevices",
        "env.XInputOpen",
        "env.XInputClose",
        "env.XInputGetState",
        "env.XInputSetState",
        "env.XInputDebugInitKeyboardQueue",
        "env.XInputDebugGetKeystroke",
        "env.halo_keyboard_actions",
        "env.halo_linux_mouse_look",
        "env.halo_linux_mouse_aiming",
        "env.halo_linux_touch_look",
        "env.halo_linux_touch_move",
        "env.halo_linux_touch_aiming",
        "env.halo_push_to_talk_held",
        "env.halo_ui_pointer_update",
        "env.host_menus_active",
        "env.host_ui_pointer_read",
        "env.halo_scoreboard_pointer_update",
        "env.halo_chat_pad_buttons",
        "env.halo_chat_touch_layout",
        "env.platform_text_typing",
        "env.platform_text_field",
    ] {
        assert!(answers(name), "{name}");
    }
}

#[test]
fn the_keyboard_is_the_first_controller_and_the_debug_keyboard_once() -> Result<(), Error> {
    let mut game = game()?;
    call(&mut game, "init", &[0, 0])?;
    assert_eq!(call(&mut game, "changes", &[GAMEPAD, OUT, OTHER])?, 1);
    assert_eq!((word(&game, OUT)?, word(&game, OTHER)?), (1, 0));
    assert_eq!(call(&mut game, "changes", &[GAMEPAD, OUT, OTHER])?, 0);
    assert_eq!((word(&game, OUT)?, word(&game, OTHER)?), (0, 0));
    assert_eq!(
        call(&mut game, "changes", &[DEBUG_KEYBOARD, OUT, OTHER])?,
        1
    );
    assert_eq!(word(&game, OUT)?, 1);
    assert_eq!(
        call(&mut game, "changes", &[DEBUG_KEYBOARD, OUT, OTHER])?,
        0
    );
    assert_eq!(call(&mut game, "queue", &[0])?, 0);
    Ok(())
}

#[test]
fn a_device_that_is_not_there_does_not_open() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "open", &[GAMEPAD, 4, 0, 0])?, 0);
    assert_eq!(
        call(&mut game, "last_error", &[])?,
        ERROR_DEVICE_NOT_CONNECTED
    );
    assert_eq!(call(&mut game, "open", &[DEBUG_KEYBOARD, 1, 0, 0])?, 0);
    assert_ne!(call(&mut game, "open", &[DEBUG_KEYBOARD, 0, 0, 0])?, 0);
    let pad = first_controller(&mut game)?;
    assert_eq!(call(&mut game, "state", &[pad, STATE])?, 0);
    call(&mut game, "close", &[pad])?;
    assert_eq!(
        call(&mut game, "state", &[pad, STATE])?,
        ERROR_DEVICE_NOT_CONNECTED
    );
    assert_eq!(
        call(&mut game, "state", &[0x1234, STATE])?,
        ERROR_DEVICE_NOT_CONNECTED
    );
    Ok(())
}

#[test]
fn rumble_is_taken_and_needs_its_feedback() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    game.write(u64::from(OUT), &[0xFF; 4])?;
    assert_eq!(call(&mut game, "set_state", &[pad, OUT])?, 0);
    assert_eq!(word(&game, OUT)?, 0);
    assert_eq!(call(&mut game, "set_state", &[pad, 0])?, 87);
    assert_eq!(
        call(&mut game, "set_state", &[0x1234, OUT])?,
        ERROR_DEVICE_NOT_CONNECTED
    );
    Ok(())
}

#[test]
fn in_the_menus_the_keys_move_about_them() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "pointer", &[1, OUT])?;
    game.key(VK_W, b'w', 0, true);
    game.key(VK_D, b'd', 0, true);
    game.key(VK_UP, 0, 0, true);
    game.key(VK_SPACE, b' ', 0, true);
    assert_eq!(call(&mut game, "state", &[pad, STATE])?, 0);
    // (the keys were down as the menus opened: they count once let go of)
    assert_eq!(half(&game, BUTTONS)?, 0);
    for key in [VK_W, VK_D, VK_UP, VK_SPACE] {
        game.key(key, 0, 0, false);
    }
    call(&mut game, "state", &[pad, STATE])?;
    for key in [VK_W, VK_D, VK_UP, VK_SPACE] {
        game.key(key, 0, 0, true);
    }
    call(&mut game, "state", &[pad, STATE])?;
    let diagonal = (32767.0_f32 * std::f32::consts::FRAC_1_SQRT_2) as i16;
    assert_eq!(half(&game, THUMB_LX)?.cast_signed(), diagonal);
    assert_eq!(half(&game, THUMB_LY)?.cast_signed(), diagonal);
    assert_eq!(half(&game, BUTTONS)?, DPAD_UP);
    assert_eq!(bytes::<1>(&game, ANALOG + ANALOG_A)?, [0xFF]);
    assert_eq!(bytes::<1>(&game, ANALOG + ANALOG_B)?, [0]);
    Ok(())
}

#[test]
fn the_packet_number_counts_the_changes() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "pointer", &[1, OUT])?;
    call(&mut game, "state", &[pad, STATE])?;
    let first = word(&game, STATE)?;
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(word(&game, STATE)?, first);
    game.key(VK_ESCAPE, 0x1B, 0, true);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(word(&game, STATE)?, first + 1);
    assert_eq!(bytes::<1>(&game, ANALOG + ANALOG_B)?, [0xFF]);
    Ok(())
}

#[test]
fn in_the_game_the_bindings_are_the_actions_and_escape_pauses() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "state", &[pad, STATE])?;
    game.key(VK_W, b'w', 0, true);
    game.key(VK_SPACE, b' ', 0, true);
    game.key(VK_D, b'd', 0, true);
    game.mouse_button(1, true);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(
        call(&mut game, "actions", &[0])?,
        MOVE_FORWARD | STRAFE_RIGHT | JUMP | FIRE
    );
    assert_eq!(call(&mut game, "actions", &[1])?, 0);
    assert_eq!(half(&game, BUTTONS)?, 0);
    game.key(VK_ESCAPE, 0x1B, 0, true);
    game.key(VK_TAB, 0x09, 0, true);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(half(&game, BUTTONS)?, START | BACK);
    assert_eq!(half(&game, THUMB_LY)?, 0);
    Ok(())
}

#[test]
fn the_console_takes_the_keys_from_the_controller() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "set_console", &[1])?;
    call(&mut game, "state", &[pad, STATE])?;
    game.key(VK_ESCAPE, 0x1B, 0, true);
    game.key(VK_W, b'w', 0, true);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(half(&game, BUTTONS)?, 0);
    assert_eq!(call(&mut game, "actions", &[0])?, 0);
    Ok(())
}

#[test]
fn keystrokes_pass_to_the_console_and_typing_and_key_ups_always() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "keystroke", &[OUT])?, ERROR_HANDLE_EOF);
    // (in the game a key down is no keystroke, but the console's key is)
    game.key(VK_W, b'w', 0, true);
    game.key(VK_BACKQUOTE, b'`', 0, true);
    game.key(VK_W, b'w', 0, false);
    assert_eq!(call(&mut game, "keystroke", &[OUT])?, 0);
    assert_eq!(bytes::<3>(&game, OUT)?, [VK_BACKQUOTE, b'`', 0]);
    assert_eq!(call(&mut game, "keystroke", &[OUT])?, 0);
    assert_eq!(bytes::<3>(&game, OUT)?, [VK_W, b'w', KEYUP]);
    assert_eq!(call(&mut game, "keystroke", &[OUT])?, ERROR_HANDLE_EOF);
    assert_eq!(bytes::<3>(&game, OUT)?, [0; 3]);
    // (typing takes all but escape; the console all)
    call(&mut game, "typing", &[1])?;
    game.key(VK_ESCAPE, 0x1B, 0, true);
    game.key(b'A', b'a', 0x02, true);
    assert_eq!(call(&mut game, "keystroke", &[OUT])?, 0);
    assert_eq!(bytes::<3>(&game, OUT)?, [b'A', b'a', 0x02]);
    call(&mut game, "typing", &[0])?;
    call(&mut game, "set_console", &[1])?;
    game.key(VK_ESCAPE, 0x1B, 0, true);
    assert_eq!(call(&mut game, "keystroke", &[OUT])?, 0);
    assert_eq!(bytes::<3>(&game, OUT)?, [VK_ESCAPE, 0x1B, 0]);
    Ok(())
}

#[test]
fn the_mouse_turns_the_first_player_by_its_motion() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    assert_eq!(call(&mut game, "aiming", &[0])?, 0);
    game.mouse_motion(10.0, -5.0);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(call(&mut game, "aiming", &[0])?, 1);
    assert_eq!(call(&mut game, "look", &[1, OUT, OTHER])?, 0);
    assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 1);
    assert!((real(&game, OUT)? - -0.022).abs() < 1e-6);
    assert!((real(&game, OTHER)? - 0.011).abs() < 1e-6);
    assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 0);
    assert_eq!((real(&game, OUT)?, real(&game, OTHER)?), (0.0, 0.0));
    Ok(())
}

#[test]
fn motion_nobody_takes_for_a_few_polls_is_dropped() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    game.mouse_motion(10.0, 0.0);
    for _ in 0..5 {
        call(&mut game, "state", &[pad, STATE])?;
    }
    assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 0);
    Ok(())
}

#[test]
fn push_to_talk_is_its_binding_held() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "talk", &[])?, 0);
    game.key(VK_V, b'v', 0, true);
    assert_eq!(call(&mut game, "talk", &[])?, 1);
    Ok(())
}

#[test]
fn the_screenshot_binding_asks_once_a_press() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "state", &[pad, STATE])?;
    game.key(0x79, 0, 0, true);
    call(&mut game, "state", &[pad, STATE])?;
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(game.take_screenshots(), 1);
    assert_eq!(game.take_screenshots(), 0);
    Ok(())
}

#[test]
fn there_are_no_touch_controls() -> Result<(), Error> {
    let mut game = game()?;
    game.write(u64::from(OUT), &[0xFF; 16])?;
    assert_eq!(
        call(
            &mut game,
            "touch_look",
            &[0, OUT, OUT + 4, OUT + 8, OUT + 12]
        )?,
        0
    );
    assert_eq!(bytes::<16>(&game, OUT)?, [0; 16]);
    game.write(u64::from(OUT), &[0xFF; 8])?;
    assert_eq!(call(&mut game, "touch_move", &[0, OUT, OUT + 4])?, 0);
    assert_eq!(bytes::<8>(&game, OUT)?, [0; 8]);
    assert_eq!(call(&mut game, "touch_aiming", &[0])?, 0);
    Ok(())
}

/// `struct platform_ui_pointer` as wasm32 lays it out, read at `at`
#[derive(Debug, PartialEq)]
struct Pointer {
    at: [f32; 2],
    click: [f32; 2],
    moved: u32,
    left_clicks: u32,
    right_clicks: u32,
    wheel_steps: i32,
    downs: u32,
    touch: u32,
}

/// the pointer the game reads, and whether the menus are up
fn read_pointer(game: &mut opence_runtime::Game) -> Result<(u32, Pointer), Error> {
    game.write(u64::from(OUT), &[0xAA; 48])?;
    let active = call(game, "read_pointer", &[OUT])?;
    let pointer = Pointer {
        at: [real(game, OUT)?, real(game, OUT + 4)?],
        click: [real(game, OUT + 8)?, real(game, OUT + 12)?],
        moved: word(game, OUT + 16)?,
        left_clicks: word(game, OUT + 20)?,
        right_clicks: word(game, OUT + 24)?,
        wheel_steps: word(game, OUT + 28)?.cast_signed(),
        downs: word(game, OUT + 32)?,
        touch: word(game, OUT + 44)?,
    };
    // the down's place is left as nothing touched it
    assert_eq!((real(game, OUT + 36)?, real(game, OUT + 40)?), (0.0, 0.0));
    Ok((active, pointer))
}

#[test]
fn the_menus_coming_up_centre_the_pointer_and_out_of_them_there_is_none() -> Result<(), Error> {
    let mut game = game()?;
    let (active, _) = read_pointer(&mut game)?;
    assert_eq!(active, 0);
    call(&mut game, "menus", &[1])?;
    let (active, pointer) = read_pointer(&mut game)?;
    assert_eq!(active, 1);
    assert_eq!(
        pointer,
        Pointer {
            at: [320.0, 240.0],
            click: [0.0, 0.0],
            moved: 0,
            left_clicks: 0,
            right_clicks: 0,
            wheel_steps: 0,
            downs: 0,
            touch: 0,
        }
    );
    // on a drawable of another size, its middle
    call(&mut game, "menus", &[0])?;
    game.set_drawable_size(3840, 2160);
    call(&mut game, "menus", &[1])?;
    assert_eq!(read_pointer(&mut game)?.1.at, [1920.0, 1080.0]);
    call(&mut game, "menus", &[0])?;
    assert_eq!(read_pointer(&mut game)?.0, 0);
    Ok(())
}

#[test]
fn in_the_menus_the_mouse_moves_the_pointer_and_not_the_view() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "menus", &[1])?;
    game.mouse_motion(10.0, -5.0);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 0);
    let (_, pointer) = read_pointer(&mut game)?;
    assert_eq!((pointer.at, pointer.moved), ([330.0, 235.0], 1));
    // read once: moved is taken
    assert_eq!(read_pointer(&mut game)?.1.moved, 0);
    // kept on the drawable
    game.mouse_motion(-1000.0, 1000.0);
    assert_eq!(read_pointer(&mut game)?.1.at, [0.0, 479.0]);
    Ok(())
}

#[test]
fn where_the_window_puts_the_mouse_is_the_pointer() -> Result<(), Error> {
    let mut game = game()?;
    call(&mut game, "menus", &[1])?;
    game.mouse_position(100.0, 50.0);
    let (_, pointer) = read_pointer(&mut game)?;
    assert_eq!((pointer.at, pointer.moved), ([100.0, 50.0], 1));
    // once the window says where, motion no longer moves it
    game.mouse_motion(10.0, 10.0);
    assert_eq!(
        read_pointer(&mut game)?.1,
        Pointer {
            moved: 0,
            ..pointer_at([100.0, 50.0])
        }
    );
    game.mouse_position(9000.0, -3.0);
    assert_eq!(read_pointer(&mut game)?.1.at, [639.0, 0.0]);
    Ok(())
}

#[test]
fn the_app_reads_whether_the_menus_are_up_to_free_the_mouse() -> Result<(), Error> {
    let mut game = game()?;
    let controls = game.controls();
    assert!(!controls.menus_active());
    call(&mut game, "menus", &[1])?;
    assert!(controls.menus_active());
    call(&mut game, "menus", &[0])?;
    assert!(!controls.menus_active());
    Ok(())
}

#[test]
fn a_plugin_holds_the_games_map_load_on_its_loading_screen() -> Result<(), Error> {
    let mut game = game()?;
    let controls = game.controls();
    assert_eq!(call(&mut game, "held", &[])?, 0);
    controls.set_loading_held(true);
    assert_eq!(call(&mut game, "held", &[])?, 1);
    controls.set_loading_held(false);
    assert_eq!(call(&mut game, "held", &[])?, 0);
    Ok(())
}

fn pointer_at(at: [f32; 2]) -> Pointer {
    Pointer {
        at,
        click: [0.0, 0.0],
        moved: 1,
        left_clicks: 0,
        right_clicks: 0,
        wheel_steps: 0,
        downs: 0,
        touch: 0,
    }
}

#[test]
fn in_the_menus_a_click_is_counted_where_the_pointer_is_and_holds_nothing() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "menus", &[1])?;
    game.mouse_motion(5.0, 5.0);
    game.mouse_button(1, true);
    game.mouse_button(1, false);
    game.mouse_button(3, true);
    game.mouse_button(1, true);
    call(&mut game, "state", &[pad, STATE])?;
    // the fire binding is not held by a click in the menus
    assert_eq!(call(&mut game, "actions", &[0])? & FIRE, 0);
    let (_, pointer) = read_pointer(&mut game)?;
    assert_eq!(
        (pointer.left_clicks, pointer.right_clicks, pointer.click),
        (2, 1, [325.0, 245.0])
    );
    let (_, pointer) = read_pointer(&mut game)?;
    assert_eq!((pointer.left_clicks, pointer.right_clicks), (0, 0));
    Ok(())
}

#[test]
fn in_the_menus_the_wheel_is_counted_in_whole_notches() -> Result<(), Error> {
    let mut game = game()?;
    call(&mut game, "menus", &[1])?;
    game.mouse_wheel(1.5);
    assert_eq!(read_pointer(&mut game)?.1.wheel_steps, 1);
    game.mouse_wheel(0.5);
    assert_eq!(read_pointer(&mut game)?.1.wheel_steps, 1);
    game.mouse_wheel(-2.0);
    assert_eq!(read_pointer(&mut game)?.1.wheel_steps, -2);
    assert_eq!(read_pointer(&mut game)?.1.wheel_steps, 0);
    Ok(())
}

#[test]
fn the_menus_coming_up_drop_what_the_mouse_held() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    game.mouse_button(1, true);
    game.mouse_motion(20.0, 0.0);
    call(&mut game, "menus", &[1])?;
    call(&mut game, "menus", &[0])?;
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(call(&mut game, "actions", &[0])? & FIRE, 0);
    assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 0);
    Ok(())
}

#[test]
fn with_the_menus_up_the_keys_move_about_them() -> Result<(), Error> {
    let mut game = game()?;
    let pad = first_controller(&mut game)?;
    call(&mut game, "menus", &[1])?;
    call(&mut game, "state", &[pad, STATE])?;
    game.key(VK_UP, 0, 0, true);
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(half(&game, BUTTONS)? & DPAD_UP, DPAD_UP);
    Ok(())
}

/// the control socket's answer to `line`, as JSON
fn ask(driven: &opence_runtime::control::Driven, line: &str) -> serde_json::Value {
    let answered = opence_runtime::control::answer(driven, line);
    serde_json::from_str(&answered).unwrap_or_default()
}

#[test]
fn the_control_socket_puts_the_pointer_and_clicks_where_it_is_told() -> Result<(), Error> {
    let mut game = game()?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    call(&mut game, "menus", &[1])?;
    let moved = ask(&driven, r#"{"request": "pointer", "x": 100, "y": 50}"#);
    assert_eq!(moved["ok"], true, "{moved}");
    assert_eq!(read_pointer(&mut game)?.1.at, [100.0, 50.0]);
    let clicked = ask(&driven, r#"{"request": "click", "x": 200, "y": 120}"#);
    assert_eq!(clicked["ok"], true, "{clicked}");
    let (_, pointer) = read_pointer(&mut game)?;
    assert_eq!(
        (pointer.at, pointer.left_clicks, pointer.right_clicks),
        ([200.0, 120.0], 1, 0)
    );
    // the right button, by its number
    ask(
        &driven,
        r#"{"request": "click", "x": 10, "y": 10, "button": 3}"#,
    );
    assert_eq!(read_pointer(&mut game)?.1.right_clicks, 1);
    ask(&driven, r#"{"request": "wheel", "notches": -2}"#);
    assert_eq!(read_pointer(&mut game)?.1.wheel_steps, -2);
    Ok(())
}

#[test]
fn the_control_socket_holds_a_key_until_it_is_let_go() -> Result<(), Error> {
    let mut game = game()?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    let pad = first_controller(&mut game)?;
    call(&mut game, "menus", &[1])?;
    call(&mut game, "state", &[pad, STATE])?;
    let down = ask(
        &driven,
        &format!(r#"{{"request": "key", "vk": {VK_UP}, "down": true}}"#),
    );
    assert_eq!(down["ok"], true, "{down}");
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(half(&game, BUTTONS)? & DPAD_UP, DPAD_UP);
    ask(
        &driven,
        &format!(r#"{{"request": "key", "vk": {VK_UP}, "down": false}}"#),
    );
    call(&mut game, "state", &[pad, STATE])?;
    assert_eq!(half(&game, BUTTONS)? & DPAD_UP, 0);
    Ok(())
}

#[test]
fn the_control_socket_quits_the_game_at_its_next_call() -> Result<(), Error> {
    let mut game = game()?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    call(&mut game, "menus", &[1])?;
    let quit = ask(&driven, r#"{"request": "quit"}"#);
    assert_eq!(quit["ok"], true, "{quit}");
    match call(&mut game, "menus", &[1]) {
        Err(Error::Deadline) => Ok(()),
        other => panic!("the game ran on after quit: {other:?}"),
    }
}

#[test]
fn the_control_socket_refuses_what_it_cannot_do() -> Result<(), Error> {
    let mut game = game()?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    for line in [
        r#"{"request": "key", "vk": 38}"#,
        r#"{"request": "key", "vk": 300, "down": true}"#,
        r#"{"request": "key", "vk": "up", "down": true}"#,
        r#"{"request": "pointer", "x": 1}"#,
        r#"{"request": "pointer", "x": "a", "y": 1}"#,
        r#"{"request": "click", "x": 1, "y": 1, "button": 0}"#,
        r#"{"request": "wheel"}"#,
        r#"{"request": "open_screen"}"#,
        r#"{"request": "open_screen", "name": 3}"#,
        r#"{"request": "open_screen", "name": ""}"#,
        r#"{"request": "fly"}"#,
    ] {
        let refused = ask(&driven, line);
        assert_eq!(refused["ok"], false, "{line}: {refused}");
        assert!(refused["error"].is_string(), "{line}: {refused}");
    }
    // the menus' focus, before the game has drawn a screen
    let focus = ask(&driven, r#"{"request": "focus"}"#);
    assert_eq!(focus["ok"], false, "{focus}");
    Ok(())
}

#[test]
fn the_look_is_linear_unsmoothed_and_its_sensitivity_set_through_the_socket() -> Result<(), Error> {
    let mut game = game()?;
    let driven = opence_runtime::control::Driven::of(&mut game);
    let pad = first_controller(&mut game)?;
    let set = |name: &str, value: &str| {
        let line = format!(r#"{{"request": "setting", "name": "{name}", "value": "{value}"}}"#);
        ask(&driven, &line)
    };
    assert_eq!(set("input.mouse_sensitivity", "2")["ok"], true);
    assert_eq!(set("input.mouse_vertical_sensitivity", "0.5")["ok"], true);
    // as the Linux port's halo_linux_mouse_look: 0.0022 radians a pixel
    // times the sensitivity, the same at every speed, and no motion is
    // held back for a later frame
    for (dx, dy) in [(10.0, -5.0), (400.0, 200.0), (1.0, 0.0)] {
        game.mouse_motion(dx, dy);
        call(&mut game, "state", &[pad, STATE])?;
        assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 1);
        assert!(
            (real(&game, OUT)? - -dx * 0.0022 * 2.0).abs() < 1e-6,
            "yaw for {dx}"
        );
        assert!(
            (real(&game, OTHER)? - -dy * 0.0022 * 0.5).abs() < 1e-6,
            "pitch for {dy}"
        );
        assert_eq!(call(&mut game, "look", &[0, OUT, OTHER])?, 0);
    }
    Ok(())
}
