//! `opence`: the developer shell of a running engine (the app with
//! `--headless`, or with `--control` beside its window), over its control
//! socket.
//!
//! ```text
//! opence [--socket PATH] run [--json] LINE...   a shell line
//! opence [--socket PATH] state                  the world's state, as JSON
//! opence [--socket PATH] frame PATH [--size WxH] the view, drawn into a PNG
//! opence [--socket PATH] watch [--backlog N]    the log, as it is written
//! opence [--socket PATH] script FILE            a script's lines, stopping at a failure
//! opence [--socket PATH] shell                  lines typed (completed by the engine), or read
//! opence [--socket PATH] key VK [--down | --up]  a key pressed (a Windows key code), or held, or let go
//! opence [--socket PATH] pointer X Y            the menus' pointer moved, in the window's units
//! opence [--socket PATH] click X Y [--button N] a click there (1 left, 2 right, 3 middle, 4, 5)
//! opence [--socket PATH] wheel N                the wheel turned N notches (up is positive)
//! opence [--socket PATH] focus                  the menu screen shown and its focused widget, as JSON
//! opence [--socket PATH] setting NAME [VALUE]   a setting's value, as JSON, set to VALUE first if given
//! opence [--socket PATH] ui open SCREEN          a menu screen opened by its name ("main_menu/settings_select")
//! opence scripts MAP                          a map's scripts, each with when it runs
//! opence census [--check | --write | --rank | --ratchet] [--record-importer] [--root DIR] the coverage census of the repository
//! ```
//!
//! `scripts` and `census` need no engine: it reads the repository (the folder holding
//! `source/` and `engine/`, found upwards from the current folder).
//!
//! It exits 0 when everything succeeded, 1 when the engine refused
//! something (or the census's check failed), and 2 when the engine could not be reached or the arguments
//! were wrong.

use std::io::{self, BufReader, IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use opence_cli::{
    bake_level, complete, drive, frame, key, number, parse_size, run, script, script_lines,
    shell_lines, state, watch, Connection, Error, Press,
};
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::{Context, Editor, Helper, Highlighter, Hinter, Validator};

const USAGE: &str = "usage: opence [--socket PATH] (run [--json] LINE... | state | frame PATH [--size WxH] | watch [--backlog N] | script FILE | shell | key VK [--down | --up] | pointer X Y | click X Y [--button N] | wheel N | focus | setting NAME [VALUE] | ui open SCREEN | scripts MAP | bake MAP [--data DIR] [--cache DIR] [--samples N] | census [--check | --write | --rank | --ratchet] [--record-importer] [--root DIR])";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match start(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Error::Refused(reason)) => {
            eprintln!("opence: {reason}");
            ExitCode::from(1)
        }
        Err(Error::Usage(text)) => {
            eprintln!("opence: {text}\n{USAGE}");
            ExitCode::from(2)
        }
        Err(error) => {
            eprintln!("opence: {error}");
            ExitCode::from(2)
        }
    }
}

