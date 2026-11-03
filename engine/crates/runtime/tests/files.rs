//! The file imports the platform answers (`opence_runtime::files`), on a
//! small module that calls each, over a data folder and a save folder made
//! for each test.

use std::fs;
use std::path::PathBuf;

use opence_runtime::{Error, Game, Runtime, Val};

/// the module: each import re-exported as a function the test calls
const MODULE: &str = r#"
(module
  (import "env" "CreateFileA" (func $create (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "ReadFile" (func $read (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "ReadFileEx" (func $read_ex (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "WriteFile" (func $write (param i32 i32 i32 i32 i32) (result i32)))
  (import "env" "SetFilePointer" (func $seek (param i32 i32 i32 i32) (result i32)))
  (import "env" "GetFileSize" (func $size (param i32 i32) (result i32)))
  (import "env" "SetEndOfFile" (func $truncate (param i32) (result i32)))
  (import "env" "CloseHandle" (func $close (param i32) (result i32)))
  (import "env" "GetLastError" (func $last (result i32)))
  (import "env" "SetLastError" (func $set_last (param i32)))
  (import "env" "GetFileAttributesA" (func $attributes (param i32) (result i32)))
  (import "env" "GetFileAttributesExA" (func $attributes_ex (param i32 i32 i32) (result i32)))
  (import "env" "CreateDirectoryA" (func $mkdir (param i32 i32) (result i32)))
  (import "env" "RemoveDirectoryA" (func $rmdir (param i32) (result i32)))
  (import "env" "DeleteFileA" (func $delete (param i32) (result i32)))
  (import "env" "MoveFileA" (func $move (param i32 i32) (result i32)))
  (import "env" "CopyFileA" (func $copy (param i32 i32 i32) (result i32)))
  (import "env" "FindFirstFileA" (func $find_first (param i32 i32) (result i32)))
  (import "env" "FindNextFileA" (func $find_next (param i32 i32) (result i32)))
  (import "env" "XFindClose" (func $find_close (param i32) (result i32)))
  (import "env" "CompareFileTime" (func $compare (param i32 i32) (result i32)))
  (import "env" "SystemTimeToFileTime" (func $system_time (param i32 i32) (result i32)))
  (import "env" "GetDiskFreeSpaceExA" (func $free_space (param i32 i32 i32 i32) (result i32)))
  (import "env" "XCreateSaveGame" (func $create_save (param i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "XDeleteSaveGame" (func $delete_save (param i32 i32) (result i32)))
  (import "env" "XFindFirstSaveGame" (func $first_save (param i32 i32) (result i32)))
  (import "env" "XFindNextSaveGame" (func $next_save (param i32 i32) (result i32)))
  (import "env" "XCalculateSignatureBegin" (func $sign_begin (param i32) (result i32)))
  (import "env" "XCalculateSignatureUpdate" (func $sign_update (param i32 i32 i32) (result i32)))
  (import "env" "XCalculateSignatureEnd" (func $sign_end (param i32 i32) (result i32)))
  (import "env" "XFindFirstNicknameW" (func $first_nickname (param i32 i32 i32) (result i32)))
  (import "env" "XSetNicknameW" (func $set_nickname (param i32 i32) (result i32)))
  (memory (export "memory") 2)
  (func (export "create") (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (call $create (local.get 0) (local.get 1) (local.get 2) (local.get 3)
                  (local.get 4) (local.get 5) (local.get 6)))
  (func (export "read") (param i32 i32 i32 i32 i32) (result i32)
    (call $read (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "read_ex") (param i32 i32 i32 i32 i32) (result i32)
    (call $read_ex (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "write") (param i32 i32 i32 i32 i32) (result i32)
    (call $write (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4)))
  (func (export "seek") (param i32 i32 i32 i32) (result i32)
    (call $seek (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "size") (param i32 i32) (result i32) (call $size (local.get 0) (local.get 1)))
  (func (export "truncate") (param i32) (result i32) (call $truncate (local.get 0)))
  (func (export "close") (param i32) (result i32) (call $close (local.get 0)))
  (func (export "last") (result i32) (call $last))
  (func (export "set_last") (param i32) (call $set_last (local.get 0)))
  (func (export "attributes") (param i32) (result i32) (call $attributes (local.get 0)))
  (func (export "attributes_ex") (param i32 i32 i32) (result i32)
    (call $attributes_ex (local.get 0) (local.get 1) (local.get 2)))
  (func (export "mkdir") (param i32 i32) (result i32) (call $mkdir (local.get 0) (local.get 1)))
  (func (export "rmdir") (param i32) (result i32) (call $rmdir (local.get 0)))
  (func (export "delete") (param i32) (result i32) (call $delete (local.get 0)))
  (func (export "move") (param i32 i32) (result i32) (call $move (local.get 0) (local.get 1)))
  (func (export "copy") (param i32 i32 i32) (result i32)
    (call $copy (local.get 0) (local.get 1) (local.get 2)))
  (func (export "find_first") (param i32 i32) (result i32)
    (call $find_first (local.get 0) (local.get 1)))
  (func (export "find_next") (param i32 i32) (result i32)
    (call $find_next (local.get 0) (local.get 1)))
  (func (export "find_close") (param i32) (result i32) (call $find_close (local.get 0)))
  (func (export "compare") (param i32 i32) (result i32) (call $compare (local.get 0) (local.get 1)))
  (func (export "system_time") (param i32 i32) (result i32)
    (call $system_time (local.get 0) (local.get 1)))
  (func (export "free_space") (param i32 i32 i32 i32) (result i32)
    (call $free_space (local.get 0) (local.get 1) (local.get 2) (local.get 3)))
  (func (export "create_save") (param i32 i32 i32 i32 i32 i32) (result i32)
    (call $create_save (local.get 0) (local.get 1) (local.get 2) (local.get 3)
                       (local.get 4) (local.get 5)))
  (func (export "delete_save") (param i32 i32) (result i32)
    (call $delete_save (local.get 0) (local.get 1)))
  (func (export "first_save") (param i32 i32) (result i32)
    (call $first_save (local.get 0) (local.get 1)))
  (func (export "next_save") (param i32 i32) (result i32)
    (call $next_save (local.get 0) (local.get 1)))
  (func (export "sign_begin") (param i32) (result i32) (call $sign_begin (local.get 0)))
  (func (export "sign_update") (param i32 i32 i32) (result i32)
    (call $sign_update (local.get 0) (local.get 1) (local.get 2)))
  (func (export "sign_end") (param i32 i32) (result i32)
    (call $sign_end (local.get 0) (local.get 1)))
  (func (export "first_nickname") (param i32 i32 i32) (result i32)
    (call $first_nickname (local.get 0) (local.get 1) (local.get 2)))
  (func (export "set_nickname") (param i32 i32) (result i32)
    (call $set_nickname (local.get 0) (local.get 1))))
"#;

const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const CREATE_NEW: u32 = 1;
const CREATE_ALWAYS: u32 = 2;
const OPEN_EXISTING: u32 = 3;
const OPEN_ALWAYS: u32 = 4;
const FILE_BEGIN: u32 = 0;
const FILE_END: u32 = 2;
const INVALID: u32 = u32::MAX;
const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_PATH_NOT_FOUND: u32 = 3;
const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_NO_MORE_FILES: u32 = 18;
const ERROR_FILE_EXISTS: u32 = 80;
const ERROR_ALREADY_EXISTS: u32 = 183;
const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

/// where the tests put their strings, buffers and structures
const PATH: u32 = 0x100;
const OTHER: u32 = 0x500;
const BUFFER: u32 = 0x1000;
const DONE: u32 = 0x900;
const OVERLAPPED: u32 = 0x940;
const FOUND: u32 = 0x2000;

/// a test's data folder and save folder, empty
struct Folders {
    data: PathBuf,
    saves: PathBuf,
}

fn folders(test: &str) -> Result<Folders, Error> {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("files")
        .join(test);
    let _ = fs::remove_dir_all(&root);
    let folders = Folders {
        data: root.join("data"),
        saves: root.join("saves"),
    };
    fs::create_dir_all(&folders.data).map_err(io)?;
    fs::create_dir_all(&folders.saves).map_err(io)?;
    Ok(folders)
}

fn io(error: std::io::Error) -> Error {
    Error::Setup(error.to_string())
}

fn game(folders: &Folders) -> Result<Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    let mut game = Runtime::new(None)?.load(&bytes)?;
    game.set_folders(&folders.data, &folders.saves);
    Ok(game)
}

/// calls `name` with these unsigned arguments; its one result, or 0
fn call(game: &mut Game, name: &str, args: &[u32]) -> Result<u32, Error> {
    let args: Vec<Val> = args.iter().map(|arg| Val::I32(arg.cast_signed())).collect();
    Ok(match game.call_with(name, &args)?.as_slice() {
        [Val::I32(value)] => value.cast_unsigned(),
        [] => 0,
        other => panic!("{name} returned {other:?}"),
    })
}

/// writes `text` as a C string at `address`
fn string(game: &mut Game, address: u32, text: &str) -> Result<(), Error> {
    let mut bytes = text.as_bytes().to_vec();
    bytes.push(0);
    game.write(u64::from(address), &bytes)
}

fn word(game: &Game, address: u32) -> Result<u32, Error> {
    let bytes = game.read(u64::from(address), 4)?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// opens the Xbox path `path` as `CreateFileA` would
fn open(game: &mut Game, path: &str, access: u32, disposition: u32) -> Result<u32, Error> {
    string(game, PATH, path)?;
    call(game, "create", &[PATH, access, 0, 0, disposition, 0, 0])
}

#[test]
fn create_file_reads_a_map_below_the_data_folder_whatever_its_case() -> Result<(), Error> {
    let folders = folders("read")?;
    fs::create_dir_all(folders.data.join("maps")).map_err(io)?;
    fs::write(folders.data.join("maps/Levels.map"), b"head0123456789").map_err(io)?;
    let mut game = game(&folders)?;

    let file = open(
        &mut game,
        r"d:\MAPS\levels.MAP",
        GENERIC_READ,
        OPEN_EXISTING,
    )?;
    assert_ne!(file, INVALID);
    assert_eq!(call(&mut game, "last", &[])?, 0);
    assert_eq!(call(&mut game, "size", &[file, DONE])?, 14);
    assert_eq!(word(&game, DONE)?, 0, "the size's high word");

    // a read from the file's position moves it
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 4, DONE, 0])?, 1);
    assert_eq!(game.read(u64::from(BUFFER), 4)?, b"head".to_vec());
    assert_eq!(word(&game, DONE)?, 4);
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 2, DONE, 0])?, 1);
    assert_eq!(game.read(u64::from(BUFFER), 2)?, b"01".to_vec());

    // an overlapped read is at its offset, and leaves the position alone
    game.write(u64::from(OVERLAPPED), &[0; 20])?;
    game.write(u64::from(OVERLAPPED + 8), &8u32.to_le_bytes())?;
    assert_eq!(
        call(&mut game, "read", &[file, BUFFER, 3, DONE, OVERLAPPED])?,
        1
    );
    assert_eq!(game.read(u64::from(BUFFER), 3)?, b"456".to_vec());
    assert_eq!(
        word(&game, OVERLAPPED + 4)?,
        3,
        "InternalHigh: what was read"
    );
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 2, DONE, 0])?, 1);
    assert_eq!(game.read(u64::from(BUFFER), 2)?, b"23".to_vec());

    // past the end, a read is short, and at the end it reads nothing
    assert_eq!(call(&mut game, "seek", &[file, 10, 0, FILE_BEGIN])?, 10);
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 100, DONE, 0])?, 1);
    assert_eq!(word(&game, DONE)?, 4);
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 100, DONE, 0])?, 1);
    assert_eq!(word(&game, DONE)?, 0);
    // (from the end, backwards)
    assert_eq!(
        call(
            &mut game,
            "seek",
            &[file, (-4i32).cast_unsigned(), 0, FILE_END]
        )?,
        10
    );

    // a buffer outside the memory is refused, not a trap
    assert_eq!(call(&mut game, "read", &[file, 0x1_FFF0, 100, DONE, 0])?, 0);

    // and closed, the handle is no longer a file
    assert_eq!(call(&mut game, "close", &[file])?, 1);
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 1, DONE, 0])?, 0);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_INVALID_HANDLE);
    assert_eq!(call(&mut game, "close", &[file])?, 0);

    // a file that is not there
    assert_eq!(
        open(
            &mut game,
            r"d:\maps\missing.map",
            GENERIC_READ,
            OPEN_EXISTING
        )?,
        INVALID
    );
    assert_eq!(call(&mut game, "last", &[])?, ERROR_FILE_NOT_FOUND);
    Ok(())
}

