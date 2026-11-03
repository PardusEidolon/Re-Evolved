//! The plugin hooks: the moments the game's own code already marks, where
//! the runtime hands each plugin the event and a read-only view of the
//! game's memory. No line of `source/` changes for them: the tick, AI, HUD,
//! UI and frame hooks follow the exits of the game's profile sections
//! ([`crate::profile`]'s imports, built into game.wasm by
//! `port/wasmtime/include/profile_sections.h`), the render hook follows
//! each batch the Metal device submits ([`crate::batches`]), with its
//! commands, and the sound hook follows
//! the mixer's `halo_audio_output` with its samples.
//!
//! A plugin is called only for the hooks it asks for, in the order the
//! plugins were added, after the game's span for the section has ended, so
//! a plugin's time is never counted as the game's. While no plugin is
//! added a hook costs one atomic load.
//!
//! Each plugin's time is logged per frame (a frame ends at the frame hook,
//! [`Hook::Frame`]), with its share of the frame ([`Plugin::share`]);
//! [`over_share`] names the plugins over theirs, which a test fails on.
//!
//! The view reads the game's memory only through checked ranges and never
//! writes it: whatever a plugin reads there is untrusted. Its readers of
//! the game's own structures are in [`crate::views`].

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use wasmtime::Caller;

use super::space::Space;
use super::{with_space, Record};

/// A moment a plugin can ask to be called at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Hook {
    /// the end of a game tick (its last section, "game_tick.player_effect")
    Tick,
    /// the end of the tick's AI ("game_tick.ai")
    Ai,
    /// the end of the tick's HUD ("game_tick.hud")
    Hud,
    /// the end of the main loop's UI update ("ui_update")
    Ui,
    /// the end of each pass of the main loop ("throttle"), menus included
    Frame,
    /// a render batch submitted, with its commands
    Render,
    /// sound mixed, with its samples
    Sound,
}

impl Hook {
    /// every hook, in the order above
    pub const ALL: [Self; 7] = [
        Self::Tick,
        Self::Ai,
        Self::Hud,
        Self::Ui,
        Self::Frame,
        Self::Render,
        Self::Sound,
    ];

    /// the hook that follows the exit of the profile section `name`, if any
    pub(crate) fn after_section(name: &str) -> Option<Self> {
        Some(match name {
            "game_tick.player_effect" => Self::Tick,
            "game_tick.ai" => Self::Ai,
            "game_tick.hud" => Self::Hud,
            "ui_update" => Self::Ui,
            "throttle" => Self::Frame,
            _ => return None,
        })
    }
}

/// What a plugin is handed: the hook, and what it carries.
#[derive(Debug, Clone, Copy)]
pub enum Event<'a> {
    /// [`Hook::Tick`]
    Tick,
    /// [`Hook::Ai`]
    Ai,
    /// [`Hook::Hud`]
    Hud,
    /// [`Hook::Ui`]
    Ui,
    /// [`Hook::Frame`]
    Frame,
    /// [`Hook::Render`]: a render batch's commands that passed, after the
    /// app's renderer was handed them
    Render(&'a [opence_render::Command]),
    /// [`Hook::Sound`]: the frames mixed, interleaved stereo at 48 kHz
    Sound(&'a [f32]),
}

impl Event<'_> {
    /// its hook
    #[must_use]
    pub fn hook(&self) -> Hook {
        match self {
            Self::Tick => Hook::Tick,
            Self::Ai => Hook::Ai,
            Self::Hud => Hook::Hud,
            Self::Ui => Hook::Ui,
            Self::Frame => Hook::Frame,
            Self::Render(_) => Hook::Render,
            Self::Sound(_) => Hook::Sound,
        }
    }

    /// the event of a hook that carries nothing
    fn bare(hook: Hook) -> Option<Self> {
        Some(match hook {
            Hook::Tick => Self::Tick,
            Hook::Ai => Self::Ai,
            Hook::Hud => Self::Hud,
            Hook::Ui => Self::Ui,
            Hook::Frame => Self::Frame,
            Hook::Render | Hook::Sound => return None,
        })
    }
}

