//! The tag fields: each tag structure's fields, from the layout, marked
//! with whether the original reads them.
//!
//! The original reads a tag through a pointer it declares to the tag's
//! structure (`struct unit_definition *definition= ...`, a function's
//! parameter or a local), and then a field through that pointer
//! (`definition->unit.seats`). The census follows exactly that: within each
//! function, the pointers declared to a laid-out structure, and the
//! members declared as one (`camera_info.camera->`), and the paths read
//! through them, array indexes left out. A path reaching a field
//! that holds smaller ones (a point, a tag block) reads all of them; one
//! reaching an embedded tag structure (`unit.object`) does not, since the
//! original reads that through a pointer of the structure's own. What it
//! reads in an embedded structure, by either way, is read wherever that
//! structure is embedded: `object_definition->object.flags` reads the
//! flags of every unit and weapon too.
//!
//! `opence census --write` keeps the marks in `engine/census/tag_fields.tsv`;
//! `--check` marks them again and compares.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::classes::Class;
use crate::layout::{Field, Record};
use crate::{read_text, walk, Error};

/// The written fields, from `engine/`.
pub const FIELDS: &str = "census/tag_fields.tsv";

/// The hand-kept traces, from `engine/`: for each kept field whose use
/// the census cannot follow, the engine file that uses it and the text of
/// that use.
pub const TRACES: &str = "census/field_traces.tsv";

/// The kept fields the engine uses nowhere, each with its reason, from
/// `engine/` (`opence census --write`).
pub const UNUSED: &str = "census/unused_fields.tsv";

/// How deep embedded structures are followed.
const DEPTH: usize = 8;

/// A field of a tag structure, marked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mark {
    /// The structure.
    pub structure: String,
    /// The field's path in it.
    pub path: String,
    /// Its byte offset.
    pub offset: u32,
    /// Its type, as clang prints it.
    pub kind: String,
    /// Whether the original reads it.
    pub original: bool,
    /// Whether the importer keeps it (reads it from a map).
    pub importer: bool,
    /// Whether the engine uses what the importer keeps of it.
    pub engine: Engine,
}

/// Whether the engine uses a field the importer keeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    /// The importer does not keep it.
    Unkept,
    /// The engine reads what the importer fills from it.
    Used,
    /// A block the importer reads only to walk its elements, which are
    /// marked on their own: its count and address fill nothing.
    Walked,
    /// What the importer fills from it is read nowhere in the engine.
    Unused,
    /// A read that keeps it could not be traced to what it fills.
    Untraced,
}

/// The fields of `record` that hold no smaller ones, once each, in order.
pub fn leaves(record: &Record) -> Vec<&Field> {
    let paths: BTreeSet<&str> = record.fields.iter().map(|f| f.path.as_str()).collect();
    let mut seen = BTreeSet::new();
    record
        .fields
        .iter()
        .filter(|field| {
            let inside = format!("{}.", field.path);
            !paths
                .range(inside.as_str()..)
                .next()
                .is_some_and(|p| p.starts_with(&inside))
        })
        .filter(|field| seen.insert(field.path.as_str()))
        .collect()
}

/// A token of the C text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Token<'a> {
    /// An identifier or keyword.
    Name(&'a str),
    /// One character of punctuation.
    Punct(u8),
}

/// The text's names and punctuation, without comments, literals and
/// preprocessor lines.
pub(crate) fn tokens(text: &str) -> Vec<Token<'_>> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    let mut line_start = true;
    while let Some(&byte) = bytes.get(at) {
        let rest = bytes.get(at..).unwrap_or_default();
        if byte == b'\n' {
            line_start = true;
            at += 1;
            continue;
        }
        if byte.is_ascii_whitespace() {
            at += 1;
            continue;
        }
        if line_start && byte == b'#' {
            // The directive, through its continued lines.
            while let Some(&b) = bytes.get(at) {
                if b == b'\n' && bytes.get(at.wrapping_sub(1)) != Some(&b'\\') {
                    break;
                }
                at += 1;
            }
            continue;
        }
        line_start = false;
        if rest.starts_with(b"//") {
            while bytes.get(at).is_some_and(|&b| b != b'\n') {
                at += 1;
            }
        } else if rest.starts_with(b"/*") {
            at = find(bytes, at + 2, b"*/").map_or(bytes.len(), |end| end + 2);
        } else if byte == b'"' || byte == b'\'' {
            at += 1;
            while let Some(&b) = bytes.get(at) {
                at += if b == b'\\' { 2 } else { 1 };
                if b == byte {
                    break;
                }
            }
        } else if byte.is_ascii_alphabetic() || byte == b'_' {
            let start = at;
            while bytes
                .get(at)
                .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'_')
            {
                at += 1;
            }
            if let Some(name) = text.get(start..at) {
                found.push(Token::Name(name));
            }
        } else if byte.is_ascii_digit() {
            while bytes
                .get(at)
                .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'.')
            {
                at += 1;
            }
        } else {
            found.push(Token::Punct(byte));
            at += 1;
        }
    }
    found
}

