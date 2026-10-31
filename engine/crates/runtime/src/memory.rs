//! The Xbox's memory, as the game asks for it: the contiguous window its
//! caches live in (`XPhysicalAlloc` and its page protection), and the C
//! heap behind `system_malloc` (`GlobalAlloc`, `GlobalReAlloc`,
//! `LocalFree`, `GlobalMemoryStatus`).
//!
//! The window follows `port/linux/src/xbox_memory.c`: physical address P is
//! at 0x80000000 + P, and the game places its game state and tag cache at
//! fixed addresses that way (`source/cache/physical_memory_map.c`). Blocks
//! are page-granular, placed exactly when asked and otherwise top-down
//! first fit, as the Xbox kernel's allocator is. A block is cleared when it
//! is allocated, as fresh pages would be. Wasm's memory has no pages to
//! protect, so protection is only remembered, for the game's own checks
//! (`physical_memory_verify`).

use wasmtime::{Caller, Extern, Val};

use super::{with_space, Record};

/// where the window begins: physical address 0
pub const WINDOW_BASE: u32 = 0x8000_0000;
/// the window's size at the desktop sizes (`port/linux/src/platform.h`),
/// whose caches outgrow the Xbox's 64 MB (`halo_port_capacity.h`)
pub const WINDOW_SIZE: u32 = 0x2000_0000;
/// a page, the window's grain
const PAGE: u32 = 0x1000;
/// the window's pages
const PAGES: u32 = WINDOW_SIZE / PAGE;
/// an `XPhysicalAlloc` physical address at or above the window's size asks
/// for a block anywhere (the game passes -1)
pub(crate) const ANYWHERE: u32 = WINDOW_SIZE;

/// Win32's page protections, as the game passes them
pub const PAGE_NOACCESS: u32 = 0x01;
/// (read only)
pub const PAGE_READONLY: u32 = 0x02;
/// (read and write)
pub const PAGE_READWRITE: u32 = 0x04;
/// (uncached: no meaning without the GPU's caches)
const PAGE_NOCACHE: u32 = 0x200;
/// (write-combined: likewise)
const PAGE_WRITECOMBINE: u32 = 0x400;

/// `GlobalAlloc`'s flag for cleared memory
const GMEM_ZEROINIT: u32 = 0x40;
/// the header before each `GlobalAlloc` block: its size, padded to keep
/// malloc's 16-byte alignment
const HEADER: u32 = 16;
/// what `GlobalMemoryStatus` reports as physical memory: an Xbox's 64 MB,
/// so the game's 32-bit size arithmetic cannot overflow (as the Linux
/// port), and the same on every machine
const REPORTED_MEMORY: u32 = 64 << 20;
/// and as virtual memory, a 32-bit Windows process's 2 GB less 128 KB
const REPORTED_VIRTUAL: u32 = 0x7FFE_0000;

/// the window's pages: which are allocated, with what protection
pub struct Window {
    /// per page: 0 if free, otherwise its protection (`PAGE_*`)
    protection: Vec<u32>,
    /// per page: on a block's first page, the block's length in pages
    length: Vec<u32>,
    /// per allocated page: its block's first page (the search passes a
    /// block whole)
    owner: Vec<u32>,
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

/// the protection a block keeps: the cache attributes dropped, and none
/// meaning read and write
fn kept(protect: u32) -> u32 {
    match protect & !(PAGE_WRITECOMBINE | PAGE_NOCACHE) {
        0 => PAGE_READWRITE,
        protect => protect,
    }
}

impl Window {
    /// An empty window.
    #[must_use]
    pub fn new() -> Self {
        Self {
            protection: vec![0; PAGES as usize],
            length: vec![0; PAGES as usize],
            owner: vec![0; PAGES as usize],
        }
    }

    /// The page `address` is on, if it is in the window.
    fn page(address: u32) -> Option<u32> {
        let offset = address.wrapping_sub(WINDOW_BASE);
        (address >= WINDOW_BASE && offset < WINDOW_SIZE).then_some(offset / PAGE)
    }

    /// The last allocated page of `count` from `first`, if any (and the
    /// last page of the window if they run past it).
    fn used(&self, first: u32, count: u32) -> Option<u32> {
        let end = first.checked_add(count).filter(|end| *end <= PAGES);
        let Some(end) = end else {
            return Some(PAGES - 1);
        };
        (first..end)
            .rev()
            .find(|page| self.protection.get(*page as usize).is_some_and(|p| *p != 0))
    }

