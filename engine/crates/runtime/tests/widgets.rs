//! The plugins' reader of the game's widget tree
//! (`opence_runtime::widgets`): the screens interface/ui_widget.c has open,
//! read from memory laid out as game.wasm lays out
//! `ui_widget_globals_storage` and `struct widget_instance`, untrusted, so
//! that a bad pointer, a loop or a cut memory is answered, never a panic
//! or a read outside.

use std::time::{Duration, Instant};

use opence_runtime::plugins::View;
use opence_runtime::widgets::{List, WidgetType, MAXIMUM_WIDGETS};

/// where the widget globals are
const GLOBALS: u32 = 0x1000;
/// the screen, its two children (a text box and a spinner list, the focus
/// on the list) and the list's own child
const SCREEN: u32 = 0x2000;
const TITLE: u32 = 0x2100;
const LIST: u32 = 0x2200;
const ITEM: u32 = 0x2300;
/// the title's text and the list's item text, UTF-16
const TITLE_TEXT: u32 = 0x3000;
const ITEM_TEXT: u32 = 0x3100;
/// the screen's name
const NAME: u32 = 0x3200;

fn put16(memory: &mut [u8], at: u32, value: u16) {
    let at = at as usize;
    memory[at..at + 2].copy_from_slice(&value.to_le_bytes());
}

