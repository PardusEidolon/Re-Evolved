//! The game's own main menu, driven through the control socket as a test
//! (or `opence key`) drives it: the focus is read back, a key moves it.
//! Needs game.wasm and the user's maps (assets/maps, for ui.map); skips
//! itself without them.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use std::sync::{Arc, Mutex, PoisonError};

use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::widgets::WIDGET_GLOBALS_EXPORT;
use opence_runtime::{control, Error, Runtime};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

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

/// The focus once the game has shown a screen, waited for until `until`.
fn shown(stream: &mut BufReader<UnixStream>, until: Instant) -> serde_json::Value {
    while Instant::now() < until {
        let focus = ask(stream, r#"{"request": "focus"}"#);
        if focus["ok"] == true && !focus["widget"].is_null() {
            return focus;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    serde_json::Value::Null
}

/// What the client saw: the focus before a key, and after it.
fn drive(socket: &Path, until: Instant) -> (serde_json::Value, serde_json::Value) {
    let Ok(stream) = UnixStream::connect(socket) else {
        return (serde_json::Value::Null, serde_json::Value::Null);
    };
    let mut stream = BufReader::new(stream);
    let before = shown(&mut stream, until);
    // (a settled menu: the screen's opening animation over)
    std::thread::sleep(Duration::from_secs(2));
    let before = match shown(&mut stream, until) {
        serde_json::Value::Null => before,
        settled => settled,
    };
    // the down arrow, held past a few of the game's frames
    ask(&mut stream, r#"{"request": "key", "vk": 40, "down": true}"#);
    std::thread::sleep(Duration::from_millis(150));
    ask(
        &mut stream,
        r#"{"request": "key", "vk": 40, "down": false}"#,
    );
    std::thread::sleep(Duration::from_millis(500));
    (before, shown(&mut stream, until))
}

#[test]
fn the_down_arrow_moves_the_main_menus_focus() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let folder = std::env::temp_dir().join(format!("opence-menus-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    // (the socket's own folder, made private by the bind)
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));

    let deadline = Instant::now() + Duration::from_secs(40);
    let (sender, seen) = mpsc::channel();
    let client = socket.clone();
    std::thread::spawn(move || {
        let _ = sender.send(drive(&client, deadline - Duration::from_secs(2)));
    });
    // (ended by its deadline: the menus have no end of their own)
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let Ok((before, after)) = seen.recv_timeout(Duration::from_secs(5)) else {
        panic!("the client did not finish");
    };
    eprintln!("menus: {before} then {after}");
    assert_eq!(before["ok"], true, "no screen was shown: {before}");
    assert_eq!(after["screen"], before["screen"], "{before} then {after}");
    assert!(
        after["widget"] != before["widget"] || after["selected"] != before["selected"],
        "the focus stayed: {before} then {after}"
    );
    Ok(())
}

/// Every text the first player's open widgets have been given.
struct Texts(Arc<Mutex<Vec<String>>>);

impl Plugin for Texts {
    fn name(&self) -> &str {
        "the menus' texts"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Ui]
    }

    fn on(&mut self, _event: &Event<'_>, view: &View<'_>) {
        let Some(tree) = view
            .export(WIDGET_GLOBALS_EXPORT)
            .and_then(|globals| view.widget_tree(globals, 0))
        else {
            return;
        };
        let mut seen = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        for text in tree.widgets.into_iter().filter_map(|widget| widget.text) {
            if !seen.contains(&text) {
                seen.push(text);
            }
        }
    }
}

/// The texts of the main menu's widgets over its first `seconds`, with
/// `display.build_number` set to `shown`; `None` without the user's data.
fn main_menu_texts(shown: bool, seconds: u64) -> Result<Option<Vec<String>>, Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        return Ok(None);
    };
    let folder =
        std::env::temp_dir().join(format!("opence-menu-texts-{}-{shown}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    assert!(game
        .tuning()
        .set("display.build_number", if shown { "true" } else { "false" }));
    let texts = Arc::new(Mutex::new(Vec::new()));
    game.add_plugin(Box::new(Texts(Arc::clone(&texts))));
    game.set_deadline(Some(Instant::now() + Duration::from_secs(seconds)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before its deadline: {other:?}"),
    }
    let texts = texts.lock().unwrap_or_else(PoisonError::into_inner).clone();
    Ok(Some(texts))
}

#[test]
fn the_main_menu_shows_no_build_number_unless_asked() -> Result<(), Error> {
    let build_number = |texts: &[String]| texts.iter().any(|text| text.contains("01.01.14"));
    let Some(hidden) = main_menu_texts(false, 20)? else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    eprintln!("the main menu's texts: {hidden:?}");
    assert!(!build_number(&hidden), "{hidden:?}");
    // the same menu with the setting on shows it, so the reader sees it
    let shown = main_menu_texts(true, 20)?.unwrap_or_default();
    assert!(build_number(&shown), "{shown:?}");
    Ok(())
}
