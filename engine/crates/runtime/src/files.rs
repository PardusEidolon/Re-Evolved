//! The Xbox's files, as the game asks for them: `CreateFileA` and the
//! handle calls on what it opens, the attribute, directory and find calls,
//! and the file times (`source/tag_files/files_windows.c`,
//! `source/cache/cache_files_windows.c`, the saved games).
//!
//! A saved game (`XCreateSaveGame`) is a folder of its drive named by a
//! twelve-digit hexadecimal number, the lowest free, with its name in a
//! `SaveMeta.xbx` as the Xbox keeps it (`Name=` in UTF-16). Its signature
//! (`XCalculateSignature*`) is the first 20 bytes of a SHA-256 of what it
//! was given: the Xbox's keyed signature needs the console's key, and these
//! saves are only read here. The nicknames (`XSetNicknameW`) are kept, newest
//! first, in `nicknames.txt` of the save folder.
//!
//! The paths follow `port/linux/src/xbox_files.c`. `d:\` (and a path with
//! no drive) is the data folder, where the maps are; every other drive
//! `X:\` is the folder `x` of the save folder, made on first use. Each
//! component is matched without regard to case, an exact match first, as
//! FATX matches them, and a component that is empty or only dots and spaces
//! is skipped, which also keeps every path inside its folder. With no
//! folders set, no path is found. `maps\devgulch.map`, with no file of its
//! own, is Blood Gulch's map (`DONORS`), read with devgulch's name in its
//! header, as the original asks of a map.
//!
//! `ReadFileEx` and `WriteFileEx` complete at once; their completion
//! routines wait in [`Files::completions`] for the game's next alertable
//! wait, which the threads service delivers. Directory listings are in name
//! order, so a find is the same on every machine.

use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};
use wasmtime::{Caller, Val};

use super::memory::{arg, give};
use super::space::Space;
use super::{with_space, Record};

/// Win32's error codes, as `GetLastError` gives them
const ERROR_SUCCESS: u32 = 0;
const ERROR_FILE_NOT_FOUND: u32 = 2;
const ERROR_PATH_NOT_FOUND: u32 = 3;
const ERROR_ACCESS_DENIED: u32 = 5;
const ERROR_INVALID_HANDLE: u32 = 6;
const ERROR_NO_MORE_FILES: u32 = 18;
const ERROR_HANDLE_EOF: u32 = 38;
const ERROR_FILE_EXISTS: u32 = 80;
const ERROR_INVALID_PARAMETER: u32 = 87;
const ERROR_DISK_FULL: u32 = 112;
const ERROR_NEGATIVE_SEEK: u32 = 131;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const ERROR_DIR_NOT_EMPTY: u32 = 145;
const ERROR_ALREADY_EXISTS: u32 = 183;

/// `CreateFileA`'s access and dispositions
const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const CREATE_NEW: u32 = 1;
const CREATE_ALWAYS: u32 = 2;
const OPEN_EXISTING: u32 = 3;
const OPEN_ALWAYS: u32 = 4;
const TRUNCATE_EXISTING: u32 = 5;
/// `CreateFileA`'s and `FindFirstFileA`'s failure
const INVALID_HANDLE_VALUE: u32 = u32::MAX;
/// `SetFilePointer`'s methods, and its failure
const FILE_BEGIN: u32 = 0;
const FILE_CURRENT: u32 = 1;
const FILE_END: u32 = 2;
const INVALID_SET_FILE_POINTER: u32 = u32::MAX;

/// file attributes
const FILE_ATTRIBUTE_READONLY: u32 = 0x01;
const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_FILE_ATTRIBUTES: u32 = u32::MAX;

/// the FILETIME of the Unix epoch: 100 ns intervals since 1601
const UNIX_EPOCH_FILETIME: u64 = 116_444_736_000_000_000;
/// what `GetDiskFreeSpaceExA` reports, free and in all: the same on every
/// machine, and more than any save needs
const REPORTED_DISK: u64 = 8 << 30;

/// the longest path string read from the game, its NUL included (FATX's
/// paths are far shorter)
const MAX_PATH_STRING: usize = 1024;
/// `WIN32_FIND_DATAA` up to the end of `cFileName`, all that is written: the
/// Xbox's structure may end there
const FIND_DATA: u32 = 44 + 260;
/// the first handle, and the step between handles (as Windows numbers them)
/// `XGAME_FIND_DATA`: a `WIN32_FIND_DATAA`, then the save's directory
/// (`szSaveGameDirectory[MAX_PATH]`) and its name
/// (`WCHAR szSaveGameName[MAX_GAMENAME]`)
const SAVE_DIRECTORY_AT: u32 = 320;
const SAVE_NAME_AT: u32 = 580;
const GAME_FIND_DATA: u32 = SAVE_NAME_AT + MAX_GAMENAME * 2;
/// the longest saved game name, its NUL included (`MAX_GAMENAME`)
const MAX_GAMENAME: u32 = 128;
/// a saved game's folder name: twelve hexadecimal digits
const SAVE_FOLDER_DIGITS: usize = 12;
const SAVE_META: &str = "SaveMeta.xbx";

/// the longest nickname, its NUL included, and how many are kept
const MAX_NICKNAME: u32 = 32;
const NICKNAMES_KEPT: usize = 16;
const NICKNAMES: &str = "nicknames.txt";

const SIGNATURE_SIZE: usize = 20;

const FIRST_HANDLE: u32 = 0x100;
const HANDLE_STEP: u32 = 4;

/// a completion routine `ReadFileEx` or `WriteFileEx` queued, for the
/// game's next alertable wait
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Completion {
    /// the routine, an index in the game's function table
    pub routine: u32,
    /// its error code (`ERROR_HANDLE_EOF` for a read at the end)
    pub error: u32,
    /// the bytes read or written
    pub transferred: u32,
    /// the `OVERLAPPED` the call was given
    pub overlapped: u32,
    /// the game's thread that queued it, which alone runs it (as Windows
    /// runs an APC only on its own thread)
    pub thread: u32,
}

/// what a find found, as `WIN32_FIND_DATAA` gives it
struct Entry {
    name: String,
    attributes: u32,
    size: u64,
    times: [u64; 3],
}

/// a saved game a find found: its folder and its name
struct Save {
    folder: Entry,
    name: String,
}

