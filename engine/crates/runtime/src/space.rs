//! The game's memory as the imports see it: its own (a [`wasmtime::Memory`])
//! or shared between its threads (a [`wasmtime::SharedMemory`]), read and
//! written only through checked ranges.
//!
//! Shared memory may change under the host while it reads, so it is never
//! lent out as a slice: wiggle copies each range in or out, and a range
//! filled in place ([`Space::update`]) is copied in, changed and copied
//! back. The game's own threads race on such a range just as they would on
//! Windows.

use std::cell::UnsafeCell;

use wiggle::{GuestMemory, GuestPtr};

/// the most bytes [`Space::fill`] writes to shared memory at once
const FILL_CHUNK: u32 = 1 << 16;

/// the game's memory, borrowed for one import call
pub(crate) struct Space<'a> {
    memory: GuestMemory<'a>,
    len: usize,
    /// while an import's answer is being recorded, each write it makes
    /// ([`crate::replay`])
    pub(crate) journal: Option<Vec<(u32, Vec<u8>)>>,
}

/// `len` bytes at `address`, as wiggle names them
fn span(address: u32, len: u32) -> GuestPtr<[u8]> {
    GuestPtr::new((address, len))
}

impl<'a> Space<'a> {
    /// a memory the game's thread owns
    pub(crate) fn own(data: &'a mut [u8]) -> Self {
        let len = data.len();
        Self {
            memory: GuestMemory::Unshared(data),
            len,
            journal: None,
        }
    }

    /// a memory the game's threads share
    pub(crate) fn shared(data: &'a [UnsafeCell<u8>]) -> Self {
        Self {
            memory: GuestMemory::Shared(data),
            len: data.len(),
            journal: None,
        }
    }

    /// its size in bytes
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    /// whether `len` bytes at `address` are all inside
    fn inside(&self, address: u32, len: u32) -> bool {
        (address as usize)
            .checked_add(len as usize)
            .is_some_and(|end| end <= self.len)
    }

    /// `len` bytes at `address`, if all of them are inside
    pub(crate) fn read(&self, address: u32, len: u32) -> Option<Vec<u8>> {
        self.memory.to_vec(span(address, len)).ok()
    }

    /// up to `max` bytes at `address`, cut at the end of the memory; `None`
    /// if `address` is outside it
    pub(crate) fn read_up_to(&self, address: u32, max: u32) -> Option<Vec<u8>> {
        let left = self.len.checked_sub(address as usize)?;
        let len = u32::try_from(left).map_or(max, |left| left.min(max));
        self.read(address, len)
    }

    /// writes `bytes` at `address`; false (and nothing written) if any of
    /// them would be outside
    pub(crate) fn write(&mut self, address: u32, bytes: &[u8]) -> bool {
        let Ok(len) = u32::try_from(bytes.len()) else {
            return false;
        };
        let written = self
            .memory
            .copy_from_slice(bytes, span(address, len))
            .is_ok();
        if written {
            self.note(address, bytes);
        }
        written
    }

    /// keeps the write of `bytes` at `address` in the journal, if one is kept
    fn note(&mut self, address: u32, bytes: &[u8]) {
        if let Some(journal) = &mut self.journal {
            journal.push((address, bytes.to_vec()));
        }
    }

    /// sets `len` bytes at `address` to `byte`; false (and nothing set) if
    /// any of them is outside
    pub(crate) fn fill(&mut self, address: u32, len: u32, byte: u8) -> bool {
        if !self.inside(address, len) {
            return false;
        }
        if let GuestMemory::Unshared(data) = &mut self.memory {
            let start = address as usize;
            let filled = data
                .get_mut(start..start + len as usize)
                .map(|bytes| bytes.fill(byte))
                .is_some();
            if filled && self.journal.is_some() {
                self.note(address, &vec![byte; len as usize]);
            }
            return filled;
        }
        let chunk = vec![byte; len.min(FILL_CHUNK) as usize];
        let mut done = 0;
        while done < len {
            let step = (len - done).min(FILL_CHUNK);
            if !self.write(address + done, chunk.get(..step as usize).unwrap_or(&[])) {
                return false;
            }
            done += step;
        }
        true
    }

