//! The tag fields the importer keeps. `opence census --record-importer`
//! runs the importer on the user's maps with its readers recording each
//! field read (the tag group, the blocks it was reached through, its
//! offset and width) and writes them to `engine/census/importer_reads.tsv`,
//! which holds only the importer's own offsets and nothing of the maps.
//! The census lays each read over the original's tag structures: the
//! group's structure is the one `source/` casts `tag_get` of it to (or the
//! one named for it), each block's element is the structure `source/`
//! takes from a block of that name with `TAG_BLOCK_GET_ELEMENT`, and the
//! read keeps the fields it overlaps there.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::fields::{access, leaves, tokens, Token};
use crate::layout::Record;
use crate::{read_text, walk, Error};

/// The recorded reads, under `engine/`.
pub const READS: &str = "census/importer_reads.tsv";

/// A block a read was reached through: its offset in the element that
/// holds it and the size of its elements.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hop {
    /// The block's offset in the element holding it.
    pub at: u32,
    /// The size of each of its elements.
    pub element_size: u32,
}

/// A field read the importer made.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Read {
    /// The group the tag was asked for as, as its four characters read
    /// big-endian.
    pub group: u32,
    /// The blocks it was reached through, outermost first.
    pub hops: Vec<Hop>,
    /// Its offset in the element it is in.
    pub at: u32,
    /// How many bytes it read.
    pub width: u32,
    /// The label the importer gave the read: the field it was reading.
    pub label: String,
}

/// The fields the reads keep, how many reads reached no structure's
/// field, and the blocks the importer steps through at a stride that is
/// not the original's element size.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Kept {
    /// Each structure with the path of a field kept.
    pub fields: BTreeSet<(String, String)>,
    /// The reads that could not be laid over a structure.
    pub unresolved: usize,
    /// Each element structure with the importer's stride and its size.
    pub resized: BTreeSet<(String, u32, u32)>,
    /// Each kept field with the labels of the reads that kept it.
    pub labels: BTreeMap<(String, String), BTreeSet<String>>,
}

/// `importer_reads.tsv`'s text: a line for each read.
pub fn write_reads(reads: &BTreeSet<Read>) -> String {
    let mut text = String::from(
        "# group\tblocks (at:element size)\tat\twidth\tlabel (opence census --record-importer)\n",
    );
    for read in reads {
        let route = if read.hops.is_empty() {
            "-".to_owned()
        } else {
            read.hops
                .iter()
                .map(|hop| format!("{}:{}", hop.at, hop.element_size))
                .collect::<Vec<_>>()
                .join("/")
        };
        text.push_str(&format!(
            "{:08x}\t{route}\t{}\t{}\t{}\n",
            read.group, read.at, read.width, read.label
        ));
    }
    text
}

/// `importer_reads.tsv` read back.
pub fn read_reads(text: &str) -> Result<BTreeSet<Read>, Error> {
    let mut reads = BTreeSet::new();
    for (index, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let malformed = |why: &str| Error::Malformed {
            file: format!("engine/{READS}"),
            line: index + 1,
            why: why.to_owned(),
        };
        let columns: Vec<&str> = line.split('\t').collect();
        let [group, route, at, width, label] = columns[..] else {
            return Err(malformed("not five columns"));
        };
        if group.len() != 8 {
            return Err(malformed("a group is eight hex digits"));
        }
        let number = |text: &str| text.parse::<u32>().map_err(|_| malformed("not a number"));
        let mut hops = Vec::new();
        if route != "-" {
            for hop in route.split('/') {
                let (at, size) = hop
                    .split_once(':')
                    .ok_or_else(|| malformed("a block is at:element size"))?;
                hops.push(Hop {
                    at: number(at)?,
                    element_size: number(size)?,
                });
            }
        }
        reads.insert(Read {
            group: u32::from_str_radix(group, 16).map_err(|_| malformed("not hex"))?,
            hops,
            at: number(at)?,
            width: number(width)?,
            label: label.to_owned(),
        });
    }
    Ok(reads)
}