/// what a handle is
enum Open {
    File(File),
    Find {
        entries: Vec<Entry>,
        next: usize,
    },
    SaveGames {
        root: String,
        saves: Vec<Save>,
        next: usize,
    },
    Nicknames,
    Signature(Box<Sha256>),
}

/// the game's files: its folders, its open handles and its last error
#[derive(Default)]
pub struct Files {
    data: Option<PathBuf>,
    saves: Option<PathBuf>,
    handles: BTreeMap<u32, Open>,
    next: u32,
    /// the game's thread calling now
    thread: u32,
    /// each thread's last error
    last_errors: BTreeMap<u32, u32>,
    completions: Vec<Completion>,
    /// the name each handle of a map read from another's reads in its
    /// header
    renamed: BTreeMap<u32, [u8; NAME_LEN]>,
    /// the console's init script, `d:\init.txt` (`halo_init_text`)
    init: Vec<u8>,
}

impl Files {
    /// Sets the data folder (`d:\`) and the save folder (the other drives).
    pub fn set_folders(&mut self, data: &Path, saves: &Path) {
        self.data = Some(data.to_path_buf());
        self.saves = Some(saves.to_path_buf());
    }

    /// Sets the console's init script, the commands `console_startup` runs
    /// as the game starts (`source/main/console.c`), one to a line: the
    /// original's own way to start on a map (`map_name`).
    pub fn set_init_commands(&mut self, text: &str) {
        self.init = text.as_bytes().to_vec();
    }

    /// `halo_init_text(buffer, size)`: the init script's length, its first
    /// `size` bytes written to `buffer` (none past the memory)
    fn init_text(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let size = usize::try_from(a[1]).unwrap_or(usize::MAX);
        if a[0] != 0 {
            data.write(
                a[0],
                self.init
                    .get(..size.min(self.init.len()))
                    .unwrap_or_default(),
            );
        }
        u32::try_from(self.init.len()).unwrap_or(u32::MAX)
    }

    /// The completion routines waiting for an alertable wait, oldest first.
    #[must_use]
    pub fn completions(&self) -> &[Completion] {
        &self.completions
    }

    /// Takes the completion routines waiting for `thread`, oldest first.
    pub fn take_completions(&mut self, thread: u32) -> Vec<Completion> {
        let (taken, left) = std::mem::take(&mut self.completions)
            .into_iter()
            .partition(|done| done.thread == thread);
        self.completions = left;
        taken
    }

    /// whether any completion routine waits for `thread`
    pub(crate) fn has_completions(&self, thread: u32) -> bool {
        self.completions.iter().any(|done| done.thread == thread)
    }

    /// sets the game's thread `thread`'s last error, as another module's
    /// import fails
    pub(crate) fn fail_for(&mut self, thread: u32, error: u32) {
        self.last_errors.insert(thread, error);
    }

    /// sets the last error and gives `value`
    fn fail(&mut self, error: u32, value: u32) -> u32 {
        self.last_errors.insert(self.thread, error);
        value
    }

    fn add(&mut self, open: Open) -> u32 {
        let handle = self.next.max(FIRST_HANDLE);
        self.next = handle.wrapping_add(HANDLE_STEP);
        self.handles.insert(handle, open);
        handle
    }

    fn file(&mut self, handle: u32) -> Option<&mut File> {
        match self.handles.get_mut(&handle) {
            Some(Open::File(file)) => Some(file),
            _ => None,
        }
    }

    /// the host path of the game's path at `address`, if it is a string and
    /// its drive has a folder
    fn path(&self, data: &Space<'_>, address: u32) -> Option<PathBuf> {
        self.resolve(&string(data, address)?)
    }

    fn resolve(&self, path: &str) -> Option<PathBuf> {
        self.locate(path).map(|(host, _)| host)
    }

    /// [`Files::resolve`], with the name in its header of a map read from
    /// another's (`DONORS`)
    fn locate(&self, path: &str) -> Option<(PathBuf, Option<&'static str>)> {
        let bytes = path.as_bytes();
        let (drive, rest) = match bytes {
            [letter, b':', ..] if letter.is_ascii_alphabetic() => {
                (letter.to_ascii_lowercase(), path.get(2..).unwrap_or(""))
            }
            _ => (b'd', path),
        };
        let mut host = if drive == b'd' {
            self.data.clone()?
        } else {
            let folder = self.saves.as_ref()?.join(char::from(drive).to_string());
            fs::create_dir_all(&folder).ok()?;
            folder
        };
        for component in rest.split(['\\', '/']) {
            if component.chars().all(|c| c == '.' || c == ' ') {
                continue;
            }
            let found = matching(&host, component);
            host.push(found.as_deref().unwrap_or(component));
        }
        if drive == b'd' && !host.exists() {
            if let Some((donor, name)) = donor_of(rest) {
                return Some((self.resolve(donor)?, Some(name)));
            }
        }
        Some((host, None))
    }

    fn create_file(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (access, disposition) = (a[1], a[4]);
        let Some((path, name)) = string(data, a[0]).and_then(|path| self.locate(&path)) else {
            return self.fail(ERROR_PATH_NOT_FOUND, INVALID_HANDLE_VALUE);
        };
        if path.is_dir() {
            return self.fail(ERROR_ACCESS_DENIED, INVALID_HANDLE_VALUE);
        }
        let existed = path.is_file();
        let mut options = OpenOptions::new();
        let write = access & GENERIC_WRITE != 0;
        options
            .read(access & GENERIC_READ != 0 || !write)
            .write(write);
        match disposition {
            CREATE_NEW => options.write(true).create_new(true),
            CREATE_ALWAYS => options.write(true).create(true).truncate(true),
            OPEN_EXISTING => &mut options,
            OPEN_ALWAYS => options.write(true).create(true),
            // (truncating a read-only open makes it read and write)
            TRUNCATE_EXISTING => options.write(true).truncate(true),
            _ => return self.fail(ERROR_INVALID_PARAMETER, INVALID_HANDLE_VALUE),
        };
        match options.open(&path) {
            Ok(file) => {
                let handle = self.add(Open::File(file));
                if let Some(name) = name {
                    self.renamed.insert(handle, header_name(name));
                }
                let replaced = existed && matches!(disposition, OPEN_ALWAYS | CREATE_ALWAYS);
                self.fail(
                    if replaced {
                        ERROR_ALREADY_EXISTS
                    } else {
                        ERROR_SUCCESS
                    },
                    handle,
                )
            }
            Err(error) => self.fail(code(&error, &path), INVALID_HANDLE_VALUE),
        }
    }

