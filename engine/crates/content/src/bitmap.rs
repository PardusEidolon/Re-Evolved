//! Bitmaps: a tag's images, and how they are grouped into sequences.

use rkyv::{Archive, Deserialize, Serialize};

use crate::TagRef;

/// A bitmap tag.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct Bitmap {
    /// Its tag.
    pub tag: TagRef,
    /// 2D textures, 3D textures, cube maps, sprites or interface bitmaps.
    pub kind: i16,
    /// The format it was imported to.
    pub format: i16,
    /// What it is for (alpha blend, default, height map, ...).
    pub usage: i16,
    /// Its flags.
    pub flags: u16,
    /// Its height map's height, for a bump map.
    pub bump_height: f32,
    /// Its sequences of images (an animation, a font's glyphs).
    pub sequences: Vec<Sequence>,
    /// Its images.
    pub images: Vec<BitmapImage>,
}

impl Bitmap {
    /// The image frame `frame` of sequence `sequence` is drawn from, as
    /// `bitmap_group_get_bitmap_from_sequence` finds it: the sequence
    /// wrapping through the bitmap's, a run of images wrapping the frame
    /// through it and a sequence of sprites taking its sprite's, and the
    /// frame itself for a bitmap without sequences, a negative sequence or
    /// a run that lands on no image. `None` past the bitmap's images.
    pub fn sequence_image(&self, sequence: i16, frame: usize) -> Option<u16> {
        let wrapped = usize::try_from(sequence)
            .ok()
            .filter(|_| !self.sequences.is_empty())
            .and_then(|index| self.sequences.get(index % self.sequences.len()));
        let image = match wrapped {
            Some(sequence) if sequence.image_count > 0 => {
                // (the game's first is a short: without one it is -1, and a
                // run from it that lands on -1 is no image)
                let first = sequence.first_image.map_or(-1, i32::from);
                let step =
                    u16::try_from(frame % usize::from(sequence.image_count)).map_or(0, i32::from);
                usize::try_from(first + step).ok()
            }
            Some(sequence) => sequence
                .sprites
                .get(frame)
                .map(|sprite| usize::from(sprite.image)),
            None => None,
        }
        .unwrap_or(frame);
        (image < self.images.len())
            .then(|| u16::try_from(image).ok())
            .flatten()
    }
}

/// A sequence of a bitmap's images.
#[derive(Clone, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct Sequence {
    /// Its name.
    pub name: String,
    /// Its first image.
    pub first_image: Option<u16>,
    /// How many images, from the first.
    pub image_count: u16,
    /// Its sprites, for a sprite bitmap.
    pub sprites: Vec<Sprite>,
}

/// A sprite: part of an image.
#[derive(Clone, Copy, Debug, PartialEq, Archive, Serialize, Deserialize)]
pub struct Sprite {
    /// The image it is in.
    pub image: u16,
    /// Its left, right, top and bottom, from 0 to 1.
    pub bounds: [f32; 4],
    /// Its registration point.
    pub registration_point: [f32; 2],
}

/// One image of a bitmap, with its mipmaps.
#[derive(Clone, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub struct BitmapImage {
    /// Its width.
    pub width: u16,
    /// Its height.
    pub height: u16,
    /// Its depth (1 but for 3D textures).
    pub depth: u16,
    /// 2D texture, 3D texture or cube map.
    pub kind: i16,
    /// Its pixels' format (A8, R5G6B5, DXT1, P8, ...).
    pub format: i16,
    /// Its flags: power-of-two dimensions, compressed, palettized,
    /// swizzled, linear, in the resource map.
    pub flags: u16,
    /// Its registration point.
    pub registration_point: [i16; 2],
    /// How many mipmaps it has below the first.
    pub mipmap_count: u16,
    /// Its pixels.
    pub pixels: Pixels,
}

/// Where an image's pixels are.
#[derive(Clone, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub enum Pixels {
    /// In Custom Edition's `bitmaps.map`.
    ResourceMap(#[doc = "Where."] ResourceRange),
    /// Converted for the GPU (the native format's).
    Texture(#[doc = "The texture."] Texture),
}

/// Bytes in a resource map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub struct ResourceRange {
    /// Where, in that file.
    pub offset: u32,
    /// How many bytes.
    pub size: u32,
}

/// A texture's texels, as the GPU takes them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub enum TextureFormat {
    /// DXT1's 8-byte blocks, as BC1.
    Bc1,
    /// DXT3's 16-byte blocks, as BC2.
    Bc2,
    /// DXT5's 16-byte blocks, as BC3.
    Bc3,
    /// Red, green, blue and alpha, a byte each.
    Rgba8,
}

impl TextureFormat {
    /// The bytes of one row of `width` texels (of 4x4 blocks, for BC).
    pub fn row_bytes(self, width: u32) -> usize {
        let blocks = width.div_ceil(4) as usize;
        match self {
            Self::Bc1 => blocks * 8,
            Self::Bc2 | Self::Bc3 => blocks * 16,
            Self::Rgba8 => width as usize * 4,
        }
    }

    /// The rows of a level `height` texels high (of blocks, for BC).
    pub fn rows(self, height: u32) -> usize {
        match self {
            Self::Rgba8 => height as usize,
            _ => height.div_ceil(4) as usize,
        }
    }
}

/// A texture: its levels, largest first, unswizzled and tightly packed.
#[derive(Clone, Debug, PartialEq, Eq, Archive, Serialize, Deserialize)]
pub struct Texture {
    /// Its texels' format.
    pub format: TextureFormat,
    /// The first level's width.
    pub width: u16,
    /// The first level's height.
    pub height: u16,
    /// The first level's depth (1 but for 3D textures).
    pub depth: u16,
    /// 6 for a cube map, else 1.
    pub faces: u8,
    /// Each level's texels: every face's (in +x, -x, +y, -y, +z, -z
    /// order), each face's slices first to last, each slice's rows top to
    /// bottom.
    pub levels: Vec<Vec<u8>>,
}

impl Texture {
    /// A level's width, height and depth.
    pub fn level_size(&self, level: usize) -> [u32; 3] {
        let shrink = |size: u16| (u32::from(size) >> level.min(31)).max(1);
        [shrink(self.width), shrink(self.height), shrink(self.depth)]
    }

    /// The bytes a level holds.
    pub fn level_bytes(&self, level: usize) -> usize {
        let [width, height, depth] = self.level_size(level);
        self.format.row_bytes(width)
            * self.format.rows(height)
            * depth as usize
            * usize::from(self.faces)
    }
}
