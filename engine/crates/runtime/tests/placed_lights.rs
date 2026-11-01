//! The dynamic lights the game submits each frame (`rasterizer_lights`)
//! reach the host: game.wasm's device sends them as one LIGHTS command
//! before each PRESENT, every light checked. With `lighting.placed_lights`
//! shadowed, the light passes are shadowed through a copy of the frame's
//! depth, made once a frame and bound to the pass's pixel shader. A map is
//! played for a time and the frames' commands are read. Needs game.wasm (or
//! the one `OPENCE_GAME_WASM` names) and the user's maps (assets/maps);
//! skips itself without them.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use opence_render::{Command, Stage};
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime, Stopper};

/// what the frames' commands held
#[derive(Default, Debug, Clone, Copy)]
struct Seen {
    presents: usize,
    lights: usize,
    /// a PRESENT without a LIGHTS before it since the last one
    unlit_presents: usize,
    most: usize,
    lit_frames: usize,
    pending: bool,
    /// copies of a depth texture: in all, the most in one frame, the frames
    /// with one, and this frame's
    depth_copies: usize,
    most_depth_copies: usize,
    depth_copied_frames: usize,
    frame_depth_copies: usize,
    /// a depth copy bound as the pixel shader's texture 4
    shadow_binds: usize,
}

/// the fragment texture slot the light passes read the depth copy from
/// (nv2a_psh.c)
const SHADOW_SLOT: u32 = 4;

struct Lights {
    seen: Arc<Mutex<Seen>>,
    depths: BTreeSet<u32>,
    copies: BTreeSet<u32>,
    /// ends the game once `enough` holds for what was seen
    stop: Stopper,
    enough: fn(&Seen) -> bool,
}

impl Plugin for Lights {
    fn name(&self) -> &str {
        "lights"
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
            match command {
                Command::Lights { lights } => {
                    seen.lights += 1;
                    seen.pending = true;
                    seen.most = seen.most.max(lights.len());
                    seen.lit_frames += usize::from(!lights.is_empty());
                }
                Command::CreateTexture { id, format, .. } => {
                    if format.is_depth() {
                        self.depths.insert(*id);
                    } else {
                        self.depths.remove(id);
                    }
                }
                Command::CopyTexture {
                    source,
                    destination,
                    ..
                } if self.depths.contains(source) => {
                    seen.depth_copies += 1;
                    seen.frame_depth_copies += 1;
                    self.copies.insert(*destination);
                }
                Command::SetTexture { stage, slot, id }
                    if *stage == Stage::Fragment
                        && *slot == SHADOW_SLOT
                        && self.copies.contains(id) =>
                {
                    seen.shadow_binds += 1;
                }
                Command::Present => {
                    seen.presents += 1;
                    seen.unlit_presents += usize::from(!seen.pending);
                    seen.pending = false;
                    seen.most_depth_copies = seen.most_depth_copies.max(seen.frame_depth_copies);
                    seen.depth_copied_frames += usize::from(seen.frame_depth_copies > 0);
                    seen.frame_depth_copies = 0;
                }
                _ => {}
            }
        }
        if (self.enough)(&seen) {
            self.stop.stop();
        }
    }
}

/// what `map` drew with `lighting.placed_lights` at `placed_lights`, played
/// until `enough` holds for it; None without game.wasm or the map
fn play(map: &str, placed_lights: &str, enough: fn(&Seen) -> bool) -> Result<Option<Seen>, Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm = std::env::var_os("OPENCE_GAME_WASM")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("build/engine/wasm/game.wasm"));
    let (Ok(bytes), true) = (
        std::fs::read(&wasm),
        root.join(format!("assets/maps/{map}.map")).is_file(),
    ) else {
        eprintln!("skipped: needs game.wasm and assets/maps/{map}.map");
        return Ok(None);
    };
    let saves = std::env::temp_dir().join(format!("opence-lights-{}-{map}", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let cache = wasm.parent().map(|folder| folder.join("cache"));
    let runtime = Runtime::new(cache.as_deref())?;
    let mut game = runtime.load(&bytes)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    assert!(game.set_setting("lighting.placed_lights", placed_lights));
    game.set_init_commands(&format!("map_name {map}\n"));
    let seen = Arc::new(Mutex::new(Seen::default()));
    game.add_plugin(Box::new(Lights {
        seen: Arc::clone(&seen),
        depths: BTreeSet::new(),
        copies: BTreeSet::new(),
        stop: game.stopper(),
        enough,
    }));
    // (the deadline is for a run that never has enough)
    game.set_deadline(Some(Instant::now() + Duration::from_secs(120)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    assert!(matches!(ran, Err(Error::Deadline)), "{ran:?}");
    let seen = seen.lock().map(|seen| *seen).unwrap_or_default();
    eprintln!("{map} ({placed_lights}): {seen:?}");
    Ok(Some(seen))
}

#[test]
fn each_frame_sends_its_lights_before_it_is_shown() -> Result<(), Error> {
    // OPENCE_LIGHTS_MAP measures another map (idle Blood Gulch submits none)
    let map = std::env::var("OPENCE_LIGHTS_MAP").unwrap_or_else(|_| "bloodgulch".into());
    // (ten seconds of frames, at 30 a second)
    let Some(seen) = play(&map, "stock", |seen| seen.presents >= 300)? else {
        return Ok(());
    };
    assert!(seen.presents > 0, "{seen:?}");
    assert_eq!(seen.lights, seen.presents, "{seen:?}");
    assert_eq!(seen.unlit_presents, 0, "{seen:?}");
    // a refused LIGHTS never reaches a plugin, and the game sends at most
    // MAXIMUM_LIGHTS_PER_WINDOW
    assert!(seen.most <= 128, "{seen:?}");
    // stock draws no shadows
    assert_eq!(seen.shadow_binds, 0, "{seen:?}");
    Ok(())
}

#[test]
fn shadowed_light_passes_read_one_depth_copy_a_frame() -> Result<(), Error> {
    // the Pillar of Autumn's corridors have the lights (up to 7 a frame)
    let map = std::env::var("OPENCE_SHADOW_MAP").unwrap_or_else(|_| "c10".into());
    // (two seconds of lit frames)
    let Some(seen) = play(&map, "shadowed", |seen| seen.depth_copied_frames >= 60)? else {
        return Ok(());
    };
    assert!(seen.lit_frames > 0, "{seen:?}");
    assert!(seen.depth_copied_frames > 0, "{seen:?}");
    assert_eq!(seen.most_depth_copies, 1, "{seen:?}");
    assert!(seen.shadow_binds >= seen.depth_copied_frames, "{seen:?}");
    Ok(())
}
