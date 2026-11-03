//! The census's docs: `docs/census.md`, every function by system, and its
//! summary table in `docs/engine.md`. `opence census --write` writes both;
//! the check fails when either is not what the census writes.

use std::fmt::Write as _;
use std::path::Path;

use crate::census::{Census, Function};
use crate::classes::{Class, Entry, From};
use crate::{read_text, Error};

/// The census's own doc, from the repository's root.
pub const CENSUS_MD: &str = "docs/census.md";

/// The engine's doc, which holds the summary table.
pub const ENGINE_MD: &str = "docs/engine.md";

/// The line before the summary table in `docs/engine.md`.
pub const BEGIN: &str = "<!-- coverage census: opence census --write writes this table -->";

/// The line after it.
pub const END: &str = "<!-- end of the coverage census -->";

/// The system a function belongs to: its directory of `source/`.
pub fn system(function: &Function) -> &str {
    function.file.split('/').next().unwrap_or_default()
}

/// The entry or rule that gives `function` its class.
pub fn entry<'a>(census: &'a Census, function: &Function) -> Option<&'a Entry> {
    match function.from {
        From::Entry => census.manifest.function.get(&function.name),
        From::Rule(index) => census.manifest.rule.get(index),
        From::Trace | From::Game | From::Nothing => None,
    }
}

/// Where the engine does `function`'s job: its first citation, the
/// entry's engine file or its stand-in.
pub fn engine_place(census: &Census, function: &Function) -> String {
    let entry = entry(census, function);
    function
        .cited
        .first()
        .cloned()
        .or_else(|| entry.and_then(|e| e.engine.clone()))
        .or_else(|| entry.and_then(|e| e.stands_in.clone()))
        .unwrap_or_default()
}

/// The evidence for `function`'s class, in a few words.
pub fn evidence(census: &Census, function: &Function) -> String {
    let entry = entry(census, function);
    let field = |pick: fn(&Entry) -> Option<&String>| entry.and_then(pick).cloned();
    let mut parts = Vec::new();
    match function.class {
        Some(Class::Ported) => match &function.traced_by {
            Some(script) => parts.push(format!("`tools/{script}.py`")),
            None => {
                if let Some(through) = field(|e| e.through.as_ref()) {
                    parts.push(format!("through `{through}`'s trace"));
                }
                parts.extend(field(|e| e.why.as_ref()));
            }
        },
        Some(Class::Matched | Class::Partial) => {
            if let Some(check) = field(|e| e.check.as_ref()) {
                let against = entry
                    .and_then(|e| e.against)
                    .map(|a| format!(" ({})", format!("{a:?}").to_lowercase()))
                    .unwrap_or_default();
                parts.push(format!("{check}{against}"));
            }
            if let Some(e) = entry.filter(|e| !e.leaves_out.is_empty()) {
                parts.push(format!("leaves out {}", e.leaves_out.join(", ")));
            }
        }
        Some(Class::Compiled) => parts.push("compiled into game.wasm".to_owned()),
        Some(Class::Native) => {
            if let Some(primitive) = field(|e| e.primitive.as_ref()) {
                parts.push(format!("`{primitive}`"));
            }
        }
        Some(Class::Replaced) => {
            if let Some(caller) = &function.through_caller {
                parts.push(format!("through its caller `{caller}`"));
            } else if let Some(because) = entry.and_then(|e| e.because) {
                parts.push(because.text().to_owned());
            }
            parts.extend(field(|e| e.lives_on.as_ref()));
        }
        Some(Class::Missing) | None => {}
    }
    if let Some(owed) = &function.owed {
        parts.push(format!("owed: {owed}"));
    }
    parts.extend(field(|e| e.note.as_ref()));
    parts.join("; ").replace('|', "\\|")
}

/// A class's count and bytes among `functions`.
fn count<'a>(functions: impl Iterator<Item = &'a Function>, class: Class) -> (usize, u64) {
    functions
        .filter(|f| f.class == Some(class))
        .fold((0, 0), |(count, bytes), f| {
            (count + 1, bytes + u64::from(f.size))
        })
}

