//! The game's text at the drawable's resolution: on a 3840 by 2160
//! drawable the main menu's and the pause menu's strings are drawn from the
//! font atlas, each glyph made from its font tag's own character at the
//! display's pixels (port/wasmtime/text_tags.c), not from the tag's 640 by
//! 480 bitmap scaled up. The HUD's messages go through the same
//! `rasterizer_draw_unicode_string`; Blood Gulch's own HUD has no text of
//! its own (its counters are bitmap digits), so the pause menu is the map's
//! check. Needs game.wasm and the user's maps (assets/maps, ui.map and
//! bloodgulch.map); skips itself without them.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_render::{Command, PixelFormat};
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Controls, Error, Runtime, Stopper};

/// the atlas's side, in texels (text_tags.c's `ATLAS_SIZE`)
const ATLAS_SIZE: u32 = 2048;

/// what the plugin saw of the atlas
#[derive(Default, Debug, Clone, Copy)]
struct Seen {
    /// the atlas's texture, once made
    atlas: Option<u32>,
    /// the rows uploaded to it
    rows: u32,
    /// its uploaded texels with any coverage
    inked: u64,
    /// of those, the texels uploaded once a key was pressed
    inked_after: u64,
}

/// watches each frame's commands for the atlas and its uploads; presses
/// Escape `escape_at` ticks into the map if given, and ends the game
/// `frames` frames after that press (or after the start, without one)
struct Atlas {
    seen: Arc<Mutex<Seen>>,
    escape_at: Option<u32>,
    pressed: bool,
    /// the frames Escape stays held
    held: u32,
    frames: u32,
    controls: Controls,
    stop: Stopper,
}

impl Plugin for Atlas {
    fn name(&self) -> &str {
        "the text atlas"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Tick, Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        if let Event::Tick = event {
            if let Some(at) = &mut self.escape_at {
                *at = at.saturating_sub(1);
                if *at == 0 {
                    self.escape_at = None;
                    self.pressed = true;
                    self.controls.key(VK_ESCAPE, 0x1B, 0, true);
                }
            }
            return;
        }
        let Event::Render(commands) = event else {
            return;
        };
        if self.pressed && self.held > 0 {
            self.held -= 1;
            if self.held == 0 {
                self.controls.key(VK_ESCAPE, 0x1B, 0, false);
            }
        }
        if self.escape_at.is_none() {
            self.frames = self.frames.saturating_sub(1);
            if self.frames == 0 {
                self.stop.stop();
            }
        }
        let mut seen = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        for command in *commands {
            match *command {
                Command::CreateTexture {
                    id,
                    format: PixelFormat::Rgba8,
                    width: ATLAS_SIZE,
                    height: ATLAS_SIZE,
                    render_target: false,
                    ..
                } => seen.atlas = Some(id),
                Command::UploadTexture {
                    id,
                    height,
                    address,
                    length,
                    ..
                } if seen.atlas == Some(id) => {
                    seen.rows += height;
                    // (white texels, the coverage their alpha)
                    if let Some(texels) = view.bytes(address, length) {
                        let inked = texels
                            .as_chunks::<4>()
                            .0
                            .iter()
                            .filter(|texel| texel[3] > 0)
                            .count() as u64;
                        seen.inked += inked;
                        if self.pressed {
                            seen.inked_after += inked;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Escape's Windows virtual key (the pause menu's, in play)
const VK_ESCAPE: u8 = 0x1B;

/// what the atlas saw over `frames` frames of the game at 4K, from the main
/// menu or from `commands`, Escape pressed `escape_at` ticks into the map
/// and the frames counted from then if given, each
/// of `settings` set first; None (skipped) without game.wasm or `map`
fn atlas_at_4k(
    map: &str,
    commands: Option<&str>,
    frames: u32,
    escape_at: Option<u32>,
    settings: &[(&str, &str)],
) -> Result<Option<Seen>, Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps").join(map).is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/{map}");
        return Ok(None);
    };
    let saves = std::env::temp_dir().join(format!("opence-text-{}-{map}", std::process::id()));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (3840, 2160))?;
    if let Some(commands) = commands {
        game.set_init_commands(commands);
    }
    for (name, value) in settings {
        assert!(game.set_setting(name, value), "no setting {name}");
    }
    let seen = Arc::new(Mutex::new(Seen::default()));
    game.add_plugin(Box::new(Atlas {
        seen: Arc::clone(&seen),
        escape_at,
        pressed: false,
        held: 6,
        frames,
        controls: game.controls(),
        stop: game.stopper(),
    }));
    // (the menus and the map have no end of their own; the deadline is for
    // a stuck run)
    game.set_deadline(Some(Instant::now() + Duration::from_secs(300)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let seen = *seen.lock().unwrap_or_else(PoisonError::into_inner);
    eprintln!("text: {seen:?}");
    Ok(Some(seen))
}

#[test]
fn the_main_menus_text_is_drawn_from_its_fonts_at_4k() -> Result<(), Error> {
    // (the game's own text: the Halo UI plugin's redraw switched off)
    let Some(seen) = atlas_at_4k(
        "ui.map",
        None,
        300,
        None,
        &[("display.redraw_menus", "false")],
    )?
    else {
        return Ok(());
    };
    assert!(seen.atlas.is_some(), "no text atlas was made");
    assert!(seen.rows > 0, "nothing was uploaded to the atlas");
    // (the menu's strings at 4.5 pixels a unit: thousands of inked texels)
    assert!(
        seen.inked > 10_000,
        "the atlas has only {} inked texels",
        seen.inked
    );
    Ok(())
}

#[test]
fn the_pause_menus_text_is_drawn_from_its_fonts_at_4k() -> Result<(), Error> {
    let Some(seen) = atlas_at_4k(
        "bloodgulch.map",
        Some("map_name bloodgulch\n"),
        90,
        Some(60),
        &[("display.redraw_menus", "false")],
    )?
    else {
        return Ok(());
    };
    assert!(seen.atlas.is_some(), "no text atlas was made in the map");
    // (the map draws no text until the pause menu opens)
    assert!(
        seen.inked_after > 0,
        "the pause menu drew no new glyphs ({seen:?})"
    );
    Ok(())
}

#[test]
fn the_main_menus_text_is_drawn_from_its_bitmaps_with_the_setting_off() -> Result<(), Error> {
    let Some(seen) = atlas_at_4k(
        "ui.map",
        None,
        300,
        None,
        &[("display.high_res_text", "false")],
    )?
    else {
        return Ok(());
    };
    assert!(
        seen.atlas.is_none(),
        "the atlas was made with the setting off ({seen:?})"
    );
    Ok(())
}
