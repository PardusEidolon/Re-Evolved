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

use opence_runtime::keyboard::KEYBOARD_EXPORT;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::widgets::WIDGET_GLOBALS_EXPORT;
use opence_runtime::{control, Error, Runtime, Stopper};

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
    let stop = game.stopper();
    std::thread::spawn(move || {
        let _ = sender.send(drive(&client, deadline - Duration::from_secs(2)));
        // (the answer is in: the game need not wait for its deadline)
        stop.stop();
    });
    // (the menus have no end of their own; the deadline is for a stuck run)
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
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

/// Every text the first player's open widgets have been given, over
/// `frames` frames; the game is ended after them.
struct Texts {
    seen: Arc<Mutex<Vec<String>>>,
    frames: u32,
    stop: Stopper,
}

impl Plugin for Texts {
    fn name(&self) -> &str {
        "the menus' texts"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Ui]
    }

    fn on(&mut self, _event: &Event<'_>, view: &View<'_>) {
        self.frames = self.frames.saturating_sub(1);
        if self.frames == 0 {
            self.stop.stop();
        }
        let Some(tree) = view
            .export(WIDGET_GLOBALS_EXPORT)
            .and_then(|globals| view.widget_tree(globals, 0))
        else {
            return;
        };
        let mut seen = self.seen.lock().unwrap_or_else(PoisonError::into_inner);
        for text in tree.widgets.into_iter().filter_map(|widget| widget.text) {
            if !seen.contains(&text) {
                seen.push(text);
            }
        }
    }
}

/// The texts of the main menu's widgets over its first `frames` frames, with
/// `display.build_number` set to `shown`; `None` without the user's data.
fn main_menu_texts(shown: bool, frames: u32) -> Result<Option<Vec<String>>, Error> {
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
    game.add_plugin(Box::new(Texts {
        seen: Arc::clone(&texts),
        frames,
        stop: game.stopper(),
    }));
    // (the deadline is for a stuck run)
    game.set_deadline(Some(Instant::now() + Duration::from_secs(300)));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let texts = texts.lock().unwrap_or_else(PoisonError::into_inner).clone();
    Ok(Some(texts))
}

#[test]
fn the_main_menu_shows_no_build_number_unless_asked() -> Result<(), Error> {
    let build_number = |texts: &[String]| texts.iter().any(|text| text.contains("01.01.14"));
    let Some(hidden) = main_menu_texts(false, 300)? else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    eprintln!("the main menu's texts: {hidden:?}");
    assert!(!build_number(&hidden), "{hidden:?}");
    // the same menu with the setting on shows it, so the reader sees it
    let shown = main_menu_texts(true, 300)?.unwrap_or_default();
    assert!(build_number(&shown), "{shown:?}");
    Ok(())
}