#[test]
fn paths_never_leave_their_folders() -> Result<(), Error> {
    let folders = folders("confined")?;
    let outside = folders.data.parent().map(|root| root.join("outside.txt"));
    if let Some(outside) = &outside {
        fs::write(outside, b"not the game's").map_err(io)?;
    }
    fs::write(folders.data.join("inside.txt"), b"the game's").map_err(io)?;
    let mut game = game(&folders)?;

    // .. is skipped, as FATX names are never only dots
    for path in [
        r"d:\..\outside.txt",
        r"d:\..\..\outside.txt",
        r"d:/../outside.txt",
    ] {
        assert_eq!(
            open(&mut game, path, GENERIC_READ, OPEN_EXISTING)?,
            INVALID,
            "{path}"
        );
    }
    let file = open(&mut game, r"d:\..\inside.txt", GENERIC_READ, OPEN_EXISTING)?;
    assert_ne!(file, INVALID);
    call(&mut game, "close", &[file])?;
    // a string without its NUL inside the memory is not a path
    game.write(0x1_FFFC, b"abcd")?;
    assert_eq!(
        call(
            &mut game,
            "create",
            &[0x1_FFFC, GENERIC_READ, 0, 0, OPEN_EXISTING, 0, 0]
        )?,
        INVALID
    );
    Ok(())
}