fn start(arguments: &[String]) -> Result<(), Error> {
    let mut arguments = arguments.to_vec();
    let mut socket = opence_dev::control::default_socket_path();
    if let Some(index) = arguments.iter().position(|word| word == "--socket") {
        let path = arguments
            .get(index + 1)
            .ok_or_else(|| Error::Usage("--socket needs a path".into()))?;
        socket = PathBuf::from(path);
        arguments.drain(index..=index + 1);
    }
    let (subcommand, rest) = arguments
        .split_first()
        .ok_or_else(|| Error::Usage("no subcommand".into()))?;
    let mut out = io::stdout().lock();
    match subcommand.as_str() {
        "run" => {
            let as_json = rest.first().is_some_and(|word| word == "--json");
            let words = if as_json { &rest[1..] } else { rest };
            if words.is_empty() {
                return Err(Error::Usage("run needs a line".into()));
            }
            run(
                &mut Connection::connect(&socket)?,
                &words.join(" "),
                as_json,
                &mut out,
            )
        }
        "state" => state(&mut Connection::connect(&socket)?, &mut out),
        "key" => {
            let vk: u8 = number(rest.first(), "a key")?;
            if vk == 0 || vk == 0xFF {
                return Err(Error::Usage("a key is 1 to 254".into()));
            }
            let press = match rest.get(1).map(String::as_str) {
                None => Press::Whole,
                Some("--down") => Press::Down,
                Some("--up") => Press::Up,
                Some(other) => return Err(Error::Usage(format!("key does not take {other}"))),
            };
            if rest.len() > 2 {
                return Err(Error::Usage("key takes one key".into()));
            }
            key(&mut Connection::connect(&socket)?, vk, press)
        }
        "pointer" | "click" => {
            let x: f64 = number(rest.first(), "x")?;
            let y: f64 = number(rest.get(1), "y")?;
            let mut request = serde_json::json!({"request": subcommand, "x": x, "y": y});
            match (subcommand.as_str(), &rest[2.min(rest.len())..]) {
                (_, []) => {}
                ("click", [flag, button]) if flag == "--button" => {
                    let button: u8 = number(Some(button), "a button")?;
                    if !(1..=5).contains(&button) {
                        return Err(Error::Usage("a button is 1 to 5".into()));
                    }
                    request["button"] = serde_json::json!(button);
                }
                _ => {
                    return Err(Error::Usage(format!(
                        "{subcommand} takes X Y{}",
                        if subcommand == "click" {
                            " [--button N]"
                        } else {
                            ""
                        }
                    )))
                }
            }
            drive(
                &mut Connection::connect(&socket)?,
                &request,
                false,
                &mut out,
            )
        }
        "wheel" => {
            let notches: i32 = number(rest.first(), "notches")?;
            if rest.len() > 1 {
                return Err(Error::Usage("wheel takes one count".into()));
            }
            let request = serde_json::json!({"request": "wheel", "notches": notches});
            drive(
                &mut Connection::connect(&socket)?,
                &request,
                false,
                &mut out,
            )
        }
        "focus" => {
            if !rest.is_empty() {
                return Err(Error::Usage("focus takes nothing".into()));
            }
            let request = serde_json::json!({"request": "focus"});
            drive(&mut Connection::connect(&socket)?, &request, true, &mut out)
        }
        "setting" => {
            let (name, value) = match rest {
                [name] => (name, None),
                [name, value] => (name, Some(value)),
                _ => return Err(Error::Usage("setting takes NAME and maybe VALUE".into())),
            };
            let mut request = serde_json::json!({"request": "setting", "name": name});
            if let Some(value) = value {
                request["value"] = value.as_str().into();
            }
            drive(&mut Connection::connect(&socket)?, &request, true, &mut out)
        }
        "ui" => {
            let [verb, screen] = rest else {
                return Err(Error::Usage("ui takes open and a screen's name".into()));
            };
            if verb != "open" {
                return Err(Error::Usage(format!("ui does not take {verb}")));
            }
            let request = serde_json::json!({"request": "open_screen", "name": screen});
            drive(&mut Connection::connect(&socket)?, &request, true, &mut out)
        }
        "frame" => {
            let path = rest
                .first()
                .ok_or_else(|| Error::Usage("frame needs a PNG's path".into()))?;
            let size = match rest.get(1).map(String::as_str) {
                None => None,
                Some("--size") => Some(parse_size(
                    rest.get(2)
                        .ok_or_else(|| Error::Usage("--size needs WxH".into()))?,
                )?),
                Some(other) => return Err(Error::Usage(format!("frame does not take {other}"))),
            };
            frame(
                &mut Connection::connect(&socket)?,
                path.as_ref(),
                size,
                &mut out,
            )
            .map(|_| ())
        }
        "watch" => {
            let backlog = match rest {
                [] => 0,
                [flag, count] if flag == "--backlog" => count
                    .parse()
                    .map_err(|_| Error::Usage(format!("--backlog is a count, not {count}")))?,
                _ => return Err(Error::Usage("watch takes only --backlog N".into())),
            };
            watch(&mut Connection::connect(&socket)?, backlog, None, &mut out)
        }
        "script" => {
            let file = rest
                .first()
                .ok_or_else(|| Error::Usage("script needs a file".into()))?;
            let text = std::fs::read_to_string(file)?;
            // (its frames beside it, in <name>-frames)
            let path = std::path::absolute(file)?;
            let stem = path
                .file_stem()
                .map_or_else(|| "script".into(), |stem| stem.to_string_lossy());
            let folder = path
                .parent()
                .map(|parent| parent.join(format!("{stem}-frames")));
            script(
                &mut Connection::connect(&socket)?,
                &text,
                folder.as_deref(),
                &mut out,
            )
        }
        "shell" => {
            let mut connection = Connection::connect(&socket)?;
            if io::stdin().is_terminal() {
                drop(out);
                interactive(connection)
            } else {
                let refused = shell_lines(&mut connection, BufReader::new(io::stdin()), &mut out)?;
                match refused {
                    0 => Ok(()),
                    count => Err(Error::Refused(format!("{count} lines were refused"))),
                }
            }
        }
        "scripts" => {
            let [map] = rest else {
                return Err(Error::Usage("scripts takes a map's file".into()));
            };
            let cache = opence_import::Cache::open(std::fs::read(map)?)
                .map_err(|error| Error::Refused(format!("{map}: {error}")))?;
            let scripts = opence_import::import_scripts(&cache)
                .map_err(|error| Error::Refused(format!("{map}: {error}")))?;
            for line in script_lines(&scripts)? {
                writeln!(out, "{line}")?;
            }
            Ok(())
        }
        "census" => {
            let mut mode = opence_census::Mode::Summary;
            let mut root = None;
            let mut record = false;
            let mut words = rest.iter();
            while let Some(word) = words.next() {
                match word.as_str() {
                    "--check" => mode = opence_census::Mode::Check,
                    "--write" => mode = opence_census::Mode::Write,
                    "--rank" => mode = opence_census::Mode::Rank,
                    "--ratchet" => mode = opence_census::Mode::Ratchet,
                    "--record-importer" => record = true,
                    "--root" => {
                        root =
                            Some(PathBuf::from(words.next().ok_or_else(|| {
                                Error::Usage("--root needs a folder".into())
                            })?));
                    }
                    other => return Err(Error::Usage(format!("census does not take {other}"))),
                }
            }
            let root = match root {
                Some(root) => root,
                None => repository()?,
            };
            if record {
                let maps = record_importer(&root)?;
                writeln!(out, "recorded the importer's reads on {maps} maps")?;
            }
            let checked = opence_census::census(&root, mode).and_then(|summary| {
                // (the traces' own checks need Python and clang, so they run
                // here and not in the crate's cargo test)
                match mode {
                    opence_census::Mode::Check => match opence_census::check_traces(&root)? {
                        problems if problems.is_empty() => Ok(summary),
                        problems => Err(opence_census::Error::Problems(problems)),
                    },
                    opence_census::Mode::Summary
                    | opence_census::Mode::Write
                    | opence_census::Mode::Rank
                    | opence_census::Mode::Ratchet => Ok(summary),
                }
            });
            match checked {
                Ok(summary) => Ok(write!(out, "{summary}")?),
                Err(opence_census::Error::Problems(problems)) => {
                    for problem in &problems {
                        writeln!(out, "{problem}")?;
                    }
                    Err(Error::Refused(format!(
                        "the census has {} problems",
                        problems.len()
                    )))
                }
                Err(error) => Err(Error::Io(io::Error::other(error.to_string()))),
            }
        }
        "bake" => {
            let mut words = rest.iter();
            let map = words
                .next()
                .ok_or_else(|| Error::Usage("bake needs a map's name".into()))?;
            let (mut data, mut cache, mut samples) = (None, None, 64);
            while let Some(word) = words.next() {
                match word.as_str() {
                    "--data" => data = words.next().map(PathBuf::from),
                    "--cache" => cache = words.next().map(PathBuf::from),
                    "--samples" => samples = number(words.next(), "--samples")?,
                    other => return Err(Error::Usage(format!("bake does not take {other}"))),
                }
            }
            let data = match data {
                Some(data) => data,
                None => repository()?.join("assets"),
            };
            // (the headless game's cache; the app's is its support folder's)
            let cache = match cache {
                Some(cache) => cache,
                None => repository()?.join("build/engine/wasm/cache"),
            };
            let map_file = data.join("maps").join(format!("{map}.map"));
            let loaded = opence_import::load_level(&map_file, &cache.join("import"))
                .map_err(|error| Error::Refused(format!("{}: {error}", map_file.display())))?;
            let started = std::time::Instant::now();
            let written = bake_level(&loaded.level, map, &cache.join("bake"), samples)?;
            for path in &written {
                writeln!(out, "{}", path.display())?;
            }
            writeln!(
                out,
                "baked {} BSPs at {samples} paths a texel in {:.1} s",
                written.len(),
                started.elapsed().as_secs_f64()
            )?;
            Ok(())
        }
        other => Err(Error::Usage(format!("no subcommand {other}"))),
    }
}

