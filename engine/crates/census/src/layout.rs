//! The tag layout: every tag structure of `source/`'s tag headers
//! with each field's offset, as the original's compiler laid it out.
//!
//! clang lays the headers out for 32-bit MSVC (`i686-pc-windows-msvc`),
//! the original's own target, and dumps each record's layout; the headers'
//! own `verify_*` asserts hold on that target, so the layout is the
//! original's. `opence census --write` keeps the result in
//! `engine/census/tag_layout.tsv`, so that the census's own check needs no
//! clang; `--check` lays the headers out again and compares. The crate's
//! test checks the written layout against every size and offset assert in
//! the headers. Only `source/`'s folders are on the include path, with a
//! few stand-in system headers.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{read_text, walk, Error};

/// The written layout, from `engine/`.
pub const LAYOUT: &str = "census/tag_layout.tsv";

/// The headers every definitions header leans on without including.
const PREAMBLE: &[&str] = &[
    "cseries.h",
    "real_math.h",
    "integer_math.h",
    "tag_groups.h",
    "game/game_globals.h",
];

/// Stand-ins for the system headers `cseries.h` includes. `StdDef.h`
/// defines what the headers use itself, since on a case-blind disk it is
/// also `stddef.h`.
const STAND_INS: &[(&str, &str)] = &[
    (
        "StdDef.h",
        "#pragma once\n#define offsetof(t, m) __builtin_offsetof(t, m)\n#define NULL ((void *)0)\ntypedef unsigned int size_t;\ntypedef unsigned short wchar_t;\n",
    ),
    ("stdio.h", "#pragma once\n"),
    ("stdlib.h", "#pragma once\n"),
    ("string.h", "#pragma once\n"),
    ("stdarg.h", "#pragma once\n"),
    (
        "math.h",
        "#pragma once\ndouble sqrt(double); double sin(double); double cos(double); double tan(double);\n\
         double asin(double); double acos(double); double atan(double); double atan2(double, double);\n\
         double fabs(double); double floor(double); double ceil(double); double fmod(double, double);\n\
         double pow(double, double); double exp(double); double log(double);\n",
    ),
];

/// A record's field: its path from the record (`parts.count`; an unnamed
/// member adds nothing to it), its byte offset and its type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// The path, dotted through nested structures.
    pub path: String,
    /// The byte offset from the record's start.
    pub offset: u32,
    /// The type as clang prints it.
    pub kind: String,
}

/// A tag structure and its fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// The structure's name, without `struct`.
    pub name: String,
    /// Its size in bytes.
    pub size: u32,
    /// Its fields, in clang's order.
    pub fields: Vec<Field>,
}

/// A header's own check of a structure's size (`field` is `None`) or of a
/// field's offset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assert {
    /// The structure.
    pub structure: String,
    /// The field, for an offset.
    pub field: Option<String>,
    /// The size or offset the header asserts.
    pub value: u32,
}

/// The number a dump's offset column holds (`12`, or `12:0-3` for a bit
/// field).
fn offset(column: &str) -> Option<u32> {
    column.trim().split(':').next()?.parse().ok()
}

/// A dumped type, with the absolute path clang gives an unnamed record
/// cut back to `source/`.
fn kind(text: &str) -> String {
    match (text.find(" at "), text.find("/source/")) {
        (Some(at), Some(source)) if source > at => {
            format!("{} at {}", &text[..at], &text[source + 1..])
        }
        _ => text.to_owned(),
    }
}

/// clang's `-fdump-record-layouts` output, read into records. A record
/// that is not complete (no offset, no size line) is left out.
pub fn read_dump(text: &str) -> Vec<Record> {
    let mut records = Vec::new();
    for block in text.split("*** Dumping AST Record Layout").skip(1) {
        if let Some(record) = read_block(block) {
            records.push(record);
        }
    }
    records
}