    /// `ReadFile` and `WriteFile`: at the file's position, or at the
    /// `OVERLAPPED`'s offset (leaving the position alone)
    fn transfer(&mut self, data: &mut Space<'_>, a: &[u32; 8], writing: bool) -> u32 {
        let (handle, buffer, count, done, overlapped) = (a[0], a[1], a[2], a[3], a[4]);
        let offset = (overlapped != 0)
            .then(|| offset(data, overlapped))
            .flatten();
        if overlapped != 0 && offset.is_none() {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        }
        let name = self.renamed.get(&handle).copied();
        let Some(file) = self.file(handle) else {
            return self.fail(ERROR_INVALID_HANDLE, 0);
        };
        let moved = if writing {
            let Some(buffer) = data.read(buffer, count) else {
                return self.fail(ERROR_INVALID_PARAMETER, 0);
            };
            match offset {
                Some(at) => file.write_all_at(&buffer, at),
                None => file.write_all(&buffer),
            }
            .map(|()| buffer.len())
        } else {
            let read = data.update(buffer, count, |buffer| {
                let at = match offset {
                    Some(at) => at,
                    None => file.stream_position()?,
                };
                let read = match offset {
                    Some(at) => read_at(file, buffer, at),
                    None => read_all(file, buffer),
                }?;
                rename(
                    buffer.get_mut(..read).unwrap_or_default(),
                    at,
                    name.as_ref(),
                );
                Ok(read)
            });
            let Some(read) = read else {
                return self.fail(ERROR_INVALID_PARAMETER, 0);
            };
            read
        };
        let moved = match moved {
            Ok(moved) => u32::try_from(moved).unwrap_or(count),
            Err(error) => return self.fail(code(&error, Path::new("")), 0),
        };
        if done != 0 {
            data.put(done, moved);
        }
        if overlapped != 0 {
            let error = if moved == 0 && count != 0 && !writing {
                ERROR_HANDLE_EOF
            } else {
                ERROR_SUCCESS
            };
            data.put(overlapped, error);
            data.put(overlapped + 4, moved);
            if error != ERROR_SUCCESS {
                return self.fail(error, 0);
            }
        }
        1
    }

    /// `ReadFileEx` and `WriteFileEx`: done at once, at the `OVERLAPPED`'s
    /// offset, with the completion routine queued
    fn transfer_ex(&mut self, data: &mut Space<'_>, a: &[u32; 8], writing: bool) -> u32 {
        let (handle, buffer, count, overlapped, routine) = (a[0], a[1], a[2], a[3], a[4]);
        let Some(at) = (overlapped != 0)
            .then(|| offset(data, overlapped))
            .flatten()
        else {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        };
        let name = self.renamed.get(&handle).copied();
        let Some(file) = self.file(handle) else {
            return self.fail(ERROR_INVALID_HANDLE, 0);
        };
        let moved = if writing {
            data.read(buffer, count)
                .map(|buffer| file.write_all_at(&buffer, at).map(|()| buffer.len()))
        } else {
            data.update(buffer, count, |buffer| {
                let read = read_at(file, buffer, at)?;
                rename(
                    buffer.get_mut(..read).unwrap_or_default(),
                    at,
                    name.as_ref(),
                );
                Ok(read)
            })
        };
        let Some(moved) = moved else {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        };
        let (error, moved) = match moved {
            Ok(0) if !writing && count != 0 => (ERROR_HANDLE_EOF, 0),
            Ok(moved) => (ERROR_SUCCESS, u32::try_from(moved).unwrap_or(count)),
            Err(error) => (code(&error, Path::new("")), 0),
        };
        data.put(overlapped, error);
        data.put(overlapped + 4, moved);
        self.completions.push(Completion {
            routine,
            error,
            transferred: moved,
            overlapped,
            thread: self.thread,
        });
        self.fail(ERROR_SUCCESS, 1)
    }

    fn set_file_pointer(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (handle, low, high_at, method) = (a[0], a[1], a[2], a[3]);
        let distance = if high_at == 0 {
            i64::from(low.cast_signed())
        } else {
            let Some(high) = data.get(high_at) else {
                return self.fail(ERROR_INVALID_PARAMETER, INVALID_SET_FILE_POINTER);
            };
            ((u64::from(high) << 32) | u64::from(low)).cast_signed()
        };
        let Some(file) = self.file(handle) else {
            return self.fail(ERROR_INVALID_HANDLE, INVALID_SET_FILE_POINTER);
        };
        let from = match method {
            FILE_BEGIN => match u64::try_from(distance) {
                Ok(at) => SeekFrom::Start(at),
                Err(_) => return self.fail(ERROR_NEGATIVE_SEEK, INVALID_SET_FILE_POINTER),
            },
            FILE_CURRENT => SeekFrom::Current(distance),
            FILE_END => SeekFrom::End(distance),
            _ => return self.fail(ERROR_INVALID_PARAMETER, INVALID_SET_FILE_POINTER),
        };
        match file.seek(from) {
            Ok(at) => {
                if high_at != 0 {
                    data.put(high_at, (at >> 32) as u32);
                }
                self.fail(ERROR_SUCCESS, at as u32)
            }
            Err(_) => self.fail(ERROR_NEGATIVE_SEEK, INVALID_SET_FILE_POINTER),
        }
    }

    fn get_file_size(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(file) = self.file(a[0]) else {
            return self.fail(ERROR_INVALID_HANDLE, u32::MAX);
        };
        match file.metadata() {
            Ok(metadata) => {
                if a[1] != 0 {
                    data.put(a[1], (metadata.len() >> 32) as u32);
                }
                metadata.len() as u32
            }
            Err(error) => self.fail(code(&error, Path::new("")), u32::MAX),
        }
    }

    fn set_end_of_file(&mut self, _: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(file) = self.file(a[0]) else {
            return self.fail(ERROR_INVALID_HANDLE, 0);
        };
        let cut = file.stream_position().and_then(|at| file.set_len(at));
        match cut {
            Ok(()) => 1,
            Err(error) => self.fail(code(&error, Path::new("")), 0),
        }
    }

