//! The plugins' reader of the game's live widget tree: the screens the
//! original's UI code (interface/ui_widget.c) has open for each local
//! player, each widget as `struct widget_instance` lays it out in
//! game.wasm, reached from `ui_widget_globals_storage`'s `active_widgets`.
//!
//! What a widget looks like (its bounds, its bitmaps, its font) is in its
//! definition, the `DeLa` tag `definition` names, which the importer has
//! read from the map already; the tree says which are open, where they
//! have been moved, which has the focus and what text the game has given
//! them. As with every view, it is untrusted: each pointer is checked
//! against the memory, the walk stops at a widget it has seen, and it
//! reads at most [`MAXIMUM_WIDGETS`] widgets and [`MAXIMUM_DEPTH`] levels.

use std::collections::BTreeSet;

use crate::plugins::View;

/// The name game.wasm exports `ui_widget_globals_storage` by
/// (tools/engine_wasm.py's EXPORTS).
pub const WIDGET_GLOBALS_EXPORT: &str = "ui_widget_globals_storage";
/// `MAXIMUM_NUMBER_OF_LOCAL_PLAYERS`
pub const LOCAL_PLAYERS: u32 = 4;
/// The most widgets read from one player's tree: the widget memory pool's
/// blocks (`MAXIMUM_WIDGET_MEMORY_POOL_BLOCKS`).
pub const MAXIMUM_WIDGETS: usize = 4096;
/// The deepest a widget is read below its screen.
pub const MAXIMUM_DEPTH: u16 = 64;
/// The longest text read from a widget, in UTF-16 units.
pub const MAXIMUM_TEXT: u32 = 1024;
/// The longest widget name read.
const MAXIMUM_NAME: u32 = 64;

/// `offsetof(struct ui_widget_bss_prefix, widget_globals)`, where
/// `active_widgets[]` starts
const ACTIVE_WIDGETS: u32 = 0x800;
/// `offsetof(struct ui_widget_bss_prefix, we_are_at_the_main_menu)`
const AT_THE_MAIN_MENU: u32 = 0x868;
/// `sizeof(struct widget_instance)`
const WIDGET_SIZE: u32 = 0x58;

/// `struct widget_instance`'s fields, at their offsets
mod field {
    pub const DEFINITION: u32 = 0x00;
    pub const NAME: u32 = 0x04;
    pub const LOCAL_PLAYER: u32 = 0x08;
    pub const HORIZONTAL_OFFSET: u32 = 0x0A;
    pub const VERTICAL_OFFSET: u32 = 0x0C;
    pub const TYPE: u32 = 0x0E;
    pub const VISIBLE: u32 = 0x10;
    pub const DISABLED: u32 = 0x12;
    pub const ALPHA: u32 = 0x24;
    pub const NEXT: u32 = 0x2C;
    pub const CHILD: u32 = 0x34;
    pub const FOCUSED_CHILD: u32 = 0x38;
    /// `parameters.text_box.text`
    pub const TEXT: u32 = 0x3C;
    /// `parameters.text_box.string_list_index`
    pub const STRING_LIST_INDEX: u32 = 0x40;
    /// `parameters.list.selected_index`
    pub const SELECTED: u32 = 0x3C;
    /// `parameters.list.number_of_items`
    pub const ITEMS: u32 = 0x44;
    /// `parameters.list.extended_description`
    pub const DESCRIPTION: u32 = 0x48;
    /// `parameters.list.item_text`
    pub const ITEM_TEXT: u32 = 0x4C;
    /// `animation.current_frame_index`
    pub const FRAME: u32 = 0x50;
}

/// A widget's type, `_ui_widget_type_*` (ui_widget.c).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    /// `_ui_widget_type_container`
    Container,
    /// `_ui_widget_type_text_box`
    TextBox,
    /// `_ui_widget_type_spinner_list`
    SpinnerList,
    /// `_ui_widget_type_column_list`
    ColumnList,
    /// `_ui_widget_type_game_model`
    GameModel,
    /// `_ui_widget_type_movie`
    Movie,
    /// `_ui_widget_type_custom`
    Custom,
}

impl WidgetType {
    /// the type the game numbers `value`, if it is one
    fn of(value: u16) -> Option<Self> {
        Some(match value {
            0 => Self::Container,
            1 => Self::TextBox,
            2 => Self::SpinnerList,
            3 => Self::ColumnList,
            4 => Self::GameModel,
            5 => Self::Movie,
            6 => Self::Custom,
            _ => return None,
        })
    }

