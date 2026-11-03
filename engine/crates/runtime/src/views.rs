//! The plugins' readers of the game's own structures, on a [`View`] of its
//! memory: its data arrays (`struct data_array`, memory/data.h), its tag
//! table (`struct cache_file_tag_instance`, cache/cache_files.c) and the
//! tag blocks and references in tags (tag_files/tag_groups.h), laid out as
//! game.wasm lays them out, with 32-bit pointers.
//!
//! Whatever a plugin reads is untrusted, as the importer treats a map: each
//! reader checks what the game's own code asserts or checks
//! (`data_verify`, `datum_try_and_get`, `cache_get_tag_instance`) and
//! answers `None` where the game would not accept it, and every range it
//! reads is checked against the memory's size; the widget tree's reader
//! is in [`crate::widgets`]. [`walk`] is their fuzz
//! target's entry (engine/fuzz, and tests/views.rs on the stable compiler).

use crate::plugins::View;

/// `'d@t@'`, the signature `data_verify` checks
const DATA_SIGNATURE: u32 = 0x6440_7440;
/// a data array's name, `char name[TAG_STRING_LENGTH+1]`
const DATA_NAME_LENGTH: u32 = 32;
/// `sizeof(struct cache_file_tag_instance)`
const TAG_INSTANCE_SIZE: u32 = 0x20;
/// the longest tag path read
const TAG_NAME_LENGTH: u32 = 256;
/// `NONE`, as an index
const NONE: u32 = u32::MAX;
/// how much [`View::find_data_array`] reads at a time
const FIND_CHUNK: u32 = 1 << 20;

/// Where game.wasm's game state starts, `HALO_PORT_GAME_STATE_BASE_ADDRESS`
/// (port/linux/include/halo_port_capacity.h): its data arrays (objects,
/// particles, effects and the rest) are in it.
pub const GAME_STATE_ADDRESS: u32 = 0x81A0_0000;
/// The game state's size, `HALO_PORT_GAME_STATE_SIZE`.
pub const GAME_STATE_SIZE: u32 = 0x13C_0000 + 0x4_0000;

/// A data array's header, as `data_verify` accepts it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataArray {
    /// its name ("objects", "actors" and the rest)
    pub name: String,
    /// how many datums it has room for
    pub maximum_count: u16,
    /// each datum's size in bytes, its identifier first
    pub size: u16,
    /// whether an index with no identifier is refused
    pub identifier_zero_invalid: bool,
    /// the datums below the last one used
    pub count: u16,
    /// the address of its first datum
    pub data: u32,
}

impl DataArray {
    /// The address of the datum `index` (its identifier in the high half,
    /// its absolute index in the low), if it is in use and its identifier
    /// matches, as `datum_try_and_get` finds it.
    #[must_use]
    pub fn datum(&self, view: &View<'_>, index: u32) -> Option<u32> {
        if index == NONE {
            return None;
        }
        let identifier = u16::try_from(index >> 16).ok()?;
        let absolute = u16::try_from(index & 0xFFFF).ok()?;
        // the game asserts both: a negative absolute index is below 0, and
        // an index with no identifier is refused where zero is invalid
        if absolute >= self.maximum_count || (identifier == 0 && self.identifier_zero_invalid) {
            return None;
        }
        let address = self.at(absolute)?;
        let found = view.u16(address)?;
        (found != 0 && (identifier == 0 || found == identifier)).then_some(address)
    }

    /// Each datum in use below its count, as `data_iterator_next` visits
    /// them: its index and its address.
    #[must_use]
    pub fn datums(&self, view: &View<'_>) -> Vec<(u32, u32)> {
        (0..self.count)
            .filter_map(|absolute| {
                let address = self.at(absolute)?;
                let identifier = view.u16(address)?;
                (identifier != 0)
                    .then_some((u32::from(identifier) << 16 | u32::from(absolute), address))
            })
            .collect()
    }

    /// the address of the datum at `absolute`
    fn at(&self, absolute: u16) -> Option<u32> {
        let offset = u32::from(self.size).checked_mul(u32::from(absolute))?;
        self.data.checked_add(offset)
    }
}