/// The tag groups a header names (`X_TAG= 'abcd'`, or
/// `#define X_TAG 'abcd'`), with their codes.
pub fn groups(text: &str) -> Vec<(String, u32)> {
    let mut found = Vec::new();
    for (at, word) in text.match_indices("_TAG") {
        let start = text[..at]
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |i| i + 1);
        let after = &text[at + word.len()..];
        if after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_') {
            continue;
        }
        let line = text[..start].rfind('\n').map_or(0, |i| i + 1);
        let defined = text[line..start].trim_end() == "#define";
        let rest = after.trim_start();
        let rest = match rest.strip_prefix('=') {
            Some(rest) => rest,
            None if defined => rest,
            None => continue,
        };
        let Some(code) = literal(rest.trim_start()) else {
            continue;
        };
        found.push((text[start..at + word.len()].to_owned(), code));
    }
    found
}

/// The code of the four-character literal `text` starts with.
fn literal(text: &str) -> Option<u32> {
    let code = text.as_bytes();
    let (Some(b'\''), Some(&[a, b, c, d]), Some(b'\'')) =
        (code.first(), code.get(1..5), code.get(5))
    else {
        return None;
    };
    Some(u32::from_be_bytes([a, b, c, d]))
}

/// The name a cast's group literal (`tag_get('coll', i)`) stands under.
fn literal_group(code: u32) -> String {
    format!("GROUP_{code:08x}_TAG")
}

/// `text` with its macros' bodies read as code and each four-character
/// literal given a name, so that the casts and blocks in
/// `#define x_get(i) ((struct x *)tag_get('xxxx', i))` are seen.
fn unhidden(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('\'') {
        out.push_str(&rest[..at]);
        match literal(&rest[at..]) {
            Some(code) => {
                out.push_str(&literal_group(code));
                rest = &rest[at + 6..];
            }
            None => {
                out.push('\'');
                rest = &rest[at + 1..];
            }
        }
    }
    out.push_str(rest);
    let out = out.replace("\n#", "\n ");
    match out.strip_prefix('#') {
        Some(rest) => format!(" {rest}"),
        None => out,
    }
}

/// The structure the token before `at` closes a `struct S *` of (with
/// any qualifiers), walking back.
fn pointer_type_before<'a>(tokens: &[Token<'a>], at: usize) -> Option<&'a str> {
    let mut back = at.checked_sub(1)?;
    let qualifier =
        |token: Option<&Token<'_>>| matches!(token, Some(Token::Name("const" | "volatile")));
    while qualifier(tokens.get(back)) {
        back = back.checked_sub(1)?;
    }
    if tokens.get(back) != Some(&Token::Punct(b'*')) {
        return None;
    }
    back = back.checked_sub(1)?;
    while qualifier(tokens.get(back)) {
        back = back.checked_sub(1)?;
    }
    let Some(Token::Name(structure)) = tokens.get(back) else {
        return None;
    };
    (tokens.get(back.checked_sub(1)?) == Some(&Token::Name("struct"))).then_some(*structure)
}

/// Each `tag_get(X_TAG, ...)` the C casts to or assigns to a
/// `struct S *` (in macros too): the group's name (a literal's is
/// `GROUP_<hex>_TAG`) and `S`, in order.
pub fn casts(text: &str) -> Vec<(String, String)> {
    let text = unhidden(text);
    let tokens = tokens(&text);
    let mut found = Vec::new();
    for (at, token) in tokens.iter().enumerate() {
        if *token != Token::Name("tag_get") || tokens.get(at + 1) != Some(&Token::Punct(b'(')) {
            continue;
        }
        let Some(Token::Name(group)) = tokens.get(at + 2) else {
            continue;
        };
        if !group.ends_with("_TAG") {
            continue;
        }
        let structure = match at.checked_sub(1).and_then(|i| tokens.get(i)) {
            // (struct S *)tag_get(...)
            Some(Token::Punct(b')')) => pointer_type_before(&tokens, at - 1),
            // struct S *name= tag_get(...)
            Some(Token::Punct(b'=')) => match at.checked_sub(2).and_then(|i| tokens.get(i)) {
                Some(Token::Name(_)) => pointer_type_before(&tokens, at - 2),
                _ => None,
            },
            _ => None,
        };
        if let Some(structure) = structure {
            found.push(((*group).to_owned(), structure.to_owned()));
        }
    }
    found
}