/// One record of the dump.
fn read_block(block: &str) -> Option<Record> {
    let mut lines = block.lines().filter(|line| line.contains('|'));
    let (_, head) = lines.next()?.split_once('|')?;
    let head = head.trim();
    let name = head
        .strip_prefix("struct ")
        .or_else(|| head.strip_prefix("union "))?
        .to_owned();
    let mut fields = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    for line in lines {
        let (column, rest) = line.split_once('|')?;
        if let Some(size) = rest.trim().strip_prefix("[sizeof=") {
            let size = size
                .split(|c: char| !c.is_ascii_digit())
                .next()?
                .parse()
                .ok()?;
            return Some(Record { name, size, fields });
        }
        let depth = (rest.len() - rest.trim_start().len()).saturating_sub(1) / 2;
        let text = rest.trim();
        let (kind_text, field) = match text.rsplit_once(' ') {
            Some((kind, field)) if !field.ends_with(')') => (kind, field),
            _ => (text, ""),
        };
        if depth == 0 {
            return None;
        }
        stack.truncate(depth - 1);
        stack.push(field.to_owned());
        fields.push(Field {
            path: stack
                .iter()
                .filter(|s| !s.is_empty())
                .cloned()
                .collect::<Vec<_>>()
                .join("."),
            offset: offset(column)?,
            kind: kind(kind_text),
        });
    }
    None
}

/// Whether a header holds tag structures: a `*_definitions.h`, or one
/// that names a tag group (`X_TAG= 'abcd'`), as `bitmap_group.h` does.
pub fn is_tag_header(path: &Path, text: &str) -> bool {
    if path.to_string_lossy().ends_with("_definitions.h") {
        return true;
    }
    text.match_indices("_TAG").any(|(at, word)| {
        let rest = text[at + word.len()..].trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            return false;
        };
        let rest = rest.trim_start().as_bytes();
        rest.len() >= 6 && rest[0] == b'\'' && rest[5] == b'\''
    })
}

/// The structures a header defines (`struct name` with its body on the
/// next line), in order.
pub fn tag_structures(text: &str) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    lines
        .windows(2)
        .filter(|pair| pair[1].trim() == "{")
        .filter_map(|pair| {
            let name = pair[0].trim().strip_prefix("struct ")?;
            name.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
                .then(|| name.to_owned())
        })
        .collect()
}

/// A C number (`0x48` or `72`).
fn number(text: &str) -> Option<u32> {
    let text = text.trim();
    match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        Some(hex) => u32::from_str_radix(hex, 16).ok(),
        None => text.parse().ok(),
    }
}

/// A header's size and offset asserts: `sizeof(struct s) == n` and
/// `offsetof(struct s, f) == n`.
pub fn asserts(text: &str) -> Vec<Assert> {
    let mut found = Vec::new();
    for (word, sized) in [("sizeof(struct ", true), ("offsetof(struct ", false)] {
        let mut rest = text;
        while let Some(at) = rest.find(word) {
            rest = &rest[at + word.len()..];
            let Some(close) = rest.find(')') else { break };
            let inside = &rest[..close];
            let after = &rest[close + 1..];
            let Some(value) = after
                .trim_start()
                .strip_prefix("==")
                .and_then(|v| number(v.split('?').next().unwrap_or_default()))
            else {
                continue;
            };
            let (structure, field) = match inside.split_once(',') {
                Some((s, f)) if !sized => (s.trim(), Some(f.trim().to_owned())),
                None if sized => (inside.trim(), None),
                _ => continue,
            };
            found.push(Assert {
                structure: structure.to_owned(),
                field,
                value,
            });
        }
    }
    found.sort_by_key(|a| a.field.is_some());
    found
}

