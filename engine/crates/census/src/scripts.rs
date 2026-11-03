//! The original's script functions and globals: its function table
//! (`hs_function_table`, with `hs_function_allowed_in_maps` beside it) and
//! its globals' table (`hs_external_globals`), each name marked with
//! whether the engine wires it.
//!
//! What the engine wires is `engine/census/script_wired.tsv`, which the
//! campaign crate's test writes from its registry and the runtime's
//! special forms (the census does not build the engine). From the two,
//! `opence census --write` writes `engine/census/scripts.tsv`, and the
//! check fails when it is stale.

use std::collections::BTreeSet;
use std::path::Path;

use crate::{read_text, Error};

/// The engine's wired names, under `engine/`.
pub const WIRED: &str = "census/script_wired.tsv";
/// The marked tables, under `engine/`.
pub const SCRIPTS: &str = "census/scripts.tsv";
/// The original's function table and its globals' table, in `source/`.
const FUNCTIONS_FILE: &str = "source/hs/hs.c";
const GLOBALS_FILE: &str = "source/hs/hs_globals_external.c";

/// A function or a global.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A function scripts call.
    Function,
    /// A global of the engine's that scripts read or set.
    Global,
}

/// Whether the engine has it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    /// A native the registry gives, or a global it gives.
    Wired,
    /// A form the runtime evaluates itself.
    Special,
    /// Not in the engine.
    Missing,
    /// The engine's own, in none of the original's tables.
    Extra,
}

/// A row of the marked tables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// Function or global.
    pub kind: Kind,
    /// Its name.
    pub name: String,
    /// For one of the original's functions, whether a map may call it.
    pub maps: Option<bool>,
    /// Whether the engine has it.
    pub engine: Engine,
}

/// What the engine wires, by kind.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Wired {
    /// The runtime's special forms.
    pub specials: BTreeSet<String>,
    /// The registry's natives.
    pub functions: BTreeSet<String>,
    /// The registry's globals.
    pub globals: BTreeSet<String>,
}

/// `text` without its comments, each replaced by a space.
fn uncommented(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('/') {
        let (before, from) = rest.split_at(at);
        out.push_str(before);
        if let Some(body) = from.strip_prefix("/*") {
            rest = body
                .find("*/")
                .and_then(|end| body.get(end + 2..))
                .unwrap_or_default();
            out.push(' ');
        } else if let Some(body) = from.strip_prefix("//") {
            rest = body
                .find('\n')
                .and_then(|end| body.get(end..))
                .unwrap_or_default();
            out.push(' ');
        } else {
            out.push('/');
            rest = from.get(1..).unwrap_or_default();
        }
    }
    out.push_str(rest);
    out
}

/// Each `X_definition` initialiser's first string: the name the scripts
/// use, by the definition's name.
fn names(text: &str) -> std::collections::BTreeMap<&str, &str> {
    let mut found = std::collections::BTreeMap::new();
    let mut from = 0;
    while let Some(at) = text.get(from..).and_then(|rest| rest.find("_definition")) {
        let end = from + at + "_definition".len();
        from = end;
        let start = text
            .get(..end)
            .unwrap_or_default()
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |at| at + 1);
        let after = text.get(end..).unwrap_or_default().trim_start();
        let Some(initialiser) = after.strip_prefix('=') else {
            continue;
        };
        let initialiser = initialiser.trim_start();
        if !initialiser.starts_with('{') {
            continue;
        }
        let Some(name) = initialiser
            .split_once('"')
            .and_then(|(_, rest)| rest.split_once('"'))
            .map(|(name, _)| name)
        else {
            continue;
        };
        if let Some(definition) = text.get(start..end) {
            found.insert(definition, name);
        }
    }
    found
}

