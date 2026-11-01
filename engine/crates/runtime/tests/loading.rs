//! The loading screen, the original's own (`progress_bar.c`), drawn while
//! Blood Gulch loads on a 3840 by 2160 drawable, and the frames from it
//! into the map's first. Needs game.wasm and the user's maps
//! (assets/maps); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use opence_render::Command;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime, Stopper};

/// one frame the game presented
#[derive(Debug, Clone, Copy)]
struct Frame {
    /// when it was presented
    at: Instant,
    /// its draws
    draws: u32,
    /// the tallest viewport it set
    height: f32,
    /// whether the game had ticked before it
    ticked: bool,
}

/// what the plugin saw: each frame presented
#[derive(Default)]
struct Seen {
    frames: Vec<Frame>,
    draws: u32,
    height: f32,
    ticked: bool,
}

/// records each presented frame's draws and viewports, and the first tick;
/// ends the game after `ticks` ticks of play
struct Frames {
    seen: Arc<Mutex<Seen>>,
    ticks: u32,
    stop: Stopper,
}

impl Plugin for Frames {
    fn name(&self) -> &str {
        "frames"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick, Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        let Ok(mut seen) = self.seen.lock() else {
            return;
        };
        let Event::Render(commands) = event else {
            seen.ticked = true;
            self.ticks = self.ticks.saturating_sub(1);
            if self.ticks == 0 {
                self.stop.stop();
            }
            return;
        };
        for command in *commands {
            match *command {
                Command::Draw { .. } | Command::DrawIndexed { .. } => seen.draws += 1,
                Command::SetViewport { height, .. } => seen.height = seen.height.max(height),
                Command::Present => {
                    let frame = Frame {
                        at: Instant::now(),
                        draws: seen.draws,
                        height: seen.height,
                        ticked: seen.ticked,
                    };
                    seen.frames.push(frame);
                    seen.draws = 0;
                    seen.height = 0.0;
                }
                _ => {}
            }
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// the frames of one run: Blood Gulch loaded at 4K and played two seconds;
/// none without game.wasm or the maps
fn frames() -> Option<&'static [Frame]> {
    static FRAMES: OnceLock<Option<Vec<Frame>>> = OnceLock::new();
    FRAMES.get_or_init(|| run().ok().flatten()).as_deref()
}

fn run() -> Result<Option<Vec<Frame>>, Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(None);
    };
    let saves = std::env::temp_dir().join(format!("opence-loading-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (3840, 2160))?;
    game.set_init_commands("map_name bloodgulch\n");
    let seen = Arc::new(Mutex::new(Seen::default()));
    game.add_plugin(Box::new(Frames {
        seen: Arc::clone(&seen),
        // (the load and two seconds of play, 30 ticks a second)
        ticks: 60,
        stop: game.stopper(),
    }));
    // (the deadline is for a stuck run)
    game.set_deadline(Some(Instant::now() + Duration::from_secs(300)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let frames = seen
        .lock()
        .map(|seen| seen.frames.clone())
        .unwrap_or_default();
    Ok(Some(frames))
}

#[test]
fn the_loading_screen_is_drawn_at_4k_while_the_map_loads() {
    let Some(frames) = frames() else {
        return;
    };
    let loading: Vec<&Frame> = frames.iter().filter(|frame| !frame.ticked).collect();
    let drawn = loading.iter().filter(|frame| frame.draws > 0).count();
    let height = loading.iter().map(|frame| frame.height).fold(0.0, f32::max);
    eprintln!(
        "loading: {} frames, {drawn} drawn, tallest viewport {height}",
        loading.len()
    );
    assert!(drawn >= 10, "{drawn} loading frames drawn");
    assert!(height >= 2160.0, "the tallest loading viewport is {height}");
}

#[test]
fn the_loading_screen_runs_into_the_maps_first_frame_without_a_hitch() {
    let Some(frames) = frames() else {
        return;
    };
    let Some(first) = frames.iter().position(|frame| frame.ticked) else {
        panic!("the map never finished loading");
    };
    let start = frames.iter().position(|frame| frame.draws > 0).unwrap_or(0);
    // the loading screen's frames, then a second of the map's
    let end = frames[first..]
        .iter()
        .position(|frame| frame.at >= frames[first].at + Duration::from_secs(1))
        .map_or(frames.len(), |end| first + end);
    let window = &frames[start..end];
    let gap = window
        .windows(2)
        .map(|pair| pair[1].at - pair[0].at)
        .max()
        .unwrap_or_default();
    let empty = window.iter().filter(|frame| frame.draws == 0).count();
    eprintln!(
        "transition: frames {start}..{end}, the first of the map {first}, longest gap {gap:?}, {empty} undrawn"
    );
    assert_eq!(empty, 0, "{empty} frames presented with nothing drawn");
    assert!(
        gap <= Duration::from_millis(250),
        "the longest gap between frames is {gap:?}"
    );
}
