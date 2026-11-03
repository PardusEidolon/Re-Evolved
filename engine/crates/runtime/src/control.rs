//! The dev crate's control socket answered by the game's own console: each
//! command line a client sends is handed to the telnet console
//! ([`Console::ask`]) and its reply sent back as `{"ok": true, "text": ...}`,
//! so `opence run <line>` reaches the original's script compiler, as a line
//! typed at its telnet console does.
//! The console is one session, so the lines are asked one at a time, in the
//! order they arrive.
//!
//! Structured requests drive the keyboard and mouse as the window would
//! (`key`, `pointer`, `click`, `wheel`) and read where the menus' focus is
//! (`focus`), so tests can walk the menus as a player does; `setting` reads
//! a setting, or writes it with `value`, while the game runs; `shell` opens
//! or closes the app's dev shell (`open`), which the app takes at its next
//! poll ([`Driven::take_shell`]); `frame` asks the app to write its next
//! frame as the PNG at `path`, answered once the app has taken it
//! ([`Driven::take_frame`]).

use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::time::{Duration, Instant};

use opence_dev::control::{
    bind_private, parse_request, refusal, reply, RateLimit, Request, RequestError, MAX_CONNECTIONS,
    MAX_REQUEST,
};
use serde_json::{Map, Value};

use super::plugins::{Event, Hook, Plugin, View};
use super::widgets::WIDGET_GLOBALS_EXPORT;
use super::Stopper;
use super::{Console, Controls, Game, Tuning};

/// how long a line is given for its reply
const REPLY_LIMIT: Duration = Duration::from_secs(5);
/// how long a reply may take to write before the client is dropped
const WRITE_TIMEOUT: Duration = Duration::from_secs(5);

/// Binds the control socket at `path`, readable by this user alone.
pub fn bind(path: &Path) -> io::Result<UnixListener> {
    bind_private(path)
}

/// Where the first player's focus was in the menus when the game last
/// updated them.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Focus {
    /// the screen's name, if it has one
    pub screen: Option<String>,
    /// the focused widget's name, if it has one
    pub widget: Option<String>,
    /// a focused list's selected item, `NONE` as -1
    pub selected: Option<i16>,
    /// a focused text box's text
    pub text: Option<String>,
}

/// What the control socket drives: the game's console, its keyboard and
/// mouse, and the menus' focus as the game last had it.
#[derive(Clone)]
pub struct Driven {
    console: Console,
    controls: Controls,
    tuning: Tuning,
    focus: Arc<Mutex<Option<Focus>>>,
    /// the dev shell's last asked state, until the app takes it
    shell: Arc<Mutex<Option<bool>>>,
    /// the frame asked for, until the app takes it
    frame: Arc<(Mutex<FrameAsk>, Condvar)>,
    /// how long a frame request waits for the app to take it
    frame_wait: Duration,
    /// ends the game when a client asks it to quit
    stopper: Stopper,
}

/// The PNG a client asked the app to write its next frame to, and how many
/// the app has taken, so that a request knows when its own was.
#[derive(Default)]
struct FrameAsk {
    path: Option<PathBuf>,
    taken: u64,
}

/// How long a frame request waits for the app by default: a few of its
/// frames, and the game's loading past them.
const FRAME_WAIT: Duration = Duration::from_secs(5);

impl Driven {
    /// Takes `game`'s console and controls, and adds the plugin that keeps
    /// the menus' focus at the end of each UI update.
    pub fn of(game: &mut Game) -> Self {
        let focus = Arc::new(Mutex::new(None));
        game.add_plugin(Box::new(FocusWatch(Arc::clone(&focus))));
        Self {
            console: game.console(),
            controls: game.controls(),
            tuning: game.tuning(),
            focus,
            shell: Arc::new(Mutex::new(None)),
            frame: Arc::new((Mutex::new(FrameAsk::default()), Condvar::new())),
            frame_wait: FRAME_WAIT,
            stopper: game.stopper(),
        }
    }

