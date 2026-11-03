//! The original's load-time fix-ups: what it does to a tag once it is
//! loaded, beyond reading it. A tag block definition names a default
//! element and a postprocess, a tag group a postprocess, and the load path
//! has its `*_postprocess` functions (the per-frame `*_postprocess_node_matrices`
//! are not load-time, and their names do not end so).
//!
//! `engine/census/fixups.tsv` is written by hand: each fix-up, how the
//! importer compares with it, and why. The check fails when a fix-up has
//! no row or a row names none.

use std::collections::BTreeMap;
use std::path::Path;

use crate::c::{definitions, Kind};
use crate::{read_text, walk, Error};

/// The manifest, under `engine/`.
pub const FIXUPS: &str = "census/fixups.tsv";

/// The entries of a tag block definition's initialiser that are fix-ups,
/// from 0, and what each is.
const BLOCK_ENTRIES: [(usize, &str); 2] = [(4, "default element"), (7, "postprocess")];
/// The entry of a tag group's initialiser that is its postprocess.
const GROUP_ENTRIES: [(usize, &str); 1] = [(5, "postprocess")];

/// How the importer compares with a fix-up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    /// The importer (or the engine past it) does the same.
    Same,
    /// It does otherwise, or not at all, where the fix-up changes
    /// something.
    Differs,
    /// The engine has its own way, with nothing of the fix-up's to match.
    Replaced,
    /// The fix-up changes nothing.
    None,
}

impl Comparison {
    fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "same" => Self::Same,
            "differs" => Self::Differs,
            "replaced" => Self::Replaced,
            "none" => Self::None,
            _ => return None,
        })
    }
}

/// A row of the manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    /// The fix-up's function.
    pub name: String,
    /// How the importer compares.
    pub comparison: Comparison,
    /// What the fix-up does and what the importer does instead.
    pub note: String,
}

/// The top-level entries of the initialiser that starts after `at`, to
/// its closing brace.
fn entries(text: &str, at: usize) -> Option<Vec<&str>> {
    let open = at + text.get(at..)?.find('{')?;
    let close = open + text.get(open..)?.find('}')?;
    Some(
        text.get(open + 1..close)?
            .split(',')
            .map(str::trim)
            .collect(),
    )
}

/// The fix-ups of one file's text, with where each is named: the
/// initialisers' entries first, then the functions it defines.
pub fn found(text: &str) -> Vec<(String, String)> {
    let mut fixups = Vec::new();
    for (keyword, wanted) in [
        ("struct tag_block_definition ", &BLOCK_ENTRIES[..]),
        ("struct tag_group ", &GROUP_ENTRIES[..]),
    ] {
        let mut from = 0;
        while let Some(start) = text.get(from..).and_then(|rest| rest.find(keyword)) {
            let start = from + start;
            from = start + keyword.len();
            let rest = text.get(from..).unwrap_or_default();
            let name_end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            let (name, after) = rest.split_at(name_end);
            if name.is_empty() || !after.trim_start().starts_with('=') {
                continue;
            }
            let Some(entries) = entries(text, from + name_end) else {
                continue;
            };
            fixups.push((start, name.to_owned(), entries, wanted));
        }
    }
    fixups.sort_by_key(|(start, ..)| *start);
    let mut out = Vec::new();
    for (_, name, entries, wanted) in fixups {
        for &(index, what) in wanted {
            let Some(&entry) = entries.get(index) else {
                continue;
            };
            let named = !entry.is_empty()
                && entry != "NULL"
                && entry
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_')
                && !entry.bytes().next().is_some_and(|b| b.is_ascii_digit());
            if named {
                out.push((entry.to_owned(), format!("{name}'s {what}")));
            }
        }
    }
    for definition in definitions(text) {
        let name = &definition.name;
        if definition.kind == Kind::Function
            && (name.starts_with("postprocess_") || name.ends_with("_postprocess"))
        {
            out.push((name.clone(), "defined".to_owned()));
        }
    }
    out
}

/// The manifest's rows; a line starting `#` is a comment.
pub fn read_fixups(text: &str) -> Result<Vec<Row>, Error> {
    let malformed = |line: usize, why: &str| Error::Malformed {
        file: format!("engine/{FIXUPS}"),
        line,
        why: why.to_owned(),
    };
    let mut rows = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let columns: Vec<&str> = line.split('\t').collect();
        let [name, comparison, note] = columns[..] else {
            return Err(malformed(number + 1, "not three columns"));
        };
        let comparison = Comparison::parse(comparison)
            .ok_or_else(|| malformed(number + 1, "not same, differs, replaced or none"))?;
        if name.is_empty() || note.trim().is_empty() {
            return Err(malformed(number + 1, "a name and a note are needed"));
        }
        rows.push(Row {
            name: name.to_owned(),
            comparison,
            note: note.to_owned(),
        });
    }
    Ok(rows)
}

/// What is wrong with the manifest's `rows` beside the `fixups` (each
/// with where it is named): a row twice or for no fix-up, then a fix-up
/// with no row.
pub fn problems(fixups: &BTreeMap<String, Vec<String>>, rows: &[Row]) -> Vec<String> {
    let mut found = Vec::new();
    let mut seen = BTreeMap::new();
    for row in rows {
        if seen.insert(row.name.as_str(), ()).is_some() {
            found.push(format!("{FIXUPS}: {} has two rows", row.name));
        } else if !fixups.contains_key(&row.name) {
            found.push(format!("{FIXUPS}: {} is no fix-up of source/", row.name));
        }
    }
    for (name, places) in fixups {
        if !seen.contains_key(name.as_str()) {
            found.push(format!(
                "{FIXUPS}: {name} ({}) has no row",
                places.join(", ")
            ));
        }
    }
    found
}

/// The fix-ups of `source/` in the repository at `root`.
pub fn generate(root: &Path) -> Result<BTreeMap<String, Vec<String>>, Error> {
    let mut fixups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in walk(&root.join("source"), &["c"])? {
        for (name, place) in found(&read_text(&file)?) {
            fixups.entry(name).or_default().push(place);
        }
    }
    Ok(fixups)
}

/// The manifest's problems in the repository at `root`.
pub fn stale(root: &Path) -> Result<Vec<String>, Error> {
    let text = read_text(&root.join("engine").join(FIXUPS)).unwrap_or_default();
    Ok(problems(&generate(root)?, &read_fixups(&text)?))
}
