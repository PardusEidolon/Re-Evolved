//! The plugins' readers of the game's own structures (`opence_runtime::views`):
//! its data arrays (memory/data.h), its tag table (cache/cache_files.c) and
//! its tag blocks and references (tag_files/tag_groups.h), read from memory
//! a plugin is handed as untrusted, so that each answers `None` for what
//! the game itself would not accept, never a read outside, and never a
//! panic. They are fuzzed below for a fixed time on the stable compiler,
//! and by `cargo fuzz run views` (engine/fuzz) for longer.

use std::time::{Duration, Instant};

use opence_runtime::plugins::View;
use opence_runtime::views::{self, Block, DataArray, Reference, Tag};

/// `'d@t@'`
const SIGNATURE: u32 = 0x6440_7440;

fn put16(memory: &mut [u8], at: usize, value: u16) {
    memory[at..at + 2].copy_from_slice(&value.to_le_bytes());
}

fn put32(memory: &mut [u8], at: usize, value: u32) {
    memory[at..at + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_name(memory: &mut [u8], at: usize, name: &str) {
    memory[at..at + name.len()].copy_from_slice(name.as_bytes());
}

/// "objects": a data array at 0x100 of 4 datums of 8 bytes at 0x200, the
/// first 3 used, the second of them free
fn objects(memory: &mut [u8]) {
    put_name(memory, 0x100, "objects");
    put16(memory, 0x100 + 32, 4);
    put16(memory, 0x100 + 34, 8);
    memory[0x100 + 36] = 1;
    memory[0x100 + 37] = 1;
    put32(memory, 0x100 + 40, SIGNATURE);
    put16(memory, 0x100 + 46, 3);
    put32(memory, 0x100 + 52, 0x200);
    put16(memory, 0x200, 0xE174);
    put16(memory, 0x210, 0xE176);
}

fn with_memory<R>(build: impl FnOnce(&mut [u8]), read: impl FnOnce(&View<'_>) -> R) -> R {
    let mut memory = vec![0; 0x1000];
    build(&mut memory);
    View::over(&mut memory, read)
}

#[test]
fn a_data_array_is_read_as_the_game_reads_it() {
    with_memory(objects, |view| {
        assert_eq!(
            view.data_array(0x100),
            Some(DataArray {
                name: "objects".into(),
                maximum_count: 4,
                size: 8,
                identifier_zero_invalid: true,
                count: 3,
                data: 0x200,
            })
        );
    });
}

#[test]
fn its_datums_are_the_used_ones_below_its_count() {
    with_memory(objects, |view| {
        let objects = view.data_array(0x100);
        let datums = objects.map(|objects| objects.datums(view));
        assert_eq!(
            datums,
            Some(vec![(0xE174_0000, 0x200), (0xE176_0002, 0x210)])
        );
    });
}

#[test]
fn a_datum_is_found_by_its_index_as_datum_try_and_get_finds_it() {
    with_memory(objects, |view| {
        let Some(objects) = view.data_array(0x100) else {
            panic!("no data array");
        };
        let found: Vec<Option<u32>> = [
            0xE174_0000, // in use
            0xE176_0002, // in use
            0x0000_0002, // its identifier unchecked: refused, as zero is invalid here
            0xE175_0002, // the wrong identifier
            0xE175_0001, // free
            0xE177_0004, // past the maximum
            0xE177_8000, // a negative absolute index
            u32::MAX,    // NONE
        ]
        .into_iter()
        .map(|index| objects.datum(view, index))
        .collect();
        assert_eq!(
            found,
            [Some(0x200), Some(0x210), None, None, None, None, None, None]
        );
    });
}

#[test]
fn a_data_array_the_game_would_not_accept_is_none() {
    let broken: [fn(&mut [u8]); 6] = [
        |memory| put32(memory, 0x100 + 40, 0),      // no signature
        |memory| memory[0x100 + 36] = 0,            // not valid
        |memory| put16(memory, 0x100 + 46, 5),      // count past the maximum
        |memory| put16(memory, 0x100 + 32, 0x8000), // a negative maximum
        |memory| put16(memory, 0x100 + 34, 1),      // a datum smaller than its header
        |memory| put32(memory, 0x100 + 52, 0x0FF0), // datums past the end
    ];
    for (case, break_it) in broken.into_iter().enumerate() {
        let read = with_memory(
            |memory| {
                objects(memory);
                break_it(memory);
            },
            |view| view.data_array(0x100),
        );
        assert_eq!(read, None, "case {case}");
    }
    with_memory(objects, |view| {
        assert_eq!(view.data_array(0x0FE0), None);
        assert_eq!(view.data_array(u32::MAX), None);
    });
}

/// a tag table of 2 at 0x400: "levels\\a10" (a scenario, 'scnr') and an
/// unnamed bitmap, its data at 0x800
fn tags(memory: &mut [u8]) {
    put32(memory, 0x400, 0x7363_6E72);
    put32(memory, 0x400 + 4, u32::MAX);
    put32(memory, 0x400 + 8, u32::MAX);
    put32(memory, 0x400 + 12, 0xE741_0000);
    put32(memory, 0x400 + 16, 0x600);
    put32(memory, 0x400 + 20, 0x800);
    put32(memory, 0x420, 0x6269_746D);
    put32(memory, 0x420 + 12, 0xE742_0001);
    put_name(memory, 0x600, "levels\\a10");
}

#[test]
fn a_tag_is_found_as_cache_get_tag_instance_finds_it() {
    with_memory(tags, |view| {
        assert_eq!(
            view.tag(0x400, 2, 0xE741_0000),
            Some(Tag {
                group: 0x7363_6E72,
                parents: [u32::MAX, u32::MAX],
                index: 0xE741_0000,
                name: "levels\\a10".into(),
                data: 0x800,
            })
        );
        // the absolute index alone finds it too
        assert_eq!(
            view.tag(0x400, 2, 1).map(|tag| tag.index),
            Some(0xE742_0001)
        );
        // an unnamed tag's name is empty
        assert_eq!(
            view.tag(0x400, 2, 1).map(|tag| tag.name),
            Some(String::new())
        );
    });
}

#[test]
fn a_tag_index_that_is_not_one_is_none() {
    with_memory(tags, |view| {
        for index in [u32::MAX, 0xE741_0001, 2, 0x0000_8000] {
            assert_eq!(view.tag(0x400, 2, index), None, "{index:08x}");
        }
        // a table that runs past the end
        assert_eq!(view.tag(0x0FF0, 2, 1), None);
    });
}

#[test]
fn a_tag_block_and_its_elements_are_read_in_bounds() {
    with_memory(
        |memory| {
            put32(memory, 0x300, 3);
            put32(memory, 0x304, 0x900);
            put32(memory, 0x310, 0x7FFF_FFFF);
            put32(memory, 0x314, 0x900);
            put32(memory, 0x320, u32::MAX);
        },
        |view| {
            let block = view.block(0x300, 12);
            assert_eq!(
                block,
                Some(Block {
                    count: 3,
                    address: 0x900,
                    element_size: 12
                })
            );
            let elements: Vec<Option<u32>> = (0..4)
                .map(|index| block.as_ref().and_then(|block| block.element(index)))
                .collect();
            assert_eq!(elements, [Some(0x900), Some(0x90C), Some(0x918), None]);
            // more elements than the memory holds, and a negative count
            assert_eq!(view.block(0x310, 12), None);
            assert_eq!(view.block(0x320, 12), None);
            // an empty block is read wherever it points
            assert_eq!(view.block(0x330, 12).map(|block| block.count), Some(0));
        },
    );
}

#[test]
fn a_tag_reference_is_its_group_and_index() {
    with_memory(
        |memory| {
            put32(memory, 0x340, 0x6269_746D);
            put32(memory, 0x34C, 0xE742_0001);
        },
        |view| {
            assert_eq!(
                view.reference(0x340),
                Some(Reference {
                    group: 0x6269_746D,
                    index: 0xE742_0001
                })
            );
            assert_eq!(view.reference(0x0FF8), None);
        },
    );
}

#[test]
fn a_string_is_cut_at_its_end_its_limit_or_the_memory() {
    with_memory(
        |memory| {
            put_name(memory, 0x500, "warthog");
            put_name(memory, 0x0FFC, "jeep");
        },
        |view| {
            assert_eq!(view.c_string(0x500, 64), Some("warthog".into()));
            assert_eq!(view.c_string(0x500, 4), Some("wart".into()));
            assert_eq!(view.c_string(0x0FFC, 64), Some("jeep".into()));
            assert_eq!(view.c_string(0x1000, 64), None);
        },
    );
}

/// xorshift64*: a fixed seed, so that a failure is found again
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// Memory with the structures above in it, changed in a few places, walked
/// by the fuzz target's entry, for two seconds: every reader answers, and
/// none panics or reads outside.
#[test]
fn the_readers_fuzzed_answer_and_never_panic() {
    let mut random = Random(0x0BAD_5EED);
    let mut base = vec![0; 0x1000];
    objects(&mut base);
    tags(&mut base);
    let interesting = [
        0,
        1,
        0x7FFF,
        0x8000,
        0xFFFF,
        0x7FFF_FFFF,
        u32::MAX,
        0x200,
        0x400,
        0x0FFC,
    ];
    let started = Instant::now();
    let mut cases = 0;
    let mut found = 0;
    while cases < 256 || started.elapsed() < Duration::from_secs(2) {
        let mut memory = base.clone();
        for _ in 0..1 + random.next() % 8 {
            let at = (random.next() % 0x1000) as usize & !3;
            match random.next() % 3 {
                0 => memory[at] ^= 1 << (random.next() % 8),
                1 => put32(&mut memory, at, interesting[(random.next() % 10) as usize]),
                _ => memory[at] = random.next() as u8,
            }
        }
        let cut = 0x800 + (random.next() % 0x800) as usize;
        memory.truncate(cut);
        found += views::walk(&memory);
        cases += 1;
    }
    println!("{cases} cases, {found} structures found");
    assert!(found > 0);
}

#[test]
fn a_data_array_is_found_by_its_name_in_a_range() {
    with_memory(objects, |view| {
        let found = view.find_data_array(0, 0x1000, "objects");
        assert_eq!(
            found.map(|(address, array)| (address, array.count)),
            Some((0x100, 3))
        );
        // a name the array's only begins with, or a range that misses it
        assert_eq!(view.find_data_array(0, 0x1000, "object"), None);
        assert_eq!(view.find_data_array(0x104, 0xF00, "objects"), None);
        assert_eq!(view.find_data_array(0, 0x100, "objects"), None);
        // a range past the memory's end is read only as far as the end
        let past = view.find_data_array(0x80, u32::MAX, "objects");
        assert_eq!(past.map(|(address, _)| address), Some(0x100));
        assert_eq!(view.find_data_array(0x2000, 0x1000, "objects"), None);
    });
}