    /// The first page of the block allocated page `page` is in (`page`
    /// itself if it is free: the window's end, which [`Self::used`] gives
    /// for a run past it).
    fn block_start(&self, page: u32) -> u32 {
        let allocated = self.protection.get(page as usize).is_some_and(|p| *p != 0);
        match self.owner.get(page as usize) {
            Some(first) if allocated && *first <= page => *first,
            _ => page,
        }
    }

    /// `XPhysicalAlloc(size, physical_address, alignment, protect)`: a
    /// block of `size` bytes (at least one page) at `physical_address` if
    /// that is in the window, else the highest free run aligned to
    /// `alignment`; its address, or `None` if there is no room.
    pub fn allocate(
        &mut self,
        size: u32,
        physical_address: u32,
        alignment: u32,
        protect: u32,
    ) -> Option<u32> {
        let count = size.div_ceil(PAGE).max(1);
        let alignment = (alignment / PAGE).max(1);
        let first = if physical_address < ANYWHERE {
            // (placed: exactly there, or not at all)
            let wanted = physical_address / PAGE;
            self.used(wanted, count).is_none().then_some(wanted)?
        } else {
            // top-down first fit: below each allocated block found, the
            // next candidate that leaves it out
            let mut candidate = PAGES.checked_sub(count)?;
            loop {
                candidate -= candidate % alignment;
                match self.used(candidate, count) {
                    None => break candidate,
                    Some(used) => candidate = self.block_start(used).checked_sub(count)?,
                }
            }
        };
        let protect = kept(protect);
        for page in first..first + count {
            if let Some(slot) = self.protection.get_mut(page as usize) {
                *slot = protect;
            }
            if let Some(slot) = self.owner.get_mut(page as usize) {
                *slot = first;
            }
        }
        if let Some(slot) = self.length.get_mut(first as usize) {
            *slot = count;
        }
        Some(WINDOW_BASE + first * PAGE)
    }

    /// `XPhysicalFree(address)`: frees the block that begins at `address`;
    /// the bytes it held, or `None` if no block begins there.
    pub fn free(&mut self, address: u32) -> Option<std::ops::Range<u32>> {
        let first = Self::page(address)?;
        let count = self.length.get_mut(first as usize).map(std::mem::take)?;
        if count == 0 {
            return None;
        }
        for page in first..first + count {
            if let Some(slot) = self.protection.get_mut(page as usize) {
                *slot = 0;
            }
        }
        let start = WINDOW_BASE + first * PAGE;
        Some(start..start + count * PAGE)
    }

    /// `VirtualProtect(address, size, protect, &old)`: sets the protection
    /// of the allocated pages `address..address + size` touches; returns
    /// the old protection of the first. Outside the window every page
    /// reads and writes, and stays so.
    pub fn protect(&mut self, address: u32, size: u32, protect: u32) -> u32 {
        let Some(first) = Self::page(address) else {
            return PAGE_READWRITE;
        };
        let old = self.query(address);
        let end = u64::from(address) + u64::from(size);
        let last =
            (end.div_ceil(u64::from(PAGE)) - u64::from(WINDOW_BASE / PAGE)).min(u64::from(PAGES));
        let protect = kept(protect);
        for page in u64::from(first)..last {
            if let Some(slot) = self.protection.get_mut(page as usize) {
                if *slot != 0 {
                    *slot = protect;
                }
            }
        }
        old
    }

    /// `XQueryMemoryProtect(address)`: the protection of the page
    /// `address` is on; a free page in the window has no access, and every
    /// page outside it reads and writes.
    #[must_use]
    pub fn query(&self, address: u32) -> u32 {
        match Self::page(address).and_then(|page| self.protection.get(page as usize)) {
            None => PAGE_READWRITE,
            Some(0) => PAGE_NOACCESS,
            Some(protect) => *protect,
        }
    }
}

/// an import the platform answers: its arguments, and where its result
/// goes
pub(crate) type Handler = fn(&mut Caller<'_, Record>, &[Val], &mut [Val]) -> wasmtime::Result<()>;

/// the memory import named "module.name", if this module answers it
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "XPhysicalAlloc" => x_physical_alloc,
        "XPhysicalFree" => x_physical_free,
        "XPhysicalProtect" => x_physical_protect,
        "VirtualProtect" => virtual_protect,
        "XQueryMemoryProtect" => x_query_memory_protect,
        "GlobalAlloc" => global_alloc,
        "GlobalReAlloc" => global_re_alloc,
        "LocalFree" => local_free,
        "GlobalMemoryStatus" => global_memory_status,
        _ => return None,
    };
    Some(handler)
}

