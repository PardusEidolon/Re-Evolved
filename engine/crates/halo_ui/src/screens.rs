//! Each screen the game has open, laid out as its prototype draws it
//! (the Halo UI design artifact): the game's own widgets say what is on
//! it, its texts, its items and where the focus is; the layout and the
//! look are the prototype's.
//!
//! The game no longer draws the menus' text, lists or pictures, but the
//! main menu's logo and the ui map's world behind them (its ring,
//! camera and models), while the plugin draws them (`display.redraw_menus`,
//! interface/ui_widget.c), so every screen must
//! give a scene: one the plugin does not know is drawn by the generic
//! layout, each of its texts where the game would have put it.

use opence_runtime::keyboard::Keyboard;
use opence_runtime::redrawn::{
    Placed, Target, ANY_KIND, KEYBOARD_BACK_KIND, KEYBOARD_ENTER_KIND, KEYBOARD_WIDGET, VALUE_KIND,
};
use opence_runtime::widgets::{Widget, WidgetTree, WidgetType};

use crate::scene::{hex, Align, Face, Scene, Shape, Text, HEIGHT, WIDTH};

/// The prototypes' blue, the items'
const BLUE: u32 = 0x2796FE;
/// the focused item's text
const FOCUSED: u32 = 0xF2F6FA;
/// the focused item's glow
const GLOW: u32 = 0x6FC0FF;
/// the rules under the title and over the footer
const RULE: u32 = 0x1D6AB4;
/// a list's focused item's text, the descriptions and the keys' caps
const LIT: u32 = 0xCBCBCB;
/// the focused item's tab
const TAB: u32 = 0x020B24;
/// the title, the middle of the design's gradient (#8fd0ff to the blue)
const TITLE: u32 = 0x4FA9FE;
/// the top of the footer's band, below its rule
const FOOT: f32 = 1836.0;
/// A list screen's pane's left edge, where its items' tabs end.
const PANE: f32 = 231.0 + 1149.0;
/// Where a screen's message begins, under the title.
const MESSAGE: f32 = 240.0;
/// the first of the Custom Edition maps' display indices
/// (port/linux/game/custom_edition_maps.c's FIRST_DISPLAY_INDEX)
const FIRST_DISPLAY_INDEX: i16 = 0x4000;

/// The scene of the first player's open screen; an empty one if none is
/// open. It is laid over what the game still draws behind the menus:
/// the ui map's world, its ring, camera and models.
#[must_use]
pub fn scene(tree: &WidgetTree) -> Scene {
    let Some(screen) = tree.widgets.first() else {
        return Scene::default();
    };
    // (the Xbox ui map's root, and the PC menus' `main_menu.xml`)
    let main = matches!(screen.name.as_deref(), Some("the_main_menu" | "main_menu"));
    let mut scene = Scene::default();
    if main {
        main_menu(tree, &mut scene);
    } else if let Some(list) = list_of(tree, WidgetType::SpinnerList) {
        frame(&mut scene);
        let titled = title(tree, &mut scene);
        // (the profile manager has no header: it says what it is by its
        // name, as a list with none does)
        if titled.is_none() {
            title_by_name(tree, &mut scene);
        }
        profiles(tree, list, titled, &mut scene);
    } else if let Some(list) = named(tree, "solo_level_select_list") {
        frame(&mut scene);
        let titled = title(tree, &mut scene);
        campaign(tree, list, titled, &mut scene);
    } else if let Some(list) = list_of(tree, WidgetType::ColumnList) {
        frame(&mut scene);
        let titled = title(tree, &mut scene);
        // (a list with no header, as the profile's editing is, says what
        // it is by its name)
        if titled.is_none() {
            title_by_name(tree, &mut scene);
        }
        list_screen(tree, list, titled, &mut scene);
    } else {
        frame(&mut scene);
        let titled = title(tree, &mut scene);
        let mut drawn = footer(tree, &mut scene);
        drawn.extend(titled);
        rest(tree, &drawn, MESSAGE, &mut scene);
        // (a screen of nothing but its picture, a moment's message as
        // "creating profile" is, says what it is in its title)
        if titled.is_none() && scene.texts.is_empty() {
            title_by_name(tree, &mut scene);
        }
    }
    scene
}

/// The on-screen keyboard, over the screen under it: a scrim, the
/// caption, the text in a field bordered as a list's tab is with the
/// caret where the cursor is, and its keys, the keyboard's own (the game
/// still reads every key; the pad's grid of letters is not drawn).
pub fn keyboard(keyboard: &Keyboard, scene: &mut Scene) {
    const WIDE: f32 = 1800.0;
    const TALL: f32 = 156.0;
    const SIZE: f32 = 78.0;
    let x = (WIDTH - WIDE) / 2.0;
    let y = (HEIGHT - TALL) / 2.0;
    scene.shapes.push(Shape::fill(
        [0.0, 0.0, WIDTH, HEIGHT],
        [1.0 / 255.0, 5.0 / 255.0, 13.0 / 255.0, 0.72],
    ));
    if let Some(caption) = &keyboard.caption {
        scene.texts.push(Text::new(
            &caption.to_uppercase(),
            [x, y - 126.0],
            66.0,
            Face::Heavy,
            hex(TITLE, 1.0),
        ));
    }
    scene.shapes.push(Shape {
        additive: false,
        rect: [x, y, WIDE, TALL],
        fill: hex(TAB, 1.0),
        border: 9.0,
        border_color: hex(BLUE, 1.0),
        radii: [54.0; 4],
    });
    let left = x + 75.0;
    let top = y + (TALL - SIZE) / 2.0;
    scene.texts.push(Text::new(
        &keyboard.text,
        [left, top],
        SIZE,
        Face::Bold,
        hex(LIT, 1.0),
    ));
    if let Some(cursor) = keyboard.cursor {
        // (Overpass's letters are about six tenths of an em across)
        let caret = left + 0.6 * SIZE * cursor as f32;
        scene
            .shapes
            .push(Shape::fill([caret + 3.0, top, 6.0, SIZE], hex(GLOW, 1.0)));
    }
    let mut keys = x;
    let line = y + TALL + 60.0;
    for (cap, words, kind) in [
        ("ESC", "CANCEL", KEYBOARD_BACK_KIND),
        ("ENTER", "DONE", KEYBOARD_ENTER_KIND),
    ] {
        let began = keys;
        let width = 0.66 * 36.0 * cap.chars().count() as f32 + 42.0;
        scene.shapes.push(Shape {
            additive: false,
            rect: [keys, line, width, 72.0],
            fill: hex(0x0A223B, 1.0),
            border: 3.0,
            border_color: hex(BLUE, 1.0),
            radii: [15.0; 4],
        });
        let mut key = Text::new(
            cap,
            [keys + width / 2.0, line + 18.0],
            36.0,
            Face::Heavy,
            hex(LIT, 1.0),
        );
        key.align = Align::Center;
        scene.texts.push(key);
        keys += width + 24.0;
        scene.texts.push(Text::new(
            words,
            [keys, line + (72.0 - 57.0) / 2.0],
            57.0,
            Face::Heavy,
            hex(BLUE, 1.0),
        ));
        keys += 0.63 * 57.0 * words.chars().count() as f32;
        // (the legends are the keyboard's picture, not widgets: the game
        // takes a click on one by its kind)
        scene.targets.push(Target {
            rect: [began - 12.0, line - 18.0, keys - began + 24.0, 108.0],
            widget: KEYBOARD_WIDGET,
            kind,
        });
        keys += 84.0;
    }
}

/// The screen's title: its header, drawn the design's way. A header is
/// a picture of the screen's own named for it (`header_multiplayer`), or
/// a shown text box so named anywhere on it (the saving message's
/// "SAVING...", by its text). The header's place, if it had one.
fn title(tree: &WidgetTree, scene: &mut Scene) -> Option<usize> {
    let header = (0..tree.widgets.len()).find_map(|at| {
        let widget = tree.widgets.get(at)?;
        let name = widget.name.as_deref()?.strip_prefix("header_")?;
        if widget.kind == WidgetType::TextBox {
            return shown(tree, at)
                .then(|| label(widget))
                .flatten()
                .map(|text| (at, text));
        }
        let words: Vec<&str> = name.split('_').filter(|word| !word.is_empty()).collect();
        (widget.visible && widget.depth == own_depth(tree)).then(|| (at, words.join(" ")))
    });
    let (at, words) = header?;
    scene.texts.push(title_text(&words.to_uppercase()));
    Some(at)
}

/// A screen with no header titled by its own name.
fn title_by_name(tree: &WidgetTree, scene: &mut Scene) {
    let screen = tree.widgets.first().map(|screen| Widget {
        text: None,
        ..screen.clone()
    });
    if let Some(name) = screen.as_ref().and_then(label) {
        scene.texts.push(title_text(&name.replace('&', " & ")));
    }
}

/// A title's text, above the scrim's rule.
fn title_text(words: &str) -> Text {
    Text::new(words, [240.0, 42.0], 126.0, Face::Title, hex(TITLE, 1.0))
}

/// The place of the first list of `kind` among the screen's own
/// children, if it has items of its own to draw.
fn list_of(tree: &WidgetTree, kind: WidgetType) -> Option<usize> {
    tree.widgets
        .iter()
        .position(|widget| widget.kind == kind && widget.depth == own_depth(tree) && widget.visible)
        .filter(|&list| !children(tree, list).is_empty())
}

/// The depth of the screen's own children: 1, or 2 where the screen only
/// wraps another (the PC menus' `player_profile_manager` holds
/// `player_profile_manager_screen`, and its header and list are in that).
fn own_depth(tree: &WidgetTree) -> u16 {
    match children(tree, 0).as_slice() {
        [only]
            if tree
                .widgets
                .get(*only)
                .is_some_and(|widget| widget.kind == WidgetType::Container)
                && !children(tree, *only).is_empty() =>
        {
            2
        }
        _ => 1,
    }
}

/// The places of `parent`'s children, in the order the game draws them.
fn children(tree: &WidgetTree, parent: usize) -> Vec<usize> {
    (0..tree.widgets.len())
        .filter(|&at| tree.widgets.get(at).and_then(|widget| widget.parent) == Some(parent))
        .collect()
}

/// The places of `root` and everything below it.
fn subtree(tree: &WidgetTree, root: usize) -> Vec<usize> {
    let mut places = vec![root];
    let mut at = 0;
    while let Some(&parent) = places.get(at) {
        places.extend(children(tree, parent));
        at += 1;
    }
    places
}

