//! What the Gearbox PC port lost, measured in the original's own code: each
//! effect of the Xbox rasterizer has its switch among the original's
//! console globals (`rasterizer_debug_options`,
//! rasterizer/rasterizer_console_vars.h), so Blood Gulch is played once as
//! it is and once with each effect switched off, the time stepped a 60th a
//! frame (`debug.frame_clock`) so that frame N is the same moment in every
//! run, and the draws each frame's commands make are counted. An effect
//! the game draws in Blood Gulch draws less with its switch off; the
//! catalogue (docs/pc_port.md) records what each one measured. Needs
//! game.wasm and the user's maps (assets/maps); skips itself without them.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use opence_render::Command;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime, Stopper};

/// the effects measured: the original's console global that switches each
/// off, and what it draws
const EFFECTS: &[(&str, &str)] = &[
    ("rasterizer_environment_lightmaps", "the lightmaps"),
    ("rasterizer_environment_decals", "the environment's decals"),
    ("rasterizer_environment_specular_lights", "specular lights"),
    (
        "rasterizer_environment_specular_lightmaps",
        "specular lightmaps",
    ),
    ("rasterizer_environment_reflections", "bumped reflections"),
    (
        "rasterizer_environment_transparents",
        "transparent geometry",
    ),
    ("rasterizer_environment_fog", "the environment's fog"),
    ("rasterizer_environment_fog_screen", "the fog screen"),
    ("rasterizer_fog_plane", "planar fog"),
    ("rasterizer_fog_atmosphere", "atmospheric fog"),
    ("rasterizer_water", "the water"),
    ("rasterizer_lens_flares", "lens flares"),
    ("rasterizer_lens_flares_occlusion", "lens flare occlusion"),
    ("rasterizer_detail_objects", "detail objects"),
    ("rasterizer_models", "the models"),
    ("rasterizer_model_transparents", "the models' transparents"),
    ("rasterizer_bump_mapping", "bump mapping"),
    ("rasterizer_shadows_convolution", "the shadows' softening"),
    ("rasterizer_hud_motion_sensor", "the motion sensor"),
    ("rasterizer_active_camouflage", "active camouflage"),
    ("rasterizer_plasma_energy", "plasma energy"),
    ("rasterizer_screen_flashes", "screen flashes"),
    ("rasterizer_screen_effects", "screen effects"),
];

/// the effects Blood Gulch draws in the frames played, each found to draw
/// less with its switch off
const DRAWN_IN_BLOOD_GULCH: &[&str] = &[
    "rasterizer_environment_lightmaps",
    "rasterizer_environment_decals",
    "rasterizer_environment_reflections",
    "rasterizer_environment_transparents",
    "rasterizer_environment_fog",
    "rasterizer_models",
    "rasterizer_model_transparents",
    "rasterizer_shadows_convolution",
    "rasterizer_hud_motion_sensor",
];

/// how long each run plays once the map is loaded: ten seconds, in ticks
/// (frame N is the same moment in every run, so each run's frames compare)
const PLAYED: u32 = 300;

/// runs at once
const AT_ONCE: usize = 4;

/// each frame's draws and the vertices (or indices) they draw
type Frames = Vec<(u64, u64)>;

/// counts each frame's draws, and ends the game after `ticks` ticks of play
struct Draws {
    frames: Arc<Mutex<Frames>>,
    ticks: u32,
    stop: Stopper,
}