/// argument `index` of an import, as the unsigned 32 bits the game means
pub(crate) fn arg(params: &[Val], index: usize) -> u32 {
    match params.get(index) {
        Some(Val::I32(value)) => value.cast_unsigned(),
        _ => 0,
    }
}

/// sets an import's one result
pub(crate) fn give(results: &mut [Val], value: u32) {
    if let Some(result) = results.first_mut() {
        *result = Val::I32(value.cast_signed());
    }
}

/// calls the game's exported `name` with `args`, which returns one i32
pub(crate) fn call_game(
    caller: &mut Caller<'_, Record>,
    name: &str,
    args: &[Val],
) -> wasmtime::Result<u32> {
    let func = caller
        .get_export(name)
        .and_then(Extern::into_func)
        .ok_or_else(|| wasmtime::format_err!("the game exports no {name}"))?;
    let mut results = [Val::I32(0)];
    let returns = func.ty(&*caller).results().len();
    func.call(&mut *caller, args, &mut results[..returns.min(1)])?;
    Ok(arg(&results, 0))
}

/// `XPhysicalAlloc(size, physical_address, alignment, protect)`: a cleared
/// block from the window, or NULL
pub(crate) fn x_physical_alloc(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (size, physical, alignment, protect) = (
        arg(params, 0),
        arg(params, 1),
        arg(params, 2),
        arg(params, 3),
    );
    let block = window_block(caller, size, physical, alignment, protect)?;
    give(results, block.unwrap_or(0));
    Ok(())
}

/// a cleared block of `size` bytes from the window, as [`Window::allocate`]
/// places it; `None` if there is no room, or the memory does not cover the
/// window
pub(crate) fn window_block(
    caller: &mut Caller<'_, Record>,
    size: u32,
    physical: u32,
    alignment: u32,
    protect: u32,
) -> wasmtime::Result<Option<u32>> {
    with_space(caller, |space, record| {
        // (the window must be in the memory: the platform grows it so)
        let covered = space.len() as u64 >= u64::from(WINDOW_BASE) + u64::from(WINDOW_SIZE);
        let block = covered
            .then(|| record.window().allocate(size, physical, alignment, protect))
            .flatten()?;
        space.fill(block, size.div_ceil(PAGE).max(1) * PAGE, 0);
        Some(block)
    })
}

/// `XPhysicalFree(address)`
pub(crate) fn x_physical_free(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _results: &mut [Val],
) -> wasmtime::Result<()> {
    caller.data().window().free(arg(params, 0));
    Ok(())
}

/// `VirtualProtect(address, size, protect, old)`: TRUE, with the old
/// protection at `old` if it is not NULL; FALSE if `old` is outside the
/// memory
pub(crate) fn virtual_protect(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, size, protect, old_at) = (
        arg(params, 0),
        arg(params, 1),
        arg(params, 2),
        arg(params, 3),
    );
    let old = caller.data().window().protect(address, size, protect);
    let done = old_at == 0 || with_space(caller, |space, _| space.put(old_at, old))?;
    give(results, u32::from(done));
    Ok(())
}

/// `XPhysicalProtect(address, size, protect)`
pub(crate) fn x_physical_protect(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _results: &mut [Val],
) -> wasmtime::Result<()> {
    caller
        .data()
        .window()
        .protect(arg(params, 0), arg(params, 1), arg(params, 2));
    Ok(())
}

/// `XQueryMemoryProtect(address)`
pub(crate) fn x_query_memory_protect(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let protect = caller.data().window().query(arg(params, 0));
    give(results, protect);
    Ok(())
}