#[test]
fn other_drives_are_folders_of_the_save_folder() -> Result<(), Error> {
    let folders = folders("saves")?;
    let mut game = game(&folders)?;

    string(&mut game, PATH, r"z:\cache")?;
    assert_eq!(call(&mut game, "mkdir", &[PATH, 0])?, 1);
    assert_eq!(call(&mut game, "mkdir", &[PATH, 0])?, 0);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_ALREADY_EXISTS);

    let file = open(
        &mut game,
        r"z:\cache\Cache.bin",
        GENERIC_READ | GENERIC_WRITE,
        CREATE_ALWAYS,
    )?;
    assert_ne!(file, INVALID);
    assert_eq!(call(&mut game, "last", &[])?, 0, "it did not exist");
    game.write(u64::from(BUFFER), b"0123456789")?;
    assert_eq!(call(&mut game, "write", &[file, BUFFER, 10, DONE, 0])?, 1);
    assert_eq!(word(&game, DONE)?, 10);
    // SetEndOfFile cuts the file at its position
    assert_eq!(call(&mut game, "seek", &[file, 6, 0, FILE_BEGIN])?, 6);
    assert_eq!(call(&mut game, "truncate", &[file])?, 1);
    call(&mut game, "close", &[file])?;
    assert_eq!(
        fs::read(folders.saves.join("z/cache/Cache.bin")).map_err(io)?,
        b"012345".to_vec()
    );

    // opened again always, it existed
    let again = open(&mut game, r"z:\CACHE\cache.bin", GENERIC_WRITE, OPEN_ALWAYS)?;
    assert_ne!(again, INVALID);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_ALREADY_EXISTS);
    call(&mut game, "close", &[again])?;
    assert_eq!(
        open(&mut game, r"z:\cache\cache.bin", GENERIC_WRITE, CREATE_NEW)?,
        INVALID
    );
    assert_eq!(call(&mut game, "last", &[])?, ERROR_FILE_EXISTS);
    // a file in a folder that is not there
    assert_eq!(
        open(
            &mut game,
            r"u:\nowhere\file.bin",
            GENERIC_WRITE,
            CREATE_ALWAYS
        )?,
        INVALID
    );
    assert_eq!(call(&mut game, "last", &[])?, ERROR_PATH_NOT_FOUND);
    Ok(())
}