/// The game's memory as a plugin sees it, for one event: read only, and
/// every read checked against its size.
pub struct View<'a> {
    space: &'a Space<'a>,
    /// the addresses of the data the game exports, by name
    exports: &'a [(String, u32)],
}

impl<'a> View<'a> {
    /// a view of `space`
    pub(crate) fn of(space: &'a Space<'a>) -> Self {
        Self {
            space,
            exports: &[],
        }
    }
    /// Calls `read` with a view of `memory`, as a plugin would be handed
    /// the game's: for the readers' tests and fuzzing, and for testing a
    /// plugin without a game.
    pub fn over<R>(memory: &mut [u8], read: impl FnOnce(&View<'_>) -> R) -> R {
        Self::over_with(memory, &[], read)
    }

    /// As [`View::over`], with `exports` as the data the game exports:
    /// each name and its address.
    pub fn over_with<R>(
        memory: &mut [u8],
        exports: &[(String, u32)],
        read: impl FnOnce(&View<'_>) -> R,
    ) -> R {
        let space = Space::own(memory);
        read(&View {
            space: &space,
            exports,
        })
    }

    /// The address of the data the game exports as `name`
    /// (tools/engine_wasm.py's EXPORTS), if it exports it.
    #[must_use]
    pub fn export(&self, name: &str) -> Option<u32> {
        self.exports
            .iter()
            .find(|(export, _)| export == name)
            .map(|&(_, address)| address)
    }

    /// The memory's size in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.space.len()
    }

    /// Whether the memory is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.space.len() == 0
    }

    /// The `len` bytes at `address`, or `None` if any is outside.
    #[must_use]
    pub fn bytes(&self, address: u32, len: u32) -> Option<Vec<u8>> {
        self.space.read(address, len)
    }

    /// Up to `max` bytes at `address`, cut at the end of the memory, or
    /// `None` if `address` is outside it.
    #[must_use]
    pub fn bytes_up_to(&self, address: u32, max: u32) -> Option<Vec<u8>> {
        self.space.read_up_to(address, max)
    }

    /// The little-endian u32 at `address`.
    #[must_use]
    pub fn u32(&self, address: u32) -> Option<u32> {
        self.space.get(address)
    }

    /// The little-endian u64 at `address`.
    #[must_use]
    pub fn u64(&self, address: u32) -> Option<u64> {
        self.space.get64(address)
    }

    /// The little-endian u16 at `address`.
    #[must_use]
    pub fn u16(&self, address: u32) -> Option<u16> {
        let bytes = self.space.read(address, 2)?;
        Some(u16::from_le_bytes(<[u8; 2]>::try_from(bytes).ok()?))
    }

    /// The little-endian f32 at `address`.
    #[must_use]
    pub fn f32(&self, address: u32) -> Option<f32> {
        self.space.get(address).map(f32::from_bits)
    }
}

/// A plugin: Rust code the runtime calls at the hooks it asks for.
pub trait Plugin: Send {
    /// its name, as the dev shell and the timings show it
    fn name(&self) -> &str;

    /// the hooks it is called at
    fn hooks(&self) -> &[Hook];

    /// called at each of its hooks, with the game's memory to read
    fn on(&mut self, event: &Event<'_>, view: &View<'_>);

    /// the most of a frame's time it may take, all its hooks together
    fn share(&self) -> Duration {
        SHARE
    }
}

/// A plugin's share of a frame unless it says otherwise: a twentieth of
/// the 8.33 ms a frame has at 120 Hz.
pub const SHARE: Duration = Duration::from_micros(416);

/// The frames whose times are kept until they are taken.
pub const FRAMES_KEPT: usize = 3600;

/// One plugin's time in one frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginTime {
    /// the plugin's name
    pub name: String,
    /// its time in the frame, all its hooks together, in nanoseconds
    pub nanos: u64,
    /// its share of a frame, in nanoseconds
    pub share: u64,
}

impl PluginTime {
    /// Whether it took more than its share.
    #[must_use]
    pub fn over(&self) -> bool {
        self.nanos > self.share
    }
}

/// Each plugin over its share in `frames`: the frame's place among them,
/// and its time.
#[must_use]
pub fn over_share(frames: &[Vec<PluginTime>]) -> Vec<(usize, PluginTime)> {
    frames
        .iter()
        .enumerate()
        .flat_map(|(at, frame)| {
            frame
                .iter()
                .filter(|time| time.over())
                .map(move |time| (at, time.clone()))
        })
        .collect()
}

