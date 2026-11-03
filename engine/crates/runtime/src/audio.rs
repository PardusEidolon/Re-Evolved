//! The game's mixed sound on its way to the speakers. The mixer is the
//! Linux port's (`port/linux/src/dsound_sdl.c`), compiled into the game: a
//! thread of the game's mixes and hands its frames, 48 kHz stereo floats,
//! to `halo_audio_output`, which keeps them in a ring and, once the ring
//! is below its target, answers how many more to mix. The app pulls them
//! from the ring on Core Audio's thread ([`Speakers`]), as the macOS VM
//! port's host pulled them from its guest (its `host_audio.c`, now removed).
//!
//! While nothing pulls (headless, or the device stopped), the ring drains
//! at the output's rate by the clock, as the Linux port's silent clock
//! thread mixed without a device, so the game's streams still play out in
//! time.

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::time::{Duration, Instant};

use wasmtime::{Caller, Val};

use super::memory::{arg, give, Handler};
use super::plugins::{self, Event};
use super::{with_space, Record, Shared};

/// The output's rate, a second.
pub const SPEAKER_RATE: u32 = 48_000;
/// The frames the ring holds ahead of the speakers: 32 ms, the macOS VM
/// port's (at least 30 ms, and 1536 frames).
pub const SPEAKER_TARGET_FRAMES: u32 = 1536;
/// the output's channels
const CHANNELS: usize = 2;
/// how long the speakers may go without a pull before the clock drains
/// the ring in their place
const STALLED: Duration = Duration::from_millis(100);
/// how often a waiting mixer looks at the clock
const TICK: Duration = Duration::from_millis(5);

/// the ring of frames, and what drains it
pub(crate) struct Ring {
    /// the samples, interleaved
    samples: VecDeque<f32>,
    /// when the speakers last pulled
    pulled_at: Option<Instant>,
    /// when the clock last drained the ring
    drained_at: Instant,
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            samples: VecDeque::new(),
            pulled_at: None,
            drained_at: Instant::now(),
        }
    }
}

impl Ring {
    fn frames(&self) -> u32 {
        u32::try_from(self.samples.len() / CHANNELS).unwrap_or(u32::MAX)
    }

    /// the frames that would have played since the clock last drained,
    /// dropped, while the speakers are not pulling
    fn drain_by_clock(&mut self, now: Instant) {
        let pulling = self
            .pulled_at
            .is_some_and(|at| now.duration_since(at) < STALLED);
        let elapsed = now.duration_since(self.drained_at);
        let frames = elapsed.as_micros() * u128::from(SPEAKER_RATE) / 1_000_000;
        if frames == 0 {
            return;
        }
        if !pulling {
            let samples = usize::try_from(frames)
                .unwrap_or(usize::MAX)
                .saturating_mul(CHANNELS)
                .min(self.samples.len());
            self.samples.drain(..samples);
        }
        self.drained_at = now;
    }
}

/// the ring, and the condition variable a pull wakes the mixer with
#[derive(Default)]
pub(crate) struct Output {
    ring: Mutex<Ring>,
    pulled: Condvar,
}

/// The speakers of a running game, to pull its mixed sound from on the
/// audio device's thread; each clone is the same speakers.
#[derive(Clone)]
pub struct Speakers {
    shared: Arc<Shared>,
}

impl Speakers {
    pub(crate) fn new(shared: Arc<Shared>) -> Self {
        Self { shared }
    }

    /// Fills `buffer` (interleaved stereo) with the frames the game has
    /// mixed, the rest with silence; the frames that were the game's.
    pub fn pull(&self, buffer: &mut [f32]) -> usize {
        let output = &self.shared.audio;
        let taken = {
            let mut ring = output.ring.lock().unwrap_or_else(PoisonError::into_inner);
            ring.pulled_at = Some(Instant::now());
            let taken = (buffer.len() / CHANNELS * CHANNELS).min(ring.samples.len());
            for (slot, sample) in buffer.iter_mut().zip(ring.samples.drain(..taken)) {
                *slot = sample;
            }
            taken
        };
        if let Some(rest) = buffer.get_mut(taken..) {
            rest.fill(0.0);
        }
        output.pulled.notify_all();
        taken / CHANNELS
    }
}

/// the audio import named "module.name", if this module answers it
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "halo_audio_output" => audio_output,
        _ => return None,
    };
    Some(handler)
}

/// `halo_audio_output(frames, count)`: keeps the `count` stereo frames at
/// `frames`, waits until the ring is below its target, and answers how
/// many frames would fill it again
fn audio_output(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (address, count) = (arg(params, 0), arg(params, 1));
    let len = count
        .checked_mul(u32::try_from(CHANNELS * size_of::<f32>()).unwrap_or(u32::MAX))
        .ok_or_else(|| wasmtime::format_err!("{count} frames of sound are too many"))?;
    let bytes = with_space(caller, |space, _| space.read(address, len))?.ok_or_else(|| {
        wasmtime::format_err!("the sound's frames at {address:#x} are outside the memory")
    })?;
    let samples: Vec<f32> = bytes
        .as_chunks::<{ size_of::<f32>() }>()
        .0
        .iter()
        .map(|sample| f32::from_le_bytes(*sample))
        .collect();
    plugins::fire(caller, &Event::Sound(&samples))?;
    let shared = Arc::clone(&caller.data().shared);
    let output = &shared.audio;
    let mut ring = output.ring.lock().unwrap_or_else(PoisonError::into_inner);
    ring.drain_by_clock(Instant::now());
    ring.samples.extend(samples);
    while ring.frames() >= SPEAKER_TARGET_FRAMES {
        ring = output
            .pulled
            .wait_timeout(ring, TICK)
            .unwrap_or_else(PoisonError::into_inner)
            .0;
        ring.drain_by_clock(Instant::now());
    }
    give(results, SPEAKER_TARGET_FRAMES - ring.frames());
    Ok(())
}