/// a `GlobalAlloc` block of `size` bytes from the game's malloc, its size
/// in the header before it, cleared if `zero`; 0 if malloc has no room
fn global_alloc_block(
    caller: &mut Caller<'_, Record>,
    size: u32,
    zero: bool,
) -> wasmtime::Result<u32> {
    let Some(total) = size.checked_add(HEADER) else {
        return Ok(0);
    };
    let block = call_game(caller, "malloc", &[Val::I32(total.cast_signed())])?;
    if block == 0 {
        return Ok(0);
    }
    let placed = with_space(caller, |space, _| {
        space.fill(block, HEADER, 0)
            && space.put(block, size)
            && (!zero || space.fill(block + HEADER, size, 0))
    })?;
    if !placed {
        return Err(wasmtime::format_err!(
            "malloc gave {block:#x}, outside the memory"
        ));
    }
    Ok(block + HEADER)
}

/// `GlobalAlloc(flags, size)`: a block from the C heap, cleared with
/// `GMEM_ZEROINIT`; NULL if there is no room
pub(crate) fn global_alloc(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (flags, size) = (arg(params, 0), arg(params, 1));
    let block = global_alloc_block(caller, size, flags & GMEM_ZEROINIT != 0)?;
    give(results, block);
    Ok(())
}

/// the size in the header of the `GlobalAlloc` block at `block`, if its
/// header is inside the memory
fn global_size(caller: &mut Caller<'_, Record>, block: u32) -> wasmtime::Result<Option<u32>> {
    with_space(caller, |space, _| {
        block
            .checked_sub(HEADER)
            .and_then(|header| space.get(header))
    })
}

/// `GlobalReAlloc(block, size, flags)`: the block moved to one of `size`
/// bytes, its contents kept up to the smaller size (and the rest cleared
/// with `GMEM_ZEROINIT`); NULL, with the block left as it was, if there is
/// no room. A NULL block is a `GlobalAlloc`.
pub(crate) fn global_re_alloc(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (old, size, flags) = (arg(params, 0), arg(params, 1), arg(params, 2));
    let zero = flags & GMEM_ZEROINIT != 0;
    if old == 0 {
        let block = global_alloc_block(caller, size, zero)?;
        give(results, block);
        return Ok(());
    }
    let Some(old_size) = global_size(caller, old)? else {
        give(results, 0);
        return Ok(());
    };
    let block = global_alloc_block(caller, size, zero)?;
    if block != 0 {
        let kept = old_size.min(size);
        with_space(caller, |space, _| space.copy_within(old, kept, block))?;
        call_game(caller, "free", &[Val::I32((old - HEADER).cast_signed())])?;
    }
    give(results, block);
    Ok(())
}

/// `LocalFree(block)`: frees a `GlobalAlloc` block; NULL
pub(crate) fn local_free(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let block = arg(params, 0);
    if let Some(header) = block.checked_sub(HEADER).filter(|_| block != 0) {
        call_game(caller, "free", &[Val::I32(header.cast_signed())])?;
    }
    give(results, 0);
    Ok(())
}