/// a plugin added, and its time in the frame so far
struct Entry {
    plugin: Box<dyn Plugin>,
    spent: Duration,
}

/// the plugins, in the order they were added, and the frames logged
#[derive(Default)]
struct State {
    entries: Vec<Entry>,
    frames: VecDeque<Vec<PluginTime>>,
}

impl State {
    /// ends the frame: logs each plugin's time in it and starts again
    fn end_frame(&mut self) {
        let frame = self
            .entries
            .iter_mut()
            .map(|entry| PluginTime {
                name: entry.plugin.name().to_string(),
                nanos: nanos(std::mem::take(&mut entry.spent)),
                share: nanos(entry.plugin.share()),
            })
            .collect();
        if self.frames.len() == FRAMES_KEPT {
            self.frames.pop_front();
        }
        self.frames.push_back(frame);
    }
}

/// a time in nanoseconds, as far as a u64 holds it
fn nanos(time: Duration) -> u64 {
    u64::try_from(time.as_nanos()).unwrap_or(u64::MAX)
}

/// the plugins added, in order
#[derive(Default)]
pub(crate) struct Plugins {
    /// whether any is added: the hooks' one check while none is
    any: AtomicBool,
    state: Mutex<State>,
    /// the addresses of the data the game exports, set once it is
    /// instantiated
    exports: OnceLock<Vec<(String, u32)>>,
}

impl Plugins {
    /// keeps the addresses of the data the game exports, for the views;
    /// the first instance's are kept, as every thread's are the same
    pub(crate) fn set_exports(&self, exports: Vec<(String, u32)>) {
        let _ = self.exports.set(exports);
    }

    /// the address of the data the game exports as `name`
    pub(crate) fn export(&self, name: &str) -> Option<u32> {
        self.exports
            .get()?
            .iter()
            .find(|(export, _)| export == name)
            .map(|&(_, address)| address)
    }

    /// adds `plugin`, after those already added
    pub(crate) fn add(&self, plugin: Box<dyn Plugin>) {
        self.state
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entries
            .push(Entry {
                plugin,
                spent: Duration::ZERO,
            });
        self.any.store(true, Ordering::Release);
    }

    /// the frames logged since they were last taken, oldest first
    pub(crate) fn take_frames(&self) -> Vec<Vec<PluginTime>> {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        std::mem::take(&mut state.frames).into()
    }

    /// whether any plugin is added
    pub(crate) fn any(&self) -> bool {
        self.any.load(Ordering::Acquire)
    }
}

/// calls each plugin that asks for `event`'s hook, with a view of the
/// caller's memory, timing each; at the frame hook, ends the frame;
/// nothing while none is added
///
/// # Errors
/// If the game has no memory yet and exports none.
pub(crate) fn fire(caller: &mut Caller<'_, Record>, event: &Event<'_>) -> wasmtime::Result<()> {
    let shared = Arc::clone(&caller.data().shared);
    if !shared.plugins.any() {
        return Ok(());
    }
    let hook = event.hook();
    let mut state = shared
        .plugins
        .state
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    if state
        .entries
        .iter()
        .any(|entry| entry.plugin.hooks().contains(&hook))
    {
        with_space(caller, |space, _| {
            let view = View {
                space: &*space,
                exports: shared.plugins.exports.get().map_or(&[], Vec::as_slice),
            };
            for entry in &mut state.entries {
                if entry.plugin.hooks().contains(&hook) {
                    let start = Instant::now();
                    entry.plugin.on(event, &view);
                    entry.spent += start.elapsed();
                }
            }
        })?;
    }
    if hook == Hook::Frame {
        state.end_frame();
    }
    Ok(())
}

/// fires the hook that follows the exit of the profile section `name`
///
/// # Errors
/// As [`fire`].
pub(crate) fn after_section(caller: &mut Caller<'_, Record>, name: &str) -> wasmtime::Result<()> {
    match Hook::after_section(name).and_then(Event::bare) {
        Some(event) => fire(caller, &event),
        None => Ok(()),
    }
}
