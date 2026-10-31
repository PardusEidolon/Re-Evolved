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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_render::{Command, PixelFormat};
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::{Error, Runtime};

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

/// watches each frame's commands for the atlas and its uploads
struct Atlas(Arc<Mutex<Seen>>, Arc<AtomicBool>);

impl Plugin for Atlas {
    fn name(&self) -> &str {
        "the text atlas"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Render]
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        let Event::Render(commands) = event else {
            return;
        };
        let mut seen = self.0.lock().unwrap_or_else(PoisonError::into_inner);
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
                        if self.1.load(Ordering::Relaxed) {
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

/// what the atlas saw over `seconds` of the game at 4K, from the main menu
/// or from `commands`, Escape pressed `escape_at` seconds in if given;
/// None (skipped) without game.wasm or `map`
fn atlas_at_4k(
    map: &str,
    commands: Option<&str>,
    seconds: u64,
    escape_at: Option<u64>,
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
    let seen = Arc::new(Mutex::new(Seen::default()));
    let pressed = Arc::new(AtomicBool::new(false));
    game.add_plugin(Box::new(Atlas(Arc::clone(&seen), Arc::clone(&pressed))));
    let controls = game.controls();
    let presser = escape_at.map(|at| {
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(at));
            pressed.store(true, Ordering::Relaxed);
            controls.key(VK_ESCAPE, 0x1B, 0, true);
            std::thread::sleep(Duration::from_millis(200));
            controls.key(VK_ESCAPE, 0x1B, 0, false);
        })
    });
    // (ended by its deadline: the menus and the map have no end of their own)
    game.set_deadline(Some(Instant::now() + Duration::from_secs(seconds)));
    let ran = game.call("halo_game_main");
    if let Some(presser) = presser {
        let _ = presser.join();
    }
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let seen = *seen.lock().unwrap_or_else(PoisonError::into_inner);
    eprintln!("text: {seen:?}");
    Ok(Some(seen))
}

#[test]
fn the_main_menus_text_is_drawn_from_its_fonts_at_4k() -> Result<(), Error> {
    let Some(seen) = atlas_at_4k("ui.map", None, 40, None)? else {
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
        75,
        Some(50),
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