/// Runs the importer on every map in `root/assets/maps` with its reads
/// recorded, and writes them to `engine/census/importer_reads.tsv` (only
/// the importer's offsets, nothing of the maps). A map the importer
/// refuses adds the reads it made before refusing. With no maps, nothing
/// is written.
fn record_importer(root: &std::path::Path) -> Result<usize, Error> {
    let folder = root.join("assets").join("maps");
    let Ok(entries) = std::fs::read_dir(&folder) else {
        return Ok(0);
    };
    let mut maps: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "map"))
        .collect();
    maps.sort();
    let mut reads = std::collections::BTreeSet::new();
    let mut imported = 0;
    for map in &maps {
        let file = std::fs::read(map)?;
        let Ok(cache) = opence_import::Cache::open(file) else {
            continue;
        };
        let (_, recorded) =
            opence_import::record::recording(|| opence_import::import_level(&cache));
        imported += 1;
        reads.extend(recorded.into_iter().map(|read| {
            opence_census::importer::Read {
                group: read.group.0,
                hops: read
                    .hops
                    .iter()
                    .map(|hop| opence_census::importer::Hop {
                        at: hop.at,
                        element_size: hop.element_size,
                    })
                    .collect(),
                at: read.at,
                width: read.width,
                label: read.label,
            }
        }));
    }
    if imported > 0 {
        let path = root.join("engine").join(opence_census::importer::READS);
        std::fs::write(path, opence_census::importer::write_reads(&reads))?;
    }
    Ok(imported)
}

