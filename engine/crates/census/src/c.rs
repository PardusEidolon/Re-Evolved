//! The original's C, read as far as the census needs it: the functions
//! and data each file defines at its top level, the macros it defines,
//! and the names each of them refers to.
//!
//! The reader tokenises the text, skipping comments, string and character
//! literals and every preprocessor line but `#define`. It never fails: a
//! file it cannot follow (an unbalanced brace in an `#if` branch, say)
//! gives fewer definitions, never a panic.

/// What a definition is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// A function with a body.
    Function,
    /// Data with an initialiser (a function table, say).
    Data,
    /// A `#define`.
    Macro,
}

/// One top-level definition of a file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    /// Its name.
    pub name: String,
    /// What it is.
    pub kind: Kind,
    /// The line it starts on, from 1.
    pub line: usize,
    /// The identifiers its body (or initialiser, or replacement) holds, in
    /// order and repeated.
    pub refers_to: Vec<String>,
    /// Whether the old hosted port added it: a `port:` comment on its
    /// first line or the four before it, or `port` as a word of its name.
    pub port: bool,
    /// Whether it is a function declared `__inline` (or `inline`), which
    /// the compiler gives no symbol of its own.
    pub inline: bool,
}

/// The prefix of hs.c's evaluator macros (`HS_EVALUATE_NO_ARGUMENTS(
/// game_all_quiet_evaluate, game_all_quiet)`), each of which defines the
/// function its first argument names.
const EVALUATOR_MACRO: &str = "HS_EVALUATE_";

/// How many lines above a definition its `port:` comment may be.
const PORT_LINES: usize = 4;

/// A token of the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token<'a> {
    /// An identifier or keyword.
    Name(&'a str),
    /// One character of punctuation.
    Punct(u8),
    /// A number or a literal.
    Other,
}

/// A token and the line it is on.
type Placed<'a> = (Token<'a>, usize);

/// The definitions of one file's text, in the order they appear.
pub fn definitions(text: &str) -> Vec<Definition> {
    let (tokens, mut found) = tokenise(text);
    found.extend(top_level(&tokens));
    found.sort_by_key(|definition| definition.line);
    let port_lines: Vec<usize> = text
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("port:"))
        .map(|(index, _)| index + 1)
        .collect();
    for definition in &mut found {
        let first = definition.line.saturating_sub(PORT_LINES);
        definition.port = definition.name.split('_').any(|word| word == "port")
            || port_lines
                .iter()
                .any(|line| (first..=definition.line).contains(line));
    }
    found
}

/// The file's tokens, and its `#define`s.
fn tokenise(text: &str) -> (Vec<Placed<'_>>, Vec<Definition>) {
    let bytes = text.as_bytes();
    let mut tokens = Vec::new();
    let mut macros = Vec::new();
    let mut at = 0;
    let mut line = 1;
    let mut line_start = true;
    while let Some(&byte) = bytes.get(at) {
        match byte {
            b'\n' => {
                line += 1;
                line_start = true;
                at += 1;
            }
            b' ' | b'\t' | b'\r' | 0x0c | 0x0b => at += 1,
            b'#' if line_start => {
                let end = directive_end(bytes, at);
                let directive = text.get(at + 1..end).unwrap_or("");
                if let Some(definition) = define(directive, line) {
                    macros.push(definition);
                }
                line += directive.matches('\n').count();
                at = end;
            }
            b'/' if bytes.get(at + 1) == Some(&b'/') => {
                while bytes.get(at).is_some_and(|&b| b != b'\n') {
                    at += 1;
                }
            }
            b'/' if bytes.get(at + 1) == Some(&b'*') => {
                at += 2;
                while let Some(&b) = bytes.get(at) {
                    if b == b'*' && bytes.get(at + 1) == Some(&b'/') {
                        at += 2;
                        break;
                    }
                    if b == b'\n' {
                        line += 1;
                    }
                    at += 1;
                }
            }
            b'"' | b'\'' => {
                at = literal_end(bytes, at);
                tokens.push((Token::Other, line));
                line_start = false;
            }
            b if is_name_start(b) => {
                let start = at;
                while bytes.get(at).is_some_and(|&b| is_name_byte(b)) {
                    at += 1;
                }
                tokens.push((Token::Name(text.get(start..at).unwrap_or("")), line));
                line_start = false;
            }
            b if b.is_ascii_digit() => {
                while bytes.get(at).is_some_and(|&b| is_name_byte(b) || b == b'.') {
                    at += 1;
                }
                tokens.push((Token::Other, line));
                line_start = false;
            }
            b => {
                tokens.push((Token::Punct(b), line));
                line_start = false;
                at += 1;
            }
        }
    }
    (tokens, macros)
}

