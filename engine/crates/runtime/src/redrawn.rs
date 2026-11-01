//! Where a plugin that draws the menus anew has put their items, so that
//! the pointer works on what is drawn.
//!
//! The game's menus (interface/ui_widget.c) find the item under the
//! pointer from where their tags place it on a 640×480 screen. A plugin
//! drawing the menus in another layout (the Halo UI) gives the input
//! the rectangles it drew each item in, in its design's units, and the
//! game asks for the one under the pointer (`halo_ui_pointer_redrawn`):
//! its widget, and where in the rectangle the pointer is, so that the
//! game can put the point at the same place in that item's own target.

/// The width of the design's screen a plugin lays the menus out on.
pub const DESIGN_WIDTH: f32 = 3840.0;
/// Its height.
pub const DESIGN_HEIGHT: f32 = 2160.0;

/// An item as a plugin drew it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target {
    /// left, top, width and height, in the design's units
    pub rect: [f32; 4],
    /// the address of its `widget_instance` in the game's memory
    pub widget: u32,
    /// the game's kind of target it stands for (`ui_mouse_target_kind`),
    /// or [`ANY_KIND`] for whichever the widget has
    pub kind: i16,
}

/// A target that stands for whichever kind the game has for its widget.
pub const ANY_KIND: i16 = -1;

/// The game's `_ui_mouse_target_value`: a list showing one value, which
/// a click on either half steps that way.
pub const VALUE_KIND: i16 = 1;

/// The game's `_ui_mouse_target_list_back`.
pub const LIST_BACK_KIND: i16 = 4;

/// The game's `_ui_mouse_target_list_forward`.
pub const LIST_FORWARD_KIND: i16 = 5;

/// The item under a point, as the game reads it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit {
    /// its widget's address; 0 when no item is drawn under the point
    pub widget: u32,
    /// where across and down the rectangle the point is, 0 to 1
    pub across: f32,
    /// as `across`, down
    pub down: f32,
    /// the kind of target it stands for, or [`ANY_KIND`]
    pub kind: i16,
}

/// `struct halo_ui_redrawn`'s size on wasm32.
pub const HIT_BYTES: usize = 16;

impl Hit {
    /// No item under the point.
    pub const NONE: Self = Self {
        widget: 0,
        across: 0.0,
        down: 0.0,
        kind: ANY_KIND,
    };

    /// `struct halo_ui_redrawn` as wasm32 lays it out.
    #[must_use]
    pub fn bytes(&self) -> [u8; HIT_BYTES] {
        let mut bytes = [0; HIT_BYTES];
        bytes[0..4].copy_from_slice(&self.widget.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.across.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.down.to_le_bytes());
        bytes[12..14].copy_from_slice(&self.kind.to_le_bytes());
        bytes
    }
}

/// A picture of the game's that a plugin has placed: the game draws the
/// frame `frame` of the widget's own bitmap (from the map, as it always
/// does) stretched over `rect`, under the plugin's drawing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placed {
    /// left, top, width and height, in the design's units
    pub rect: [f32; 4],
    /// the address of its `widget_instance` in the game's memory
    pub widget: u32,
    /// the frame of the widget's bitmap's first sequence
    pub frame: i16,
}

/// `struct halo_ui_placed`'s size on wasm32.
pub const PLACED_BYTES: usize = 12;

impl Placed {
    /// `struct halo_ui_placed` as wasm32 lays it out, for a game screen
    /// `screen_width` units wide and 480 tall: the rectangle in the
    /// menus' centered 640 (x0, y0, x1, y1), then the frame.
    #[must_use]
    pub fn bytes(&self, screen_width: f32) -> [u8; PLACED_BYTES] {
        let [x0, y0, x1, y1] = in_menus(self.rect, screen_width);
        let mut bytes = [0; PLACED_BYTES];
        for (at, value) in [x0, y0, x1, y1, self.frame].into_iter().enumerate() {
            bytes[at * 2..at * 2 + 2].copy_from_slice(&value.to_le_bytes());
        }
        bytes
    }
}

