//! The keyboard and mouse, as the game's input imports read them: the
//! Xbox's controller and debug keyboard (`XInput*`) and the Linux port's
//! hooks for the mouse's look and the keyboard's actions. Each follows the
//! Linux port (`port/linux/src/xinput_sdl.c`), with the layouts of the
//! project's own header (`port/include/xdk/`). There is no gamepad: the
//! keyboard and mouse are the first controller, and nothing else is.
//!
//! The app hands the keys and the mouse to the [`crate::Game`]'s host
//! methods as Windows' virtual keys and SDL's button numbers; the game
//! reads them at its own pace, through these imports.

use std::collections::{BTreeSet, VecDeque};

use wasmtime::{Caller, Extern, Val};

use super::memory::{arg, call_game, give, Handler};
use super::{with_space, Record};

/// Win32's error codes the imports give
const ERROR_SUCCESS: u32 = 0;
const ERROR_HANDLE_EOF: u32 = 38;
const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;

/// the controllers' ports
const PORTS: usize = 4;
/// the handles `XInputOpen` gives: a controller's from its port, and the
/// debug keyboard's
const GAMEPAD_HANDLE: u32 = 0x1000;
const GAMEPAD_HANDLE_STEP: u32 = 0x10;
const KEYBOARD_HANDLE: u32 = 0x2000;

/// `XINPUT_STATE`: the packet number, then the gamepad
const STATE_BYTES: u32 = 22;
const GAMEPAD_BYTES: usize = 18;
/// a keystroke: its virtual key, its character and its flags
const KEYSTROKE_BYTES: u32 = 3;
const KEYSTROKE_KEYUP: u8 = 0x40;

/// `XINPUT_GAMEPAD`'s buttons
const DPAD_UP: u16 = 0x1;
const DPAD_DOWN: u16 = 0x2;
const DPAD_LEFT: u16 = 0x4;
const DPAD_RIGHT: u16 = 0x8;
const START: u16 = 0x10;
const BACK: u16 = 0x20;
/// its analog buttons' indices
const ANALOG_A: usize = 0;
const ANALOG_B: usize = 1;
const ANALOG_X: usize = 2;
const ANALOG_Y: usize = 3;
const ANALOG_PRESSED: u8 = 0xFF;
/// a stick all the way, and on a diagonal
const STICK_FULL: f32 = 32767.0;
const DIAGONAL: f32 = std::f32::consts::FRAC_1_SQRT_2;

/// the keys the app keeps for itself, never bound (as the Linux port's
/// F11 and F12)
const HOST_KEYS: [u8; 2] = [VK_F1 + 10, VK_F1 + 11];

/// Windows' virtual keys the controls read
const VK_BACK: u8 = 0x08;
const VK_TAB: u8 = 0x09;
const VK_RETURN: u8 = 0x0D;
const VK_ESCAPE: u8 = 0x1B;
const VK_SPACE: u8 = 0x20;
const VK_LEFT: u8 = 0x25;
const VK_UP: u8 = 0x26;
const VK_RIGHT: u8 = 0x27;
const VK_DOWN: u8 = 0x28;
const VK_DELETE: u8 = 0x2E;
const VK_F1: u8 = 0x70;
const VK_OEM_3_BACKQUOTE: u8 = 0xC0;
/// the mouse's buttons, as SDL numbers them
const MOUSE_X1: u8 = 4;

/// the keyboard's actions, in `halo_keyboard_actions`' bit order
const BINDINGS: [&str; 21] = [
    "controls.move_forward",
    "controls.move_backward",
    "controls.strafe_left",
    "controls.strafe_right",
    "controls.jump",
    "controls.crouch",
    "controls.fire",
    "controls.throw_grenade",
    "controls.melee",
    "controls.reload",
    "controls.zoom",
    "controls.switch_weapon",
    "controls.switch_grenade",
    "controls.action",
    "controls.flashlight",
    "controls.scoreboard",
    "controls.pause",
    "controls.screenshot",
    "controls.push_to_talk",
    "controls.chat",
    "controls.team_chat",
];
const SCOREBOARD: usize = 15;
const PAUSE: usize = 16;
const SCREENSHOT: usize = 17;
const PUSH_TO_TALK: usize = 18;
/// the inputs an action may be bound to
const MAXIMUM_BINDINGS: usize = 2;

/// the chat pad's bits (`halo_keyboard.h`)
const CHAT_PAD_UP: u32 = 0;
const CHAT_PAD_DOWN: u32 = 1;
const CHAT_PAD_LEFT: u32 = 2;
const CHAT_PAD_RIGHT: u32 = 3;
const CHAT_PAD_A: u32 = 4;
const CHAT_PAD_B: u32 = 5;
const CHAT_PAD_X: u32 = 6;
const CHAT_PAD_Y: u32 = 7;
const CHAT_PAD_START: u32 = 8;
const CHAT_PAD_STICK_UP: u32 = 9;
const CHAT_PAD_STICK_DOWN: u32 = 10;
const CHAT_PAD_STICK_LEFT: u32 = 11;
const CHAT_PAD_STICK_RIGHT: u32 = 12;
/// an analog button half way, and the stick most of the way
const CHAT_PRESSED: u8 = 0x80;
const CHAT_PUSHED: i16 = 22000;

/// the mouse's look: radians per pixel at sensitivity 1
const LOOK_SCALE: f32 = 0.0022;
/// the polls the mouse's motion waits for its look, before it is dropped
const UNCONSUMED_POLLS: u32 = 4;
/// the wheel: a notch presses its binding this long, and it is let go
/// once it has been still this long
const WHEEL_PRESS_MS: u64 = 50;
const WHEEL_STILL_MS: u64 = 200;

/// an input an action is bound to
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Code {
    /// a key, by its virtual key
    Key(u8),
    /// a mouse button, as SDL numbers them (1 the left)
    Mouse(u8),
    /// the wheel either way, up or down
    Wheel,
    WheelUp,
    WheelDown,
}

