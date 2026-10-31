//! The Metal device's render batches. The game's Direct3D is the Metal
//! device compiled into game.wasm (`port/wasmtime/metal/d3d8_metal_device.c`),
//! which writes its commands into a batch in its own memory
//! (`port/wasmtime/include/opence_render.h`) and rings `host_render_submit`.
//! Each batch is copied out, checked by one [`RenderValidator`] that
//! keeps the objects of the batches before it, and what passed is handed
//! to the app's renderer ([`RenderSink`]) with a read-only view of the
//! memory its uploads name, then to the plugins at the render hook. The
//! answer is how many commands were refused, or -1 for a batch refused
//! whole, as the VM's host answered (its `host_render.c`, now removed).
//!
//! The renderer answers with the textures it read back ([`Readback`]),
//! each written into the game's memory only where a ReadTexture or a
//! ReadBuffer of the batch names ([`readback_fits`]); one that does not fit is refused and
//! logged with the rest.

use std::sync::{Arc, PoisonError};

use opence_render::{BatchError, Command, DecodedBatch, RenderValidator};
use wasmtime::{Caller, Val};

use super::memory::{arg, give, Handler};
use super::plugins::{self, Event, View};
use super::{with_space, Record};

/// the drawable until the app sets it: the Xbox's screen
const XBOX: (u32, u32) = (640, 480);

/// the refusals kept for [`crate::Game::render_refusals`], as the VM's
/// host logs its first (`HOST_RENDER_LOGGED_REFUSALS`)
const REFUSALS_KEPT: usize = 32;

/// The app's renderer: each batch the game submits, as checked.
pub trait RenderSink: Send {
    /// Carries out `batch`'s commands that passed; `memory` is the game's,
    /// read only, for the bytes its uploads name (already checked to be
    /// inside it). Answers the textures its ReadTextures read back.
    fn submit(&mut self, batch: &DecodedBatch, memory: &View<'_>) -> Vec<Readback>;
}

/// Bytes the renderer read back from a texture, for the game's memory at
/// `address`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Readback {
    /// where in the game's memory
    pub address: u32,
    /// the bytes
    pub bytes: Vec<u8>,
}

/// Whether `readback` lies inside the range one of `batch`'s ReadTextures
/// or ReadBuffers names, so that the renderer writes nothing the game did not ask for.
#[must_use]
pub fn readback_fits(batch: &DecodedBatch, readback: &Readback) -> bool {
    let start = u64::from(readback.address);
    let end = start + u64::try_from(readback.bytes.len()).unwrap_or(u64::MAX);
    batch.commands.iter().any(|command| match command {
        Command::ReadTexture {
            address, length, ..
        }
        | Command::ReadBuffer {
            address, length, ..
        } => {
            let named = u64::from(*address);
            start >= named && end <= named + u64::from(*length)
        }
        _ => false,
    })
}

/// The bytes `batch`'s uploads, libraries and buffer writes name, one after
/// another in the order of its commands, as the renderer reads them with
/// each command's length; `None` if any is outside `memory` (the validator
/// has checked them, so only a memory that shrank would be).
#[must_use]
pub fn payload(batch: &DecodedBatch, memory: &View<'_>) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    for command in &batch.commands {
        let (address, length) = match command {
            Command::UploadBuffer {
                address, length, ..
            }
            | Command::UploadTexture {
                address, length, ..
            }
            | Command::CreateLibrary {
                address, length, ..
            }
            | Command::WriteBuffer {
                address, length, ..
            } => (*address, *length),
            _ => continue,
        };
        bytes.extend(memory.bytes(address, length)?);
    }
    Some(bytes)
}

/// How many batches and commands the game has submitted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderCounts {
    /// batches checked and handed on
    pub batches: u64,
    /// their commands that passed
    pub commands: u64,
    /// their commands refused
    pub refused: u64,
    /// batches refused whole
    pub refused_batches: u64,
}

/// the batches' state, shared by the game's threads
pub(crate) struct Batches {
    validator: RenderValidator,
    sink: Option<Box<dyn RenderSink>>,
    drawable: (u32, u32),
    counts: RenderCounts,
    refusals: Vec<String>,
}

