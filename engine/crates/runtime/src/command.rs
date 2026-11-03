//! A WASI command's arguments and standard input, which the reference
//! traces and the harness's tests are given (`opence-wasm module.wasm
//! args...`, [`crate::Game::set_command`]). The game itself is given none,
//! so it reads no arguments and finds its standard input at its end.

use std::sync::{MutexGuard, PoisonError};

use wasmtime::{Caller, Val};

use super::memory::{arg, give, Handler};
use super::{with_space, Record};

/// WASI's errno 8, EBADF
const EBADF: u32 = 8;
/// WASI's errno 21, EFAULT
const EFAULT: u32 = 21;
/// the size of an iovec: a pointer and a length
const IOVEC: u32 = 8;

/// what a command is given
#[derive(Default)]
pub(crate) struct Command {
    /// its arguments, each with its terminating NUL
    args: Vec<Vec<u8>>,
    /// its standard input, and how much of it has been read
    stdin: Vec<u8>,
    read: usize,
}

impl Command {
    /// a command run with `args` (its own name first) and `stdin`
    pub(crate) fn new(args: &[impl AsRef<[u8]>], stdin: Vec<u8>) -> Self {
        let args = args
            .iter()
            .map(|arg| {
                let mut arg = arg.as_ref().to_vec();
                arg.push(0);
                arg
            })
            .collect();
        Self {
            args,
            stdin,
            read: 0,
        }
    }

    /// the bytes of all the arguments' strings
    fn size(&self) -> usize {
        self.args.iter().map(Vec::len).sum()
    }
}

/// the handler for the import "module.name", if this module answers it
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("wasi_snapshot_preview1.")? {
        "args_sizes_get" => args_sizes_get,
        "args_get" => args_get,
        "fd_read" => fd_read,
        _ => return None,
    };
    Some(handler)
}

/// the command, held while the guard lives
fn command<'a>(record: &'a Record) -> MutexGuard<'a, Command> {
    record
        .shared
        .command
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// a count or size as the game's 32 bits, or u32::MAX if it has more
fn word(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

/// `args_sizes_get(count, size)`: how many arguments, and their strings'
/// bytes
fn args_sizes_get(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (count, size) = (arg(params, 0), arg(params, 1));
    let done = with_space(caller, |space, record| {
        let command = command(record);
        space.put(count, word(command.args.len())) && space.put(size, word(command.size()))
    })?;
    give(results, if done { 0 } else { EFAULT });
    Ok(())
}

/// `args_get(pointers, strings)`: the arguments' strings, one after
/// another at `strings`, and a pointer to each at `pointers`
fn args_get(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (pointers, strings) = (arg(params, 0), arg(params, 1));
    let done = with_space(caller, |space, record| {
        let command = command(record);
        let mut at = strings;
        for (index, text) in command.args.iter().enumerate() {
            let Some(pointer) = word(index)
                .checked_mul(4)
                .and_then(|offset| pointers.checked_add(offset))
            else {
                return false;
            };
            if !space.put(pointer, at) || !space.write(at, text) {
                return false;
            }
            let Some(next) = at.checked_add(word(text.len())) else {
                return false;
            };
            at = next;
        }
        true
    })?;
    give(results, if done { 0 } else { EFAULT });
    Ok(())
}

/// `fd_read(fd, iovs, count, read)`: standard input's next bytes into the
/// iovecs, as many as they hold, and how many; any other descriptor is
/// EBADF
fn fd_read(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (fd, iovs, count, read) = (
        arg(params, 0),
        arg(params, 1),
        arg(params, 2),
        arg(params, 3),
    );
    if fd != 0 {
        give(results, EBADF);
        return Ok(());
    }
    let done = with_space(caller, |space, record| {
        let mut command = command(record);
        let Some(vectors) = count
            .checked_mul(IOVEC)
            .and_then(|len| space.read(iovs, len))
        else {
            return false;
        };
        let mut total = 0usize;
        for &[b0, b1, b2, b3, l0, l1, l2, l3] in vectors.as_chunks::<8>().0 {
            let (buffer, len) = (
                u32::from_le_bytes([b0, b1, b2, b3]),
                u32::from_le_bytes([l0, l1, l2, l3]),
            );
            let start = command.read;
            let end = start
                .saturating_add(usize::try_from(len).unwrap_or(usize::MAX))
                .min(command.stdin.len());
            let Some(bytes) = command.stdin.get(start..end) else {
                break;
            };
            if !space.write(buffer, bytes) {
                return false;
            }
            total += bytes.len();
            command.read = end;
            if end == command.stdin.len() {
                break;
            }
        }
        space.put(read, word(total))
    })?;
    give(results, if done { 0 } else { EFAULT });
    Ok(())
}
