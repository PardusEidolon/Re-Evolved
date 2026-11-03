//! The baked lightmaps the game draws in place of its own
//! (`lighting.bounce=baked`), read from the cache where `opence bake` wrote
//! them: `<cache>/bake/<map>.bsp<N>.<key>.bake`.
//!
//! A bake is read only as it was written: a file that is not one, or is
//! another baker's, is left out, and the game draws that BSP stock. The
//! render sink checks each page's size against the game's own before it
//! swaps it in ([`opence_render::BakedPages`]).

use std::path::Path;

use opence_bake::BakedLighting;
use opence_render::BakedTexture;

/// The folder under the cache that holds the bakes.
pub const BAKE_FOLDER: &str = "bake";

/// The BSP a bake's file name is for, if it is one of `map`'s bakes.
fn bsp_of(file: &str, map: &str) -> Option<u32> {
    let rest = file.strip_prefix(map)?.strip_prefix(".bsp")?;
    let (bsp, rest) = rest.split_once('.')?;
    let key = rest.strip_suffix(".bake")?;
    let hex = key.len() == 16 && key.bytes().all(|byte| byte.is_ascii_hexdigit());
    hex.then(|| bsp.parse().ok()).flatten()
}

/// Every page of `map`'s bakes under `cache`, as the host draws them (in
/// file-name order, so a later bake of a BSP comes after an earlier one).
#[must_use]
pub fn baked_textures(cache: &Path, map: &str) -> Vec<BakedTexture> {
    let Ok(entries) = std::fs::read_dir(cache.join(BAKE_FOLDER)) else {
        return Vec::new();
    };
    let mut files: Vec<(String, u32)> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            let bsp = bsp_of(&name, map)?;
            Some((name, bsp))
        })
        .collect();
    files.sort();
    files
        .into_iter()
        .filter_map(|(name, bsp)| {
            let bytes = std::fs::read(cache.join(BAKE_FOLDER).join(name)).ok()?;
            BakedLighting::decode(&bytes).ok().map(|baked| (bsp, baked))
        })
        .flat_map(|(bsp, baked)| {
            baked.pages.into_iter().map(move |page| BakedTexture {
                bsp,
                page: page.page,
                width: page.width,
                height: page.height,
                texels: page.shown_texels(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use opence_bake::{BakedPage, ProbeGrid};

    fn lighting(key: u64) -> BakedLighting {
        BakedLighting {
            key,
            exposure: 1.0,
            pages: vec![BakedPage {
                page: 2,
                width: 2,
                height: 1,
                // (π, then 0: shown as 1 and 0)
                irradiance: vec![0x4248, 0x4248, 0x4248, 0x3c00, 0, 0, 0, 0x3c00],
                directions: vec![0; 8],
            }],
            grid: ProbeGrid {
                origin: [0.0; 3],
                spacing: 1.0,
                counts: [0; 3],
            },
            probes: Vec::new(),
            sun: None,
            reflections: Vec::new(),
            radiance: Vec::new(),
        }
    }

    fn folder(name: &str) -> std::path::PathBuf {
        let folder =
            std::env::temp_dir().join(format!("opence_bakes_{name}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        let _ = std::fs::create_dir_all(folder.join(BAKE_FOLDER));
        folder
    }

    #[test]
    fn only_a_maps_own_bake_files_are_named() {
        assert_eq!(
            bsp_of("bloodgulch.bsp0.00000000deadbeef.bake", "bloodgulch"),
            Some(0)
        );
        assert_eq!(bsp_of("a30.bsp12.0123456789abcdef.bake", "a30"), Some(12));
        assert_eq!(bsp_of("a30.bsp1.0123456789abcdef.bake", "a3"), None);
        assert_eq!(bsp_of("a30.bsp1.0123.bake", "a30"), None);
        assert_eq!(bsp_of("a30.bspx.0123456789abcdef.bake", "a30"), None);
        assert_eq!(bsp_of("a30.bsp1.0123456789abcdef.bake.tmp", "a30"), None);
    }

    #[test]
    fn a_maps_bakes_are_read_as_shown_pages() {
        let cache = folder("read");
        let bake = cache.join(BAKE_FOLDER);
        let written = std::fs::write(
            bake.join(BakedLighting::file_name("devmap", 3, 7)),
            lighting(7).encode(),
        );
        assert!(written.is_ok());
        let _ = std::fs::write(
            bake.join(BakedLighting::file_name("other", 0, 7)),
            lighting(7).encode(),
        );
        let _ = std::fs::write(
            bake.join(BakedLighting::file_name("devmap", 4, 8)),
            b"not a bake",
        );

        let pages = baked_textures(&cache, "devmap");
        assert_eq!(pages.len(), 1);
        let page = &pages[0];
        assert_eq!((page.bsp, page.page, page.width, page.height), (3, 2, 2, 1));
        assert_eq!(page.texels.len(), 16);
        // (the first texel shown as 1.0 a channel, alpha 1)
        assert_eq!(
            page.texels[..8],
            [0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c]
        );
        assert_eq!(page.texels[8..14], [0; 6]);
        let _ = std::fs::remove_dir_all(&cache);
    }

    #[test]
    fn no_bake_folder_is_no_pages() {
        let cache = std::env::temp_dir().join("opence_bakes_none_here");
        assert!(baked_textures(&cache, "devmap").is_empty());
    }
}
