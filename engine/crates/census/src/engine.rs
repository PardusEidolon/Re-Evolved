//! The tag fields the engine uses. The importer labels each read with the
//! field it is reading; the label's call is assigned to a field or local of
//! the importer's own types, and a field the engine reads through a `.`
//! outside the importer is used.
//!
//! The match is by name, so it is approximate: a name shared by two types
//! counts a use of either as a use of both.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::{read_text, walk, Error};

/// The crates whose code is not the engine's: the importer itself, this
/// census and the command line's tools.
const NOT_ENGINE: [&str; 3] = ["import", "census", "cli"];

/// `text` with its comments and the insides of its string literals
/// blanked, and the spans of those literals' insides.
fn code(text: &str) -> (Vec<u8>, Vec<(usize, usize)>) {
    let mut bytes = text.as_bytes().to_vec();
    let mut literals = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'/' if bytes.get(at + 1) == Some(&b'/') => {
                while at < bytes.len() && bytes[at] != b'\n' {
                    bytes[at] = b' ';
                    at += 1;
                }
            }
            // A character literal, so that '"' opens no string (a
            // lifetime has no closing quote two or three bytes on).
            b'\'' if bytes.get(at + 2) == Some(&b'\'') => at += 3,
            b'\'' if bytes.get(at + 1) == Some(&b'\\') && bytes.get(at + 3) == Some(&b'\'') => {
                at += 4;
            }
            b'"' => {
                let start = at + 1;
                at = start;
                while at < bytes.len() && bytes[at] != b'"' {
                    at += if bytes[at] == b'\\' { 2 } else { 1 };
                }
                let end = at.min(bytes.len());
                literals.push((start, end));
                for byte in &mut bytes[start..end] {
                    *byte = b' ';
                }
                at = end + 1;
            }
            _ => at += 1,
        }
    }
    (bytes, literals)
}

fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The identifier `segment` assigns: `let (mut) x =`, `let Some(x) =`,
/// `x:`, `path.x =` or `path.x.push(`.
fn target(segment: &str) -> Option<String> {
    let segment = segment.trim_start();
    let (segment, binding) = match segment.strip_prefix("let ") {
        Some(rest) => (
            rest.trim_start().trim_start_matches("mut ").trim_start(),
            true,
        ),
        None => (segment, false),
    };
    if binding {
        if let Some(inner) = segment.strip_prefix("Some(") {
            let end = inner
                .bytes()
                .position(|b| !is_ident(b))
                .unwrap_or(inner.len());
            let rest = inner[end..].trim_start();
            return (end > 0 && rest.starts_with(')')).then(|| inner[..end].to_owned());
        }
    } else {
        // (a field's list, not a local's: the local's name is not the
        // field's it ends up in)
        let squeezed: String = segment.split_whitespace().collect();
        if let Some((path, _)) = squeezed.split_once(".push(") {
            if path.contains('.') && path.bytes().all(|b| is_ident(b) || b == b'.') {
                return path
                    .rsplit('.')
                    .next()
                    .filter(|list| !list.is_empty())
                    .map(str::to_owned);
            }
        }
    }
    let path_end = segment
        .bytes()
        .position(|b| !(is_ident(b) || b == b'.'))
        .unwrap_or(segment.len());
    let path = &segment[..path_end];
    let ident = path.rsplit('.').next().unwrap_or_default();
    if ident.is_empty() || ident.bytes().next().is_some_and(|b| b.is_ascii_digit()) {
        return None;
    }
    let rest = segment[path_end..].trim_start();
    let assigns = rest.starts_with('=') && !rest.starts_with("==");
    let typed = rest.starts_with(':') && !rest.starts_with("::");
    let fits = if binding {
        !path.contains('.') && (assigns || typed)
    } else if path.contains('.') {
        assigns
    } else {
        typed
    };
    fits.then(|| ident.to_owned())
}

