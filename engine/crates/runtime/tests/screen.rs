//! The screen at the drawable's resolution: Blood Gulch on a 3840 by 2160
//! drawable is drawn to targets of that many pixels, as the GL port's
//! device scales them (`screen_mode_choose`, port/linux/src/d3d8_gl.c),
//! not to the Xbox's 640 by 480 scaled up at the end. Needs game.wasm and
//! the user's maps (assets/maps); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use opence_render::Command;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime, Stopper};

/// what the plugin saw: the tallest render target made, and the widest
/// and tallest viewport set
#[derive(Default, Debug, Clone, Copy)]
struct Seen {
    target: (u32, u32),
    viewport: (f32, f32),
}

/// records the render targets and viewports in each frame's commands
struct Screen {
    seen: Arc<Mutex<Seen>>,
}

impl Plugin for Screen {
    fn name(&self) -> &str {
        "screen"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        let Event::Render(commands) = event else {
            return;
        };
        let Ok(mut seen) = self.seen.lock() else {
            return;
        };
        for command in *commands {
            match *command {
                Command::CreateTexture {
                    width,
                    height,
                    render_target: true,
                    ..
                } if height > seen.target.1 => seen.target = (width, height),
                Command::SetViewport { width, height, .. } => {
                    seen.viewport.0 = seen.viewport.0.max(width);
                    seen.viewport.1 = seen.viewport.1.max(height);
                }
                _ => {}
            }
        }
    }
}

/// ends the game once it has played `ticks` ticks of the map
struct Played {
    ticks: u32,
    stop: Stopper,
}

impl Plugin for Played {
    fn name(&self) -> &str {
        "played"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick]
    }

    fn on(&mut self, _event: &Event<'_>, _view: &View<'_>) {
        self.ticks = self.ticks.saturating_sub(1);
        if self.ticks == 0 {
            self.stop.stop();
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

#[test]
fn blood_gulch_on_a_4k_drawable_is_drawn_at_4k() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(());
    };
    let saves = std::env::temp_dir().join(format!("opence-screen-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (3840, 2160))?;
    game.set_init_commands("map_name bloodgulch\n");
    let seen = Arc::new(Mutex::new(Seen::default()));
    game.add_plugin(Box::new(Screen {
        seen: Arc::clone(&seen),
    }));
    // (the load and two seconds of play, 30 ticks a second; the deadline
    // is for a stuck run)
    game.add_plugin(Box::new(Played {
        ticks: 60,
        stop: game.stopper(),
    }));
    game.set_deadline(Some(Instant::now() + Duration::from_secs(300)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let seen = seen.lock().map(|seen| *seen).unwrap_or_default();
    eprintln!("screen: {seen:?}");
    // 852 by 480 units at 4.5 pixels each: 3834 by 2160
    assert!(
        seen.target.1 >= 2160,
        "the tallest render target is {:?}",
        seen.target
    );
    assert!(
        seen.target.0 >= 3800,
        "the tallest render target is {:?}",
        seen.target
    );
    assert!(
        seen.viewport.1 >= 2160.0,
        "the tallest viewport is {:?}",
        seen.viewport
    );
    Ok(())
}
