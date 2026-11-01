//! The chant's file: made into the cache from the player's executable and
//! taken from there afterwards; nothing without an executable or a tone.

use std::path::Path;

use opence_ffi::loading::{chant_tone, chant_wave, CHANT_FILE};

fn folder(name: &str) -> String {
    let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&folder);
    folder.to_string_lossy().into_owned()
}

#[test]
fn without_an_executable_there_is_no_chant() {
    let cache = folder("chant_none");
    assert_eq!(
        chant_wave("/nonexistent/default.xbe".into(), cache.clone()),
        None
    );
    let fake = Path::new(env!("CARGO_TARGET_TMPDIR")).join("not_an.xbe");
    assert!(std::fs::write(&fake, b"XBEH and nothing else").is_ok());
    assert_eq!(
        chant_wave(fake.to_string_lossy().into_owned(), cache.clone()),
        None
    );
    assert!(!Path::new(&cache).join(CHANT_FILE).exists());
}

#[test]
fn the_chant_is_made_from_the_players_executable_into_the_cache() {
    let xbe = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../Haloce_dump/default.xbe");
    if !xbe.exists() {
        return; // (no executable here)
    }
    let cache = folder("chant_made");
    let made = chant_wave(xbe.to_string_lossy().into_owned(), cache.clone());
    let wave = Path::new(&cache).join(CHANT_FILE);
    assert_eq!(made.as_deref(), Some(&*wave.to_string_lossy()));
    let length = std::fs::metadata(&wave).map(|meta| meta.len()).unwrap_or(0);
    assert_eq!(length, 44 + 2 * 511 * 64);
    // taken from the cache afterwards, without the executable
    assert_eq!(
        chant_wave("/nonexistent/default.xbe".into(), cache).as_deref(),
        Some(&*wave.to_string_lossy())
    );
}

#[test]
fn the_game_takes_the_tone_from_the_players_executable_beside_the_data() {
    // none in a folder with no executable, or with one that has no tone
    let empty = folder("chant_tone_none");
    assert!(std::fs::create_dir_all(&empty).is_ok());
    assert_eq!(chant_tone(Path::new(&empty)), None);
    assert!(std::fs::write(Path::new(&empty).join("default.xbe"), b"XBEH").is_ok());
    assert_eq!(chant_tone(Path::new(&empty)), None);
    // the player's, beside assets/
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets");
    if !data.join("../Haloce_dump/default.xbe").exists() {
        return; // (no executable here)
    }
    let tone = chant_tone(&data);
    assert!(tone.is_some_and(|tone| tone.iter().any(|&byte| byte != 0)));
}

/// the sound mixed before the game's first tick, while the map loads
/// behind the loading screen: its energy, summed over the samples; the
/// game is stopped at that first tick, the answer in
struct Loading {
    energy: std::sync::Arc<std::sync::Mutex<(f64, bool)>>,
    stop: opence_runtime::Stopper,
}

impl opence_runtime::plugins::Plugin for Loading {
    fn name(&self) -> &str {
        "loading"
    }

    fn hooks(&self) -> &[opence_runtime::plugins::Hook] {
        &[
            opence_runtime::plugins::Hook::Tick,
            opence_runtime::plugins::Hook::Sound,
        ]
    }

    fn on(
        &mut self,
        event: &opence_runtime::plugins::Event<'_>,
        _view: &opence_runtime::plugins::View<'_>,
    ) {
        let Ok(mut energy) = self.energy.lock() else {
            return;
        };
        match event {
            opence_runtime::plugins::Event::Tick => {
                energy.1 = true;
                self.stop.stop();
            }
            opence_runtime::plugins::Event::Sound(samples) if !energy.1 => {
                energy.0 += samples
                    .iter()
                    .map(|&sample| f64::from(sample) * f64::from(sample))
                    .sum::<f64>();
            }
            _ => {}
        }
    }
}

/// the energy mixed while Blood Gulch loads, with `tone` as the chant's
fn loading_energy(
    tone: Option<&[u8; opence_runtime::TONE_BYTES]>,
) -> Result<f64, opence_runtime::Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let wasm = std::fs::read(root.join("build/engine/wasm/game.wasm"))
        .map_err(|error| opence_runtime::Error::Setup(error.to_string()))?;
    let saves = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("chant_saves_{}", u8::from(tone.is_some())));
    let _ = std::fs::create_dir_all(&saves);
    let runtime = opence_runtime::Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &saves, (640, 480))?;
    assert!(game.set_tone(tone)?, "game.wasm exports no tone");
    game.set_init_commands("map_name bloodgulch\n");
    let energy = std::sync::Arc::new(std::sync::Mutex::new((0.0, false)));
    game.add_plugin(Box::new(Loading {
        energy: std::sync::Arc::clone(&energy),
        stop: game.stopper(),
    }));
    // (stopped at the first tick; the deadline is for a stuck load)
    game.set_deadline(Some(
        std::time::Instant::now() + std::time::Duration::from_secs(300),
    ));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&saves);
    match ran {
        Err(opence_runtime::Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let (energy, ticked) = energy.lock().map(|energy| *energy).unwrap_or_default();
    assert!(ticked, "the map never finished loading");
    Ok(energy)
}

#[test]
fn the_loading_screen_chants_with_the_players_tone_and_is_silent_without() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let data = root.join("assets");
    let (true, true, Some(tone)) = (
        root.join("build/engine/wasm/game.wasm").is_file(),
        data.join("maps/bloodgulch.map").is_file(),
        chant_tone(&data),
    ) else {
        eprintln!("skipped: needs game.wasm, assets/maps and the player's xbe");
        return;
    };
    let heard = loading_energy(Some(&tone));
    let silent = loading_energy(None);
    eprintln!("loading: {heard:?} with the tone, {silent:?} without");
    let (Ok(heard), Ok(silent)) = (heard, silent) else {
        panic!("the game did not run");
    };
    assert!(heard > 1.0, "the chant was not heard: {heard}");
    assert!(
        silent < heard / 100.0,
        "{silent} without the tone, {heard} with it"
    );
}
