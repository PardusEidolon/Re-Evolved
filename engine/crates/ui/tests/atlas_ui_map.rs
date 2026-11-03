//! Every font of the user's ui.map packs into one atlas holding each of its
//! glyphs; skipped when assets/maps/ui.map is missing.

use std::path::Path;

use opence_ui::atlas::Atlas;

#[test]
fn every_font_of_the_ui_map_packs_into_an_atlas() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/maps/ui.map");
    let Ok(bytes) = std::fs::read(&path) else {
        eprintln!("skipped: {} is missing", path.display());
        return;
    };
    let ui = opence_import::Cache::open(bytes)
        .and_then(|cache| opence_import::ui::import_ui_tags(&cache));
    assert!(ui.is_ok(), "ui.map's tags: {:?}", ui.err());
    let ui = ui.unwrap_or_default();
    assert!(!ui.fonts.is_empty(), "ui.map has no fonts");
    for font in &ui.fonts {
        let atlas = Atlas::build(font);
        assert_eq!(
            atlas.size[1].count_ones(),
            1,
            "{}: height not a power of two",
            font.tag.path
        );
        for glyph in &font.glyphs {
            assert!(
                atlas.glyph(glyph.character).is_some(),
                "{}: {} not placed",
                font.tag.path,
                glyph.character
            );
        }
    }
}