impl Default for Batches {
    fn default() -> Self {
        let mut validator = RenderValidator::new();
        validator.set_drawable_size(XBOX.0, XBOX.1);
        Self {
            validator,
            sink: None,
            drawable: XBOX,
            counts: RenderCounts::default(),
            refusals: Vec::new(),
        }
    }
}

impl Batches {
    /// installs the app's renderer, in place of any before it
    pub(crate) fn set_sink(&mut self, sink: Box<dyn RenderSink>) {
        self.sink = Some(sink);
    }

    /// the drawable's size in pixels, as the device asks it
    pub(crate) fn set_drawable_size(&mut self, width: u32, height: u32) {
        self.drawable = (width, height);
        self.validator.set_drawable_size(width, height);
    }

    /// the drawable's size in pixels
    pub(crate) fn drawable(&self) -> (u32, u32) {
        self.drawable
    }

    pub(crate) fn counts(&self) -> RenderCounts {
        self.counts
    }

    pub(crate) fn refusals(&self) -> Vec<String> {
        self.refusals.clone()
    }

    /// keeps `reason` while fewer than [`REFUSALS_KEPT`] are
    fn refused(&mut self, reason: String) {
        if self.refusals.len() < REFUSALS_KEPT {
            self.refusals.push(reason);
        }
    }
}

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "host_render_submit" => submit,
        "host_drawable_width" => width,
        "host_drawable_height" => height,
        _ => return None,
    };
    Some(handler)
}

/// `host_drawable_width()`
fn width(caller: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, caller.data().batches().drawable.0);
    Ok(())
}

/// `host_drawable_height()`
fn height(caller: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, caller.data().batches().drawable.1);
    Ok(())
}

/// `host_render_submit(batch, size)`: the batch's refused commands, or -1
fn submit(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, size) = (arg(params, 0), arg(params, 1));
    let shared = Arc::clone(&caller.data().shared);
    let decoded = with_space(caller, |space, _| {
        let mut batches = shared
            .batches
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let decoded = match space.read(address, size) {
            Some(bytes) => {
                let memory = u64::try_from(space.len()).unwrap_or(u64::MAX);
                batches.validator.decode(&bytes, &memory)
            }
            None => Err(BatchError::Guest {
                reason: format!("{size} bytes at {address:#010x} are outside the memory"),
            }),
        };
        match decoded {
            Ok(batch) => {
                let refused = u64::try_from(batch.rejected.len()).unwrap_or(u64::MAX);
                batches.counts.batches += 1;
                batches.counts.commands += u64::try_from(batch.commands.len()).unwrap_or(u64::MAX);
                batches.counts.refused += refused;
                for rejection in &batch.rejected {
                    let reason = format!(
                        "render command refused at byte {} (op {}): {}",
                        rejection.offset, rejection.op, rejection.reason
                    );
                    batches.refused(reason);
                }
                let readbacks = batches
                    .sink
                    .as_mut()
                    .map(|sink| sink.submit(&batch, &View::of(space)))
                    .unwrap_or_default();
                for readback in readbacks {
                    if !(readback_fits(&batch, &readback)
                        && space.write(readback.address, &readback.bytes))
                    {
                        batches.refused(format!(
                            "render read back refused: {} bytes at {:#010x} that no ReadTexture names",
                            readback.bytes.len(),
                            readback.address
                        ));
                    }
                }
                Some(batch)
            }
            Err(error) => {
                batches.counts.refused_batches += 1;
                batches.refused(format!(
                    "render batch refused: {size} bytes at {address:#010x}: {error}"
                ));
                None
            }
        }
    })?;
    let Some(batch) = decoded else {
        give(results, u32::MAX);
        return Ok(());
    };
    let refused = u32::try_from(batch.rejected.len()).unwrap_or(u32::MAX);
    if batch.commands.contains(&Command::Present) {
        shared.threads.frame_shown(|| {
            shared
                .settings
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .boolean("debug.frame_clock")
                .unwrap_or(false)
        });
    }
    fire(caller, &batch.commands)?;
    give(results, refused);
    Ok(())
}

/// the render hook, with the commands that passed
fn fire(caller: &mut Caller<'_, Record>, commands: &[Command]) -> wasmtime::Result<()> {
    plugins::fire(caller, &Event::Render(commands))
}