/// the keys named by more than a letter, a digit, an F key or a keypad
/// digit, as SDL names them (and the Linux port's controls menu writes
/// them, `port/linux/src/xinput_sdl.c`)
const KEY_NAMES: [(&str, u8); 39] = [
    ("Space", VK_SPACE),
    ("Tab", VK_TAB),
    ("Escape", VK_ESCAPE),
    ("Return", VK_RETURN),
    ("Backspace", VK_BACK),
    ("Delete", VK_DELETE),
    ("Insert", 0x2D),
    ("Home", 0x24),
    ("End", 0x23),
    ("PageUp", 0x21),
    ("PageDown", 0x22),
    ("Left", VK_LEFT),
    ("Up", VK_UP),
    ("Right", VK_RIGHT),
    ("Down", VK_DOWN),
    ("Left Shift", 0xA0),
    ("Right Shift", 0xA1),
    ("Left Ctrl", 0xA2),
    ("Right Ctrl", 0xA3),
    ("Left Alt", 0xA4),
    ("Right Alt", 0xA5),
    ("CapsLock", 0x14),
    ("Comma", 0xBC),
    ("Keypad Comma", 0x6C),
    ("Keypad *", 0x6A),
    ("Keypad +", 0x6B),
    ("Keypad -", 0x6D),
    ("Keypad .", 0x6E),
    ("Keypad /", 0x6F),
    (".", 0xBE),
    ("/", 0xBF),
    (";", 0xBA),
    ("'", 0xDE),
    ("[", 0xDB),
    ("]", 0xDD),
    ("\\", 0xDC),
    ("-", 0xBD),
    ("=", 0xBB),
    ("`", VK_OEM_3_BACKQUOTE),
];
/// the mouse's buttons with names of their own
const BUTTON_NAMES: [(&str, u8); 5] = [
    ("Mouse Left", 1),
    ("Mouse Middle", 2),
    ("Mouse Right", 3),
    ("Mouse 4", 4),
    ("Mouse 5", 5),
];

/// the input named `name` in a `controls.*` setting, whatever its case:
/// SDL's key names, "Mouse Left" and the like, and the wheel's
fn code_of(name: &str) -> Option<Code> {
    let name = name.trim().to_ascii_lowercase();
    let named = |table: &[(&str, u8)]| {
        table
            .iter()
            .find(|(each, _)| each.eq_ignore_ascii_case(&name))
            .map(|&(_, value)| value)
    };
    if let Some(button) = named(&BUTTON_NAMES) {
        return Some(Code::Mouse(button));
    }
    if let Some(key) = named(&KEY_NAMES) {
        return Some(Code::Key(key));
    }
    match name.as_str() {
        "wheel" => return Some(Code::Wheel),
        "wheel up" => return Some(Code::WheelUp),
        "wheel down" => return Some(Code::WheelDown),
        "keypad enter" => return Some(Code::Key(VK_RETURN)),
        _ => {}
    }
    if let Some(number) = name.strip_prefix("mouse ") {
        return number
            .parse::<u8>()
            .ok()
            .filter(|button| (1..=9).contains(button))
            .map(Code::Mouse);
    }
    if let Some(digit) = name.strip_prefix("keypad ") {
        return digit
            .parse::<u8>()
            .ok()
            .filter(|&digit| digit <= 9)
            .map(|digit| Code::Key(0x60 + digit));
    }
    if let Some(number) = name.strip_prefix('f') {
        if let Ok(number @ 1..=12) = number.parse::<u8>() {
            return Some(Code::Key(VK_F1 + number - 1));
        }
    }
    match name.as_bytes() {
        [letter @ b'a'..=b'z'] => Some(Code::Key(letter.to_ascii_uppercase())),
        [digit @ b'0'..=b'9'] => Some(Code::Key(*digit)),
        _ => None,
    }
}

/// the name a `controls.*` setting gives `code`, which `code_of` reads
/// back; none for a key with no name
fn name_of(code: Code) -> Option<String> {
    let named = |table: &[(&str, u8)], value| {
        table
            .iter()
            .find(|&&(_, each)| each == value)
            .map(|(name, _)| (*name).to_owned())
    };
    match code {
        Code::Wheel => Some("Wheel".to_owned()),
        Code::WheelUp => Some("Wheel Up".to_owned()),
        Code::WheelDown => Some("Wheel Down".to_owned()),
        Code::Mouse(button @ 1..=9) => {
            named(&BUTTON_NAMES, button).or_else(|| Some(format!("Mouse {button}")))
        }
        Code::Mouse(_) => None,
        Code::Key(key) => named(&KEY_NAMES, key).or_else(|| match key {
            b'A'..=b'Z' | b'0'..=b'9' => Some(char::from(key).to_string()),
            0x60..=0x69 => Some(format!("Keypad {}", key - 0x60)),
            _ if (VK_F1..VK_F1 + 12).contains(&key) => Some(format!("F{}", key - VK_F1 + 1)),
            _ => None,
        }),
    }
}

/// an input as the game's controls menu passes it about (`int input` in
/// `control_capture_poll`): a key is its virtual key, a mouse button 256
/// and on, and the wheel 272 and on
const INPUT_MOUSE: u32 = 256;
const INPUT_WHEEL: u32 = 272;
const INPUT_WHEEL_UP: u32 = 273;
const INPUT_WHEEL_DOWN: u32 = 274;

fn input_of(code: Code) -> u32 {
    match code {
        Code::Key(key) => u32::from(key),
        Code::Mouse(button) => INPUT_MOUSE + u32::from(button),
        Code::Wheel => INPUT_WHEEL,
        Code::WheelUp => INPUT_WHEEL_UP,
        Code::WheelDown => INPUT_WHEEL_DOWN,
    }
}

fn code_of_input(input: u32) -> Option<Code> {
    match input {
        INPUT_WHEEL => Some(Code::Wheel),
        INPUT_WHEEL_UP => Some(Code::WheelUp),
        INPUT_WHEEL_DOWN => Some(Code::WheelDown),
        _ if input < INPUT_MOUSE => u8::try_from(input).ok().map(Code::Key),
        _ => u8::try_from(input - INPUT_MOUSE).ok().map(Code::Mouse),
    }
}

/// what `platform_binding_capture_poll` answers: nothing yet, an input
/// taken, Delete (clear the binding) or Escape (keep it)
const CAPTURE_NONE: u32 = 0;
const CAPTURE_TAKEN: u32 = 1;
const CAPTURE_CLEAR: u32 = 2;
const CAPTURE_CANCEL: u32 = 3;
/// a wait nobody polls is given up after this long, and an input taken
/// that nobody claims is dropped after this long
const CAPTURE_ABANDON_MS: u64 = 500;
const CAPTURE_UNCLAIMED_MS: u64 = 2000;

/// the controls menu's capture of an input to bind, as the Linux port's
/// (`port/linux/src/sdl_platform.c`)
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
enum Capture {
    #[default]
    Idle,
    /// waiting for an input, polled last at this time
    Waiting(u64),
    /// what was taken, and when it was first seen taken (none yet)
    Taken(u32, u32, Option<u64>),
}

/// each action's inputs, from the settings
type Bindings = [[Option<Code>; MAXIMUM_BINDINGS]; BINDINGS.len()];

/// the bindings the settings name now
fn bindings_of(record: &Record) -> Bindings {
    let settings = record.settings();
    let mut bindings: Bindings = [[None; MAXIMUM_BINDINGS]; BINDINGS.len()];
    for (slots, name) in bindings.iter_mut().zip(BINDINGS) {
        let text = settings.text(name).unwrap_or_default();
        for (slot, part) in slots.iter_mut().zip(text.split(',')) {
            *slot = code_of(part);
        }
    }
    bindings
}

/// the mouse's settings: its sensitivities, whether it is inverted and
/// whether the game's aim assist turns it
struct Mouse {
    sensitivity: f32,
    vertical: f32,
    invert: bool,
    aim_assist: bool,
}

