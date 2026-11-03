//! A font's glyphs as one texture: each glyph's signed distance field
//! (built from its coverage when the map is imported,
//! [`opence_content::Glyph::distance`]) packed into an atlas the UI
//! plugin draws its text from at any size. An atlas is built behind the
//! loading screen and kept on disk, named by the map's hash and the font,
//! so a map loads its fonts' atlases once.
//!
//! The packing is fixed by the font alone: glyphs are placed in shelves,
//! tallest first, each with a texel of empty space around it, so the same
//! font always gives the same texture.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use opence_content::{Font, Glyph};

/// An atlas's width, in texels.
pub const ATLAS_WIDTH: u32 = 1024;
/// The empty texels between glyphs, so that filtering at one glyph's edge
/// reads nothing of the next.
const GUTTER: u32 = 1;
/// What an atlas file begins with, and its version.
const MAGIC: &[u8; 4] = b"OCSA";
const VERSION: u32 = 1;
/// The most glyphs an atlas file is read with (a font tag's characters
/// are 16 bits).
const MAXIMUM_GLYPHS: u32 = 0x1_0000;
/// The tallest atlas read from a file.
const MAXIMUM_HEIGHT: u32 = 16_384;

/// Where a glyph's distance field is in its atlas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    /// Its character.
    pub character: u16,
    /// Its field's top left, in texels.
    pub place: [u32; 2],
    /// Its field's width and height, in texels.
    pub size: [u32; 2],
}

/// A font's distance fields in one texture of one byte a texel, 128 on a
/// glyph's edge (as [`Glyph::distance`]), 0 where no glyph is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atlas {
    /// Its width and height, in texels; its height is a power of two.
    pub size: [u32; 2],
    /// Its texels, row by row.
    pub texels: Vec<u8>,
    /// Each glyph's place, sorted by character.
    pub glyphs: Vec<Placed>,
}

impl Atlas {
    /// The atlas of `font`'s glyphs. A glyph whose field is not as large as
    /// its size says, or is wider than an atlas, is left out.
    #[must_use]
    pub fn build(font: &Font) -> Self {
        let mut characters = std::collections::BTreeSet::new();
        let mut glyphs: Vec<(&Glyph, [u32; 2])> = font
            .glyphs
            .iter()
            // (a character's first glyph, as `Font::characters` would
            // find only one)
            .filter(|glyph| characters.insert(glyph.character))
            .filter_map(|glyph| {
                let [width, height] = glyph.field_size();
                let size = [u32::try_from(width).ok()?, u32::try_from(height).ok()?];
                let whole = width.checked_mul(height)? == glyph.distance.len();
                (whole && size[0] + 2 * GUTTER <= ATLAS_WIDTH && size[1] > 0)
                    .then_some((glyph, size))
            })
            .collect();
        // (tallest first; a font has each character once, so the order is
        // fixed)
        glyphs.sort_by_key(|&(glyph, size)| (std::cmp::Reverse(size[1]), glyph.character));

        let mut placed = Vec::with_capacity(glyphs.len());
        let (mut x, mut y, mut shelf) = (GUTTER, GUTTER, 0);
        for &(glyph, size) in &glyphs {
            if x + size[0] + GUTTER > ATLAS_WIDTH {
                (x, y, shelf) = (GUTTER, y + shelf + GUTTER, 0);
            }
            placed.push(Placed {
                character: glyph.character,
                place: [x, y],
                size,
            });
            x += size[0] + GUTTER;
            shelf = shelf.max(size[1]);
        }
        let height = (y + shelf + GUTTER).next_power_of_two();
        let mut texels = vec![0; ATLAS_WIDTH as usize * height as usize];
        for (&(glyph, size), at) in glyphs.iter().zip(&placed) {
            for (row, line) in glyph.distance.chunks_exact(size[0] as usize).enumerate() {
                let start =
                    (at.place[1] as usize + row) * ATLAS_WIDTH as usize + at.place[0] as usize;
                if let Some(target) = texels.get_mut(start..start + line.len()) {
                    target.copy_from_slice(line);
                }
            }
        }
        placed.sort_by_key(|glyph| glyph.character);
        Self {
            size: [ATLAS_WIDTH, height],
            texels,
            glyphs: placed,
        }
    }

