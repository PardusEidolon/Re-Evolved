//! The particles, drawn by the original's own code in game.wasm: Blood
//! Gulch with the rifle fired makes particles in the game state's
//! "particle" data array (`particles_initialize`, effects/particles.c), and
//! `render_particles` (render/render_particles.c) runs each frame they are
//! live. Needs game.wasm and the user's maps (assets/maps); skips itself
//! without them.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::views::{GAME_STATE_ADDRESS, GAME_STATE_SIZE};
use opence_runtime::{Error, Runtime, Stopper};

/// what the plugin saw: whether the game has ticked, the most particles
/// live in a frame, and the frames with any
#[derive(Default)]
struct Seen {
    ticked: AtomicBool,
    most: AtomicU32,
    frames_with: AtomicU32,
}

/// the frames with particles live that answer the check: a second's
const FRAMES_WITH: u32 = 30;

/// counts the particle array's live datums at the end of each frame, and
/// ends the game once `FRAMES_WITH` frames have had some
struct Particles {
    seen: Arc<Seen>,
    array: Option<u32>,
    stop: Stopper,
}

impl Plugin for Particles {
    fn name(&self) -> &str {
        "particles"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick, Hook::Frame]
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        if matches!(event, Event::Tick) {
            self.seen.ticked.store(true, Ordering::Relaxed);
            return;
        }
        if !self.seen.ticked.load(Ordering::Relaxed) {
            return;
        }
        // (found once: the game state's arrays stay where they were made)
        if self.array.is_none() {
            self.array = view
                .find_data_array(GAME_STATE_ADDRESS, GAME_STATE_SIZE, "particle")
                .map(|(address, _)| address);
        }
        let live = self
            .array
            .and_then(|address| view.data_array(address))
            .map_or(0, |array| array.datums(view).len());
        let live = u32::try_from(live).unwrap_or(u32::MAX);
        self.seen.most.fetch_max(live, Ordering::Relaxed);
        if live > 0 && self.seen.frames_with.fetch_add(1, Ordering::Relaxed) + 1 >= FRAMES_WITH {
            self.stop.stop();
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

#[test]
fn the_rifle_fired_in_blood_gulch_makes_particles_the_original_draws() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(());
    };
    let saves = std::env::temp_dir().join(format!("opence-particles-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    game.set_init_commands("map_name bloodgulch\n");
    let seen = Arc::new(Seen::default());
    game.add_plugin(Box::new(Particles {
        seen: Arc::clone(&seen),
        array: None,
        stop: game.stopper(),
    }));
    game.set_profiling(true);
    // the left button held from a second after the first tick
    let (controls, watching) = (game.controls(), Arc::clone(&seen));
    let started = Instant::now();
    let deadline = started + Duration::from_secs(90);
    std::thread::spawn(move || {
        while !watching.ticked.load(Ordering::Relaxed) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(50));
        }
        std::thread::sleep(Duration::from_secs(1));
        controls.mouse_button(1, true);
    });
    // (the deadline is for a rifle that never makes particles)
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let spans = game.take_profile_spans();
    let drawn = spans
        .iter()
        .filter(|span| span.name == "render_particles")
        .count();
    let (most, frames_with) = (
        seen.most.load(Ordering::Relaxed),
        seen.frames_with.load(Ordering::Relaxed),
    );
    eprintln!("particles: at most {most} live, {frames_with} frames with some, render_particles {drawn} times");
    assert!(seen.ticked.load(Ordering::Relaxed), "the game never ticked");
    assert!(most > 0, "no particle was made");
    assert!(
        u32::try_from(drawn).unwrap_or(u32::MAX) >= frames_with,
        "render_particles ran {drawn} times in {frames_with} frames with particles"
    );
    Ok(())
}