    fn get_file_time(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(file) = self.file(a[0]) else {
            return self.fail(ERROR_INVALID_HANDLE, 0);
        };
        match file.metadata() {
            Ok(metadata) => {
                for (at, time) in [a[1], a[2], a[3]].into_iter().zip(times(&metadata)) {
                    if at != 0 {
                        data.put64(at, time);
                    }
                }
                1
            }
            Err(error) => self.fail(code(&error, Path::new("")), 0),
        }
    }

    /// `SetFileTime`: only the last write time, the one the game sets
    fn set_file_time(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let written = (a[3] != 0).then(|| data.get64(a[3])).flatten();
        let Some(file) = self.file(a[0]) else {
            return self.fail(ERROR_INVALID_HANDLE, 0);
        };
        match written.map(|time| file.set_modified(system_time(time))) {
            Some(Err(error)) => self.fail(code(&error, Path::new("")), 0),
            _ => 1,
        }
    }

    fn close_handle(&mut self, _: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        self.renamed.remove(&a[0]);
        match self.handles.remove(&a[0]) {
            Some(_) => 1,
            None => self.fail(ERROR_INVALID_HANDLE, 0),
        }
    }

    fn find_close(&mut self, _: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        if matches!(
            self.handles.get(&a[0]),
            Some(Open::Find { .. } | Open::SaveGames { .. } | Open::Nicknames)
        ) {
            self.handles.remove(&a[0]);
            return 1;
        }
        self.fail(ERROR_INVALID_HANDLE, 0)
    }

    fn get_last_error(&mut self, _: &mut Space<'_>, _: &[u32; 8]) -> u32 {
        self.last_errors.get(&self.thread).copied().unwrap_or(0)
    }

    fn set_last_error(&mut self, _: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        self.fail(a[0], 0)
    }

    fn get_file_attributes(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(path) = self.path(data, a[0]) else {
            return self.fail(ERROR_PATH_NOT_FOUND, INVALID_FILE_ATTRIBUTES);
        };
        match fs::metadata(&path) {
            Ok(metadata) => attributes(&metadata),
            Err(error) => self.fail(code(&error, &path), INVALID_FILE_ATTRIBUTES),
        }
    }

