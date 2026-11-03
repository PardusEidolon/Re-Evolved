//! The `opence` tool's work: a connection to a running engine's control
//! socket, and its subcommands (`run`, `state`, `frame`, `watch`, `script`,
//! `shell`), each writing what it got to a writer so that it can be tested.

use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{json, Value};

/// How long a reply may take (a frame is drawn by the app, which may be
/// loading a map).
pub const REPLY_TIMEOUT: Duration = Duration::from_secs(120);

/// What went wrong.
#[derive(Debug)]
pub enum Error {
    /// The socket could not be reached or read.
    Io(io::Error),
    /// The engine sent what is not a JSON reply, or closed the connection.
    Reply(String),
    /// The engine refused or failed the request; its reason.
    Refused(String),
    /// The arguments were wrong.
    Usage(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Reply(text) => write!(formatter, "the engine's reply: {text}"),
            Self::Refused(text) => write!(formatter, "{text}"),
            Self::Usage(text) => write!(formatter, "{text}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// A connection to the engine.
pub struct Connection {
    reader: BufReader<UnixStream>,
    writer: UnixStream,
}

impl Connection {
    /// Connects to the socket at `path`.
    pub fn connect(path: &Path) -> Result<Self, Error> {
        let writer = UnixStream::connect(path).map_err(|error| {
            Error::Io(io::Error::new(
                error.kind(),
                format!(
                    "no engine at {} ({error}); start one with --headless or --control",
                    path.display()
                ),
            ))
        })?;
        writer.set_read_timeout(Some(REPLY_TIMEOUT))?;
        Ok(Self {
            reader: BufReader::new(writer.try_clone()?),
            writer,
        })
    }

    /// Sends `request` and reads its reply.
    pub fn request(&mut self, request: &Value) -> Result<Value, Error> {
        writeln!(self.writer, "{request}")?;
        self.read()
    }

    /// Reads a line the engine sent.
    pub fn read(&mut self) -> Result<Value, Error> {
        let mut line = String::new();
        if self.reader.read_line(&mut line)? == 0 {
            return Err(Error::Reply("the connection closed".into()));
        }
        serde_json::from_str(&line).map_err(|error| Error::Reply(error.to_string()))
    }

    /// Sends `request`; its reply, or the engine's refusal as an error.
    pub fn ask(&mut self, request: &Value) -> Result<Value, Error> {
        let reply = self.request(request)?;
        if reply["ok"] == true {
            Ok(reply)
        } else {
            Err(Error::Refused(
                reply["error"].as_str().unwrap_or("refused").to_owned(),
            ))
        }
    }
}

/// Runs a shell line: writes its text (or with `as_json`, the whole
/// reply).
pub fn run(
    connection: &mut Connection,
    line: &str,
    as_json: bool,
    out: &mut impl Write,
) -> Result<(), Error> {
    let reply = connection.ask(&Value::String(line.to_owned()))?;
    if as_json {
        writeln!(out, "{reply}")?;
    } else if let Some(text) = reply["text"].as_str().filter(|text| !text.is_empty()) {
        writeln!(out, "{text}")?;
    }
    Ok(())
}

/// Writes the world's state as JSON.
pub fn state(connection: &mut Connection, out: &mut impl Write) -> Result<(), Error> {
    let mut reply = connection.ask(&json!({"request": "state"}))?;
    if let Some(fields) = reply.as_object_mut() {
        fields.remove("ok");
    }
    writeln!(out, "{reply:#}")?;
    Ok(())
}

/// How long a pressed key is held: past a frame of the game's (which
/// reads held keys at 60 Hz), so the menus see it go down.
pub const PRESS: std::time::Duration = std::time::Duration::from_millis(50);

/// Which part of a key's press to send.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Press {
    /// Down, held for [`PRESS`], then up.
    Whole,
    /// Down only, held until an `Up`.
    Down,
    /// Up only.
    Up,
}

/// Presses the virtual key `vk` (a Windows key code, 1 to 254).
pub fn key(connection: &mut Connection, vk: u8, press: Press) -> Result<(), Error> {
    let mut send = |down: bool| connection.ask(&json!({"request": "key", "vk": vk, "down": down}));
    match press {
        Press::Whole => {
            send(true)?;
            std::thread::sleep(PRESS);
            send(false)?;
        }
        Press::Down => {
            send(true)?;
        }
        Press::Up => {
            send(false)?;
        }
    }
    Ok(())
}

/// Sends a structured request to the game (`pointer`, `click`, `wheel`)
/// and writes nothing; or with `answer`, writes the reply without its
/// `ok` (`focus`).
pub fn drive(
    connection: &mut Connection,
    request: &Value,
    answer: bool,
    out: &mut impl Write,
) -> Result<(), Error> {
    let mut reply = connection.ask(request)?;
    if answer {
        if let Some(fields) = reply.as_object_mut() {
            fields.remove("ok");
        }
        writeln!(out, "{reply}")?;
    }
    Ok(())
}

/// Reads a number of the command line, naming `what` when it is not one.
pub fn number<T: std::str::FromStr>(word: Option<&String>, what: &str) -> Result<T, Error> {
    word.and_then(|word| word.parse().ok())
        .ok_or_else(|| Error::Usage(format!("{what} must be a number")))
}

/// Reads `WIDTHxHEIGHT`.
pub fn parse_size(size: &str) -> Result<(u32, u32), Error> {
    size.split_once('x')
        .and_then(|(width, height)| Some((width.parse().ok()?, height.parse().ok()?)))
        .ok_or_else(|| Error::Usage(format!("a size is WIDTHxHEIGHT, not {size}")))
}

/// Asks the engine to draw its view into the PNG at `path` (made absolute
/// against the working folder).
pub fn frame(
    connection: &mut Connection,
    path: &Path,
    size: Option<(u32, u32)>,
    out: &mut impl Write,
) -> Result<PathBuf, Error> {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut request = json!({"request": "frame", "path": path});
    if let Some((width, height)) = size {
        request["width"] = json!(width);
        request["height"] = json!(height);
    }
    connection.ask(&request)?;
    writeln!(out, "{}", path.display())?;
    Ok(path)
}

/// A log line as the tool writes it.
pub fn log_line(entry: &Value) -> String {
    format!(
        "{:>5} {}: {}",
        entry["level"].as_str().unwrap_or("?"),
        entry["target"].as_str().unwrap_or("?"),
        entry["text"].as_str().unwrap_or("")
    )
}

/// Writes the log as it is written, after the last `backlog` lines kept,
/// until the engine closes the connection (or `limit` lines, if given).
pub fn watch(
    connection: &mut Connection,
    backlog: u32,
    limit: Option<usize>,
    out: &mut impl Write,
) -> Result<(), Error> {
    let mut last = 0;
    if backlog > 0 {
        let kept = connection.ask(&json!({"request": "log", "since": 0, "max": 100_000}))?;
        let lines = kept["lines"].as_array().cloned().unwrap_or_default();
        let skip = lines.len().saturating_sub(backlog as usize);
        for entry in &lines[skip..] {
            writeln!(out, "{}", log_line(entry))?;
            last = entry["sequence"].as_u64().unwrap_or(last);
        }
    }
    // (from the backlog's last line, so none written in between is lost)
    let mut subscribe = json!({"request": "subscribe", "topic": "log"});
    if last > 0 {
        subscribe["since"] = json!(last);
    }
    connection.ask(&subscribe)?;
    let mut written = 0;
    while limit.is_none_or(|limit| written < limit) {
        let line = match connection.read() {
            Ok(line) => line,
            Err(Error::Reply(_)) => break,
            Err(error) => return Err(error),
        };
        let entry = &line["log"];
        if entry.is_null() || entry["sequence"].as_u64().unwrap_or(u64::MAX) <= last {
            continue;
        }
        writeln!(out, "{}", log_line(entry))?;
        out.flush()?;
        written += 1;
    }
    Ok(())
}

/// Runs a script's lines in order, writing each one's result; stops at,
/// and fails with, the first that fails.
pub fn script(
    connection: &mut Connection,
    text: &str,
    folder: Option<&Path>,
    out: &mut impl Write,
) -> Result<(), Error> {
    let mut request = json!({"request": "script", "text": text});
    if let Some(folder) = folder {
        request["folder"] = json!(folder.to_string_lossy());
    }
    let reply = connection.request(&request)?;
    let results = reply["results"].as_array().cloned().unwrap_or_default();
    for result in &results {
        let line = result["line"].as_u64().unwrap_or(0);
        let text = result["text"]
            .as_str()
            .or_else(|| result["error"].as_str())
            .unwrap_or("");
        let mark = if result["ok"] == true { "ok" } else { "FAILED" };
        writeln!(out, "{line:>4} {mark} {text}")?;
    }
    if reply["ok"] == true {
        return Ok(());
    }
    let failure = &reply["failure"];
    if let Some(expression) = failure["expression"].as_str() {
        let value = failure["value"].as_str().unwrap_or("(none)");
        writeln!(out, "     {expression} was {value}")?;
    }
    if failure["state"].is_object() {
        writeln!(out, "     state: {}", failure["state"])?;
    }
    let reason = failure["reason"]
        .as_str()
        .or_else(|| reply["error"].as_str())
        .unwrap_or("refused");
    Err(Error::Refused(match failure["line"].as_u64() {
        Some(line) if line > 0 => format!("line {line}: {reason}"),
        _ => reason.to_owned(),
    }))
}

/// The engine's completions of `line`.
pub fn complete(connection: &mut Connection, line: &str) -> Result<Vec<String>, Error> {
    let reply = connection.ask(&json!({"request": "complete", "line": line}))?;
    Ok(reply["completions"]
        .as_array()
        .map(|all| {
            all.iter()
                .filter_map(|word| word.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default())
}

/// Runs each line read from `input` (blank lines and `#` comments are
/// skipped), writing results and refusals; a refusal does not stop it.
/// Returns the number refused.
pub fn shell_lines(
    connection: &mut Connection,
    input: impl BufRead,
    out: &mut impl Write,
) -> Result<usize, Error> {
    let mut refused = 0;
    for line in input.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match run(connection, line, false, out) {
            Ok(()) => {}
            Err(Error::Refused(reason)) => {
                writeln!(out, "error: {reason}")?;
                refused += 1;
            }
            Err(error) => return Err(error),
        }
    }
    Ok(refused)
}

/// A map's scripts as `opence scripts MAP` prints them, in the scenario's
/// order: a line each, its name and when it runs (`startup`, `dormant`,
/// `continuous`, `static` or `stub`), a tab between.
///
/// # Errors
/// [`Error::Refused`] with the reason when the game would not run them.
pub fn script_lines(scripts: &opence_content::Scripts) -> Result<Vec<String>, Error> {
    use opence_content::ScriptKind;
    if let Some(reason) = &scripts.disabled {
        return Err(Error::Refused(format!(
            "the map's scripts do not run: {reason}"
        )));
    }
    Ok(scripts
        .scripts
        .iter()
        .map(|script| {
            let kind = match script.kind {
                ScriptKind::Startup => "startup",
                ScriptKind::Dormant => "dormant",
                ScriptKind::Continuous => "continuous",
                ScriptKind::Static => "static",
                ScriptKind::Stub => "stub",
            };
            format!("{}\t{kind}", script.name)
        })
        .collect())
}

/// `opence bake`: bakes each of `level`'s BSPs' lightmaps at `samples`
/// paths a texel into `folder` ([`write_bake`]), where the game reads them
/// (`lighting.bounce=baked`). A BSP with no lightmap pages is left out.
/// Answers the files written, BSP by BSP.
///
/// # Errors
/// [`Error::Io`] if the folder cannot be written.
pub fn bake_level(
    level: &opence_content::Level,
    map: &str,
    folder: &Path,
    samples: u32,
) -> Result<Vec<PathBuf>, Error> {
    use opence_bake::{BakePlan, BakeSettings};
    let settings = BakeSettings {
        lightmap_samples: samples,
        ..BakeSettings::default()
    };
    let mut written = Vec::new();
    for bsp in 0..level.structure_bsps.len() {
        let Some(plan) = BakePlan::of(level, bsp, settings) else {
            continue;
        };
        if plan.atlas.points.is_empty() {
            continue;
        }
        written.push(write_bake(folder, map, bsp, &plan.bake_lightmaps())?);
    }
    Ok(written)
}

/// Writes `baked`, BSP `bsp` of `map`'s bake, into `folder` as
/// `<map>.bsp<N>.<key>.bake`, and removes that BSP's older bakes. The file
/// is written beside its name and then renamed, so the game never reads
/// half of one. Answers its path.
///
/// # Errors
/// [`Error::Io`] if the folder cannot be written.
pub fn write_bake(
    folder: &Path,
    map: &str,
    bsp: usize,
    baked: &opence_bake::BakedLighting,
) -> Result<PathBuf, Error> {
    std::fs::create_dir_all(folder)?;
    let name = opence_bake::BakedLighting::file_name(map, bsp, baked.key);
    let prefix = format!("{map}.bsp{bsp}.");
    for entry in std::fs::read_dir(folder)? {
        let old = entry?.file_name();
        let old = old.to_string_lossy();
        if old.starts_with(&prefix) && old.ends_with(".bake") && *old != name {
            std::fs::remove_file(folder.join(&*old))?;
        }
    }
    let path = folder.join(&name);
    let partial = folder.join(format!("{name}.partial"));
    std::fs::write(&partial, baked.encode())?;
    std::fs::rename(&partial, &path)?;
    Ok(path)
}