/// The token closing the `(` at `open`.
fn closing_paren(tokens: &[Token<'_>], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (at, token) in tokens.iter().enumerate().skip(open) {
        match token {
            Token::Punct(b'(') => depth += 1,
            Token::Punct(b')') => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// Each `TAG_BLOCK_GET_ELEMENT(&p->block, i, type)` the C makes (in
/// macros too): the block's path after its last `->` and the element
/// type, in order.
pub fn elements(text: &str) -> Vec<(String, String)> {
    let text = unhidden(text);
    let tokens = tokens(&text);
    let mut found = Vec::new();
    for (at, token) in tokens.iter().enumerate() {
        if !matches!(
            token,
            Token::Name("TAG_BLOCK_GET_ELEMENT" | "TAG_BLOCK_TRY_AND_GET_ELEMENT")
        ) || tokens.get(at + 1) != Some(&Token::Punct(b'('))
        {
            continue;
        }
        let Some(end) = closing_paren(&tokens, at + 1) else {
            continue;
        };
        // The arguments, split at the commas outside parentheses.
        let mut arguments = vec![at + 2];
        let mut depth = 0usize;
        for (inside, token) in tokens.iter().enumerate().take(end).skip(at + 2) {
            match token {
                Token::Punct(b'(' | b'[') => depth += 1,
                Token::Punct(b')' | b']') => depth = depth.saturating_sub(1),
                Token::Punct(b',') if depth == 0 => arguments.push(inside + 1),
                _ => {}
            }
        }
        let [block, _, kind] = arguments[..] else {
            continue;
        };
        let mut depth = 0usize;
        let mut arrow = None;
        for inside in block..arguments[1] {
            match tokens.get(inside) {
                Some(Token::Punct(b'(' | b'[')) => depth += 1,
                Some(Token::Punct(b')' | b']')) => depth = depth.saturating_sub(1),
                Some(Token::Punct(b'-'))
                    if depth == 0 && tokens.get(inside + 1) == Some(&Token::Punct(b'>')) =>
                {
                    arrow = Some(inside);
                }
                _ => {}
            }
        }
        let Some(path) = arrow
            .and_then(|arrow| arrow.checked_sub(1))
            .and_then(|before| access(&tokens, before))
        else {
            continue;
        };
        let element =
            tokens
                .get(kind..end)
                .unwrap_or_default()
                .iter()
                .find_map(|token| match token {
                    Token::Name("struct" | "union" | "const") => None,
                    Token::Name(name) => Some(*name),
                    Token::Punct(_) => None,
                });
        if let Some(element) = element {
            found.push((path, element.to_owned()));
        }
    }
    found
}

/// Each tag group's structure: the largest laid-out one `source/` casts
/// `tag_get` of it to, or else the laid-out one named for it
/// (`X_TAG` is `x`, `x_definition`, `x` less `_definition`, or
/// `x_group`).
pub fn roots(
    groups: &[(String, u32)],
    casts: &[(String, String)],
    records: &[Record],
) -> BTreeMap<u32, String> {
    let by_name: BTreeMap<&str, &Record> = records.iter().map(|r| (r.name.as_str(), r)).collect();
    let mut found = BTreeMap::new();
    let literals: Vec<(String, u32)> = casts
        .iter()
        .filter_map(|(group, _)| {
            let hex = group.strip_prefix("GROUP_")?.strip_suffix("_TAG")?;
            Some((group.clone(), u32::from_str_radix(hex, 16).ok()?))
        })
        .collect();
    for (name, code) in groups.iter().chain(&literals) {
        let cast = casts
            .iter()
            .filter(|(group, _)| group == name)
            .filter_map(|(_, structure)| by_name.get(structure.as_str()))
            .max_by_key(|record| record.size)
            .map(|record| record.name.clone());
        let base = name.trim_end_matches("_TAG").to_ascii_lowercase();
        let named = [
            Some(base.clone()),
            Some(format!("{base}_definition")),
            base.strip_suffix("_definition").map(str::to_owned),
            Some(format!("{base}_group")),
        ]
        .into_iter()
        .flatten()
        .find(|candidate| by_name.contains_key(candidate.as_str()));
        if let Some(structure) = cast.or(named) {
            found.entry(*code).or_insert(structure);
        }
    }
    for (group, structure) in SHADER_GROUPS {
        if by_name.contains_key(structure) {
            found
                .entry(u32::from_be_bytes(**group))
                .or_insert_with(|| (*structure).to_owned());
        }
    }
    found
}

/// The shader classes' groups and the structure each is: `source/` names
/// no tag constant for them, as the original tells a shader's class by
/// its `shader.base.type`, but the cache names each tag's group, and the
/// importer asks for a shader as its class's. (`scex`, the PC's chicago
/// extended, has no layout in `source/`.)
const SHADER_GROUPS: &[(&[u8; 4], &str)] = &[
    (b"senv", "shader_environment_definition"),
    (b"soso", "shader_model_definition"),
    (b"sotr", "shader_transparent_generic_definition"),
    (b"schi", "shader_transparent_chicago_definition"),
    (b"sgla", "shader_transparent_glass_definition"),
    (b"smet", "shader_transparent_meter_definition"),
    (b"spla", "shader_transparent_plasma_definition"),
    (b"swat", "shader_transparent_water_definition"),
];

/// What a block's elements are: a laid-out structure, something the C
/// names that is not one (`real_plane3d`), or not known.
enum Element<'a> {
    Structure(&'a Record),
    /// The one structure the C names for the block, at another size.
    Resized(&'a Record),
    Other,
    Unknown,
}

/// The elements of the block at `path` of `element_size` bytes: the
/// structure of that size the C takes from a block of that name, or the
/// one such structure at another size when `owner`, the structure
/// holding the block, shares its first word and has no structure of its
/// own of the stepped size (another group's block of the same name is
/// not it), or else the one laid-out structure of that size.
fn element<'a>(
    owner: &str,
    path: &str,
    element_size: u32,
    elements: &[(String, String)],
    by_name: &BTreeMap<&str, &'a Record>,
) -> Element<'a> {
    let named: BTreeSet<&str> = elements
        .iter()
        .filter(|(block, _)| path == block || path.ends_with(&format!(".{block}")))
        .map(|(_, element)| element.as_str())
        .collect();
    let sized: Vec<&Record> = named
        .iter()
        .filter_map(|name| by_name.get(name))
        .filter(|record| record.size == element_size)
        .copied()
        .collect();
    if let [record] = sized[..] {
        return Element::Structure(record);
    }
    let laid: Vec<&Record> = named
        .iter()
        .filter_map(|name| by_name.get(name))
        .copied()
        .collect();
    let first = |name: &str| name.split('_').next().unwrap_or(name).to_owned();
    if let ([], [record]) = (&sized[..], &laid[..]) {
        // The owner's own structure of the stepped size is the block's,
        // though the C names another of that block name elsewhere.
        let owned: Vec<&Record> = by_name
            .values()
            .filter(|r| r.size == element_size && first(&r.name) == first(owner))
            .copied()
            .collect();
        return match owned[..] {
            [owned] => Element::Structure(owned),
            [] if first(&record.name) == first(owner) => Element::Resized(record),
            _ => Element::Unknown,
        };
    }
    if sized.is_empty() && !named.is_empty() && laid.is_empty() {
        return Element::Other;
    }
    let any: Vec<&Record> = by_name
        .values()
        .filter(|record| record.size == element_size)
        .copied()
        .collect();
    if !sized.is_empty() {
        return Element::Unknown;
    }
    if let [record] = any[..] {
        return Element::Structure(record);
    }
    // A block the C never names (`scenario.weapons`, reached through
    // shared placement code) is the one structure of its size its name
    // gives: `scenario_weapon_datum`, `scenario_object_palette_entry`.
    let block = path.rsplit('.').next().unwrap_or(path);
    let stem = match block.rfind("_palette") {
        Some(_) => "palette",
        None => block.strip_suffix('s').unwrap_or(block),
    };
    match any
        .iter()
        .filter(|record| record.name.contains(stem))
        .collect::<Vec<_>>()[..]
    {
        [record] if named.is_empty() => Element::Structure(record),
        _ => Element::Unknown,
    }
}

/// The fields of `records` the `reads` keep: each read is laid over the
/// element its blocks reach from its group's structure, and keeps every
/// leaf it overlaps (a leaf runs to the next leaf's offset).
pub fn resolve(
    reads: &BTreeSet<Read>,
    roots: &BTreeMap<u32, String>,
    elements: &[(String, String)],
    records: &[Record],
) -> Kept {
    let by_name: BTreeMap<&str, &Record> = records.iter().map(|r| (r.name.as_str(), r)).collect();
    let mut kept = Kept::default();
    'reads: for read in reads {
        let Some(&root) = roots.get(&read.group).and_then(|s| by_name.get(s.as_str())) else {
            kept.unresolved += 1;
            continue;
        };
        let mut record: &Record = root;
        for hop in &read.hops {
            let Some(block) = record
                .fields
                .iter()
                .find(|f| f.offset == hop.at && f.kind == "struct tag_block")
            else {
                kept.unresolved += 1;
                continue 'reads;
            };
            match element(
                &record.name,
                &block.path,
                hop.element_size,
                elements,
                &by_name,
            ) {
                Element::Structure(inner) => record = inner,
                Element::Resized(inner) => {
                    kept.resized
                        .insert((inner.name.clone(), hop.element_size, inner.size));
                    record = inner;
                }
                Element::Other => continue 'reads,
                Element::Unknown => {
                    kept.unresolved += 1;
                    continue 'reads;
                }
            }
        }
        if read.at >= record.size {
            kept.unresolved += 1;
            continue;
        }
        let leaves = leaves(record);
        let end = read.at.saturating_add(read.width.max(1));
        for leaf in &leaves {
            let next = leaves
                .iter()
                .map(|f| f.offset)
                .filter(|&offset| offset > leaf.offset)
                .min()
                .unwrap_or(record.size);
            if leaf.offset < end && next > read.at {
                let field = (record.name.clone(), leaf.path.clone());
                kept.labels
                    .entry(field.clone())
                    .or_default()
                    .insert(read.label.clone());
                kept.fields.insert(field);
            }
        }
    }
    kept
}

/// The fields the written reads keep, against `source/` in the
/// repository at `root` (none when no reads are written).
pub fn kept(root: &Path, records: &[Record]) -> Result<Kept, Error> {
    let text = read_text(&root.join("engine").join(READS)).unwrap_or_default();
    let reads = read_reads(&text)?;
    if reads.is_empty() {
        return Ok(Kept::default());
    }
    let mut all_groups = Vec::new();
    let mut all_casts = Vec::new();
    let mut all_elements = Vec::new();
    for file in walk(&root.join("source"), &["c", "h"])? {
        let text = read_text(&file)?;
        all_groups.extend(groups(&text));
        all_casts.extend(casts(&text));
        all_elements.extend(elements(&text));
    }
    let roots = roots(&all_groups, &all_casts, records);
    Ok(resolve(&reads, &roots, &all_elements, records))
}