/// Where the expression holding the byte at `from` starts: the `,`, `;`,
/// `{` or `}` before it, outside the brackets it is in, once it has left
/// one (a call's or an array's) — `exited` if it already has.
fn stop_before(bytes: &[u8], from: usize, mut exited: bool) -> Option<usize> {
    let mut level = 0usize;
    for at in (0..from).rev() {
        match bytes[at] {
            b')' | b']' => level += 1,
            b'(' | b'[' if level > 0 => level -= 1,
            b'(' | b'[' => exited = true,
            b',' | b';' | b'{' | b'}' if level == 0 && exited => return Some(at),
            b';' | b'{' | b'}' if level == 0 => return None,
            _ => {}
        }
    }
    None
}

/// The `[` of the array literal whose element ends at the comma at
/// `comma`, if the comma is one of an array's.
fn element_of(bytes: &[u8], comma: usize) -> Option<usize> {
    let mut level = 0usize;
    for at in (0..comma).rev() {
        match bytes[at] {
            b')' | b']' | b'}' => level += 1,
            b'(' | b'[' | b'{' if level > 0 => level -= 1,
            b'[' => return Some(at),
            b'(' | b'{' | b';' if level == 0 => return None,
            _ => {}
        }
    }
    None
}

/// The array `*x = ...` in `segment` fills, where `x` is bound by a loop
/// of `before` over `array.iter_mut()`: the loops followed out, so that
/// an element of an element is the outer array's.
fn looped(before: &[u8], segment: &str) -> Option<String> {
    let deref = segment.trim_start().strip_prefix('*')?;
    let end = deref
        .bytes()
        .position(|b| !is_ident(b))
        .unwrap_or(deref.len());
    let rest = deref[end..].trim_start();
    if end == 0 || !rest.starts_with('=') || rest.starts_with("==") {
        return None;
    }
    let mut ident = deref[..end].to_owned();
    let mut before = before;
    let mut found = None;
    for _ in 0..4 {
        let Some((at, array)) = binding_loop(before, &ident) else {
            break;
        };
        found = Some(array.clone());
        ident = array;
        before = &before[..at];
    }
    found
}

/// The nearest loop of `before` that binds `ident`, and the array it runs
/// over: `for (i, ident) in array.iter_mut()...`.
fn binding_loop(before: &[u8], ident: &str) -> Option<(usize, String)> {
    let text = String::from_utf8_lossy(before);
    let mut search = text.len();
    while let Some(at) = text[..search].rfind("for ") {
        search = at;
        if at > 0 && is_ident(before[at - 1]) {
            continue;
        }
        let after = &text[at + 4..];
        let Some(within) = after.find(" in ") else {
            continue;
        };
        let pattern = &after[..within];
        if !pattern
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .any(|word| word == ident)
        {
            continue;
        }
        let expression = after[within + 4..].trim_start().trim_start_matches("&mut ");
        let path_end = expression
            .bytes()
            .position(|b| !(is_ident(b) || b == b'.'))
            .unwrap_or(expression.len());
        let path = expression[..path_end].strip_suffix(".iter_mut")?;
        let array = path.rsplit('.').next().unwrap_or_default();
        return (!array.is_empty()).then(|| (at, array.to_owned()));
    }
    None
}

/// Each string literal of `text` that is an argument of a call, with the
/// identifier the call's value is assigned to, in order.
pub fn assigned(text: &str) -> Vec<(String, String)> {
    let (bytes, literals) = code(text);
    let mut found = Vec::new();
    for (start, end) in literals {
        let open = start - 1;
        let before = bytes[..open].iter().rposition(|b| !b.is_ascii_whitespace());
        if !before.is_some_and(|at| bytes[at] == b'(' || bytes[at] == b',') {
            continue;
        }
        let mut from = open;
        let mut exited = false;
        // (an element of an array literal names nothing before its comma:
        // the array's own statement names what it fills)
        while let Some(stop) = stop_before(&bytes, from, exited) {
            let segment = String::from_utf8_lossy(&bytes[stop + 1..open]);
            if let Some(ident) = target(&segment).or_else(|| looped(&bytes[..stop], &segment)) {
                let label = text[start..end].to_owned();
                let stored = if segment.trim_start().starts_with("let ") {
                    stored(&bytes, end, &ident)
                } else {
                    Vec::new()
                };
                found.push((label.clone(), ident));
                found.extend(stored.into_iter().map(|field| (label.clone(), field)));
                break;
            }
            match element_of(&bytes, stop) {
                Some(bracket) if bytes[stop] == b',' => (from, exited) = (bracket, true),
                _ => break,
            }
        }
    }
    found
}