/// Where a preprocessor line that starts at `at` ends: its newline,
/// after any lines it continues with a backslash.
fn directive_end(bytes: &[u8], mut at: usize) -> usize {
    while let Some(&b) = bytes.get(at) {
        if b == b'\n' && bytes.get(at.wrapping_sub(1)) != Some(&b'\\') {
            break;
        }
        at += 1;
    }
    at
}

/// Where a string or character literal that starts at `at` ends.
fn literal_end(bytes: &[u8], at: usize) -> usize {
    let quote = bytes.get(at).copied();
    let mut end = at + 1;
    while let Some(&b) = bytes.get(end) {
        end += 1;
        if b == b'\\' {
            end += 1;
        } else if Some(b) == quote || b == b'\n' {
            break;
        }
    }
    end.min(bytes.len())
}

fn is_name_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// A `#define`'s name and the names its replacement holds (`directive`
/// is the line after its `#`).
fn define(directive: &str, line: usize) -> Option<Definition> {
    let rest = directive.trim_start().strip_prefix("define")?;
    if !rest.starts_with([' ', '\t']) {
        return None;
    }
    let rest = rest.trim_start();
    let end = rest
        .bytes()
        .position(|b| !is_name_byte(b))
        .unwrap_or(rest.len());
    let name = rest.get(..end)?;
    if name.is_empty() || !name.bytes().next().is_some_and(is_name_start) {
        return None;
    }
    let (tokens, _) = tokenise(rest.get(end..)?);
    Some(Definition {
        name: name.to_owned(),
        kind: Kind::Macro,
        line,
        refers_to: names(&tokens),
        port: false,
        inline: false,
    })
}