/// the mouse's settings now
fn mouse_of(record: &Record) -> Mouse {
    let settings = record.settings();
    let real = |name| settings.real(name).unwrap_or_default() as f32;
    let sensitivity = Some(real("input.mouse_sensitivity"))
        .filter(|&value| value > 0.0)
        .unwrap_or(1.0);
    Mouse {
        sensitivity,
        vertical: Some(real("input.mouse_vertical_sensitivity"))
            .filter(|&value| value > 0.0)
            .unwrap_or(sensitivity),
        invert: settings.boolean("input.invert_mouse").unwrap_or_default(),
        aim_assist: settings
            .boolean("input.mouse_aim_assist")
            .unwrap_or_default(),
    }
}

/// the keyboard, the mouse, and what the game has opened of them
#[derive(Default)]
pub(crate) struct Input {
    /// the keys held, and those held over from before the game's context
    /// last changed (which count as up until let go)
    keys: BTreeSet<u8>,
    held_over: BTreeSet<u8>,
    context: Option<u8>,
    /// the mouse's buttons held, a bit for each of SDL's numbers
    buttons: u16,
    /// the keystrokes the debug keyboard has not given yet
    queue: VecDeque<[u8; 3]>,
    /// the mouse's motion since the last poll, and since the last look
    fresh: [f32; 2],
    pending: [f32; 2],
    unconsumed: u32,
    aimed: bool,
    /// the wheel's notches since the last poll and since it last scrolled,
    /// and its scroll: when it moved, its direction, and until when it
    /// presses its bindings
    fresh_wheel: f32,
    wheel: f32,
    wheel_moved: u64,
    scrolling: bool,
    direction: i8,
    press_until: u64,
    /// whether the game is in its menus, and is typing (on its keyboard
    /// or in a text field); Enter starts only once it has been let go of
    menus: bool,
    keyboard_typing: bool,
    field_typing: bool,
    enter_armed: bool,
    /// the controllers and debug keyboard opened, and reported
    open: [bool; PORTS],
    keyboard_open: bool,
    reported: u32,
    keyboard_reported: bool,
    /// each controller's last gamepad and packet number
    previous: [[u8; GAMEPAD_BYTES]; PORTS],
    packets: [u32; PORTS],
    /// the actions held at the last poll
    actions: u32,
    /// whether the screenshot binding was held, and the screenshots asked
    screenshot_down: bool,
    screenshots: u32,
    /// the chat's pad, and what it held over from its on-screen keyboard
    chat_pad: u32,
    chat_held_buttons: u16,
    chat_held_analog: u8,
    /// the menus' pointer
    pointer: Pointer,
    /// the controls menu's capture, whether what was held into or during
    /// it is still being let go of (the game sees nothing until it is),
    /// and the mouse's buttons held, in the menus or not
    capture: Capture,
    settling: bool,
    pressed: u16,
}

/// the menus' pointer, as the Linux port keeps it
/// (`port/linux/src/sdl_platform.c`): where it is on the drawable, in its
/// pixels, where it was last clicked, and what happened since the game
/// last read it
#[derive(Default)]
struct Pointer {
    /// the drawable's size, which keeps the pointer on it
    area: [f32; 2],
    at: [f32; 2],
    click: [f32; 2],
    moved: bool,
    left_clicks: u32,
    right_clicks: u32,
    wheel_steps: i32,
    /// the wheel's turn short of a whole notch
    wheel_part: f32,
    /// whether the window says where the mouse is, so that its motion no
    /// longer moves the pointer
    placed: bool,
}

impl Pointer {
    /// `at` kept on the drawable
    fn place(&mut self, at: [f32; 2]) {
        let keep = |value: f32, size: f32| value.clamp(0.0, (size - 1.0).max(0.0));
        self.at = [keep(at[0], self.area[0]), keep(at[1], self.area[1])];
        self.moved = true;
    }

    /// `struct platform_ui_pointer` as wasm32 lays it out: no touch, so no
    /// downs
    fn bytes(&self) -> [u8; POINTER_BYTES] {
        let mut bytes = [0; POINTER_BYTES];
        let words = [
            self.at[0].to_bits(),
            self.at[1].to_bits(),
            self.click[0].to_bits(),
            self.click[1].to_bits(),
            u32::from(self.moved),
            self.left_clicks,
            self.right_clicks,
            self.wheel_steps.cast_unsigned(),
        ];
        for (chunk, word) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(words) {
            chunk.copy_from_slice(&word.to_le_bytes());
        }
        bytes
    }
}

/// `struct platform_ui_pointer`'s size on wasm32
const POINTER_BYTES: usize = 48;

/// `XINPUT_GAMEPAD`, as the game reads it
#[derive(Default, Clone, Copy)]
struct Gamepad {
    buttons: u16,
    analog: [u8; 8],
    thumbs: [i16; 4],
}

impl Gamepad {
    fn bytes(&self) -> [u8; GAMEPAD_BYTES] {
        let mut bytes = [0; GAMEPAD_BYTES];
        bytes[..2].copy_from_slice(&self.buttons.to_le_bytes());
        bytes[2..10].copy_from_slice(&self.analog);
        for (chunk, thumb) in bytes[10..]
            .as_chunks_mut::<2>()
            .0
            .iter_mut()
            .zip(self.thumbs)
        {
            chunk.copy_from_slice(&thumb.to_le_bytes());
        }
        bytes
    }
}

/// the left stick's thumbs
const THUMB_LX: usize = 0;
const THUMB_LY: usize = 1;

/// what the game is doing that the input reads: its console, its chat
/// being typed and the chat's on-screen keyboard
#[derive(Clone, Copy, Default)]
struct Game {
    console: bool,
    chat_typing: bool,
    chat_keyboard: bool,
}

impl Input {
    /// a key `vk` going down or up, with its character and flags
    pub(crate) fn key(&mut self, vk: u8, ascii: u8, flags: u8, down: bool) {
        if down && !self.keys.contains(&vk) && !HOST_KEYS.contains(&vk) {
            let result = match vk {
                VK_ESCAPE => CAPTURE_CANCEL,
                VK_DELETE => CAPTURE_CLEAR,
                _ => CAPTURE_TAKEN,
            };
            self.take(result, input_of(Code::Key(vk)));
        }
        if self.swallowing() {
            if down {
                self.keys.insert(vk);
            } else {
                self.keys.remove(&vk);
                self.settle();
            }
            return;
        }
        let flags = if down {
            flags & !KEYSTROKE_KEYUP
        } else {
            flags | KEYSTROKE_KEYUP
        };
        self.queue.push_back([vk, ascii, flags]);
        if down {
            self.keys.insert(vk);
        } else {
            self.keys.remove(&vk);
        }
    }