/// The asserts `records` break, a line each.
pub fn disagreements(records: &[Record], asserts: &[Assert]) -> Vec<String> {
    let mut found = Vec::new();
    for assert in asserts {
        let Some(record) = records.iter().find(|r| r.name == assert.structure) else {
            found.push(format!("{} is not laid out", assert.structure));
            continue;
        };
        let (what, laid) = match &assert.field {
            None => (assert.structure.clone(), Some(record.size)),
            Some(field) => (
                format!("{}.{field}", assert.structure),
                record
                    .fields
                    .iter()
                    .find(|f| &f.path == field)
                    .map(|f| f.offset),
            ),
        };
        if laid != Some(assert.value) {
            found.push(format!(
                "{what}: the header asserts {:#x}, the layout gives {}",
                assert.value,
                laid.map_or_else(|| "nothing".to_owned(), |v| format!("{v:#x}"))
            ));
        }
    }
    found.sort();
    found
}

/// `tag_layout.tsv`'s text: a line for each record (`name`, an empty
/// field, its size, `sizeof`) followed by a line for each field.
pub fn write_layout(records: &[Record]) -> String {
    let mut text = String::from("# structure\tfield\toffset\ttype (opence census --write)\n");
    for record in records {
        text.push_str(&format!("{}\t\t{}\tsizeof\n", record.name, record.size));
        for field in &record.fields {
            text.push_str(&format!(
                "{}\t{}\t{}\t{}\n",
                record.name, field.path, field.offset, field.kind
            ));
        }
    }
    text
}

/// `tag_layout.tsv` read back.
pub fn read_layout(text: &str) -> Result<Vec<Record>, Error> {
    let mut records: Vec<Record> = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        let malformed = |why: &str| Error::Malformed {
            file: format!("engine/{LAYOUT}"),
            line: index + 1,
            why: why.to_owned(),
        };
        let columns: Vec<&str> = line.split('\t').collect();
        let [name, path, number, kind] = columns[..] else {
            return Err(malformed("not four columns"));
        };
        let value: u32 = number.parse().map_err(|_| malformed("not a number"))?;
        if path.is_empty() && kind == "sizeof" {
            records.push(Record {
                name: name.to_owned(),
                size: value,
                fields: Vec::new(),
            });
            continue;
        }
        match records.last_mut() {
            Some(record) if record.name == name => record.fields.push(Field {
                path: path.to_owned(),
                offset: value,
                kind: kind.to_owned(),
            }),
            _ => return Err(malformed("a field before its structure")),
        }
    }
    Ok(records)
}

/// A folder of its own under the system's temporary folder, removed when
/// dropped.
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Writes `text` at `path`.
fn put(path: &Path, text: &str) -> Result<(), Error> {
    std::fs::write(path, text).map_err(|source| Error::Io {
        path: path.to_owned(),
        source,
    })
}

