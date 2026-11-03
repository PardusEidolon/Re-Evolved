//! The tag fields the engine uses: the importer's labels traced to the
//! fields it fills, and those fields' uses outside the importer.

use std::collections::{BTreeMap, BTreeSet};

use opence_census::engine::{assigned, uses, walked};
use opence_census::fields::{engine, keep, marks, read_fields, write_fields, Engine};
use opence_census::layout::read_dump;

type Failure = Box<dyn std::error::Error>;

const IMPORTER: &str = r#"
fn thing(tag: &Tag) -> Result<Thing, ImportError> {
    let mut flags = tag.u16(0, "flags")?;
    // a comment's "quote" is not a label: x: f(y, "no")
    let parts = tag.block(4, "parts", 8, 16)?;
    Ok(Thing {
        flags,
        mass: parts.get(0).map(|p| p.f32(0, "mass")).transpose()?,
        origin: point(tag, 12, "origin")?,
    })
}
fn other(tag: &Tag, out: &mut Other) {
    out.inner.scale = tag.f32(8, "scale").unwrap_or(1.0);
    *out = fail(tag.fault("odd", format!("is {}", 1)));
}
"#;

#[test]
fn each_label_is_the_field_its_read_fills() {
    let pairs = assigned(IMPORTER);
    let found: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(l, i)| (l.as_str(), i.as_str()))
        .collect();
    assert_eq!(
        found,
        [
            ("flags", "flags"),
            ("parts", "parts"),
            ("mass", "mass"),
            ("origin", "origin"),
            ("scale", "scale"),
        ]
    );
}

#[test]
fn a_label_read_as_a_block_is_walked() {
    let text = r#"
fn weather(tag: &Tag) -> Result<Weather, ImportError> {
    let block = tag.block(0x24, "particle types", 604, 8)?;
    for kind in tag
        .block(0x30, "kinds", 4, 8)?
        .iter() {}
    let size = tag.f32(block.len(), "size")?;
}
"#;
    assert_eq!(
        walked(text),
        ["kinds".to_owned(), "particle types".to_owned()].into()
    );
    assert_eq!(walked(IMPORTER), ["parts".to_owned()].into());
}

/// Reads into an array: its literal's elements, and the elements a loop
/// over it fills, one loop or two deep; a read pushed onto a list; and a
/// read bound by a `let Some(..)`.
const ARRAYS: &str = r#"
fn device(tag: &Tag) -> Result<Device, ImportError> {
    let mut modes = [0; 4];
    for (index, mode) in modes.iter_mut().enumerate() {
        *mode = tag.i16(index * 2, "function modes")?;
    }
    let mut boxes = [[0.0; 2]; 3];
    for (axis, bounds) in boxes.iter_mut().enumerate() {
        for (end, bound) in bounds.iter_mut().enumerate() {
            *bound = tag.f32(axis * 2 + end, "pathfinding box")?;
        }
    }
    let mut out = Device::default();
    out.berserk = [tag.f32(0, "berserk amount")?, tag.f32(4, "berserk threshold")?];
    out.sounds
        .push(reference(tag, 8, "hit sound")?);
    let Some(map) = tag.bitmap(12, "secondary map")? else {
        return Ok(out);
    };
    Ok(Device {
        effects: [
            effect(0x24, "start effect")?,
            effect(0x34, "stop effect")?,
        ],
        modes,
        boxes,
        ..out
    })
}
"#;

#[test]
fn a_read_into_an_array_is_the_arrays_field() {
    let pairs = assigned(ARRAYS);
    let found: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(l, i)| (l.as_str(), i.as_str()))
        .collect();
    assert_eq!(
        found,
        [
            ("function modes", "modes"),
            ("pathfinding box", "boxes"),
            ("berserk amount", "berserk"),
            ("berserk threshold", "berserk"),
            ("hit sound", "sounds"),
            ("secondary map", "map"),
            ("start effect", "effects"),
            ("stop effect", "effects"),
        ]
    );
}