/// What a text box says: the game's text, or, before the game has drawn
/// it once and given it its text, its name made readable.
fn label(widget: &Widget) -> Option<String> {
    // (one line: a text's own breaks, "\r\n", are the game's wrapping)
    let text = widget
        .text
        .as_deref()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if !text.is_empty() {
        return Some(text);
    }
    let name = widget.name.as_deref()?.trim_start_matches('=');
    let name = name.trim_end_matches('_').trim_end_matches("_item");
    let words: Vec<&str> = name.split('_').filter(|word| !word.is_empty()).collect();
    (!words.is_empty()).then(|| words.join(" ").to_uppercase())
}

/// The visible items of the list at `list`, with whether each has the
/// focus and is enabled.
fn items(tree: &WidgetTree, list: usize) -> Vec<(&Widget, bool)> {
    children(tree, list)
        .into_iter()
        .filter_map(|at| tree.widgets.get(at))
        .filter(|item| item.visible)
        .map(|item| (item, !item.disabled))
        .collect()
}

/// The text of an item: what it does (where its text is wrong), its own,
/// or its first text box's.
fn item_label(tree: &WidgetTree, item: &Widget) -> Option<String> {
    if let Some(label) = pc_item_label(item) {
        return Some(label);
    }
    if item.kind == WidgetType::TextBox {
        return label(item);
    }
    let at = tree.widgets.iter().position(|widget| widget == item)?;
    subtree(tree, at)
        .into_iter()
        .filter_map(|at| tree.widgets.get(at))
        .find(|widget| widget.kind == WidgetType::TextBox && widget.visible)
        .and_then(label)
}

/// A settings row's value: what its spinner shows, the spinner's own
/// text or its text box's.
fn item_spinner(tree: &WidgetTree, item: usize) -> Option<usize> {
    subtree(tree, item).into_iter().find(|&at| {
        tree.widgets
            .get(at)
            .is_some_and(|widget| widget.kind == WidgetType::SpinnerList && widget.visible)
    })
}

fn item_value(tree: &WidgetTree, item: usize) -> Option<String> {
    let spinner = item_spinner(tree, item)?;
    let own = tree
        .widgets
        .get(spinner)?
        .list
        .as_ref()
        .and_then(|list| list.item_text.as_deref())
        .map(|text| text.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|text| !text.is_empty());
    own.or_else(|| {
        subtree(tree, spinner)
            .into_iter()
            .filter_map(|at| tree.widgets.get(at))
            .find(|widget| widget.kind == WidgetType::TextBox && widget.visible)
            .and_then(|widget| widget.text.as_deref())
            .map(|text| text.split_whitespace().collect::<Vec<_>>().join(" "))
            .filter(|text| !text.is_empty())
    })
}

/// A menu item's label, from what it does, where its text is a string
/// list's and wrong (the PC main menu's profile reads "MULTIPLAYER", the
/// campaign menu's items all but one "NEW CAMPAIGN": the original's
/// labels were in the art), or none.
fn pc_item_label(item: &Widget) -> Option<String> {
    let name = item.name.as_deref()?;
    let name = name.rsplit('/').next().unwrap_or(name);
    let label = match name {
        "main_menu_item_load_camp" => "CAMPAIGN",
        "main_menu_item_multiplayer" => "MULTIPLAYER",
        "main_menu_item_profile" => "PROFILES",
        "main_menu_item_settings" => "SETTINGS",
        "main_menu_item_mods" => "MODS",
        "main_menu_item_quit_game" => "QUIT",
        "menu_item_continue" => "CONTINUE",
        "menu_item_new_campaign" => "NEW CAMPAIGN",
        "menu_item_load_game" => "LOAD GAME",
        "menu_item_back" => "BACK",
        _ => return None,
    };
    Some(label.to_owned())
}

/// The main menu: its items centred under the logo (the map's own,
/// still drawn by the game), the version at the bottom right.
fn main_menu(tree: &WidgetTree, scene: &mut Scene) {
    const SIZE: f32 = 108.0;
    const GAP: f32 = 18.0;
    if let Some(list) = list_of(tree, WidgetType::ColumnList) {
        let mut y = 1185.0;
        for (item, enabled) in items(tree, list) {
            let Some(text) = pc_item_label(item).or_else(|| item_label(tree, item)) else {
                continue;
            };
            // the menu's boxes are a word wide: the game wraps a longer
            // label ("Load Campaign") and the list's clip shows its last line
            let text = text.split_whitespace().last().unwrap_or_default();
            let mut line = Text::new(
                &text.to_uppercase(),
                [WIDTH / 2.0, y],
                SIZE,
                Face::Heavy,
                hex(BLUE, 0.82),
            );
            line.align = Align::Center;
            // (the line, and a little room each side of it)
            let wide = width(&line.text, SIZE) + 2.0 * SIZE;
            target(
                scene,
                [(WIDTH - wide) / 2.0, y - GAP / 2.0, wide, SIZE + GAP],
                item,
            );
            if item.focused {
                line.color = hex(FOCUSED, 1.0);
                line.glow = hex(GLOW, 0.6);
            } else if !enabled {
                line.color = hex(BLUE, 0.35);
            }
            scene.texts.push(line);
            y += SIZE + GAP;
        }
    }
    let mut version = Text::new(
        concat!("OpenCE ", env!("CARGO_PKG_VERSION")),
        [WIDTH - 48.0, HEIGHT - 36.0 - 36.0],
        36.0,
        Face::Semibold,
        [1.0, 1.0, 1.0, 0.38],
    );
    version.align = Align::Right;
    scene.texts.push(version);
}

/// The title's scrim and rule, and the footer's band and rule, of every
/// screen but the main menu.
fn frame(scene: &mut Scene) {
    scene.shapes.push(Shape::fill(
        [0.0, 198.0, WIDTH, FOOT - 198.0],
        [4.0 / 255.0, 19.0 / 255.0, 39.0 / 255.0, 0.74],
    ));
    scene
        .shapes
        .push(Shape::fill([0.0, 198.0, WIDTH, 6.0], hex(RULE, 1.0)));
    scene.shapes.push(Shape::fill(
        [0.0, FOOT, WIDTH, 162.0],
        [5.0 / 255.0, 4.0 / 255.0, 7.0 / 255.0, 0.62],
    ));
    scene
        .shapes
        .push(Shape::fill([0.0, FOOT, WIDTH, 6.0], hex(RULE, 1.0)));
}

/// Whether the widget at `at` is a button of a PC menu's button bar: a
/// text in a column list inside another list ("Back", "Create New").
fn bar_button(tree: &WidgetTree, at: usize) -> bool {
    let kind = |at: Option<usize>| {
        at.and_then(|at| tree.widgets.get(at))
            .map(|widget| widget.kind)
    };
    tree.widgets.get(at).is_some_and(|widget| {
        let bar = widget.parent;
        widget.visible
            && widget.kind == WidgetType::TextBox
            && kind(bar) == Some(WidgetType::ColumnList)
            && kind(bar.and_then(|bar| tree.widgets.get(bar)?.parent))
                == Some(WidgetType::ColumnList)
    })
}

/// Whether the screen's focus is on the widget at `at`: it and each of
/// its parents has its parent's focus. (A bar keeps its own focus on a
/// button while the list's is on a row, and Enter goes to the row.)
fn in_focus(tree: &WidgetTree, at: usize) -> bool {
    let mut next = Some(at);
    while let Some(at) = next {
        let Some(widget) = tree.widgets.get(at) else {
            return false;
        };
        if widget.parent.is_some() && !widget.focused {
            return false;
        }
        next = widget.parent;
    }
    true
}

/// The key a PC menu's bar button is pressed by, from its name.
fn bar_key(widget: &Widget) -> &'static str {
    let name = widget.name.as_deref().unwrap_or_default();
    if name.contains("back") || name.contains("cancel") {
        "ESC"
    } else if name.ends_with("_ok") || name.contains("accept") || name.contains("done") {
        "ENTER"
    } else {
        ""
    }
}

/// The footer: each of the screen's button keys (its `=Name` text
/// boxes, and the PC menus' button bars), with the key that presses it,
/// from the right.
fn footer(tree: &WidgetTree, scene: &mut Scene) -> Vec<usize> {
    let keys: Vec<usize> = (0..tree.widgets.len())
        .filter(|&at| {
            bar_button(tree, at)
                || tree.widgets.get(at).is_some_and(|widget| {
                    widget.visible
                        && widget.kind == WidgetType::TextBox
                        && widget
                            .name
                            .as_deref()
                            .is_some_and(|name| name.starts_with('='))
                })
        })
        .collect();
    // (the band's middle, below its rule)
    let middle = FOOT + 6.0 + (162.0 - 6.0) / 2.0;
    let mut right = WIDTH - 240.0;
    for &at in keys.iter().rev() {
        let Some(widget) = tree.widgets.get(at) else {
            continue;
        };
        let Some(text) = label(widget) else {
            continue;
        };
        // (the map's key texts begin with its "=", as their names do)
        let text = text.trim_start_matches('=').trim_start();
        // (the button picture beside it names the pad's button; the
        // keyboard's key stands for it)
        let button = at
            .checked_sub(1)
            .and_then(|before| tree.widgets.get(before))
            .and_then(|before| before.name.as_deref())
            .and_then(|name| name.chars().next());
        let cap = if bar_button(tree, at) {
            bar_key(widget)
        } else {
            match button {
                Some('a') => "ENTER",
                Some('b') => "ESC",
                Some('x') => "X",
                Some('y') => "Y",
                _ => "",
            }
        };
        let mut words = Text::new(
            &text.to_uppercase(),
            [right, middle - 57.0 / 2.0],
            57.0,
            Face::Heavy,
            hex(if in_focus(tree, at) { FOCUSED } else { BLUE }, 1.0),
        );
        words.align = Align::Right;
        scene.texts.push(words);
        let began = right;
        // (Overpass is about six tenths of an em wide a capital, and the
        // design spaces its letters by .03 em)
        right -= 0.63 * 57.0 * text.chars().count() as f32;
        if !cap.is_empty() {
            right -= 24.0;
            let width = (0.66 * 36.0 * cap.chars().count() as f32 + 42.0).max(72.0);
            scene.shapes.push(Shape {
                additive: false,
                rect: [right - width, middle - 36.0, width, 72.0],
                fill: hex(0x0A223B, 1.0),
                border: 3.0,
                border_color: hex(BLUE, 1.0),
                radii: [15.0; 4],
            });
            let mut key = Text::new(
                cap,
                [right - width / 2.0, middle - 18.0],
                36.0,
                Face::Heavy,
                hex(LIT, 1.0),
            );
            key.align = Align::Center;
            scene.texts.push(key);
            right -= width;
        }
        // (a map's key is its button's picture, the widget before its
        // text; a bar's button is its text)
        let clicked = if bar_button(tree, at) {
            Some(widget)
        } else {
            at.checked_sub(1)
                .and_then(|before| tree.widgets.get(before))
        };
        if let Some(clicked) = clicked {
            target(
                scene,
                [right - 12.0, middle - 54.0, began - right + 24.0, 108.0],
                clicked,
            );
        }
        right -= 84.0;
    }
    keys
}