    /// `GetFileAttributesExA(name, GetFileExInfoStandard, data)`: a
    /// `WIN32_FILE_ATTRIBUTE_DATA`
    fn get_file_attributes_ex(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        if a[1] != 0 {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        }
        let Some(path) = self.path(data, a[0]) else {
            return self.fail(ERROR_PATH_NOT_FOUND, 0);
        };
        let metadata = match fs::metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) => return self.fail(code(&error, &path), 0),
        };
        let size = if metadata.is_dir() { 0 } else { metadata.len() };
        let mut out = [0; 36];
        out[0..4].copy_from_slice(&attributes(&metadata).to_le_bytes());
        for (index, time) in times(&metadata).into_iter().enumerate() {
            out[4 + index * 8..12 + index * 8].copy_from_slice(&time.to_le_bytes());
        }
        out[28..32].copy_from_slice(&((size >> 32) as u32).to_le_bytes());
        out[32..36].copy_from_slice(&(size as u32).to_le_bytes());
        if !data.write(a[2], &out) {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        }
        1
    }

    /// `SetFileAttributesA`: only the read-only bit has a meaning here
    fn set_file_attributes(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(path) = self.path(data, a[0]) else {
            return self.fail(ERROR_PATH_NOT_FOUND, 0);
        };
        let changed = fs::metadata(&path).and_then(|metadata| {
            let mut permissions = metadata.permissions();
            permissions.set_readonly(a[1] & FILE_ATTRIBUTE_READONLY != 0);
            fs::set_permissions(&path, permissions)
        });
        match changed {
            Ok(()) => 1,
            Err(error) => self.fail(code(&error, &path), 0),
        }
    }

    fn create_directory(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(path) = self.path(data, a[0]) else {
            return self.fail(ERROR_PATH_NOT_FOUND, 0);
        };
        match fs::create_dir(&path) {
            Ok(()) => 1,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                self.fail(ERROR_ALREADY_EXISTS, 0)
            }
            Err(error) => self.fail(code(&error, &path), 0),
        }
    }

    fn remove_directory(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(path) = self.path(data, a[0]) else {
            return self.fail(ERROR_PATH_NOT_FOUND, 0);
        };
        match fs::remove_dir(&path) {
            Ok(()) => 1,
            Err(error) => self.fail(code(&error, &path), 0),
        }
    }

    fn delete_file(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(path) = self.path(data, a[0]) else {
            return self.fail(ERROR_PATH_NOT_FOUND, 0);
        };
        if path.is_dir() {
            return self.fail(ERROR_ACCESS_DENIED, 0);
        }
        match fs::remove_file(&path) {
            Ok(()) => 1,
            Err(error) => self.fail(code(&error, &path), 0),
        }
    }

    /// `MoveFileA`, which never replaces what is there
    fn move_file(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (Some(from), Some(to)) = (self.path(data, a[0]), self.path(data, a[1])) else {
            return self.fail(ERROR_PATH_NOT_FOUND, 0);
        };
        if !from.exists() {
            return self.fail(code(&io::ErrorKind::NotFound.into(), &from), 0);
        }
        if to.exists() {
            return self.fail(ERROR_ALREADY_EXISTS, 0);
        }
        match fs::rename(&from, &to) {
            Ok(()) => 1,
            Err(error) => self.fail(code(&error, &to), 0),
        }
    }

    /// `CopyFileA(from, to, fail_if_exists)`
    fn copy_file(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (Some(from), Some(to)) = (self.path(data, a[0]), self.path(data, a[1])) else {
            return self.fail(ERROR_PATH_NOT_FOUND, 0);
        };
        if a[2] != 0 && to.exists() {
            return self.fail(ERROR_FILE_EXISTS, 0);
        }
        if !from.is_file() {
            return self.fail(code(&io::ErrorKind::NotFound.into(), &from), 0);
        }
        match fs::copy(&from, &to) {
            Ok(_) => 1,
            Err(error) => self.fail(code(&error, &to), 0),
        }
    }

    fn find_first_file(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(pattern) = string(data, a[0]) else {
            return self.fail(ERROR_INVALID_PARAMETER, INVALID_HANDLE_VALUE);
        };
        let split = pattern.rfind(['\\', '/']).map_or(0, |at| at + 1);
        let (folder, name) = pattern.split_at(split);
        let Some(folder) = self.resolve(folder) else {
            return self.fail(ERROR_PATH_NOT_FOUND, INVALID_HANDLE_VALUE);
        };
        let listing = match fs::read_dir(&folder) {
            Ok(listing) => listing,
            Err(_) => return self.fail(ERROR_PATH_NOT_FOUND, INVALID_HANDLE_VALUE),
        };
        let name = if name == "*.*" { "*" } else { name }.to_ascii_lowercase();
        let mut entries: Vec<Entry> = listing
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let found = entry.file_name().into_string().ok()?;
                let fits = found.len() < 260;
                (fits && wildcard(name.as_bytes(), found.to_ascii_lowercase().as_bytes()))
                    .then_some(())?;
                let metadata = fs::metadata(entry.path()).ok()?;
                Some(Entry {
                    attributes: attributes(&metadata),
                    size: if metadata.is_dir() { 0 } else { metadata.len() },
                    times: times(&metadata),
                    name: found,
                })
            })
            .collect();
        entries.sort_by(|a, b| {
            (a.name.to_ascii_lowercase(), &a.name).cmp(&(b.name.to_ascii_lowercase(), &b.name))
        });
        let Some(first) = entries.first() else {
            return self.fail(ERROR_FILE_NOT_FOUND, INVALID_HANDLE_VALUE);
        };
        if !find_data(data, a[1], first) {
            return self.fail(ERROR_INVALID_PARAMETER, INVALID_HANDLE_VALUE);
        }
        let handle = self.add(Open::Find { entries, next: 1 });
        self.fail(ERROR_SUCCESS, handle)
    }

    fn find_next_file(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(Open::Find { entries, next }) = self.handles.get_mut(&a[0]) else {
            return self.fail(ERROR_INVALID_HANDLE, 0);
        };
        let Some(entry) = entries.get(*next) else {
            return self.fail(ERROR_NO_MORE_FILES, 0);
        };
        if !find_data(data, a[1], entry) {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        }
        *next += 1;
        1
    }

    /// `GetDiskFreeSpaceExA(path, free_to_caller, total, total_free)`
    fn get_disk_free_space(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        for at in [a[1], a[2], a[3]] {
            if at != 0 {
                data.put64(at, REPORTED_DISK);
            }
        }
        1
    }

    /// `CompareFileTime(a, b)`: -1, 0 or 1
    fn compare_file_time(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (Some(first), Some(second)) = (data.get64(a[0]), data.get64(a[1])) else {
            return 0;
        };
        (first.cmp(&second) as i32).cast_unsigned()
    }

    /// `SystemTimeToFileTime(system, file)`
    fn system_time_to_file_time(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(system) = data.read(a[0], 16) else {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        };
        let mut parts = [0i64; 8];
        for (part, pair) in parts.iter_mut().zip(system.as_chunks::<2>().0) {
            *part = i64::from(u16::from_le_bytes(*pair));
        }
        let [year, month, _, day, hour, minute, second, millisecond] = parts;
        let valid = (1601..=30827).contains(&year)
            && (1..=12).contains(&month)
            && (1..=days_in_month(year, month)).contains(&day)
            && hour < 24
            && minute < 60
            && second < 60
            && millisecond < 1000;
        if !valid {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        }
        let days = days_from_civil(year, month, day) - days_from_civil(1601, 1, 1);
        let seconds = days * 86_400 + hour * 3_600 + minute * 60 + second;
        let time = (seconds * 1_000 + millisecond) * 10_000;
        if !data.put64(a[1], time.cast_unsigned()) {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        }
        1
    }

    /// `XCreateSaveGame(root, name, disposition, flags, path, size)`: the
    /// saved game's folder, as `root` and a `\`, made if asked for
    fn create_save_game(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (Some(root), Some(name)) = (string(data, a[0]), wide_string(data, a[1], MAX_GAMENAME))
        else {
            return ERROR_INVALID_PARAMETER;
        };
        let Some(folder) = self.resolve(&root).filter(|_| !name.is_empty()) else {
            return ERROR_INVALID_PARAMETER;
        };
        let root = with_separator(root);
        if root.len() + SAVE_FOLDER_DIGITS + 2 > a[5] as usize {
            return ERROR_INSUFFICIENT_BUFFER;
        }
        let found = saves(&folder).into_iter().find(|save| save.name == name);
        let save = match (found, a[2]) {
            (Some(_), CREATE_NEW) => return ERROR_ALREADY_EXISTS,
            (Some(save), _) => save.folder.name,
            (None, OPEN_EXISTING) => return ERROR_FILE_NOT_FOUND,
            (None, CREATE_NEW | OPEN_ALWAYS) => match new_save(&folder, &name) {
                Ok(save) => save,
                Err(error) => return code(&error, &folder),
            },
            (None, _) => return ERROR_INVALID_PARAMETER,
        };
        let path = format!("{root}{save}\\");
        if !put_string(data, a[4], &path) {
            return ERROR_INVALID_PARAMETER;
        }
        ERROR_SUCCESS
    }

    /// `XDeleteSaveGame(root, name)`: the folder and all in it
    fn delete_save_game(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (Some(root), Some(name)) = (string(data, a[0]), wide_string(data, a[1], MAX_GAMENAME))
        else {
            return ERROR_INVALID_PARAMETER;
        };
        let Some(folder) = self.resolve(&root) else {
            return ERROR_PATH_NOT_FOUND;
        };
        let Some(save) = saves(&folder).into_iter().find(|save| save.name == name) else {
            return ERROR_FILE_NOT_FOUND;
        };
        let path = folder.join(&save.folder.name);
        match fs::remove_dir_all(&path) {
            Ok(()) => ERROR_SUCCESS,
            Err(error) => code(&error, &path),
        }
    }

    fn find_first_save_game(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(root) = string(data, a[0]) else {
            return self.fail(ERROR_INVALID_PARAMETER, INVALID_HANDLE_VALUE);
        };
        let Some(folder) = self.resolve(&root) else {
            return self.fail(ERROR_PATH_NOT_FOUND, INVALID_HANDLE_VALUE);
        };
        let root = with_separator(root);
        let saves = saves(&folder);
        let Some(first) = saves.first() else {
            return self.fail(ERROR_FILE_NOT_FOUND, INVALID_HANDLE_VALUE);
        };
        if !game_find_data(data, a[1], &root, first) {
            return self.fail(ERROR_INVALID_PARAMETER, INVALID_HANDLE_VALUE);
        }
        let handle = self.add(Open::SaveGames {
            root,
            saves,
            next: 1,
        });
        self.fail(ERROR_SUCCESS, handle)
    }

    fn find_next_save_game(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(Open::SaveGames { root, saves, next }) = self.handles.get_mut(&a[0]) else {
            return self.fail(ERROR_INVALID_HANDLE, 0);
        };
        let Some(save) = saves.get(*next) else {
            return self.fail(ERROR_NO_MORE_FILES, 0);
        };
        if !game_find_data(data, a[1], root, save) {
            return self.fail(ERROR_INVALID_PARAMETER, 0);
        }
        *next += 1;
        1
    }

    fn signature_begin(&mut self, _: &mut Space<'_>, _: &[u32; 8]) -> u32 {
        self.add(Open::Signature(Box::default()))
    }

    /// `XCalculateSignatureUpdate(handle, data, size)`
    fn signature_update(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let given = data.read(a[1], a[2]);
        let Some(Open::Signature(hash)) = self.handles.get_mut(&a[0]) else {
            return ERROR_INVALID_HANDLE;
        };
        let Some(given) = given else {
            return ERROR_INVALID_PARAMETER;
        };
        hash.update(&given);
        ERROR_SUCCESS
    }

    /// `XCalculateSignatureEnd(handle, signature)`: the signature, and the
    /// handle freed
    fn signature_end(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        if !matches!(self.handles.get(&a[0]), Some(Open::Signature(_))) {
            return ERROR_INVALID_HANDLE;
        }
        let Some(Open::Signature(hash)) = self.handles.remove(&a[0]) else {
            return ERROR_INVALID_HANDLE;
        };
        let digest = hash.finalize();
        match digest.get(..SIGNATURE_SIZE) {
            Some(signature) if data.write(a[1], signature) => ERROR_SUCCESS,
            _ => ERROR_INVALID_PARAMETER,
        }
    }

    fn nicknames_file(&self) -> Option<PathBuf> {
        Some(self.saves.as_ref()?.join(NICKNAMES))
    }

    fn nicknames(&self) -> Vec<String> {
        self.nicknames_file()
            .and_then(|file| fs::read_to_string(file).ok())
            .map(|text| {
                text.lines()
                    .filter(|name| !name.is_empty())
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// `XSetNicknameW(name, roaming)`: the name first, kept
    fn set_nickname(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let (Some(name), Some(file)) =
            (wide_string(data, a[0], MAX_NICKNAME), self.nicknames_file())
        else {
            return 0;
        };
        let name: String = name.chars().filter(|c| !c.is_control()).collect();
        if name.is_empty() {
            return 0;
        }
        let mut names = self.nicknames();
        names.retain(|kept| *kept != name);
        names.insert(0, name);
        names.truncate(NICKNAMES_KEPT);
        let mut text = names.join("\n");
        text.push('\n');
        u32::from(fs::write(file, text).is_ok())
    }

    /// `XFindFirstNicknameW(roaming, name, size)`: the newest, cut to `size`
    /// units with its NUL
    fn find_first_nickname(&mut self, data: &mut Space<'_>, a: &[u32; 8]) -> u32 {
        let Some(name) = self.nicknames().into_iter().next() else {
            return self.fail(ERROR_NO_MORE_FILES, INVALID_HANDLE_VALUE);
        };
        if !put_wide(data, a[1], &name, a[2]) {
            return self.fail(ERROR_INVALID_PARAMETER, INVALID_HANDLE_VALUE);
        }
        let handle = self.add(Open::Nicknames);
        self.fail(ERROR_SUCCESS, handle)
    }
}

/// a file service: the game's files, its memory and the import's arguments
/// (as unsigned 32 bits) to the import's result
pub(crate) type Service = fn(&mut Files, &mut Space<'_>, &[u32; 8]) -> u32;

/// the file import named "module.name", if this module answers it
pub(crate) fn service(name: &str) -> Option<Service> {
    let service: Service = match name.strip_prefix("env.")? {
        "CreateFileA" => Files::create_file,
        "ReadFile" => |files, data, a| files.transfer(data, a, false),
        "WriteFile" => |files, data, a| files.transfer(data, a, true),
        "ReadFileEx" => |files, data, a| files.transfer_ex(data, a, false),
        "WriteFileEx" => |files, data, a| files.transfer_ex(data, a, true),
        "SetFilePointer" => Files::set_file_pointer,
        "GetFileSize" => Files::get_file_size,
        "SetEndOfFile" => Files::set_end_of_file,
        "GetFileTime" => Files::get_file_time,
        "SetFileTime" => Files::set_file_time,
        "CloseHandle" => Files::close_handle,
        "XFindClose" => Files::find_close,
        "GetLastError" => Files::get_last_error,
        "SetLastError" => Files::set_last_error,
        "GetFileAttributesA" => Files::get_file_attributes,
        "GetFileAttributesExA" => Files::get_file_attributes_ex,
        "SetFileAttributesA" => Files::set_file_attributes,
        "CreateDirectoryA" => Files::create_directory,
        "RemoveDirectoryA" => Files::remove_directory,
        "DeleteFileA" => Files::delete_file,
        "MoveFileA" => Files::move_file,
        "CopyFileA" => Files::copy_file,
        "FindFirstFileA" => Files::find_first_file,
        "FindNextFileA" => Files::find_next_file,
        "GetDiskFreeSpaceExA" => Files::get_disk_free_space,
        "CompareFileTime" => Files::compare_file_time,
        "SystemTimeToFileTime" => Files::system_time_to_file_time,
        "XCreateSaveGame" => Files::create_save_game,
        "XDeleteSaveGame" => Files::delete_save_game,
        "XFindFirstSaveGame" => Files::find_first_save_game,
        "XFindNextSaveGame" => Files::find_next_save_game,
        "XCalculateSignatureBegin" => Files::signature_begin,
        "XCalculateSignatureUpdate" => Files::signature_update,
        "XCalculateSignatureEnd" => Files::signature_end,
        "XSetNicknameW" => Files::set_nickname,
        "XFindFirstNicknameW" => Files::find_first_nickname,
        "halo_init_text" => Files::init_text,
        _ => return None,
    };
    Some(service)
}

/// answers an import with `service`, on the caller's memory and files
pub(crate) fn serve(
    caller: &mut Caller<'_, Record>,
    service: Service,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let mut args = [0; 8];
    for (index, slot) in args.iter_mut().enumerate() {
        *slot = arg(params, index);
    }
    let value = with_space(caller, |space, record| {
        let mut files = record.files();
        files.thread = record.thread;
        service(&mut files, space, &args)
    })?;
    give(results, value);
    Ok(())
}

/// the NUL-terminated string at `address`, if its NUL is inside the memory
/// and within [`MAX_PATH_STRING`]
fn string(data: &Space<'_>, address: u32) -> Option<String> {
    let rest = data.read_up_to(address, MAX_PATH_STRING as u32)?;
    let end = rest.iter().position(|&byte| byte == 0)?;
    Some(String::from_utf8_lossy(rest.get(..end)?).into_owned())
}

/// the name in `folder` that is `component`: itself if there, otherwise the
/// first in name order that differs only in case
fn matching(folder: &Path, component: &str) -> Option<String> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.eq_ignore_ascii_case(component))
        .collect();
    names.sort();
    names
        .iter()
        .find(|name| *name == component)
        .or_else(|| names.first())
        .cloned()
}

/// The map a map with no file of its own is read from: devgulch, the test
/// level, is Blood Gulch's map under its own name
/// (`engine/crates/content/src/devgulch.rs`), made from the player's map and
/// never a file of the repository. Each is read with its own name in its
/// header, which the original checks against the file's
/// (`cache_files_windows.c`); the rest is the donor's as it is.
const DONORS: [(&str, &str, &str); 1] =
    [(r"maps\devgulch.map", r"maps\bloodgulch.map", "devgulch")];

/// where a cache file's header keeps its name, and the field's length
/// (`struct cache_file_header`)
const NAME_AT: u64 = 0x20;
const NAME_LEN: usize = 0x20;

/// the donor of the data folder's path `rest` and the name its header is
/// read with, if it has one
fn donor_of(rest: &str) -> Option<(&'static str, &'static str)> {
    let rest = rest.trim_start_matches(['\\', '/']).replace('/', "\\");
    DONORS
        .iter()
        .find(|(path, _, _)| path.eq_ignore_ascii_case(&rest))
        .map(|&(_, donor, name)| (donor, name))
}

/// `name` as a header's name field, NUL-padded
fn header_name(name: &str) -> [u8; NAME_LEN] {
    let mut field = [0; NAME_LEN];
    for (to, from) in field.iter_mut().zip(name.bytes().take(NAME_LEN - 1)) {
        *to = from;
    }
    field
}

/// writes `name` over the header's name field in `read`, the bytes read
/// from the file at `at`
fn rename(read: &mut [u8], at: u64, name: Option<&[u8; NAME_LEN]>) {
    let Some(name) = name else {
        return;
    };
    for (place, &byte) in (NAME_AT..).zip(name) {
        let Some(index) = place
            .checked_sub(at)
            .and_then(|index| usize::try_from(index).ok())
        else {
            continue;
        };
        if let Some(to) = read.get_mut(index) {
            *to = byte;
        }
    }
}

/// `*` and `?` against a name, both lowercase
fn wildcard(pattern: &[u8], name: &[u8]) -> bool {
    match (pattern.split_first(), name.split_first()) {
        (None, None) => true,
        (Some((b'*', rest)), _) => {
            wildcard(rest, name)
                || name
                    .split_first()
                    .is_some_and(|(_, tail)| wildcard(pattern, tail))
        }
        (Some((b'?', rest)), Some((_, tail))) => wildcard(rest, tail),
        (Some((want, rest)), Some((have, tail))) => want == have && wildcard(rest, tail),
        _ => false,
    }
}

/// Win32's error for an I/O error on `path`: a missing path is a missing
/// file when its folder is there
fn code(error: &io::Error, path: &Path) -> u32 {
    match error.kind() {
        io::ErrorKind::NotFound => {
            if path.parent().is_some_and(Path::is_dir) {
                ERROR_FILE_NOT_FOUND
            } else {
                ERROR_PATH_NOT_FOUND
            }
        }
        io::ErrorKind::AlreadyExists => ERROR_FILE_EXISTS,
        io::ErrorKind::StorageFull => ERROR_DISK_FULL,
        io::ErrorKind::DirectoryNotEmpty => ERROR_DIR_NOT_EMPTY,
        io::ErrorKind::NotADirectory => ERROR_PATH_NOT_FOUND,
        _ => ERROR_ACCESS_DENIED,
    }
}

fn attributes(metadata: &fs::Metadata) -> u32 {
    if metadata.is_dir() {
        FILE_ATTRIBUTE_DIRECTORY
    } else if metadata.permissions().readonly() {
        FILE_ATTRIBUTE_READONLY
    } else {
        FILE_ATTRIBUTE_NORMAL
    }
}

/// creation, last access and last write, as FILETIMEs
fn times(metadata: &fs::Metadata) -> [u64; 3] {
    [metadata.created(), metadata.accessed(), metadata.modified()].map(|time| file_time(time.ok()))
}

fn file_time(time: Option<SystemTime>) -> u64 {
    let Some(time) = time else { return 0 };
    let intervals = |since: Duration| u64::try_from(since.as_nanos() / 100).unwrap_or(u64::MAX);
    match time.duration_since(UNIX_EPOCH) {
        Ok(since) => UNIX_EPOCH_FILETIME.saturating_add(intervals(since)),
        Err(before) => UNIX_EPOCH_FILETIME.saturating_sub(intervals(before.duration())),
    }
}

fn system_time(time: u64) -> SystemTime {
    let span = |intervals: u64| Duration::from_nanos(intervals.saturating_mul(100));
    if time >= UNIX_EPOCH_FILETIME {
        UNIX_EPOCH.checked_add(span(time - UNIX_EPOCH_FILETIME))
    } else {
        UNIX_EPOCH.checked_sub(span(UNIX_EPOCH_FILETIME - time))
    }
    .unwrap_or(UNIX_EPOCH)
}

/// days since 1970-01-01 of a proleptic Gregorian date
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let of_era = year - era * 400;
    let of_year = (153 * ((month + 9) % 12) + 2) / 5 + day - 1;
    era * 146_097 + of_era * 365 + of_era / 4 - of_era / 100 + of_year - 719_468
}