#[test]
fn read_file_ex_reads_at_its_offset_and_queues_its_completion() -> Result<(), Error> {
    let folders = folders("read_ex")?;
    fs::write(folders.data.join("bitmaps.map"), b"abcdefgh").map_err(io)?;
    let mut game = game(&folders)?;
    let file = open(&mut game, r"d:\bitmaps.map", GENERIC_READ, OPEN_EXISTING)?;

    game.write(u64::from(OVERLAPPED), &[0; 20])?;
    game.write(u64::from(OVERLAPPED + 8), &5u32.to_le_bytes())?;
    assert_eq!(
        call(&mut game, "read_ex", &[file, BUFFER, 8, OVERLAPPED, 7])?,
        1
    );
    assert_eq!(game.read(u64::from(BUFFER), 3)?, b"fgh".to_vec());
    // past the end, the completion says so
    game.write(u64::from(OVERLAPPED + 8), &8u32.to_le_bytes())?;
    assert_eq!(
        call(&mut game, "read_ex", &[file, BUFFER, 8, OVERLAPPED, 7])?,
        1
    );
    let completions: Vec<_> = game
        .completions()
        .iter()
        .map(|done| (done.routine, done.error, done.transferred, done.overlapped))
        .collect();
    assert_eq!(
        completions,
        vec![(7, 0, 3, OVERLAPPED), (7, 38, 0, OVERLAPPED)]
    );
    Ok(())
}

/// the name `FindFirstFileA` or `FindNextFileA` found
fn found(game: &Game) -> Result<(u32, u32, String), Error> {
    let data = game.read(u64::from(FOUND), 304)?;
    let attributes = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let size = u32::from_le_bytes([data[32], data[33], data[34], data[35]]);
    let name = &data[44..];
    let end = name
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(name.len());
    Ok((
        attributes,
        size,
        String::from_utf8_lossy(&name[..end]).into_owned(),
    ))
}

#[test]
fn find_first_file_lists_what_matches_in_name_order() -> Result<(), Error> {
    let folders = folders("find")?;
    let maps = folders.data.join("maps");
    fs::create_dir_all(maps.join("sub.map")).map_err(io)?;
    for (name, bytes) in [
        ("b30.map", 3),
        ("A10.MAP", 1),
        ("c10.txt", 2),
        ("d40.map", 4),
    ] {
        fs::write(maps.join(name), vec![0; bytes]).map_err(io)?;
    }
    let mut game = game(&folders)?;

    string(&mut game, PATH, r"d:\maps\*.map")?;
    let find = call(&mut game, "find_first", &[PATH, FOUND])?;
    assert_ne!(find, INVALID);
    let mut names = vec![found(&game)?];
    while call(&mut game, "find_next", &[find, FOUND])? == 1 {
        names.push(found(&game)?);
    }
    assert_eq!(call(&mut game, "last", &[])?, ERROR_NO_MORE_FILES);
    assert_eq!(
        names,
        vec![
            (FILE_ATTRIBUTE_NORMAL, 1, "A10.MAP".to_string()),
            (FILE_ATTRIBUTE_NORMAL, 3, "b30.map".to_string()),
            (FILE_ATTRIBUTE_NORMAL, 4, "d40.map".to_string()),
            (FILE_ATTRIBUTE_DIRECTORY, 0, "sub.map".to_string()),
        ]
    );
    assert_eq!(call(&mut game, "find_close", &[find])?, 1);
    assert_eq!(call(&mut game, "find_next", &[find, FOUND])?, 0);

    // ? is one character, and nothing matching is not found
    string(&mut game, PATH, r"d:\maps\?10.*")?;
    let find = call(&mut game, "find_first", &[PATH, FOUND])?;
    assert_eq!(found(&game)?.2, "A10.MAP");
    assert_eq!(call(&mut game, "find_next", &[find, FOUND])?, 1);
    assert_eq!(found(&game)?.2, "c10.txt");
    call(&mut game, "find_close", &[find])?;
    string(&mut game, PATH, r"d:\maps\*.xbe")?;
    assert_eq!(call(&mut game, "find_first", &[PATH, FOUND])?, INVALID);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_FILE_NOT_FOUND);
    Ok(())
}

