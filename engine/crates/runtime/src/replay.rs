//! Recorded inputs: what the game's main thread asks the world outside it,
//! recorded as it is answered, so that a run can be made again exactly.
//!
//! The answers recorded are those that change from run to run: the clocks
//! (`GetTickCount`, `QueryPerformanceCounter` and the rest), the keyboard
//! and mouse (every import [`crate::input`] answers), the files' times
//! (`GetFileTime`), the network's random bytes (`XNetRandom`,
//! `XNetCreateKey`) and the mixer's pace (`halo_audio_output`, which waits
//! on the speakers). Each answer is the import's results and every byte it
//! wrote to the game's memory. A replayed run is handed the recorded
//! answers in order, without running the imports, so it reads the same
//! clocks and the same keys whatever they say now, and ends in the same
//! state; an import asked for out of order ends it with
//! [`crate::Error::Diverged`]. Once the recording runs out the game runs
//! live again.
//!
//! Its limits: only the main thread (the game's first) is recorded, and the
//! game's worker threads run live; an import that calls back into the game
//! (the keystroke's query of the console) is recorded as one answer, and on
//! replay the game is not asked; the network's sockets are not replayed;
//! and sound is not played on replay, so the sound hook is not called.
//!
//! A recording's bytes ([`Recording::to_bytes`]) are untrusted when they are
//! read back: [`Recording::from_bytes`] checks every count and length
//! against what is left.

use wasmtime::{Val, V128};

use crate::{Error, Value};

/// a recording's first four bytes
const MAGIC: &[u8; 4] = b"OCER";
/// the version of its layout
const VERSION: u32 = 1;

/// Whether the import "module.name" is recorded and replayed.
#[must_use]
pub fn recorded(name: &str) -> bool {
    matches!(
        name,
        "wasi_snapshot_preview1.clock_time_get"
            | "env.GetTickCount"
            | "env.QueryPerformanceCounter"
            | "env.QueryPerformanceFrequency"
            | "env.GetSystemTime"
            | "env.GetFileTime"
            | "env.XNetRandom"
            | "env.XNetCreateKey"
            | "env.halo_audio_output"
    ) || crate::input::handler(name).is_some()
}

/// One answer: the import asked, its results and what it wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    /// the import, as "module.name"
    pub import: String,
    /// its results
    pub results: Vec<Value>,
    /// each write to the game's memory, in order: its address and bytes
    pub writes: Vec<(u32, Vec<u8>)>,
}

/// The answers of a recorded run, in the order they were asked for.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Recording {
    /// the answers
    pub answers: Vec<Answer>,
}

impl Recording {
    /// How many answers it holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.answers.len()
    }

    /// Whether it holds none.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.answers.is_empty()
    }

    /// Its bytes: "OCER", its version and its answers' count, then each
    /// answer's import (a u16 length and its name), results (a u8 count,
    /// each a tag and its bits) and writes (a u32 count, each an address, a
    /// length and its bytes), all little-endian.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = MAGIC.to_vec();
        bytes.extend(VERSION.to_le_bytes());
        bytes.extend(count(self.answers.len()).to_le_bytes());
        for answer in &self.answers {
            let name = answer.import.as_bytes();
            let name = name.get(..usize::from(u16::MAX)).unwrap_or(name);
            bytes.extend(u16::try_from(name.len()).unwrap_or(u16::MAX).to_le_bytes());
            bytes.extend(name);
            let results = answer
                .results
                .get(..usize::from(u8::MAX))
                .unwrap_or(&answer.results);
            bytes.push(u8::try_from(results.len()).unwrap_or(u8::MAX));
            for result in results {
                match result {
                    Value::I32(value) => {
                        bytes.push(0);
                        bytes.extend(i64::from(*value).to_le_bytes());
                    }
                    Value::I64(value) => {
                        bytes.push(1);
                        bytes.extend(value.to_le_bytes());
                    }
                    Value::F32(bits) => {
                        bytes.push(2);
                        bytes.extend(u64::from(*bits).to_le_bytes());
                    }
                    Value::F64(bits) => {
                        bytes.push(3);
                        bytes.extend(bits.to_le_bytes());
                    }
                    Value::V128(bits) => {
                        bytes.push(4);
                        bytes.extend(bits.to_le_bytes());
                    }
                    Value::Ref => bytes.push(5),
                }
            }
            bytes.extend(count(answer.writes.len()).to_le_bytes());
            for (address, written) in &answer.writes {
                bytes.extend(address.to_le_bytes());
                bytes.extend(count(written.len()).to_le_bytes());
                bytes.extend(written);
            }
        }
        bytes
    }

    /// The recording `bytes` hold, as [`Recording::to_bytes`] wrote it.
    ///
    /// # Errors
    /// [`Error::Recording`] if they are not one: another magic or version,
    /// a count or length past their end, a name that is not UTF-8, an
    /// unknown tag, or bytes left over.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader { bytes };
        if reader.take(4)? != MAGIC {
            return Err(Error::Recording("not a recording".into()));
        }
        let version = reader.u32()?;
        if version != VERSION {
            return Err(Error::Recording(format!("version {version}")));
        }
        let answers = reader.u32()?;
        let mut recording = Self::default();
        for _ in 0..answers {
            let name = reader.u16()?;
            let import = std::str::from_utf8(reader.take(usize::from(name))?)
                .map_err(|_| Error::Recording("a name that is not UTF-8".into()))?
                .to_string();
            let mut results = Vec::new();
            for _ in 0..reader.u8()? {
                results.push(match reader.u8()? {
                    0 => Value::I32(
                        i32::try_from(reader.i64()?)
                            .map_err(|_| Error::Recording("an i32 out of range".into()))?,
                    ),
                    1 => Value::I64(reader.i64()?),
                    2 => Value::F32(
                        u32::try_from(reader.u64()?)
                            .map_err(|_| Error::Recording("an f32 out of range".into()))?,
                    ),
                    3 => Value::F64(reader.u64()?),
                    4 => Value::V128(u128::from_le_bytes(reader.array()?)),
                    5 => Value::Ref,
                    tag => return Err(Error::Recording(format!("the tag {tag}"))),
                });
            }
            let mut writes = Vec::new();
            for _ in 0..reader.u32()? {
                let address = reader.u32()?;
                let len = reader.u32()?;
                let written = reader.take(usize::try_from(len).unwrap_or(usize::MAX))?;
                writes.push((address, written.to_vec()));
            }
            recording.answers.push(Answer {
                import,
                results,
                writes,
            });
        }
        if reader.bytes.is_empty() {
            Ok(recording)
        } else {
            Err(Error::Recording(format!(
                "{} bytes after its end",
                reader.bytes.len()
            )))
        }
    }
}