/// The game's screen's height, in its units.
const SCREEN_HEIGHT: f32 = 480.0;

/// A rectangle in the design's units as the menus place theirs: in the
/// game's screen, `screen_width` by 480, less the centering of its 640
/// columns, as x0, y0, x1, y1. The design is fitted to the screen as the
/// app fits it to the drawable.
#[must_use]
pub fn in_menus(rect: [f32; 4], screen_width: f32) -> [i16; 4] {
    let scale = (screen_width / DESIGN_WIDTH).min(SCREEN_HEIGHT / DESIGN_HEIGHT);
    let left = (screen_width - DESIGN_WIDTH * scale) / 2.0 - (screen_width - 640.0) / 2.0;
    let top = (SCREEN_HEIGHT - DESIGN_HEIGHT * scale) / 2.0;
    let [x, y, width, height] = rect;
    // (a short holds any screen the game makes; the cast saturates)
    let unit = |value: f32| value.round() as i16;
    [
        unit(left + x * scale),
        unit(top + y * scale),
        unit(left + (x + width) * scale),
        unit(top + (y + height) * scale),
    ]
}

/// The `index`th picture placed for the widget at `widget`, if there is
/// one.
#[must_use]
pub fn placed(pictures: &[Placed], widget: u32, index: usize) -> Option<&Placed> {
    pictures
        .iter()
        .filter(|picture| picture.widget == widget)
        .nth(index)
}

/// A point in the drawable's pixels, `at`, in the design's units: the
/// design's screen is fitted to the drawable and centred, as the app
/// draws it (MenuPass.fit). None for an empty drawable.
#[must_use]
pub fn design_point(at: [f32; 2], drawable: [f32; 2]) -> Option<[f32; 2]> {
    let scale = (drawable[0] / DESIGN_WIDTH).min(drawable[1] / DESIGN_HEIGHT);
    if scale <= 0.0 || !scale.is_finite() {
        return None;
    }
    let left = (drawable[0] - DESIGN_WIDTH * scale) / 2.0;
    let top = (drawable[1] - DESIGN_HEIGHT * scale) / 2.0;
    Some([(at[0] - left) / scale, (at[1] - top) / scale])
}

