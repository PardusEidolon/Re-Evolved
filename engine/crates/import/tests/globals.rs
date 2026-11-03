//! The game globals bloodgulch's level imports. Passes without
//! assets/maps/bloodgulch.map.

use std::error::Error;
use std::path::Path;

use opence_content::Level;
use opence_import::{import_level, Cache};

fn bloodgulch() -> Result<Option<Level>, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/maps/bloodgulch.map");
    let Ok(file) = std::fs::read(&path) else {
        return Ok(None); // (no game data here)
    };
    Ok(Some(import_level(&Cache::open(file)?)?))
}

#[test]
fn the_difficulty_block_holds_the_originals_35_values() -> Result<(), Box<dyn Error>> {
    let Some(level) = bloodgulch()? else {
        return Ok(());
    };
    // `NUMBER_OF_GAME_DIFFICULTY_VALUES`: enemy damage to unused34, then
    // 21 words of padding, which are not a value.
    assert_eq!(level.globals.difficulty.len(), 35);
    Ok(())
}