    /// the mouse moving by `dx` and `dy` pixels: in the menus it moves
    /// the pointer (unless the window says where the mouse is), not the
    /// view
    pub(crate) fn motion(&mut self, dx: f32, dy: f32) {
        if self.swallowing() {
            return;
        }
        if self.menus {
            if !self.pointer.placed {
                let [x, y] = self.pointer.at;
                self.pointer.place([x + dx, y + dy]);
            }
            return;
        }
        self.fresh[0] += dx;
        self.fresh[1] += dy;
    }

    /// the window putting the mouse at `x` and `y`, the drawable's pixels
    /// from its top left: in the menus, the pointer
    pub(crate) fn position(&mut self, x: f32, y: f32) {
        self.pointer.placed = true;
        if self.menus {
            self.pointer.place([x, y]);
        }
    }

    /// the mouse's button `button` (SDL's numbers) going down or up; in the
    /// menus a press is a click at the pointer, and holds nothing
    pub(crate) fn button(&mut self, button: u8, down: bool) {
        let Some(bit) = 1_u16.checked_shl(u32::from(button)) else {
            return;
        };
        if down {
            if self.pressed & bit == 0 {
                self.take(CAPTURE_TAKEN, input_of(Code::Mouse(button)));
            }
            self.pressed |= bit;
        } else {
            self.pressed &= !bit;
        }
        if self.swallowing() {
            self.buttons &= !bit;
            self.settle();
            return;
        }
        if !down {
            self.buttons &= !bit;
        } else if self.menus {
            match button {
                1 => {
                    self.pointer.left_clicks = self.pointer.left_clicks.saturating_add(1);
                    self.pointer.click = self.pointer.at;
                }
                3 => self.pointer.right_clicks = self.pointer.right_clicks.saturating_add(1),
                _ => {}
            }
        } else {
            self.buttons |= bit;
        }
    }

    /// the wheel turning by `notches` (up positive); in the menus, whole
    /// notches step them
    pub(crate) fn wheel(&mut self, notches: f32) {
        if notches != 0.0 {
            let code = if notches > 0.0 {
                Code::WheelUp
            } else {
                Code::WheelDown
            };
            self.take(CAPTURE_TAKEN, input_of(code));
        }
        if self.swallowing() {
            return;
        }
        if self.menus {
            let turned = self.pointer.wheel_part + notches;
            let steps = turned.trunc();
            self.pointer.wheel_part = turned - steps;
            self.pointer.wheel_steps = self.pointer.wheel_steps.saturating_add(steps as i32);
            return;
        }
        self.fresh_wheel += notches;
    }

    /// the drawable's size, which keeps the pointer on it
    pub(crate) fn set_area(&mut self, width: u32, height: u32) {
        self.pointer.area = [width as f32, height as f32];
    }

    /// whether the menus are up (`host_menus_active`)
    pub(crate) fn menus(&self) -> bool {
        self.menus
    }

    /// the menus coming up or going: what the mouse held is let go, and
    /// coming up the pointer starts in the drawable's middle
    pub(crate) fn set_menus(&mut self, active: bool) {
        if active == self.menus {
            return;
        }
        self.menus = active;
        let area = self.pointer.area;
        let placed = self.pointer.placed;
        self.pointer = Pointer {
            area,
            placed,
            ..Pointer::default()
        };
        if active {
            self.pointer.at = [area[0] / 2.0, area[1] / 2.0];
        }
        self.buttons = 0;
        self.fresh = [0.0; 2];
        self.pending = [0.0; 2];
        self.fresh_wheel = 0.0;
    }

    /// the pointer as the game reads it, what happened since taken, and
    /// whether the menus are up
    fn read_pointer(&mut self) -> ([u8; POINTER_BYTES], bool) {
        let bytes = self.pointer.bytes();
        self.pointer.moved = false;
        self.pointer.left_clicks = 0;
        self.pointer.right_clicks = 0;
        self.pointer.wheel_steps = 0;
        (bytes, self.menus)
    }

    /// whether the game sees nothing of the keyboard and mouse: the
    /// controls menu is capturing, or what was held is being let go of
    fn swallowing(&self) -> bool {
        self.capture != Capture::Idle || self.settling
    }

    /// while the controls menu waits, the input it takes
    fn take(&mut self, result: u32, input: u32) {
        if matches!(self.capture, Capture::Waiting(_)) {
            self.capture = Capture::Taken(result, input, None);
        }
    }

    /// once the capture is over and nothing is held, the game sees the
    /// keyboard and mouse again
    fn settle(&mut self) {
        if self.capture == Capture::Idle && self.keys.is_empty() && self.pressed == 0 {
            self.settling = false;
        }
    }

    /// `platform_binding_capture_begin()`: the controls menu waits for
    /// an input to bind
    fn capture_begin(&mut self, now: u64) {
        self.capture = Capture::Waiting(now);
        self.settling = true;
    }

    /// `platform_binding_capture_poll`: what was taken, once
    fn capture_poll(&mut self, now: u64) -> (u32, u32) {
        match self.capture {
            Capture::Taken(result, input, _) => {
                self.capture = Capture::Idle;
                self.settle();
                (result, input)
            }
            Capture::Waiting(_) => {
                self.capture = Capture::Waiting(now);
                (CAPTURE_NONE, 0)
            }
            Capture::Idle => (CAPTURE_NONE, 0),
        }
    }

    /// a wait nobody polls is given up, and an input nobody claims dropped
    fn capture_lapse(&mut self, now: u64) {
        self.capture = match self.capture {
            Capture::Waiting(at) if now.saturating_sub(at) > CAPTURE_ABANDON_MS => Capture::Idle,
            Capture::Taken(result, input, None) => Capture::Taken(result, input, Some(now)),
            Capture::Taken(_, _, Some(at)) if now.saturating_sub(at) > CAPTURE_UNCLAIMED_MS => {
                Capture::Idle
            }
            other => other,
        };
        self.settle();
    }

    /// the screenshots asked for since the last call
    pub(crate) fn take_screenshots(&mut self) -> u32 {
        std::mem::take(&mut self.screenshots)
    }

    fn typing(&self) -> bool {
        self.keyboard_typing || self.field_typing
    }

    /// sets whether the game types on its keyboard or in a field
    fn set_typing(&mut self, keyboard: Option<bool>, field: Option<bool>) {
        let was = self.typing();
        self.keyboard_typing = keyboard.unwrap_or(self.keyboard_typing);
        self.field_typing = field.unwrap_or(self.field_typing);
        if self.typing() && !was {
            self.enter_armed = false;
        }
    }

    /// whether `code` is held, the keys being `keys`
    fn held(&self, keys: &BTreeSet<u8>, code: Code, now: u64) -> bool {
        let scrolled = now < self.press_until;
        match code {
            Code::Key(vk) => keys.contains(&vk),
            Code::Mouse(button) => 1_u16
                .checked_shl(u32::from(button))
                .is_some_and(|bit| self.buttons & bit != 0),
            Code::Wheel => scrolled,
            Code::WheelUp => scrolled && self.direction > 0,
            Code::WheelDown => scrolled && self.direction < 0,
        }
    }