#[test]
fn a_read_bound_to_a_local_fills_the_field_the_local_is_stored_in() {
    let text = r#"
fn range(range: &Element<'_>) -> Result<Range, ImportError> {
    let actual = range.i16(0x2C, "actual permutation count")?;
    let pitch = range.f32(0x20, "natural pitch")?;
    let rate = if pitch > 0.0 { 1.0 / pitch } else { 1.0 };
    Ok(Range {
        actual_permutation_count: u16::try_from(clamp(1, actual)).unwrap_or(0),
        natural_pitch: pitch,
        rate,
    })
}
"#;
    let pairs = assigned(text);
    let found: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(l, i)| (l.as_str(), i.as_str()))
        .collect();
    // (the local used only to work out another value fills nothing more)
    assert_eq!(
        found,
        [
            ("actual permutation count", "actual"),
            ("actual permutation count", "actual_permutation_count"),
            ("natural pitch", "pitch"),
            ("natural pitch", "natural_pitch"),
        ]
    );
}

#[test]
fn a_use_is_a_field_read_through_a_dot_not_a_method_or_a_comment() {
    let text = r#"
struct Drawn {
    #[cfg(test)]
    seen: u32,
    shown: u32,
}
#[cfg(test)]
fn helper(thing: &Thing) -> f32 { thing.scale }
fn draw(thing: &Thing) -> f32 {
    // thing.unused
    let label = "thing.quoted";
    thing.mass * thing.origin.len() + thing.items.iter().count() as f32 + t.0
}
#[cfg(test)]
mod tests {
    fn t(thing: &Thing) { thing.flags; }
}
fn after(kind: &Kind) -> f32 { kind.fade[3] }
"#;
    // (a test-only item is left out, and what follows it is not)
    assert_eq!(
        uses(text),
        [
            "mass".to_owned(),
            "origin".to_owned(),
            "items".to_owned(),
            "fade".to_owned()
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );
}

const DUMP: &str = "
*** Dumping AST Record Layout
         0 | struct thing_definition
         0 |   short flags
         4 |   float mass
         8 |   float scale
        12 |   float unread
        16 |   struct tag_block parts
        16 |     long count
        20 |     void * address
           | [sizeof=24, align=4]
";

#[test]
fn a_kept_field_is_used_walked_unused_or_untraced() -> Result<(), Failure> {
    let records = read_dump(DUMP);
    let mut marked = marks(&records, &BTreeSet::new());
    let field = |path: &str| ("thing_definition".to_owned(), path.to_owned());
    let kept: BTreeSet<(String, String)> = [
        field("flags"),
        field("mass"),
        field("scale"),
        field("parts"),
    ]
    .into();
    keep(&mut marked, &records, &kept);
    let labels: BTreeMap<(String, String), BTreeSet<String>> = [
        (field("flags"), ["flags".to_owned()].into()),
        (field("mass"), ["mass".to_owned()].into()),
        (field("scale"), ["passed on".to_owned()].into()),
        (field("parts"), ["parts".to_owned()].into()),
    ]
    .into();
    let assigned: BTreeMap<String, BTreeSet<String>> = [
        ("flags".to_owned(), ["flags".to_owned()].into()),
        ("mass".to_owned(), ["mass".to_owned()].into()),
    ]
    .into();
    let walked: BTreeSet<String> = ["parts".to_owned()].into();
    let used: BTreeSet<String> = ["mass".to_owned()].into();
    engine(&mut marked, &records, &labels, &assigned, &walked, &used);
    let found: Vec<(&str, Engine)> = marked.iter().map(|m| (m.path.as_str(), m.engine)).collect();
    assert_eq!(
        found,
        [
            ("flags", Engine::Unused),
            ("mass", Engine::Used),
            ("scale", Engine::Untraced),
            ("unread", Engine::Unkept),
            ("parts.count", Engine::Walked),
            ("parts.address", Engine::Walked),
        ]
    );
    assert_eq!(read_fields(&write_fields(&marked))?, marked);
    let text = write_fields(&marked);
    assert!(text.contains("\tkept\tused\n"), "{text}");
    assert!(read_fields("t\tf\t0\tshort\tread\tkept\tmaybe\n").is_err());
    Ok(())
}