/// The summary table: each system's functions by class, in counts and
/// bytes, and how many owe evidence.
pub fn summary_table(census: &Census) -> String {
    let mut systems: Vec<&str> = census.functions.iter().map(system).collect();
    systems.sort_unstable();
    systems.dedup();
    let mut text = String::from("| System |");
    for class in Class::ALL {
        let _ = write!(text, " {} |", class.name());
    }
    text.push_str(" Owed |\n| --- |");
    for _ in Class::ALL {
        text.push_str(" ---: |");
    }
    text.push_str(" ---: |\n");
    let mut row = |name: &str, functions: &[&Function]| {
        let _ = write!(text, "| {name} |");
        for class in Class::ALL {
            match count(functions.iter().copied(), class) {
                (0, _) => text.push_str(" |"),
                (count, bytes) => {
                    let _ = write!(text, " {count} ({bytes} B) |");
                }
            }
        }
        let owed = functions.iter().filter(|f| f.owed.is_some()).count();
        let _ = writeln!(text, " {owed} |");
    };
    for name in systems {
        let functions: Vec<&Function> = census
            .functions
            .iter()
            .filter(|f| system(f) == name)
            .collect();
        row(&format!("`{name}`"), &functions);
    }
    let all: Vec<&Function> = census.functions.iter().collect();
    row("**all**", &all);
    text
}

/// `docs/census.md`.
pub fn census_md(census: &Census) -> String {
    let mut text = String::from(
        "# The coverage census\n\n\
         Every function of the original (`source/`), by system, with its class and\n\
         the evidence for it. `opence census --write` writes this file from\n\
         `engine/census/census.toml` and the reference traces; do not edit it.\n\
         docs/engine.md's \"The coverage census\" says what each class needs.\n\n",
    );
    text.push_str(&summary_table(census));
    let mut systems: Vec<&str> = census.functions.iter().map(system).collect();
    systems.sort_unstable();
    systems.dedup();
    for name in systems {
        let functions: Vec<&Function> = census
            .functions
            .iter()
            .filter(|f| system(f) == name)
            .collect();
        let _ = write!(text, "\n## `{name}`\n\n");
        let (classed, missing): (Vec<&Function>, Vec<&Function>) = functions
            .iter()
            .partition(|f| f.class != Some(Class::Missing) && f.class.is_some());
        if !classed.is_empty() {
            text.push_str("| Function | File | Class | Engine | Evidence |\n| --- | --- | --- | --- | --- |\n");
            for f in &classed {
                let _ = writeln!(
                    text,
                    "| `{}` | {} | {} | {} | {} |",
                    f.name,
                    f.file,
                    f.class.map(Class::name).unwrap_or_default(),
                    engine_place(census, f),
                    evidence(census, f)
                );
            }
        }
        if !missing.is_empty() {
            let reached = missing.iter().filter(|f| f.reached).count();
            let _ = write!(
                text,
                "\n{} Missing ({reached} reached from `main_loop`), by file:\n\n",
                missing.len()
            );
            let mut files: Vec<&str> = missing.iter().map(|f| f.file.as_str()).collect();
            files.sort_unstable();
            files.dedup();
            for file in files {
                let names: Vec<String> = missing
                    .iter()
                    .filter(|f| f.file == file)
                    .map(|f| format!("`{}`", f.name))
                    .collect();
                let _ = writeln!(text, "- {file}: {}", names.join(", "));
            }
        }
    }
    text
}

/// `engine_md` with the summary table put between its markers, or `None`
/// when it lacks them.
pub fn with_summary(engine_md: &str, table: &str) -> Option<String> {
    let start = engine_md.find(BEGIN)? + BEGIN.len();
    let end = engine_md[start..].find(END)? + start;
    Some(format!(
        "{}\n{table}{}",
        &engine_md[..start],
        &engine_md[end..]
    ))
}

/// The docs that are not what `census` writes.
pub fn stale(root: &Path, census: &Census) -> Vec<String> {
    let mut found = Vec::new();
    let written = read_text(&root.join(CENSUS_MD)).unwrap_or_default();
    if written != census_md(census) {
        found.push(format!("{CENSUS_MD} is stale: run opence census --write"));
    }
    let engine_md = read_text(&root.join(ENGINE_MD)).unwrap_or_default();
    match with_summary(&engine_md, &summary_table(census)) {
        Some(expected) if expected == engine_md => {}
        Some(_) => found.push(format!(
            "{ENGINE_MD}'s census table is stale: run opence census --write"
        )),
        None => found.push(format!("{ENGINE_MD} lacks the census table's markers")),
    }
    found
}

/// Writes both docs.
pub fn write(root: &Path, census: &Census) -> Result<(), Error> {
    let put = |file: &str, text: String| {
        let path = root.join(file);
        std::fs::write(&path, text).map_err(|source| Error::Io { path, source })
    };
    put(CENSUS_MD, census_md(census))?;
    let engine_md = read_text(&root.join(ENGINE_MD))?;
    match with_summary(&engine_md, &summary_table(census)) {
        Some(text) => put(ENGINE_MD, text),
        None => Err(Error::Problems(vec![format!(
            "{ENGINE_MD} lacks the census table's markers"
        )])),
    }
}