/// Where `needle` next starts in `bytes` from `from`.
fn find(bytes: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    bytes
        .get(from..)?
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|at| at + from)
}

/// The token after the `[` at `open` that closes it.
fn closing(tokens: &[Token<'_>], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (at, token) in tokens.iter().enumerate().skip(open) {
        match token {
            Token::Punct(b'[') => depth += 1,
            Token::Punct(b']') => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// The pointer `struct S *name` declared at `at`, if `S` is one of
/// `structures`: the name and `S`.
fn declaration<'a>(
    tokens: &[Token<'a>],
    at: usize,
    structures: &BTreeSet<String>,
) -> Option<(&'a str, &'a str)> {
    let mut next = at + 1;
    let Some(Token::Name(structure)) = tokens.get(next) else {
        return None;
    };
    if !structures.contains(*structure) {
        return None;
    }
    next += 1;
    let qualifier =
        |token: Option<&Token<'_>>| matches!(token, Some(Token::Name("const" | "volatile")));
    while qualifier(tokens.get(next)) {
        next += 1;
    }
    if tokens.get(next) != Some(&Token::Punct(b'*')) {
        return None;
    }
    next += 1;
    while qualifier(tokens.get(next)) {
        next += 1;
    }
    match tokens.get(next) {
        Some(Token::Name(name)) => Some((name, structure)),
        _ => None,
    }
}

/// The path read through the pointer at `at` (`name->a[i].b`), if it
/// is one.
pub(crate) fn access(tokens: &[Token<'_>], at: usize) -> Option<String> {
    if tokens.get(at + 1) != Some(&Token::Punct(b'-'))
        || tokens.get(at + 2) != Some(&Token::Punct(b'>'))
    {
        return None;
    }
    let Some(Token::Name(first)) = tokens.get(at + 3) else {
        return None;
    };
    let mut path = vec![*first];
    let mut next = at + 4;
    loop {
        match tokens.get(next) {
            Some(Token::Punct(b'[')) => next = closing(tokens, next)? + 1,
            Some(Token::Punct(b'.')) => match tokens.get(next + 1) {
                Some(Token::Name(name)) => {
                    path.push(name);
                    next += 2;
                }
                _ => break,
            },
            _ => break,
        }
    }
    Some(path.join("."))
}

/// Every pointer `text` declares to one of `structures` (a parameter, a
/// local or a member of a structure of its own): the name and the
/// structures it is declared as.
pub fn declared(text: &str, structures: &BTreeSet<String>) -> BTreeMap<String, BTreeSet<String>> {
    let tokens = tokens(text);
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (at, token) in tokens.iter().enumerate() {
        if *token == Token::Name("struct") {
            if let Some((name, structure)) = declaration(&tokens, at, structures) {
                found
                    .entry(name.to_owned())
                    .or_default()
                    .insert(structure.to_owned());
            }
        }
    }
    found
}

/// The names of `declared` that are only ever declared as one structure,
/// with it: the members a read through `holder.member->` can trust.
pub fn members(declared: &BTreeMap<String, BTreeSet<String>>) -> BTreeMap<String, String> {
    declared
        .iter()
        .filter_map(
            |(name, structures)| match structures.iter().collect::<Vec<_>>()[..] {
                [only] => Some((name.clone(), only.clone())),
                _ => None,
            },
        )
        .collect()
}

/// The fields `text` reads through pointers to `structures`: each
/// structure with the path read, once each. A pointer is a parameter or a
/// local the function declares, or a member (`info.camera->`) of
/// `members`.
pub fn reads(
    text: &str,
    structures: &BTreeSet<String>,
    members: &BTreeMap<String, String>,
) -> BTreeSet<(String, String)> {
    readers(text, structures, members)
        .into_values()
        .flatten()
        .collect()
}

/// The fields each function of `text` reads, as `reads` finds them: the
/// function is the last name before a `(` at the top level, outside any
/// parentheses (so not a function pointer parameter's type), ahead of the
/// body that holds the read, and a read outside any function is the
/// empty name's.
pub fn readers(
    text: &str,
    structures: &BTreeSet<String>,
    members: &BTreeMap<String, String>,
) -> BTreeMap<String, BTreeSet<(String, String)>> {
    let tokens = tokens(text);
    let mut found: BTreeMap<String, BTreeSet<(String, String)>> = BTreeMap::new();
    let mut pointers: BTreeMap<&str, &str> = BTreeMap::new();
    let mut depth = 0usize;
    let mut parentheses = 0usize;
    let mut candidate = "";
    let mut function = "";
    for (at, token) in tokens.iter().enumerate() {
        match token {
            Token::Punct(b'{') => {
                if depth == 0 {
                    function = candidate;
                }
                depth += 1;
            }
            Token::Punct(b'}') => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    pointers.clear();
                    candidate = "";
                    function = "";
                }
            }
            Token::Punct(b'(') => parentheses += 1,
            Token::Punct(b')') => parentheses = parentheses.saturating_sub(1),
            Token::Punct(b';' | b'=') if depth == 0 => {
                if *token == Token::Punct(b';') {
                    pointers.clear();
                }
                candidate = "";
            }
            Token::Name("struct") => {
                if let Some((name, structure)) = declaration(&tokens, at, structures) {
                    pointers.insert(name, structure);
                }
            }
            Token::Name(name) => {
                if depth == 0 && parentheses == 0 && tokens.get(at + 1) == Some(&Token::Punct(b'('))
                {
                    candidate = name;
                }
                let member = at.checked_sub(1).and_then(|i| tokens.get(i))
                    == Some(&Token::Punct(b'.'))
                    || at.checked_sub(1).and_then(|i| tokens.get(i)) == Some(&Token::Punct(b'>'))
                        && at.checked_sub(2).and_then(|i| tokens.get(i))
                            == Some(&Token::Punct(b'-'));
                let structure = if member {
                    members.get(*name).map(String::as_str)
                } else {
                    pointers.get(name).copied()
                };
                if let Some(structure) = structure {
                    if let Some(path) = access(&tokens, at) {
                        found
                            .entry(function.to_owned())
                            .or_default()
                            .insert((structure.to_owned(), path));
                    }
                }
            }
            Token::Punct(_) => {}
        }
    }
    found
}

/// The structure a field of type `kind` embeds, if it is a laid-out one.
fn embedded<'a>(kind: &'a str, records: &BTreeMap<&str, &Record>) -> Option<&'a str> {
    let name = kind.strip_prefix("struct ")?;
    (!name.contains(['[', '*', ':']) && records.contains_key(name)).then_some(name)
}