/// A screen of a list: its items down the left, the focused one tabbed,
/// and the pane on the right.
fn list_screen(tree: &WidgetTree, list: usize, titled: Option<usize>, scene: &mut Scene) {
    const LEFT_OF: f32 = 231.0;
    const WIDE: f32 = 1149.0;
    const TALL: f32 = 108.0;
    const GAP: f32 = 6.0;
    /// a heading's size, and the room it takes
    const HEADING: f32 = 45.0;
    let mut y = 243.0;
    let mut option = None;
    let rows: Vec<usize> = children(tree, list)
        .into_iter()
        .filter(|&at| {
            tree.widgets
                .get(at)
                .is_some_and(|item| item.visible && item.kind != WidgetType::ColumnList)
        })
        .collect();
    // (a list longer than the room above the footer shows the rows around
    // the focused one, so that the focus never leaves the screen: Video's
    // settings)
    let room = ((FOOT - y) / (TALL + GAP)).floor().max(1.0) as usize;
    let rows: Vec<usize> = if rows.len() > room {
        let focused = rows
            .iter()
            .position(|&at| tree.widgets.get(at).is_some_and(|item| item.focused))
            .unwrap_or_default();
        let first = focused.saturating_sub(room / 2).min(rows.len() - room);
        rows.get(first..first + room).unwrap_or_default().to_vec()
    } else {
        rows
    };
    // (an item set left of the others is a heading over those below it:
    // the PC multiplayer's "Join Game" and "Create Game")
    let indent = rows
        .iter()
        .filter_map(|&at| tree.widgets.get(at))
        .map(|item| item.offset[0])
        .max()
        .unwrap_or_default();
    for at in rows {
        let Some(item) = tree.widgets.get(at) else {
            continue;
        };
        let enabled = !item.disabled;
        let Some(text) = item_label(tree, item) else {
            continue;
        };
        if item.kind == WidgetType::TextBox && item.offset[0] + 8 < indent && !item.focused {
            scene.texts.push(Text::new(
                &text.to_uppercase(),
                [LEFT_OF + 30.0, y + 24.0],
                HEADING,
                Face::Bold,
                hex(TITLE, 0.8),
            ));
            y += HEADING + 36.0;
            continue;
        }
        let mut color = hex(BLUE, if enabled { 1.0 } else { 0.35 });
        if item.focused {
            // (the tab has no right border: its border runs on under the
            // pane's rule)
            scene.shapes.push(Shape {
                additive: false,
                rect: [LEFT_OF, y, WIDE + 9.0, TALL],
                fill: hex(TAB, 1.0),
                border: 9.0,
                border_color: hex(BLUE, 1.0),
                radii: [54.0, 0.0, 0.0, 54.0],
            });
            // (its inner glow, 6 across at the left: the inside's curve
            // less the same curve 6 to the right)
            let inside = [LEFT_OF + 9.0, y + 9.0, WIDE - 9.0, TALL - 18.0];
            scene.shapes.push(Shape {
                radii: [45.0, 0.0, 0.0, 45.0],
                ..Shape::fill(inside, hex(GLOW, 0.35))
            });
            scene.shapes.push(Shape {
                radii: [45.0, 0.0, 0.0, 45.0],
                ..Shape::fill(
                    [inside[0] + 6.0, inside[1], inside[2] - 6.0, inside[3]],
                    hex(TAB, 1.0),
                )
            });
            color = hex(LIT, 1.0);
        }
        // (a line too long for its room is set smaller, on the same middle:
        // the label shares the row with its value, the value takes what is
        // left)
        let label = text.to_uppercase();
        let label_room = PANE - 60.0 - (LEFT_OF + 75.0);
        let label_size = fitted(&label, 66.0, label_room);
        scene.texts.push(Text::new(
            &label,
            [LEFT_OF + 75.0, y + (TALL - label_size) / 2.0],
            label_size,
            Face::Heavy,
            color,
        ));
        target(scene, [LEFT_OF, y, WIDE, TALL], item);
        // (a setting's value is set in the pane, the focused one's)
        if item.focused {
            if let Some(value) = item_value(tree, at) {
                option = Some((label, value.to_uppercase(), item_spinner(tree, at)));
            }
        }
        y += TALL + GAP;
    }
    scene.shapes.push(Shape::fill(
        [PANE, 204.0, WIDTH - PANE, FOOT - 204.0],
        [1.0 / 255.0, 5.0 / 255.0, 13.0 / 255.0, 0.8],
    ));
    scene.shapes.push(Shape::fill(
        [PANE, 204.0, 6.0, FOOT - 204.0],
        hex(RULE, 1.0),
    ));
    let mut drawn = subtree(tree, list);
    drawn.extend(footer(tree, scene));
    drawn.extend(titled);
    let below = rest(tree, &drawn, PANE + 75.0, scene);
    if let Some((label, value, spinner)) = option {
        let spinner = spinner.and_then(|at| tree.widgets.get(at));
        setting(scene, &label, &value, spinner, below.max(600.0));
    }
}

/// The place of the widget whose name ends in `leaf`, its last part (the
/// PC menus name theirs by their path).
fn named(tree: &WidgetTree, leaf: &str) -> Option<usize> {
    tree.widgets.iter().position(|widget| {
        widget
            .name
            .as_deref()
            .is_some_and(|name| name.rsplit('/').next() == Some(leaf))
    })
}

/// The campaign's levels (Replay Level, `solo_level_select`) as a row of
/// cards, five at a time about the focused one: each its mission's
/// number, its name and, the focused one's, what the game says of it,
/// over the map's own picture of it (`sp_levels`, which the game draws
/// where the card leaves room), with the difficulties under them. The
/// list's first item, the kind of map, is a setting under the row.
fn campaign(tree: &WidgetTree, list: usize, titled: Option<usize>, scene: &mut Scene) {
    const LEFT: f32 = 240.0;
    const TOP: f32 = 300.0;
    const WIDE: f32 = 612.0;
    const TALL: f32 = 1320.0;
    const GAP: f32 = 72.0;
    const PAD: f32 = 36.0;
    const BORDER: f32 = 9.0;
    const RADIUS: f32 = 42.0;
    const PICTURE: f32 = 390.0;
    const SHOWN: usize = 5;
    let rows: Vec<usize> = children(tree, list)
        .into_iter()
        .filter(|&at| tree.widgets.get(at).is_some_and(|item| item.visible))
        .collect();
    // (the kind of map is the first item; the button bar the last list)
    let kind = rows.iter().copied().find(|&at| {
        tree.widgets
            .get(at)
            .and_then(|item| item.name.as_deref())
            .is_some_and(|name| name.ends_with("map_kind"))
    });
    let levels: Vec<usize> = rows
        .iter()
        .copied()
        .filter(|&at| {
            Some(at) != kind
                && tree
                    .widgets
                    .get(at)
                    .is_some_and(|item| item.kind != WidgetType::ColumnList)
        })
        .collect();
    let focus = levels
        .iter()
        .position(|&at| tree.widgets.get(at).is_some_and(|item| item.focused));
    let picture = named(tree, "replay_level_right_pic").and_then(|at| tree.widgets.get(at));
    let about = named(tree, "replay_level_right_data")
        .and_then(|at| tree.widgets.get(at))
        .and_then(label);
    let first = focus
        .unwrap_or_default()
        .saturating_sub(SHOWN / 2)
        .min(levels.len().saturating_sub(SHOWN));
    let card_fill = [6.0 / 255.0, 16.0 / 255.0, 28.0 / 255.0, 0.86];
    let focused_fill = [20.0 / 255.0, 60.0 / 255.0, 110.0 / 255.0, 0.45];
    for (place, (index, &at)) in levels
        .iter()
        .enumerate()
        .skip(first)
        .take(SHOWN)
        .enumerate()
    {
        let Some(item) = tree.widgets.get(at) else {
            continue;
        };
        // (a usize of a handful of cards)
        let x = LEFT + place as f32 * (WIDE + GAP);
        let fill = if item.focused {
            focused_fill
        } else {
            card_fill
        };
        // (the card's fill leaves a hole where the picture is, for the
        // game draws it under the plugin's drawing)
        let hole = [x + PAD, TOP + PAD, WIDE - 2.0 * PAD, PICTURE];
        let below = hole[1] + hole[3];
        for (rect, radii) in [
            ([x, TOP, WIDE, PAD], [RADIUS, RADIUS, 0.0, 0.0]),
            ([x, hole[1], PAD, PICTURE], [0.0; 4]),
            ([hole[0] + hole[2], hole[1], PAD, PICTURE], [0.0; 4]),
            (
                [x, below, WIDE, TOP + TALL - below],
                [0.0, 0.0, RADIUS, RADIUS],
            ),
        ] {
            scene.shapes.push(Shape {
                radii,
                ..Shape::fill(rect, fill)
            });
        }
        scene.shapes.push(Shape {
            additive: false,
            rect: [x, TOP, WIDE, TALL],
            fill: [0.0; 4],
            border: BORDER,
            border_color: hex(if item.focused { GLOW } else { RULE }, 1.0),
            radii: [RADIUS; 4],
        });
        if let (Some(picture), Some(focus)) = (picture, focus) {
            // (the game sets the focused card's frame, which may be its
            // unknown level's; a stock level's picture is the frame of its
            // place in the list, and a Custom Edition map's is a display
            // index counted from the focused one's. A frame below 0 halts
            // the game, so such a card has no picture)
            let level = i16::try_from(index).unwrap_or(i16::MAX);
            let frame = if index == focus {
                picture.frame
            } else if picture.frame >= FIRST_DISPLAY_INDEX {
                picture
                    .frame
                    .saturating_add(level - i16::try_from(focus).unwrap_or(i16::MAX))
            } else {
                level
            };
            if frame >= 0 {
                scene.pictures.push(Placed {
                    rect: hole,
                    widget: picture.address,
                    frame,
                });
            }
        }
        let inside = WIDE - 2.0 * PAD;
        let mut y = below + 42.0;
        scene.texts.push(Text::new(
            &format!("MISSION {}", index + 1),
            [x + PAD, y],
            39.0,
            Face::Heavy,
            hex(0x7D8A99, 1.0),
        ));
        y += 39.0 + 6.0;
        let name = item_label(tree, item).unwrap_or_default().to_uppercase();
        let size = fitted(&name, 63.0, inside);
        scene.texts.push(Text::new(
            &name,
            [x + PAD, y],
            size,
            Face::Heavy,
            hex(if item.focused { GLOW } else { BLUE }, 1.0),
        ));
        y += 63.0 + 30.0;
        if item.focused {
            for line in wrapped(about.as_deref().unwrap_or_default(), 42.0, inside) {
                scene.texts.push(Text::new(
                    &line,
                    [x + PAD, y],
                    42.0,
                    Face::Bold,
                    hex(LIT, 1.0),
                ));
                y += 42.0 * 1.3;
            }
        }
        badges(scene, [x + PAD, TOP + TALL - PAD - 90.0], item.focused);
        target(scene, [x, TOP, WIDE, TALL], item);
    }
    // (the arrows say there are more levels either way)
    let middle = TOP + TALL / 2.0;
    if first > 0 {
        arrow(scene, [LEFT - 78.0, middle], false);
    }
    if first + SHOWN < levels.len() {
        arrow(scene, [LEFT + 3348.0 + 48.0, middle], true);
    }
    let mut drawn = subtree(tree, list);
    drawn.extend(footer(tree, scene));
    drawn.extend(titled);
    if let Some(about) = named(tree, "solo_level_extended_desc") {
        drawn.extend(subtree(tree, about));
    }
    if let Some(kind) = kind {
        let label = tree
            .widgets
            .get(kind)
            .and_then(|item| item_label(tree, item))
            .unwrap_or_else(|| "MAP KIND".to_owned())
            .to_uppercase();
        let value = item_value(tree, kind).unwrap_or_default().to_uppercase();
        let spinner = item_spinner(tree, kind).and_then(|at| tree.widgets.get(at));
        setting(scene, &label, &value, spinner, TOP + TALL + 42.0);
        if let Some(item) = tree.widgets.get(kind) {
            target(scene, [1626.0, TOP + TALL + 42.0, 1974.0, 102.0], item);
        }
    }
}