/// The `&X` entries of the table whose initialiser follows `keyword`, to
/// the first closing brace after its first entry.
fn table<'a>(text: &'a str, keyword: &str, file: &str) -> Result<Vec<&'a str>, Error> {
    let missing = |why: &str| Error::Malformed {
        file: file.to_owned(),
        line: 0,
        why: why.to_owned(),
    };
    let at = text.find(keyword).ok_or_else(|| missing("no table"))?;
    let rest = text.get(at..).unwrap_or_default();
    let first = rest.find('&').ok_or_else(|| missing("an empty table"))?;
    let close = first + rest.get(first..).unwrap_or_default().find('}').unwrap_or(0);
    Ok(rest
        .get(first..close)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let entry = entry.trim_start_matches('&');
            entry.strip_suffix(".definition").unwrap_or(entry)
        })
        .collect())
}

/// The function table's names in order, each with whether a map may call
/// it.
pub fn functions(text: &str) -> Result<Vec<(String, bool)>, Error> {
    let text = uncommented(text);
    let malformed = |why: String| Error::Malformed {
        file: FUNCTIONS_FILE.to_owned(),
        line: 0,
        why,
    };
    let names = names(&text);
    let entries = table(&text, "hs_function_table=", FUNCTIONS_FILE)
        .or_else(|_| table(&text, "hs_function_table =", FUNCTIONS_FILE))?;
    let flags_at = text
        .find("hs_function_allowed_in_maps[]")
        .ok_or_else(|| malformed("no hs_function_allowed_in_maps".to_owned()))?;
    let flags_text = text.get(flags_at..).unwrap_or_default();
    let open = flags_text.find('{').unwrap_or(0);
    let close = open
        + flags_text
            .get(open..)
            .unwrap_or_default()
            .find('}')
            .unwrap_or(0);
    let flags = flags_text
        .get(open + 1..close)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|flag| !flag.is_empty())
        .map(|flag| match flag {
            "TRUE" => Ok(true),
            "FALSE" => Ok(false),
            other => Err(malformed(format!("a map flag of {other}"))),
        })
        .collect::<Result<Vec<bool>, Error>>()?;
    if flags.len() != entries.len() {
        return Err(malformed(format!(
            "{} functions and {} map flags",
            entries.len(),
            flags.len()
        )));
    }
    entries
        .into_iter()
        .zip(flags)
        .map(|(entry, flag)| {
            names
                .get(entry)
                .map(|name| ((*name).to_owned(), flag))
                .ok_or_else(|| malformed(format!("{entry} has no name")))
        })
        .collect()
}

/// The globals' table's names in order.
pub fn globals(text: &str) -> Result<Vec<String>, Error> {
    let text = uncommented(text);
    let names = names(&text);
    table(&text, "hs_external_globals[", GLOBALS_FILE)?
        .into_iter()
        .map(|entry| {
            names
                .get(entry)
                .map(|name| (*name).to_owned())
                .ok_or_else(|| Error::Malformed {
                    file: GLOBALS_FILE.to_owned(),
                    line: 0,
                    why: format!("{entry} has no name"),
                })
        })
        .collect()
}

/// The engine's wired names: `special`, `function` or `global`, a tab and
/// the name; a line starting `#` is a comment.
pub fn read_wired(text: &str) -> Result<Wired, Error> {
    let mut wired = Wired::default();
    for (number, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let set = match line.split_once('\t') {
            Some(("special", name)) => (&mut wired.specials, name),
            Some(("function", name)) => (&mut wired.functions, name),
            Some(("global", name)) => (&mut wired.globals, name),
            _ => {
                return Err(Error::Malformed {
                    file: format!("engine/{WIRED}"),
                    line: number + 1,
                    why: "not special, function or global and a name".to_owned(),
                })
            }
        };
        set.0.insert(set.1.to_owned());
    }
    Ok(wired)
}

