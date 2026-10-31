//! `debug.frame_clock` makes a run the same run to run: its vertical blanks
//! pass only while the game waits for them, so a frame drawn late (a busy
//! machine, a test's other runs) still lasts its two blanks and its one
//! tick, as on an Xbox that never fell behind. Blood Gulch is played at 30
//! frames a second (display.interpolation off) with every tenth frame held
//! up by 70 ms, more than four blanks, and each frame after the first ticks
//! must tick the game once. Needs game.wasm and the user's maps
//! (assets/maps); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime};

/// the ticks counted in each frame shown, from the first tick on
struct Ticks {
    frames: Arc<Mutex<Vec<u32>>>,
    this_frame: u32,
    ticked: bool,
    shown: u32,
}

impl Plugin for Ticks {
    fn name(&self) -> &str {
        "ticks"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick, Hook::Frame]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        if matches!(event, Event::Tick) {
            self.ticked = true;
            self.this_frame += 1;
            return;
        }
        if !self.ticked {
            return;
        }
        self.frames
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(self.this_frame);
        self.this_frame = 0;
        self.shown += 1;
        // (a frame held up: the machine busy elsewhere)
        if self.shown.is_multiple_of(10) {
            std::thread::sleep(Duration::from_millis(70));
        }
    }
}

#[test]
fn a_frame_drawn_late_still_ticks_the_game_once() -> Result<(), Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(());
    };
    let saves = std::env::temp_dir().join(format!("opence-frame-clock-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    assert!(game.set_setting("debug.frame_clock", "true"));
    assert!(game.set_setting("display.interpolation", "false"));
    game.set_init_commands("map_name bloodgulch\n");
    let frames = Arc::new(Mutex::new(Vec::new()));
    game.add_plugin(Box::new(Ticks {
        frames: Arc::clone(&frames),
        this_frame: 0,
        ticked: false,
        shown: 0,
    }));
    game.set_deadline(Some(Instant::now() + Duration::from_secs(60)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let frames = frames
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    // (the first few frames after the load settle the interval)
    let played: Vec<u32> = frames.iter().copied().skip(30).collect();
    assert!(
        played.len() > 100,
        "only {} frames were played",
        frames.len()
    );
    let odd: Vec<(usize, u32)> = played
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, ticks)| ticks != 1)
        .collect();
    assert!(
        odd.is_empty(),
        "frames that did not tick once (of {}): {odd:?}",
        played.len()
    );
    Ok(())
}