fn days_in_month(year: i64, month: i64) -> i64 {
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// an `OVERLAPPED`'s offset (`Offset`, `OffsetHigh`)
fn offset(data: &Space<'_>, overlapped: u32) -> Option<u64> {
    data.get64(overlapped.checked_add(8)?)
}

/// reads until `buffer` is full or the file ends
fn read_all(file: &mut File, buffer: &mut [u8]) -> io::Result<usize> {
    let mut done = 0;
    while let Some(rest) = buffer.get_mut(done..).filter(|rest| !rest.is_empty()) {
        match file.read(rest)? {
            0 => break,
            read => done += read,
        }
    }
    Ok(done)
}

/// [`read_all`] at `at`, leaving the file's position alone
fn read_at(file: &File, buffer: &mut [u8], at: u64) -> io::Result<usize> {
    let mut done = 0;
    while let Some(rest) = buffer.get_mut(done..).filter(|rest| !rest.is_empty()) {
        match file.read_at(rest, at + done as u64)? {
            0 => break,
            read => done += read,
        }
    }
    Ok(done)
}

/// writes `entry` as a `WIN32_FIND_DATAA` up to the end of `cFileName`
fn find_data(data: &mut Space<'_>, address: u32, entry: &Entry) -> bool {
    let mut out = [0; FIND_DATA as usize];
    out[0..4].copy_from_slice(&entry.attributes.to_le_bytes());
    for (index, time) in entry.times.iter().enumerate() {
        out[4 + index * 8..12 + index * 8].copy_from_slice(&time.to_le_bytes());
    }
    out[28..32].copy_from_slice(&((entry.size >> 32) as u32).to_le_bytes());
    out[32..36].copy_from_slice(&(entry.size as u32).to_le_bytes());
    out[44..44 + entry.name.len()].copy_from_slice(entry.name.as_bytes());
    data.write(address, &out)
}

/// the NUL-terminated UTF-16 string at `address`, if its NUL is inside the
/// memory and within `most` units
fn wide_string(data: &Space<'_>, address: u32, most: u32) -> Option<String> {
    let rest = data.read_up_to(address, most.saturating_mul(2))?;
    let units: Vec<u16> = rest
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect();
    let end = units.iter().position(|&unit| unit == 0)?;
    Some(String::from_utf16_lossy(units.get(..end)?))
}

/// writes `text` at `address` as a C string
fn put_string(data: &mut Space<'_>, address: u32, text: &str) -> bool {
    let mut out = text.as_bytes().to_vec();
    out.push(0);
    data.write(address, &out)
}

/// writes `text` at `address` as a UTF-16 string of at most `size` units,
/// its NUL included
fn put_wide(data: &mut Space<'_>, address: u32, text: &str, size: u32) -> bool {
    let mut units: Vec<u16> = text
        .encode_utf16()
        .take((size as usize).saturating_sub(1))
        .collect();
    if size == 0 {
        return false;
    }
    units.push(0);
    let out: Vec<u8> = units.into_iter().flat_map(u16::to_le_bytes).collect();
    data.write(address, &out)
}

/// `root` ending in one `\`
fn with_separator(mut root: String) -> String {
    if !root.ends_with(['\\', '/']) {
        root.push('\\');
    }
    root
}

/// the saved games in `folder`, in folder-name order
fn saves(folder: &Path) -> Vec<Save> {
    let Ok(listing) = fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut saves: Vec<Save> = listing
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let found = entry.file_name().into_string().ok()?;
            let numbered =
                found.len() == SAVE_FOLDER_DIGITS && found.bytes().all(|b| b.is_ascii_hexdigit());
            numbered.then_some(())?;
            let metadata = fs::metadata(entry.path())
                .ok()
                .filter(fs::Metadata::is_dir)?;
            let name = save_name(&fs::read(entry.path().join(SAVE_META)).ok()?)?;
            Some(Save {
                folder: Entry {
                    attributes: attributes(&metadata),
                    size: 0,
                    times: times(&metadata),
                    name: found,
                },
                name,
            })
        })
        .collect();
    saves.sort_by(|a, b| a.folder.name.cmp(&b.folder.name));
    saves
}

