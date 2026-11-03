//! A recording of the tag fields the importer reads, for the coverage
//! census: each read with the tag group its tag was asked for as, the
//! blocks it was reached through, its offset and width in the element
//! it is in, and the reader's label for it. The census lays that over the original's tag structures to
//! mark the fields the importer keeps.
//!
//! Nothing is recorded outside [`recording`], and a read costs one atomic
//! load then.

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use crate::tags::GroupTag;

/// A block a read was reached through: its offset in the element that
/// holds it and the size of its elements.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hop {
    /// The block's offset in the element holding it.
    pub at: u32,
    /// The size of each of its elements.
    pub element_size: u32,
}

/// A read of a tag field.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Read {
    /// The group the tag was asked for as (`unit` for a vehicle read as a
    /// unit).
    pub group: GroupTag,
    /// The blocks it was reached through, outermost first.
    pub hops: Vec<Hop>,
    /// Its offset in the element it is in.
    pub at: u32,
    /// How many bytes it read (1 for a string: its start).
    pub width: u32,
    /// The label the reader was given (`"physics ground depth scale"`).
    pub label: String,
}

static RECORDING: AtomicBool = AtomicBool::new(false);
static READS: Mutex<BTreeSet<Read>> = Mutex::new(BTreeSet::new());

/// Runs `run`, and gives its result with every distinct read it made
/// (reads on other threads at the same time are in it too).
pub fn recording<T>(run: impl FnOnce() -> T) -> (T, BTreeSet<Read>) {
    if let Ok(mut reads) = READS.lock() {
        reads.clear();
    }
    RECORDING.store(true, Ordering::SeqCst);
    let result = run();
    RECORDING.store(false, Ordering::SeqCst);
    let reads = READS
        .lock()
        .map(|mut reads| std::mem::take(&mut *reads))
        .unwrap_or_default();
    (result, reads)
}

/// Notes a read, if a recording is running.
pub(crate) fn note(group: GroupTag, hops: &[Hop], at: usize, width: usize, label: &str) {
    if !RECORDING.load(Ordering::Relaxed) {
        return;
    }
    let (Ok(at), Ok(width)) = (u32::try_from(at), u32::try_from(width)) else {
        return;
    };
    if let Ok(mut reads) = READS.lock() {
        reads.insert(Read {
            group,
            hops: hops.to_vec(),
            at,
            width,
            label: label.replace(['\t', '\n'], " "),
        });
    }
}
