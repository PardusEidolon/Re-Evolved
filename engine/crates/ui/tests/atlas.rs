//! A font's distance fields packed into one atlas (`opence_ui::atlas`):
//! every glyph in it once, none overlapping, each texel its field's, the
//! same font always the same atlas, and kept on disk by the map's hash.

use std::fs;

use opence_content::{Font, Glyph, Group, TagRef};
use opence_ui::atlas::{atlas_path, cached, Atlas, Placed, ATLAS_WIDTH};

/// a glyph of `character` `size` texels of coverage, each texel of its
/// field its own row and column mixed with the character, so that a
/// texel out of place is seen
fn glyph(character: u16, size: [i16; 2]) -> Glyph {
    let width = (size[0] + 2 * Font::PADDING) as usize;
    let height = (size[1] + 2 * Font::PADDING) as usize;
    let distance = (0..width * height)
        .map(|at| ((at % width) * 7 + (at / width) * 13 + usize::from(character)) as u8 | 1)
        .collect();
    Glyph {
        character,
        advance: size[0] + 1,
        size,
        origin: [0, size[1]],
        distance,
    }
}

fn font(glyphs: Vec<Glyph>) -> Font {
    let mut characters: Vec<(u16, u32)> = glyphs
        .iter()
        .enumerate()
        .map(|(index, glyph)| (glyph.character, index as u32))
        .collect();
    characters.sort_unstable();
    characters.dedup_by_key(|&mut (character, _)| character);
    Font {
        tag: TagRef {
            group: Group(*b"font"),
            path: "ui\\shell\\fonts\\large".into(),
        },
        flags: 0,
        ascent: 12,
        descent: 4,
        leading_height: 0,
        leading_width: 1,
        styles: [None, None, None, None],
        glyphs,
        characters,
    }
}

/// the printable ASCII characters, of sizes that vary
fn ascii() -> Font {
    font(
        (32u16..127)
            .map(|c| glyph(c, [(c % 17) as i16, (c % 23) as i16]))
            .collect(),
    )
}

fn overlap(a: &Placed, b: &Placed) -> bool {
    let apart = |axis: usize| {
        a.place[axis] + a.size[axis] <= b.place[axis]
            || b.place[axis] + b.size[axis] <= a.place[axis]
    };
    !(apart(0) || apart(1))
}

#[test]
fn each_glyph_is_placed_once_inside_the_atlas_none_overlapping() {
    let atlas = Atlas::build(&ascii());
    assert_eq!(atlas.glyphs.len(), 95);
    assert_eq!(atlas.size[0], ATLAS_WIDTH);
    assert!(atlas.size[1].is_power_of_two());
    assert_eq!(atlas.texels.len(), (atlas.size[0] * atlas.size[1]) as usize);
    for (index, a) in atlas.glyphs.iter().enumerate() {
        assert!(a.place[0] + a.size[0] < atlas.size[0]);
        assert!(a.place[1] + a.size[1] < atlas.size[1]);
        for b in &atlas.glyphs[index + 1..] {
            assert!(!overlap(a, b), "{a:?} overlaps {b:?}");
        }
    }
    let characters: Vec<u16> = atlas.glyphs.iter().map(|g| g.character).collect();
    assert_eq!(characters, (32..127).collect::<Vec<u16>>());
}

#[test]
fn each_texel_of_a_glyph_is_its_fields() {
    let font = ascii();
    let atlas = Atlas::build(&font);
    for glyph in &font.glyphs {
        let Some(placed) = atlas.glyph(glyph.character) else {
            panic!("{} not placed", glyph.character);
        };
        let [width, height] = glyph.field_size();
        assert_eq!(placed.size, [width as u32, height as u32]);
        for row in 0..height {
            for column in 0..width {
                let at = (placed.place[1] as usize + row) * ATLAS_WIDTH as usize
                    + placed.place[0] as usize
                    + column;
                assert_eq!(atlas.texels[at], glyph.distance[row * width + column]);
            }
        }
    }
    // and where no glyph is, nothing
    let covered: usize = atlas
        .glyphs
        .iter()
        .map(|g| (g.size[0] * g.size[1]) as usize)
        .sum();
    let written = atlas.texels.iter().filter(|&&texel| texel != 0).count();
    assert_eq!(written, covered);
}