impl Plugin for Draws {
    fn name(&self) -> &str {
        "draws"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Render, Hook::Tick]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        if let Event::Tick = event {
            self.ticks = self.ticks.saturating_sub(1);
            if self.ticks == 0 {
                self.stop.stop();
            }
            return;
        }
        let Event::Render(commands) = event else {
            return;
        };
        let (mut draws, mut drawn) = (0u64, 0u64);
        for command in *commands {
            match *command {
                Command::Draw {
                    vertices,
                    instances,
                    ..
                } => {
                    draws += 1;
                    drawn += u64::from(vertices) * u64::from(instances.max(1));
                }
                Command::DrawIndexed {
                    indices, instances, ..
                } => {
                    draws += 1;
                    drawn += u64::from(indices) * u64::from(instances.max(1));
                }
                _ => {}
            }
        }
        if let Ok(mut frames) = self.frames.lock() {
            frames.push((draws, drawn));
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Blood Gulch played with `off` switched off (or as it is), and its
/// frames' draws
fn play(wasm: &[u8], off: Option<&str>) -> Result<Frames, Error> {
    let root = root();
    let saves = std::env::temp_dir().join(format!(
        "opence-pc-port-{}-{}",
        std::process::id(),
        off.unwrap_or("stock")
    ));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    assert!(game.set_setting("debug.frame_clock", "true"));
    let mut commands = String::new();
    if let Some(global) = off {
        let _ = writeln!(commands, "{global} false");
    }
    commands.push_str("map_name bloodgulch\n");
    game.set_init_commands(&commands);
    let frames = Arc::new(Mutex::new(Frames::new()));
    game.add_plugin(Box::new(Draws {
        frames: Arc::clone(&frames),
        ticks: PLAYED,
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
        .map(|frames| frames.clone())
        .unwrap_or_default();
    Ok(frames)
}

/// the mean draws and vertices a frame of play draws: the frames from the
/// first that draws at least `threshold` (the loading screen's frames
/// before it draw a few, and each run reaches play at a different frame)
fn per_frame(run: &Frames, threshold: u64) -> (f64, f64) {
    let play: Vec<_> = run
        .iter()
        .skip_while(|(draws, _)| *draws < threshold)
        .collect();
    let frames = play.len().max(1) as f64;
    let (draws, drawn) = play
        .iter()
        .fold((0, 0), |(draws, drawn), (d, v)| (draws + d, drawn + v));
    (draws as f64 / frames, drawn as f64 / frames)
}

#[test]
fn each_effect_blood_gulch_draws_draws_less_with_its_switch_off() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(());
    };
    let wasm = Arc::new(wasm);
    let mut runs: Vec<Option<&str>> = vec![None];
    runs.extend(EFFECTS.iter().map(|(global, _)| Some(*global)));
    let mut played: Vec<(Option<&str>, Frames)> = Vec::new();
    for batch in runs.chunks(AT_ONCE) {
        let handles: Vec<_> = batch
            .iter()
            .map(|off| {
                let wasm = Arc::clone(&wasm);
                let off = off.map(str::to_owned);
                std::thread::spawn(move || play(&wasm, off.as_deref()))
            })
            .collect();
        for (off, handle) in batch.iter().zip(handles) {
            let frames = handle.join().unwrap_or_else(|_| Ok(Frames::new()))?;
            played.push((*off, frames));
        }
    }
    let Some((None, stock)) = played.first().cloned() else {
        panic!("no stock run");
    };
    assert!(
        stock.iter().any(|(draws, _)| *draws > 0),
        "the stock run drew nothing"
    );
    let threshold = stock.iter().map(|(draws, _)| *draws).max().unwrap_or(0) / 2;
    let (draws_on, drawn_on) = per_frame(&stock, threshold);
    let off: Vec<_> = played
        .iter()
        .skip(1)
        .filter_map(|(off, frames)| off.map(|global| (global, per_frame(frames, threshold))))
        .collect();
    // (a switched-off run draws a little more a frame than the stock one,
    // the console echoing its init command, so each is measured against the
    // median switched-off run: most effects Blood Gulch does not draw)
    let mut each: Vec<f64> = off.iter().map(|(_, (draws, _))| *draws).collect();
    each.sort_by(f64::total_cmp);
    let median = each.get(each.len() / 2).copied().unwrap_or(draws_on);
    let mut table = String::from("global\tdraws on\tdraws off\tdrawn on\tdrawn off\n");
    let _ = writeln!(
        table,
        "(median off)\t{draws_on:.2}\t{median:.2}\t{drawn_on:.0}\t"
    );
    let mut less = Vec::new();
    for (global, (draws_off, drawn_off)) in &off {
        let _ = writeln!(
            table,
            "{global}\t{draws_on:.2}\t{draws_off:.2}\t{drawn_on:.0}\t{drawn_off:.0}"
        );
        // (a frame of play draws the same each run to a hundredth of a draw,
        // so an effect counts as drawn when switching it off saves at least
        // half a draw a frame)
        if median >= draws_off + 0.5 {
            less.push(*global);
        }
    }
    eprintln!("{table}");
    let _ = std::fs::create_dir_all(root.join("build/engine"));
    let _ = std::fs::write(root.join("build/engine/pc_port.tsv"), &table);
    for global in DRAWN_IN_BLOOD_GULCH {
        assert!(
            less.contains(global),
            "{global} drew no less switched off:\n{table}"
        );
    }
    Ok(())
}