/// A tag in the game's tag table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    /// its group, as four characters ('bitm', 'scnr' and the rest)
    pub group: u32,
    /// the groups it inherits from, or `NONE`
    pub parents: [u32; 2],
    /// its index, identifier and absolute index
    pub index: u32,
    /// its path, empty if it has none
    pub name: String,
    /// the address of its data
    pub data: u32,
}

/// A tag block: its elements' count and where they are.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// how many elements it has
    pub count: u32,
    /// the address of the first
    pub address: u32,
    /// each element's size, as the reader was told
    pub element_size: u32,
}

impl Block {
    /// The address of element `index`, if the block has it.
    #[must_use]
    pub fn element(&self, index: u32) -> Option<u32> {
        if index >= self.count {
            return None;
        }
        self.address
            .checked_add(index.checked_mul(self.element_size)?)
    }
}

/// A tag reference: the group and index of the tag it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// the tag's group
    pub group: u32,
    /// the tag's index, or `NONE`
    pub index: u32,
}

impl View<'_> {
    /// The string at `address`, up to its terminator, `max` bytes or the end
    /// of the memory, whichever comes first; `None` if `address` is outside.
    #[must_use]
    pub fn c_string(&self, address: u32, max: u32) -> Option<String> {
        // a string needs its first byte inside, even an empty one
        self.bytes(address, 1)?;
        let bytes = self.bytes_up_to(address, max)?;
        let text = bytes.split(|byte| *byte == 0).next().unwrap_or_default();
        Some(String::from_utf8_lossy(text).into_owned())
    }

    /// The data array whose header is at `address`, if `data_verify` would
    /// accept it and its datums are all inside the memory.
    #[must_use]
    pub fn data_array(&self, address: u32) -> Option<DataArray> {
        let field = |offset: u32| address.checked_add(offset);
        let short = |offset: u32| {
            let value = self.u16(field(offset)?)?;
            // `short`: a negative one is refused
            (value < 0x8000).then_some(value)
        };
        let byte = |offset: u32| {
            self.bytes(field(offset)?, 1)
                .and_then(|bytes| bytes.first().copied())
        };
        if self.u32(field(40)?)? != DATA_SIGNATURE || byte(36)? == 0 {
            return None;
        }
        let maximum_count = short(32)?;
        let size = short(34)?;
        let count = short(46)?;
        let data = self.u32(field(52)?)?;
        let end = u64::from(data) + u64::from(size) * u64::from(maximum_count);
        let inside = u64::try_from(self.len()).is_ok_and(|len| end <= len);
        if data == 0 || size < 2 || count > maximum_count || !inside {
            return None;
        }
        Some(DataArray {
            name: self.c_string(address, DATA_NAME_LENGTH)?,
            maximum_count,
            size,
            identifier_zero_invalid: byte(37)? != 0,
            count,
            data,
        })
    }

    /// The first data array named `name` whose header starts in the
    /// `length` bytes from `start` (read only as far as the memory's end),
    /// with its address: the game keeps no list of its arrays, so a plugin
    /// finds one by its header's name, as `data_new` wrote it. Its arrays of
    /// the game state are in [`GAME_STATE_ADDRESS`]'s [`GAME_STATE_SIZE`].
    #[must_use]
    pub fn find_data_array(&self, start: u32, length: u32, name: &str) -> Option<(u32, DataArray)> {
        let mut needle = name.as_bytes().to_vec();
        needle.push(0);
        let needle_length = u32::try_from(needle.len()).ok()?;
        if needle_length > DATA_NAME_LENGTH {
            return None;
        }
        let end = u64::from(start)
            .saturating_add(u64::from(length))
            .min(u64::try_from(self.len()).ok()?);
        let mut at = u64::from(start);
        while at < end {
            // (each chunk overlaps the next by the needle, less a byte)
            let chunk_end = at.saturating_add(u64::from(FIND_CHUNK)).min(end);
            let read = chunk_end + u64::from(needle_length - 1);
            let read = read.min(u64::try_from(self.len()).ok()?);
            let address = u32::try_from(at).ok()?;
            let bytes = self.bytes(address, u32::try_from(read - at).ok()?)?;
            let starts = u32::try_from(chunk_end - at).ok()?;
            let found = bytes
                .windows(needle.len())
                .take(usize::try_from(starts).ok()?)
                .enumerate()
                .filter(|(_, window)| *window == needle.as_slice())
                .find_map(|(offset, _)| {
                    let header = address.checked_add(u32::try_from(offset).ok()?)?;
                    Some((header, self.data_array(header)?))
                });
            if found.is_some() {
                return found;
            }
            at = chunk_end;
        }
        None
    }

    /// The tag `index` in the table of `count` tags at `instances`, as
    /// `cache_get_tag_instance` finds it: by its absolute index, and its
    /// identifier too when the index has one.
    #[must_use]
    pub fn tag(&self, instances: u32, count: u32, index: u32) -> Option<Tag> {
        let absolute = index & 0xFFFF;
        // `(short)tag_index`: NONE and the other negative ones are refused
        if index == NONE || absolute >= 0x8000 || absolute >= count {
            return None;
        }
        let at = instances.checked_add(absolute * TAG_INSTANCE_SIZE)?;
        let field = |offset: u32| at.checked_add(offset);
        let found = self.u32(field(12)?)?;
        if index & 0xFFFF_0000 != 0 && found != index {
            return None;
        }
        let name = match self.u32(field(16)?)? {
            0 => String::new(),
            address => self.c_string(address, TAG_NAME_LENGTH)?,
        };
        Some(Tag {
            group: self.u32(at)?,
            parents: [self.u32(field(4)?)?, self.u32(field(8)?)?],
            index: found,
            name,
            data: self.u32(field(20)?)?,
        })
    }

    /// The tag block at `address`, its elements `element_size` bytes each,
    /// if its count is not negative and its elements are all inside.
    #[must_use]
    pub fn block(&self, address: u32, element_size: u32) -> Option<Block> {
        let count = self.u32(address)?;
        let elements = self.u32(address.checked_add(4)?)?;
        if count >= 0x8000_0000 {
            return None;
        }
        let end = u64::from(elements) + u64::from(count) * u64::from(element_size);
        let inside = count == 0 || u64::try_from(self.len()).is_ok_and(|len| end <= len);
        inside.then_some(Block {
            count,
            address: elements,
            element_size,
        })
    }

    /// The tag reference at `address`.
    #[must_use]
    pub fn reference(&self, address: u32) -> Option<Reference> {
        Some(Reference {
            group: self.u32(address)?,
            index: self.u32(address.checked_add(12)?)?,
        })
    }
}

