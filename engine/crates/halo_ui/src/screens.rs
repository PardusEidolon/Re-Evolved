//! Each screen the game has open, laid out as its prototype draws it
//! (the Halo UI design artifact): the game's own widgets say what is on
//! it, its texts, its items and where the focus is; the layout and the
//! look are the prototype's.
//!
//! The game no longer draws the menus' text, lists or the pictures below
//! a screen's own children while the plugin draws them
//! (`display.redraw_menus`, interface/ui_widget.c), so every screen must
//! give a scene: one the plugin does not know is drawn by the generic
//! layout, each of its texts where the game would have put it.

use opence_runtime::widgets::{Widget, WidgetTree, WidgetType};

use crate::scene::{hex, Align, Face, Scene, Shape, Text, HEIGHT, WIDTH};

/// How much larger the design is than the game's 640×480 screen, whose
/// 4:3 is centred in the design's 16:9.
const SCALE: f32 = HEIGHT / 480.0;
/// Where the game's screen starts across the design's.
const LEFT: f32 = (WIDTH - 640.0 * SCALE) / 2.0;

/// The prototypes' blue, the items'
const BLUE: u32 = 0x2796FE;
/// the focused item's text
const FOCUSED: u32 = 0xF2F6FA;
/// the focused item's glow
const GLOW: u32 = 0x6FC0FF;
/// the rules under the title and over the footer
const RULE: u32 = 0x1D6AB4;

/// The scene of the first player's open screen; an empty one if none is
/// open.
#[must_use]
pub fn scene(tree: &WidgetTree) -> Scene {
    let Some(screen) = tree.widgets.first() else {
        return Scene::default();
    };
    let mut scene = Scene::default();
    if screen.name.as_deref() == Some("the_main_menu") {
        main_menu(tree, &mut scene);
    } else if let Some(list) = list_of(tree, WidgetType::SpinnerList) {
        profiles(tree, list, &mut scene);
    } else if let Some(list) = list_of(tree, WidgetType::ColumnList) {
        list_screen(tree, list, &mut scene);
    } else {
        frame(&mut scene);
        let drawn = footer(tree, &mut scene);
        rest(tree, &drawn, &mut scene);
        // (a screen of nothing but its picture, a moment's message as
        // "creating profile" is, says what it is in its title)
        if scene.texts.is_empty() {
            if let Some(title) = label(&Widget {
                text: None,
                ..screen.clone()
            }) {
                scene.texts.push(Text::new(
                    &title,
                    [231.0, 198.0 - 30.0 - 72.0],
                    72.0,
                    Face::Heavy,
                    hex(FOCUSED, 1.0),
                ));
            }
        }
    }
    scene
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
        [WIDTH - 96.0, HEIGHT - 108.0],
        36.0,
        Face::Semibold,
        [1.0, 1.0, 1.0, 0.38],
    );
    version.align = Align::Right;
    scene.texts.push(version);
}

/// The title's scrim and rule, and the footer's, of every screen but
/// the main menu.
fn frame(scene: &mut Scene) {
    scene.shapes.push(Shape::fill(
        [0.0, 198.0, WIDTH, HEIGHT - 324.0 - 198.0],
        [0.0, 0.0, 0.0, 0.45],
    ));
    scene
        .shapes
        .push(Shape::fill([0.0, 195.0, WIDTH, 3.0], hex(RULE, 1.0)));
    scene.shapes.push(Shape::fill(
        [0.0, 1836.0, WIDTH, 162.0],
        [0.0, 0.0, 0.0, 0.6],
    ));
    scene
        .shapes
        .push(Shape::fill([0.0, 1836.0, WIDTH, 3.0], hex(RULE, 1.0)));
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
            [right, 1836.0 + 54.0],
            54.0,
            Face::Semibold,
            hex(FOCUSED, 0.9),
        );
        words.align = Align::Right;
        scene.texts.push(words);
        // (Overpass is about half an em wide a letter)
        right -= 0.6 * 54.0 * text.chars().count() as f32 + 30.0;
        if !cap.is_empty() {
            let width = 0.62 * 42.0 * cap.chars().count() as f32 + 48.0;
            scene.shapes.push(Shape {
                rect: [right - width, 1836.0 + 42.0, width, 78.0],
                fill: hex(BLUE, 0.16),
                border: 3.0,
                border_color: hex(BLUE, 0.9),
                radii: [12.0; 4],
            });
            let mut key = Text::new(
                cap,
                [right - width / 2.0, 1836.0 + 60.0],
                42.0,
                Face::Bold,
                hex(FOCUSED, 1.0),
            );
            key.align = Align::Center;
            scene.texts.push(key);
            right -= width;
        }
        right -= 90.0;
    }
    keys
}

/// A screen of a list: its items down the left, the focused one tabbed.
fn list_screen(tree: &WidgetTree, list: usize, scene: &mut Scene) {
    const WIDE: f32 = 1149.0;
    const TALL: f32 = 108.0;
    const GAP: f32 = 18.0;
    frame(scene);
    let mut y = 243.0;
    for (item, enabled) in items(tree, list) {
        let Some(text) = item_label(tree, item) else {
            continue;
        };
        let mut color = hex(BLUE, if enabled { 1.0 } else { 0.35 });
        if item.focused {
            scene.shapes.push(Shape {
                rect: [231.0, y, WIDE, TALL],
                fill: hex(BLUE, 0.18),
                border: 9.0,
                border_color: hex(BLUE, 1.0),
                radii: [54.0, 0.0, 0.0, 54.0],
            });
            color = hex(FOCUSED, 1.0);
        }
        let line = Text::new(
            &text.to_uppercase(),
            [231.0 + 72.0, y + (TALL - 66.0) / 2.0],
            66.0,
            Face::Heavy,
            color,
        );
        scene.texts.push(line);
        y += TALL + GAP;
    }
    let mut drawn = subtree(tree, list);
    drawn.extend(footer(tree, scene));
    rest(tree, &drawn, scene);
}