/// The target drawn last (so on top) under `at`, a point in the
/// drawable's pixels, or [`Hit::NONE`].
#[must_use]
pub fn hit(targets: &[Target], at: [f32; 2], drawable: [f32; 2]) -> Hit {
    let Some([x, y]) = design_point(at, drawable) else {
        return Hit::NONE;
    };
    targets
        .iter()
        .rev()
        .find(|target| {
            let [left, top, width, height] = target.rect;
            x >= left && x < left + width && y >= top && y < top + height
        })
        .map_or(Hit::NONE, |target| {
            let [left, top, width, height] = target.rect;
            Hit {
                widget: target.widget,
                across: ((x - left) / width).clamp(0.0, 1.0),
                down: ((y - top) / height).clamp(0.0, 1.0),
                kind: target.kind,
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn target(rect: [f32; 4], widget: u32) -> Target {
        Target {
            rect,
            widget,
            kind: ANY_KIND,
        }
    }

    #[test]
    fn a_16_by_9_drawable_maps_by_its_scale() {
        let point = design_point([960.0, 540.0], [1920.0, 1080.0]);
        assert_eq!(point, Some([1920.0, 1080.0]));
    }

    #[test]
    fn a_16_by_10_drawable_is_letterboxed_top_and_bottom() {
        // 2560×1600: the design is 2560×1440, 80 pixels down
        let point = design_point([0.0, 80.0], [2560.0, 1600.0]);
        assert_eq!(point, Some([0.0, 0.0]));
        let bottom = design_point([2560.0, 1520.0], [2560.0, 1600.0]);
        assert_eq!(bottom, Some([3840.0, 2160.0]));
    }

    #[test]
    fn an_ultrawide_drawable_adds_room_at_the_sides() {
        // 5120×1440: the design is 2560×1440, 1280 pixels across
        let point = design_point([1280.0, 0.0], [5120.0, 1440.0]);
        assert_eq!(point, Some([0.0, 0.0]));
    }

    #[test]
    fn an_empty_drawable_has_no_point() {
        assert_eq!(design_point([1.0, 1.0], [0.0, 0.0]), None);
    }

    #[test]
    fn the_hit_says_where_in_the_item_the_point_is() {
        let targets = [target([1000.0, 500.0, 400.0, 100.0], 0x1234)];
        // 3840×2160 drawable: design units are pixels
        let hit = hit(&targets, [1100.0, 575.0], [3840.0, 2160.0]);
        assert_eq!(hit.widget, 0x1234);
        assert!((hit.across - 0.25).abs() < 1e-6);
        assert!((hit.down - 0.75).abs() < 1e-6);
    }

    #[test]
    fn the_last_target_drawn_wins_and_a_gap_hits_nothing() {
        let targets = [
            target([0.0, 0.0, 1000.0, 1000.0], 1),
            target([100.0, 100.0, 100.0, 100.0], 2),
        ];
        let drawable = [3840.0, 2160.0];
        assert_eq!(hit(&targets, [150.0, 150.0], drawable).widget, 2);
        assert_eq!(hit(&targets, [500.0, 500.0], drawable).widget, 1);
        assert_eq!(hit(&targets, [2000.0, 500.0], drawable), Hit::NONE);
    }

    #[test]
    fn the_hit_on_a_retina_16_by_10_screen_lands_on_the_drawn_item() {
        // a MacBook's 3456×2234 drawable: scale 0.9, 106.2 pixels down
        let targets = [target([231.0, 400.0, 1149.0, 108.0], 7)];
        let drawable = [3456.0, 2234.0];
        let scale = 3456.0 / 3840.0;
        let top = (2234.0 - 2160.0 * scale) / 2.0;
        let at = [231.0 * scale + 10.0, 454.0 * scale + top];
        assert_eq!(hit(&targets, at, drawable).widget, 7);
    }

    #[test]
    fn a_placed_picture_is_in_the_menus_centered_640() {
        // a 16:9 screen is 853 units wide: the design's 3840 is 853.3, its
        // left edge 106.7 left of the 640's
        let [x0, y0, x1, y1] = in_menus([0.0, 0.0, 3840.0, 2160.0], 853.333);
        assert_eq!([x0, y0, x1, y1], [-107, 0, 747, 480]);
        // the design's middle is the 640's
        let middle = in_menus([1920.0 - 450.0, 1080.0 - 450.0, 900.0, 900.0], 853.333);
        assert_eq!(middle, [220, 140, 420, 340]);
    }

    #[test]
    fn a_16_by_10_screen_letterboxes_a_placed_picture() {
        // 768 wide: the design is 768×432, 24 down
        let [_, y0, _, y1] = in_menus([0.0, 0.0, 3840.0, 2160.0], 768.0);
        assert_eq!([y0, y1], [24, 456]);
    }

    #[test]
    fn each_widget_has_its_own_pictures_in_order() {
        let picture = |widget, frame| Placed {
            rect: [0.0; 4],
            widget,
            frame,
        };
        let pictures = [picture(1, 4), picture(2, 0), picture(1, 5)];
        assert_eq!(placed(&pictures, 1, 1).map(|p| p.frame), Some(5));
        assert_eq!(placed(&pictures, 2, 1), None);
        let bytes = picture(1, 7).bytes(640.0);
        assert_eq!(bytes[8..10], [7, 0]);
    }

    #[test]
    fn the_bytes_are_the_c_struct() {
        let bytes = Hit {
            widget: 0x0102_0304,
            across: 0.5,
            down: 1.0,
            kind: VALUE_KIND,
        }
        .bytes();
        assert_eq!(bytes[0..4], [4, 3, 2, 1]);
        assert_eq!(bytes[4..8], 0.5_f32.to_le_bytes());
        assert_eq!(bytes[12..14], [1, 0]);
    }
}