/// Whether a read of `read` in `record` reaches its field `path` there,
/// not through a structure `path` is inside: the field itself, one inside
/// it, or a part of a field the census does not lay out.
fn directly(record: &Record, path: &str, read: &str, records: &BTreeMap<&str, &Record>) -> bool {
    if path == read || read.starts_with(&format!("{path}.")) {
        return true;
    }
    path.starts_with(&format!("{read}."))
        && record
            .fields
            .iter()
            .find(|f| f.path == read)
            .is_some_and(|f| embedded(&f.kind, records).is_none())
}

/// Whether the original reads `path` of `record`.
fn is_read(
    record: &Record,
    path: &str,
    records: &BTreeMap<&str, &Record>,
    read: &BTreeMap<&str, Vec<&str>>,
    depth: usize,
) -> bool {
    let direct = read
        .get(record.name.as_str())
        .is_some_and(|paths| paths.iter().any(|a| directly(record, path, a, records)));
    direct
        || depth < DEPTH
            && record.fields.iter().any(|field| {
                let Some(inner) = embedded(&field.kind, records) else {
                    return false;
                };
                let Some(rest) = path.strip_prefix(&format!("{}.", field.path)) else {
                    return false;
                };
                records
                    .get(inner)
                    .is_some_and(|r| is_read(r, rest, records, read, depth + 1))
            })
}

