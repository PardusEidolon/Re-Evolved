//! The tool runs bitmap_group.c's `postprocess_bitmap_group` as it builds a
//! map, and the map stores what it leaves: an interface bitmaps group's
//! images are linear, a sprite group's sequences have no first bitmap
//! index or bitmap count, and no group ends in a sequence with neither
//! bitmaps nor sprites. The importer reads the groups as stored, so it
//! sees what the game sees (skipped where there is no game data).

use std::path::Path;

use opence_import::{Cache, GroupTag};

type Failure = Box<dyn std::error::Error>;

const MAPS: [&str; 11] = [
    "a10",
    "a30",
    "a50",
    "b30",
    "b40",
    "c10",
    "c20",
    "c40",
    "d20",
    "d40",
    "bloodgulch",
];

/// The bitmap group's size, its type, its sequences and its bitmaps.
const GROUP_SIZE: usize = 108;
const SEQUENCES: (usize, usize) = (84, 64);
const BITMAPS: (usize, usize) = (96, 48);
/// `_bitmap_group_type_sprites` and `_bitmap_group_type_interface_bitmaps`.
const SPRITES: i16 = 3;
const INTERFACE_BITMAPS: i16 = 4;
/// A bitmap's flags, and `_bitmap_linear_bit`.
const BITMAP_FLAGS: usize = 14;
const LINEAR: u16 = 1 << 4;

#[test]
fn the_stock_maps_store_the_bitmap_groups_as_the_tool_left_them() -> Result<(), Failure> {
    let group = GroupTag::new(b"bitm");
    let (mut interface, mut sequences) = (0, 0);
    for map in MAPS {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/maps")
            .join(format!("{map}.map"));
        let Ok(bytes) = std::fs::read(&path) else {
            continue; // (no game data here)
        };
        let cache = Cache::open(bytes)?;
        for (index, tag) in cache.tags().iter().enumerate() {
            if !tag.is(group) || tag.in_resource_map {
                continue;
            }
            let bitmap = cache.tag(index, group, GROUP_SIZE)?;
            let kind = bitmap.i16(0, "type")?;
            if kind == INTERFACE_BITMAPS {
                for image in bitmap.block(BITMAPS.0, "bitmaps", BITMAPS.1, 2048)?.iter() {
                    let flags = image.u16(BITMAP_FLAGS, "flags")?;
                    assert_ne!(flags & LINEAR, 0, "{map} {}", tag.name);
                    interface += 1;
                }
            }
            let block = bitmap.block(SEQUENCES.0, "sequences", SEQUENCES.1, 256)?;
            let last = block.len().checked_sub(1);
            for (at, sequence) in block.iter().enumerate() {
                let first = sequence.i16(32, "first bitmap index")?;
                let count = sequence.i16(34, "bitmap count")?;
                let sprites = sequence.u32(52, "sprites")?;
                if kind == SPRITES {
                    assert_eq!((first, count), (0, 0), "{map} {} {at}", tag.name);
                    sequences += 1;
                }
                if Some(at) == last {
                    assert!(count != 0 || sprites != 0, "{map} {} ends empty", tag.name);
                }
            }
        }
    }
    println!("{interface} interface images and {sequences} sprite sequences as the tool left them");
    Ok(())
}