    /// Where `character`'s glyph is, if the atlas has it.
    #[must_use]
    pub fn glyph(&self, character: u16) -> Option<&Placed> {
        let index = self
            .glyphs
            .binary_search_by_key(&character, |glyph| glyph.character)
            .ok()?;
        self.glyphs.get(index)
    }

    /// The atlas as its file holds it: [`MAGIC`], the version, its size and
    /// its glyphs' count, each glyph, then its texels; little-endian.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(20 + self.glyphs.len() * 18 + self.texels.len());
        bytes.extend_from_slice(MAGIC);
        for word in [
            VERSION,
            self.size[0],
            self.size[1],
            u32::try_from(self.glyphs.len()).unwrap_or(u32::MAX),
        ] {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        for glyph in &self.glyphs {
            bytes.extend_from_slice(&glyph.character.to_le_bytes());
            for word in [glyph.place[0], glyph.place[1], glyph.size[0], glyph.size[1]] {
                bytes.extend_from_slice(&word.to_le_bytes());
            }
        }
        bytes.extend_from_slice(&self.texels);
        bytes
    }

    /// The atlas a file holds, if it is one this version wrote whole: each
    /// glyph inside the texture, sorted, and the texels all there.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut reader = Reader(bytes);
        if reader.take(4)? != MAGIC || reader.u32()? != VERSION {
            return None;
        }
        let size = [reader.u32()?, reader.u32()?];
        let count = reader.u32()?;
        if size[0] != ATLAS_WIDTH || size[1] > MAXIMUM_HEIGHT || count > MAXIMUM_GLYPHS {
            return None;
        }
        let mut glyphs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            let character = reader.u16()?;
            let place = [reader.u32()?, reader.u32()?];
            let glyph_size = [reader.u32()?, reader.u32()?];
            let inside = |axis: usize| {
                place[axis]
                    .checked_add(glyph_size[axis])
                    .is_some_and(|end| end <= size[axis])
            };
            if !(inside(0) && inside(1)) {
                return None;
            }
            glyphs.push(Placed {
                character,
                place,
                size: glyph_size,
            });
        }
        let sorted = glyphs
            .windows(2)
            .all(|pair| pair[0].character < pair[1].character);
        let texels = reader.take(size[0] as usize * size[1] as usize)?;
        (sorted && reader.0.is_empty()).then(|| Self {
            size,
            texels: texels.to_vec(),
            glyphs,
        })
    }
}

/// The atlas of `font` for the map `map_hash`, from `directory` if it has
/// been built there before, else built and kept there. A file that cannot
/// be read or is not whole is built again; a failure to keep it is the
/// only error, and the atlas is still built.
pub fn cached(directory: &Path, map_hash: &str, font: &Font) -> (Atlas, io::Result<()>) {
    let path = atlas_path(directory, map_hash, &font.tag.path);
    if let Some(atlas) = fs::read(&path)
        .ok()
        .and_then(|bytes| Atlas::from_bytes(&bytes))
    {
        return (atlas, Ok(()));
    }
    let atlas = Atlas::build(font);
    let kept = fs::create_dir_all(directory).and_then(|()| {
        // (written whole, then renamed, so a reader never finds half)
        let partial = path.with_extension("partial");
        fs::write(&partial, atlas.to_bytes())?;
        fs::rename(&partial, &path)
    });
    (atlas, kept)
}

/// The file of the atlas of the font tag at `font` for the map `map_hash`:
/// the tag's path with each character that is not a letter or a digit as
/// `_`, so that a name never leaves `directory`.
#[must_use]
pub fn atlas_path(directory: &Path, map_hash: &str, font: &str) -> PathBuf {
    let safe = |text: &str| -> String {
        text.chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    };
    directory.join(format!("{}-{}.sdfatlas", safe(map_hash), safe(font)))
}

/// a reader of little-endian words that answers `None` past its end
struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let (taken, rest) = self.0.split_at_checked(len)?;
        self.0 = rest;
        Some(taken)
    }

    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }

    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
}