/// The read of `path` in `structure` again in each structure it reaches
/// inside (`object.flags` of a `unit_definition` is `flags` of its
/// `_object_definition`), so that every structure embedding that one
/// has it read.
fn innermost(
    structure: &str,
    path: &str,
    records: &BTreeMap<&str, &Record>,
    found: &mut BTreeSet<(String, String)>,
    depth: usize,
) {
    let Some(record) = records.get(structure) else {
        return;
    };
    for field in &record.fields {
        let Some(inner) = embedded(&field.kind, records) else {
            continue;
        };
        if let Some(rest) = path.strip_prefix(&format!("{}.", field.path)) {
            if depth < DEPTH && found.insert((inner.to_owned(), rest.to_owned())) {
                innermost(inner, rest, records, found, depth + 1);
            }
        }
    }
}

/// For every leaf of `records`, in their order, whether `reads` reach
/// it.
fn reached(records: &[Record], reads: &BTreeSet<(String, String)>) -> Vec<bool> {
    let by_name: BTreeMap<&str, &Record> = records.iter().map(|r| (r.name.as_str(), r)).collect();
    let mut inner = BTreeSet::new();
    for (structure, path) in reads {
        innermost(structure, path, &by_name, &mut inner, 0);
    }
    let mut read: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (structure, path) in reads.iter().chain(&inner) {
        read.entry(structure.as_str())
            .or_default()
            .push(path.as_str());
    }
    records
        .iter()
        .flat_map(|record| {
            leaves(record)
                .into_iter()
                .map(|field| is_read(record, &field.path, &by_name, &read, 0))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// For every leaf of `records`, in their order, the functions of
/// `readers` whose reads reach it, as `marks` marks a leaf read: each
/// read reaches the leaves it reads directly, in its structure and the
/// ones inside it, and each structure embedding one of those has that
/// leaf read too.
pub fn readers_of(
    records: &[Record],
    readers: &BTreeMap<String, BTreeSet<(String, String)>>,
) -> Vec<BTreeSet<String>> {
    let by_name: BTreeMap<&str, &Record> = records.iter().map(|r| (r.name.as_str(), r)).collect();
    let mut index: BTreeMap<(&str, &str), usize> = BTreeMap::new();
    let mut embedders: BTreeMap<&str, Vec<(&str, &str)>> = BTreeMap::new();
    let mut count = 0;
    for record in records {
        for field in leaves(record) {
            index.insert((record.name.as_str(), field.path.as_str()), count);
            count += 1;
        }
        for field in &record.fields {
            if let Some(inner) = embedded(&field.kind, &by_name) {
                embedders
                    .entry(inner)
                    .or_default()
                    .push((record.name.as_str(), field.path.as_str()));
            }
        }
    }
    let mut functions: BTreeMap<&(String, String), BTreeSet<&String>> = BTreeMap::new();
    for (function, reads) in readers {
        for read in reads {
            functions.entry(read).or_default().insert(function);
        }
    }
    let mut found = vec![BTreeSet::new(); count];
    for ((structure, path), by) in functions {
        let mut expanded = BTreeSet::from([(structure.clone(), path.clone())]);
        innermost(structure, path, &by_name, &mut expanded, 0);
        let mut frontier = Vec::new();
        for (structure, read) in &expanded {
            let Some(record) = by_name.get(structure.as_str()) else {
                continue;
            };
            for field in leaves(record) {
                if directly(record, &field.path, read, &by_name) {
                    frontier.push((record.name.as_str(), field.path.clone(), 0));
                }
            }
        }
        let mut reached = BTreeSet::new();
        while let Some((structure, path, depth)) = frontier.pop() {
            let Some(&at) = index.get(&(structure, path.as_str())) else {
                continue;
            };
            if !reached.insert(at) || depth >= DEPTH {
                continue;
            }
            for (outer, field) in embedders.get(structure).into_iter().flatten() {
                frontier.push((outer, format!("{field}.{path}"), depth + 1));
            }
        }
        for at in reached {
            if let Some(leaf) = found.get_mut(at) {
                leaf.extend(by.iter().map(|f| (*f).clone()));
            }
        }
    }
    found
}

/// Every leaf of `records`, in their order, marked with whether `reads`
/// reach it (and none kept by the importer).
pub fn marks(records: &[Record], reads: &BTreeSet<(String, String)>) -> Vec<Mark> {
    let reached = reached(records, reads);
    records
        .iter()
        .flat_map(|record| {
            leaves(record).into_iter().map(|field| Mark {
                structure: record.name.clone(),
                path: field.path.clone(),
                offset: field.offset,
                kind: field.kind.clone(),
                original: false,
                importer: false,
                engine: Engine::Unkept,
            })
        })
        .zip(reached)
        .map(|(mark, original)| Mark { original, ..mark })
        .collect()
}

/// `marks` of `records` with the fields the importer `kept` marked, as
/// the original's reads are (a field kept in an embedded structure is kept
/// wherever that structure is embedded).
pub fn keep(marks: &mut [Mark], records: &[Record], kept: &BTreeSet<(String, String)>) {
    for (mark, importer) in marks.iter_mut().zip(reached(records, kept)) {
        mark.importer = importer;
    }
}

/// `marks` of `records` with what the engine does with each kept field:
/// the `labels` of the reads that keep it are traced through `assigned`
/// to what they fill, and that is used if the engine reads it (`used`).
/// A field none of whose reads fills anything the engine uses, and all of
/// whose reads are blocks the importer only walks (`walked`), is walked.
/// A field kept in an embedded structure is marked wherever that
/// structure is embedded, as `keep` marks it.
pub fn engine(
    marks: &mut [Mark],
    records: &[Record],
    labels: &BTreeMap<(String, String), BTreeSet<String>>,
    assigned: &BTreeMap<String, BTreeSet<String>>,
    walked: &BTreeSet<String>,
    used: &BTreeSet<String>,
) {
    let mut using = BTreeSet::new();
    let mut unused = BTreeSet::new();
    let mut walking = BTreeSet::new();
    for (field, labels) in labels {
        let mut traced = true;
        let mut fills = BTreeSet::new();
        for label in labels {
            match assigned.get(label) {
                Some(idents) => fills.extend(idents),
                None => traced = false,
            }
        }
        if fills.iter().any(|ident| used.contains(*ident)) {
            using.insert(field.clone());
        } else if !labels.is_empty() && labels.iter().all(|label| walked.contains(label)) {
            walking.insert(field.clone());
        } else if traced {
            unused.insert(field.clone());
        }
    }
    let using = reached(records, &using);
    let unused = reached(records, &unused);
    let walking = reached(records, &walking);
    for (((mark, using), unused), walking) in marks.iter_mut().zip(using).zip(unused).zip(walking) {
        mark.engine = match (mark.importer, using, walking, unused) {
            (false, ..) => Engine::Unkept,
            (true, true, ..) => Engine::Used,
            (true, false, true, _) => Engine::Walked,
            (true, false, false, true) => Engine::Unused,
            (true, false, false, false) => Engine::Untraced,
        };
    }
}

/// The fields of `records` each function of `source/`'s C in the
/// repository at `root` reads (`readers`).
pub fn source_readers(
    root: &Path,
    records: &[Record],
) -> Result<BTreeMap<String, BTreeSet<(String, String)>>, Error> {
    let structures: BTreeSet<String> = records.iter().map(|r| r.name.clone()).collect();
    let mut texts = Vec::new();
    let mut all: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for file in walk(&root.join("source"), &["c", "h"])? {
        let text = read_text(&file)?;
        for (name, types) in declared(&text, &structures) {
            all.entry(name).or_default().extend(types);
        }
        if file.extension().is_some_and(|e| e == "c") {
            texts.push(text);
        }
    }
    let members = members(&all);
    let mut by: BTreeMap<String, BTreeSet<(String, String)>> = BTreeMap::new();
    for text in &texts {
        for (function, read) in readers(text, &structures, &members) {
            by.entry(function).or_default().extend(read);
        }
    }
    Ok(by)
}

/// A kept field's use, named by hand where the census cannot follow it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
    /// The structure.
    pub structure: String,
    /// The field's path in it.
    pub path: String,
    /// The file that uses it, from `engine/crates/`, or [`NO_FILE`] where
    /// no Rust code does.
    pub file: String,
    /// The text of the use in that file.
    pub uses: String,
}

/// A trace's file where no Rust code uses its field: game.wasm's own
/// reader does, and the field is marked unused (which [`gaps`] lets stand
/// only when a reader of it is compiled).
pub const NO_FILE: &str = "-";

/// `field_traces.tsv` read: a line for each field, its structure, path,
/// file and use.
pub fn read_traces(text: &str) -> Result<Vec<Trace>, Error> {
    let mut traces = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let [structure, path, file, uses] = line.split('\t').collect::<Vec<_>>()[..] else {
            return Err(Error::Malformed {
                file: format!("engine/{TRACES}"),
                line: index + 1,
                why: "not four columns".to_owned(),
            });
        };
        traces.push(Trace {
            structure: structure.to_owned(),
            path: path.to_owned(),
            file: file.to_owned(),
            uses: uses.to_owned(),
        });
    }
    Ok(traces)
}

/// Marks each untraced field `traces` names used, where the text of its
/// file (`text`) holds its use, or unused, where its file is [`NO_FILE`]; the problems are the traces whose use is
/// not there, that name a field the census traces itself, or that name no
/// kept field.
pub fn trace(
    marks: &mut [Mark],
    traces: &[Trace],
    text: impl Fn(&str) -> Option<String>,
) -> Vec<String> {
    let mut problems = Vec::new();
    for t in traces {
        let name = format!("engine/{TRACES}: {}.{}", t.structure, t.path);
        let found = marks
            .iter_mut()
            .find(|m| m.importer && m.structure == t.structure && m.path == t.path);
        match found {
            None => problems.push(format!("{name} names no kept field")),
            Some(mark) if mark.engine != Engine::Untraced => {
                problems.push(format!("{name} needs no trace: the census traces it"));
            }
            Some(mark) if t.file == NO_FILE => mark.engine = Engine::Unused,
            Some(mark) => {
                if text(&t.file).is_some_and(|text| text.contains(&t.uses)) {
                    mark.engine = Engine::Used;
                } else {
                    problems.push(format!("{name}'s use {} is not in {}", t.uses, t.file));
                }
            }
        }
    }
    problems
}

/// The marks of `records` against `source/`'s C in the repository at
/// `root`.
pub fn generate(root: &Path, records: &[Record]) -> Result<Vec<Mark>, Error> {
    Ok(generate_traced(root, records)?.0)
}

/// `generate`, with the problems of its hand-kept traces (`trace`).
fn generate_traced(root: &Path, records: &[Record]) -> Result<(Vec<Mark>, Vec<String>), Error> {
    let found: BTreeSet<(String, String)> = source_readers(root, records)?
        .into_values()
        .flatten()
        .collect();
    let mut marked = marks(records, &found);
    let kept = crate::importer::kept(root, records)?;
    keep(&mut marked, records, &kept.fields);
    let (assigned, walked, used) = crate::engine::traced(root)?;
    engine(
        &mut marked,
        records,
        &kept.labels,
        &assigned,
        &walked,
        &used,
    );
    let engine = root.join("engine");
    let traces = read_traces(&read_text(&engine.join(TRACES)).unwrap_or_default())?;
    let crates = engine.join("crates");
    let problems = trace(&mut marked, &traces, |file| {
        read_text(&crates.join(file)).ok()
    });
    Ok((marked, problems))
}

/// `tag_fields.tsv`'s text: a line for each field.
pub fn write_fields(marks: &[Mark]) -> String {
    let mut text = String::from(
        "# structure\tfield\toffset\ttype\toriginal\timporter\tengine (opence census --write)\n",
    );
    for mark in marks {
        text.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            mark.structure,
            mark.path,
            mark.offset,
            mark.kind,
            if mark.original { "read" } else { "-" },
            if mark.importer { "kept" } else { "-" },
            match mark.engine {
                Engine::Unkept => "-",
                Engine::Used => "used",
                Engine::Walked => "walked",
                Engine::Unused => "unused",
                Engine::Untraced => "untraced",
            }
        ));
    }
    text
}