/// the `Name=` of a `SaveMeta.xbx`: UTF-16, a byte-order mark first or not
fn save_name(meta: &[u8]) -> Option<String> {
    let units: Vec<u16> = meta
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect();
    let text = String::from_utf16_lossy(units.strip_prefix(&[0xFEFF]).unwrap_or(&units));
    text.lines()
        .find_map(|line| line.trim_end_matches('\r').strip_prefix("Name="))
        .map(str::to_owned)
}

/// makes a saved game named `name` in `folder`, in the lowest free number
fn new_save(folder: &Path, name: &str) -> io::Result<String> {
    let mut number: u64 = 0;
    let save = loop {
        let save = format!("{number:0width$X}", width = SAVE_FOLDER_DIGITS);
        let path = folder.join(&save);
        match fs::create_dir(&path) {
            Ok(()) => break save,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => number += 1,
            Err(error) => return Err(error),
        }
    };
    let meta: Vec<u8> = "\u{FEFF}Name="
        .chars()
        .chain(name.chars())
        .chain("\r\n".chars())
        .collect::<String>()
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect();
    fs::write(folder.join(&save).join(SAVE_META), meta)?;
    Ok(save)
}

/// writes `save` as an `XGAME_FIND_DATA`, its directory under `root`
fn game_find_data(data: &mut Space<'_>, address: u32, root: &str, save: &Save) -> bool {
    // (the tail first: when it fits, the whole of it does)
    let Some(tail) = address.checked_add(FIND_DATA) else {
        return false;
    };
    if !data.fill(tail, GAME_FIND_DATA - FIND_DATA, 0) || !find_data(data, address, &save.folder) {
        return false;
    }
    put_string(
        data,
        address + SAVE_DIRECTORY_AT,
        &format!("{root}{}\\", save.folder.name),
    ) && put_wide(data, address + SAVE_NAME_AT, &save.name, MAX_GAMENAME)
}
