//! The original's functions, from the symbol header each file of
//! `source/` opens with:
//!
//! ```text
//! /*
//! UNITS.C
//!
//! symbols in this file:
//! 001977E0 0040:
//!     _units_initialize (0000)
//! 00198690 0020: _unit_kill
//! */
//! ```
//!
//! A block is an address and a size, then the names at that address,
//! each indented by a tab with its offset in brackets (or, in a few
//! files, one name on the address's own line). The headers list the
//! file's data as well as its code; the code is what lies below the
//! first string literal (`??_C@…`) or float constant (`__real@…`) of the
//! whole tree, since the xbe puts its read-only data after its code.

use std::path::Path;

use crate::{read_text, walk, Error};

/// The marker the symbols follow.
const MARKER: &str = "symbols in this file:";

/// One block of a symbol header: an address, a size, and its names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    /// Its address in the xbe.
    pub address: u32,
    /// Its size in bytes.
    pub size: u32,
    /// The names at the address, as the header spells them (`_unit_kill`).
    pub names: Vec<String>,
}

/// One function of the original.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// Its address in the xbe.
    pub address: u32,
    /// Its size in bytes.
    pub size: u32,
    /// Its C name (`unit_kill`): the header's, without the compiler's
    /// leading underscore or a stdcall's `@N`.
    pub name: String,
    /// The file of `source/` it is in (`units/units.c`).
    pub file: String,
}

impl Function {
    /// The directory of `source/` it is in (`units`), which names its
    /// system.
    pub fn directory(&self) -> &str {
        self.file.split('/').next().unwrap_or("")
    }
}

/// A header's name as C spells it: without the compiler's leading
/// underscore, and without a stdcall's `@N`.
pub fn c_name(header_name: &str) -> &str {
    let name = header_name.strip_prefix('_').unwrap_or(header_name);
    match name.rfind('@') {
        Some(at) if at > 0 && name[at + 1..].bytes().all(|b| b.is_ascii_digit()) => &name[..at],
        _ => name,
    }
}

/// Whether a header name is one of the xbe's read-only constants: a
/// string literal or a float.
fn is_constant(header_name: &str) -> bool {
    header_name.starts_with("??_C@") || header_name.starts_with("__real@")
}

/// The symbol header of one file, or nothing when the file has none.
/// `file` names it in errors.
pub fn parse_header(file: &str, text: &str) -> Result<Vec<Symbol>, Error> {
    let Some(body) = text.strip_prefix("/*") else {
        return Ok(Vec::new());
    };
    let header = body.find("*/").map_or(body, |end| &body[..end]);
    let mut lines = header.lines().enumerate();
    if !lines.by_ref().any(|(_, line)| line.trim_end() == MARKER) {
        return Ok(Vec::new());
    }
    let mut symbols: Vec<Symbol> = Vec::new();
    for (index, line) in lines {
        // (the header starts on the file's first line, after the "/*")
        let number = index + 1;
        let line = line.trim_end_matches('\r');
        let malformed = |why: &str| Error::Malformed {
            file: file.to_owned(),
            line: number,
            why: why.to_owned(),
        };
        if line.trim().is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('\t') {
            let symbol = symbols
                .last_mut()
                .ok_or_else(|| malformed("a name before any address"))?;
            symbol
                .names
                .push(name_line(rest).ok_or_else(|| malformed("not a name and an offset"))?);
            continue;
        }
        let (address, size, name) =
            address_line(line).ok_or_else(|| malformed("not an address, a size and a colon"))?;
        symbols.push(Symbol {
            address,
            size,
            names: name.into_iter().collect(),
        });
    }
    if let Some(symbol) = symbols.iter().find(|symbol| symbol.names.is_empty()) {
        return Err(Error::Malformed {
            file: file.to_owned(),
            line: 0,
            why: format!("the block at {:08X} has no name", symbol.address),
        });
    }
    Ok(symbols)
}

/// `001977E0 0040:`, or `00008690 0080: _action_uncover_setup_target`.
fn address_line(line: &str) -> Option<(u32, u32, Option<String>)> {
    let (address, rest) = line.split_once(' ')?;
    let (size, name) = rest.split_once(':')?;
    if address.len() != 8 || size.is_empty() || size.len() > 8 {
        return None;
    }
    let address = u32::from_str_radix(address, 16).ok()?;
    let size = u32::from_str_radix(size, 16).ok()?;
    let name = name.trim();
    if name.contains(char::is_whitespace) {
        return None;
    }
    Some((address, size, (!name.is_empty()).then(|| name.to_owned())))
}

/// `_units_initialize (0000)`: the name, without its offset.
fn name_line(line: &str) -> Option<String> {
    let (name, offset) = line.split_once(' ')?;
    let offset = offset.strip_prefix('(')?.strip_suffix(')')?;
    (!name.is_empty() && !offset.is_empty()).then(|| name.to_owned())
}

/// Every function of the tree at `source`, in address order.
pub fn functions(source: &Path) -> Result<Vec<Function>, Error> {
    let mut blocks = Vec::new();
    for path in walk(source, &["c"])? {
        let file = path
            .strip_prefix(source)
            .map_err(|_| Error::Outside(path.clone()))?
            .to_string_lossy()
            .replace('\\', "/");
        let text = read_text(&path)?;
        for symbol in parse_header(&file, &text)? {
            blocks.push((file.clone(), symbol));
        }
    }
    Ok(code(blocks))
}

/// The code among `blocks`: what lies below the first constant.
pub fn code(blocks: Vec<(String, Symbol)>) -> Vec<Function> {
    let data_start = blocks
        .iter()
        .filter(|(_, symbol)| symbol.names.iter().any(|name| is_constant(name)))
        .map(|(_, symbol)| symbol.address)
        .min()
        .unwrap_or(u32::MAX);
    let mut functions: Vec<Function> = blocks
        .into_iter()
        .filter(|(_, symbol)| symbol.address < data_start)
        .filter_map(|(file, symbol)| {
            let name = symbol.names.first()?;
            Some(Function {
                address: symbol.address,
                size: symbol.size,
                name: c_name(name).to_owned(),
                file,
            })
        })
        .collect();
    functions.sort_by(|a, b| (a.address, &a.file).cmp(&(b.address, &b.file)));
    functions
}