#[test]
fn attributes_moves_copies_and_deletes() -> Result<(), Error> {
    let folders = folders("attributes")?;
    fs::create_dir_all(folders.data.join("maps")).map_err(io)?;
    fs::write(folders.data.join("maps/a.map"), vec![7; 300]).map_err(io)?;
    let mut game = game(&folders)?;

    string(&mut game, PATH, r"d:\maps")?;
    assert_eq!(
        call(&mut game, "attributes", &[PATH])?,
        FILE_ATTRIBUTE_DIRECTORY
    );
    string(&mut game, PATH, r"d:\maps\a.map")?;
    assert_eq!(
        call(&mut game, "attributes", &[PATH])?,
        FILE_ATTRIBUTE_NORMAL
    );
    assert_eq!(call(&mut game, "attributes_ex", &[PATH, 0, FOUND])?, 1);
    let data = game.read(u64::from(FOUND), 36)?;
    assert_eq!(
        u32::from_le_bytes([data[32], data[33], data[34], data[35]]),
        300
    );
    assert_ne!(&data[20..28], &[0; 8], "a last write time");
    string(&mut game, PATH, r"d:\maps\b.map")?;
    assert_eq!(call(&mut game, "attributes", &[PATH])?, INVALID);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_FILE_NOT_FOUND);

    // CopyFileA, refused over a file when asked
    string(&mut game, PATH, r"d:\maps\a.map")?;
    string(&mut game, OTHER, r"d:\maps\b.map")?;
    assert_eq!(call(&mut game, "copy", &[PATH, OTHER, 1])?, 1);
    assert_eq!(call(&mut game, "copy", &[PATH, OTHER, 1])?, 0);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_FILE_EXISTS);
    assert_eq!(call(&mut game, "copy", &[PATH, OTHER, 0])?, 1);
    // MoveFileA never replaces a file
    assert_eq!(call(&mut game, "move", &[PATH, OTHER])?, 0);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_ALREADY_EXISTS);
    string(&mut game, OTHER, r"d:\maps\c.map")?;
    assert_eq!(call(&mut game, "move", &[PATH, OTHER])?, 1);
    assert_eq!(
        fs::read(folders.data.join("maps/c.map")).map_err(io)?.len(),
        300
    );
    // DeleteFileA, then a folder that is empty can go
    assert_eq!(call(&mut game, "delete", &[OTHER])?, 1);
    assert_eq!(call(&mut game, "delete", &[OTHER])?, 0);
    string(&mut game, OTHER, r"d:\maps\b.map")?;
    assert_eq!(call(&mut game, "delete", &[OTHER])?, 1);
    string(&mut game, PATH, r"d:\maps")?;
    assert_eq!(call(&mut game, "rmdir", &[PATH])?, 1);
    assert!(!folders.data.join("maps").exists());
    Ok(())
}

#[test]
fn file_times_compare_and_convert_as_win32_s() -> Result<(), Error> {
    let folders = folders("times")?;
    let mut game = game(&folders)?;
    // 15 November 2001, 12:30:15.005, a Thursday
    let system: [u16; 8] = [2001, 11, 4, 15, 12, 30, 15, 5];
    let bytes: Vec<u8> = system.iter().flat_map(|part| part.to_le_bytes()).collect();
    game.write(u64::from(BUFFER), &bytes)?;
    assert_eq!(call(&mut game, "system_time", &[BUFFER, PATH])?, 1);
    assert_eq!(
        game.read(u64::from(PATH), 8)?,
        0x01C1_6DD1_4702_E8D0u64.to_le_bytes().to_vec()
    );
    // a month of 13 is not a time
    game.write(u64::from(BUFFER + 2), &13u16.to_le_bytes())?;
    assert_eq!(call(&mut game, "system_time", &[BUFFER, OTHER])?, 0);

    game.write(u64::from(OTHER), &0x01C1_6DD1_4702_E8D1u64.to_le_bytes())?;
    assert_eq!(call(&mut game, "compare", &[PATH, OTHER])?, u32::MAX);
    assert_eq!(call(&mut game, "compare", &[OTHER, PATH])?, 1);
    assert_eq!(call(&mut game, "compare", &[PATH, PATH])?, 0);

    // SetLastError and GetLastError keep the game's own codes
    call(&mut game, "set_last", &[1234])?;
    assert_eq!(call(&mut game, "last", &[])?, 1234);

    // GetDiskFreeSpaceExA reports the same 8 GB, free and in all, everywhere
    string(&mut game, PATH, "z:\\")?;
    assert_eq!(
        call(&mut game, "free_space", &[PATH, BUFFER, BUFFER + 8, 0])?,
        1
    );
    assert_eq!(
        game.read(u64::from(BUFFER), 16)?,
        [(8u64 << 30).to_le_bytes(); 2].concat()
    );
    Ok(())
}