/// Runs `command`, its standard output, or the problem.
fn run(command: &mut Command, what: &str) -> Result<String, Error> {
    let output = command.output().map_err(|source| Error::Io {
        path: PathBuf::from("clang"),
        source,
    })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let first: Vec<&str> = stderr
            .lines()
            .filter(|l| l.contains("error"))
            .take(5)
            .collect();
        return Err(Error::Problems(vec![format!(
            "{what} failed: {}",
            first.join("; ")
        )]));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// The old port's capacity macros the tag headers' includes use, at the
/// original's own values (the stock values `halo_port_capacity.h` notes).
const STOCK_CAPACITIES: [(&str, &str); 22] = [
    ("HALO_PORT_MAXIMUM_ACTORS", "256"),
    ("HALO_PORT_MAXIMUM_PROPS", "768"),
    ("HALO_PORT_MAXIMUM_SWARMS", "32"),
    ("HALO_PORT_MAXIMUM_SWARM_COMPONENTS", "256"),
    ("HALO_PORT_MAXIMUM_OBJECTS_PER_MAP", "2048"),
    ("HALO_PORT_OBJECT_MEMORY_POOL_SIZE", "0x100000"),
    ("HALO_PORT_MAXIMUM_CLUSTER_REFERENCES", "2048"),
    ("HALO_PORT_MAXIMUM_RENDERED_OBJECTS", "256"),
    ("HALO_PORT_MAXIMUM_CACHED_OBJECT_RENDER_STATES", "256"),
    ("HALO_PORT_MAXIMUM_AREA_OF_EFFECT_OBJECTS", "64"),
    ("HALO_PORT_MAXIMUM_OBJECT_LISTS_PER_MAP", "48"),
    ("HALO_PORT_MAXIMUM_LISTED_OBJECTS_PER_MAP", "128"),
    ("HALO_PORT_MAXIMUM_WIDGETS", "64"),
    ("HALO_PORT_MAXIMUM_LIGHT_VOLUMES", "256"),
    ("HALO_PORT_MAXIMUM_EFFECTS", "256"),
    ("HALO_PORT_MAXIMUM_EFFECT_LOCATIONS", "512"),
    ("HALO_PORT_MAXIMUM_PARTICLES", "1024"),
    ("HALO_PORT_MAXIMUM_PARTICLE_SYSTEMS", "64"),
    ("HALO_PORT_MAXIMUM_SYSTEM_PARTICLES", "512"),
    ("HALO_PORT_MAXIMUM_CONTRAILS", "256"),
    ("HALO_PORT_MAXIMUM_CONTRAIL_POINTS", "1024"),
    ("HALO_PORT_MAXIMUM_LIGHTS_PER_MAP", "896"),
];

/// The include path and defines clang lays `source/` out with: its own
/// resource headers, the stand-ins (written under `scratch`), and
/// `source/` with every folder under it but zlib.
fn includes(source: &Path, scratch: &Path) -> Result<Vec<String>, Error> {
    let stand_ins = scratch.join("include");
    std::fs::create_dir_all(&stand_ins).map_err(|e| Error::Io {
        path: stand_ins.clone(),
        source: e,
    })?;
    for (name, text) in STAND_INS {
        put(&stand_ins.join(name), text)?;
    }
    let resources = run(
        Command::new("clang").arg("-print-resource-dir"),
        "clang -print-resource-dir",
    )?;
    let mut includes = vec![
        "-internal-isystem".to_owned(),
        Path::new(resources.trim())
            .join("include")
            .display()
            .to_string(),
        format!("-I{}", stand_ins.display()),
        format!("-I{}", source.display()),
    ];
    let mut folders = vec![source.to_owned()];
    while let Some(folder) = folders.pop() {
        let entries = std::fs::read_dir(&folder).map_err(|e| Error::Io {
            path: folder.clone(),
            source: e,
        })?;
        let mut found: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.is_dir() && !p.ends_with("zlib"))
            .collect();
        found.sort();
        for path in found {
            includes.push(format!("-I{}", path.display()));
            folders.push(path);
        }
    }
    includes.extend(
        STOCK_CAPACITIES
            .iter()
            .map(|(name, value)| format!("-D{name}={value}")),
    );
    Ok(includes)
}

/// clang dumping the layouts of `file`'s records for the original's
/// target.
fn clang(file: &Path, includes: &[String]) -> Command {
    let mut clang = Command::new("clang");
    clang
        .args(["-cc1", "-triple", "i686-pc-windows-msvc", "-fsyntax-only"])
        .args(["-fms-extensions", "-fms-compatibility", "-w"])
        .args(includes)
        .arg("-fdump-record-layouts-complete")
        .arg(file);
    clang
}

/// The `.c` files whose own definitions of tag structures are laid out
/// with them, and where each misnames the structure. The fog's
/// `shader_transparent_chicago_definition` is the model shader's layout:
/// the file's own shader type enum numbers chicago 4, the model shader's
/// type, and its `map_u_scale` at 0x9C is the model shader's.
pub const MISNAMED: &[(&str, &str)] = &[(
    "rasterizer/xbox/rasterizer_xbox_environment_fog.c",
    "shader_transparent_chicago_definition",
)];