/// The profiles: a card for each of the spinner's items, the focused one
/// bordered.
fn profiles(tree: &WidgetTree, list: usize, scene: &mut Scene) {
    const WIDE: f32 = 750.0;
    const TALL: f32 = 1260.0;
    const GAP: f32 = 72.0;
    frame(scene);
    let mut x = 240.0;
    for at in children(tree, list) {
        let Some(card) = tree.widgets.get(at) else {
            continue;
        };
        if !card.visible {
            continue;
        }
        scene.shapes.push(Shape {
            rect: [x, 330.0, WIDE, TALL],
            fill: [0.0, 0.0, 0.0, 0.5],
            border: if card.focused { 9.0 } else { 3.0 },
            border_color: hex(BLUE, if card.focused { 1.0 } else { 0.5 }),
            radii: [30.0; 4],
        });
        let mut y = 330.0 + 60.0;
        for (line, below) in subtree(tree, at)
            .into_iter()
            .filter_map(|at| tree.widgets.get(at))
            .filter(|widget| widget.kind == WidgetType::TextBox && widget.visible)
            .filter_map(label)
            .enumerate()
        {
            let (size, face, color) = if line == 0 {
                (66.0, Face::Heavy, hex(FOCUSED, 1.0))
            } else {
                (42.0, Face::Bold, hex(BLUE, 0.9))
            };
            scene
                .texts
                .push(Text::new(&below, [x + 54.0, y], size, face, color));
            y += size + 24.0;
        }
        x += WIDE + GAP;
    }
    let mut drawn = subtree(tree, list);
    drawn.extend(footer(tree, scene));
    rest(tree, &drawn, scene);
}

/// The texts of the widgets not in `drawn`, by the generic layout.
fn rest(tree: &WidgetTree, drawn: &[usize], scene: &mut Scene) {
    let left: Vec<usize> = (0..tree.widgets.len())
        .filter(|at| !drawn.contains(at))
        .collect();
    generic(tree, &left, scene);
}

/// Each visible text box at `places` (its text, or its name before the
/// game has given it one) where the game would have drawn it:
/// its offsets and its parents', on the game's 4:3 screen centred in the
/// design's.
fn generic(tree: &WidgetTree, places: &[usize], scene: &mut Scene) {
    for &at in places {
        let Some(widget) = tree.widgets.get(at) else {
            continue;
        };
        if widget.kind != WidgetType::TextBox || !shown(tree, at) {
            continue;
        }
        let Some(text) = label(widget) else {
            continue;
        };
        let [x, y] = placed(tree, at);
        let color = if widget.focused {
            hex(FOCUSED, 1.0)
        } else {
            hex(BLUE, 0.9)
        };
        scene.texts.push(Text::new(
            &text,
            [LEFT + x * SCALE, y * SCALE],
            42.0,
            Face::Bold,
            color,
        ));
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
        assert_eq!(
            texts[..4],
            ["CO-OP", "SPLIT SCREEN", "SYSTEM LINK", "GAME TYPES"]
        );
        let Some(tab) = scene.shapes.iter().find(|shape| shape.border == 9.0) else {
            panic!("no focused tab in {:?}", scene.shapes);
        };
        assert_eq!(tab.radii, [54.0, 0.0, 0.0, 54.0]);
        // (the tab is on the second item, where the focus is)
        assert!((tab.rect[1] - (243.0 + 108.0 + 18.0)).abs() < 0.5);
        assert_eq!(scene.texts[1].color, hex(FOCUSED, 1.0));
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
        assert!(
            cards[1].border > cards[0].border,
            "the focused card is the second"
        );
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        assert_eq!(texts, ["Chief", "Level", "Cortana"]);
    }

    #[test]
    fn another_screen_draws_its_texts_where_the_game_would() {
        let mut widgets = vec![
            widget("a_message", WidgetType::Container, 0, None),
            widget("box", WidgetType::Container, 1, Some(0)),
            text_box("message_text", 2, 1, "Saving..."),
            text_box("hidden", 2, 1, "Not drawn"),
        ];
        widgets[1].offset = [100, 200];
        widgets[2].offset = [20, 40];
        widgets[1].visible = true;
        widgets[3].visible = false;
        let scene = scene(&WidgetTree { widgets });
        assert_eq!(scene.texts.len(), 1);
        let text = &scene.texts[0];
        assert_eq!(text.text, "Saving...");
        assert!((text.x - (LEFT + 120.0 * SCALE)).abs() < 0.5);
        assert!((text.y - 240.0 * SCALE).abs() < 0.5);
        // (framed, as every screen but the main menu)
        assert!(scene.shapes.len() >= 4, "{:?}", scene.shapes);
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
    }

    #[test]
    fn an_items_lines_are_one_line() {
        let mut widgets = multiplayer_tree().widgets;
        widgets[3].text = Some("Create New\r\nPlayer Profile".to_owned());
        let scene = scene(&WidgetTree { widgets });
        assert_eq!(scene.texts[0].text, "CREATE NEW PLAYER PROFILE");
    }
}