#[test]
fn without_folders_nothing_is_found() -> Result<(), Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    let mut game = Runtime::new(None)?.load(&bytes)?;
    assert_eq!(
        open(&mut game, r"d:\maps\a.map", GENERIC_READ, OPEN_EXISTING)?,
        INVALID
    );
    assert_eq!(call(&mut game, "last", &[])?, ERROR_PATH_NOT_FOUND);
    Ok(())
}

/// writes `text` as a NUL-terminated UTF-16 string at `address`
fn wide(game: &mut Game, address: u32, text: &str) -> Result<(), Error> {
    let bytes: Vec<u8> = text
        .encode_utf16()
        .chain([0])
        .flat_map(u16::to_le_bytes)
        .collect();
    game.write(u64::from(address), &bytes)
}

/// the NUL-terminated UTF-16 string at `address`, of at most `most` units
fn read_wide(game: &Game, address: u32, most: u64) -> Result<String, Error> {
    let bytes = game.read(u64::from(address), most * 2)?;
    let units: Vec<u16> = bytes
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .take_while(|&unit| unit != 0)
        .collect();
    Ok(String::from_utf16_lossy(&units))
}

/// the C string at `address`, of at most `most` bytes
fn read_string(game: &Game, address: u32, most: u64) -> Result<String, Error> {
    let bytes = game.read(u64::from(address), most)?;
    let end = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    Ok(String::from_utf8_lossy(&bytes[..end]).into_owned())
}

/// where `XGAME_FIND_DATA` keeps the save's directory and its name, after
/// its `WIN32_FIND_DATAA`
const SAVE_DIRECTORY: u32 = 320;
const SAVE_NAME: u32 = 580;

#[test]
fn saved_games_are_folders_of_u_named_by_their_metadata() -> Result<(), Error> {
    let folders = folders("saved_games")?;
    let mut game = game(&folders)?;
    string(&mut game, PATH, "u:\\")?;
    wide(&mut game, OTHER, "Cortana")?;
    let create = |game: &mut Game, disposition: u32, size: u32| {
        call(
            game,
            "create_save",
            &[PATH, OTHER, disposition, 0, BUFFER, size],
        )
    };
    // saved_game_file_name_unique: OPEN_EXISTING fails while it is not there
    assert_eq!(create(&mut game, OPEN_EXISTING, 64)?, ERROR_FILE_NOT_FOUND);
    assert_eq!(create(&mut game, CREATE_NEW, 64)?, 0);
    assert_eq!(read_string(&game, BUFFER, 64)?, "u:\\000000000000\\");
    let folder = folders.saves.join("u").join("000000000000");
    assert!(folder.join("SaveMeta.xbx").is_file());
    assert_eq!(create(&mut game, CREATE_NEW, 64)?, ERROR_ALREADY_EXISTS);
    assert_eq!(create(&mut game, OPEN_EXISTING, 64)?, 0);
    assert_eq!(read_string(&game, BUFFER, 64)?, "u:\\000000000000\\");
    assert_eq!(
        create(&mut game, OPEN_EXISTING, 4)?,
        ERROR_INSUFFICIENT_BUFFER
    );
    wide(&mut game, OTHER, "Chief")?;
    assert_eq!(create(&mut game, OPEN_ALWAYS, 64)?, 0);
    assert_eq!(read_string(&game, BUFFER, 64)?, "u:\\000000000001\\");
    // the game writes its files in the folder it was given
    string(&mut game, PATH, "u:\\000000000001\\blam.sav")?;
    let handle = call(
        &mut game,
        "create",
        &[
            PATH,
            GENERIC_WRITE,
            0,
            0,
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            0,
        ],
    )?;
    assert_ne!(handle, INVALID);
    call(&mut game, "close", &[handle])?;
    // a find lists them in folder order, with their names
    string(&mut game, PATH, "u:\\")?;
    let find = call(&mut game, "first_save", &[PATH, FOUND])?;
    assert_ne!(find, INVALID);
    assert_eq!(word(&game, FOUND)?, FILE_ATTRIBUTE_DIRECTORY);
    assert_eq!(read_string(&game, FOUND + 44, 260)?, "000000000000");
    assert_eq!(
        read_string(&game, FOUND + SAVE_DIRECTORY, 260)?,
        "u:\\000000000000\\"
    );
    assert_eq!(read_wide(&game, FOUND + SAVE_NAME, 128)?, "Cortana");
    assert_eq!(call(&mut game, "next_save", &[find, FOUND])?, 1);
    assert_eq!(read_wide(&game, FOUND + SAVE_NAME, 128)?, "Chief");
    assert_eq!(call(&mut game, "next_save", &[find, FOUND])?, 0);
    assert_eq!(call(&mut game, "last", &[])?, ERROR_NO_MORE_FILES);
    assert_eq!(call(&mut game, "find_close", &[find])?, 1);
    // deleting takes the folder and what is in it
    assert_eq!(call(&mut game, "delete_save", &[PATH, OTHER])?, 0);
    assert!(!folders.saves.join("u").join("000000000001").exists());
    assert_eq!(
        call(&mut game, "delete_save", &[PATH, OTHER])?,
        ERROR_FILE_NOT_FOUND
    );
    let find = call(&mut game, "first_save", &[PATH, FOUND])?;
    assert_eq!(read_wide(&game, FOUND + SAVE_NAME, 128)?, "Cortana");
    assert_eq!(call(&mut game, "next_save", &[find, FOUND])?, 0);
    call(&mut game, "find_close", &[find])?;
    // and the lowest free number is used again
    assert_eq!(create(&mut game, CREATE_NEW, 64)?, 0);
    assert_eq!(read_string(&game, BUFFER, 64)?, "u:\\000000000001\\");
    // with none, a find fails
    call(&mut game, "delete_save", &[PATH, OTHER])?;
    wide(&mut game, OTHER, "Cortana")?;
    call(&mut game, "delete_save", &[PATH, OTHER])?;
    assert_eq!(call(&mut game, "first_save", &[PATH, FOUND])?, INVALID);
    Ok(())
}

