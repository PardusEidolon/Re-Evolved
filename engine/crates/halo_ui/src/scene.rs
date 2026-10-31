//! What the plugin draws: rounded rectangles and lines of text, in the
//! prototypes' design units, a 3840×2160 screen (the app scales them to
//! the display). The app draws the shapes first, then the texts, over the
//! frame the game has drawn.

/// The width of the design's screen.
pub const WIDTH: f32 = 3840.0;
/// The height of the design's screen.
pub const HEIGHT: f32 = 2160.0;

/// A colour, red, green, blue and alpha, each 0 to 1, not premultiplied.
pub type Rgba = [f32; 4];

/// `#rrggbb` at `alpha`.
#[must_use]
pub fn hex(rgb: u32, alpha: f32) -> Rgba {
    let channel = |shift: u32| f32::from(u8::try_from((rgb >> shift) & 0xFF).unwrap_or(0)) / 255.0;
    [channel(16), channel(8), channel(0), alpha]
}

/// A rectangle, filled, with a border, its corners rounded.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// left, top, width and height
    pub rect: [f32; 4],
    /// its fill
    pub fill: Rgba,
    /// how wide its border is, drawn inside the rectangle; 0 for none
    pub border: f32,
    /// its border's colour
    pub border_color: Rgba,
    /// the radii of its corners: top left, top right, bottom right,
    /// bottom left
    pub radii: [f32; 4],
}

impl Shape {
    /// A filled rectangle with square corners and no border.
    #[must_use]
    pub fn fill(rect: [f32; 4], fill: Rgba) -> Self {
        Self {
            rect,
            fill,
            border: 0.0,
            border_color: [0.0; 4],
            radii: [0.0; 4],
        }
    }
}

/// The faces the prototypes use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    /// OpenCE, the titles'
    Title,
    /// Overpass at weight 900, the menus' items
    Heavy,
    /// Overpass at weight 750, descriptions
    Bold,
    /// Overpass at weight 600, small print and the button keys
    Semibold,
}

/// Where a text's `x` is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    /// its left edge
    Left,
    /// its middle
    Center,
    /// its right edge
    Right,
}

/// One line of text.
#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    /// what it says
    pub text: String,
    /// where it is across, as `align` says
    pub x: f32,
    /// the top of its line, which is `size` tall
    pub y: f32,
    /// its size, the font's em
    pub size: f32,
    /// its face
    pub face: Face,
    /// its colour
    pub color: Rgba,
    /// the colour of the glow around it; transparent for none
    pub glow: Rgba,
    /// how its `x` is read
    pub align: Align,
}

impl Text {
    /// A left-aligned line with no glow.
    #[must_use]
    pub fn new(text: &str, at: [f32; 2], size: f32, face: Face, color: Rgba) -> Self {
        Self {
            text: text.to_owned(),
            x: at[0],
            y: at[1],
            size,
            face,
            color,
            glow: [0.0; 4],
            align: Align::Left,
        }
    }
}

/// Everything the plugin draws over one frame.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scene {
    /// the shapes, in the order they are drawn
    pub shapes: Vec<Shape>,
    /// the texts, drawn over the shapes in this order
    pub texts: Vec<Text>,
}

impl Scene {
    /// Whether it draws nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty() && self.texts.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_reads_each_channel() {
        assert_eq!(hex(0xFF0000, 1.0), [1.0, 0.0, 0.0, 1.0]);
        let blue = hex(0x2796FE, 0.5);
        assert!((blue[2] - 254.0 / 255.0).abs() < 1e-6);
        assert!((blue[3] - 0.5).abs() < 1e-6);
    }
}