/// a count as a recording keeps it
fn count(len: usize) -> u32 {
    u32::try_from(len).unwrap_or(u32::MAX)
}

/// a recording's bytes, read from the front, every read checked
struct Reader<'a> {
    bytes: &'a [u8],
}

impl<'a> Reader<'a> {
    fn take(&mut self, len: usize) -> Result<&'a [u8], Error> {
        if len > self.bytes.len() {
            return Err(Error::Recording("cut short".into()));
        }
        let (taken, rest) = self.bytes.split_at(len);
        self.bytes = rest;
        Ok(taken)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        <[u8; N]>::try_from(self.take(N)?).map_err(|_| Error::Recording("cut short".into()))
    }

    fn u8(&mut self) -> Result<u8, Error> {
        Ok(u8::from_le_bytes(self.array()?))
    }

    fn u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_le_bytes(self.array()?))
    }

    fn u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_le_bytes(self.array()?))
    }

    fn u64(&mut self) -> Result<u64, Error> {
        Ok(u64::from_le_bytes(self.array()?))
    }

    fn i64(&mut self) -> Result<i64, Error> {
        Ok(i64::from_le_bytes(self.array()?))
    }
}

/// what the main thread's imports are doing with a recording
#[derive(Debug, Default)]
pub(crate) enum Tape {
    /// nothing: the imports answer live
    #[default]
    Off,
    /// each recorded import's answer is kept
    Recording(Recording),
    /// each recorded import is answered from the recording, from `next`
    Replaying {
        /// the recording
        recording: Recording,
        /// the next answer to hand out
        next: usize,
    },
}

impl Tape {
    /// the answers a replay has still to hand out
    pub(crate) fn left(&self) -> usize {
        match self {
            Self::Replaying { recording, next } => recording.len().saturating_sub(*next),
            _ => 0,
        }
    }
}

/// a recorded result as the game is handed it, in the type it expects
pub(crate) fn val(value: &Value, expected: &Val) -> Option<Val> {
    Some(match (value, expected) {
        (Value::I32(value), Val::I32(_)) => Val::I32(*value),
        (Value::I64(value), Val::I64(_)) => Val::I64(*value),
        (Value::F32(bits), Val::F32(_)) => Val::F32(*bits),
        (Value::F64(bits), Val::F64(_)) => Val::F64(*bits),
        (Value::V128(bits), Val::V128(_)) => Val::V128(V128::from(*bits)),
        // (a reference is not recorded: the game is handed its null)
        (
            Value::Ref,
            expected @ (Val::FuncRef(_)
            | Val::ExternRef(_)
            | Val::AnyRef(_)
            | Val::ExnRef(_)
            | Val::ContRef(_)),
        ) => *expected,
        _ => return None,
    })
}

/// the answer the replay is at diverging from what was asked, carried out
/// of the game as a trap
#[derive(Debug)]
pub(crate) struct Diverged {
    /// the answer's place in the recording
    pub(crate) at: usize,
    /// the import it answers
    pub(crate) recorded: String,
    /// the import asked for
    pub(crate) asked: String,
}

impl std::fmt::Display for Diverged {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "answer {} answers {}, not {}",
            self.at, self.recorded, self.asked
        )
    }
}

impl std::error::Error for Diverged {}