    /// the actions held, a bit for each binding
    fn bound(&self, keys: &BTreeSet<u8>, bindings: &Bindings, now: u64) -> u32 {
        (0_u32..)
            .zip(bindings)
            .filter(|(_, slots)| {
                slots
                    .iter()
                    .flatten()
                    .any(|&code| self.held(keys, code, now))
            })
            .fold(0, |actions, (bit, _)| actions | (1 << bit))
    }

    /// the mouse's motion and wheel since the last poll
    fn mouse_poll(&mut self, now: u64) {
        self.unconsumed = self.unconsumed.saturating_add(1);
        if self.unconsumed > UNCONSUMED_POLLS {
            self.pending = [0.0; 2];
        }
        let [dx, dy] = std::mem::take(&mut self.fresh);
        self.pending[0] += dx;
        self.pending[1] += dy;
        if dx != 0.0 || dy != 0.0 {
            self.aimed = true;
        }
        let notches = std::mem::take(&mut self.fresh_wheel);
        if notches != 0.0 {
            self.wheel += notches;
            self.wheel_moved = now;
        }
        if !self.scrolling && self.wheel.abs() >= 1.0 {
            self.scrolling = true;
            self.direction = if self.wheel > 0.0 { 1 } else { -1 };
            self.press_until = now + WHEEL_PRESS_MS;
        } else if self.scrolling
            && now >= self.press_until
            && now - self.wheel_moved >= WHEEL_STILL_MS
        {
            self.scrolling = false;
            self.wheel = 0.0;
        }
    }

    /// the keys held, less those held over from another context
    fn keys_held_over_switch(&mut self, game: Game) -> BTreeSet<u8> {
        let context = u8::from(self.menus)
            | u8::from(self.typing()) << 1
            | u8::from(game.console) << 2
            | u8::from(game.chat_typing) << 3;
        if self.context != Some(context) {
            self.context = Some(context);
            self.held_over.clone_from(&self.keys);
        }
        self.held_over.retain(|key| self.keys.contains(key));
        self.keys.difference(&self.held_over).copied().collect()
    }

    /// in the menus: the keys as a controller's, to move about them
    fn keyboard_gamepad(&mut self, keys: &BTreeSet<u8>, pad: &mut Gamepad) {
        let held = |key| keys.contains(&key);
        let dpad = |pad: &mut Gamepad| {
            for (key, button) in [
                (VK_UP, DPAD_UP),
                (VK_DOWN, DPAD_DOWN),
                (VK_LEFT, DPAD_LEFT),
                (VK_RIGHT, DPAD_RIGHT),
            ] {
                if held(key) {
                    pad.buttons |= button;
                }
            }
        };
        if self.typing() {
            dpad(pad);
            if !held(VK_RETURN) {
                self.enter_armed = true;
            } else if self.enter_armed {
                pad.buttons |= START;
            }
            if held(VK_ESCAPE) {
                pad.analog[ANALOG_B] = ANALOG_PRESSED;
            }
            return;
        }
        let x = f32::from(i8::from(held(b'D')) - i8::from(held(b'A')));
        let y = f32::from(i8::from(held(b'W')) - i8::from(held(b'S')));
        let scale = if x != 0.0 && y != 0.0 {
            STICK_FULL * DIAGONAL
        } else {
            STICK_FULL
        };
        pad.thumbs[THUMB_LX] = (x * scale) as i16;
        pad.thumbs[THUMB_LY] = (y * scale) as i16;
        dpad(pad);
        if held(VK_F1) {
            pad.buttons |= BACK;
        }
        let x1 = self.buttons & (1 << MOUSE_X1) != 0;
        for (pressed, index) in [
            (held(VK_SPACE) || held(VK_RETURN), ANALOG_A),
            (held(VK_ESCAPE) || held(VK_BACK) || x1, ANALOG_B),
            (held(VK_DELETE) || held(b'E'), ANALOG_X),
            (held(VK_TAB), ANALOG_Y),
        ] {
            if pressed {
                pad.analog[index] = ANALOG_PRESSED;
            }
        }
    }

    /// what the chat's pad reads of the controller
    fn chat_pad_record(&mut self, pad: &Gamepad) {
        let lx = pad.thumbs[THUMB_LX];
        let ly = pad.thumbs[THUMB_LY];
        self.chat_pad = [
            (pad.buttons & DPAD_UP != 0, CHAT_PAD_UP),
            (pad.buttons & DPAD_DOWN != 0, CHAT_PAD_DOWN),
            (pad.buttons & DPAD_LEFT != 0, CHAT_PAD_LEFT),
            (pad.buttons & DPAD_RIGHT != 0, CHAT_PAD_RIGHT),
            (pad.buttons & START != 0, CHAT_PAD_START),
            (pad.analog[ANALOG_A] >= CHAT_PRESSED, CHAT_PAD_A),
            (pad.analog[ANALOG_B] >= CHAT_PRESSED, CHAT_PAD_B),
            (pad.analog[ANALOG_X] >= CHAT_PRESSED, CHAT_PAD_X),
            (pad.analog[ANALOG_Y] >= CHAT_PRESSED, CHAT_PAD_Y),
            (ly > CHAT_PUSHED, CHAT_PAD_STICK_UP),
            (ly < -CHAT_PUSHED, CHAT_PAD_STICK_DOWN),
            (lx < -CHAT_PUSHED, CHAT_PAD_STICK_LEFT),
            (lx > CHAT_PUSHED, CHAT_PAD_STICK_RIGHT),
        ]
        .into_iter()
        .filter(|&(pressed, _)| pressed)
        .fold(0, |bits, (_, bit)| bits | (1 << bit));
    }

    /// while the chat's on-screen keyboard is open it takes the controller;
    /// what it held is let go of before the game sees it again
    fn chat_pad_hold_over(&mut self, pad: &mut Gamepad, keyboard_open: bool) {
        if keyboard_open {
            self.chat_held_buttons = pad.buttons;
            self.chat_held_analog = (0_u8..)
                .zip(pad.analog)
                .filter(|&(_, value)| value != 0)
                .fold(0, |bits, (index, _)| bits | (1 << index));
            *pad = Gamepad::default();
            return;
        }
        self.chat_held_buttons &= pad.buttons;
        pad.buttons &= !self.chat_held_buttons;
        for (index, value) in (0_u8..).zip(pad.analog.iter_mut()) {
            if self.chat_held_analog & (1 << index) == 0 {
                continue;
            }
            if *value != 0 {
                *value = 0;
            } else {
                self.chat_held_analog &= !(1 << index);
            }
        }
    }