#[test]
fn the_same_font_is_always_the_same_atlas() {
    let font = ascii();
    let mut shuffled = font.clone();
    shuffled.glyphs.reverse();
    assert_eq!(Atlas::build(&font), Atlas::build(&font));
    assert_eq!(Atlas::build(&font).glyphs, Atlas::build(&shuffled).glyphs);
}

#[test]
fn a_glyph_not_whole_too_wide_or_twice_is_left_out() {
    let mut short = glyph(u16::from(b'b'), [3, 3]);
    short.distance.pop();
    let wide = glyph(u16::from(b'w'), [ATLAS_WIDTH as i16, 2]);
    let font = font(vec![
        glyph(u16::from(b'a'), [3, 3]),
        short,
        wide,
        glyph(u16::from(b'a'), [9, 9]),
    ]);
    let atlas = Atlas::build(&font);
    assert_eq!(atlas.glyphs.len(), 1);
    assert_eq!(atlas.glyph(u16::from(b'a')).map(|g| g.size), Some([11, 11]));
    assert_eq!(atlas.glyph(u16::from(b'b')), None);
    // a font with no glyphs is an atlas of one shelf of nothing
    let empty = Atlas::build(&self::font(Vec::new()));
    assert!(empty.glyphs.is_empty());
    assert_eq!(empty.texels.len(), (empty.size[0] * empty.size[1]) as usize);
}

#[test]
fn an_atlas_is_read_back_as_written_and_nothing_else_is() {
    let atlas = Atlas::build(&ascii());
    let bytes = atlas.to_bytes();
    assert_eq!(Atlas::from_bytes(&bytes), Some(atlas.clone()));
    // cut anywhere, or one byte more
    for cut in [0, 3, 4, 19, 20, 38, bytes.len() - 1] {
        assert_eq!(Atlas::from_bytes(&bytes[..cut]), None, "cut at {cut}");
    }
    let mut longer = bytes.clone();
    longer.push(0);
    assert_eq!(Atlas::from_bytes(&longer), None);
    // a glyph outside the texture, another version, glyphs out of order
    let mut outside = bytes.clone();
    outside[22..26].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(Atlas::from_bytes(&outside), None);
    let mut version = bytes.clone();
    version[4] = 9;
    assert_eq!(Atlas::from_bytes(&version), None);
    let mut order = bytes;
    order[20..22].copy_from_slice(&200u16.to_le_bytes());
    assert_eq!(Atlas::from_bytes(&order), None);
}

#[test]
fn an_atlas_is_built_once_and_then_read_from_its_file() {
    let directory = std::env::temp_dir().join(format!("opence-atlas-{}", std::process::id()));
    let _ = fs::remove_dir_all(&directory);
    let font = ascii();
    let (built, kept) = cached(&directory, "0123abcd", &font);
    assert!(kept.is_ok());
    let path = atlas_path(&directory, "0123abcd", &font.tag.path);
    assert_eq!(fs::read(&path).ok(), Some(built.to_bytes()));
    // read from the file: a file holding another atlas is what is read
    let other = Atlas::build(&self::font(vec![glyph(65, [1, 1])]));
    assert!(fs::write(&path, other.to_bytes()).is_ok());
    assert_eq!(cached(&directory, "0123abcd", &font).0, other);
    // a file not whole is built again, and kept again
    assert!(fs::write(&path, b"OCSA").is_ok());
    assert_eq!(cached(&directory, "0123abcd", &font).0, built);
    assert_eq!(fs::read(&path).ok(), Some(built.to_bytes()));
    let _ = fs::remove_dir_all(&directory);
}

#[test]
fn a_font_path_never_leaves_its_directory() {
    let directory = std::path::Path::new("/cache/fonts");
    let path = atlas_path(directory, "../../etc", "ui\\..\\fonts/x");
    assert_eq!(path.parent(), Some(directory));
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("______etc-ui____fonts_x.sdfatlas")
    );
}
