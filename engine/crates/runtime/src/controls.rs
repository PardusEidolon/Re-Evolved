//! The app's hold on the keyboard and mouse while the game runs.
//! `halo_game_main` keeps the [`crate::Game`] busy for the whole run, so
//! the app takes a [`Controls`] first ([`crate::Game::controls`]) and hands
//! it the keys and the mouse from its own thread; the game reads them at
//! its own pace, through the input imports (the input module).
//!
//! The app's keys are macOS's virtual key codes (`NSEvent.keyCode`,
//! Carbon's `kVK_` constants), given to the game as the Windows virtual
//! keys its bindings name ([`virtual_key`]). Nothing but the keyboard and
//! the mouse is ever the game's controller: there is no entry here for a
//! pad, and the app does not listen for one.

use std::sync::{Arc, PoisonError};

use super::Shared;

/// the debug keystroke's flags (`port/include/xdk/xdk_xkbd.h`)
const FLAG_CTRL: u8 = 0x01;
const FLAG_SHIFT: u8 = 0x02;
const FLAG_ALT: u8 = 0x04;

/// The keyboard and mouse of a running game, to hand input to from any
/// thread; each clone is the same keyboard and mouse.
#[derive(Clone)]
pub struct Controls {
    shared: Arc<Shared>,
}

impl Controls {
    pub(crate) fn new(shared: Arc<Shared>) -> Self {
        Self { shared }
    }

    fn with<T>(&self, change: impl FnOnce(&mut crate::input::Input) -> T) -> T {
        change(
            &mut self
                .shared
                .input
                .lock()
                .unwrap_or_else(PoisonError::into_inner),
        )
    }

    /// A key going down or up: its Windows virtual key, its character (0
    /// if none) and its keystroke flags ([`keystroke_flags`]).
    pub fn key(&self, vk: u8, ascii: u8, flags: u8, down: bool) {
        self.with(|input| input.key(vk, ascii, flags, down));
    }

    /// The mouse moving by `dx` and `dy` pixels, right and down.
    pub fn mouse_motion(&self, dx: f32, dy: f32) {
        self.with(|input| input.motion(dx, dy));
    }

    /// A mouse button going down or up, numbered as SDL numbers them: 1
    /// the left, 2 the middle, 3 the right, 4 and on the side buttons.
    pub fn mouse_button(&self, button: u8, down: bool) {
        self.with(|input| input.button(button, down));
    }

    /// The window putting the mouse at `x` and `y`, the drawable's pixels
    /// from its top left: the menus' pointer.
    pub fn mouse_position(&self, x: f32, y: f32) {
        self.with(|input| input.position(x, y));
    }

    /// The mouse's wheel turning by `notches`, up positive.
    pub fn mouse_wheel(&self, notches: f32) {
        self.with(|input| input.wheel(notches));
    }

    /// The window's drawable changing to `width` by `height` pixels (a
    /// resize, or full screen): the game takes up its shape at the next
    /// frame (`halo_screen_commit`), and the menus' pointer its area.
    pub fn set_drawable_size(&self, width: u32, height: u32) {
        self.shared
            .batches
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .set_drawable_size(width, height);
        self.with(|input| input.set_area(width, height));
    }

    /// Where a plugin drew the menus' items, so that the pointer works on
    /// what is drawn; empty when the game's own layout is drawn.
    pub fn set_redrawn(&self, targets: Vec<crate::redrawn::Target>) {
        self.with(|input| input.set_redrawn(targets));
    }

    /// Where a plugin placed the game's own pictures in its drawing, for
    /// the game to draw them there; empty when the game's layout is drawn.
    pub fn set_placed(&self, pictures: Vec<crate::redrawn::Placed>) {
        self.with(|input| input.set_placed(pictures));
    }

    /// Whether a plugin holds the game on its map load's loop, its loading
    /// screen not yet done (the game asks through `host_ui_loading_held`).
    pub fn set_loading_held(&self, held: bool) {
        self.with(|input| input.set_loading_held(held));
    }

    /// Whether the game's menus are up: the app frees the mouse for their
    /// pointer while they are, and captures it for the look while not.
    #[must_use]
    pub fn menus_active(&self) -> bool {
        self.with(|input| input.menus())
    }

    /// The screen the window is on, `width` by `height` pixels: Video
    /// Setup offers the window sizes that fit it.
    pub fn set_screen_size(&self, width: u32, height: u32) {
        self.shared
            .display
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .set_screen(width, height);
    }

    /// The display settings the game has applied since the last call
    /// (Video Setup saved), for the app to set its window from.
    #[must_use]
    pub fn take_display(&self) -> Option<crate::DisplayRequest> {
        self.shared
            .display
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }

    /// The message the game has shown the player since the last call, its
    /// title and text (a host of another version), for the app to show.
    #[must_use]
    pub fn take_message(&self) -> Option<(String, String)> {
        self.shared
            .display
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take_message()
    }

