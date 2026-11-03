//! The baked lightmaps drawn in place of the game's own
//! (`lighting.bounce=baked`).
//!
//! The game names each lightmap page as it binds it (LIGHTMAP_PAGE); a
//! fragment texture bind of that texture is turned to the bake's page, which
//! is created and uploaded the first time it is drawn. A page is swapped only
//! when the bake has it at a whole multiple of the stock page's size (the
//! bake draws at twice it), so a bake of another map, or of another version
//! of this one, draws stock.

use rustc_hash::FxHashMap;

use crate::{Command, ObjectKind, PixelFormat, Stage, TextureKind};

/// The first id of the baked textures: above every id the game's device
/// gives out.
pub const FIRST_BAKED_TEXTURE: u32 = 0x4000_0000;

/// One baked lightmap page, as the host draws it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BakedTexture {
    /// the structure bsp's index
    pub bsp: u32,
    /// the bitmap's index in the bsp's lightmap group
    pub page: u32,
    /// its width in texels
    pub width: u32,
    /// its height in texels
    pub height: u32,
    /// its texels, rows top first, each four little-endian halves (red,
    /// green, blue, alpha)
    pub texels: Vec<u8>,
}

/// The baked pages, and what the game's textures are.
#[derive(Debug, Default)]
pub struct BakedPages {
    /// the pages by (bsp, page), with their texture once it is made
    pages: FxHashMap<(u32, u32), (BakedTexture, Option<u32>)>,
    /// the game's plane textures' sizes
    sizes: FxHashMap<u32, (u32, u32)>,
    /// the game's lightmap textures drawn as a baked one
    swapped: FxHashMap<u32, u32>,
    /// the next baked texture's id
    next: u32,
}

impl BakedPages {
    /// The swap for `pages` (a later page of the same bsp and index replaces
    /// an earlier one).
    #[must_use]
    pub fn new(pages: Vec<BakedTexture>) -> Self {
        Self {
            pages: pages
                .into_iter()
                .filter(|page| {
                    u64::from(page.width) * u64::from(page.height) * 8 == page.texels.len() as u64
                })
                .map(|page| ((page.bsp, page.page), (page, None)))
                .collect(),
            next: FIRST_BAKED_TEXTURE,
            ..Self::default()
        }
    }

    /// Whether there are any pages to swap in.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// `commands` and their `data` with the baked pages drawn in place of
    /// the game's: the pages first drawn in this batch are created and
    /// uploaded at its start, their texels before the batch's own.
    #[must_use]
    pub fn apply(&mut self, commands: Vec<Command>, data: Vec<u8>) -> (Vec<Command>, Vec<u8>) {
        let mut made = Vec::new();
        let mut texels = Vec::new();
        let mut out = Vec::with_capacity(commands.len());
        for mut command in commands {
            match &mut command {
                Command::CreateTexture {
                    id,
                    kind,
                    width,
                    height,
                    ..
                } => {
                    if *kind == TextureKind::Plane {
                        self.sizes.insert(*id, (*width, *height));
                    } else {
                        self.sizes.remove(id);
                    }
                    self.swapped.remove(id);
                }
                Command::Destroy {
                    kind: ObjectKind::Texture,
                    id,
                } => {
                    self.sizes.remove(id);
                    self.swapped.remove(id);
                }
                Command::LightmapPage { texture, bsp, page } => {
                    match self.baked(*texture, *bsp, *page, &mut made, &mut texels) {
                        Some(baked) => self.swapped.insert(*texture, baked),
                        None => self.swapped.remove(texture),
                    };
                }
                Command::SetTexture {
                    stage: Stage::Fragment,
                    id,
                    ..
                } => {
                    if let Some(&baked) = self.swapped.get(id) {
                        *id = baked;
                    }
                }
                _ => {}
            }
            out.push(command);
        }
        if made.is_empty() {
            return (out, data);
        }
        made.extend(out);
        texels.extend(data);
        (made, texels)
    }