    /// whether its parameters are a list's
    fn is_list(self) -> bool {
        matches!(self, Self::SpinnerList | Self::ColumnList)
    }
}

/// What a list widget holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List {
    /// the item selected, `NONE` as -1
    pub selected: i16,
    /// how many items it has
    pub items: u16,
    /// the text the game has given its items, if any
    pub item_text: Option<String>,
}

/// One open widget, as the game has it now.
#[derive(Debug, Clone, PartialEq)]
pub struct Widget {
    /// where it is in the game's memory
    pub address: u32,
    /// how far below its screen it is: 0 for the screen itself
    pub depth: u16,
    /// the place, in the tree's order, of its parent; `None` for the
    /// screen
    pub parent: Option<usize>,
    /// its definition, the `DeLa` tag's index
    pub definition: u32,
    /// its name, if it has one
    pub name: Option<String>,
    /// its type
    pub kind: WidgetType,
    /// the local player it is for
    pub local_player: i16,
    /// how far it has been moved from its definition's bounds,
    /// horizontally and vertically, in 640×480 units
    pub offset: [i16; 2],
    /// whether it is drawn
    pub visible: bool,
    /// whether it is greyed out
    pub disabled: bool,
    /// its alpha, 0 to 1
    pub alpha: f32,
    /// whether its parent's focus is on it
    pub focused: bool,
    /// a text box's text, if the game has given it one
    pub text: Option<String>,
    /// a text box's string, from its definition's string list; `NONE` as
    /// -1
    pub string_list_index: Option<i16>,
    /// a list's items
    pub list: Option<List>,
    /// its animation's current frame
    pub frame: i16,
}

/// One local player's open widgets, its screen first and the rest in the
/// order the game draws them: each widget, then its children.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WidgetTree {
    /// the widgets, each before its children
    pub widgets: Vec<Widget>,
}

impl WidgetTree {
    /// The widget the player's focus is on: the deepest one each of whose
    /// ancestors has the focus on it.
    #[must_use]
    pub fn focus(&self) -> Option<&Widget> {
        // (each child is after its parent, so the walk only goes forward)
        let mut at = 0;
        loop {
            let next = self
                .widgets
                .iter()
                .position(|widget| widget.focused && widget.parent == Some(at));
            match next {
                Some(next) => at = next,
                None => return self.widgets.get(at),
            }
        }
    }
}

