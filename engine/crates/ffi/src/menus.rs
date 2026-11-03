//! The Halo UI plugin's scene, as the app draws it over each frame
//! (`opence_halo_ui`): its rounded rectangles and lines of text, in the
//! design's 3840×2160 units, which the app scales to its drawable.

use opence_halo_ui::scene::{Align, Face, Scene, Shape, Text};

/// A rounded rectangle, filled, with a border drawn inside it.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct MenuShape {
    /// left, top, width and height
    pub rect: Vec<f32>,
    /// its fill, red, green, blue and alpha, not premultiplied
    pub fill: Vec<f32>,
    /// how wide its border is; 0 for none
    pub border: f32,
    /// its border's colour
    pub border_color: Vec<f32>,
    /// its corners' radii: top left, top right, bottom right, bottom left
    pub radii: Vec<f32>,
    /// whether it is added to the frame under it rather than laid over it
    #[uniffi(default = false)]
    pub additive: bool,
}

/// The faces the prototypes use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MenuFace {
    /// OpenCE, the titles'
    Title,
    /// Overpass at weight 900
    Heavy,
    /// Overpass at weight 750
    Bold,
    /// Overpass at weight 600
    Semibold,
}

/// Where a line's `x` is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum MenuAlign {
    /// its left edge
    Left,
    /// its middle
    Center,
    /// its right edge
    Right,
}

/// One line of text.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct MenuText {
    /// what it says
    pub text: String,
    /// where it is across, as `align` says
    pub x: f32,
    /// the top of its line
    pub y: f32,
    /// its size, the font's em
    pub size: f32,
    /// its face
    pub face: MenuFace,
    /// its colour
    pub color: Vec<f32>,
    /// the colour of the glow around it; transparent for none
    pub glow: Vec<f32>,
    /// how its `x` is read
    pub align: MenuAlign,
}

/// What the plugin draws over one frame: the shapes, then the texts.
#[derive(Debug, Clone, PartialEq, Default, uniffi::Record)]
pub struct MenuScene {
    /// the shapes, in order
    pub shapes: Vec<MenuShape>,
    /// the texts, over the shapes, in order
    pub texts: Vec<MenuText>,
}

impl From<&Shape> for MenuShape {
    fn from(shape: &Shape) -> Self {
        Self {
            rect: shape.rect.to_vec(),
            fill: shape.fill.to_vec(),
            border: shape.border,
            border_color: shape.border_color.to_vec(),
            radii: shape.radii.to_vec(),
            additive: shape.additive,
        }
    }
}

impl From<&Text> for MenuText {
    fn from(text: &Text) -> Self {
        Self {
            text: text.text.clone(),
            x: text.x,
            y: text.y,
            size: text.size,
            face: match text.face {
                Face::Title => MenuFace::Title,
                Face::Heavy => MenuFace::Heavy,
                Face::Bold => MenuFace::Bold,
                Face::Semibold => MenuFace::Semibold,
            },
            color: text.color.to_vec(),
            glow: text.glow.to_vec(),
            align: match text.align {
                Align::Left => MenuAlign::Left,
                Align::Center => MenuAlign::Center,
                Align::Right => MenuAlign::Right,
            },
        }
    }
}

impl From<&Scene> for MenuScene {
    fn from(scene: &Scene) -> Self {
        Self {
            shapes: scene.shapes.iter().map(MenuShape::from).collect(),
            texts: scene.texts.iter().map(MenuText::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opence_halo_ui::scene::hex;

    #[test]
    fn a_scene_crosses_whole() {
        let mut text = Text::new(
            "CAMPAIGN",
            [1920.0, 1185.0],
            108.0,
            Face::Heavy,
            hex(0xF2F6FA, 1.0),
        );
        text.align = Align::Center;
        let scene = Scene {
            shapes: vec![Shape::fill(
                [0.0, 198.0, 3840.0, 1638.0],
                [0.0, 0.0, 0.0, 0.45],
            )],
            texts: vec![text],
            ..Scene::default()
        };
        let crossed = MenuScene::from(&scene);
        assert_eq!(crossed.shapes[0].rect, [0.0, 198.0, 3840.0, 1638.0]);
        assert_eq!(crossed.shapes[0].radii, [0.0; 4]);
        assert_eq!(crossed.texts[0].text, "CAMPAIGN");
        assert_eq!(crossed.texts[0].face, MenuFace::Heavy);
        assert_eq!(crossed.texts[0].align, MenuAlign::Center);
        assert_eq!(crossed.texts[0].color.len(), 4);
    }
}