/// The original's functions then its globals, marked, then the engine's
/// own names in neither table.
pub fn rows(functions: &[(String, bool)], globals: &[String], wired: &Wired) -> Vec<Row> {
    let mut rows = Vec::new();
    for (name, maps) in functions {
        let engine = if wired.specials.contains(name) {
            Engine::Special
        } else if wired.functions.contains(name) {
            Engine::Wired
        } else {
            Engine::Missing
        };
        rows.push(Row {
            kind: Kind::Function,
            name: name.clone(),
            maps: Some(*maps),
            engine,
        });
    }
    for name in globals {
        rows.push(Row {
            kind: Kind::Global,
            name: name.clone(),
            maps: None,
            engine: if wired.globals.contains(name) {
                Engine::Wired
            } else {
                Engine::Missing
            },
        });
    }
    let original: BTreeSet<&str> = functions
        .iter()
        .map(|(name, _)| name.as_str())
        .chain(globals.iter().map(String::as_str))
        .collect();
    for (kind, names) in [
        (Kind::Function, &wired.specials),
        (Kind::Function, &wired.functions),
        (Kind::Global, &wired.globals),
    ] {
        for name in names {
            if !original.contains(name.as_str()) {
                rows.push(Row {
                    kind,
                    name: name.clone(),
                    maps: None,
                    engine: Engine::Extra,
                });
            }
        }
    }
    rows
}

/// The marked tables as `engine/census/scripts.tsv`.
pub fn write_rows(rows: &[Row]) -> String {
    let mut text = String::from(
        "# The original's script functions and globals (opence_census::scripts), written by opence census --write.\n\
         # kind\tname\tmaps (whether a map may call it)\tengine (wired, special, missing or extra)\n",
    );
    for row in rows {
        let kind = match row.kind {
            Kind::Function => "function",
            Kind::Global => "global",
        };
        let maps = match row.maps {
            Some(true) => "yes",
            Some(false) => "no",
            None => "-",
        };
        let engine = match row.engine {
            Engine::Wired => "wired",
            Engine::Special => "special",
            Engine::Missing => "missing",
            Engine::Extra => "extra",
        };
        text.push_str(&format!("{kind}\t{}\t{maps}\t{engine}\n", row.name));
    }
    text
}

/// The rows of `engine/census/scripts.tsv`.
pub fn read_rows(text: &str) -> Result<Vec<Row>, Error> {
    let mut rows = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let malformed = |why: &str| Error::Malformed {
            file: format!("engine/{SCRIPTS}"),
            line: number + 1,
            why: why.to_owned(),
        };
        let columns: Vec<&str> = line.split('\t').collect();
        let [kind, name, maps, engine] = columns[..] else {
            return Err(malformed("not four columns"));
        };
        rows.push(Row {
            kind: match kind {
                "function" => Kind::Function,
                "global" => Kind::Global,
                _ => return Err(malformed("not function or global")),
            },
            name: name.to_owned(),
            maps: match maps {
                "yes" => Some(true),
                "no" => Some(false),
                "-" => None,
                _ => return Err(malformed("maps is not yes, no or -")),
            },
            engine: match engine {
                "wired" => Engine::Wired,
                "special" => Engine::Special,
                "missing" => Engine::Missing,
                "extra" => Engine::Extra,
                _ => return Err(malformed("not wired, special, missing or extra")),
            },
        });
    }
    Ok(rows)
}

/// The marked tables of the repository at `root` (none in a tree without
/// the original's tables).
pub fn generate(root: &Path) -> Result<Vec<Row>, Error> {
    if !root.join(FUNCTIONS_FILE).exists() {
        return Ok(Vec::new());
    }
    let functions = functions(&read_text(&root.join(FUNCTIONS_FILE))?)?;
    let globals = globals(&read_text(&root.join(GLOBALS_FILE))?)?;
    let wired = read_wired(&read_text(&root.join("engine").join(WIRED)).unwrap_or_default())?;
    Ok(rows(&functions, &globals, &wired))
}

/// The written tables, if they are not what `generate` finds.
pub fn stale(root: &Path) -> Result<Vec<String>, Error> {
    let written = read_rows(&read_text(&root.join("engine").join(SCRIPTS)).unwrap_or_default())?;
    Ok(if written == generate(root)? {
        Vec::new()
    } else {
        vec![format!(
            "engine/{SCRIPTS} is stale: run opence census --write"
        )]
    })
}
