//! `debug.frame_clock` makes a run the same run to run: its vertical blanks
//! pass only while the game waits for them, so a frame drawn late (a busy
//! machine, a test's other runs) still lasts its two blanks and its one
//! tick, as on an Xbox that never fell behind. Blood Gulch is played at 30
//! frames a second (display.interpolation off) with every tenth frame held
//! up by 70 ms, more than four blanks, and each frame after the first must
//! tick the game once and carry the sound of its two blanks (1600 frames
//! at 48 kHz), which is mixed on the frame clock too: the sounds then end on
//! the same frames run to run, and the sound manager's random choices that
//! follow from them repeat. Needs game.wasm and the user's maps
//! (assets/maps); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime, Stopper};

/// the sound frames mixed in two blanks at 48 kHz
const TWO_BLANKS_OF_SOUND: usize = 1600;

/// the frames shown before the game is ended: ten seconds at 30 a second,
/// thirty of them held up
const SHOWN: u32 = 300;

/// the ticks and the frames of sound in each frame shown, from the first
/// tick on
struct Ticks {
    frames: Arc<Mutex<Vec<(u32, usize)>>>,
    this_frame: u32,
    sound: usize,
    ticked: bool,
    shown: u32,
    stop: Stopper,
}

impl Plugin for Ticks {
    fn name(&self) -> &str {
        "ticks"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick, Hook::Frame, Hook::Sound]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        match event {
            Event::Tick => {
                self.ticked = true;
                self.this_frame += 1;
                return;
            }
            // (interleaved stereo)
            Event::Sound(samples) => {
                self.sound += samples.len() / 2;
                return;
            }
            _ => {}
        }
        if !self.ticked {
            self.sound = 0;
            return;
        }
        self.frames
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push((self.this_frame, self.sound));
        self.this_frame = 0;
        self.sound = 0;
        self.shown += 1;
        // (a frame held up: the machine busy elsewhere)
        if self.shown.is_multiple_of(10) {
            std::thread::sleep(Duration::from_millis(70));
        }
        if self.shown >= SHOWN {
            self.stop.stop();
        }
    }
}

#[test]
fn a_frame_drawn_late_still_lasts_its_two_blanks() -> Result<(), Error> {
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
        sound: 0,
        ticked: false,
        shown: 0,
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
    let frames = frames
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    // (the first few frames after the load settle the interval)
    let played: Vec<(u32, usize)> = frames.iter().copied().skip(30).collect();
    assert!(
        played.len() > 100,
        "only {} frames were played",
        frames.len()
    );
    let odd: Vec<(usize, u32)> = played
        .iter()
        .map(|&(ticks, _)| ticks)
        .enumerate()
        .filter(|&(_, ticks)| ticks != 1)
        .collect();
    assert!(
        odd.is_empty(),
        "frames that did not tick once (of {}): {odd:?}",
        played.len()
    );
    let unlike: Vec<(usize, usize)> = played
        .iter()
        .map(|&(_, sound)| sound)
        .enumerate()
        .filter(|&(_, sound)| sound != TWO_BLANKS_OF_SOUND)
        .collect();
    assert!(
        unlike.is_empty(),
        "frames without two blanks of sound (of {}): {unlike:?}",
        played.len()
    );
    Ok(())
}