impl View<'_> {
    /// Whether the game says it is at the main menu, read from the widget
    /// globals at `globals`.
    #[must_use]
    pub fn at_the_main_menu(&self, globals: u32) -> Option<bool> {
        let at = globals.checked_add(AT_THE_MAIN_MENU)?;
        Some(self.bytes(at, 1)?.first().copied()? != 0)
    }

    /// The open widgets of `player`'s screen, read from the widget globals
    /// at `globals` (the address game.wasm exports as
    /// [`WIDGET_GLOBALS_EXPORT`]): an empty tree if it has none open, or
    /// `None` if the globals or the screen are not in the memory.
    #[must_use]
    pub fn widget_tree(&self, globals: u32, player: u32) -> Option<WidgetTree> {
        if player >= LOCAL_PLAYERS {
            return None;
        }
        let slot = globals
            .checked_add(ACTIVE_WIDGETS)?
            .checked_add(player.checked_mul(4)?)?;
        let screen = self.u32(slot)?;
        let mut tree = WidgetTree::default();
        if screen == 0 {
            return Some(tree);
        }
        let mut seen = BTreeSet::new();
        // (each widget to read, its depth, its parent's place and whether
        // its parent's focus is on it; siblings are pushed in reverse so
        // they are read in order)
        let mut stack = vec![(screen, 0u16, None::<usize>, false)];
        while let Some((address, depth, parent, focused)) = stack.pop() {
            if tree.widgets.len() == MAXIMUM_WIDGETS || !seen.insert(address) {
                continue;
            }
            let Some(widget) = self.widget(address, depth, parent, focused) else {
                // (a screen not in the memory is no tree; a child is skipped)
                parent?;
                continue;
            };
            let place = tree.widgets.len();
            tree.widgets.push(widget);
            if depth == MAXIMUM_DEPTH {
                continue;
            }
            let focus = self.u32(address.checked_add(field::FOCUSED_CHILD)?)?;
            let mut children = Vec::new();
            let mut child = self.u32(address.checked_add(field::CHILD)?)?;
            while child != 0 && children.len() < MAXIMUM_WIDGETS && !children.contains(&child) {
                children.push(child);
                match child
                    .checked_add(field::NEXT)
                    .and_then(|next| self.u32(next))
                {
                    Some(next) => child = next,
                    None => break,
                }
            }
            // (a list's description is no child of anything, the game
            // draws it with the list: it is read after the list, beside it)
            let listed = tree
                .widgets
                .get(place)
                .is_some_and(|widget| widget.kind.is_list());
            if listed {
                let description = self.u32(address.checked_add(field::DESCRIPTION)?)?;
                if description != 0 && !children.contains(&description) {
                    let beside =
                        parent.map_or((depth + 1, Some(place)), |parent| (depth, Some(parent)));
                    stack.push((description, beside.0, beside.1, false));
                }
            }
            for &child in children.iter().rev() {
                stack.push((child, depth + 1, Some(place), child == focus));
            }
        }
        Some(tree)
    }

    /// the widget at `address`, if all of it is in the memory and its type
    /// is one the game has
    fn widget(
        &self,
        address: u32,
        depth: u16,
        parent: Option<usize>,
        focused: bool,
    ) -> Option<Widget> {
        let bytes = self.bytes(address, WIDGET_SIZE)?;
        let u16_at = |at: u32| -> Option<u16> {
            let at = usize::try_from(at).ok()?;
            Some(u16::from_le_bytes([*bytes.get(at)?, *bytes.get(at + 1)?]))
        };
        let u32_at = |at: u32| -> Option<u32> {
            let at = usize::try_from(at).ok()?;
            let word = bytes.get(at..at + 4)?;
            Some(u32::from_le_bytes(<[u8; 4]>::try_from(word).ok()?))
        };
        let byte_at = |at: u32| -> Option<u8> { bytes.get(usize::try_from(at).ok()?).copied() };
        let signed = |value: u16| i16::from_le_bytes(value.to_le_bytes());
        let kind = WidgetType::of(u16_at(field::TYPE)?)?;
        let name = match u32_at(field::NAME)? {
            0 => None,
            pointer => self.c_string(pointer, MAXIMUM_NAME),
        };
        let (text, string_list_index, list) = if kind == WidgetType::TextBox {
            (
                self.wide_string(u32_at(field::TEXT)?),
                Some(signed(u16_at(field::STRING_LIST_INDEX)?)),
                None,
            )
        } else if kind.is_list() {
            let list = List {
                selected: signed(u16_at(field::SELECTED)?),
                items: u16_at(field::ITEMS)?,
                item_text: self.wide_string(u32_at(field::ITEM_TEXT)?),
            };
            (None, None, Some(list))
        } else {
            (None, None, None)
        };
        let alpha = f32::from_bits(u32_at(field::ALPHA)?);
        Some(Widget {
            address,
            depth,
            parent,
            definition: u32_at(field::DEFINITION)?,
            name,
            kind,
            local_player: signed(u16_at(field::LOCAL_PLAYER)?),
            offset: [
                signed(u16_at(field::HORIZONTAL_OFFSET)?),
                signed(u16_at(field::VERTICAL_OFFSET)?),
            ],
            visible: byte_at(field::VISIBLE)? != 0,
            disabled: byte_at(field::DISABLED)? != 0,
            alpha: if alpha.is_finite() {
                alpha.clamp(0.0, 1.0)
            } else {
                0.0
            },
            focused,
            text,
            string_list_index,
            list,
            frame: signed(u16_at(field::FRAME)?),
        })
    }

    /// The NUL-terminated UTF-16 text at `address` (game.wasm's `wchar_t`
    /// is two bytes), at most [`MAXIMUM_TEXT`] units, cut at the memory's
    /// end; `None` for a null pointer or one outside the memory. Units that
    /// are not UTF-16 read as U+FFFD.
    #[must_use]
    pub fn wide_string(&self, address: u32) -> Option<String> {
        if address == 0 {
            return None;
        }
        // (an address at the memory's end reads no bytes: it is outside)
        let bytes = self
            .bytes_up_to(address, MAXIMUM_TEXT * 2)
            .filter(|bytes| !bytes.is_empty())?;
        let units: Vec<u16> = bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&pair| u16::from_le_bytes(pair))
            .take_while(|&unit| unit != 0)
            .collect();
        Some(String::from_utf16_lossy(&units))
    }
}