fn put32(memory: &mut [u8], at: u32, value: u32) {
    let at = at as usize;
    memory[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_wide(memory: &mut [u8], at: u32, text: &str) {
    for (index, unit) in text.encode_utf16().chain([0]).enumerate() {
        put16(memory, at + 2 * index as u32, unit);
    }
}

/// a widget of `kind` at `at`, its definition `definition`, visible, its
/// alpha 1
fn widget(memory: &mut [u8], at: u32, definition: u32, kind: u16) {
    put32(memory, at, definition);
    put16(memory, at + 0x0E, kind);
    memory[at as usize + 0x10] = 1;
    put32(memory, at + 0x24, 1.0f32.to_bits());
}

/// player 0's main menu: SCREEN ("main menu") holds TITLE ("Campaign")
/// then LIST (3 items, the second selected), LIST holds ITEM; the
/// screen's focus is on LIST; TITLE has been moved 10 right and 4 up
fn main_menu(memory: &mut [u8]) {
    put32(memory, GLOBALS + 0x800, SCREEN);
    memory[(GLOBALS + 0x868) as usize] = 1;
    widget(memory, SCREEN, 7, 0);
    put32(memory, SCREEN + 0x04, NAME);
    memory[NAME as usize..NAME as usize + 10].copy_from_slice(b"main menu\0");
    put32(memory, SCREEN + 0x34, TITLE);
    put32(memory, SCREEN + 0x38, LIST);
    widget(memory, TITLE, 8, 1);
    put16(memory, TITLE + 0x0A, 10);
    put16(memory, TITLE + 0x0C, (-4i16).cast_unsigned());
    put32(memory, TITLE + 0x2C, LIST);
    put32(memory, TITLE + 0x3C, TITLE_TEXT);
    put16(memory, TITLE + 0x40, 2);
    put_wide(memory, TITLE_TEXT, "Campaign");
    widget(memory, LIST, 9, 2);
    put32(memory, LIST + 0x34, ITEM);
    put32(memory, LIST + 0x38, ITEM);
    put16(memory, LIST + 0x3C, 1);
    put16(memory, LIST + 0x44, 3);
    put32(memory, LIST + 0x4C, ITEM_TEXT);
    put_wide(memory, ITEM_TEXT, "Solo – Normal");
    widget(memory, ITEM, 10, 0);
    memory[ITEM as usize + 0x12] = 1;
}

fn with_memory<R>(build: impl FnOnce(&mut [u8]), read: impl FnOnce(&View<'_>) -> R) -> R {
    let mut memory = vec![0; 0x4000];
    build(&mut memory);
    View::over(&mut memory, read)
}

#[test]
fn a_screen_is_read_with_each_widget_before_its_children() {
    with_memory(main_menu, |view| {
        let tree = view.widget_tree(GLOBALS, 0);
        assert!(tree.is_some(), "no tree");
        let tree = tree.unwrap_or_default();
        let addresses: Vec<u32> = tree.widgets.iter().map(|w| w.address).collect();
        assert_eq!(addresses, [SCREEN, TITLE, LIST, ITEM]);
        let parents: Vec<Option<usize>> = tree.widgets.iter().map(|w| w.parent).collect();
        assert_eq!(parents, [None, Some(0), Some(0), Some(2)]);
        let depths: Vec<u16> = tree.widgets.iter().map(|w| w.depth).collect();
        assert_eq!(depths, [0, 1, 1, 2]);

        let screen = &tree.widgets[0];
        assert_eq!(screen.name.as_deref(), Some("main menu"));
        assert_eq!((screen.definition, screen.kind), (7, WidgetType::Container));
        let title = &tree.widgets[1];
        assert_eq!(title.kind, WidgetType::TextBox);
        assert_eq!(title.text.as_deref(), Some("Campaign"));
        assert_eq!(title.string_list_index, Some(2));
        assert_eq!(title.offset, [10, -4]);
        assert!(title.visible && !title.disabled && !title.focused);
        assert_eq!(title.alpha, 1.0);
        let list = &tree.widgets[2];
        assert_eq!(
            list.list,
            Some(List {
                selected: 1,
                items: 3,
                item_text: Some("Solo – Normal".to_string()),
            })
        );
        assert!(list.focused);
        assert!(tree.widgets[3].disabled);
        assert_eq!(view.at_the_main_menu(GLOBALS), Some(true));
    });
}

#[test]
fn the_focus_is_the_deepest_widget_each_parent_has_it_on() {
    with_memory(main_menu, |view| {
        let tree = view.widget_tree(GLOBALS, 0);
        assert!(tree.is_some(), "no tree");
        let tree = tree.unwrap_or_default();
        assert_eq!(tree.focus().map(|w| w.address), Some(ITEM));
    });
    // with the screen's focus on nothing, it is the screen's
    with_memory(
        |memory| {
            main_menu(memory);
            put32(memory, SCREEN + 0x38, 0);
        },
        |view| {
            let tree = view.widget_tree(GLOBALS, 0);
            assert!(tree.is_some(), "no tree");
            let tree = tree.unwrap_or_default();
            assert_eq!(tree.focus().map(|w| w.address), Some(SCREEN));
        },
    );
}

#[test]
fn a_player_with_no_screen_open_has_an_empty_tree() {
    with_memory(main_menu, |view| {
        let tree = view.widget_tree(GLOBALS, 1);
        assert!(tree.is_some(), "no tree");
        let tree = tree.unwrap_or_default();
        assert!(tree.widgets.is_empty());
        assert_eq!(tree.focus(), None);
        // a player the game does not have
        assert_eq!(view.widget_tree(GLOBALS, 4), None);
    });
}

#[test]
fn a_screen_or_globals_outside_the_memory_is_no_tree() {
    with_memory(
        |memory| {
            main_menu(memory);
            put32(memory, GLOBALS + 0x800, 0x3FD0);
        },
        |view| assert_eq!(view.widget_tree(GLOBALS, 0), None),
    );
    with_memory(main_menu, |view| {
        assert_eq!(view.widget_tree(0x3900, 0), None);
        assert_eq!(view.widget_tree(u32::MAX - 0x10, 0), None);
        assert_eq!(view.at_the_main_menu(u32::MAX), None);
    });
}

#[test]
fn a_child_outside_the_memory_or_of_no_type_is_left_out() {
    with_memory(
        |memory| {
            main_menu(memory);
            // the title's next is past the memory's end; the item's type is
            // not one the game has
            put32(memory, TITLE + 0x2C, 0x3FF0);
            put16(memory, ITEM + 0x0E, 99);
        },
        |view| {
            let tree = view.widget_tree(GLOBALS, 0);
            assert!(tree.is_some(), "no tree");
            let tree = tree.unwrap_or_default();
            let addresses: Vec<u32> = tree.widgets.iter().map(|w| w.address).collect();
            assert_eq!(addresses, [SCREEN, TITLE]);
        },
    );
}

#[test]
fn a_loop_in_the_tree_is_read_once() {
    with_memory(
        |memory| {
            main_menu(memory);
            // the list's next is the title, and the item's child is the
            // screen
            put32(memory, LIST + 0x2C, TITLE);
            put32(memory, ITEM + 0x34, SCREEN);
        },
        |view| {
            let tree = view.widget_tree(GLOBALS, 0);
            assert!(tree.is_some(), "no tree");
            let tree = tree.unwrap_or_default();
            let addresses: Vec<u32> = tree.widgets.iter().map(|w| w.address).collect();
            assert_eq!(addresses, [SCREEN, TITLE, LIST, ITEM]);
        },
    );
}

#[test]
fn a_tree_is_read_no_further_than_its_limits() {
    // a chain of widgets each the child of the last, deeper than the limit
    let mut memory = vec![0; 0x60 * 200 + 0x2000];
    put32(&mut memory, GLOBALS + 0x800, SCREEN);
    for index in 0..200u32 {
        let at = SCREEN + index * 0x60;
        widget(&mut memory, at, index, 0);
        put32(&mut memory, at + 0x34, at + 0x60);
    }
    View::over(&mut memory, |view| {
        let tree = view.widget_tree(GLOBALS, 0);
        assert!(tree.is_some(), "no tree");
        let tree = tree.unwrap_or_default();
        assert_eq!(tree.widgets.len(), 65);
        assert!(tree.widgets.len() <= MAXIMUM_WIDGETS);
    });
}

#[test]
fn a_text_is_read_to_its_nul_or_the_memory_end() {
    with_memory(
        |memory| {
            put_wide(memory, 0x100, "Quit");
            // unpaired surrogates, and text running to the end
            put16(memory, 0x200, 0xD800);
            put16(memory, 0x202, u16::from(b'a'));
            put16(memory, 0x3FFE, u16::from(b'z'));
        },
        |view| {
            assert_eq!(view.wide_string(0x100).as_deref(), Some("Quit"));
            assert_eq!(view.wide_string(0x200).as_deref(), Some("\u{FFFD}a"));
            assert_eq!(view.wide_string(0x3FFE).as_deref(), Some("z"));
            assert_eq!(view.wide_string(0), None);
            assert_eq!(view.wide_string(0x4000), None);
        },
    );
}

struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// The main menu, changed in a few places, read for two seconds: the
/// reader always answers, and never panics or reads outside.
#[test]
fn the_reader_answers_whatever_the_memory_holds() {
    let mut random = Random(0x5EED_0F0C);
    let mut base = vec![0; 0x4000];
    main_menu(&mut base);
    let interesting = [
        0,
        1,
        u32::MAX,
        SCREEN,
        TITLE,
        LIST,
        ITEM,
        TITLE_TEXT,
        0x3FFE,
        0x4000,
    ];
    let places = [
        GLOBALS + 0x800,
        SCREEN,
        TITLE,
        LIST,
        ITEM,
        TITLE_TEXT,
        ITEM_TEXT,
    ];
    let started = Instant::now();
    let (mut cases, mut found) = (0, 0);
    while cases < 256 || started.elapsed() < Duration::from_secs(2) {
        let mut memory = base.clone();
        for _ in 0..1 + random.next() % 8 {
            let place = places[(random.next() % places.len() as u64) as usize];
            let at = (place + (random.next() % 0x58) as u32) & !1;
            match random.next() % 3 {
                0 => memory[at as usize] ^= 1 << (random.next() % 8),
                1 => put32(
                    &mut memory,
                    at & !3,
                    interesting[(random.next() % 10) as usize],
                ),
                _ => memory[at as usize] = random.next() as u8,
            }
        }
        let cut = 0x2000 + (random.next() % 0x2000) as usize;
        memory.truncate(cut);
        found += opence_runtime::views::walk(&memory);
        View::over(&mut memory, |view| {
            for player in 0..4 {
                if let Some(tree) = view.widget_tree(GLOBALS, player) {
                    found += tree.widgets.len() + usize::from(tree.focus().is_some());
                }
            }
        });
        cases += 1;
    }
    println!("{cases} cases, {found} found");
    assert!(found > 0);
}