/// The difficulties under a card, easy to legendary, as rings with their
/// initials, from `at`, their top left.
fn badges(scene: &mut Scene, at: [f32; 2], focused: bool) {
    const SIZE: f32 = 90.0;
    const GAP: f32 = 30.0;
    for (place, initial) in ["E", "N", "H", "L"].into_iter().enumerate() {
        // (four places)
        let x = at[0] + place as f32 * (SIZE + GAP);
        scene.shapes.push(Shape {
            additive: false,
            rect: [x, at[1], SIZE, SIZE],
            fill: [0.0; 4],
            border: 6.0,
            border_color: hex(if focused { GLOW } else { RULE }, 1.0),
            radii: [SIZE / 2.0; 4],
        });
        let mut text = Text::new(
            initial,
            [x + SIZE / 2.0, at[1] + (SIZE - 39.0) / 2.0],
            39.0,
            Face::Heavy,
            hex(if focused { LIT } else { BLUE }, 1.0),
        );
        text.align = Align::Center;
        scene.texts.push(text);
    }
}

/// `words` broken into lines that fit `room` at `size`.
fn wrapped(words: &str, size: f32, room: f32) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    for word in words.split_whitespace() {
        match lines.last_mut() {
            Some(line) if width(&format!("{line} {word}"), size) <= room => {
                line.push(' ');
                line.push_str(word);
            }
            _ => lines.push(word.to_owned()),
        }
    }
    lines
}

/// The focused setting in the pane, its label at the left and its value
/// at the right between the arrows that step it back and on, its row's
/// top at `top`.
fn setting(scene: &mut Scene, label: &str, value: &str, spinner: Option<&Widget>, top: f32) {
    const LEFT: f32 = 1626.0;
    const WIDE: f32 = 1974.0;
    const TALL: f32 = 102.0;
    const PAD: f32 = 54.0;
    const SIZE: f32 = 45.0;
    /// an arrow's width, and its gap to the value
    const ARROW: f32 = 21.0;
    const GAP: f32 = 24.0;
    scene.shapes.push(Shape {
        additive: false,
        rect: [LEFT, top, WIDE, TALL],
        fill: [20.0 / 255.0, 60.0 / 255.0, 110.0 / 255.0, 0.45],
        border: 3.0,
        border_color: hex(GLOW, 0.6),
        radii: [24.0; 4],
    });
    let room = WIDE - 2.0 * PAD;
    // (the value takes up to half the row; the label what is left)
    let value_size = fitted(value, SIZE, room / 2.0 - 2.0 * (ARROW + GAP));
    let value_wide = width(value, value_size);
    let right = LEFT + WIDE - PAD - ARROW - GAP;
    let label_room = room - value_wide - 2.0 * (ARROW + GAP) - 45.0;
    let label_size = fitted(label, SIZE, label_room);
    let middle = top + TALL / 2.0;
    scene.texts.push(Text::new(
        label,
        [LEFT + PAD, middle - label_size / 2.0],
        label_size,
        Face::Heavy,
        hex(BLUE, 1.0),
    ));
    let mut line = Text::new(
        value,
        [right, middle - value_size / 2.0],
        value_size,
        Face::Bold,
        hex(LIT, 1.0),
    );
    line.align = Align::Right;
    scene.texts.push(line);
    let back = right - value_wide - GAP - ARROW;
    arrow(scene, [back, middle], false);
    arrow(scene, [right + GAP, middle], true);
    // (clicked on its left half the value steps back, on its right on,
    // as the game's value target does)
    if let Some(spinner) = spinner {
        let left = back - GAP;
        scene.targets.push(Target {
            rect: [left, top, right + GAP + ARROW + GAP - left, TALL],
            widget: spinner.address,
            kind: VALUE_KIND,
        });
    }
}

/// A setting's arrow, 21 across and 30 tall, its flat side at `at`'s
/// left (pointing right) or right (pointing left), centred on its middle.
fn arrow(scene: &mut Scene, at: [f32; 2], right: bool) {
    const WIDE: f32 = 21.0;
    const HALF: f32 = 15.0;
    const SLICE: f32 = 2.0;
    let mut dy = -HALF;
    while dy < HALF {
        let along = WIDE * (1.0 - (dy + SLICE / 2.0).abs() / HALF);
        let x = if right { at[0] } else { at[0] + WIDE - along };
        scene
            .shapes
            .push(Shape::fill([x, at[1] + dy, along, SLICE], hex(BLUE, 1.0)));
        dy += SLICE;
    }
}

/// The profiles: a card for each of the spinner's items, centred, the
/// focused one lit.
fn profiles(tree: &WidgetTree, list: usize, titled: Option<usize>, scene: &mut Scene) {
    const WIDE: f32 = 750.0;
    const TALL: f32 = 1260.0;
    const GAP: f32 = 72.0;
    const TOP: f32 = 330.0;
    /// the card's border and padding
    const INSET: f32 = 9.0 + 36.0;
    /// the picture's square
    const PICTURE: f32 = 660.0;
    let cards: Vec<usize> = children(tree, list)
        .into_iter()
        .filter(|&at| tree.widgets.get(at).is_some_and(|card| card.visible))
        .collect();
    let across = cards.len() as f32 * (WIDE + GAP) - GAP;
    let mut x = (WIDTH - across) / 2.0;
    for at in cards {
        let Some(card) = tree.widgets.get(at) else {
            continue;
        };
        let (fill, border) = if card.focused {
            (
                [20.0 / 255.0, 60.0 / 255.0, 110.0 / 255.0, 0.45],
                hex(GLOW, 1.0),
            )
        } else {
            (
                [6.0 / 255.0, 16.0 / 255.0, 28.0 / 255.0, 0.86],
                hex(RULE, 1.0),
            )
        };
        scene.shapes.push(Shape {
            additive: false,
            rect: [x, TOP, WIDE, TALL],
            fill,
            border: 9.0,
            border_color: border,
            radii: [42.0; 4],
        });
        target(scene, [x, TOP, WIDE, TALL], card);
        // (the profile's picture, the game's no longer drawn: its frame)
        scene.shapes.push(Shape {
            additive: false,
            rect: [x + INSET, TOP + INSET, WIDE - 2.0 * INSET, PICTURE],
            fill: [2.0 / 255.0, 11.0 / 255.0, 36.0 / 255.0, 0.5],
            border: 3.0,
            border_color: hex(GLOW, 0.6),
            radii: [24.0; 4],
        });
        let mut y = TOP + INSET + PICTURE + 42.0;
        for (line, below) in subtree(tree, at)
            .into_iter()
            .filter_map(|at| tree.widgets.get(at))
            .filter(|widget| widget.kind == WidgetType::TextBox && widget.visible)
            .filter_map(label)
            .enumerate()
        {
            let (size, face, color, after) = if line == 0 {
                let lit = if card.focused { FOCUSED } else { LIT };
                (66.0, Face::Heavy, hex(lit, 1.0), 24.0)
            } else {
                (45.0, Face::Bold, hex(LIT, 1.0), 0.75 * 45.0)
            };
            scene
                .texts
                .push(Text::new(&below, [x + INSET, y], size, face, color));
            y += size + after;
        }
        x += WIDE + GAP;
    }
    let mut drawn = subtree(tree, list);
    drawn.extend(footer(tree, scene));
    drawn.extend(titled);
    rest(tree, &drawn, MESSAGE, scene);
}

/// The texts of the widgets not in `drawn`, stacked from `left`.
fn rest(tree: &WidgetTree, drawn: &[usize], left: f32, scene: &mut Scene) -> f32 {
    let others: Vec<usize> = (0..tree.widgets.len())
        .filter(|at| !drawn.contains(at))
        .collect();
    generic(tree, &others, left, scene)
}