#[test]
fn a_signature_is_a_hash_of_what_it_was_given() -> Result<(), Error> {
    let folders = folders("signatures")?;
    let mut game = game(&folders)?;
    game.write(u64::from(BUFFER), b"blam! saved")?;
    let sign = |game: &mut Game, parts: &[(u32, u32)]| -> Result<Vec<u8>, Error> {
        let handle = call(game, "sign_begin", &[0])?;
        assert_ne!(handle, INVALID);
        for &(at, size) in parts {
            assert_eq!(call(game, "sign_update", &[handle, at, size])?, 0);
        }
        assert_eq!(call(game, "sign_end", &[handle, DONE])?, 0);
        // the end frees it
        assert_eq!(
            call(game, "sign_end", &[handle, DONE])?,
            ERROR_INVALID_HANDLE
        );
        game.read(u64::from(DONE), 20)
    };
    let whole = sign(&mut game, &[(BUFFER, 11)])?;
    let parts = sign(&mut game, &[(BUFFER, 4), (BUFFER + 4, 7)])?;
    assert_eq!(whole, parts);
    assert_ne!(whole, vec![0; 20]);
    let other = sign(&mut game, &[(BUFFER, 10)])?;
    assert_ne!(whole, other);
    // data outside the memory is refused, not a trap
    let handle = call(&mut game, "sign_begin", &[0])?;
    assert_eq!(
        call(&mut game, "sign_update", &[handle, 0xFFFF_FFF0, 64])?,
        87
    );
    Ok(())
}

#[test]
fn nicknames_are_kept_in_the_save_folder_newest_first() -> Result<(), Error> {
    let folders = folders("nicknames")?;
    let mut game = game(&folders)?;
    assert_eq!(
        call(&mut game, "first_nickname", &[0, BUFFER, 32])?,
        INVALID
    );
    wide(&mut game, OTHER, "Banshee")?;
    assert_eq!(call(&mut game, "set_nickname", &[OTHER, 1])?, 1);
    wide(&mut game, OTHER, "Warthog")?;
    assert_eq!(call(&mut game, "set_nickname", &[OTHER, 1])?, 1);
    let find = call(&mut game, "first_nickname", &[0, BUFFER, 32])?;
    assert_ne!(find, INVALID);
    assert_eq!(read_wide(&game, BUFFER, 32)?, "Warthog");
    assert_eq!(call(&mut game, "find_close", &[find])?, 1);
    // they outlast the game, and a short buffer takes what fits
    let mut again = self::game(&folders)?;
    let find = call(&mut again, "first_nickname", &[0, BUFFER, 4])?;
    assert_eq!(read_wide(&again, BUFFER, 4)?, "War");
    call(&mut again, "find_close", &[find])?;
    // set again, a name moves to the front
    wide(&mut again, OTHER, "Banshee")?;
    call(&mut again, "set_nickname", &[OTHER, 1])?;
    let find = call(&mut again, "first_nickname", &[0, BUFFER, 32])?;
    assert_eq!(read_wide(&again, BUFFER, 32)?, "Banshee");
    call(&mut again, "find_close", &[find])?;
    Ok(())
}

/// a module asking for the init script (`halo_init_text`)
const INIT: &str = r#"
(module
  (import "env" "halo_init_text" (func $init (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "init") (param i32 i32) (result i32) (call $init (local.get 0) (local.get 1)))
  (data (i32.const 0x100) "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"))
"#;

#[test]
fn the_init_script_is_what_the_platform_was_given() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&wat::parse_str(INIT).unwrap_or_default())?;
    let init = |game: &mut Game, buffer: i32, size: i32| -> Result<Option<i32>, Error> {
        Ok(game
            .call_with("init", &[Val::I32(buffer), Val::I32(size)])?
            .first()
            .and_then(Val::i32))
    };
    // none at first
    assert_eq!(init(&mut game, 0x100, 16)?, Some(0));
    game.set_init_commands("map_name a\\b\n");
    // its length, asked with no buffer
    assert_eq!(init(&mut game, 0, 0)?, Some(13));
    assert_eq!(init(&mut game, 0x100, 16)?, Some(13));
    assert_eq!(game.read(0x100, 14)?, b"map_name a\\b\n\xff".to_vec());
    // cut to the buffer's size, still giving the whole length
    assert_eq!(init(&mut game, 0x200, 4)?, Some(13));
    assert_eq!(game.read(0x200, 5)?, b"map_\0".to_vec());
    // past the memory, nothing is written
    assert_eq!(init(&mut game, 0x1_0000 - 2, 16)?, Some(13));
    Ok(())
}

