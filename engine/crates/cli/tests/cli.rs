//! The `opence` tool against a control socket: a thread here stands in for
//! the game's console, answering each command line as the runtime's
//! control server does (`{"ok": true, "text": ...}`, or a refusal).

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use opence_cli::{parse_size, Connection, Error};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

/// A socket in a folder of its own, answered by [`console`].
struct Standing {
    socket: PathBuf,
}

impl Standing {
    fn start(name: &str) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Self::answered(name, console)
    }

    /// A socket answered by `answer`, a thread for each connection.
    fn answered(
        name: &str,
        answer: impl Fn(std::os::unix::net::UnixStream) + Clone + Send + 'static,
    ) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let folder = std::env::temp_dir().join(format!("ocl{}{name}", std::process::id()));
        std::fs::create_dir_all(&folder)?;
        let socket = folder.join("c.sock");
        let _ = std::fs::remove_file(&socket);
        let listener = UnixListener::bind(&socket)?;
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                let answer = answer.clone();
                std::thread::spawn(move || answer(stream));
            }
        });
        Ok(Self { socket })
    }

    fn opence(&self, arguments: &[&str]) -> std::io::Result<std::process::Output> {
        Command::new(env!("CARGO_BIN_EXE_opence"))
            .arg("--socket")
            .arg(&self.socket)
            .args(arguments)
            .output()
    }
}

impl Drop for Standing {
    fn drop(&mut self) {
        if let Some(folder) = self.socket.parent() {
            let _ = std::fs::remove_dir_all(folder);
        }
    }
}

/// The stand-in console: a command line comes back as `ran <line>`, but
/// `time 99` is refused, as is any structured request.
fn console(stream: std::os::unix::net::UnixStream) {
    let Ok(mut writer) = stream.try_clone() else {
        return;
    };
    for line in BufReader::new(stream).lines() {
        let Ok(line) = line else { return };
        let reply = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(serde_json::Value::String(command)) if command != "time 99" => {
                serde_json::json!({"ok": true, "text": format!("ran {command}")})
            }
            Ok(serde_json::Value::String(command)) => {
                serde_json::json!({"ok": false, "error": format!("{command}: out of range")})
            }
            _ => serde_json::json!({"ok": false, "error": "command lines only"}),
        };
        if writeln!(writer, "{reply}").is_err() {
            return;
        }
    }
}

/// The stand-in game's input: every structured request is kept in `heard`
/// and accepted, `focus` answers with the main menu, `open_screen` with the
/// screen it was asked for, and `setting` with the value it was given, or
/// "stock".
fn listening(
    heard: Arc<Mutex<Vec<serde_json::Value>>>,
) -> impl Fn(std::os::unix::net::UnixStream) + Clone + Send {
    move |stream| {
        let Ok(mut writer) = stream.try_clone() else {
            return;
        };
        for line in BufReader::new(stream).lines() {
            let Ok(line) = line else { return };
            let Ok(request) = serde_json::from_str::<serde_json::Value>(&line) else {
                return;
            };
            let reply = if request["request"] == "focus" {
                serde_json::json!({"ok": true, "screen": "main menu", "widget": "campaign", "selected": 0})
            } else if request["request"] == "open_screen" {
                serde_json::json!({"ok": true, "screen": request["name"]})
            } else if request["request"] == "setting" {
                let value = request.get("value").cloned().unwrap_or("stock".into());
                serde_json::json!({"ok": true, "name": request["name"], "value": value})
            } else {
                serde_json::json!({"ok": true})
            };
            if let Ok(mut heard) = heard.lock() {
                heard.push(request);
            }
            if writeln!(writer, "{reply}").is_err() {
                return;
            }
        }
    }
}

fn text(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn lines_are_run_and_refusals_fail() -> Result {
    let console = Standing::start("a")?;
    let ran = console.opence(&["run", "game_speed", "2"])?;
    assert!(ran.status.success(), "{ran:?}");
    assert_eq!(text(&ran).trim(), "ran game_speed 2");
    let json = console.opence(&["run", "--json", "game_speed 1"])?;
    let reply: serde_json::Value = serde_json::from_str(text(&json).trim())?;
    assert_eq!(reply["text"], "ran game_speed 1");

    let refused = console.opence(&["run", "time", "99"])?;
    assert_eq!(refused.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&refused.stderr).contains("99"));
    // (the game's console has no state of its own to report)
    assert_eq!(console.opence(&["state"])?.status.code(), Some(1));
    Ok(())
}

#[test]
fn wrong_usage_and_no_engine_exit_two() -> Result {
    let console = Standing::start("b")?;
    for wrong in [
        &[][..],
        &["run"][..],
        &["teleport_everything"][..],
        &["frame"][..],
        &["frame", "a.png", "--size", "big"][..],
        &["watch", "--backlog", "many"][..],
        &["script"][..],
    ] {
        assert_eq!(console.opence(wrong)?.status.code(), Some(2), "{wrong:?}");
    }
    let nowhere = Command::new(env!("CARGO_BIN_EXE_opence"))
        .args(["--socket", "/nonexistent/c.sock", "state"])
        .output()?;
    assert_eq!(nowhere.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&nowhere.stderr).contains("--headless"));
    assert_eq!(parse_size("640x360")?, (640, 360));
    assert!(matches!(parse_size("640"), Err(Error::Usage(_))));
    Ok(())
}