/// `tag_fields.tsv` read back.
pub fn read_fields(text: &str) -> Result<Vec<Mark>, Error> {
    let mut marks = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let malformed = |why: &str| Error::Malformed {
            file: format!("engine/{FIELDS}"),
            line: index + 1,
            why: why.to_owned(),
        };
        let columns: Vec<&str> = line.split('\t').collect();
        let [structure, path, offset, kind, original, importer, engine] = columns[..] else {
            return Err(malformed("not seven columns"));
        };
        marks.push(Mark {
            structure: structure.to_owned(),
            path: path.to_owned(),
            offset: offset.parse().map_err(|_| malformed("not a number"))?,
            kind: kind.to_owned(),
            original: match original {
                "read" => true,
                "-" => false,
                _ => return Err(malformed("neither read nor -")),
            },
            importer: match importer {
                "kept" => true,
                "-" => false,
                _ => return Err(malformed("neither kept nor -")),
            },
            engine: match engine {
                "-" => Engine::Unkept,
                "used" => Engine::Used,
                "walked" => Engine::Walked,
                "unused" => Engine::Unused,
                "untraced" => Engine::Untraced,
                _ => return Err(malformed("not used, walked, unused, untraced or -")),
            },
        });
    }
    Ok(marks)
}

/// The kept fields the engine owes a use: each untraced one, and each
/// unused one the original reads in a function the engine has finished
/// (Ported, Matched or Native) and in none it has yet to finish. `marks`
/// and `readers` are in the same order (`readers_of`); `class` is each
/// reader's class. A field with a Missing or Partial reader is owed to
/// that function's track, and one read only where the engine replaced
/// the original owes nothing.
pub fn gaps(
    marks: &[Mark],
    readers: &[BTreeSet<String>],
    class: impl Fn(&str) -> Option<Class>,
) -> Vec<String> {
    let mut gaps = Vec::new();
    for (mark, by) in marks.iter().zip(readers) {
        let name = format!("{}.{}", mark.structure, mark.path);
        match mark.engine {
            Engine::Untraced => gaps.push(format!("{name}: kept, but its use could not be traced")),
            Engine::Unused => {
                let classes: Vec<Option<Class>> = by.iter().map(|f| class(f)).collect();
                let finished = |c: &Option<Class>| {
                    matches!(c, Some(Class::Ported | Class::Matched | Class::Native))
                };
                let owed = |c: &Option<Class>| matches!(c, Some(Class::Missing | Class::Partial));
                // (game.wasm runs the original's own reader)
                let compiled = classes.contains(&Some(Class::Compiled));
                if classes.iter().any(finished) && !classes.iter().any(owed) && !compiled {
                    let list: Vec<String> = by
                        .iter()
                        .zip(&classes)
                        .map(|(f, c)| format!("{f} ({})", c.map_or("?", Class::name)))
                        .collect();
                    gaps.push(format!(
                        "{name}: read by {}, used nowhere in the engine",
                        list.join(", ")
                    ));
                }
            }
            _ => {}
        }
    }
    gaps
}