/// The fields of a structure literal the local `name`, bound in the
/// statement holding `from`, is stored in to the end of its block: each
/// `field: ..` whose value holds it, through calls and brackets.
fn stored(bytes: &[u8], from: usize, name: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut level = 0usize;
    let Some(bound) = bytes[from..].iter().position(|&b| b == b';') else {
        return found;
    };
    let mut at = from + bound + 1;
    while at < bytes.len() {
        match bytes[at] {
            b'(' | b'[' | b'{' => level += 1,
            b')' | b']' | b'}' if level == 0 => break,
            b')' | b']' | b'}' => level -= 1,
            _ => {}
        }
        let starts = at == 0 || !is_ident(bytes[at - 1]) && bytes[at - 1] != b'.';
        let end = at + name.len();
        if starts
            && bytes.get(at..end) == Some(name.as_bytes())
            && !bytes.get(end).is_some_and(|&b| is_ident(b))
            // (a local read through, such as a block walked for its
            // elements, is not itself stored)
            && bytes[end..].iter().find(|b| !b.is_ascii_whitespace()) != Some(&b'.')
        {
            if let Some(field) = field_holding(bytes, at) {
                if !found.contains(&field) {
                    found.push(field);
                }
            }
        }
        at += 1;
    }
    found
}

/// The field of the structure literal whose value holds the byte `at`:
/// out through any call or bracket around it to the `{` or `,` before
/// its `field:`.
fn field_holding(bytes: &[u8], at: usize) -> Option<String> {
    let mut level = 0usize;
    let mut separator = None;
    for back in (0..at).rev() {
        match bytes[back] {
            b')' | b']' | b'}' => level += 1,
            b'(' | b'[' | b'{' if level > 0 => level -= 1,
            b'(' | b'[' => separator = None,
            b'{' => {
                let start = separator.unwrap_or(back) + 1;
                let segment = String::from_utf8_lossy(&bytes[start..at]);
                let (field, rest) = segment.trim_start().split_once(':')?;
                let field = field.trim_end();
                let named = !field.is_empty() && field.bytes().all(is_ident);
                return (named && !rest.starts_with(':')).then(|| field.to_owned());
            }
            b',' if level == 0 && separator.is_none() => separator = Some(back),
            b';' if level == 0 => return None,
            _ => {}
        }
    }
    None
}

/// The labels of `text` passed to a `block(` call: a block the importer
/// reads only to walk its elements, whose own count and address fill
/// nothing (its elements are read and labelled on their own).
pub fn walked(text: &str) -> BTreeSet<String> {
    let (bytes, literals) = code(text);
    let mut found = BTreeSet::new();
    for (start, end) in literals {
        let mut level = 0usize;
        let mut open = None;
        for at in (0..start - 1).rev() {
            match bytes[at] {
                b')' | b']' => level += 1,
                b'(' | b'[' if level > 0 => level -= 1,
                b'(' => {
                    open = Some(at);
                    break;
                }
                b'[' | b';' | b'{' | b'}' => break,
                _ => {}
            }
        }
        let Some(open) = open else {
            continue;
        };
        let callee_end = bytes[..open]
            .iter()
            .rposition(|b| !b.is_ascii_whitespace())
            .map_or(0, |at| at + 1);
        let callee_start = bytes[..callee_end]
            .iter()
            .rposition(|&b| !is_ident(b))
            .map_or(0, |at| at + 1);
        let dotted = callee_start > 0 && bytes[callee_start - 1] == b'.';
        if dotted && &bytes[callee_start..callee_end] == b"block" {
            found.insert(text[start..end].to_owned());
        }
    }
    found
}