/// Each shown text box at `places` (its text, or its name before the
/// game has given it one), a line of its own for each of the game's
/// lines, stacked down from under the title at `left`. They go in the
/// order the game places them, top to bottom and then left to right:
/// the game's own places overlap once its pictures are gone, and its
/// 4:3 screen is not the design's.
fn generic(tree: &WidgetTree, places: &[usize], left: f32, scene: &mut Scene) -> f32 {
    const SIZE: f32 = 54.0;
    let mut texts: Vec<(usize, [f32; 2])> = places
        .iter()
        .filter(|&&at| {
            tree.widgets.get(at).is_some_and(|widget| {
                widget.kind == WidgetType::TextBox || spinner_value(tree, widget).is_some()
            }) && shown(tree, at)
        })
        .map(|&at| (at, placed(tree, at)))
        .collect();
    texts.sort_by(|a, b| a.1[1].total_cmp(&b.1[1]).then(a.1[0].total_cmp(&b.1[0])));
    let mut y = 300.0;
    for (at, _) in texts {
        let Some(widget) = tree.widgets.get(at) else {
            continue;
        };
        let color = if widget.focused {
            hex(FOCUSED, 1.0)
        } else {
            hex(BLUE, 0.9)
        };
        let lines: Vec<String> = match spinner_value(tree, widget).or(widget.text.as_deref()) {
            Some(text) if !text.trim().is_empty() => text
                .lines()
                .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
                .filter(|line| !line.is_empty())
                .collect(),
            _ => label(widget).into_iter().collect(),
        };
        for line in lines {
            scene
                .texts
                .push(Text::new(&line, [left, y], SIZE, Face::Bold, color));
            y += 1.4 * SIZE;
        }
        y += SIZE;
    }
    y
}

/// `widget` drawn in `rect`, for the pointer, as whichever target the
/// game has for it.
fn target(scene: &mut Scene, rect: [f32; 4], widget: &Widget) {
    scene.targets.push(Target {
        rect,
        widget: widget.address,
        kind: ANY_KIND,
    });
}

/// A capitalised line's width at `size`, as Overpass sets it: its letters'
/// advance is near 0.62 of the em (measured on the 4K frames), a space's
/// near 0.25.
fn width(line: &str, size: f32) -> f32 {
    line.chars()
        .map(|letter| if letter == ' ' { 0.25 } else { 0.62 })
        .sum::<f32>()
        * size
}

/// The size, `size` or less, at which `line` fits in `room`.
fn fitted(line: &str, size: f32, room: f32) -> f32 {
    let wide = width(line, size);
    if wide <= room || wide <= 0.0 {
        size
    } else {
        (size * room / wide).max(size / 2.0)
    }
}

/// A spinner's shown value (its item text, which the game keeps from its
/// string list as it draws it), for a spinner that is not one of another
/// spinner's items.
fn spinner_value<'a>(tree: &WidgetTree, widget: &'a Widget) -> Option<&'a str> {
    let item_of_a_spinner = widget
        .parent
        .and_then(|parent| tree.widgets.get(parent))
        .is_some_and(|parent| parent.kind == WidgetType::SpinnerList);
    if widget.kind != WidgetType::SpinnerList || item_of_a_spinner {
        return None;
    }
    widget.list.as_ref()?.item_text.as_deref()
}

/// Whether the widget at `at` and all its parents are visible.
fn shown(tree: &WidgetTree, at: usize) -> bool {
    let mut next = Some(at);
    while let Some(at) = next {
        let Some(widget) = tree.widgets.get(at) else {
            return false;
        };
        if !widget.visible {
            return false;
        }
        next = widget.parent;
    }
    true
}