    /// the first controller: the keyboard and mouse, as the game is now
    fn keyboard_state(&mut self, bindings: &Bindings, game: Game, now: u64) -> Gamepad {
        let mut pad = Gamepad::default();
        self.actions = 0;
        self.capture_lapse(now);
        if self.swallowing() {
            self.chat_pad = 0;
            return pad;
        }
        self.mouse_poll(now);
        let keys = self.keys_held_over_switch(game);
        let console_active = game.console || game.chat_typing;
        let held = if console_active {
            0
        } else {
            self.bound(&keys, bindings, now)
        };
        let screenshot = held & (1 << SCREENSHOT) != 0;
        if screenshot && !self.screenshot_down {
            self.screenshots = self.screenshots.saturating_add(1);
        }
        self.screenshot_down = screenshot;
        if !console_active {
            if self.menus {
                self.keyboard_gamepad(&keys, &mut pad);
            } else {
                if held & (1 << PAUSE) != 0 {
                    pad.buttons |= START;
                }
                if held & (1 << SCOREBOARD) != 0 {
                    pad.buttons |= BACK;
                }
                self.actions = held;
            }
        }
        self.chat_pad_record(&pad);
        self.chat_pad_hold_over(&mut pad, game.chat_keyboard);
        pad
    }
}

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "XInitDevices" => nothing,
        "XGetDeviceChanges" => get_device_changes,
        "XInputOpen" => open,
        "XInputClose" => close,
        "XInputGetState" => get_state,
        "XInputSetState" => set_state,
        "XInputDebugInitKeyboardQueue" => zero,
        "XInputDebugGetKeystroke" => get_keystroke,
        "halo_keyboard_actions" => keyboard_actions,
        "halo_linux_mouse_look" => mouse_look,
        "halo_linux_mouse_aiming" => mouse_aiming,
        "halo_push_to_talk_held" => push_to_talk_held,
        "halo_chat_pad_buttons" => chat_pad_buttons,
        "halo_ui_pointer_update" => ui_pointer_update,
        "host_menus_active" => menus_active,
        "host_ui_pointer_read" => ui_pointer_read,
        "halo_linux_touch_look" => touch_look,
        "halo_linux_touch_move" => touch_move,
        "halo_chat_touch_layout" | "halo_linux_touch_aiming" | "halo_scoreboard_pointer_update" => {
            zero
        }
        "platform_text_typing" => text_typing,
        "halo_input_name" => input_name,
        "platform_binding_capture_begin" => capture_begin,
        "platform_binding_capture_poll" => capture_poll,
        "platform_text_field" => text_field,
        _ => return None,
    };
    Some(handler)
}

/// the milliseconds since the game was loaded
fn now(caller: &Caller<'_, Record>) -> u64 {
    caller.data().shared.threads.elapsed() / 1_000_000
}

/// the kinds of device the game asks for
#[derive(PartialEq, Eq)]
enum Device {
    Gamepad,
    DebugKeyboard,
    Other,
}

/// the device whose type the game names by `table`, the address of one of
/// its `XDEVICE_TYPE_*_TABLE`s; Other if it exports no such table
fn device_of(caller: &mut Caller<'_, Record>, table: u32) -> Device {
    let mut address = |name| match caller.get_export(name).and_then(Extern::into_global) {
        Some(global) => match global.get(&mut *caller) {
            Val::I32(address) => Some(address.cast_unsigned()),
            _ => None,
        },
        None => None,
    };
    if address("XDEVICE_TYPE_GAMEPAD_TABLE") == Some(table) {
        Device::Gamepad
    } else if address("XDEVICE_TYPE_DEBUG_KEYBOARD_TABLE") == Some(table) {
        Device::DebugKeyboard
    } else {
        Device::Other
    }
}

/// whether the game's exported `name` says yes; no, if it has none
fn game_says(caller: &mut Caller<'_, Record>, name: &str) -> wasmtime::Result<bool> {
    if caller
        .get_export(name)
        .and_then(Extern::into_func)
        .is_none()
    {
        return Ok(false);
    }
    Ok(call_game(caller, name, &[])? & 0xFF != 0)
}

/// what the game is doing now, as the input reads it
fn game_now(caller: &mut Caller<'_, Record>) -> wasmtime::Result<Game> {
    Ok(Game {
        console: game_says(caller, "console_is_active")?,
        chat_typing: game_says(caller, "network_chat_typing")?,
        chat_keyboard: game_says(caller, "network_chat_keyboard_open")?,
    })
}

/// writes `bytes` at the pointer `at`, which the game passed
fn write(caller: &mut Caller<'_, Record>, at: u32, bytes: &[u8]) -> wasmtime::Result<()> {
    if with_space(caller, |space, _| space.write(at, bytes))? {
        Ok(())
    } else {
        Err(wasmtime::format_err!(
            "the pointer {at:#x} is outside the memory"
        ))
    }
}

/// the controller's port a handle names, if it is an open controller's
fn port_of(input: &Input, handle: u32) -> Option<usize> {
    let offset = handle.checked_sub(GAMEPAD_HANDLE)?;
    if offset % GAMEPAD_HANDLE_STEP != 0 {
        return None;
    }
    let port = usize::try_from(offset / GAMEPAD_HANDLE_STEP).ok()?;
    input.open.get(port).copied()?.then_some(port)
}

fn nothing(_: &mut Caller<'_, Record>, _: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    Ok(())
}

fn zero(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, 0);
    Ok(())
}

/// `XGetDeviceChanges(type, &insertions, &removals)`: the keyboard's
/// controller, and the debug keyboard, each inserted once
fn get_device_changes(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (table, insertions, removals) = (arg(params, 0), arg(params, 1), arg(params, 2));
    let device = device_of(caller, table);
    let (inserted, removed) = {
        let mut input = caller.data().input();
        match device {
            Device::Gamepad => {
                let connected = 1;
                let changes = (connected & !input.reported, input.reported & !connected);
                input.reported = connected;
                changes
            }
            Device::DebugKeyboard => (
                u32::from(!std::mem::replace(&mut input.keyboard_reported, true)),
                0,
            ),
            Device::Other => (0, 0),
        }
    };
    write(caller, insertions, &inserted.to_le_bytes())?;
    write(caller, removals, &removed.to_le_bytes())?;
    give(results, u32::from(inserted != 0 || removed != 0));
    Ok(())
}

/// `XInputOpen(type, port, slot, parameters)`: the keyboard's controller
/// and the debug keyboard, each on the first port; else NULL, with the
/// last error ERROR_DEVICE_NOT_CONNECTED. The Linux port gave the other
/// three ports to gamepads; here there are none, so they never open.
fn open(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (table, port) = (arg(params, 0), arg(params, 1));
    let device = device_of(caller, table);
    let record = caller.data();
    let handle = {
        let mut input = record.input();
        match (device, usize::try_from(port)) {
            (Device::Gamepad, Ok(0)) => {
                input.open[0] = true;
                input.packets[0] = 0;
                input.previous[0] = [0; GAMEPAD_BYTES];
                GAMEPAD_HANDLE
            }
            (Device::DebugKeyboard, Ok(0)) => {
                input.keyboard_open = true;
                KEYBOARD_HANDLE
            }
            _ => 0,
        }
    };
    if handle == 0 {
        record
            .files()
            .fail_for(record.thread, ERROR_DEVICE_NOT_CONNECTED);
    }
    give(results, handle);
    Ok(())
}

