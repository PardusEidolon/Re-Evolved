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

/// The scene of the first player's open screen; an empty one if none is
/// open. It is laid over what the game still draws behind the menus:
/// the ui map's world, its ring, camera and models.
#[must_use]
pub fn scene(tree: &WidgetTree) -> Scene {
    let Some(screen) = tree.widgets.first() else {
        return Scene::default();
    };
    let main = screen.name.as_deref() == Some("the_main_menu");
    let mut scene = Scene::default();
    if main {
        main_menu(tree, &mut scene);
    } else if let Some(list) = list_of(tree, WidgetType::SpinnerList) {
        frame(&mut scene);
        let titled = title(tree, &mut scene);
        profiles(tree, list, titled, &mut scene);
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
    for (cap, words) in [("ESC", "CANCEL"), ("ENTER", "DONE")] {
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
        keys += 0.63 * 57.0 * words.chars().count() as f32 + 84.0;
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
        (widget.visible && widget.depth == 1).then(|| (at, words.join(" ")))
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
        .position(|widget| widget.kind == kind && widget.depth == 1 && widget.visible)
        .filter(|&list| !children(tree, list).is_empty())
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

/// The text of an item: its own, or its first text box's.
fn item_label(tree: &WidgetTree, item: &Widget) -> Option<String> {
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

/// The main menu: its items centred under the logo (the map's own,
/// still drawn by the game), the version at the bottom right.
fn main_menu(tree: &WidgetTree, scene: &mut Scene) {
    const SIZE: f32 = 108.0;
    const GAP: f32 = 18.0;
    if let Some(list) = list_of(tree, WidgetType::ColumnList) {
        let mut y = 1185.0;
        for (item, enabled) in items(tree, list) {
            let Some(text) = item_label(tree, item) else {
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

/// The footer: each of the screen's button keys (its `=Name` text
/// boxes), with the key that presses it, from the right.
fn footer(tree: &WidgetTree, scene: &mut Scene) -> Vec<usize> {
    let keys: Vec<usize> = (0..tree.widgets.len())
        .filter(|&at| {
            tree.widgets.get(at).is_some_and(|widget| {
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
        let cap = match button {
            Some('a') => "ENTER",
            Some('b') => "ESC",
            Some('x') => "X",
            Some('y') => "Y",
            _ => "",
        };
        let mut words = Text::new(
            &text.to_uppercase(),
            [right, middle - 57.0 / 2.0],
            57.0,
            Face::Heavy,
            hex(BLUE, 1.0),
        );
        words.align = Align::Right;
        scene.texts.push(words);
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
    let mut y = 243.0;
    for (item, enabled) in items(tree, list) {
        let Some(text) = item_label(tree, item) else {
            continue;
        };
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
        let line = Text::new(
            &text.to_uppercase(),
            [LEFT_OF + 75.0, y + (TALL - 66.0) / 2.0],
            66.0,
            Face::Heavy,
            color,
        );
        scene.texts.push(line);
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
    rest(tree, &drawn, PANE + 75.0, scene);
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
fn rest(tree: &WidgetTree, drawn: &[usize], left: f32, scene: &mut Scene) {
    let others: Vec<usize> = (0..tree.widgets.len())
        .filter(|at| !drawn.contains(at))
        .collect();
    generic(tree, &others, left, scene);
}

/// Each shown text box at `places` (its text, or its name before the
/// game has given it one), a line of its own for each of the game's
/// lines, stacked down from under the title at `left`. They go in the
/// order the game places them, top to bottom and then left to right:
/// the game's own places overlap once its pictures are gone, and its
/// 4:3 screen is not the design's.
fn generic(tree: &WidgetTree, places: &[usize], left: f32, scene: &mut Scene) {
    const SIZE: f32 = 54.0;
    let mut texts: Vec<(usize, [f32; 2])> = places
        .iter()
        .filter(|&&at| {
            tree.widgets
                .get(at)
                .is_some_and(|widget| widget.kind == WidgetType::TextBox)
                && shown(tree, at)
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
        let lines: Vec<String> = match widget.text.as_deref() {
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
        assert_eq!(texts, ["Chief", "Level", "Cortana"]);
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
}
