//! The tag fields the importer keeps: its recorded reads laid over the
//! original's tag structures.

use std::collections::BTreeSet;

use opence_census::fields::{keep, marks};
use opence_census::importer::{
    casts, elements, groups, kept, read_reads, resolve, roots, write_reads, Hop, Read,
};
use opence_census::layout::{read_dump, read_layout, LAYOUT};

mod common;

type Failure = Box<dyn std::error::Error>;

const DUMP: &str = "
*** Dumping AST Record Layout
         0 | struct thing_part
         0 |   float mass
         4 |   short flags
           | [sizeof=8, align=4]

*** Dumping AST Record Layout
         0 | struct thing_definition
         0 |   short flags
         4 |   struct tag_block parts
         4 |     long count
         8 |     void * address
        12 |   char[16] name
        28 |   float scale
           | [sizeof=32, align=4]

*** Dumping AST Record Layout
         0 | struct other_definition
         0 |   struct tag_block parts
         0 |     long count
         4 |     void * address
         8 |   float scale
           | [sizeof=12, align=4]
";

const HEADER: &str = "
enum
{
\tTHING_DEFINITION_TAG= 'thng',
\tOTHER_TAG = 'othr'
};
";

const C: &str = "
static void thing_update(long index)
{
\tstruct thing_definition const *definition= tag_get(THING_DEFINITION_TAG, index);
\tstruct thing_part *part= TAG_BLOCK_GET_ELEMENT(&definition->parts, 0, struct thing_part);
\treal_plane3d *plane= TAG_BLOCK_GET_ELEMENT(&global_get(x->y)->planes, part->flags, real_plane3d);
\tlong *bare= TAG_BLOCK_GET_ELEMENT(block, 0, long);
\tstruct thing_part *cast= (struct thing_part *)tag_get(THING_PART_TAG, 0);
}
";

const THNG: u32 = u32::from_be_bytes(*b"thng");
const OTHR: u32 = u32::from_be_bytes(*b"othr");

fn read(group: u32, hops: &[(u32, u32)], at: u32, width: u32) -> Read {
    Read {
        group,
        hops: hops
            .iter()
            .map(|&(at, element_size)| Hop { at, element_size })
            .collect(),
        at,
        width,
        label: format!("at {at}"),
    }
}

#[test]
fn the_reads_round_trip_through_their_tsv() -> Result<(), Failure> {
    let reads: BTreeSet<Read> =
        [read(THNG, &[], 0, 2), read(THNG, &[(4, 8), (0, 12)], 8, 4)].into();
    let text = write_reads(&reads);
    assert!(text.contains("74686e67\t4:8/0:12\t8\t4\tat 8"), "{text}");
    assert_eq!(read_reads(&text)?, reads);
    assert!(read_reads("7468\t-\t0\t4\tx\n").is_err());
    assert!(read_reads("7468ZZ67\t-\t0\t4\tx\n").is_err());
    assert!(read_reads("74686e67\t4:\t0\t4\tx\n").is_err());
    assert!(read_reads("74686e67\t-\t0\t4\n").is_err());
    Ok(())
}

#[test]
fn a_headers_groups_and_the_cs_casts_and_elements_are_read() {
    assert_eq!(
        groups(HEADER),
        [
            ("THING_DEFINITION_TAG".to_owned(), THNG),
            ("OTHER_TAG".to_owned(), OTHR)
        ]
    );
    assert_eq!(
        casts(C),
        [
            (
                "THING_DEFINITION_TAG".to_owned(),
                "thing_definition".to_owned()
            ),
            ("THING_PART_TAG".to_owned(), "thing_part".to_owned())
        ]
    );
    assert_eq!(
        elements(C),
        [
            ("parts".to_owned(), "thing_part".to_owned()),
            ("planes".to_owned(), "real_plane3d".to_owned())
        ]
    );
}

#[test]
fn defined_groups_and_casts_in_macros_are_read() {
    let header = "#define SHADER_DEFINITION_TAG 'shdr'\n#define NOT_A_TAG_X 'abcd'\n";
    assert_eq!(
        groups(header),
        [(
            "SHADER_DEFINITION_TAG".to_owned(),
            u32::from_be_bytes(*b"shdr")
        )]
    );
    let c = "#define thing_get(i) ((struct thing_definition *)tag_get('thng', i))\n";
    assert_eq!(
        casts(c),
        [(
            "GROUP_74686e67_TAG".to_owned(),
            "thing_definition".to_owned()
        )]
    );
    let records = read_dump(DUMP);
    let found = roots(&[], &casts(c), &records);
    assert_eq!(
        found.get(&THNG).map(String::as_str),
        Some("thing_definition")
    );
}

#[test]
fn a_group_is_the_structure_it_is_cast_to_or_the_one_named_for_it() {
    let records = read_dump(DUMP);
    let found = roots(&groups(HEADER), &casts(C), &records);
    let found: Vec<(u32, &str)> = found.iter().map(|(g, s)| (*g, s.as_str())).collect();
    assert_eq!(
        found,
        [(OTHR, "other_definition"), (THNG, "thing_definition")]
    );
}