/// `XInputClose(handle)`
fn close(caller: &mut Caller<'_, Record>, params: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    let handle = arg(params, 0);
    let mut input = caller.data().input();
    if handle == KEYBOARD_HANDLE {
        input.keyboard_open = false;
    } else if let Some(port) = port_of(&input, handle) {
        input.open[port] = false;
    }
    Ok(())
}

/// `XInputGetState(handle, &state)`: the controller's state, its packet
/// number counting its changes; the first's is the keyboard and mouse
fn get_state(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (handle, state) = (arg(params, 0), arg(params, 1));
    if !with_space(caller, |space, _| space.fill(state, STATE_BYTES, 0))? {
        return Err(wasmtime::format_err!(
            "the state {state:#x} is outside the memory"
        ));
    }
    let game = game_now(caller)?;
    let now = now(caller);
    let record = caller.data();
    let bindings = bindings_of(record);
    let written = {
        let mut input = record.input();
        let Some(port) = port_of(&input, handle) else {
            give(results, ERROR_DEVICE_NOT_CONNECTED);
            return Ok(());
        };
        // (only the first port opens: the keyboard and mouse)
        let bytes = input.keyboard_state(&bindings, game, now).bytes();
        if bytes != input.previous[port] {
            input.packets[port] = input.packets[port].wrapping_add(1);
            input.previous[port] = bytes;
        }
        let mut written = input.packets[port].to_le_bytes().to_vec();
        written.extend_from_slice(&bytes);
        written
    };
    write(caller, state, &written)?;
    give(results, ERROR_SUCCESS);
    Ok(())
}

/// `XInputSetState(handle, &feedback)`: the rumble, which there is none of,
/// done at once
fn set_state(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (handle, feedback) = (arg(params, 0), arg(params, 1));
    if feedback == 0 {
        give(results, ERROR_INVALID_PARAMETER);
        return Ok(());
    }
    write(caller, feedback, &0_u32.to_le_bytes())?;
    let open = port_of(&caller.data().input(), handle).is_some();
    give(
        results,
        if open {
            ERROR_SUCCESS
        } else {
            ERROR_DEVICE_NOT_CONNECTED
        },
    );
    Ok(())
}

/// `XInputDebugGetKeystroke(&keystroke)`: the next keystroke the console
/// or typing takes (every key up, the console's key, and while the console
/// is open or the game types, the rest but Escape typing), or
/// ERROR_HANDLE_EOF when there is none
fn get_keystroke(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let keystroke = arg(params, 0);
    let game = game_now(caller)?;
    let next = {
        let mut input = caller.data().input();
        let typing = input.typing();
        std::iter::from_fn(|| input.queue.pop_front()).find(|&[vk, _, flags]| {
            flags & KEYSTROKE_KEYUP != 0
                || vk == VK_OEM_3_BACKQUOTE
                || game.console
                || game.chat_typing
                || (typing && vk != VK_ESCAPE)
        })
    };
    write(
        caller,
        keystroke,
        &next.unwrap_or_default()[..KEYSTROKE_BYTES as usize],
    )?;
    give(
        results,
        if next.is_some() {
            ERROR_SUCCESS
        } else {
            ERROR_HANDLE_EOF
        },
    );
    Ok(())
}

/// `halo_keyboard_actions(controller)`: the first's actions held
fn keyboard_actions(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let actions = if arg(params, 0) == 0 {
        caller.data().input().actions
    } else {
        0
    };
    give(results, actions);
    Ok(())
}

/// `halo_linux_mouse_look(gamepad, &yaw, &pitch)`: the first player's turn
/// by the mouse's motion since the last look, in radians; whether it moved
fn mouse_look(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (gamepad, yaw, pitch) = (arg(params, 0) & 0xFFFF, arg(params, 1), arg(params, 2));
    write(caller, yaw, &0.0_f32.to_le_bytes())?;
    write(caller, pitch, &0.0_f32.to_le_bytes())?;
    if gamepad != 0 {
        give(results, 0);
        return Ok(());
    }
    let record = caller.data();
    let mouse = mouse_of(record);
    let [x, y] = {
        let mut input = record.input();
        input.unconsumed = 0;
        std::mem::take(&mut input.pending)
    };
    if x == 0.0 && y == 0.0 {
        give(results, 0);
        return Ok(());
    }
    let turn = -x * LOOK_SCALE * mouse.sensitivity;
    let tilt = if mouse.invert { y } else { -y } * LOOK_SCALE * mouse.vertical;
    write(caller, yaw, &turn.to_le_bytes())?;
    write(caller, pitch, &tilt.to_le_bytes())?;
    give(results, 1);
    Ok(())
}

/// `halo_linux_mouse_aiming(gamepad)`: whether the mouse aims the first
/// player, and not the game's aim assist
fn mouse_aiming(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let record = caller.data();
    let aiming =
        arg(params, 0) & 0xFFFF == 0 && !mouse_of(record).aim_assist && record.input().aimed;
    give(results, u32::from(aiming));
    Ok(())
}

/// `halo_push_to_talk_held()`: whether push to talk's binding is held now
fn push_to_talk_held(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let now = now(caller);
    let record = caller.data();
    let bindings = bindings_of(record);
    let input = record.input();
    let held = bindings[PUSH_TO_TALK]
        .iter()
        .flatten()
        .any(|&code| input.held(&input.keys, code, now));
    give(results, u32::from(held));
    Ok(())
}

/// `halo_chat_pad_buttons()`: the chat pad's buttons, at the last poll
fn chat_pad_buttons(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    give(results, caller.data().input().chat_pad);
    Ok(())
}

/// `halo_ui_pointer_update(menus_active, &pointer)`: whether the menus are
/// up; the pointer is the game's Metal device's to give, through
/// `host_ui_pointer_read`
fn ui_pointer_update(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    menus_active(caller, params, &mut [])?;
    give(results, 0);
    Ok(())
}

/// `host_menus_active(active)`: the menus coming up or going
fn menus_active(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let (width, height) = caller.data().batches().drawable();
    let mut input = caller.data().input();
    input.set_area(width, height);
    input.set_menus(arg(params, 0) != 0);
    Ok(())
}

/// `host_ui_pointer_read(&pointer)`: the menus' pointer, and whether they
/// are up
fn ui_pointer_read(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (bytes, active) = caller.data().input().read_pointer();
    write(caller, arg(params, 0), &bytes)?;
    give(results, u32::from(active));
    Ok(())
}

/// `halo_linux_touch_look(gamepad, &yaw, &pitch, &gyro_yaw, &gyro_pitch)`:
/// no touch controls, so no turn
fn touch_look(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    for index in 1..5 {
        write(caller, arg(params, index), &0.0_f32.to_le_bytes())?;
    }
    give(results, 0);
    Ok(())
}

