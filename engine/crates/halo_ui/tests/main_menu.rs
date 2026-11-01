//! The plugin on the game's own main menu: the scene it lays out from the
//! live widget tree. Needs game.wasm and the user's maps (assets/maps,
//! for ui.map); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_halo_ui::{Redraw, Scene, SETTING};
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Controls, Error, Runtime, Stopper};

/// Stops the game once its menus have been up for `frames` passes of its
/// main loop: the main menu is laid out by then.
struct Shown {
    controls: Controls,
    frames: u32,
    stop: Stopper,
}

impl Plugin for Shown {
    fn name(&self) -> &str {
        "shown"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Frame]
    }

    fn on(&mut self, event: &Event<'_>, _view: &View<'_>) {
        if matches!(event, Event::Frame) && self.controls.menus_active() {
            self.frames = self.frames.saturating_sub(1);
            if self.frames == 0 {
                self.stop.stop();
            }
        }
    }
}

/// The scene the plugin last laid out over the main menu's first
/// `frames`, with the setting `on`; `None` without the user's data.
fn main_menu_scene(on: bool, frames: u32) -> Result<Option<Scene>, Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        return Ok(None);
    };
    let folder = std::env::temp_dir().join(format!("opence-halo-ui-{}-{on}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    let tuning = game.tuning();
    assert!(tuning.set(SETTING, if on { "true" } else { "false" }));
    let scene = Arc::new(Mutex::new(Scene::default()));
    game.add_plugin(Box::new(Redraw::new(
        Arc::clone(&scene),
        tuning,
        game.controls(),
    )));
    game.add_plugin(Box::new(Shown {
        controls: game.controls(),
        frames,
        stop: game.stopper(),
    }));
    // (stopped once the menu has drawn its frames; the deadline is for a
    // stuck run)
    game.set_deadline(Some(Instant::now() + Duration::from_secs(120)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let scene = scene.lock().unwrap_or_else(PoisonError::into_inner).clone();
    Ok(Some(scene))
}

#[test]
fn the_main_menu_is_laid_out_from_the_game() -> Result<(), Error> {
    let Some(scene) = main_menu_scene(true, 240)? else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
    eprintln!("the main menu's scene: {texts:?}");
    // (the PC menu's six items, each named by what it does)
    assert_eq!(
        texts.get(..6),
        Some(
            &[
                "CAMPAIGN",
                "MULTIPLAYER",
                "PROFILES",
                "SETTINGS",
                "MODS",
                "QUIT"
            ][..]
        ),
        "{texts:?}"
    );
    // with the setting off the game draws its own, and the plugin nothing
    let off = main_menu_scene(false, 120)?.unwrap_or_default();
    assert!(off.is_empty(), "{off:?}");
    Ok(())
}
