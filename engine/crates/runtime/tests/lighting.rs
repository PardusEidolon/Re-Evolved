//! The lighting's switch to stock: `display.per_pixel_lighting` lights the
//! game's model lighting programs for each pixel, from the original's own
//! lights (the ambient light, two distant lights and two point lights,
//! nv2a_psh.c `model_lighting`), and off, as it is by default, the models
//! are lit at each vertex as on the Xbox. Blood Gulch is played for a time
//! each way, and the shader libraries the device makes are read: off, none
//! of them lights a pixel; on, some do. The lighting extensions'
//! switches (`lighting.*`) draw stock until each is drawn, and say so in
//! the game's log, also when one is switched while the game runs. Needs
//! game.wasm and the user's maps (assets/maps); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use opence_render::Command;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime, Stopper, Tuning};

/// the shader libraries made, and how many of them light each pixel
#[derive(Default, Debug, Clone, Copy)]
struct Libraries {
    made: usize,
    lit: usize,
}

/// reads the source of each library the frame's commands make
struct Shaders {
    seen: Arc<Mutex<Libraries>>,
}

impl Plugin for Shaders {
    fn name(&self) -> &str {
        "shaders"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        let Event::Render(commands) = event else {
            return;
        };
        let Ok(mut seen) = self.seen.lock() else {
            return;
        };
        for command in *commands {
            if let Command::CreateLibrary {
                address, length, ..
            } = *command
            {
                seen.made += 1;
                let source = view.bytes(address, length).unwrap_or_default();
                if source.windows(15).any(|word| word == b"model_lighting(") {
                    seen.lit += 1;
                }
            }
        }
    }
}

/// ends the game once it has played `ticks` ticks of the map, first
/// setting `switch`, as the control socket would, at `switch_at` ticks
struct Played {
    ticks: u32,
    stop: Stopper,
    switch: Option<(u32, Tuning, &'static str, &'static str)>,
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
        if let Some((at, tuning, key, value)) = &mut self.switch {
            *at = at.saturating_sub(1);
            if *at == 0 {
                assert!(tuning.set(key, value), "{key} was not set");
                self.switch = None;
            }
        }
        if self.ticks == 0 {
            self.stop.stop();
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Blood Gulch played for ten seconds (300 ticks) with `per_pixel` as the
/// setting (or left as it is), and the libraries its device made; `None`
/// without the game or the map
fn libraries(per_pixel: Option<&str>) -> Result<Option<Libraries>, Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(None);
    };
    let saves = std::env::temp_dir().join(format!(
        "opence-lighting-{}-{}",
        std::process::id(),
        per_pixel.unwrap_or("default")
    ));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    if let Some(text) = per_pixel {
        assert!(game.set_setting("display.per_pixel_lighting", text));
    }
    game.set_init_commands("map_name bloodgulch\n");
    let seen = Arc::new(Mutex::new(Libraries::default()));
    game.add_plugin(Box::new(Shaders {
        seen: Arc::clone(&seen),
    }));
    // (the deadline is for a stuck run)
    game.add_plugin(Box::new(Played {
        ticks: 300,
        stop: game.stopper(),
        switch: None,
    }));
    game.set_deadline(Some(Instant::now() + Duration::from_secs(300)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let seen = seen.lock().map(|seen| *seen).unwrap_or_default();
    eprintln!("lighting {per_pixel:?}: {seen:?}");
    Ok(Some(seen))
}

#[test]
fn by_default_the_models_are_lit_at_each_vertex_as_on_the_xbox() -> Result<(), Error> {
    let Some(seen) = libraries(None)? else {
        return Ok(());
    };
    assert!(seen.made > 0, "no shader library was made");
    assert_eq!(seen.lit, 0, "{seen:?}");
    Ok(())
}

#[test]
fn per_pixel_lighting_lights_the_models_from_the_original_lights() -> Result<(), Error> {
    let Some(seen) = libraries(Some("true"))? else {
        return Ok(());
    };
    assert!(seen.lit > 0, "{seen:?}");
    assert!(seen.lit < seen.made, "{seen:?}");
    Ok(())
}

#[test]
fn the_lighting_switches_say_what_they_draw() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/bloodgulch.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/bloodgulch.map");
        return Ok(());
    };
    let saves =
        std::env::temp_dir().join(format!("opence-lighting-{}-switches", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    assert!(game.set_setting("lighting.bounce", "baked"));
    assert!(game.set_setting("lighting.shadows", "sideways"));
    game.set_init_commands("map_name bloodgulch\n");
    let seen = Arc::new(Mutex::new(Libraries::default()));
    game.add_plugin(Box::new(Shaders {
        seen: Arc::clone(&seen),
    }));
    // one switched while the game runs, as the control socket would (the
    // deadline is for a stuck run)
    game.add_plugin(Box::new(Played {
        ticks: 300,
        stop: game.stopper(),
        switch: Some((120, game.tuning(), "lighting.atmosphere", "scattering")),
    }));
    game.set_deadline(Some(Instant::now() + Duration::from_secs(300)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    assert!(matches!(ran, Err(Error::Deadline)), "{ran:?}");
    let log = String::from_utf8_lossy(game.output(2)).into_owned();
    let lines: Vec<&str> = log
        .lines()
        .filter(|line| line.contains("lighting."))
        .collect();
    assert_eq!(
        lines,
        [
            "opence: lighting.bounce: baked",
            "opence: lighting.shadows: \"sideways\" is unknown, so stock",
            "opence: lighting.atmosphere: scattering",
        ]
    );
    // and no shader library lights a pixel (the bake swaps lightmap pages,
    // not shaders)
    let seen = seen.lock().map(|seen| *seen).unwrap_or_default();
    assert!(seen.made > 0, "{seen:?}");
    assert_eq!(seen.lit, 0, "{seen:?}");
    Ok(())
}
