//! The plugin on each screen the main menu's items open: each walked to
//! through the control socket as a player would (Return on an item, its
//! screen read, Escape back, the down arrow to the next), and the scene
//! the plugin lays out over it kept. Needs game.wasm and the user's maps
//! (assets/maps, for ui.map); skips itself without them.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
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
fn walk(socket: &Path, scene: &Mutex<Scene>, until: Instant) -> Vec<(String, Scene)> {
    const VK_RETURN: u8 = 0x0D;
    const VK_ESCAPE: u8 = 0x1B;
    const VK_DOWN: u8 = 0x28;
    let Ok(stream) = UnixStream::connect(socket) else {
        return Vec::new();
    };
    let mut stream = BufReader::new(stream);
    let mut seen: Vec<(String, Scene)> = Vec::new();
    let note = |stream: &mut BufReader<UnixStream>, seen: &mut Vec<(String, Scene)>| {
        let Some(name) = screen(stream, until) else {
            return;
        };
        let laid_out = scene.lock().unwrap_or_else(PoisonError::into_inner).clone();
        if !seen.iter().any(|(screen, _)| *screen == name) {
            seen.push((name, laid_out));
        }
    };
    note(&mut stream, &mut seen);
    std::thread::sleep(Duration::from_secs(2));
    for _ in 0..4 {
        press(&mut stream, VK_RETURN);
        note(&mut stream, &mut seen);
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

    let deadline = Instant::now() + Duration::from_secs(90);
    let (sender, walked) = mpsc::channel();
    let client = socket.clone();
    let shared = Arc::clone(&scene);
    std::thread::spawn(move || {
        let _ = sender.send(walk(&client, &shared, deadline - Duration::from_secs(2)));
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
        if name != "the_main_menu" {
            assert!(scene.shapes.len() >= 4, "{name} has no frame: {scene:?}");
        }
    }
    Ok(())
}