#[test]
fn the_shell_runs_every_piped_line() -> Result {
    let console = Standing::start("c")?;
    let mut shell = Command::new(env!("CARGO_BIN_EXE_opence"))
        .arg("--socket")
        .arg(&console.socket)
        .arg("shell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    shell
        .stdin
        .take()
        .ok_or("no stdin")?
        .write_all(b"game_speed 1\n# nothing\ntime 99\ngame_speed 2\n")?;
    let output = shell.wait_with_output()?;
    // (a refusal counted, the lines after it still run)
    assert_eq!(output.status.code(), Some(1));
    let out = text(&output);
    assert!(out.contains("error:"), "{out}");
    assert!(out.contains("ran game_speed 2"), "{out}");

    let mut connection = Connection::connect(&console.socket)?;
    let mut out = Vec::new();
    opence_cli::run(&mut connection, "cheat_all_weapons", false, &mut out)?;
    assert_eq!(String::from_utf8(out)?, "ran cheat_all_weapons\n");
    Ok(())
}

#[test]
fn the_game_is_driven_by_keys_the_pointer_and_clicks() -> Result {
    let heard = Arc::new(Mutex::new(Vec::new()));
    let game = Standing::answered("d", listening(Arc::clone(&heard)))?;
    for arguments in [
        &["key", "38"][..],
        &["key", "13", "--down"][..],
        &["key", "13", "--up"][..],
        &["pointer", "200", "120.5"][..],
        &["click", "10", "20"][..],
        &["click", "10", "20", "--button", "3"][..],
        &["wheel", "-2"][..],
    ] {
        let done = game.opence(arguments)?;
        assert!(done.status.success(), "{arguments:?} {done:?}");
        assert_eq!(text(&done), "", "{arguments:?}");
    }
    let focus = game.opence(&["focus"])?;
    assert!(focus.status.success(), "{focus:?}");
    let reply: serde_json::Value = serde_json::from_str(text(&focus).trim())?;
    assert_eq!(reply["screen"], "main menu");
    assert!(reply.get("ok").is_none());

    let Ok(heard) = heard.lock() else {
        panic!("the stand-in game panicked");
    };
    let expected = [
        serde_json::json!({"request": "key", "vk": 38, "down": true}),
        serde_json::json!({"request": "key", "vk": 38, "down": false}),
        serde_json::json!({"request": "key", "vk": 13, "down": true}),
        serde_json::json!({"request": "key", "vk": 13, "down": false}),
        serde_json::json!({"request": "pointer", "x": 200.0, "y": 120.5}),
        serde_json::json!({"request": "click", "x": 10.0, "y": 20.0}),
        serde_json::json!({"request": "click", "x": 10.0, "y": 20.0, "button": 3}),
        serde_json::json!({"request": "wheel", "notches": -2}),
        serde_json::json!({"request": "focus"}),
    ];
    assert_eq!(heard.as_slice(), &expected[..]);
    Ok(())
}

#[test]
fn driving_takes_numbers_only() -> Result {
    let console = Standing::start("e")?;
    for wrong in [
        &["key"][..],
        &["key", "enter"][..],
        &["key", "0"][..],
        &["key", "13", "--sideways"][..],
        &["pointer", "1"][..],
        &["pointer", "a", "b"][..],
        &["click", "1", "2", "--button"][..],
        &["click", "1", "2", "--button", "9"][..],
        &["wheel"][..],
        &["wheel", "up"][..],
        &["focus", "now"][..],
        &["setting"][..],
        &["setting", "lighting.bounce", "baked", "now"][..],
        &["ui"][..],
        &["ui", "open"][..],
        &["ui", "open", "a", "b"][..],
        &["ui", "close", "a"][..],
    ] {
        assert_eq!(console.opence(wrong)?.status.code(), Some(2), "{wrong:?}");
    }
    // (the stand-in console refuses structured requests)
    assert_eq!(console.opence(&["wheel", "1"])?.status.code(), Some(1));
    Ok(())
}

#[test]
fn a_setting_is_read_or_written_while_the_game_runs() -> Result {
    let heard = Arc::new(Mutex::new(Vec::new()));
    let game = Standing::answered("s", listening(Arc::clone(&heard)))?;
    let read = game.opence(&["setting", "lighting.bounce"])?;
    assert!(read.status.success(), "{read:?}");
    let reply: serde_json::Value = serde_json::from_str(text(&read).trim())?;
    assert_eq!(
        reply,
        serde_json::json!({"name": "lighting.bounce", "value": "stock"})
    );
    let written = game.opence(&["setting", "lighting.bounce", "baked"])?;
    let reply: serde_json::Value = serde_json::from_str(text(&written).trim())?;
    assert_eq!(reply["value"], "baked");
    let Ok(heard) = heard.lock() else {
        panic!("the stand-in game panicked");
    };
    assert_eq!(
        heard.as_slice(),
        &[
            serde_json::json!({"request": "setting", "name": "lighting.bounce"}),
            serde_json::json!({"request": "setting", "name": "lighting.bounce", "value": "baked"}),
        ][..]
    );
    Ok(())
}

#[test]
fn a_menu_screen_is_opened_by_its_name() -> Result {
    let heard = Arc::new(Mutex::new(Vec::new()));
    let game = Standing::answered("u", listening(Arc::clone(&heard)))?;
    let name = "main_menu/settings_select/player_setup/player_profile_edit/video_settings/video_settings_screen";
    let opened = game.opence(&["ui", "open", name])?;
    assert!(opened.status.success(), "{opened:?}");
    let reply: serde_json::Value = serde_json::from_str(text(&opened).trim())?;
    assert_eq!(reply, serde_json::json!({"screen": name}));
    let Ok(heard) = heard.lock() else {
        panic!("the stand-in game panicked");
    };
    assert_eq!(
        heard.as_slice(),
        &[serde_json::json!({"request": "open_screen", "name": name})][..]
    );
    Ok(())
}