/// The repository: the nearest folder upwards from the current one that
/// holds `source/` and `engine/`.
fn repository() -> Result<PathBuf, Error> {
    let here = std::env::current_dir()?;
    here.ancestors()
        .find(|folder| folder.join("source").is_dir() && folder.join("engine").is_dir())
        .map(Into::into)
        .ok_or_else(|| Error::Usage("not inside the repository; pass --root".into()))
}

/// The interactive shell: lines edited and remembered (in
/// `~/.opence_history`), Tab completing from the engine's registry.
fn interactive(connection: Connection) -> Result<(), Error> {
    let readline = |error: ReadlineError| Error::Io(io::Error::other(error.to_string()));
    let mut editor: Editor<Engine, rustyline::history::FileHistory> =
        Editor::new().map_err(readline)?;
    editor.set_helper(Some(Engine {
        connection: std::cell::RefCell::new(connection),
    }));
    let history = std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".opence_history"));
    if let Some(history) = &history {
        let _ = editor.load_history(history);
    }
    loop {
        let line = match editor.readline("] ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(error) => return Err(readline(error)),
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let _ = editor.add_history_entry(line);
        let Some(helper) = editor.helper() else { break };
        let mut connection = helper.connection.borrow_mut();
        let mut out = io::stdout().lock();
        match run(&mut connection, line, false, &mut out) {
            Ok(()) => {}
            Err(Error::Refused(reason)) => writeln!(out, "error: {reason}")?,
            Err(error) => return Err(error),
        }
        if line == "quit" {
            break;
        }
    }
    if let Some(history) = &history {
        let _ = editor.save_history(history);
    }
    Ok(())
}

/// Completion from the engine.
#[derive(Helper, Highlighter, Hinter, Validator)]
struct Engine {
    connection: std::cell::RefCell<Connection>,
}

impl Completer for Engine {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        position: usize,
        _context: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let before = &line[..position];
        let start = before.rfind([' ', ';']).map_or(0, |space| space + 1);
        let words = complete(&mut self.connection.borrow_mut(), before).unwrap_or_default();
        let pairs = words
            .into_iter()
            .map(|word| Pair {
                display: word.clone(),
                replacement: word,
            })
            .collect();
        Ok((start, pairs))
    }
}