/// Each unused field of `marks` as a line of `unused_fields.tsv`: its
/// structure, path and why the engine owes it nothing yet. The original
/// reads it nowhere; or it is owed to the Missing and Partial functions
/// that read it, whose tracks use it; or it is read only where the engine
/// replaced the original, and perhaps by functions that are not the
/// original's (the old port's checks); or it is a gap (`gaps`). `marks`, `readers` and
/// `class` are as for `gaps`.
pub fn unused(
    marks: &[Mark],
    readers: &[BTreeSet<String>],
    class: impl Fn(&str) -> Option<Class>,
) -> Vec<String> {
    let mut lines = Vec::new();
    for (mark, by) in marks.iter().zip(readers) {
        if mark.engine != Engine::Unused {
            continue;
        }
        let classed: Vec<(&String, Option<Class>)> = by.iter().map(|f| (f, class(f))).collect();
        let owed: Vec<String> = classed
            .iter()
            .filter(|(_, c)| matches!(c, Some(Class::Missing | Class::Partial)))
            .map(|(f, c)| format!("{f} ({})", c.map_or("?", Class::name)))
            .collect();
        let replaced = classed
            .iter()
            .all(|(_, c)| matches!(c, Some(Class::Replaced)));
        let unfinished = classed
            .iter()
            .all(|(_, c)| matches!(c, Some(Class::Replaced) | None));
        let names: Vec<&str> = by.iter().map(String::as_str).collect();
        let compiled: Vec<&str> = classed
            .iter()
            .filter(|(_, c)| *c == Some(Class::Compiled))
            .map(|(f, _)| f.as_str())
            .collect();
        let reason = if by.is_empty() {
            "the original reads it nowhere".to_owned()
        } else if !compiled.is_empty() {
            format!("game.wasm reads it: {}", compiled.join(", "))
        } else if !owed.is_empty() {
            format!("owed to {}", owed.join(", "))
        } else if replaced {
            format!(
                "read only where the engine replaced the original: {}",
                names.join(", ")
            )
        } else if unfinished {
            let list: Vec<String> = classed
                .iter()
                .map(|(f, c)| match c {
                    Some(c) => format!("{f} ({})", c.name()),
                    None => format!("{f} (not the original's)"),
                })
                .collect();
            format!(
                "read only where the engine replaced the original or outside it: {}",
                list.join(", ")
            )
        } else {
            format!("a gap: read by {}", names.join(", "))
        };
        lines.push(format!("{}\t{}\t{reason}", mark.structure, mark.path));
    }
    lines
}