/// The most places [`walk`] tries.
const WALK_PLACES: usize = 1024;

/// The readers' fuzz target's entry: `memory` as the game's, each reader
/// tried at each of its first word-aligned places, with the words there
/// as counts and indices; says how many structures were found.
#[must_use]
pub fn walk(memory: &[u8]) -> usize {
    let mut memory = memory.to_vec();
    View::over(&mut memory, |view| {
        let mut found = 0;
        let places = (0..view.len()).step_by(4).take(WALK_PLACES);
        for address in places.filter_map(|place| u32::try_from(place).ok()) {
            let word = |offset: u32| {
                address
                    .checked_add(offset)
                    .and_then(|at| view.u32(at))
                    .unwrap_or(NONE)
            };
            if let Some(array) = view.data_array(address) {
                found += 1 + array.datums(view).len();
                found += usize::from(array.datum(view, word(0)).is_some());
            }
            found += usize::from(view.tag(address, word(4), word(8)).is_some());
            let element_size = 1 + address % 64;
            if let Some(block) = view.block(address, element_size) {
                found += 1 + usize::from(block.element(block.count.saturating_sub(1)).is_some());
            }
            found += usize::from(view.reference(address).is_some());
            found += usize::from(view.find_data_array(address, word(12), "objects").is_some());
            found += usize::from(
                view.c_string(address, 64)
                    .is_some_and(|text| !text.is_empty()),
            );
            found += view
                .widget_tree(address, word(0) % crate::widgets::LOCAL_PLAYERS)
                .map_or(0, |tree| {
                    tree.widgets.len() + usize::from(tree.focus().is_some())
                });
            found += usize::from(view.at_the_main_menu(address).is_some());
            found += view.wide_string(address).map_or(0, |text| text.len());
        }
        found
    })
}