    /// The screenshots the player has asked for since the last call.
    #[must_use]
    pub fn take_screenshots(&self) -> u32 {
        self.with(crate::input::Input::take_screenshots)
    }
}

/// The keystroke's flags for the modifiers held.
#[must_use]
pub fn keystroke_flags(control: bool, shift: bool, alt: bool) -> u8 {
    let flag = |held: bool, flag: u8| if held { flag } else { 0 };
    flag(control, FLAG_CTRL) | flag(shift, FLAG_SHIFT) | flag(alt, FLAG_ALT)
}

/// The Windows virtual key for macOS's virtual key code `mac`, or `None`
/// for a key the game has no name for. The command keys stay the Mac's.
#[must_use]
pub fn virtual_key(mac: u16) -> Option<u8> {
    let vk = match mac {
        // the letters, by where they sit on an ANSI keyboard
        0x00 => b'A',
        0x01 => b'S',
        0x02 => b'D',
        0x03 => b'F',
        0x04 => b'H',
        0x05 => b'G',
        0x06 => b'Z',
        0x07 => b'X',
        0x08 => b'C',
        0x09 => b'V',
        0x0B => b'B',
        0x0C => b'Q',
        0x0D => b'W',
        0x0E => b'E',
        0x0F => b'R',
        0x10 => b'Y',
        0x11 => b'T',
        0x1F => b'O',
        0x20 => b'U',
        0x22 => b'I',
        0x23 => b'P',
        0x25 => b'L',
        0x26 => b'J',
        0x28 => b'K',
        0x2D => b'N',
        0x2E => b'M',
        // the digits
        0x12 => b'1',
        0x13 => b'2',
        0x14 => b'3',
        0x15 => b'4',
        0x17 => b'5',
        0x16 => b'6',
        0x1A => b'7',
        0x1C => b'8',
        0x19 => b'9',
        0x1D => b'0',
        // the punctuation, as Windows' OEM keys
        0x18 => 0xBB,
        0x1B => 0xBD,
        0x1E => 0xDD,
        0x21 => 0xDB,
        0x27 => 0xDE,
        0x29 => 0xBA,
        0x2A => 0xDC,
        0x2B => 0xBC,
        0x2C => 0xBF,
        0x2F => 0xBE,
        0x32 => 0xC0,
        // the keys that do something
        0x24 | 0x4C => 0x0D,
        0x30 => 0x09,
        0x31 => 0x20,
        0x33 => 0x08,
        0x35 => 0x1B,
        0x39 => 0x14,
        0x72 => 0x2D,
        0x73 => 0x24,
        0x74 => 0x21,
        0x75 => 0x2E,
        0x77 => 0x23,
        0x79 => 0x22,
        0x7B => 0x25,
        0x7C => 0x27,
        0x7D => 0x28,
        0x7E => 0x26,
        // the modifiers, each side its own
        0x38 => 0xA0,
        0x3C => 0xA1,
        0x3B => 0xA2,
        0x3E => 0xA3,
        0x3A => 0xA4,
        0x3D => 0xA5,
        // the function keys
        0x7A => 0x70,
        0x78 => 0x71,
        0x63 => 0x72,
        0x76 => 0x73,
        0x60 => 0x74,
        0x61 => 0x75,
        0x62 => 0x76,
        0x64 => 0x77,
        0x65 => 0x78,
        0x6D => 0x79,
        0x67 => 0x7A,
        0x6F => 0x7B,
        // the keypad
        0x52 => 0x60,
        0x53 => 0x61,
        0x54 => 0x62,
        0x55 => 0x63,
        0x56 => 0x64,
        0x57 => 0x65,
        0x58 => 0x66,
        0x59 => 0x67,
        0x5B => 0x68,
        0x5C => 0x69,
        0x43 => 0x6A,
        0x45 => 0x6B,
        0x4E => 0x6D,
        0x41 => 0x6E,
        0x4B => 0x6F,
        _ => return None,
    };
    Some(vk)
}

/// The settings of a running game, to read and change from any thread
/// while `halo_game_main` keeps the [`crate::Game`] busy: the game sees a
/// change at its next `config_changes()`, as it sees the settings file's.
#[derive(Clone)]
pub struct Tuning {
    shared: Arc<Shared>,
}

impl Tuning {
    pub(crate) fn new(shared: Arc<Shared>) -> Self {
        Self { shared }
    }

    /// The setting `name` (as `section.key`) as the settings file writes
    /// it, if there is such a setting.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<String> {
        self.shared
            .settings
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .shown(name)
    }

    /// Sets the setting `name` to `text`, as the settings file would have
    /// it; false if there is no such setting or the text is too long.
    pub fn set(&self, name: &str, text: &str) -> bool {
        self.shared
            .settings
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .write(name, text)
    }
}
