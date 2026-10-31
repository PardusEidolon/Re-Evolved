//! The plugins' reader of the game's on-screen keyboard
//! (interface/virtual_keyboard.c): whether it is open, its caption and
//! the text being typed, from the `virtual_keyboard_port_view` the game
//! keeps for the Halo UI plugin, which draws the keyboard while
//! `display.redraw_menus` has the game not draw it. As with every view
//! it is untrusted: each pointer is checked against the memory, and each
//! text is read at most [`MAXIMUM_TEXT`] units.

use crate::plugins::View;
use crate::widgets::MAXIMUM_TEXT;

/// The name game.wasm exports `virtual_keyboard_port_view` by
/// (tools/engine_wasm.py's EXPORTS).
pub const KEYBOARD_EXPORT: &str = "virtual_keyboard_port_view";

/// `struct virtual_keyboard_port_view` in game.wasm: `active`, then the
/// caption, the text and the cursor, each a 32-bit `wchar_t *`.
const ACTIVE: u32 = 0;
const CAPTION: u32 = 4;
const TEXT: u32 = 8;
const CURSOR: u32 = 12;

/// The keyboard as the game has it open.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Keyboard {
    /// what the keyboard asks for ("ENTER NAME"), if it says
    pub caption: Option<String>,
    /// the text typed so far
    pub text: String,
    /// where the cursor is in `text`, in characters, if it is in it
    pub cursor: Option<usize>,
}

impl View<'_> {
    /// The keyboard read from the view at `view` (the address game.wasm
    /// exports as [`KEYBOARD_EXPORT`]): `None` when it is not open or the
    /// view is not in the memory.
    #[must_use]
    pub fn keyboard(&self, view: u32) -> Option<Keyboard> {
        if self.u32(view.checked_add(ACTIVE)?)? == 0 {
            return None;
        }
        let caption = self
            .wide_string(self.u32(view.checked_add(CAPTION)?)?)
            .filter(|caption| !caption.trim().is_empty());
        let text_at = self.u32(view.checked_add(TEXT)?)?;
        let text = self.wide_string(text_at).unwrap_or_default();
        let cursor_at = self.u32(view.checked_add(CURSOR)?)?;
        // (the cursor points into the text, two bytes a unit)
        let cursor = cursor_at
            .checked_sub(text_at)
            .filter(|bytes| text_at != 0 && bytes % 2 == 0 && bytes / 2 <= MAXIMUM_TEXT)
            .and_then(|bytes| usize::try_from(bytes / 2).ok())
            .filter(|&units| units <= text.encode_utf16().count())
            .map(|units| char::decode_utf16(text.encode_utf16().take(units)).count());
        Some(Keyboard {
            caption,
            text,
            cursor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// memory with the view at 0x100, a caption at 0x200 and a text at
    /// 0x300 with the cursor `cursor` units in
    fn memory(active: u32, cursor: u32) -> Vec<u8> {
        let mut memory = vec![0; 0x400];
        let mut put = |at: usize, value: u32| {
            memory[at..at + 4].copy_from_slice(&value.to_le_bytes());
        };
        put(0x100, active);
        put(0x104, 0x200);
        put(0x108, 0x300);
        put(0x10C, 0x300 + 2 * cursor);
        for (at, text) in [(0x200, "ENTER NAME"), (0x300, "Chief")] {
            for (unit, value) in text.encode_utf16().enumerate() {
                memory[at + 2 * unit..at + 2 * unit + 2].copy_from_slice(&value.to_le_bytes());
            }
        }
        memory
    }

    #[test]
    fn an_open_keyboard_reads_its_caption_text_and_cursor() {
        let mut memory = memory(1, 3);
        let keyboard = View::over(&mut memory, |view| view.keyboard(0x100));
        assert_eq!(
            keyboard,
            Some(Keyboard {
                caption: Some("ENTER NAME".to_owned()),
                text: "Chief".to_owned(),
                cursor: Some(3),
            })
        );
    }

    #[test]
    fn a_closed_keyboard_reads_as_none() {
        let mut memory = memory(0, 0);
        assert_eq!(View::over(&mut memory, |view| view.keyboard(0x100)), None);
    }

    #[test]
    fn a_cursor_outside_the_text_is_dropped() {
        let mut memory = memory(1, 40);
        let keyboard = View::over(&mut memory, |view| view.keyboard(0x100));
        assert_eq!(keyboard.and_then(|keyboard| keyboard.cursor), None);
    }

    #[test]
    fn a_view_outside_the_memory_reads_as_none() {
        let mut memory = memory(1, 0);
        assert_eq!(View::over(&mut memory, |view| view.keyboard(0x3FE)), None);
        assert_eq!(
            View::over(&mut memory, |view| view.keyboard(u32::MAX)),
            None
        );
    }
}