/// `GlobalMemoryStatus(status)`: fills the eight words of a `MEMORYSTATUS`
/// (its length, the load, then total and available physical memory, page
/// file and virtual memory) with an Xbox's, nothing in use
pub(crate) fn global_memory_status(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _results: &mut [Val],
) -> wasmtime::Result<()> {
    let status = arg(params, 0);
    let words = [
        32,
        0,
        REPORTED_MEMORY,
        REPORTED_MEMORY,
        0,
        0,
        REPORTED_VIRTUAL,
        REPORTED_VIRTUAL,
    ];
    let bytes: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
    with_space(caller, |space, _| space.write(status, &bytes))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the game's fixed blocks (`source/cache/physical_memory_map.c`, at
    /// the desktop sizes of `halo_port_capacity.h`)
    const GAME_STATE: (u32, u32) = (0x81A0_0000, 0x0140_0000);
    const TAG_CACHE: (u32, u32) = (0x803A_6000, 0x0160_0000);

    #[test]
    fn the_game_state_and_tag_cache_land_where_the_game_asks() {
        let mut window = Window::new();
        for (address, size) in [GAME_STATE, TAG_CACHE] {
            let placed = window.allocate(size, address & 0x7FFF_FFFF, 0, PAGE_READWRITE);
            assert_eq!(placed, Some(address));
        }
        // (and a block placed over one of them is refused)
        assert_eq!(
            window.allocate(0x1000, 0x01A0_1000, 0, PAGE_READWRITE),
            None
        );
    }

    #[test]
    fn blocks_anywhere_are_placed_top_down_first_fit() {
        let mut window = Window::new();
        let top = WINDOW_BASE + WINDOW_SIZE;
        assert_eq!(window.allocate(0x1800, u32::MAX, 0, 0), Some(top - 0x2000));
        assert_eq!(window.allocate(0x1000, u32::MAX, 0, 0), Some(top - 0x3000));
        // aligned: the highest 64 KB boundary whose run is free
        assert_eq!(
            window.allocate(0x1000, u32::MAX, 0x1_0000, 0),
            Some(top - 0x1_0000)
        );
        // a freed block is found again
        assert_eq!(window.free(top - 0x2000), Some(top - 0x2000..top));
        assert_eq!(window.allocate(0x2000, u32::MAX, 0, 0), Some(top - 0x2000));
    }

    #[test]
    fn small_blocks_below_large_ones_are_placed_quickly() {
        // (the menus' textures, a page each, made below the map's caches:
        // the search passes each block whole, not page by page)
        let mut window = Window::new();
        let top = WINDOW_BASE + WINDOW_SIZE;
        let large = WINDOW_SIZE / 4 * 3;
        assert_eq!(window.allocate(large, u32::MAX, 0, 0), Some(top - large));
        let started = std::time::Instant::now();
        for block in 1..=1000 {
            let placed = window.allocate(0x1000, u32::MAX, 0x80, 0);
            assert_eq!(placed, Some(top - large - block * PAGE));
        }
        let took = started.elapsed();
        assert!(took < std::time::Duration::from_millis(250), "{took:?}");
        // and a freed one among them is found again
        let freed = top - large - 500 * PAGE;
        assert!(window.free(freed).is_some());
        assert_eq!(window.allocate(0x1000, u32::MAX, 0, 0), Some(freed));
    }

    #[test]
    fn a_block_that_cannot_fit_is_refused() {
        let mut window = Window::new();
        assert_eq!(window.allocate(WINDOW_SIZE + 1, u32::MAX, 0, 0), None);
        assert_eq!(
            window.allocate(0x1000, WINDOW_SIZE - 0x1000, 0, 0),
            Some(WINDOW_BASE + WINDOW_SIZE - 0x1000)
        );
        assert_eq!(window.allocate(0x2000, WINDOW_SIZE - 0x1000, 0, 0), None);
        // a page at each quarter leaves no run of half the window, and the
        // largest run, between them, is found below the top one
        let mut window = Window::new();
        for quarter in [WINDOW_SIZE / 4, WINDOW_SIZE / 4 * 3] {
            assert!(window.allocate(0x1000, quarter, 0, 0).is_some());
        }
        assert_eq!(window.allocate(WINDOW_SIZE / 2, u32::MAX, 0, 0), None);
        let between = window.allocate(WINDOW_SIZE / 2 - 0x1000, u32::MAX, 0, 0);
        assert_eq!(between, Some(WINDOW_BASE + WINDOW_SIZE / 4 + 0x1000));
    }

    #[test]
    fn protection_is_remembered_per_page() {
        let mut window = Window::new();
        let (tags, size) = TAG_CACHE;
        assert_eq!(window.query(tags), PAGE_NOACCESS);
        let placed = window.allocate(
            size,
            tags & 0x7FFF_FFFF,
            0,
            PAGE_READWRITE | PAGE_WRITECOMBINE,
        );
        assert_eq!(placed, Some(tags));
        assert_eq!(window.query(tags + size - 1), PAGE_READWRITE);
        assert_eq!(window.protect(tags, size, PAGE_READONLY), PAGE_READWRITE);
        assert_eq!(window.query(tags), PAGE_READONLY);
        assert_eq!(window.query(tags + size - 1), PAGE_READONLY);
        // the free page after it stays free
        assert_eq!(window.query(tags + size), PAGE_NOACCESS);
        // outside the window every page reads and writes
        assert_eq!(window.query(0x1000), PAGE_READWRITE);
        assert_eq!(
            window.protect(0x1000, 0x1000, PAGE_READONLY),
            PAGE_READWRITE
        );
        assert_eq!(window.query(0x1000), PAGE_READWRITE);
        // and a free leaves no access
        assert!(window.free(tags).is_some());
        assert_eq!(window.query(tags), PAGE_NOACCESS);
        assert_eq!(window.free(tags), None);
    }
}