/// The written fields with each one's readers in `source/`.
type Written = (Vec<Mark>, Vec<BTreeSet<String>>);

/// The written fields, their readers in `source/`, and each reader's class
/// as `census` classes it; none if the fields are not the layout's.
fn written_with_readers(root: &Path) -> Result<Option<Written>, Error> {
    let engine = root.join("engine");
    let layout = read_text(&engine.join(crate::layout::LAYOUT)).unwrap_or_default();
    let records = crate::layout::read_layout(&layout)?;
    let written = read_fields(&read_text(&engine.join(FIELDS)).unwrap_or_default())?;
    let by = readers_of(&records, &source_readers(root, &records)?);
    Ok((by.len() == written.len()).then_some((written, by)))
}

/// `unused_fields.tsv`'s text for the written fields (`unused`).
pub fn write_unused(root: &Path, census: &crate::Census) -> Result<String, Error> {
    let mut text = String::from(
        "# structure\tfield\twhy the engine uses it nowhere (opence census --write)\n",
    );
    if let Some((written, by)) = written_with_readers(root)? {
        for line in unused(&written, &by, |f| census.reader_class(f)) {
            text.push_str(&line);
            text.push('\n');
        }
    }
    Ok(text)
}

/// The written fields' gaps (`gaps`), each reader classed as `census`
/// classes it, and `unused_fields.tsv` if it is stale.
pub fn owed(root: &Path, census: &crate::Census) -> Result<Vec<String>, Error> {
    let Some((written, by)) = written_with_readers(root)? else {
        return Ok(Vec::new());
    };
    let mut problems = gaps(&written, &by, |f| census.reader_class(f));
    let file = read_text(&root.join("engine").join(UNUSED)).unwrap_or_default();
    if file != write_unused(root, census)? {
        problems.push(format!(
            "engine/{UNUSED} is stale: run opence census --write"
        ));
    }
    Ok(problems)
}

/// The written fields, if they are not the marks of the written layout
/// (a tree with neither has none), and the problems of the hand-kept
/// traces.
pub fn stale(root: &Path) -> Result<Vec<String>, Error> {
    let engine = root.join("engine");
    let layout = read_text(&engine.join(crate::layout::LAYOUT)).unwrap_or_default();
    let records = crate::layout::read_layout(&layout)?;
    let written = read_fields(&read_text(&engine.join(FIELDS)).unwrap_or_default())?;
    let (fresh, mut problems) = generate_traced(root, &records)?;
    if written != fresh {
        problems.push(format!(
            "engine/{FIELDS} is stale: run opence census --write"
        ));
    }
    Ok(problems)
}
