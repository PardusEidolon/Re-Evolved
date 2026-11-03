//! The game's profile sections, timed. game.wasm is built with its own
//! `profile_sections.h` (`port/wasmtime/include/`), which makes each
//! section's entry and exit (`profile_scope` in `source/game/game.c` and
//! `source/main/main.c`) a call of `halo_profile_enter(name)` and
//! `halo_profile_exit(name)`; while profiling is on
//! ([`crate::Game::set_profiling`]) the runtime keeps a [`Span`] for each,
//! so the benchmark times the game's own frame and tick
//! (`opence-game --benchmark`). Off, each call returns at once.

use std::collections::BTreeMap;
use std::time::Instant;

use wasmtime::{Caller, Val};

use super::memory::{arg, Handler};
use super::{plugins, with_space, Record};

/// the span the game's first thread spends in `Sleep` or `SleepEx`, kept
/// while profiling is on: the stock game's 30 Hz pacing
/// (in `source/main/main.c`, before each frame) sleeps there, so the
/// benchmark leaves it out of the frame as it does the throttle
pub const SLEEP: &str = "Sleep";

/// the longest section name read
const NAME_BYTES: u32 = 64;
/// the most spans kept until they are taken: a minute of frames at well
/// over a thousand a second, each with its sections
const MAXIMUM_SPANS: usize = 1 << 22;

/// a section's run, in nanoseconds from when profiling was turned on
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// the section's name, as the game gives it
    pub name: String,
    /// when it was entered
    pub start_ns: u64,
    /// when it exited
    pub end_ns: u64,
}

/// the sections entered and the spans ended, while profiling is on
#[derive(Debug, Default)]
pub(crate) struct Profile {
    /// when profiling was turned on (`None`: off)
    origin: Option<Instant>,
    /// each name read, by its address (a section's name is a constant)
    names: BTreeMap<u32, String>,
    /// the sections entered and not yet exited, by name: when, innermost last
    open: BTreeMap<u32, Vec<u64>>,
    /// the spans ended, in the order they ended
    spans: Vec<Span>,
}

impl Profile {
    /// turns profiling on (from now) or off (forgetting what is open)
    pub(crate) fn set(&mut self, on: bool) {
        self.origin = on.then(Instant::now);
        self.open.clear();
    }

    /// the spans ended since they were last taken
    pub(crate) fn take(&mut self) -> Vec<Span> {
        std::mem::take(&mut self.spans)
    }

    /// nanoseconds since profiling was turned on, if it is
    fn now(&self) -> Option<u64> {
        let origin = self.origin?;
        Some(u64::try_from(origin.elapsed().as_nanos()).unwrap_or(u64::MAX))
    }
}

/// when a sleep of the game's first thread begins, if profiling is on
pub(crate) fn sleep_start(record: &Record) -> Option<u64> {
    (record.thread == 0)
        .then(|| record.profile().now())
        .flatten()
}

/// the sleep that began at `start` ends: a [`SLEEP`] span
pub(crate) fn sleep_end(record: &Record, start: Option<u64>) {
    let mut profile = record.profile();
    if let (Some(start_ns), Some(end_ns)) = (start, profile.now()) {
        if profile.spans.len() < MAXIMUM_SPANS {
            profile.spans.push(Span {
                name: SLEEP.to_owned(),
                start_ns,
                end_ns,
            });
        }
    }
}

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "halo_profile_enter" => enter,
        "halo_profile_exit" => exit,
        _ => return None,
    };
    Some(handler)
}

/// the section's name at `address`, read once (`None` if it is outside the
/// memory or not a string)
fn name(caller: &mut Caller<'_, Record>, address: u32) -> wasmtime::Result<Option<()>> {
    let known = caller.data().profile().names.contains_key(&address);
    if known {
        return Ok(Some(()));
    }
    let text = with_space(caller, |space, _| {
        let bytes = space.read_up_to(address, NAME_BYTES)?;
        let end = bytes.iter().position(|&byte| byte == 0)?;
        String::from_utf8(bytes.get(..end)?.to_vec()).ok()
    })?;
    Ok(text.map(|text| {
        caller.data().profile().names.insert(address, text);
    }))
}

/// `halo_profile_enter(name)`: the section begins
fn enter(caller: &mut Caller<'_, Record>, params: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    let address = arg(params, 0);
    let on = caller.data().profile().origin.is_some();
    if !on || name(caller, address)?.is_none() {
        return Ok(());
    }
    let mut profile = caller.data().profile();
    if let Some(now) = profile.now() {
        profile.open.entry(address).or_default().push(now);
    }
    Ok(())
}

/// `halo_profile_exit(name)`: the section, if it was entered, ends; then
/// the plugin hook that follows it, if any, fires
fn exit(caller: &mut Caller<'_, Record>, params: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    let address = arg(params, 0);
    end(caller.data(), address);
    if !caller.data().shared.plugins.any() || name(caller, address)?.is_none() {
        return Ok(());
    }
    let section = caller.data().profile().names.get(&address).cloned();
    match section {
        Some(section) => plugins::after_section(caller, &section),
        None => Ok(()),
    }
}

/// the span of the section at `address`, if it was entered while
/// profiling is on, ends
fn end(record: &Record, address: u32) {
    let mut profile = record.profile();
    let Some(end_ns) = profile.now() else {
        return;
    };
    let Some(start_ns) = profile.open.get_mut(&address).and_then(Vec::pop) else {
        return;
    };
    let Some(name) = profile.names.get(&address).cloned() else {
        return;
    };
    if profile.spans.len() < MAXIMUM_SPANS {
        profile.spans.push(Span {
            name,
            start_ns,
            end_ns,
        });
    }
}
