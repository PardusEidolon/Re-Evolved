//! The benchmark's numbers, from the game's own profile sections
//! ([`crate::Span`]): the CPU frame, from one `input` section's entry to the
//! next less the `throttle` and the first thread's sleeps between them
//! (`source/main/main.c`'s loop and its 30 Hz pacing), and
//! the simulation tick, from `game_tick.cheats_enforce`'s entry to
//! `game_tick.player_effect`'s exit (`source/game/game.c`'s `game_tick`).
//! Frames before the first tick are the map's load and are left out.

use std::collections::BTreeMap;

use crate::Span;

/// the first section of a frame
const FRAME: &str = "input";
/// the section where the frame waits for its time, and the stock game's
/// sleep to its 30 Hz
const WAITS: [&str; 2] = ["throttle", crate::profile::SLEEP];
/// a tick's first step
const TICK_FIRST: &str = "game_tick.cheats_enforce";
/// a tick's last step
const TICK_LAST: &str = "game_tick.player_effect";

/// a measure's spread, in nanoseconds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Summary {
    /// how many were measured
    pub count: usize,
    /// the median
    pub p50_ns: u64,
    /// the 99th percentile
    pub p99_ns: u64,
    /// the longest
    pub max_ns: u64,
}

/// the frame's and the tick's spread (`None`: none were measured)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Report {
    /// the CPU frame, less its throttle and sleeps
    pub frame: Option<Summary>,
    /// the simulation tick
    pub tick: Option<Summary>,
}

/// the nearest-rank `p`th percentile of `sorted` (ascending)
pub fn percentile(sorted: &[u64], p: u64) -> Option<u64> {
    let count = u64::try_from(sorted.len()).ok()?;
    let rank = (p.min(100) * count).div_ceil(100).max(1);
    sorted.get(usize::try_from(rank - 1).ok()?).copied()
}

fn summary(mut values: Vec<u64>) -> Option<Summary> {
    values.sort_unstable();
    Some(Summary {
        count: values.len(),
        p50_ns: percentile(&values, 50)?,
        p99_ns: percentile(&values, 99)?,
        max_ns: *values.last()?,
    })
}

/// the starts of the sections named `name`, in order
fn starts(spans: &[Span], name: &str) -> Vec<u64> {
    let mut starts: Vec<u64> = spans
        .iter()
        .filter(|span| span.name == name)
        .map(|span| span.start_ns)
        .collect();
    starts.sort_unstable();
    starts
}

/// how long the (start, end) spans, in order of their starts, cover
/// (a sleep inside the throttle is counted once)
fn covered(spans: impl Iterator<Item = (u64, u64)>) -> u64 {
    let mut total = 0;
    let mut reached = 0;
    for (start, end) in spans {
        let start = start.max(reached);
        if end > start {
            total += end - start;
            reached = end;
        }
    }
    total
}

/// each tick's (start, end)
fn ticks(spans: &[Span]) -> Vec<(u64, u64)> {
    let firsts = starts(spans, TICK_FIRST);
    let mut lasts: Vec<u64> = spans
        .iter()
        .filter(|span| span.name == TICK_LAST)
        .map(|span| span.end_ns)
        .collect();
    lasts.sort_unstable();
    let mut ticks = Vec::new();
    for (index, &start) in firsts.iter().enumerate() {
        let next = firsts.get(index + 1).copied().unwrap_or(u64::MAX);
        // the first last step after this tick's first, before the next's
        let from = lasts.partition_point(|&end| end < start);
        if let Some(&end) = lasts.get(from).filter(|&&end| end < next) {
            ticks.push((start, end));
        }
    }
    ticks
}

/// each frame's length over `spans`, less its throttle and sleeps, in the
/// order they ran, from the first tick on
pub fn frames(spans: &[Span]) -> Vec<u64> {
    let first_tick = ticks(spans).first().map_or(u64::MAX, |tick| tick.0);
    let inputs = starts(spans, FRAME);
    let mut throttles: Vec<&Span> = spans
        .iter()
        .filter(|span| WAITS.contains(&span.name.as_str()))
        .collect();
    throttles.sort_unstable_by_key(|span| span.start_ns);
    inputs
        .windows(2)
        .filter_map(|pair| match *pair {
            [start, end] => Some((start, end)),
            _ => None,
        })
        // (a frame that ends before the first tick is the load's)
        .filter(|&(_, end)| end > first_tick)
        .map(|(start, end)| {
            let from = throttles.partition_point(|span| span.start_ns < start);
            let within = throttles.get(from..).unwrap_or_default();
            let waits = within.iter().take_while(|span| span.start_ns < end);
            (end - start).saturating_sub(covered(
                waits.map(|span| (span.start_ns, span.end_ns.min(end))),
            ))
        })
        .collect()
}

/// the frame's and the tick's spread over `spans`
pub fn report(spans: &[Span]) -> Report {
    let ticks = ticks(spans);
    Report {
        frame: summary(frames(spans)),
        tick: summary(ticks.iter().map(|&(start, end)| end - start).collect()),
    }
}

/// each section's spread, by its name, so a frame's time can be placed
pub fn sections(spans: &[Span]) -> BTreeMap<String, Summary> {
    let mut lengths: BTreeMap<&str, Vec<u64>> = BTreeMap::new();
    for span in spans {
        lengths
            .entry(span.name.as_str())
            .or_default()
            .push(span.end_ns.saturating_sub(span.start_ns));
    }
    lengths
        .into_iter()
        .filter_map(|(name, values)| Some((name.to_owned(), summary(values)?)))
        .collect()
}