#[test]
fn devgulch_is_read_from_blood_gulchs_map_when_it_has_none_of_its_own() -> Result<(), Error> {
    let folders = folders("devgulch")?;
    fs::create_dir_all(folders.data.join("maps")).map_err(io)?;
    fs::write(folders.data.join("maps/bloodgulch.map"), b"gulch").map_err(io)?;
    let mut game = game(&folders)?;

    let file = open(&mut game, r"maps\DevGulch.map", GENERIC_READ, OPEN_EXISTING)?;
    assert_ne!(file, INVALID);
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 5, DONE, 0])?, 1);
    assert_eq!(game.read(u64::from(BUFFER), 5)?, b"gulch".to_vec());
    assert_eq!(call(&mut game, "close", &[file])?, 1);
    assert!(!folders.data.join("maps/devgulch.map").exists());

    // a devgulch.map of its own is read instead
    fs::write(folders.data.join("maps/devgulch.map"), b"own").map_err(io)?;
    let file = open(
        &mut game,
        r"d:\maps\devgulch.map",
        GENERIC_READ,
        OPEN_EXISTING,
    )?;
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 3, DONE, 0])?, 1);
    assert_eq!(game.read(u64::from(BUFFER), 3)?, b"own".to_vec());
    assert_eq!(call(&mut game, "close", &[file])?, 1);

    // and no other map is taken for another's
    assert_eq!(
        open(&mut game, r"maps\other.map", GENERIC_READ, OPEN_EXISTING)?,
        INVALID
    );
    Ok(())
}

/// a cache file's header (`source/cache/cache_files.c`) as far as its name,
/// which is at 0x20 and 0x20 long, then `tail`
fn map_named(name: &[u8], tail: &[u8]) -> Vec<u8> {
    let mut map = vec![0xAB; 0x20];
    let mut field = name.to_vec();
    field.resize(0x20, 0);
    map.extend(field);
    map.extend(tail);
    map
}

#[test]
fn devgulchs_header_names_devgulch_as_the_original_asks_of_a_map() -> Result<(), Error> {
    let folders = folders("devgulch_name")?;
    fs::create_dir_all(folders.data.join("maps")).map_err(io)?;
    fs::write(
        folders.data.join("maps/bloodgulch.map"),
        map_named(b"bloodgulch", b"tags"),
    )
    .map_err(io)?;
    let mut game = game(&folders)?;
    let devgulch = map_named(b"devgulch", b"tags");
    let file = open(&mut game, r"maps\devgulch.map", GENERIC_READ, OPEN_EXISTING)?;
    assert_ne!(file, INVALID);

    // read at its position, in two reads that split the name
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 0x28, DONE, 0])?, 1);
    assert_eq!(
        call(&mut game, "read", &[file, BUFFER + 0x28, 0x40, DONE, 0])?,
        1
    );
    assert_eq!(game.read(u64::from(BUFFER), 0x44)?, devgulch);

    // at an OVERLAPPED's offset, by ReadFile and by ReadFileEx
    game.write(u64::from(OVERLAPPED), &[0; 20])?;
    game.write(u64::from(OVERLAPPED + 8), &0x1Eu32.to_le_bytes())?;
    assert_eq!(
        call(&mut game, "read", &[file, BUFFER, 8, DONE, OVERLAPPED])?,
        1
    );
    assert_eq!(
        game.read(u64::from(BUFFER), 8)?,
        devgulch[0x1E..0x26].to_vec()
    );
    game.write(u64::from(OVERLAPPED), &[0; 20])?;
    game.write(u64::from(OVERLAPPED + 8), &0x24u32.to_le_bytes())?;
    assert_eq!(
        call(&mut game, "read_ex", &[file, BUFFER, 0x20, OVERLAPPED, 7])?,
        1
    );
    assert_eq!(
        game.read(u64::from(BUFFER), 0x20)?,
        devgulch[0x24..0x44].to_vec()
    );
    assert_eq!(call(&mut game, "close", &[file])?, 1);

    // Blood Gulch's own map keeps its own name
    let file = open(
        &mut game,
        r"maps\bloodgulch.map",
        GENERIC_READ,
        OPEN_EXISTING,
    )?;
    assert_eq!(call(&mut game, "read", &[file, BUFFER, 0x44, DONE, 0])?, 1);
    assert_eq!(
        game.read(u64::from(BUFFER), 0x44)?,
        map_named(b"bloodgulch", b"tags")
    );
    assert_eq!(call(&mut game, "close", &[file])?, 1);
    Ok(())
}