/// `bytes` with each `#[cfg(test)]` item blanked: a module, a function or
/// a field, to the brace that closes its body or the `,` or `;` that ends
/// it.
fn without_tests(mut bytes: Vec<u8>) -> Vec<u8> {
    const MARK: &[u8] = b"#[cfg(test)]";
    let mut from = 0;
    while let Some(found) = bytes[from..].windows(MARK.len()).position(|w| w == MARK) {
        let start = from + found;
        let mut level = 0usize;
        let mut end = bytes.len();
        for (at, byte) in bytes.iter().enumerate().skip(start + MARK.len()) {
            match *byte {
                b'(' | b'[' | b'{' => level += 1,
                b')' | b']' if level > 0 => level -= 1,
                b'}' if level > 1 => level -= 1,
                b'}' if level == 1 => {
                    end = at + 1;
                    break;
                }
                // (the brace of what holds the item: a field is its last)
                b'}' => {
                    end = at;
                    break;
                }
                b',' | b';' if level == 0 => {
                    end = at + 1;
                    break;
                }
                _ => {}
            }
        }
        for byte in &mut bytes[start..end] {
            *byte = b' ';
        }
        from = end.max(start + 1);
    }
    bytes
}

/// The fields `text` reads through a `.` (not methods, paths, tuple
/// indices, comments, strings or its `#[cfg(test)]` items).
pub fn uses(text: &str) -> BTreeSet<String> {
    let (bytes, _) = code(text);
    let bytes = without_tests(bytes);
    let mut found = BTreeSet::new();
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] != b'.' || at > 0 && bytes[at - 1] == b'.' {
            at += 1;
            continue;
        }
        let start = at + 1;
        let mut end = start;
        while end < bytes.len() && is_ident(bytes[end]) {
            end += 1;
        }
        at = end.max(start);
        if end == start || bytes[start].is_ascii_digit() {
            continue;
        }
        let rest = &bytes[end..];
        let skip = rest
            .iter()
            .position(|b| !b.is_ascii_whitespace())
            .unwrap_or(rest.len());
        let rest = &rest[skip..];
        if rest.starts_with(b"(") || rest.starts_with(b"::") {
            continue;
        }
        found.insert(String::from_utf8_lossy(&bytes[start..end]).into_owned());
    }
    found
}

/// Each of the importer's labels with the identifiers it is assigned to.
pub type Labels = BTreeMap<String, BTreeSet<String>>;

/// The importer's labels with the identifiers they are assigned to, the
/// labels it reads as blocks to walk, and the fields the engine's crates
/// read, in the repository at `root` (none when it has no crates).
pub fn traced(root: &Path) -> Result<(Labels, BTreeSet<String>, BTreeSet<String>), Error> {
    let crates = root.join("engine").join("crates");
    let mut labels = Labels::new();
    let importer = crates.join("import").join("src");
    if !importer.is_dir() {
        return Ok((labels, BTreeSet::new(), BTreeSet::new()));
    }
    let mut blocks = BTreeSet::new();
    for file in walk(&importer, &["rs"])? {
        let text = read_text(&file)?;
        for (label, ident) in assigned(&text) {
            labels.entry(label).or_default().insert(ident);
        }
        blocks.extend(walked(&text));
    }
    let mut used = BTreeSet::new();
    let mut names: Vec<_> = std::fs::read_dir(&crates)
        .map_err(|source| Error::Io {
            path: crates.clone(),
            source,
        })?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| !NOT_ENGINE.contains(&name.as_str()))
        .collect();
    names.sort();
    for name in names {
        let src = crates.join(&name).join("src");
        if !src.is_dir() {
            continue;
        }
        for file in walk(&src, &["rs"])? {
            used.extend(uses(&read_text(&file)?));
        }
    }
    Ok((labels, blocks, used))
}
