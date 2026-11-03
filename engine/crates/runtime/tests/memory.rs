//! The memory imports the platform answers (`opence_runtime::memory`), on a
//! small module that calls each and has a malloc of its own: a bump
//! allocator from 0x1000 that counts its frees.

use opence_runtime::{Error, Runtime, Val};

/// the module: each import re-exported as a function the test calls, and
/// enough memory, 0xA0000000 bytes, to cover the Xbox's window
const MODULE: &str = r#"
(module
  (import "env" "XPhysicalAlloc" (func $alloc (param i32 i32 i32 i32) (result i32)))
  (import "env" "XPhysicalFree" (func $pfree (param i32)))
  (import "env" "XPhysicalProtect" (func $pprotect (param i32 i32 i32)))
  (import "env" "VirtualProtect" (func $vprotect (param i32 i32 i32 i32) (result i32)))
  (import "env" "XQueryMemoryProtect" (func $query (param i32) (result i32)))
  (import "env" "GlobalAlloc" (func $galloc (param i32 i32) (result i32)))
  (import "env" "GlobalReAlloc" (func $grealloc (param i32 i32 i32) (result i32)))
  (import "env" "LocalFree" (func $lfree (param i32) (result i32)))
  (import "env" "GlobalMemoryStatus" (func $status (param i32)))
  (memory (export "memory") 40960)
  (global $next (mut i32) (i32.const 0x1000))
  (global $frees (mut i32) (i32.const 0))
  (func (export "malloc") (param $size i32) (result i32)
    (local $block i32)
    (local.set $block (global.get $next))
    (global.set $next
      (i32.and (i32.add (i32.add (global.get $next) (local.get $size)) (i32.const 15))
               (i32.const -16)))
    (local.get $block))
  (func (export "free") (param i32)
    (global.set $frees (i32.add (global.get $frees) (i32.const 1))))
  (func (export "frees") (result i32) (global.get $frees))
  (func (export "physical_alloc") (param i32 i32 i32 i32) (result i32)
    (call $alloc (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "physical_free") (param i32) (call $pfree (local.get 0)))
  (func (export "physical_protect") (param i32 i32 i32)
    (call $pprotect (local.get 0) (local.get 1) (local.get 2)))
  (func (export "virtual_protect") (param i32 i32 i32 i32) (result i32)
    (call $vprotect (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "query") (param i32) (result i32) (call $query (local.get 0)))
  (func (export "global_alloc") (param i32 i32) (result i32)
    (call $galloc (local.get 0) (local.get 1)))
  (func (export "global_realloc") (param i32 i32 i32) (result i32)
    (call $grealloc (local.get 0) (local.get 1) (local.get 2)))
  (func (export "local_free") (param i32) (result i32) (call $lfree (local.get 0)))
  (func (export "status") (param i32) (call $status (local.get 0))))
"#;

const PAGE_READONLY: u32 = 0x02;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_NOACCESS: u32 = 0x01;
const PAGE_WRITECOMBINE: u32 = 0x400;
const GMEM_ZEROINIT: u32 = 0x40;

/// the game's tag cache: physical address 0x003A6000, 0x1600000 bytes
const TAG_CACHE: u32 = 0x803A_6000;
const TAG_CACHE_SIZE: u32 = 0x0160_0000;

/// calls `name` with these unsigned arguments; its one result, or 0
fn call(game: &mut opence_runtime::Game, name: &str, args: &[u32]) -> Result<u32, Error> {
    let args: Vec<Val> = args.iter().map(|arg| Val::I32(arg.cast_signed())).collect();
    Ok(match game.call_with(name, &args)?.as_slice() {
        [Val::I32(value)] => value.cast_unsigned(),
        [] => 0,
        other => panic!("{name} returned {other:?}"),
    })
}

fn game() -> Result<opence_runtime::Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

#[test]
fn x_physical_alloc_places_the_tag_cache_cleared_and_protects_it() -> Result<(), Error> {
    let mut game = game()?;
    // (what was there before is cleared)
    game.write(u64::from(TAG_CACHE), &[0xAA; 64])?;
    let block = call(
        &mut game,
        "physical_alloc",
        &[
            TAG_CACHE_SIZE,
            TAG_CACHE & 0x7FFF_FFFF,
            0,
            PAGE_READWRITE | PAGE_WRITECOMBINE,
        ],
    )?;
    assert_eq!(block, TAG_CACHE);
    assert_eq!(game.read(u64::from(TAG_CACHE), 64)?, vec![0; 64]);
    assert_eq!(call(&mut game, "query", &[TAG_CACHE])?, PAGE_READWRITE);
    // the same pages again are refused
    let again = call(
        &mut game,
        "physical_alloc",
        &[0x1000, TAG_CACHE & 0x7FFF_FFFF, 0, 0],
    )?;
    assert_eq!(again, 0);
    // cache_files.c makes the tag cache read only once it is loaded
    call(
        &mut game,
        "physical_protect",
        &[TAG_CACHE, TAG_CACHE_SIZE, PAGE_READONLY],
    )?;
    assert_eq!(
        call(&mut game, "query", &[TAG_CACHE + TAG_CACHE_SIZE - 1])?,
        PAGE_READONLY
    );
    // VirtualProtect gives the old protection, and outside the window
    // nothing changes
    let old_at = 0x100;
    assert_eq!(
        call(
            &mut game,
            "virtual_protect",
            &[TAG_CACHE, 0x1000, PAGE_READWRITE, old_at]
        )?,
        1
    );
    assert_eq!(
        game.read(u64::from(old_at), 4)?,
        PAGE_READONLY.to_le_bytes().to_vec()
    );
    assert_eq!(
        call(
            &mut game,
            "virtual_protect",
            &[0x2000, 0x1000, PAGE_READONLY, old_at]
        )?,
        1
    );
    assert_eq!(
        game.read(u64::from(old_at), 4)?,
        PAGE_READWRITE.to_le_bytes().to_vec()
    );
    // an old-protection pointer outside the memory is FALSE, not a trap
    assert_eq!(
        call(
            &mut game,
            "virtual_protect",
            &[TAG_CACHE, 0x1000, PAGE_READWRITE, 0xFFFF_FFF0]
        )?,
        0
    );
    // and freed, it has no access
    call(&mut game, "physical_free", &[TAG_CACHE])?;
    assert_eq!(call(&mut game, "query", &[TAG_CACHE])?, PAGE_NOACCESS);
    // anywhere: the top of the window
    let anywhere = call(
        &mut game,
        "physical_alloc",
        &[0x4000, u32::MAX, 0, PAGE_READWRITE],
    )?;
    assert_eq!(anywhere, 0xA000_0000 - 0x4000);
    Ok(())
}

#[test]
fn x_physical_alloc_refuses_a_memory_that_does_not_cover_the_window() -> Result<(), Error> {
    let small = MODULE.replace(
        "(memory (export \"memory\") 40960)",
        "(memory (export \"memory\") 1)",
    );
    let bytes = wat::parse_str(&small).unwrap_or_default();
    let mut game = Runtime::new(None)?.load(&bytes)?;
    assert_eq!(
        call(&mut game, "physical_alloc", &[0x1000, u32::MAX, 0, 0])?,
        0
    );
    Ok(())
}

#[test]
fn global_alloc_comes_from_the_game_s_malloc_with_its_size_kept() -> Result<(), Error> {
    let mut game = game()?;
    // dirty the bump allocator's memory, so clearing shows
    game.write(0x1000, &[0xAA; 256])?;
    let plain = call(&mut game, "global_alloc", &[0, 16])?;
    assert_eq!(plain, 0x1000 + 16, "after the 16-byte header");
    assert_eq!(game.read(0x1000, 4)?, 16u32.to_le_bytes().to_vec());
    assert_eq!(
        game.read(u64::from(plain), 16)?,
        vec![0xAA; 16],
        "not cleared without ZEROINIT"
    );
    let zeroed = call(&mut game, "global_alloc", &[GMEM_ZEROINIT, 32])?;
    assert_eq!(game.read(u64::from(zeroed), 32)?, vec![0; 32]);
    // GlobalReAlloc keeps the contents up to the smaller size, clears the
    // growth with ZEROINIT and frees the old block
    game.write(u64::from(plain), b"sixteen bytes!!!")?;
    let grown = call(&mut game, "global_realloc", &[plain, 24, GMEM_ZEROINIT])?;
    assert_ne!(grown, plain);
    assert_eq!(
        game.read(u64::from(grown), 16)?,
        b"sixteen bytes!!!".to_vec()
    );
    assert_eq!(game.read(u64::from(grown) + 16, 8)?, vec![0; 8]);
    assert_eq!(call(&mut game, "frees", &[])?, 1);
    let shrunk = call(&mut game, "global_realloc", &[grown, 4, 0])?;
    assert_eq!(game.read(u64::from(shrunk), 4)?, b"sixt".to_vec());
    // a NULL block is a GlobalAlloc
    let fresh = call(&mut game, "global_realloc", &[0, 8, GMEM_ZEROINIT])?;
    assert_eq!(game.read(u64::from(fresh), 8)?, vec![0; 8]);
    assert_eq!(call(&mut game, "frees", &[])?, 2);
    // LocalFree frees through the game's free and gives NULL; NULL is
    // nothing to free
    assert_eq!(call(&mut game, "local_free", &[fresh])?, 0);
    assert_eq!(call(&mut game, "local_free", &[0])?, 0);
    assert_eq!(call(&mut game, "frees", &[])?, 3);
    Ok(())
}

#[test]
fn global_memory_status_reports_an_xbox_with_nothing_in_use() -> Result<(), Error> {
    let mut game = game()?;
    call(&mut game, "status", &[0x200])?;
    let words: Vec<u32> = game
        .read(0x200, 32)?
        .as_chunks::<4>()
        .0
        .iter()
        .map(|chunk| u32::from_le_bytes(*chunk))
        .collect();
    assert_eq!(
        words,
        vec![32, 0, 64 << 20, 64 << 20, 0, 0, 0x7FFE_0000, 0x7FFE_0000]
    );
    // a status outside the memory is ignored, not a trap
    call(&mut game, "status", &[0xFFFF_FFF0])?;
    Ok(())
}