    /// The baked texture drawn for `texture`, page `page` of bsp `bsp`
    /// (made now, onto `made` and `texels`, if it is drawn for the first
    /// time), or none when the bake has no page at a whole multiple of
    /// that size.
    fn baked(
        &mut self,
        texture: u32,
        bsp: u32,
        page: u32,
        made: &mut Vec<Command>,
        texels: &mut Vec<u8>,
    ) -> Option<u32> {
        let (width, height) = *self.sizes.get(&texture)?;
        let (baked, id) = self.pages.get_mut(&(bsp, page))?;
        // (a whole multiple of the stock size, the same in both directions)
        let scale = baked.width / width.max(1);
        if scale == 0 || (baked.width, baked.height) != (width * scale, height * scale) {
            return None;
        }
        if let Some(id) = *id {
            return Some(id);
        }
        let made_id = self.next;
        self.next += 1;
        *id = Some(made_id);
        made.push(Command::CreateTexture {
            id: made_id,
            kind: TextureKind::Plane,
            format: PixelFormat::Rgba16f,
            width: baked.width,
            height: baked.height,
            depth: 1,
            levels: 1,
            sampled: true,
            render_target: false,
            samples: 1,
        });
        made.push(Command::UploadTexture {
            id: made_id,
            level: 0,
            slice: 0,
            x: 0,
            y: 0,
            z: 0,
            width: baked.width,
            height: baked.height,
            depth: 1,
            address: 0,
            bytes_per_row: baked.width * 8,
            bytes_per_image: baked.width * baked.height * 8,
            length: baked.texels.len() as u32,
        });
        texels.extend_from_slice(&baked.texels);
        Some(made_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(bsp: u32, page: u32, width: u32, height: u32) -> BakedTexture {
        BakedTexture {
            bsp,
            page,
            width,
            height,
            texels: vec![0x3c; (width * height * 8) as usize],
        }
    }

    fn create(id: u32, width: u32, height: u32) -> Command {
        Command::CreateTexture {
            id,
            kind: TextureKind::Plane,
            format: PixelFormat::Rgba8,
            width,
            height,
            depth: 1,
            levels: 1,
            sampled: true,
            render_target: false,
            samples: 1,
        }
    }

    fn bind(stage: Stage, id: u32) -> Command {
        Command::SetTexture { stage, slot: 2, id }
    }

    fn named(texture: u32, bsp: u32, page: u32) -> Command {
        Command::LightmapPage { texture, bsp, page }
    }

    fn bound(commands: &[Command]) -> Vec<u32> {
        commands
            .iter()
            .filter_map(|command| match command {
                Command::SetTexture { id, .. } => Some(*id),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn a_named_page_is_drawn_from_the_bake_and_made_once() {
        let mut pages = BakedPages::new(vec![page(0, 3, 4, 2)]);
        let (commands, data) = pages.apply(
            vec![
                create(9, 4, 2),
                named(9, 0, 3),
                bind(Stage::Fragment, 9),
                bind(Stage::Vertex, 9),
            ],
            vec![1, 2],
        );
        assert_eq!(bound(&commands), [FIRST_BAKED_TEXTURE, 9]);
        assert!(matches!(
            commands[0],
            Command::CreateTexture {
                id: FIRST_BAKED_TEXTURE,
                format: PixelFormat::Rgba16f,
                width: 4,
                height: 2,
                ..
            }
        ));
        assert!(matches!(
            commands[1],
            Command::UploadTexture {
                length: 64,
                bytes_per_row: 32,
                ..
            }
        ));
        assert_eq!(data.len(), 66);
        assert_eq!(data[64..], [1, 2]);

        let (commands, data) = pages.apply(vec![named(9, 0, 3), bind(Stage::Fragment, 9)], vec![]);
        assert_eq!(commands.len(), 2);
        assert_eq!(bound(&commands), [FIRST_BAKED_TEXTURE]);
        assert!(data.is_empty());
    }

    #[test]
    fn a_page_of_another_size_or_missing_draws_stock() {
        let mut pages = BakedPages::new(vec![page(0, 3, 4, 2)]);
        let (commands, _) = pages.apply(
            vec![
                create(9, 8, 2),
                named(9, 0, 3),
                bind(Stage::Fragment, 9),
                create(10, 4, 2),
                named(10, 1, 3),
                bind(Stage::Fragment, 10),
                bind(Stage::Fragment, 11),
            ],
            vec![],
        );
        assert_eq!(bound(&commands), [9, 10, 11]);
        assert_eq!(commands.len(), 7);
    }

    #[test]
    fn a_page_baked_at_a_whole_multiple_of_the_stock_size_is_drawn() {
        // (the bake draws its pages at twice the stock size; the lightmap's
        // texture coordinates run 0 to 1 whatever the size)
        let mut pages = BakedPages::new(vec![page(0, 1, 8, 4), page(0, 2, 8, 2)]);
        let (commands, _) = pages.apply(
            vec![
                create(9, 4, 2),
                named(9, 0, 1),
                bind(Stage::Fragment, 9),
                create(10, 4, 2),
                named(10, 0, 2),
                bind(Stage::Fragment, 10),
            ],
            vec![],
        );
        assert_eq!(bound(&commands), [FIRST_BAKED_TEXTURE, 10]);
    }

    #[test]
    fn a_destroyed_texture_is_no_longer_swapped() {
        let mut pages = BakedPages::new(vec![page(0, 0, 2, 2)]);
        let (commands, _) = pages.apply(
            vec![
                create(5, 2, 2),
                named(5, 0, 0),
                Command::Destroy {
                    kind: ObjectKind::Texture,
                    id: 5,
                },
                create(5, 2, 2),
                bind(Stage::Fragment, 5),
            ],
            vec![],
        );
        assert_eq!(bound(&commands), [5]);
    }

    #[test]
    fn a_page_named_again_as_another_drops_the_swap() {
        let mut pages = BakedPages::new(vec![page(0, 0, 2, 2)]);
        let (commands, _) = pages.apply(
            vec![
                create(5, 2, 2),
                named(5, 0, 0),
                bind(Stage::Fragment, 5),
                named(5, 0, 1),
                bind(Stage::Fragment, 5),
            ],
            vec![],
        );
        assert_eq!(bound(&commands), [FIRST_BAKED_TEXTURE, 5]);
    }

    #[test]
    fn pages_whose_texels_are_short_are_left_out() {
        let mut short = page(0, 0, 2, 2);
        short.texels.pop();
        assert!(BakedPages::new(vec![short]).is_empty());
    }
}
