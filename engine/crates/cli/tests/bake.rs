//! `opence bake MAP`: each BSP's lightmaps baked at import, written where
//! the game reads them (`lighting.bounce=baked`), older bakes of the same
//! BSP removed.

use std::path::PathBuf;

use opence_bake::{BakedLighting, BakedPage, ProbeGrid};
use opence_cli::{bake_level, write_bake, Error};

fn folder(name: &str) -> Result<PathBuf, Error> {
    let folder = std::env::temp_dir().join(format!("opence_cli_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder)?;
    Ok(folder)
}

fn lighting(key: u64) -> BakedLighting {
    BakedLighting {
        key,
        exposure: 1.0,
        pages: vec![BakedPage {
            page: 0,
            width: 1,
            height: 1,
            irradiance: vec![0x3c00; 4],
            directions: vec![0; 4],
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

#[test]
fn a_bake_replaces_its_bsps_older_bakes_only() -> Result<(), Error> {
    let folder = folder("write")?;
    let stale = folder.join(BakedLighting::file_name("devmap", 0, 1));
    let other_bsp = folder.join(BakedLighting::file_name("devmap", 1, 1));
    let other_map = folder.join(BakedLighting::file_name("othermap", 0, 1));
    for path in [&stale, &other_bsp, &other_map] {
        std::fs::write(path, b"old")?;
    }
    let path = write_bake(&folder, "devmap", 0, &lighting(2))?;
    assert_eq!(path, folder.join(BakedLighting::file_name("devmap", 0, 2)));
    assert!(!stale.exists());
    assert!(other_bsp.exists() && other_map.exists());
    let back = BakedLighting::decode(&std::fs::read(&path)?);
    assert_eq!(back.ok(), Some(lighting(2)));
    // (written again, the same one file, no partial left beside it)
    assert_eq!(write_bake(&folder, "devmap", 0, &lighting(2))?, path);
    assert_eq!(std::fs::read_dir(&folder)?.count(), 3);
    let _ = std::fs::remove_dir_all(&folder);
    Ok(())
}

#[test]
fn a_level_with_no_lightmap_pages_bakes_nothing() -> Result<(), Error> {
    let folder = folder("none")?;
    let level = opence_content::devmap::level(None);
    assert!(bake_level(&level, "devmap", &folder, 1)?.is_empty());
    let _ = std::fs::remove_dir_all(&folder);
    Ok(())
}

#[test]
fn blood_gulch_is_baked_a_file_a_bsp() -> Result<(), Error> {
    let level = opence_import::testing::stock_level_unheld("bloodgulch")
        .map_err(|error| Error::Refused(error.to_string()))?;
    let Some(level) = level else {
        return Ok(());
    };
    let folder = folder("gulch")?;
    let written = bake_level(&level, "bloodgulch", &folder, 1)?;
    assert_eq!(written.len(), 1);
    let baked = BakedLighting::decode(&std::fs::read(&written[0])?)
        .map_err(|error| Error::Refused(format!("{error:?}")))?;
    assert!(!baked.pages.is_empty());
    let _ = std::fs::remove_dir_all(&folder);
    Ok(())
}