/// The `.c` files that define a tag structure from past its start, the
/// structure, and how far past: the rasterizer's plasma energy takes the
/// plasma shader at its `struct shader` header's end (it casts the shader
/// plus `sizeof(struct shader)`), and the rasterizer's decals lay the
/// decal's shader out from the same place (their `decal_definition` puts
/// it at 0xBC, where the effects' own puts its `struct shader` at 0x94).
pub const SHIFTED: &[(&str, &str, u32)] = &[
    (
        "rasterizer/xbox/rasterizer_xbox_plasma_energy.c",
        "shader_transparent_plasma_definition",
        0x28,
    ),
    (
        "rasterizer/xbox/rasterizer_xbox_decals.c",
        "decal_shader_definition",
        0x28,
    ),
];

/// The tag structures `source/`'s definitions end short of, and each
/// one's size in the tag: the sizes the tag format documentation gives,
/// which the stock maps bear out (a tag with no blocks or data takes
/// that much before the next: census/tests/layout_maps.rs). The bytes
/// past what `source/` defines are laid out as `unknown<offset>`.
pub const TAG_SIZES: &[(&str, u32)] = &[
    ("shader_model_definition", 440),
    ("shader_transparent_chicago_definition", 108),
    ("shader_transparent_plasma_definition", 332),
    ("shader_transparent_water_definition", 320),
];

/// `record` grown to `size` bytes, the bytes added an `unknown` field.
pub fn grown(mut record: Record, size: u32) -> Record {
    if size > record.size {
        record.fields.push(Field {
            path: format!("unknown{:x}", record.size),
            offset: record.size,
            kind: format!("byte[{}]", size - record.size),
        });
        record.size = size;
    }
    record
}

/// Where a C text defines `name`: the lines (from 1) of `struct name`
/// and of the `};` closing it.
pub fn span(text: &str, name: &str) -> Option<(usize, usize)> {
    let lines: Vec<&str> = text.lines().collect();
    let head = format!("struct {name}");
    let start = lines
        .windows(2)
        .position(|pair| pair[0].trim() == head && pair[1].trim() == "{")?;
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 2)
        .find(|(_, line)| line.starts_with("};"))?
        .0;
    Some((start + 1, end + 1))
}

/// The structure the first member of `name`'s definition in `text` is
/// (`struct shader shader;` is `shader`), if it is one.
pub fn first_member(text: &str, name: &str) -> Option<String> {
    let (start, _) = span(text, name)?;
    let line = text
        .lines()
        .skip(start + 1)
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with("/*") && !line.starts_with("//"))?;
    let (structure, member) = line.strip_prefix("struct ")?.split_once(' ')?;
    let member = member.trim().strip_suffix(';')?;
    let word = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    (word(structure) && word(member)).then(|| structure.to_owned())
}

/// The structures a header only declares (`struct name;`).
pub fn declared(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|line| {
            let name = line.trim().strip_prefix("struct ")?.strip_suffix(';')?;
            name.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
                .then(|| name.to_owned())
        })
        .collect()
}

/// The tag structures `source/`'s `.c` files define themselves, by name,
/// with the files (as `files` names them) that define each, from
/// `files`' texts: a structure no tag header defines that the C takes
/// `tag_get` or a block's element as, or that a tag header declares, or
/// whose first member is a tag structure (the `shader` of a shader of a
/// class, the `unit` of a vehicle).
pub fn local_structures(files: &[(String, String)]) -> BTreeMap<String, Vec<String>> {
    let mut defined = BTreeSet::new();
    let mut wanted = BTreeSet::new();
    let mut local: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (path, text) in files {
        if path.ends_with(".h") {
            if is_tag_header(Path::new(path), text) {
                defined.extend(tag_structures(text));
                wanted.extend(declared(text));
            }
        } else {
            for name in tag_structures(text) {
                local.entry(name).or_default().push(path.clone());
            }
        }
        wanted.extend(crate::importer::casts(text).into_iter().map(|(_, s)| s));
        wanted.extend(crate::importer::elements(text).into_iter().map(|(_, s)| s));
    }
    local.retain(|name, _| !defined.contains(name));
    let texts: BTreeMap<&str, &str> = files
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let mut chosen: BTreeSet<String> = local
        .keys()
        .filter(|name| wanted.contains(*name))
        .cloned()
        .collect();
    loop {
        let more: Vec<String> = local
            .iter()
            .filter(|(name, _)| !chosen.contains(*name))
            .filter(|(name, paths)| {
                paths.iter().any(|path| {
                    first_member(texts.get(path.as_str()).copied().unwrap_or_default(), name)
                        .is_some_and(|first| defined.contains(&first) || chosen.contains(&first))
                })
            })
            .map(|(name, _)| name.clone())
            .collect();
        if more.is_empty() {
            break;
        }
        chosen.extend(more);
    }
    local.retain(|name, _| chosen.contains(name));
    local
}