/// Where the widget at `at` has been moved to, its offset and its
/// parents', in 640×480 units.
fn placed(tree: &WidgetTree, at: usize) -> [f32; 2] {
    let mut sum = [0.0, 0.0];
    let mut next = Some(at);
    while let Some(at) = next {
        let Some(widget) = tree.widgets.get(at) else {
            break;
        };
        sum[0] += f32::from(widget.offset[0]);
        sum[1] += f32::from(widget.offset[1]);
        next = widget.parent;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use opence_runtime::widgets::List;

    /// a widget of `kind` named `name` at `depth` below `parent`
    fn widget(name: &str, kind: WidgetType, depth: u16, parent: Option<usize>) -> Widget {
        Widget {
            address: 0,
            depth,
            parent,
            definition: 0,
            name: Some(name.to_owned()),
            kind,
            local_player: 0,
            offset: [0, 0],
            visible: true,
            disabled: false,
            alpha: 1.0,
            focused: false,
            text: None,
            string_list_index: Some(-1),
            list: None,
            frame: 0,
        }
    }

    fn text_box(name: &str, depth: u16, parent: usize, text: &str) -> Widget {
        let mut widget = widget(name, WidgetType::TextBox, depth, Some(parent));
        widget.text = (!text.is_empty()).then(|| text.to_owned());
        widget
    }

    /// the main menu as the ui map opens it (the dumped tree)
    fn main_menu_tree() -> WidgetTree {
        let mut widgets = vec![
            widget("the_main_menu", WidgetType::Container, 0, None),
            widget("halo_logo", WidgetType::Container, 1, Some(0)),
            widget("main_menu_select_list", WidgetType::ColumnList, 1, Some(0)),
            text_box("load_campaign_item_", 2, 2, "Load Campaign"),
            text_box("multiplayer_item", 2, 2, "Multiplayer"),
            text_box("settings_item", 2, 2, ""),
            text_box("game_demos_item", 2, 2, "Game Demos"),
            text_box("build_number", 1, 0, ""),
        ];
        widgets[2].focused = true;
        widgets[3].focused = true;
        widgets[6].visible = false;
        widgets[6].disabled = true;
        WidgetTree { widgets }
    }

    /// the PC main menu as `main_menu.xml` opens it: its items' texts are
    /// the string list's (the profile's says "MULTIPLAYER") or none
    fn pc_main_menu_tree() -> WidgetTree {
        let mut widgets = vec![
            widget("main_menu", WidgetType::Container, 0, None),
            widget("halo_logo", WidgetType::Container, 1, Some(0)),
            widget("main_menu_select_list", WidgetType::ColumnList, 1, Some(0)),
            text_box("main_menu_item_load_camp", 2, 2, "LOAD CAMPAIGN"),
            text_box("main_menu_item_multiplayer", 2, 2, "MULTIPLAYER"),
            text_box("main_menu_item_profile", 2, 2, "MULTIPLAYER"),
            text_box("main_menu_item_settings", 2, 2, "SETTINGS"),
            text_box("main_menu_item_mods", 2, 2, ""),
            text_box("main_menu_item_quit_game", 2, 2, ""),
            text_box("build_number", 1, 0, ""),
        ];
        widgets[2].focused = true;
        widgets[3].focused = true;
        widgets[9].visible = false;
        WidgetTree { widgets }
    }

    #[test]
    fn the_pc_main_menu_draws_each_item_by_what_it_does() {
        let scene = scene(&pc_main_menu_tree());
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(
            texts[..6],
            [
                "CAMPAIGN",
                "MULTIPLAYER",
                "PROFILES",
                "SETTINGS",
                "MODS",
                "QUIT"
            ]
        );
        assert!(scene.texts[0].glow[3] > 0.0);
        // (no panel or rules: the ring shows behind the items)
        assert!(scene.shapes.is_empty(), "{:?}", scene.shapes);
    }

    /// Replay Level as the PC menus open it, its third level focused: the
    /// kind of map, ten levels and the button bar, and the description
    /// with the focused level's picture at frame 2
    fn replay_level_tree(focus: usize) -> WidgetTree {
        let mut widgets = vec![
            widget(
                "main_menu/solo_level_select/solo_level_select_screen",
                WidgetType::Container,
                0,
                None,
            ),
            widget(
                "main_menu/solo_level_select/solo_level_select_list",
                WidgetType::ColumnList,
                1,
                Some(0),
            ),
            widget(
                "main_menu/new_select/list_item_0_map_kind",
                WidgetType::Container,
                2,
                Some(1),
            ),
        ];
        let mut spinner = widget("map_kind_spinner", WidgetType::SpinnerList, 3, Some(2));
        spinner.list = Some(List {
            selected: 0,
            items: 2,
            item_text: Some("Campaign".to_owned()),
        });
        widgets.push(spinner);
        let names = [
            "The Pillar of Autumn",
            "Halo",
            "Truth and Reconciliation",
            "The Silent Cartographer",
            "Assault on the Control Room",
            "343 Guilty Spark",
            "The Library",
            "Two Betrayals",
            "Keyes",
            "The Maw",
        ];
        for (index, name) in names.into_iter().enumerate() {
            let mut item = text_box(
                &format!("main_menu/new_select/list_item_{}", index + 1),
                2,
                1,
                name,
            );
            item.focused = index == focus;
            item.address = 0x100 + u32::try_from(index).unwrap_or_default();
            widgets.push(item);
        }
        widgets.push(widget(
            "main_menu/solo_level_select/solo_level_select_button_bar",
            WidgetType::ColumnList,
            2,
            Some(1),
        ));
        let desc = widgets.len();
        widgets.push(widget(
            "main_menu/solo_level_select/solo_level_extended_desc",
            WidgetType::Container,
            1,
            Some(0),
        ));
        let mut picture = widget(
            "main_menu/solo_level_select/replay_level_right_pic",
            WidgetType::Container,
            2,
            Some(desc),
        );
        picture.address = 0x900;
        picture.frame = i16::try_from(focus).unwrap_or_default();
        widgets.push(picture);
        widgets.push(text_box(
            "main_menu/solo_level_select/replay_level_right_data",
            2,
            desc,
            "Land on the ring\r\nand find the Captain.",
        ));
        WidgetTree { widgets }
    }

    #[test]
    fn replay_level_is_a_row_of_cards_over_the_maps_pictures() {
        let scene = scene(&replay_level_tree(2));
        // (five cards from the first, the third focused; each its picture,
        // the frame in the list's order)
        let frames: Vec<i16> = scene.pictures.iter().map(|picture| picture.frame).collect();
        assert_eq!(frames, [0, 1, 2, 3, 4]);
        assert!(scene.pictures.iter().all(|picture| picture.widget == 0x900));
        assert_eq!(scene.pictures[0].rect, [276.0, 336.0, 540.0, 390.0]);
        assert_eq!(scene.pictures[1].rect[0], 276.0 + 684.0);
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        for line in [
            "MISSION 1",
            "THE PILLAR OF AUTUMN",
            "MISSION 5",
            "MAP KIND",
            "CAMPAIGN",
        ] {
            assert!(texts.contains(&line), "{line} in {texts:?}");
        }
        assert!(!texts.contains(&"MISSION 6"), "{texts:?}");
        // (what the game says of the focused level, under its name only)
        assert!(
            texts
                .iter()
                .any(|text| text.starts_with("LAND ON") || text.starts_with("Land on")),
            "{texts:?}"
        );
        // (each card is a target for the pointer, the third its level)
        assert!(scene
            .targets
            .iter()
            .any(|target| target.widget == 0x102 && target.rect[0] == 240.0 + 2.0 * 684.0));
    }

    #[test]
    fn the_row_of_levels_follows_the_focus() {
        let scene = scene(&replay_level_tree(9));
        let frames: Vec<i16> = scene.pictures.iter().map(|picture| picture.frame).collect();
        assert_eq!(frames, [5, 6, 7, 8, 9]);
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert!(
            texts.contains(&"THE MAW") && !texts.contains(&"MISSION 5"),
            "{texts:?}"
        );
    }

    #[test]
    fn no_card_asks_the_game_for_a_frame_before_the_first() {
        // (as Replay Level opens on the second level, before the game has
        // set the picture's frame: a frame below 0 halts the game, in
        // bitmap_group_get_bitmap_from_sequence)
        let mut tree = replay_level_tree(1);
        if let Some(picture) = tree.widgets.iter_mut().find(|item| {
            item.name
                .as_deref()
                .is_some_and(|name| name.ends_with("right_pic"))
        }) {
            picture.frame = 0;
        }
        let frames: Vec<i16> = scene(&tree)
            .pictures
            .iter()
            .map(|picture| picture.frame)
            .collect();
        assert_eq!(frames, [0, 0, 2, 3, 4]);
    }

    #[test]
    fn a_level_not_reached_keeps_the_games_own_frame() {
        // (the game shows a level not reached as its unknown level's
        // frame, 10; the cards beside it are their own levels)
        let mut tree = replay_level_tree(3);
        if let Some(picture) = tree.widgets.iter_mut().find(|item| {
            item.name
                .as_deref()
                .is_some_and(|name| name.ends_with("right_pic"))
        }) {
            picture.frame = 10;
        }
        let frames: Vec<i16> = scene(&tree)
            .pictures
            .iter()
            .map(|picture| picture.frame)
            .collect();
        assert_eq!(frames, [1, 2, 10, 4, 5]);
    }

    #[test]
    fn the_campaign_menu_draws_each_item_by_what_it_does() {
        // (its items' texts are the main menu's string list's: all but one
        // read "NEW CAMPAIGN"; the original's labels were in the art)
        let mut widgets = vec![
            widget("campaign_menu", WidgetType::Container, 0, None),
            widget(
                "campaign_menu_select_list",
                WidgetType::ColumnList,
                1,
                Some(0),
            ),
            text_box("menu_item_continue", 2, 1, "NEW CAMPAIGN"),
            text_box("menu_item_new_campaign", 2, 1, "LOAD CAMPAIGN"),
            text_box("menu_item_load_game", 2, 1, "NEW CAMPAIGN"),
            text_box("menu_item_back", 2, 1, "NEW CAMPAIGN"),
        ];
        widgets[2].focused = true;
        let scene = scene(&WidgetTree { widgets });
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        for label in ["CONTINUE", "NEW CAMPAIGN", "LOAD GAME", "BACK"] {
            assert_eq!(
                texts.iter().filter(|text| **text == label).count(),
                1,
                "{texts:?}"
            );
        }
        assert!(!texts.contains(&"LOAD CAMPAIGN"), "{texts:?}");
    }

    #[test]
    fn no_screen_draws_nothing() {
        assert!(scene(&WidgetTree::default()).is_empty());
    }

    #[test]
    fn the_main_menu_draws_its_visible_items_in_order() {
        let scene = scene(&main_menu_tree());
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        // (the game's "Load Campaign" wraps in its box and the list's clip
        // shows only its last line; settings_item has no text yet: its name
        // stands for it)
        assert_eq!(texts[..3], ["CAMPAIGN", "MULTIPLAYER", "SETTINGS"]);
        assert!(!texts.contains(&"GAME DEMOS"), "{texts:?}");
        assert!(
            texts.iter().any(|text| text.starts_with("OpenCE ")),
            "{texts:?}"
        );
    }

    #[test]
    fn the_main_menus_focused_item_glows() {
        let scene = scene(&main_menu_tree());
        let [campaign, multiplayer] = [&scene.texts[0], &scene.texts[1]];
        assert_eq!(campaign.color, hex(FOCUSED, 1.0));
        assert!(campaign.glow[3] > 0.0);
        assert_eq!(multiplayer.color, hex(BLUE, 0.82));
        assert_eq!(multiplayer.glow[3], 0.0);
        assert!(multiplayer.y > campaign.y);
        assert_eq!(campaign.align, Align::Center);
    }

    /// the multiplayer screen (the dumped tree)
    fn multiplayer_tree() -> WidgetTree {
        let mut widgets = vec![
            widget(
                "multiplayer_type_select_screen",
                WidgetType::Container,
                0,
                None,
            ),
            widget("header_multiplayer", WidgetType::Container, 1, Some(0)),
            widget(
                "multiplayer_type_select_list",
                WidgetType::ColumnList,
                1,
                Some(0),
            ),
            text_box("multiplayer_type_coop_item", 2, 2, "Co-op"),
            text_box("multiplayer_type_split_item", 2, 2, "Split Screen"),
            text_box("multiplayer_type_conn_item", 2, 2, "System Link"),
            text_box("multiplayer_type_gametypes_item", 2, 2, "Game Types"),
            widget("button_key", WidgetType::Container, 1, Some(0)),
            widget("b_butn", WidgetType::Container, 2, Some(7)),
            text_box("=Back", 2, 7, "Back"),
            widget("a_butn", WidgetType::Container, 2, Some(7)),
            text_box("=SELECT", 2, 7, "Select"),
            widget("blueline", WidgetType::Container, 1, Some(0)),
        ];
        widgets[2].focused = true;
        widgets[4].focused = true;
        WidgetTree { widgets }
    }

    /// `tree` with each widget at an address of its own
    fn addressed(mut tree: WidgetTree) -> WidgetTree {
        for (at, widget) in tree.widgets.iter_mut().enumerate() {
            widget.address = 0x1000 + 0x40 * u32::try_from(at).unwrap_or_default();
        }
        tree
    }

    /// the target the pointer finds at `at`, in the design's units
    fn target_at(scene: &Scene, at: [f32; 2]) -> Option<u32> {
        let hit = opence_runtime::redrawn::hit(&scene.targets, at, [WIDTH, HEIGHT]);
        (hit.widget != 0).then_some(hit.widget)
    }

    #[test]
    fn each_item_drawn_is_a_target_where_it_is_drawn() {
        let tree = addressed(multiplayer_tree());
        let scene = scene(&tree);
        let address = |name: &str| {
            tree.widgets
                .iter()
                .find(|widget| widget.name.as_deref() == Some(name))
                .map(|widget| widget.address)
        };
        // (the rows from 243, 114 apart: the third is System Link)
        let third = 243.0 + 2.0 * 114.0 + 54.0;
        assert_eq!(
            target_at(&scene, [600.0, third]),
            address("multiplayer_type_conn_item")
        );
        // (nothing in the pane, right of the rows)
        assert_eq!(target_at(&scene, [2500.0, third]), None);
        // (the footer's Back is its button's picture, b_butn)
        let back = scene
            .texts
            .iter()
            .find(|text| text.text == "BACK")
            .map(|text| [text.x - 20.0, text.y + 20.0]);
        let Some(back) = back else {
            panic!("no Back in the footer");
        };
        assert_eq!(target_at(&scene, back), address("b_butn"));
    }

    #[test]
    fn the_main_menus_items_are_targets_on_their_lines() {
        let tree = addressed(main_menu_tree());
        let scene = scene(&tree);
        let Some(first) = scene.texts.first() else {
            panic!("no items");
        };
        let found = target_at(&scene, [first.x, first.y + first.size / 2.0]);
        assert!(found.is_some(), "the first item is a target");
        assert_eq!(target_at(&scene, [100.0, first.y]), None);
    }

    #[test]
    fn a_list_screen_tabs_its_focused_item() {
        let scene = scene(&multiplayer_tree());
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(texts[0], "MULTIPLAYER", "titled by its header");
        assert_eq!(
            texts[1..5],
            ["CO-OP", "SPLIT SCREEN", "SYSTEM LINK", "GAME TYPES"]
        );
        let Some(tab) = scene.shapes.iter().find(|shape| shape.border == 9.0) else {
            panic!("no focused tab in {:?}", scene.shapes);
        };
        assert_eq!(tab.radii, [54.0, 0.0, 0.0, 54.0]);
        // (the tab is on the second item, where the focus is)
        assert!((tab.rect[1] - (243.0 + 108.0 + 6.0)).abs() < 0.5);
        assert_eq!(tab.fill, hex(TAB, 1.0));
        let items: Vec<&Text> = scene
            .texts
            .iter()
            .filter(|text| text.size == 66.0)
            .collect();
        assert_eq!(items[1].color, hex(LIT, 1.0));
        assert_eq!(items[0].color, hex(BLUE, 1.0));
        assert!((items[0].x - (231.0 + 75.0)).abs() < 0.5);
    }

    #[test]
    fn the_ui_maps_ring_scene_shows_behind_every_screen() {
        // (the game draws the ui map's world, its ring, camera and models,
        // behind the menus: the plugin paints no ground of its own over it)
        for tree in [main_menu_tree(), multiplayer_tree()] {
            let drawn = scene(&tree);
            let covers = |shape: &&Shape| {
                shape.rect[0] <= 0.0
                    && shape.rect[1] <= 0.0
                    && shape.rect[2] >= WIDTH
                    && shape.rect[3] >= HEIGHT
            };
            assert!(!drawn.shapes.iter().any(|shape| shape.additive));
            assert!(!drawn.shapes.iter().any(|shape| covers(&shape)));
        }
    }

    #[test]
    fn a_screen_is_framed_as_the_design_frames_it() {
        let scene = scene(&multiplayer_tree());
        let at = |rect: [f32; 4]| scene.shapes.iter().any(|shape| shape.rect == rect);
        assert!(at([0.0, 198.0, WIDTH, 1638.0]), "the scrim");
        assert!(at([0.0, 198.0, WIDTH, 6.0]), "the title's rule");
        assert!(at([0.0, 1836.0, WIDTH, 162.0]), "the footer");
        assert!(at([0.0, 1836.0, WIDTH, 6.0]), "the footer's rule");
        assert!(at([1380.0, 204.0, WIDTH - 1380.0, 1632.0]), "the pane");
    }

    #[test]
    fn a_footer_key_has_its_cap_before_its_label() {
        let scene = scene(&multiplayer_tree());
        let find = |word: &str| scene.texts.iter().find(|text| text.text == word);
        let (Some(esc), Some(back), Some(enter), Some(select)) =
            (find("ESC"), find("BACK"), find("ENTER"), find("SELECT"))
        else {
            panic!("the footer's keys");
        };
        assert!(esc.x < back.x && enter.x < select.x);
        assert!(back.x < enter.x, "the keys in the game's order");
        assert_eq!((select.size, select.face), (57.0, Face::Heavy));
        assert_eq!(esc.color, hex(LIT, 1.0));
        assert!(scene
            .shapes
            .iter()
            .any(|shape| shape.rect[3] == 72.0 && shape.radii == [15.0; 4]));
    }

    #[test]
    fn a_list_screens_footer_names_the_keyboards_keys() {
        let scene = scene(&multiplayer_tree());
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        for word in ["BACK", "ESC", "SELECT", "ENTER"] {
            assert!(texts.contains(&word), "{word} in {texts:?}");
        }
        // (each text is drawn once: the footer's are not drawn again)
        assert_eq!(texts.iter().filter(|text| **text == "Back").count(), 0);
    }

    #[test]
    fn the_profiles_are_cards() {
        let mut widgets = vec![
            widget(
                "player_profile_select_screen",
                WidgetType::Container,
                0,
                None,
            ),
            widget("player_profiles_list", WidgetType::SpinnerList, 1, Some(0)),
            widget("left_item", WidgetType::Container, 2, Some(1)),
            text_box("player_profile_left_name", 3, 2, "Chief"),
            widget("gametype_left_data", WidgetType::Container, 3, Some(2)),
            text_box("current_level_label", 4, 4, "Level"),
            widget("center_item", WidgetType::Container, 2, Some(1)),
            text_box("player_profile_center_name", 3, 6, "Cortana"),
        ];
        widgets[1].focused = true;
        widgets[6].focused = true;
        let scene = scene(&WidgetTree { widgets });
        let cards: Vec<&Shape> = scene
            .shapes
            .iter()
            .filter(|shape| shape.rect[2] == 750.0)
            .collect();
        assert_eq!(cards.len(), 2);
        assert_eq!(
            cards[1].border_color,
            hex(GLOW, 1.0),
            "the focused card is the second"
        );
        assert_eq!(cards[0].border_color, hex(RULE, 1.0));
        // (the cards centred across the screen)
        let middle = (cards[0].rect[0] + cards[1].rect[0] + 750.0) / 2.0;
        assert!((middle - WIDTH / 2.0).abs() < 0.5, "{middle}");
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        // (titled by its name, as it has no header)
        assert_eq!(texts[1..], ["Chief", "Level", "Cortana"]);
        assert_eq!(scene.texts[0].face, Face::Title);
    }

    /// the profile's saving message, as the dump found it: its header a
    /// text box beside the message, the two placed over each other
    fn saving_tree() -> WidgetTree {
        let mut widgets = vec![
            widget(
                "new_campaign_saving_profile",
                WidgetType::Container,
                0,
                None,
            ),
            widget(
                "warning_creating_player_profile",
                WidgetType::Container,
                1,
                Some(0),
            ),
            text_box(
                "warning_creating_plyr_prf_text",
                2,
                1,
                "Creating player profile...\r\nplease don't turn off",
            ),
            text_box("header_saving", 2, 1, "SAVING..."),
            text_box("hidden", 2, 1, "Not drawn"),
        ];
        widgets[2].offset = [192, 176];
        widgets[3].offset = [201, 180];
        widgets[4].visible = false;
        WidgetTree { widgets }
    }

    #[test]
    fn another_screens_header_text_is_its_title() {
        let scene = scene(&saving_tree());
        let title = &scene.texts[0];
        assert_eq!(title.text, "SAVING...");
        assert_eq!(title.face, Face::Title);
        assert!(title.y < 198.0, "in the title's scrim");
    }

    #[test]
    fn another_screens_texts_are_stacked_line_by_line_in_the_pane() {
        let scene = scene(&saving_tree());
        let body: Vec<&Text> = scene.texts.iter().skip(1).collect();
        let lines: Vec<&str> = body.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(
            lines,
            ["Creating player profile...", "please don't turn off"]
        );
        for pair in body.windows(2) {
            assert!(pair[1].y >= pair[0].y + pair[0].size, "{lines:?} overlap");
        }
        assert!(body.iter().all(|text| text.y > 204.0 && text.y < FOOT));
        // (framed, as every screen but the main menu)
        assert!(scene.shapes.len() >= 4, "{:?}", scene.shapes);
    }

    #[test]
    fn a_list_screens_other_texts_go_in_its_pane() {
        let mut widgets = multiplayer_tree().widgets;
        widgets.push(text_box("profile_label", 1, 0, "New001"));
        let scene = scene(&WidgetTree { widgets });
        let label = scene.texts.iter().find(|text| text.text == "New001");
        assert!(
            label.is_some_and(|text| text.x > 1380.0),
            "{:?}",
            scene.texts
        );
    }

    #[test]
    fn a_spinner_beside_a_list_shows_its_value_in_the_pane() {
        // (Replay Level's kind of map, SINGLEPLAYER, its items generated
        // from its string list)
        let mut widgets = multiplayer_tree().widgets;
        let row = widgets.len();
        widgets.push(widget(
            "list_item_0_map_kind",
            WidgetType::Container,
            1,
            Some(0),
        ));
        let at = widgets.len();
        let mut kind = widget(
            "list_item_0_map_kind_spinner",
            WidgetType::SpinnerList,
            2,
            Some(row),
        );
        kind.list = Some(List {
            selected: 0,
            items: 4,
            item_text: Some("SINGLEPLAYER".to_owned()),
        });
        widgets.push(kind);
        let mut item = widget(
            "list_item_0_map_kind_spinner",
            WidgetType::SpinnerList,
            3,
            Some(at),
        );
        item.list = Some(List {
            selected: 0,
            items: 0,
            item_text: None,
        });
        widgets.push(item);
        let scene = scene(&WidgetTree { widgets });
        let values: Vec<&Text> = scene
            .texts
            .iter()
            .filter(|text| text.text.contains("SINGLEPLAYER"))
            .collect();
        assert_eq!(values.len(), 1, "{:?}", scene.texts);
        assert!(values.iter().all(|text| text.x > 1380.0), "{values:?}");
    }

    #[test]
    fn another_screens_texts_not_yet_drawn_are_named_and_its_keys_footed() {
        let widgets = vec![
            widget(
                "new_campaign_creating_profile",
                WidgetType::Container,
                0,
                None,
            ),
            text_box("creating_new_profile", 1, 0, ""),
            widget("b_butn", WidgetType::Container, 1, Some(0)),
            text_box("=BACK", 1, 0, "=Back"),
        ];
        let scene = scene(&WidgetTree { widgets });
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert!(texts.contains(&"CREATING NEW PROFILE"), "{texts:?}");
        assert!(
            texts.contains(&"BACK") && texts.contains(&"ESC"),
            "{texts:?}"
        );
        assert!(!texts.iter().any(|text| text.starts_with('=')), "{texts:?}");
    }

    #[test]
    fn a_screen_with_nothing_of_its_own_is_titled_by_its_name() {
        let widgets = vec![widget(
            "new_campaign_creating_profile",
            WidgetType::Container,
            0,
            None,
        )];
        let scene = scene(&WidgetTree { widgets });
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(texts, ["NEW CAMPAIGN CREATING PROFILE"]);
        assert!(scene.texts[0].y < 198.0, "in the title's scrim");
        assert_eq!(scene.texts[0].face, Face::Title);
    }

    #[test]
    fn a_list_screen_with_no_header_is_titled_by_its_name() {
        let mut widgets = vec![
            widget("create&edit_player_profile", WidgetType::Container, 0, None),
            widget("profile_edit_list", WidgetType::ColumnList, 1, Some(0)),
            widget("name_profile_item", WidgetType::TextBox, 2, Some(1)),
        ];
        if let Some(item) = widgets.get_mut(2) {
            item.focused = true;
        }
        let scene = scene(&WidgetTree { widgets });
        let Some(title) = scene.texts.first() else {
            panic!("no texts");
        };
        assert_eq!(title.text, "CREATE & EDIT PLAYER PROFILE");
        assert_eq!(title.face, Face::Title);
        assert!(title.y < 198.0, "in the title's scrim");
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(texts[1..], ["NAME PROFILE"]);
    }

    #[test]
    fn a_profile_screen_with_no_header_is_titled_by_its_name() {
        let widgets = vec![
            widget("player_profile_manager", WidgetType::Container, 0, None),
            widget("pm_profile_list", WidgetType::SpinnerList, 1, Some(0)),
            widget("pm_profile_card", WidgetType::Container, 2, Some(1)),
            widget("pm_profile_name", WidgetType::TextBox, 3, Some(2)),
        ];
        let scene = scene(&WidgetTree { widgets });
        let Some(title) = scene.texts.first() else {
            panic!("no texts");
        };
        assert_eq!(title.text, "PLAYER PROFILE MANAGER");
        assert_eq!(title.face, Face::Title);
        assert!(title.y < 198.0, "in the title's scrim");
    }

    #[test]
    fn a_header_inside_the_screens_one_wrapper_titles_it() {
        // (the profile manager as main_menu.profile_manager.xml opens it:
        // its header sits in the screen it wraps)
        let mut widgets = vec![
            widget("player_profile_manager", WidgetType::Container, 0, None),
            widget(
                "player_profile_manager_screen",
                WidgetType::Container,
                1,
                Some(0),
            ),
            widget("sel_list_desc_bkd", WidgetType::Container, 2, Some(1)),
            widget(
                "profile_manager_select_list_2",
                WidgetType::ColumnList,
                2,
                Some(1),
            ),
            text_box("pm_profile_1", 3, 3, "Chief"),
            text_box("pm_create_new", 3, 3, "Create New Player Profile"),
            widget("header_select_profile", WidgetType::Container, 2, Some(1)),
        ];
        widgets[4].focused = true;
        let scene = scene(&WidgetTree { widgets });
        let titles: Vec<&str> = scene
            .texts
            .iter()
            .filter(|text| text.face == Face::Title)
            .map(|text| text.text.as_str())
            .collect();
        assert_eq!(titles, ["SELECT PROFILE"]);
    }

    fn typing() -> Keyboard {
        Keyboard {
            caption: Some("ENTER NAME".to_owned()),
            text: "Chief".to_owned(),
            cursor: Some(5),
        }
    }

    #[test]
    fn the_keyboard_is_a_field_with_its_caption_text_and_keys() {
        let mut scene = Scene::default();
        keyboard(&typing(), &mut scene);
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(
            texts,
            ["ENTER NAME", "Chief", "ESC", "CANCEL", "ENTER", "DONE"]
        );
        // (the field, bordered as a tab is)
        assert!(scene
            .shapes
            .iter()
            .any(|shape| shape.border == 9.0 && shape.border_color == hex(BLUE, 1.0)));
    }

    #[test]
    fn the_keyboards_cancel_and_done_take_a_click() {
        let mut scene = Scene::default();
        keyboard(&typing(), &mut scene);
        for (words, kind) in [
            ("CANCEL", opence_runtime::redrawn::KEYBOARD_BACK_KIND),
            ("DONE", opence_runtime::redrawn::KEYBOARD_ENTER_KIND),
        ] {
            let Some(text) = scene.texts.iter().find(|text| text.text == words) else {
                panic!("{:?}", scene.texts);
            };
            let at = [text.x + 20.0, text.y + 20.0];
            let Some(hit) = scene.targets.iter().rev().find(|target| {
                let [left, top, width, height] = target.rect;
                at[0] >= left && at[0] < left + width && at[1] >= top && at[1] < top + height
            }) else {
                panic!("no target under {words}: {:?}", scene.targets);
            };
            assert_eq!(hit.kind, kind, "{words}");
            assert_ne!(hit.widget, 0, "{words}");
        }
    }

    #[test]
    fn the_keyboards_caret_follows_the_cursor() {
        let mut at_end = Scene::default();
        keyboard(&typing(), &mut at_end);
        let mut at_start = Scene::default();
        keyboard(
            &Keyboard {
                cursor: Some(0),
                ..typing()
            },
            &mut at_start,
        );
        let caret = |scene: &Scene| {
            scene
                .shapes
                .iter()
                .find(|shape| shape.fill == hex(GLOW, 1.0))
                .map(|shape| shape.rect[0])
        };
        assert!(caret(&at_end) > caret(&at_start), "{:?}", at_end.shapes);
    }

    #[test]
    fn the_keyboard_is_drawn_over_the_screen_under_it() {
        let mut scene = scene(&multiplayer_tree());
        let under = scene.texts.len();
        keyboard(&typing(), &mut scene);
        assert_eq!(
            scene.texts.get(under).map(|text| text.text.as_str()),
            Some("ENTER NAME")
        );
    }

    #[test]
    fn an_items_lines_are_one_line() {
        let mut widgets = multiplayer_tree().widgets;
        widgets[3].text = Some("Create New\r\nPlayer Profile".to_owned());
        let scene = scene(&WidgetTree { widgets });
        assert_eq!(scene.texts[1].text, "CREATE NEW PLAYER PROFILE");
    }

    /// a settings screen as the PC menus open it: each row a container
    /// of its label and the spinner that holds its value
    fn settings_tree() -> WidgetTree {
        let mut widgets = vec![
            widget("video_settings_screen", WidgetType::Container, 0, None),
            widget("video_settings_list", WidgetType::ColumnList, 1, Some(0)),
            widget("op_display", WidgetType::Container, 2, Some(1)),
            text_box("display_label", 3, 2, "Display Mode"),
            widget("display_spinner", WidgetType::SpinnerList, 3, Some(2)),
            widget("op_vsync", WidgetType::Container, 2, Some(1)),
            text_box("vsync_label", 3, 5, "V-Sync"),
            widget("vsync_spinner", WidgetType::SpinnerList, 3, Some(5)),
            text_box("vsync_value", 4, 7, "On"),
        ];
        widgets[4].list = Some(List {
            selected: 1,
            items: 2,
            item_text: Some("Borderless".to_owned()),
        });
        widgets[1].focused = true;
        widgets[5].focused = true;
        WidgetTree { widgets }
    }

    #[test]
    fn a_long_item_is_kept_left_of_the_pane() {
        let mut tree = multiplayer_tree();
        if let Some(item) = tree
            .widgets
            .iter_mut()
            .find(|widget| widget.text.as_deref() == Some("Co-op"))
        {
            item.text = Some("The Truth and Reconciliation and more".to_owned());
        }
        let scene = scene(&tree);
        let Some(line) = scene
            .texts
            .iter()
            .find(|text| text.text.starts_with("THE TRUTH"))
        else {
            panic!("{:?}", scene.texts);
        };
        assert!(
            line.x + width(&line.text, line.size) <= PANE - 30.0,
            "{line:?}"
        );
        assert!(line.size < 66.0);
    }

    #[test]
    fn a_settings_screens_tabs_are_its_labels_alone() {
        let scene = scene(&settings_tree());
        let find = |words: &str| scene.texts.iter().find(|text| text.text == words);
        let Some(label) = find("DISPLAY MODE") else {
            panic!("{:?}", scene.texts);
        };
        assert!(label.x < PANE);
        // (no value is drawn in a tab: the unfocused row's is not drawn)
        assert!(find("BORDERLESS").is_none(), "{:?}", scene.texts);
    }

    #[test]
    fn the_focused_setting_is_set_in_the_pane_between_its_arrows() {
        let tree = addressed(settings_tree());
        let scene = scene(&tree);
        let in_pane = |words: &str| {
            scene
                .texts
                .iter()
                .find(|text| text.text == words && text.x > PANE)
        };
        let (Some(label), Some(value)) = (in_pane("V-SYNC"), in_pane("ON")) else {
            panic!("{:?}", scene.texts);
        };
        let middle = |text: &Text| text.y + text.size / 2.0;
        assert!((middle(label) - middle(value)).abs() < 0.5);
        assert!(label.y >= 600.0);
        // (a value is drawn once, in the pane)
        assert_eq!(
            scene.texts.iter().filter(|text| text.text == "ON").count(),
            1
        );
        // (its arrows either side, which step it back and on)
        let spinner = tree.widgets[7].address;
        let value_target = scene
            .targets
            .iter()
            .find(|target| target.kind == VALUE_KIND)
            .copied();
        let Some(value_target) = value_target else {
            panic!("{:?}", scene.targets);
        };
        assert_eq!(value_target.widget, spinner);
        let [left, _, wide, _] = value_target.rect;
        let ends = width(&value.text, value.size);
        assert!(left < value.x - ends && left + wide > value.x);
    }

    #[test]
    fn a_long_setting_does_not_run_into_its_value() {
        let mut tree = settings_tree();
        tree.widgets[6].text = Some("Vertical synchronisation of the frames".to_owned());
        tree.widgets[8].text = Some("Adaptive Half Rate".to_owned());
        let scene = scene(&tree);
        let in_pane = |start: &str| {
            scene
                .texts
                .iter()
                .find(|text| text.text.starts_with(start) && text.x > PANE)
        };
        let (Some(label), Some(value)) = (in_pane("VERTICAL"), in_pane("ADAPTIVE")) else {
            panic!("{:?}", scene.texts);
        };
        let label_end = label.x + width(&label.text, label.size);
        let value_start = value.x - width(&value.text, value.size);
        // (the left arrow and its gap sit between them)
        assert!(label_end + 45.0 <= value_start, "{label:?} {value:?}");
    }

    /// the PC multiplayer screen: two headings, the items under them,
    /// and its button bar the list's last child
    fn grouped_tree() -> WidgetTree {
        let mut widgets = vec![
            widget(
                "multiplayer_type_select_screen",
                WidgetType::Container,
                0,
                None,
            ),
            widget(
                "multiplayer_type_select_list",
                WidgetType::ColumnList,
                1,
                Some(0),
            ),
            text_box("multiplayer_type_join_item", 2, 1, "Join Game"),
            text_box("multiplayer_type_join_lan_item", 2, 1, "LAN"),
            text_box("multiplayer_type_create_item", 2, 1, "Create Game"),
            text_box("multiplayer_type_create_lan_item", 2, 1, "LAN"),
            widget(
                "multiplayer_type_button_bar",
                WidgetType::ColumnList,
                2,
                Some(1),
            ),
            text_box("common_button_back", 3, 6, "Back"),
        ];
        for at in [3, 5] {
            widgets[at].offset = [51, 0];
        }
        for at in [2, 4] {
            widgets[at].offset = [19, 0];
        }
        widgets[1].focused = true;
        widgets[3].focused = true;
        WidgetTree { widgets }
    }

    #[test]
    fn a_heading_in_a_list_is_drawn_as_a_heading() {
        let scene = scene(&grouped_tree());
        let rows: Vec<&Text> = scene
            .texts
            .iter()
            .filter(|text| text.size == 66.0)
            .collect();
        let words: Vec<&str> = rows.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(words, ["LAN", "LAN"], "the headings are not rows");
        let headings: Vec<&Text> = scene
            .texts
            .iter()
            .filter(|text| text.text == "JOIN GAME" || text.text == "CREATE GAME")
            .collect();
        assert_eq!(headings.len(), 2);
        // (each heading above its own items, smaller and to their left)
        assert!(headings[0].y < rows[0].y && rows[0].y < headings[1].y);
        assert!(headings[1].y < rows[1].y);
        assert!(headings[0].size < 66.0 && headings[0].x < rows[0].x);
    }

    #[test]
    fn a_lists_button_bar_is_in_the_footer() {
        let scene = scene(&grouped_tree());
        assert!(!scene
            .texts
            .iter()
            .any(|text| text.size == 66.0 && text.text == "BACK"));
        let Some(back) = scene.texts.iter().find(|text| text.text == "BACK") else {
            panic!("{:?}", scene.texts);
        };
        assert!(back.y > FOOT);
        assert!(scene.texts.iter().any(|text| text.text == "ESC"));
    }

    #[test]
    fn a_bar_button_is_lit_only_while_the_screens_focus_is_on_it() {
        // (the bar keeps its own focus on Back while the list's is on a
        // row: Enter then goes to the row, so Back is not lit)
        let mut tree = grouped_tree();
        tree.widgets[7].focused = true;
        let drawn = scene(&tree);
        let Some(back) = drawn.texts.iter().find(|text| text.text == "BACK") else {
            panic!("{:?}", drawn.texts);
        };
        assert_eq!(back.color, hex(BLUE, 1.0));
        // (the list's focus moved to the bar: Back is what Enter presses)
        tree.widgets[3].focused = false;
        tree.widgets[6].focused = true;
        let drawn = scene(&tree);
        let Some(back) = drawn.texts.iter().find(|text| text.text == "BACK") else {
            panic!("{:?}", drawn.texts);
        };
        assert_eq!(back.color, hex(FOCUSED, 1.0));
    }
}