    /// The same, with a frame request waiting `wait` for the app.
    #[must_use]
    pub fn with_frame_wait(mut self, wait: Duration) -> Self {
        self.frame_wait = wait;
        self
    }

    /// The menus' focus as the game last updated them, if it has.
    #[must_use]
    pub fn focus(&self) -> Option<Focus> {
        self.focus
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

impl Driven {
    /// Whether a client has asked for the app's dev shell open (or closed)
    /// since the last call: the last request asked, taken once.
    #[must_use]
    pub fn take_shell(&self) -> Option<bool> {
        self.shell
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }

    /// The PNG a client asked the app to write its next frame to, taken
    /// once: the request that asked is answered.
    #[must_use]
    pub fn take_frame(&self) -> Option<PathBuf> {
        let (ask, taken) = &*self.frame;
        let mut ask = ask.lock().unwrap_or_else(PoisonError::into_inner);
        let path = ask.path.take()?;
        ask.taken += 1;
        taken.notify_all();
        Some(path)
    }

    /// Leaves `path` for the app and waits until it has taken it, or says
    /// it did not.
    fn ask_frame(&self, path: PathBuf) -> Result<(), String> {
        let (ask, taken) = &*self.frame;
        let mut ask = ask.lock().unwrap_or_else(PoisonError::into_inner);
        ask.path = Some(path);
        let ticket = ask.taken;
        let (mut ask, _) = taken
            .wait_timeout_while(ask, self.frame_wait, |ask| ask.taken == ticket)
            .unwrap_or_else(PoisonError::into_inner);
        if ask.taken != ticket {
            return Ok(());
        }
        ask.path = None;
        Err("the app took no frame: is it drawing?".into())
    }
}

/// Keeps the first player's focus from the widget tree at each UI update.
struct FocusWatch(Arc<Mutex<Option<Focus>>>);

impl Plugin for FocusWatch {
    fn name(&self) -> &str {
        "control socket focus"
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
        let focus = tree.widgets.first().map(|screen| {
            let focused = tree.focus();
            Focus {
                screen: screen.name.clone(),
                widget: focused.and_then(|widget| widget.name.clone()),
                selected: focused.and_then(|widget| widget.list.as_ref().map(|list| list.selected)),
                text: focused.and_then(|widget| widget.text.clone()),
            }
        });
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = focus;
    }
}

/// Serves `listener` on a thread of its own, each connection on another,
/// at most [`MAX_CONNECTIONS`] at once.
pub fn serve(listener: UnixListener, driven: Driven) {
    let asking = Arc::new(Mutex::new(driven));
    let connections = Arc::new(AtomicUsize::new(0));
    let _ = std::thread::Builder::new()
        .name("control".into())
        .spawn(move || accept(&listener, &asking, &connections));
}

fn accept(listener: &UnixListener, asking: &Arc<Mutex<Driven>>, connections: &Arc<AtomicUsize>) {
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        if connections.fetch_add(1, Ordering::AcqRel) >= MAX_CONNECTIONS {
            connections.fetch_sub(1, Ordering::AcqRel);
            let _ = writeln!(
                stream,
                "{}",
                refusal(&format!("at most {MAX_CONNECTIONS} connections at once"))
            );
            continue;
        }
        let asking = Arc::clone(asking);
        let connections = Arc::clone(connections);
        let spawned = std::thread::Builder::new()
            .name("control connection".into())
            .spawn(move || {
                connection(&asking, stream);
                connections.fetch_sub(1, Ordering::AcqRel);
            });
        if spawned.is_err() {
            break;
        }
    }
}

fn connection(asking: &Mutex<Driven>, stream: UnixStream) {
    let _ = stream.set_write_timeout(Some(WRITE_TIMEOUT));
    let Ok(reading) = stream.try_clone() else {
        return;
    };
    let mut reader = BufReader::new(reading);
    let mut writer = stream;
    let mut rate = RateLimit::default();
    loop {
        let mut line = String::new();
        let limit = (MAX_REQUEST + 2) as u64;
        match (&mut reader).take(limit).read_line(&mut line) {
            Ok(0) => return,
            Ok(_) => {}
            Err(_) => {
                let _ = writeln!(writer, "{}", refusal("the request is not UTF-8 text"));
                return;
            }
        }
        let line = line.trim_end_matches(['\n', '\r']);
        if line.len() > MAX_REQUEST {
            // (the rest of the line cannot be told from a request)
            let _ = writeln!(writer, "{}", refusal(&RequestError::TooLong.to_string()));
            return;
        }
        if line.trim().is_empty() {
            continue;
        }
        let answered = if rate.allow(Instant::now()) {
            let driven = asking.lock().unwrap_or_else(PoisonError::into_inner);
            answer(&driven, line)
        } else {
            refusal(&RequestError::TooFrequent.to_string())
        };
        if writeln!(writer, "{answered}").is_err() {
            return;
        }
    }
}

/// Answers one request line: a command line is run on the game's console
/// and its output returned as `text`; a structured request moves the
/// keyboard or mouse, or reads the menus' focus; anything else is refused.
pub fn answer(driven: &Driven, line: &str) -> String {
    match parse_request(line) {
        Err(error) => refusal(&error.to_string()),
        Ok(Request::Structured(name, fields)) => {
            structured(driven, &name, &fields).unwrap_or_else(|error| refusal(&error))
        }
        Ok(Request::Command(command)) => match driven.console.ask(&command, REPLY_LIMIT) {
            None => refusal("the game's console is not listening yet"),
            Some(output) => {
                let mut fields = Map::new();
                fields.insert("text".into(), Value::String(text(&output)));
                reply(true, fields)
            }
        },
    }
}

/// Answers a structured request, or says why it cannot.
fn structured(driven: &Driven, name: &str, fields: &Map<String, Value>) -> Result<String, String> {
    let controls = &driven.controls;
    match name {
        "key" => {
            let vk = small(fields, "vk", 1..=0xFE)?;
            let down = fields
                .get("down")
                .and_then(Value::as_bool)
                .ok_or("a key needs \"down\", true or false")?;
            controls.key(vk, 0, 0, down);
        }
        "pointer" => {
            let [x, y] = [real(fields, "x")?, real(fields, "y")?];
            controls.mouse_position(x, y);
        }
        "click" => {
            let [x, y] = [real(fields, "x")?, real(fields, "y")?];
            let button = if fields.contains_key("button") {
                small(fields, "button", 1..=5)?
            } else {
                1
            };
            controls.mouse_position(x, y);
            controls.mouse_button(button, true);
            controls.mouse_button(button, false);
        }
        "wheel" => controls.mouse_wheel(real(fields, "notches")?),
        "focus" => {
            let focus = driven
                .focus()
                .ok_or("the game has not shown a screen of its menus yet")?;
            let text = |value: Option<String>| value.map_or(Value::Null, Value::String);
            let mut answer = Map::new();
            answer.insert("screen".into(), text(focus.screen));
            answer.insert("widget".into(), text(focus.widget));
            answer.insert(
                "selected".into(),
                focus.selected.map_or(Value::Null, Value::from),
            );
            answer.insert("text".into(), text(focus.text));
            return Ok(reply(true, answer));
        }
        "setting" => {
            let name = fields
                .get("name")
                .and_then(Value::as_str)
                .ok_or("a setting needs \"name\"")?;
            let unknown = || format!("there is no setting \"{name}\"");
            if let Some(value) = fields.get("value") {
                let text = value
                    .as_str()
                    .map_or_else(|| value.to_string(), str::to_owned);
                if !driven.tuning.set(name, &text) {
                    return Err(unknown());
                }
            }
            let value = driven.tuning.get(name).ok_or_else(unknown)?;
            let mut answer = Map::new();
            answer.insert("name".into(), name.into());
            answer.insert("value".into(), value.into());
            return Ok(reply(true, answer));
        }
        "shell" => {
            let open = fields
                .get("open")
                .and_then(Value::as_bool)
                .ok_or("a shell request needs \"open\", true or false")?;
            *driven.shell.lock().unwrap_or_else(PoisonError::into_inner) = Some(open);
        }
        "frame" => {
            let path = fields
                .get("path")
                .and_then(Value::as_str)
                .map(PathBuf::from)
                .filter(|path| {
                    path.is_absolute()
                        && path.extension().is_some_and(|extension| extension == "png")
                })
                .ok_or("a frame request needs \"path\", a PNG's absolute path")?;
            driven.ask_frame(path)?;
        }
        "open_screen" => return open_screen(driven, fields),
        // (the game ends at its next call into the host, and the app with it)
        "quit" => driven.stopper.stop(),
        other => return Err(format!("there is no request \"{other}\"")),
    }
    Ok(reply(true, Map::new()))
}

/// Opens a menu screen by its name through the game's console (`ui_open`,
/// port/linux/game/menu_tags.c), which opens it as the main menu opens its
/// own, or refuses a name it does not know, listing the screens there are.
fn open_screen(driven: &Driven, fields: &Map<String, Value>) -> Result<String, String> {
    let name = fields
        .get("name")
        .and_then(Value::as_str)
        .filter(|name| !name.is_empty())
        .ok_or("an open_screen request needs \"name\", a screen's name")?;
    // (a name the console would read as more than one, or cut short)
    if name.contains(['"', ')', '\n', '\r']) {
        return Err(format!("there is no screen named {name:?}"));
    }
    let output = driven
        .console
        .ask(&format!("ui_open \"{name}\""), REPLY_LIMIT)
        .ok_or("the game's console is not listening yet")?;
    let output = text(&output);
    // (the console echoes the command before its answer)
    let mut lines = output
        .lines()
        .skip_while(|line| !line.starts_with("ui_open: "));
    match lines.next().and_then(|line| line.strip_prefix("ui_open: ")) {
        Some(opened) if opened.strip_prefix("opened ") == Some(name) => {
            let mut answer = Map::new();
            answer.insert("screen".into(), name.into());
            Ok(reply(true, answer))
        }
        Some(refused) => {
            let screens: Vec<&str> = lines.collect();
            Err(if screens.is_empty() {
                refused.to_owned()
            } else {
                format!("{refused} {}", screens.join(", "))
            })
        }
        None => Err(format!("the game did not open {name}: {output}")),
    }
}

/// The whole number `name` in `fields`, within `range`.
fn small(
    fields: &Map<String, Value>,
    name: &str,
    range: std::ops::RangeInclusive<u8>,
) -> Result<u8, String> {
    fields
        .get(name)
        .and_then(Value::as_u64)
        .and_then(|value| u8::try_from(value).ok())
        .filter(|value| range.contains(value))
        .ok_or_else(|| {
            format!(
                "\"{name}\" must be a whole number from {} to {}",
                range.start(),
                range.end()
            )
        })
}

/// The finite number `name` in `fields`, as the window's `f32`.
fn real(fields: &Map<String, Value>, name: &str) -> Result<f32, String> {
    fields
        .get(name)
        .and_then(Value::as_f64)
        // (a pixel or a notch: f32 keeps what a window could say)
        .map(|value| value as f32)
        .filter(|value| value.is_finite())
        .ok_or_else(|| format!("\"{name}\" must be a number"))
}

/// The console's output as the shell shows it: its lines ended by `\n`,
/// with the blank lines the console writes around each print left out.
fn text(output: &str) -> String {
    output
        .split(['\r', '\n'])
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