/// The names among `tokens`.
fn names(tokens: &[Placed<'_>]) -> Vec<String> {
    tokens
        .iter()
        .filter_map(|(token, _)| match token {
            Token::Name(name) => Some((*name).to_owned()),
            _ => None,
        })
        .collect()
}

/// The functions and initialised data at the top level of `tokens`.
fn top_level(tokens: &[Placed<'_>]) -> Vec<Definition> {
    let mut found = Vec::new();
    // (the tokens since the last `;` or `}` at the top level: a
    // declaration's head)
    let mut head = 0;
    let mut at = 0;
    while let Some(&(token, _)) = tokens.get(at) {
        match token {
            Token::Punct(b';') => {
                at += 1;
                head = at;
            }
            Token::Punct(b'{') => {
                let close = matching(tokens, at, b'{', b'}');
                let declaration = tokens.get(head..at).unwrap_or(&[]);
                if let Some((name, line)) = function_name(declaration) {
                    let inline = declaration.iter().any(|(token, _)| {
                        matches!(token, Token::Name("__inline" | "inline" | "__forceinline"))
                    });
                    found.push(Definition {
                        name,
                        kind: Kind::Function,
                        line,
                        refers_to: names(tokens.get(at + 1..close).unwrap_or(&[])),
                        port: false,
                        inline,
                    });
                    at = close + 1;
                    head = at;
                } else if let Some((name, line)) = data_name(declaration) {
                    // (the initialiser runs on to the `;`)
                    let end = statement_end(tokens, at);
                    let equals = declaration
                        .iter()
                        .position(|(token, _)| *token == Token::Punct(b'='))
                        .map_or(at, |position| head + position + 1);
                    found.push(Definition {
                        name,
                        kind: Kind::Data,
                        line,
                        refers_to: names(tokens.get(equals..end).unwrap_or(&[])),
                        port: false,
                        inline: false,
                    });
                    at = end + 1;
                    head = at;
                } else {
                    // (a struct, union or enum: its head runs on to the `;`)
                    at = close + 1;
                }
            }
            Token::Name(name)
                if name.starts_with(EVALUATOR_MACRO)
                    && tokens.get(at + 1).map(|(token, _)| *token) == Some(Token::Punct(b'(')) =>
            {
                let close = matching(tokens, at + 1, b'(', b')');
                let arguments = names(tokens.get(at + 2..close).unwrap_or(&[]));
                if let Some((evaluator, rest)) = arguments.split_first() {
                    let line = tokens.get(at).map_or(0, |(_, line)| *line);
                    let mut refers_to = vec![name.to_owned()];
                    refers_to.extend(rest.iter().cloned());
                    found.push(Definition {
                        name: evaluator.clone(),
                        kind: Kind::Function,
                        line,
                        refers_to,
                        port: false,
                        inline: false,
                    });
                }
                at = close + 1;
                head = at;
            }
            Token::Punct(b'}') => {
                // (a stray brace, from an #if branch the reader cannot follow)
                at += 1;
                head = at;
            }
            _ => at += 1,
        }
    }
    found
}

/// The index of the bracket that closes the one at `open` (or the end).
fn matching(tokens: &[Placed<'_>], open: usize, opening: u8, closing: u8) -> usize {
    let mut depth = 0usize;
    for (index, (token, _)) in tokens.iter().enumerate().skip(open) {
        if *token == Token::Punct(opening) {
            depth += 1;
        } else if *token == Token::Punct(closing) {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return index;
            }
        }
    }
    tokens.len()
}

/// The index of the `;` that ends a statement, skipping brackets, from `at`.
fn statement_end(tokens: &[Placed<'_>], mut at: usize) -> usize {
    while let Some(&(token, _)) = tokens.get(at) {
        match token {
            Token::Punct(b';') => return at,
            Token::Punct(b'{') => at = matching(tokens, at, b'{', b'}') + 1,
            _ => at += 1,
        }
    }
    tokens.len()
}

/// A function's name, when `head` is a function's (`void f(int x)`): the
/// name before the brackets that end it.
fn function_name(head: &[Placed<'_>]) -> Option<(String, usize)> {
    let (last, _) = head.last()?;
    if *last != Token::Punct(b')') {
        return None;
    }
    if head
        .iter()
        .any(|(token, _)| matches!(token, Token::Punct(b'=')))
    {
        return None;
    }
    let mut depth = 0usize;
    let mut open = None;
    for (index, (token, _)) in head.iter().enumerate().rev() {
        match token {
            Token::Punct(b')') => depth += 1,
            Token::Punct(b'(') => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    open = Some(index);
                    break;
                }
            }
            _ => {}
        }
    }
    match head.get(open?.checked_sub(1)?)? {
        (Token::Name(name), line) if !is_keyword(name) => Some(((*name).to_owned(), *line)),
        _ => None,
    }
}

/// Initialised data's name, when `head` is its (`static int x[2] =`):
/// the name before the `=`, past any brackets.
fn data_name(head: &[Placed<'_>]) -> Option<(String, usize)> {
    let equals = head
        .iter()
        .position(|(token, _)| *token == Token::Punct(b'='))?;
    let mut depth = 0usize;
    for (token, line) in head.get(..equals)?.iter().rev() {
        match token {
            Token::Punct(b']') => depth += 1,
            Token::Punct(b'[') => depth = depth.saturating_sub(1),
            Token::Name(name) if depth == 0 => return Some(((*name).to_owned(), *line)),
            _ => {}
        }
    }
    None
}

/// Whether a name is one of C's keywords, which a head can end with
/// (`if (x) {` is no function).
fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "if" | "while" | "for" | "switch" | "return" | "sizeof" | "do" | "else"
    )
}