/// `halo_linux_touch_move(controller, &forward, &strafe)`: no touch
/// controls, so no movement
fn touch_move(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    for index in 1..3 {
        write(caller, arg(params, index), &0.0_f32.to_le_bytes())?;
    }
    give(results, 0);
    Ok(())
}

/// `platform_text_typing(typing)`: whether the game's keyboard types
fn text_typing(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    caller
        .data()
        .input()
        .set_typing(Some(arg(params, 0) != 0), None);
    Ok(())
}

/// `platform_text_field(typing, password)`: whether a text field types
fn text_field(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    caller
        .data()
        .input()
        .set_typing(None, Some(arg(params, 0) != 0));
    Ok(())
}

/// `halo_input_name(input, name, size)`: the input's name as the
/// `controls.*` settings write it, empty when it has none
fn input_name(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let (input, name, size) = (arg(params, 0), arg(params, 1), arg(params, 2));
    let Some(room) = usize::try_from(size)
        .ok()
        .and_then(|size| size.checked_sub(1))
    else {
        return Ok(());
    };
    let text = code_of_input(input).and_then(name_of).unwrap_or_default();
    let mut bytes: Vec<u8> = text.bytes().take(room).collect();
    bytes.push(0);
    write(caller, name, &bytes)
}

/// `platform_binding_capture_begin()`: the controls menu waits for an
/// input to bind
fn capture_begin(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let now = now(caller);
    caller.data().input().capture_begin(now);
    Ok(())
}

/// `platform_binding_capture_poll(&input)`: 1 with the input taken, 2
/// for Delete, 3 for Escape, 0 while it waits
fn capture_poll(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let now = now(caller);
    let (result, input) = caller.data().input().capture_poll(now);
    if result != CAPTURE_NONE {
        write(caller, arg(params, 0), &input.to_le_bytes())?;
    }
    give(results, result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        code_of, code_of_input, input_of, name_of, Code, Input, CAPTURE_CANCEL, CAPTURE_CLEAR,
        CAPTURE_NONE, CAPTURE_TAKEN, INPUT_WHEEL_DOWN, VK_DELETE, VK_ESCAPE,
    };

    #[test]
    fn every_input_named_is_read_back_by_its_name() {
        let codes = (0..=255)
            .map(Code::Key)
            .chain((1..=9).map(Code::Mouse))
            .chain([Code::Wheel, Code::WheelUp, Code::WheelDown]);
        let mut named = 0;
        for code in codes {
            assert_eq!(code_of_input(input_of(code)), Some(code));
            if let Some(name) = name_of(code) {
                assert_eq!(code_of(&name), Some(code), "{name}");
                named += 1;
            }
        }
        assert!(named > 90, "{named}");
        assert_eq!(name_of(Code::Key(b'W')).as_deref(), Some("W"));
        assert_eq!(name_of(Code::Key(0xA0)).as_deref(), Some("Left Shift"));
        assert_eq!(name_of(Code::Mouse(1)).as_deref(), Some("Mouse Left"));
        assert_eq!(name_of(Code::Mouse(7)).as_deref(), Some("Mouse 7"));
        assert_eq!(name_of(Code::Key(0x63)).as_deref(), Some("Keypad 3"));
        assert_eq!(name_of(Code::Key(0x7B)).as_deref(), Some("F12"));
    }

    #[test]
    fn the_capture_takes_the_first_input_then_hides_all_until_let_go() {
        let mut input = Input::default();
        input.key(b'W', b'w', 0, true);
        assert_eq!(input.capture_poll(0), (CAPTURE_NONE, 0));
        input.capture_begin(0);
        input.key(b'W', b'w', 0, false);
        assert!(input.queue.len() == 1, "W's up is hidden");
        assert_eq!(input.capture_poll(10), (CAPTURE_NONE, 0));
        input.key(b'G', b'g', 0, true);
        input.key(b'H', b'h', 0, true);
        assert_eq!(input.capture_poll(20), (CAPTURE_TAKEN, u32::from(b'G')));
        assert_eq!(input.capture_poll(30), (CAPTURE_NONE, 0));
        assert!(input.swallowing(), "G and H are still held");
        input.key(b'G', b'g', 0, false);
        input.key(b'H', b'h', 0, false);
        assert!(!input.swallowing());
        assert_eq!(input.queue.len(), 1);
    }

    #[test]
    fn escape_keeps_the_binding_delete_clears_it_and_the_mouse_binds() {
        let mut input = Input::default();
        for (press, expected) in [
            (VK_ESCAPE, (CAPTURE_CANCEL, u32::from(VK_ESCAPE))),
            (VK_DELETE, (CAPTURE_CLEAR, u32::from(VK_DELETE))),
        ] {
            input.capture_begin(0);
            input.key(press, 0, 0, true);
            input.key(press, 0, 0, false);
            assert_eq!(input.capture_poll(1), expected);
        }
        input.capture_begin(0);
        input.button(4, true);
        assert_eq!(input.capture_poll(1), (CAPTURE_TAKEN, 260));
        input.button(4, false);
        input.capture_begin(0);
        input.wheel(-1.0);
        assert_eq!(input.capture_poll(1), (CAPTURE_TAKEN, INPUT_WHEEL_DOWN));
        assert!(!input.swallowing());
    }

    #[test]
    fn a_capture_nobody_polls_or_claims_lapses() {
        let mut input = Input::default();
        input.capture_begin(0);
        input.capture_lapse(400);
        assert!(input.swallowing());
        input.capture_lapse(600);
        assert!(!input.swallowing());
        input.capture_begin(1000);
        input.key(b'G', 0, 0, true);
        input.key(b'G', 0, 0, false);
        input.capture_lapse(1100);
        input.capture_lapse(3000);
        assert!(input.swallowing());
        input.capture_lapse(3200);
        assert_eq!(input.capture_poll(3300), (CAPTURE_NONE, 0));
        assert!(!input.swallowing());
    }

    #[test]
    fn the_settings_name_keys_mouse_buttons_and_the_wheel() {
        assert_eq!(code_of("W"), Some(Code::Key(b'W')));
        assert_eq!(code_of(" left ctrl"), Some(Code::Key(0xA2)));
        assert_eq!(code_of("F10"), Some(Code::Key(0x79)));
        assert_eq!(code_of("1"), Some(Code::Key(b'1')));
        assert_eq!(code_of("Keypad 3"), Some(Code::Key(0x63)));
        assert_eq!(code_of("Mouse 4"), Some(Code::Mouse(4)));
        assert_eq!(code_of("Mouse 9"), Some(Code::Mouse(9)));
        assert_eq!(code_of("Mouse 10"), None);
        assert_eq!(code_of("Wheel Down"), Some(Code::WheelDown));
        assert_eq!(code_of("Keypad Enter"), Some(Code::Key(0x0D)));
        assert_eq!(code_of("no such key"), None);
    }
}