/// Whether a field's name says nothing of it: `unknown`, `unused`, `pad`
/// or `reserved`, then nothing, `_` and more, or an offset in hex
/// (`pad2A`, `reserved_before_map`, `__unknown308`).
pub fn says_nothing(path: &str) -> bool {
    let last = path
        .rsplit('.')
        .next()
        .unwrap_or(path)
        .trim_start_matches('_');
    ["unknown", "unused", "pad", "reserved"].iter().any(|word| {
        last.strip_prefix(word).is_some_and(|rest| {
            rest.is_empty() || rest.starts_with('_') || rest.chars().all(|c| c.is_ascii_hexdigit())
        })
    })
}

/// The fragment's leaves, each with the byte past its last: the next
/// leaf's offset, or the fragment's size.
fn extents(record: &Record) -> Vec<(&Field, u32)> {
    let leaves = crate::fields::leaves(record);
    leaves
        .iter()
        .map(|leaf| {
            let end = leaves
                .iter()
                .map(|f| f.offset)
                .filter(|&at| at > leaf.offset)
                .min()
                .unwrap_or(record.size)
                .max(leaf.offset);
            (*leaf, end)
        })
        .collect()
}

/// One structure from the definitions `.c` files give of it, each laid
/// out (`fragments`): the largest one's size; each named field at the
/// offset its definitions agree on, taken from the definition that names
/// the most bytes first, and from the others where it names bytes no
/// definition before it did; and the bytes no definition names under the
/// name a definition gives them, or as `unknown<offset>` bytes where they
/// are only part of such a field. Definitions that put one name at two
/// offsets are the problem returned.
pub fn merge(name: &str, fragments: &[Record]) -> Result<Record, Error> {
    let size = fragments.iter().map(|f| f.size).max().unwrap_or(0);
    let named_bytes = |record: &Record| -> u32 {
        extents(record)
            .iter()
            .filter(|(leaf, _)| !says_nothing(&leaf.path))
            .map(|(leaf, end)| end - leaf.offset)
            .sum()
    };
    let mut order: Vec<&Record> = fragments.iter().collect();
    order.sort_by_key(|record| std::cmp::Reverse(named_bytes(record)));
    let mut owned = vec![false; size as usize];
    let mut fields: Vec<Field> = Vec::new();
    let mut problems = Vec::new();
    let taken = |fields: &mut Vec<Field>, field: &Field, problems: &mut Vec<String>| match fields
        .iter()
        .find(|f| f.path == field.path)
    {
        Some(f) if f.offset != field.offset => problems.push(format!(
            "{name}.{}: one definition puts it at {:#x}, another at {:#x}",
            field.path, f.offset, field.offset
        )),
        Some(_) => {}
        None => fields.push(field.clone()),
    };
    for record in &order {
        let leaves = extents(record);
        let mut at = 0;
        // named leaves, those sharing an offset (bit fields) together
        while at < leaves.len() {
            let offset = leaves[at].0.offset;
            let group: Vec<&(&Field, u32)> = leaves[at..]
                .iter()
                .take_while(|(leaf, _)| leaf.offset == offset)
                .collect();
            at += group.len();
            let mut fresh = Vec::new();
            for (leaf, end) in group.iter().filter(|(leaf, _)| !says_nothing(&leaf.path)) {
                match fields.iter().find(|f| f.path == leaf.path) {
                    Some(f) if f.offset != leaf.offset => problems.push(format!(
                        "{name}.{}: one definition puts it at {:#x}, another at {:#x}",
                        leaf.path, f.offset, leaf.offset
                    )),
                    Some(_) => {}
                    None => fresh.push((*leaf, *end)),
                }
            }
            let Some(end) = fresh.iter().map(|(_, end)| *end).max() else {
                continue;
            };
            if (offset..end).all(|byte| !owned[byte as usize]) {
                for (leaf, _) in fresh {
                    fields.push(leaf.clone());
                }
                for byte in offset..end {
                    owned[byte as usize] = true;
                }
            }
        }
    }
    for record in &order {
        for (leaf, end) in extents(record) {
            if !says_nothing(&leaf.path) {
                continue;
            }
            let free: Vec<u32> = (leaf.offset..end)
                .filter(|&byte| !owned[byte as usize])
                .collect();
            if free.len() as u32 == end - leaf.offset && end > leaf.offset {
                taken(&mut fields, leaf, &mut problems);
            } else {
                let parent = leaf.path.rsplit_once('.').map(|(p, _)| format!("{p}."));
                let mut runs: Vec<(u32, u32)> = Vec::new();
                for byte in free {
                    match runs.last_mut() {
                        Some((_, last)) if *last == byte => *last = byte + 1,
                        _ => runs.push((byte, byte + 1)),
                    }
                }
                for (start, stop) in runs {
                    taken(
                        &mut fields,
                        &Field {
                            path: format!("{}unknown{start:x}", parent.clone().unwrap_or_default()),
                            offset: start,
                            kind: format!("byte[{}]", stop - start),
                        },
                        &mut problems,
                    );
                }
            }
            for byte in leaf.offset..end {
                owned[byte as usize] = true;
            }
        }
    }
    for record in &order {
        for field in &record.fields {
            let inside = format!("{}.", field.path);
            if !field.path.is_empty()
                && fields.iter().any(|f| f.path.starts_with(&inside))
                && !fields.iter().any(|f| f.path == field.path)
            {
                taken(&mut fields, field, &mut problems);
            }
        }
    }
    if !problems.is_empty() {
        return Err(Error::Problems(problems));
    }
    fields.sort_by(|a, b| {
        (a.offset, a.path.matches('.').count(), &a.path).cmp(&(
            b.offset,
            b.path.matches('.').count(),
            &b.path,
        ))
    });
    Ok(Record {
        name: name.to_owned(),
        size,
        fields,
    })
}