/// One key pressed and let go, past a few of the game's frames, then waited
/// on until the focus moves (and a moment more, for the screen's opening
/// animation), or for a second and a half at most.
fn press(stream: &mut BufReader<UnixStream>, vk: u8) {
    let before = ask(stream, r#"{"request": "focus"}"#);
    let until = Instant::now() + Duration::from_millis(1500);
    ask(
        stream,
        &format!(r#"{{"request": "key", "vk": {vk}, "down": true}}"#),
    );
    std::thread::sleep(Duration::from_millis(150));
    ask(
        stream,
        &format!(r#"{{"request": "key", "vk": {vk}, "down": false}}"#),
    );
    while Instant::now() < until {
        if ask(stream, r#"{"request": "focus"}"#) != before {
            std::thread::sleep(Duration::from_millis(300));
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// The screens the main menu's items open: each item chosen with Return,
/// its screen read, and left with Escape (the focus comes back to the
/// item), then the down arrow to the next item.
fn walk(socket: &Path, until: Instant) -> Vec<String> {
    const VK_RETURN: u8 = 0x0D;
    const VK_ESCAPE: u8 = 0x1B;
    const VK_DOWN: u8 = 0x28;
    let Ok(stream) = UnixStream::connect(socket) else {
        return Vec::new();
    };
    let mut stream = BufReader::new(stream);
    let mut screens = Vec::new();
    let note = |focus: &serde_json::Value, screens: &mut Vec<String>| {
        eprintln!("walk: {focus}");
        if let Some(screen) = focus["screen"].as_str() {
            if !screens.iter().any(|seen| seen == screen) {
                screens.push(screen.to_owned());
            }
        }
    };
    note(&shown(&mut stream, until), &mut screens);
    std::thread::sleep(Duration::from_secs(2));
    for _ in 0..5 {
        press(&mut stream, VK_RETURN);
        note(&shown(&mut stream, until), &mut screens);
        press(&mut stream, VK_ESCAPE);
        note(&shown(&mut stream, until), &mut screens);
        press(&mut stream, VK_DOWN);
        if Instant::now() >= until {
            break;
        }
    }
    screens
}

#[test]
fn each_main_menu_item_opens_its_own_screen() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let folder = std::env::temp_dir().join(format!("opence-screens-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));

    let deadline = Instant::now() + Duration::from_secs(90);
    let (sender, seen) = mpsc::channel();
    let client = socket.clone();
    let stop = game.stopper();
    std::thread::spawn(move || {
        let _ = sender.send(walk(&client, deadline - Duration::from_secs(2)));
        // (the answer is in: the game need not wait for its deadline)
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let Ok(screens) = seen.recv_timeout(Duration::from_secs(5)) else {
        panic!("the client did not finish");
    };
    eprintln!("screens: {screens:?}");
    // (the main menu and at least three screens its items open)
    assert!(screens.len() >= 4, "only {screens:?} were shown");
    Ok(())
}

/// The first screen the game shows, read on the socket, with `settings`
/// set first; `None` without the user's data.
fn first_screen(settings: &[(&str, &str)]) -> Result<Option<serde_json::Value>, Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        return Ok(None);
    };
    let folder = std::env::temp_dir().join(format!(
        "opence-first-{}-{}",
        std::process::id(),
        settings.len()
    ));
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    for (name, value) in settings {
        assert!(game.tuning().set(name, value), "no setting {name}");
    }
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));
    let deadline = Instant::now() + Duration::from_secs(30);
    let (sender, seen) = mpsc::channel();
    let client = socket.clone();
    let stop = game.stopper();
    std::thread::spawn(move || {
        let focus = UnixStream::connect(&client)
            .map(|stream| {
                shown(
                    &mut BufReader::new(stream),
                    deadline - Duration::from_secs(2),
                )
            })
            .unwrap_or_default();
        let _ = sender.send(focus);
        // (the answer is in: the game need not wait for its deadline)
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    Ok(seen.recv_timeout(Duration::from_secs(5)).ok())
}

#[test]
fn the_game_opens_the_pc_menus_with_their_settings_screens() -> Result<(), Error> {
    let Some(main) = first_screen(&[])? else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    eprintln!("the main menu: {main}");
    assert_eq!(main["ok"], true, "no screen was shown: {main}");
    // a settings screen of their own, opened by name as the dev panel does
    let video = first_screen(&[("debug.menu_open", "main_menu/settings_select/player_setup/player_profile_edit/video_settings/video_settings_screen")])?
        .unwrap_or_default();
    eprintln!("the video settings: {video}");
    assert!(
        video["screen"]
            .as_str()
            .is_some_and(|screen| screen.contains("video_settings")),
        "{video}"
    );
    Ok(())
}

/// Video Setup driven as a player drives it: the mode spinner turned one
/// step, the focus taken down to the button bar and across to OK, and OK
/// chosen; the focus at each step.
fn change_the_mode(socket: &Path, until: Instant) -> Vec<serde_json::Value> {
    const VK_RETURN: u8 = 0x0D;
    const VK_RIGHT: u8 = 0x27;
    const VK_DOWN: u8 = 0x28;
    let Ok(stream) = UnixStream::connect(socket) else {
        return Vec::new();
    };
    let mut stream = BufReader::new(stream);
    let mut seen = vec![shown(&mut stream, until)];
    std::thread::sleep(Duration::from_secs(2));
    press(&mut stream, VK_RIGHT);
    seen.push(shown(&mut stream, until));
    // (down past every row of the one Video tab to its buttons)
    for _ in 0..32 {
        let at = seen
            .last()
            .and_then(|focus| focus["widget"].as_str())
            .unwrap_or_default();
        if at.contains("button_defaults") || at.contains("button_ok") || Instant::now() >= until {
            break;
        }
        press(&mut stream, VK_DOWN);
        seen.push(shown(&mut stream, until));
    }
    press(&mut stream, VK_RIGHT);
    seen.push(shown(&mut stream, until));
    press(&mut stream, VK_RETURN);
    // (OK closes the screen, and a screen opened by name has none behind it:
    // the answer is in once the focus is off this one, or there is none)
    while Instant::now() < until {
        let focus = ask(&mut stream, r#"{"request": "focus"}"#);
        let on = focus["screen"]
            .as_str()
            .is_some_and(|screen| screen.contains("video_settings"));
        if !on {
            seen.push(focus);
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    seen
}

#[test]
fn video_setups_ok_saves_the_mode_keeps_it_and_asks_the_app_to_apply_it() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let folder = std::env::temp_dir().join(format!("opence-video-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    assert!(game.tuning().set(
        "debug.menu_open",
        "main_menu/settings_select/player_setup/player_profile_edit/video_settings/video_settings_screen"
    ));
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));
    let deadline = Instant::now() + Duration::from_secs(60);
    let (sender, steps) = mpsc::channel();
    let client = socket.clone();
    let stop = game.stopper();
    std::thread::spawn(move || {
        let _ = sender.send(change_the_mode(&client, deadline - Duration::from_secs(2)));
        // (the answer is in: the game need not wait for its deadline)
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let kept = std::fs::read_to_string(folder.join("settings.txt")).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let steps = steps
        .recv_timeout(Duration::from_secs(5))
        .unwrap_or_default();
    for step in &steps {
        eprintln!("video: {step}");
    }
    eprintln!("kept: {kept:?}");
    let applied = game.controls().take_display();
    eprintln!("applied: {applied:?}");
    // (the mode, full screen by default, turned one step to borderless or
    // windowed, written, kept for the next run and handed to the app)
    assert!(
        kept.lines().any(|line| line.starts_with("display.mode\t")),
        "{kept:?}"
    );
    assert!(
        applied.is_some(),
        "the app was not asked to apply the settings"
    );
    // (and OK went back from the screen)
    let last = steps
        .last()
        .and_then(|focus| focus["screen"].as_str())
        .unwrap_or_default();
    assert!(!last.contains("video_settings"), "still on {last}");
    Ok(())
}

/// The server browser's sort arrows as the first player's widgets show
/// them, each frame: for each visible `header_sort_arrows`, its column's
/// name and the arrow it shows ("up", "down", or "none").
struct Arrows(Arc<Mutex<Vec<String>>>);

impl Plugin for Arrows {
    fn name(&self) -> &str {
        "the server browser's sort arrows"
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
        let named = |index: usize, part: &str| {
            tree.widgets[index]
                .name
                .as_deref()
                .is_some_and(|name| name.ends_with(part))
        };
        let shown: Vec<String> = (0..tree.widgets.len())
            .filter(|&index| named(index, "header_sort_arrows") && tree.widgets[index].visible)
            .map(|index| {
                let column = tree.widgets[index]
                    .parent
                    .and_then(|parent| tree.widgets[parent].name.clone())
                    .unwrap_or_default();
                let arrow = (0..tree.widgets.len())
                    .filter(|&child| tree.widgets[child].parent == Some(index))
                    .filter(|&child| tree.widgets[child].visible)
                    .find_map(|child| {
                        if named(child, "header_sort_up_arrow") {
                            Some("up")
                        } else if named(child, "header_sort_down_arrow") {
                            Some("down")
                        } else {
                            None
                        }
                    })
                    .unwrap_or("none");
                format!("{column}:{arrow}")
            })
            .collect();
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = shown;
    }
}

/// The join screen driven as a player sorts it: the focus taken up to the
/// column titles, one chosen and chosen again; the arrows shown before,
/// after the first choice and after the second.
fn sort_twice(
    socket: &Path,
    until: Instant,
    arrows: &Arc<Mutex<Vec<String>>>,
) -> (serde_json::Value, Vec<Vec<String>>) {
    const VK_RETURN: u8 = 0x0D;
    const VK_UP: u8 = 0x26;
    let Ok(stream) = UnixStream::connect(socket) else {
        return (serde_json::Value::Null, Vec::new());
    };
    let mut stream = BufReader::new(stream);
    let seen = |arrows: &Arc<Mutex<Vec<String>>>| {
        arrows
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    };
    // (the up arrow to the column titles)
    let up_to = |stream: &mut BufReader<UnixStream>, part: &str| {
        let mut focus = shown(stream, until);
        for _ in 0..20 {
            let at = focus["widget"].as_str().unwrap_or_default();
            if at.contains(part) || Instant::now() >= until {
                break;
            }
            press(stream, VK_UP);
            focus = shown(stream, until);
        }
        focus
    };
    std::thread::sleep(Duration::from_secs(2));
    let mut states = vec![seen(arrows)];
    let focus = up_to(&mut stream, "op_sort_header");
    press(&mut stream, VK_RETURN);
    states.push(seen(arrows));
    press(&mut stream, VK_RETURN);
    states.push(seen(arrows));
    (focus, states)
}

/// From Multiplayer, Join's LAN chosen: the focus before and after.
fn lan_chosen(socket: &Path, until: Instant) -> (serde_json::Value, serde_json::Value) {
    const VK_RETURN: u8 = 0x0D;
    const VK_UP: u8 = 0x26;
    let Ok(stream) = UnixStream::connect(socket) else {
        return (serde_json::Value::Null, serde_json::Value::Null);
    };
    let mut stream = BufReader::new(stream);
    std::thread::sleep(Duration::from_secs(2));
    let mut focus = shown(&mut stream, until);
    for _ in 0..20 {
        let at = focus["widget"].as_str().unwrap_or_default();
        if at.contains("multiplayer_type_join_lan") || Instant::now() >= until {
            break;
        }
        press(&mut stream, VK_UP);
        focus = shown(&mut stream, until);
    }
    press(&mut stream, VK_RETURN);
    std::thread::sleep(Duration::from_secs(2));
    (focus, shown(&mut stream, until))
}

#[test]
fn lan_without_a_network_shows_the_games_own_error() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let folder = std::env::temp_dir().join(format!("opence-lan-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    assert!(game.tuning().set(
        "debug.menu_open",
        "main_menu/multiplayer_type_select/multiplayer_type_select_screen"
    ));
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));
    let deadline = Instant::now() + Duration::from_secs(40);
    let (sender, steps) = mpsc::channel();
    let client = socket.clone();
    let stop = game.stopper();
    std::thread::spawn(move || {
        let _ = sender.send(lan_chosen(&client, deadline - Duration::from_secs(2)));
        // (the answer is in: the game need not wait for its deadline)
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let log = String::from_utf8_lossy(game.output(2)).into_owned();
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let (before, after) = steps
        .recv_timeout(Duration::from_secs(5))
        .unwrap_or_default();
    eprintln!("lan: from {before}, then {after}");
    assert!(
        before["widget"]
            .as_str()
            .unwrap_or_default()
            .contains("join_lan"),
        "{before}"
    );
    // (the sandbox's network is an Xbox's with its cable out: no browser,
    // the game's error in its place, and no halt in the transport)
    assert!(!log.contains("EXCEPTION"), "{log}");
    assert_ne!(after["screen"], "join_game_screen", "{after}");
    assert_eq!(
        after["screen"], "error_modal_fullscreen",
        "no error shown: {after}"
    );
    Ok(())
}

#[test]
fn the_server_browsers_column_titles_sort_its_games() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let folder = std::env::temp_dir().join(format!("opence-browser-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    assert!(game.tuning().set(
        "debug.menu_open",
        "main_menu/multiplayer_type_select/join_game/join_game_screen"
    ));
    let arrows = Arc::new(Mutex::new(Vec::new()));
    game.add_plugin(Box::new(Arrows(Arc::clone(&arrows))));
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));
    let deadline = Instant::now() + Duration::from_secs(60);
    let (sender, steps) = mpsc::channel();
    let client = socket.clone();
    let stop = game.stopper();
    std::thread::spawn(move || {
        let _ = sender.send(sort_twice(
            &client,
            deadline - Duration::from_secs(2),
            &arrows,
        ));
        // (the answer is in: the game need not wait for its deadline)
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let (focus, states) = steps
        .recv_timeout(Duration::from_secs(5))
        .unwrap_or_default();
    eprintln!("browser: on {focus}, arrows {states:?}");
    let header = focus["widget"].as_str().unwrap_or_default();
    assert!(
        header.contains("op_sort_header"),
        "the column titles never took the focus: {focus}"
    );
    // (unsorted at first: no arrows; then one, under the chosen column,
    // ascending; chosen again, the same column descending)
    assert_eq!(states.first(), Some(&Vec::new()), "{states:?}");
    let [first] = states.get(1).map(Vec::as_slice).unwrap_or_default() else {
        panic!("not one column's arrows after the first choice: {states:?}");
    };
    let [second] = states.get(2).map(Vec::as_slice).unwrap_or_default() else {
        panic!("not one column's arrows after the second choice: {states:?}");
    };
    assert!(first.ends_with(":up"), "{states:?}");
    assert!(second.ends_with(":down"), "{states:?}");
    assert_eq!(
        first.split(':').next(),
        second.split(':').next(),
        "{states:?}"
    );
    Ok(())
}

/// Whether the plugins' view of the on-screen keyboard has it open, each
/// frame it changes.
struct Keyboards(Arc<Mutex<Vec<bool>>>);

impl Plugin for Keyboards {
    fn name(&self) -> &str {
        "the on-screen keyboard"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Ui]
    }

    fn on(&mut self, _event: &Event<'_>, view: &View<'_>) {
        let open = view
            .export(KEYBOARD_EXPORT)
            .and_then(|at| view.keyboard(at))
            .is_some();
        let mut seen = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if seen.last() != Some(&open) {
            seen.push(open);
        }
    }
}

/// From the main menu, Multiplayer chosen with no profile (the game asks
/// for a name on its keyboard), the keyboard cancelled, and the screen
/// left: the screens on the way.
fn name_cancelled(socket: &Path, until: Instant) -> Vec<serde_json::Value> {
    const VK_RETURN: u8 = 0x0D;
    const VK_ESCAPE: u8 = 0x1B;
    const VK_DOWN: u8 = 0x28;
    let Ok(stream) = UnixStream::connect(socket) else {
        return Vec::new();
    };
    let mut stream = BufReader::new(stream);
    std::thread::sleep(Duration::from_secs(2));
    let mut focus = shown(&mut stream, until);
    for _ in 0..8 {
        let at = focus["widget"].as_str().unwrap_or_default();
        if at.contains("multiplayer") || Instant::now() >= until {
            break;
        }
        press(&mut stream, VK_DOWN);
        focus = shown(&mut stream, until);
    }
    let mut screens = vec![focus];
    press(&mut stream, VK_RETURN);
    screens.push(shown(&mut stream, until));
    for _ in 0..2 {
        press(&mut stream, VK_ESCAPE);
        screens.push(shown(&mut stream, until));
    }
    std::thread::sleep(Duration::from_secs(2));
    screens
}

#[test]
fn the_keyboard_a_plugin_sees_closes_when_the_games_does() -> Result<(), Error> {
    let root = root();
    let (Ok(wasm), true) = (
        std::fs::read(root.join("build/engine/wasm/game.wasm")),
        root.join("assets/maps/ui.map").is_file(),
    ) else {
        eprintln!("skipped: needs build/engine/wasm/game.wasm and assets/maps/ui.map");
        return Ok(());
    };
    let folder = std::env::temp_dir().join(format!("opence-keyboard-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&folder);
    let _ = std::fs::create_dir_all(&folder);
    let socket = folder.join("control/c.sock");
    let runtime = Runtime::new(Some(&root.join("build/engine/wasm/cache")))?;
    let mut game = runtime.load(&wasm)?;
    game.prepare(&root.join("assets"), &folder, (1280, 720))?;
    assert!(game.tuning().set("display.redraw_menus", "true"));
    let opened = Arc::new(Mutex::new(Vec::new()));
    game.add_plugin(Box::new(Keyboards(Arc::clone(&opened))));
    let listener = control::bind(&socket).map_err(|error| Error::Setup(error.to_string()))?;
    control::serve(listener, control::Driven::of(&mut game));
    let deadline = Instant::now() + Duration::from_secs(50);
    let (sender, steps) = mpsc::channel();
    let client = socket.clone();
    let stop = game.stopper();
    std::thread::spawn(move || {
        let _ = sender.send(name_cancelled(&client, deadline - Duration::from_secs(2)));
        // (the answer is in: the game need not wait for its deadline)
        stop.stop();
    });
    game.set_deadline(Some(deadline));
    let ran = game.call("halo_game_main");
    let _ = std::fs::remove_dir_all(&folder);
    match ran {
        Err(Error::Deadline) => {}
        other => panic!("the game ended before it was stopped: {other:?}"),
    }
    let screens = steps
        .recv_timeout(Duration::from_secs(5))
        .unwrap_or_default();
    let opened = opened
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    eprintln!("keyboard: {opened:?} over {screens:?}");
    assert!(
        opened.contains(&true),
        "the keyboard never opened: {screens:?}"
    );
    // (the game's keyboard is drawn only while it is open, so a view kept
    // only as it is drawn stays open after it closes)
    assert_eq!(opened.last(), Some(&false), "{screens:?}");
    Ok(())
}