#[test]
fn each_read_is_the_fields_it_overlaps_in_the_element_its_blocks_reach() {
    let records = read_dump(DUMP);
    let roots = roots(&groups(HEADER), &casts(C), &records);
    let elements = elements(C);
    let reads: BTreeSet<Read> = [
        read(THNG, &[], 0, 2),
        read(THNG, &[], 4, 4),
        read(THNG, &[], 13, 1),
        read(THNG, &[(4, 8)], 0, 4),
        // Past the structure, a block of no laid-out element, and a group
        // with no structure.
        read(THNG, &[], 40, 4),
        read(THNG, &[(12, 8)], 0, 4),
        read(u32::from_be_bytes(*b"none"), &[], 0, 4),
        // The other's parts share the thing's block name, at a stride that
        // is not its size: they are not the thing's parts.
        read(OTHR, &[(0, 20)], 4, 2),
    ]
    .into();
    let kept = resolve(&reads, &roots, &elements, &records);
    let fields: Vec<(&str, &str)> = kept
        .fields
        .iter()
        .map(|(s, p)| (s.as_str(), p.as_str()))
        .collect();
    assert_eq!(
        fields,
        [
            ("thing_definition", "flags"),
            ("thing_definition", "name"),
            ("thing_definition", "parts.count"),
            ("thing_part", "mass"),
        ]
    );
    assert_eq!(kept.unresolved, 4);
    assert!(kept.resized.is_empty(), "{:?}", kept.resized);
}

#[test]
fn the_kept_fields_are_marked_beside_the_originals_reads() {
    let records = read_dump(DUMP);
    let mut marked = marks(&records, &BTreeSet::new());
    let kept: BTreeSet<(String, String)> = [
        ("thing_definition".to_owned(), "scale".to_owned()),
        ("thing_part".to_owned(), "flags".to_owned()),
    ]
    .into();
    keep(&mut marked, &records, &kept);
    let found: Vec<(&str, &str)> = marked
        .iter()
        .filter(|m| m.importer)
        .map(|m| (m.structure.as_str(), m.path.as_str()))
        .collect();
    assert_eq!(
        found,
        [("thing_part", "flags"), ("thing_definition", "scale")]
    );
}

#[test]
fn a_block_the_c_never_names_is_the_structure_its_name_gives() {
    let dump = format!(
        "{DUMP}
*** Dumping AST Record Layout
         0 | struct thing_widget_datum
         0 |   long kind
         4 |   float size
           | [sizeof=8, align=4]

*** Dumping AST Record Layout
         0 | struct holder
         0 |   struct tag_block widgets
         0 |     long count
         4 |     void * address
           | [sizeof=8, align=4]
"
    );
    let records = read_dump(&dump);
    let roots = [(THNG, "holder".to_owned())].into();
    // Two structures are 8 bytes; only one is named for the widgets.
    let reads: BTreeSet<Read> = [read(THNG, &[(0, 8)], 4, 4)].into();
    let kept = resolve(&reads, &roots, &[], &records);
    assert_eq!(
        kept.fields.into_iter().collect::<Vec<_>>(),
        [("thing_widget_datum".to_owned(), "size".to_owned())]
    );
    assert_eq!(kept.unresolved, 0);
}

#[test]
fn a_block_is_resized_only_against_a_structure_its_owner_names() {
    let dump = format!(
        "{DUMP}
*** Dumping AST Record Layout
         0 | struct other_holder
         0 |   struct tag_block parts
         0 |     long count
         4 |     void * address
           | [sizeof=8, align=4]
"
    );
    let records = read_dump(&dump);
    let elements = [("parts".to_owned(), "thing_part".to_owned())];
    // Another group's `parts`, stepped at 16, is not the thing's parts.
    let other: BTreeSet<Read> = [read(OTHR, &[(0, 16)], 0, 4)].into();
    let roots = [(OTHR, "other_holder".to_owned())].into();
    let kept = resolve(&other, &roots, &elements, &records);
    assert!(kept.resized.is_empty(), "{:?}", kept.resized);
    assert_eq!(kept.unresolved, 1);
    // The thing's own parts, stepped at 12, are resized.
    let own: BTreeSet<Read> = [read(THNG, &[(4, 12)], 0, 4)].into();
    let roots = [(THNG, "thing_definition".to_owned())].into();
    let kept = resolve(&own, &roots, &elements, &records);
    assert_eq!(
        kept.resized.into_iter().collect::<Vec<_>>(),
        [("thing_part".to_owned(), 12, 8)]
    );
}

#[test]
fn the_importer_steps_every_block_at_the_originals_element_size() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        return Ok(()); // (no source/ here)
    };
    let layout = opence_census::read_text(&root.join("engine").join(LAYOUT))?;
    let records = read_layout(&layout)?;
    let kept = kept(&root, &records)?;
    let resized: Vec<String> = kept
        .resized
        .iter()
        .map(|(name, stride, size)| format!("{name}: stepped at {stride:#x}, sized {size:#x}"))
        .collect();
    assert!(resized.is_empty(), "{resized:#?}");
    Ok(())
}