/// The lines of `file` clang's `stderr` puts an error on.
fn error_lines(stderr: &str, file: &Path) -> Vec<usize> {
    let prefix = format!("{}:", file.display());
    stderr
        .lines()
        .filter(|line| line.contains(": error:") || line.contains(": fatal error:"))
        .filter_map(|line| line.strip_prefix(&prefix)?.split(':').next()?.parse().ok())
        .collect()
}

/// The layout of the tag structures of the repository at `root`, from
/// clang, sorted by name.
///
/// The `*_definitions.h` are laid out together; each other tag header is
/// laid out on its own, since the original never includes some of them
/// with the definitions (`point_physics.h` and `physics_definitions.h`
/// both name `POINT_PHYSICS_DEFINITION_TAG`). The tag structures a `.c`
/// file defines itself (`local_structures`) are laid out from each file
/// that defines them, as it is, and merged; clang may fail on the rest of
/// such a file, but an error inside a definition, or a definition that
/// breaks its file's own asserts, is a problem.
pub fn generate(root: &Path) -> Result<Vec<Record>, Error> {
    let source = root.join("source");
    let preamble: String = PREAMBLE
        .iter()
        .map(|header| format!("#include \"{header}\"\n"))
        .collect();
    let mut names = Vec::new();
    let mut units = vec![preamble.clone()];
    let mut files = Vec::new();
    for file in walk(&source, &["c", "h"])? {
        let text = read_text(&file)?;
        let relative = file
            .strip_prefix(&source)
            .unwrap_or(&file)
            .to_string_lossy()
            .into_owned();
        if relative.ends_with(".h") && is_tag_header(&file, &text) {
            names.extend(tag_structures(&text));
            let include = format!("#include \"{}\"\n", file.display());
            if relative.ends_with("_definitions.h") {
                units[0].push_str(&include);
            } else {
                units.push(format!("{preamble}{include}"));
            }
        }
        files.push((relative, text));
    }
    let scratch =
        Scratch(std::env::temp_dir().join(format!("opence-census-layout-{}", std::process::id())));
    let includes = includes(&source, &scratch.0)?;
    let mut records = Vec::new();
    for (index, unit) in units.iter().enumerate() {
        let file = scratch.0.join(format!("layout{index}.c"));
        put(&file, unit)?;
        let dump = run(
            &mut clang(&file, &includes),
            "clang's layout of the tag structures",
        )?;
        records.extend(
            read_dump(&dump)
                .into_iter()
                .filter(|r| names.contains(&r.name)),
        );
    }
    records.sort_by(|a, b| a.name.cmp(&b.name));
    records.dedup_by(|a, b| a.name == b.name);
    let local = local_structures(&files);
    let texts: BTreeMap<&str, &str> = files
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let mut by_file: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (name, paths) in &local {
        for path in paths {
            if !MISNAMED.contains(&(path.as_str(), name.as_str())) {
                by_file.entry(path).or_default().push(name);
            }
        }
    }
    let mut fragments: BTreeMap<&str, Vec<Record>> = BTreeMap::new();
    let mut problems = Vec::new();
    for (path, wanted) in &by_file {
        let file = source.join(path);
        let output = clang(&file, &includes).output().map_err(|e| Error::Io {
            path: PathBuf::from("clang"),
            source: e,
        })?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let errors = error_lines(&stderr, &file);
        let text = texts.get(path).copied().unwrap_or_default();
        let checks = asserts(text);
        let dumped = read_dump(&String::from_utf8_lossy(&output.stdout));
        for name in wanted {
            let Some((start, end)) = span(text, name) else {
                continue;
            };
            if let Some(line) = errors.iter().find(|&&l| l >= start && l <= end) {
                problems.push(format!("{path}:{line}: clang fails inside {name}"));
                continue;
            }
            let Some(record) = dumped.iter().find(|r| &r.name == name) else {
                problems.push(format!("{path}: clang lays out no {name}"));
                continue;
            };
            let own: Vec<Assert> = checks
                .iter()
                .filter(|a| a.structure == *name)
                .cloned()
                .collect();
            problems.extend(
                disagreements(std::slice::from_ref(record), &own)
                    .into_iter()
                    .map(|why| format!("{path}: {why}")),
            );
            let mut record = record.clone();
            if let Some((_, _, shift)) = SHIFTED
                .iter()
                .find(|(file, structure, _)| file == path && structure == name)
            {
                record.size += shift;
                for field in &mut record.fields {
                    field.offset += shift;
                }
            }
            fragments.entry(name).or_default().push(record);
        }
    }
    for (name, found) in &fragments {
        match merge(name, found) {
            Ok(record) => {
                let size = TAG_SIZES
                    .iter()
                    .find(|(structure, _)| structure == name)
                    .map_or(record.size, |(_, size)| *size);
                records.push(grown(record, size));
            }
            Err(Error::Problems(more)) => problems.extend(more),
            Err(other) => return Err(other),
        }
    }
    if !problems.is_empty() {
        return Err(Error::Problems(problems));
    }
    records.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(records)
}
