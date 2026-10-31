//! The plugin on each screen the main menu's items open: each walked to
//! through the control socket as a player would (Return on an item, its
//! screen read, Escape back, the down arrow to the next), and the scene
//! the plugin lays out over it kept. Needs game.wasm and the user's maps
//! (assets/maps, for ui.map); skips itself without them.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_halo_ui::{Redraw, Scene, SETTING};
use opence_runtime::{control, Error, Runtime};

/// One request on the socket and its reply.
fn ask(stream: &mut BufReader<UnixStream>, request: &str) -> serde_json::Value {
    if writeln!(stream.get_mut(), "{request}").is_err() {
        return serde_json::Value::Null;
    }
    let mut line = String::new();
    if stream.read_line(&mut line).is_err() {
        return serde_json::Value::Null;
    }
    serde_json::from_str(&line).unwrap_or_default()
}

/// The open screen's name, waited for until `until`.
fn screen(stream: &mut BufReader<UnixStream>, until: Instant) -> Option<String> {
    while Instant::now() < until {
        let focus = ask(stream, r#"{"request": "focus"}"#);
        if let Some(screen) = focus["screen"].as_str() {
            return Some(screen.to_owned());
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    None
}

/// One key pressed and let go, past a few of the game's frames, and the
/// screen's opening animation let finish.
fn press(stream: &mut BufReader<UnixStream>, vk: u8) {
    ask(
        stream,
        &format!(r#"{{"request": "key", "vk": {vk}, "down": true}}"#),
    );
    std::thread::sleep(Duration::from_millis(150));
    ask(
        stream,
        &format!(r#"{{"request": "key", "vk": {vk}, "down": false}}"#),
    );
    std::thread::sleep(Duration::from_millis(1500));
}

/// Each screen walked to, with the scene the plugin laid out over it.
/// The plugin's scene once it has drawn one other than `before`: the
/// game's focus moves a frame or more before the plugin draws the new
/// screen (or `before`'s, at `until`).
fn drawn_after(scene: &Mutex<Scene>, before: &Scene, until: Instant) -> Scene {
    loop {
        let now = scene.lock().unwrap_or_else(PoisonError::into_inner).clone();
        if now != *before || Instant::now() >= until {
            return now;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn walk(socket: &Path, scene: &Mutex<Scene>, until: Instant) -> Vec<(String, Scene)> {
    const VK_RETURN: u8 = 0x0D;
    const VK_ESCAPE: u8 = 0x1B;
    const VK_DOWN: u8 = 0x28;
    let Ok(stream) = UnixStream::connect(socket) else {
        return Vec::new();
    };
    let mut stream = BufReader::new(stream);
    let mut seen: Vec<(String, Scene)> = Vec::new();
    let note =
        |stream: &mut BufReader<UnixStream>, seen: &mut Vec<(String, Scene)>, before: &Scene| {
            let Some(name) = screen(stream, until) else {
                return;
            };
            let laid_out = drawn_after(scene, before, until);
            if !seen.iter().any(|(screen, _)| *screen == name) {
                seen.push((name, laid_out));
            }
        };
    note(&mut stream, &mut seen, &Scene::default());
    std::thread::sleep(Duration::from_secs(2));
    for _ in 0..4 {
        let before = scene.lock().unwrap_or_else(PoisonError::into_inner).clone();
        press(&mut stream, VK_RETURN);
        note(&mut stream, &mut seen, &before);
        press(&mut stream, VK_ESCAPE);
        press(&mut stream, VK_DOWN);
        if Instant::now() >= until {
            break;
        }
    }
    seen
}

#[test]
fn each_screen_the_main_menu_opens_is_laid_out() -> Result<(), Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let folder =
        std::env::temp_dir().join(format!("opence-halo-ui-screens-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    let tuning = game.tuning();
    assert!(tuning.set(SETTING, "true"));
    let scene = Arc::new(Mutex::new(Scene::default()));
    game.add_plugin(Box::new(Redraw::new(Arc::clone(&scene), tuning)));
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));

    // (a long deadline, for a loaded machine: the walk ends the game
    // when it is done)
    let deadline = Instant::now() + Duration::from_secs(300);
    let (sender, walked) = mpsc::channel();
    let client = socket.clone();
    let shared = Arc::clone(&scene);
    let stop = game.stopper();
    std::thread::spawn(move || {
        let _ = sender.send(walk(&client, &shared, deadline - Duration::from_secs(2)));
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let Ok(screens) = walked.recv_timeout(Duration::from_secs(5)) else {
        panic!("the client did not finish");
    };
    for (name, scene) in &screens {
        let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
        eprintln!("{name}: {} shapes, texts {texts:?}", scene.shapes.len());
    }
    // the main menu and at least three screens its items open
    assert!(
        screens.len() >= 4,
        "only {:?} were shown",
        screens.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    for (name, scene) in &screens {
        // every screen has text of the plugin's; every one but the main
        // menu its title's and footer's frame as well
        assert!(!scene.texts.is_empty(), "{name} has no text");
        if name != "the_main_menu" && name != "main_menu" {
            assert!(scene.shapes.len() >= 4, "{name} has no frame: {scene:?}");
        }
    }
    Ok(())
}

/// The scene the plugin lays out at the end of `keys`, pressed from the
/// main menu as a player presses them, with the screen each key led to;
/// `None` without the user's data.
fn walked_to(keys: &'static [u8]) -> Result<Option<(Vec<String>, Scene)>, Error> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        return Ok(None);
    };
    // (a folder, and so a control socket, per walk: the tests run at once)
    static WALKS: AtomicUsize = AtomicUsize::new(0);
    let folder = std::env::temp_dir().join(format!(
        "opence-halo-ui-walked-{}-{}",
        std::process::id(),
        WALKS.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    let tuning = game.tuning();
    assert!(tuning.set(SETTING, "true"));
    let scene = Arc::new(Mutex::new(Scene::default()));
    game.add_plugin(Box::new(Redraw::new(Arc::clone(&scene), tuning)));
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));

    // (a long deadline, for a loaded machine: the walk ends the game
    // when it is done)
    let deadline = Instant::now() + Duration::from_secs(300);
    let (sender, seen) = mpsc::channel();
    let client = socket.clone();
    let shared = Arc::clone(&scene);
    let stop = game.stopper();
    std::thread::spawn(move || {
        let Ok(stream) = UnixStream::connect(&client) else {
            return;
        };
        let mut stream = BufReader::new(stream);
        let until = deadline - Duration::from_secs(2);
        let mut screens = Vec::new();
        screens.extend(screen(&mut stream, until));
        std::thread::sleep(Duration::from_secs(2));
        let mut before = Scene::default();
        for &key in keys {
            before = shared
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone();
            press(&mut stream, key);
            screens.extend(screen(&mut stream, until));
        }
        let laid_out = drawn_after(&shared, &before, until);
        let _ = sender.send((screens, laid_out));
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    Ok(seen.recv_timeout(Duration::from_secs(5)).ok())
}

const VK_RETURN: u8 = 0x0D;
const VK_DOWN: u8 = 0x28;

/// Keyboard and mouse only: the profile's settings have no Gamepads row
/// (menu_tags.c hides it, keeping the rows' places for their descriptions).
#[test]
fn the_profile_settings_have_no_gamepads_row() -> Result<(), Error> {
    // (Settings, and a profile made, which opens its settings)
    let keys = &[VK_DOWN, VK_DOWN, VK_DOWN, VK_RETURN, VK_RETURN];
    let Some((screens, scene)) = walked_to(keys)? else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
    eprintln!("walked through {screens:?} to {texts:?}");
    assert!(texts.contains(&"MOUSE SETUP"), "{texts:?}");
    assert!(
        !texts.iter().any(|text| text.contains("GAMEPAD")),
        "{texts:?}"
    );
    Ok(())
}

/// A settings screen's rows show their values: Audio Setup's output device
/// and volumes, drawn by the plugin beside their labels.
#[test]
fn the_audio_settings_show_their_values() -> Result<(), Error> {
    // (Settings, a profile made, and its fourth row, Audio Setup)
    let keys = &[
        VK_DOWN, VK_DOWN, VK_DOWN, VK_RETURN, VK_RETURN, VK_DOWN, VK_DOWN, VK_DOWN, VK_RETURN,
    ];
    let Some((screens, scene)) = walked_to(keys)? else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let texts: Vec<&str> = scene.texts.iter().map(|text| text.text.as_str()).collect();
    eprintln!("walked through {screens:?} to {texts:?}");
    assert_eq!(
        screens.last().map(String::as_str),
        Some("audio_settings_screen")
    );
    assert!(texts.contains(&"SYSTEM DEFAULT"), "{texts:?}");
    assert!(texts.contains(&"10"), "{texts:?}");
    Ok(())
}