    /// runs `change` on the `len` bytes at `address` and keeps what it
    /// leaves there; `None` (and nothing changed) if any of them is outside
    pub(crate) fn update<R>(
        &mut self,
        address: u32,
        len: u32,
        change: impl FnOnce(&mut [u8]) -> R,
    ) -> Option<R> {
        if let GuestMemory::Unshared(data) = &mut self.memory {
            let start = address as usize;
            let bytes = data.get_mut(start..start.checked_add(len as usize)?)?;
            let result = change(bytes);
            if self.journal.is_some() {
                let left = bytes.to_vec();
                self.note(address, &left);
            }
            return Some(result);
        }
        let mut bytes = self.read(address, len)?;
        let result = change(&mut bytes);
        self.write(address, &bytes).then_some(result)
    }

    /// copies `len` bytes from `from` to `to`; false (and nothing copied) if
    /// either range is outside
    pub(crate) fn copy_within(&mut self, from: u32, len: u32, to: u32) -> bool {
        if !self.inside(to, len) {
            return false;
        }
        match self.read(from, len) {
            Some(bytes) => self.write(to, &bytes),
            None => false,
        }
    }

    /// the little-endian u32 at `address`
    pub(crate) fn get(&self, address: u32) -> Option<u32> {
        let bytes = self.read(address, 4)?;
        Some(u32::from_le_bytes(<[u8; 4]>::try_from(bytes).ok()?))
    }

    /// the little-endian u64 at `address`
    pub(crate) fn get64(&self, address: u32) -> Option<u64> {
        let bytes = self.read(address, 8)?;
        Some(u64::from_le_bytes(<[u8; 8]>::try_from(bytes).ok()?))
    }

    /// writes `value` at `address`, little-endian
    pub(crate) fn put(&mut self, address: u32, value: u32) -> bool {
        self.write(address, &value.to_le_bytes())
    }

    /// writes `value` at `address`, little-endian
    pub(crate) fn put64(&mut self, address: u32, value: u64) -> bool {
        self.write(address, &value.to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a shared memory of `len` cells, as wasmtime lends one
    fn cells(len: usize) -> Vec<UnsafeCell<u8>> {
        (0..len).map(|_| UnsafeCell::new(0)).collect()
    }

    /// the same checks on either kind of memory
    fn checks(space: &mut Space<'_>) {
        assert_eq!(space.len(), 64);
        assert!(space.put(4, 0x1122_3344));
        assert_eq!(space.get(4), Some(0x1122_3344));
        assert!(space.put64(8, 0x0102_0304_0506_0708));
        assert_eq!(space.get64(8), Some(0x0102_0304_0506_0708));
        assert!(!space.put(62, 1));
        assert_eq!(space.get(61), None);
        assert_eq!(space.read(u32::MAX, 2), None);
        assert!(space.fill(16, 8, 0xAB));
        assert_eq!(space.read(16, 8), Some(vec![0xAB; 8]));
        assert!(!space.fill(60, 8, 1));
        assert_eq!(space.read(60, 4), Some(vec![0; 4]));
        assert_eq!(space.update(16, 2, |bytes| bytes.reverse()), Some(()));
        assert_eq!(space.update(63, 2, |_| ()), None);
        assert!(space.copy_within(4, 4, 32));
        assert_eq!(space.get(32), Some(0x1122_3344));
        assert!(!space.copy_within(4, 4, 62));
        assert_eq!(space.read_up_to(60, 16).map(|bytes| bytes.len()), Some(4));
        assert_eq!(space.read_up_to(64, 16), Some(Vec::new()));
        assert_eq!(space.read_up_to(65, 16), None);
    }

    #[test]
    fn a_memory_of_its_own_is_read_and_written_in_bounds() {
        let mut data = vec![0; 64];
        checks(&mut Space::own(&mut data));
    }

    #[test]
    fn a_shared_memory_is_read_and_written_in_bounds() {
        let data = cells(64);
        checks(&mut Space::shared(&data));
        assert_eq!(Space::shared(&data).get(32), Some(0x1122_3344));
    }
}
