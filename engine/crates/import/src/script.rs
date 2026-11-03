//! A scenario's scripts: its compiled HaloScript nodes (a data array in a
//! tag data field), the strings they name, its scripts and its globals.
//!
//! A node's children are linked as a list: a call's data is its first
//! child, the node naming its function, and the arguments follow it by
//! their `next` links. The nodes are first fixed up as the game fixes them
//! up as it loads the scenario ([`crate::script_fixup`]). Every link is
//! checked, and an expression that reaches a node twice (a loop among
//! them) is refused, as the original's port refuses it
//! (`hs_expression_refusal`). A scenario whose scripts the game would not
//! run, or that are refused, loads with none ([`Scripts::disabled`]).

use std::collections::BTreeMap;

use opence_content::{
    Call, Constant, GlobalRef, Named, NodeKind, Script, ScriptGlobal, ScriptKind, ScriptNode,
    Scripts, ValueType,
};

use crate::cache::Cache;
use crate::error::ImportError;
use crate::level::{tag_ref, SCENARIO_SIZE, SCNR};
use crate::reader;
pub use crate::script_fixup::Node;
use crate::script_fixup::{postprocess, Encounter, Names, DYNAMIC_SOURCE_BYTES};
use crate::script_tables::{EXTERNAL_GLOBALS, FUNCTIONS};
use crate::tags::{Element, GroupTag};

const SYNTAX_DATA_OFFSET: usize = 0x474;
const STRING_CONSTANTS_OFFSET: usize = 0x488;
const SCRIPTS_OFFSET: usize = 0x49C;
const GLOBALS_OFFSET: usize = 0x4A8;
const REFERENCES_OFFSET: usize = 0x4B4;
const SOURCE_FILES_OFFSET: usize = 0x4C0;
const REFERENCE_SIZE: usize = 0x28;
const OBJECT_NAMES: (usize, usize) = (0x204, 0x24);
const ENCOUNTERS: (usize, usize) = (0x42C, 0xB0);
const SQUADS: (usize, usize) = (0x80, 0xE8);
const PLATOONS: (usize, usize) = (0x8C, 0xAC);
const HUD_MESSAGES_OFFSET: usize = 0x594;
/// [`crate::script_fixup::DATUM_TYPES`]' blocks: where each is, its elements' size and where in
/// them the name is.
const DATUM_BLOCKS: [(usize, usize, usize); 9] = [
    (0x360, 0x60, 4),
    (0x4E4, 0x5C, 4),
    (0x4F0, 0x68, 4),
    (0x4FC, 0x60, 4),
    (0x36C, 0x40, 0),
    (0x288, 0x34, 0),
    (0x438, 0x60, 0),
    (0x348, 0x68, 0),
    (0x468, 0x74, 0),
];
/// The most elements a block the names are read from is taken to have.
const MAX_ELEMENTS: usize = 0x8000;
const NAME_SIZE: usize = 32;

const MATG: GroupTag = GroupTag::new(b"matg");
const GLOBALS_PATH: &str = "globals\\globals";
const GLOBALS_SIZE: usize = 0x1AC;
const INTERFACE: (usize, usize) = (0x140, 0x130);
const HUDG: GroupTag = GroupTag::new(b"hudg");
const HUD_GLOBALS_SIZE: usize = 0x2DC;
const WAYPOINT_ARROWS: (usize, usize) = (0x160, 0x68);
const HMT: GroupTag = GroupTag::new(b"hmt ");
const HUD_MESSAGE_TEXT_SIZE: usize = 0x80;
const HMT_MESSAGES: (usize, usize) = (0x20, 0x40);

/// The nodes a scenario has room for.
const MAX_NODES: usize = 19001;
const NODE_SIZE: usize = 0x14;
/// The data array's header, before its nodes.
const DATA_ARRAY_SIZE: usize = 0x38;
/// `d@t@`, a data array's signature.
const DATA_ARRAY_SIGNATURE: u32 = 0x6440_7440;
const MAX_STRING_BYTES: usize = 0x40000;
/// The longest string a node names.
const MAX_STRING: usize = 0x1000;
const MAX_SCRIPTS: usize = 512;
/// The globals the game's array of them holds, the external ones first.
const MAX_GLOBALS: usize = 0x400;
const SCRIPT_SIZE: usize = 0x5C;
const GLOBAL_SIZE: usize = 0x5C;

const PRIMITIVE: i16 = 1 << 0;
const SCRIPT_CALL: i16 = 1 << 1;
const VARIABLE: i16 = 1 << 2;
/// An external (engine) global's index has this bit.
const EXTERNAL_GLOBAL: i32 = 0x8000;
const NONE: i32 = -1;

struct Reader<'a, 'e> {
    scenario: &'e Element<'a>,
    nodes: &'e [Node],
    count: usize,
    strings: &'a [u8],
    globals: usize,
    scripts: usize,
    /// The nodes made so far, and by the map's index.
    made: Vec<Option<ScriptNode>>,
    by_index: BTreeMap<usize, u32>,
}

impl<'a> Reader<'a, '_> {
    fn fault(&self, reason: impl ToString) -> ImportError {
        self.scenario.fault("script syntax data", reason)
    }

    /// The node a link names, by its index in the array.
    fn index(&self, handle: i32) -> Result<usize, ImportError> {
        let index = usize::from(handle as u32 as u16);
        let raw = self.raw(index)?;
        // (a link's salt above its index is the node's)
        if raw.salt == 0 || raw.salt != (handle >> 16) as i16 {
            return Err(self.fault(format!("links to node {index}, which is free")));
        }
        Ok(index)
    }

    fn raw(&self, index: usize) -> Result<Node, ImportError> {
        self.nodes
            .get(index)
            .filter(|_| index < self.count)
            .copied()
            .ok_or_else(|| self.fault(format!("node {index} of {}", self.count)))
    }

    /// A string the nodes name, read as Latin-1 (c10's has bytes that are
    /// not UTF-8).
    fn string(&self, offset: i32) -> Result<String, ImportError> {
        let at = usize::try_from(offset).map_err(|_| self.fault("a string before the strings"))?;
        let rest = self.strings.get(at..).unwrap_or_default();
        let window = &rest[..rest.len().min(MAX_STRING + 1)];
        let end = window
            .iter()
            .position(|&byte| byte == 0)
            .ok_or_else(|| self.fault(format!("the string at {at} has no end")))?;
        Ok(window[..end].iter().map(|&byte| char::from(byte)).collect())
    }

    /// Makes the expression at `handle` and returns its root's index.
    fn expression(&mut self, handle: i32) -> Result<u32, ImportError> {
        // (each expression is walked alone: one reached from two roots is
        // made once, but one reached twice from one root is a loop)
        let mut walked = vec![false; self.count];
        let root = self.index(handle)?;
        let mut stack = vec![root];
        walked[root] = true;
        let root = self.place(root);
        while let Some(index) = stack.pop() {
            let Some(&at) = self.by_index.get(&index) else {
                continue;
            };
            if self.made.get(at as usize).is_some_and(Option::is_some) {
                continue;
            }
            let raw = self.raw(index)?;
            let node = if raw.flags & PRIMITIVE != 0 {
                self.primitive(&raw)?
            } else {
                let mut children = Vec::new();
                let mut link = raw.data;
                while link != NONE {
                    let child = self.index(link)?;
                    if std::mem::replace(&mut walked[child], true) {
                        return Err(self.fault(format!("node {child} is linked to twice")));
                    }
                    children.push(child);
                    link = self.raw(child)?.next;
                }
                let Some((_, arguments)) = children.split_first() else {
                    return Err(self.fault(format!("call {index} names no function")));
                };
                let kind = if raw.flags & SCRIPT_CALL != 0 {
                    let script = u32::try_from(raw.index)
                        .ok()
                        .filter(|&script| (script as usize) < self.scripts)
                        .ok_or_else(|| self.fault(format!("calls script {}", raw.index)))?;
                    if !arguments.is_empty() {
                        return Err(self.fault(format!("passes script {script} arguments")));
                    }
                    NodeKind::ScriptCall(script)
                } else {
                    let mut placed = Vec::with_capacity(arguments.len());
                    for &argument in arguments {
                        placed.push(self.place(argument));
                        stack.push(argument);
                    }
                    // (the fix-up found the function by its name)
                    let (function, _) = usize::try_from(raw.index)
                        .ok()
                        .and_then(|function| FUNCTIONS.get(function))
                        .ok_or_else(|| self.fault(format!("calls function {}", raw.index)))?;
                    NodeKind::Call(Call {
                        function: (*function).to_owned(),
                        arguments: placed,
                    })
                };
                ScriptNode {
                    value_type: self.value_type(raw.value_type)?,
                    kind,
                }
            };
            if let Some(slot) = self.made.get_mut(at as usize) {
                *slot = Some(node);
            }
        }
        Ok(root)
    }

    /// The index the node at `index` is made at.
    fn place(&mut self, index: usize) -> u32 {
        if let Some(&at) = self.by_index.get(&index) {
            return at;
        }
        let at = self.made.len() as u32;
        self.made.push(None);
        self.by_index.insert(index, at);
        at
    }

    fn value_type(&self, index: i16) -> Result<ValueType, ImportError> {
        ValueType::from_index(index).ok_or_else(|| self.fault(format!("a value of type {index}")))
    }

    fn primitive(&self, raw: &Node) -> Result<ScriptNode, ImportError> {
        let value_type = self.value_type(raw.value_type)?;
        let bytes = raw.data.to_le_bytes();
        let short = i16::from_le_bytes([bytes[0], bytes[1]]);
        let kind = if raw.flags & VARIABLE != 0 {
            let external = raw.data & EXTERNAL_GLOBAL != 0;
            let index = u16::try_from(raw.data & 0x7FFF).unwrap_or(0);
            if !external && usize::from(index) >= self.globals {
                return Err(self.fault(format!("names global {index} of {}", self.globals)));
            }
            NodeKind::Global(GlobalRef {
                name: self.string(raw.string)?,
                external,
                index,
            })
        } else {
            // (a constant is decoded as the type it was written as, which
            // its index holds; the runtime casts it to the node's type)
            let written = self.value_type(raw.index)?;
            NodeKind::Constant(
                written,
                match written {
                    ValueType::Boolean => Constant::Boolean(bytes[0] != 0),
                    ValueType::Real => {
                        let real = f32::from_le_bytes(bytes);
                        if !real.is_finite() {
                            return Err(self.fault("a real that is not finite"));
                        }
                        Constant::Real(real)
                    }
                    ValueType::Short => Constant::Short(short),
                    ValueType::Long => Constant::Long(raw.data),
                    ValueType::String => Constant::String(self.string(raw.string)?),
                    tag if tag.is_tag() => Constant::Tag(if raw.data == NONE {
                        None
                    } else {
                        let cache = self.scenario.cache();
                        let index = cache
                            .handle_index(raw.data as u32)
                            .ok_or_else(|| self.fault(format!("names tag {:#x}", raw.data)))?;
                        Some(tag_ref(cache, index))
                    }),
                    // (the values below a string are the special forms' and
                    // functions' names, never constants)
                    ValueType::Unparsed
                    | ValueType::SpecialForm
                    | ValueType::FunctionName
                    | ValueType::Passthrough
                    | ValueType::Void => Constant::Long(raw.data),
                    other => Constant::Named(Named {
                        name: self.string(raw.string)?,
                        // (an encounter's or an object's is a long; the rest
                        // are shorts)
                        index: if matches!(other, ValueType::Ai | ValueType::ObjectList)
                            || other.is_object()
                        {
                            raw.data
                        } else {
                            i32::from(short)
                        },
                    }),
                },
            )
        };
        Ok(ScriptNode { value_type, kind })
    }
}

/// Imports the cache's scenario's scripts and globals and the nodes of
/// their expressions.
pub fn import_scripts(cache: &Cache) -> Result<Scripts, ImportError> {
    let scenario = cache.tag(cache.scenario_index(), SCNR, SCENARIO_SIZE)?;
    Ok(scenario_scripts(&scenario))
}

/// The cache's scenario's script nodes (all the array has room for) as the
/// map stores them, and as the game's fix-up leaves them.
pub fn fixed_up_nodes(cache: &Cache) -> Result<(Vec<Node>, Vec<Node>), ImportError> {
    let scenario = cache.tag(cache.scenario_index(), SCNR, SCENARIO_SIZE)?;
    let fault = |reason: &str| scenario.fault("script syntax data", reason);
    let syntax = scenario.data(
        SYNTAX_DATA_OFFSET,
        "script syntax data",
        DATA_ARRAY_SIZE + MAX_NODES * NODE_SIZE,
    )?;
    let count = reader::i16(syntax, 0x2E).map_err(|f| scenario.fault("script syntax data", f))?;
    let stored = syntax_nodes(&scenario, syntax.get(DATA_ARRAY_SIZE..).unwrap_or_default())?;
    let strings = scenario.data(STRING_CONSTANTS_OFFSET, "script strings", MAX_STRING_BYTES)?;
    let scripts = scenario.block(SCRIPTS_OFFSET, "scripts", SCRIPT_SIZE, MAX_SCRIPTS)?;
    let globals = scenario.block(GLOBALS_OFFSET, "globals", GLOBAL_SIZE, MAX_GLOBALS)?;
    let mut fixed = stored.clone();
    let count = usize::try_from(count).unwrap_or_default();
    postprocess(
        &mut fixed,
        count,
        strings,
        &names(&scenario, &scripts, &globals)?,
    )
    .map_err(fault)?;
    Ok((stored, fixed))
}

/// Imports a scenario's scripts and globals and the nodes of their
/// expressions; or none, with the reason, when the game would not run them
/// or they are refused.
pub(crate) fn scenario_scripts(scenario: &Element<'_>) -> Scripts {
    compile(scenario).unwrap_or_else(|fault| Scripts {
        disabled: Some(fault.to_string()),
        ..Scripts::default()
    })
}

/// A block's count as the scenario holds it.
fn count(scenario: &Element<'_>, at: usize, field: &str) -> Result<usize, ImportError> {
    let count = scenario.i32(at, field)?;
    usize::try_from(count).map_err(|_| scenario.fault(field, format!("has {count} elements")))
}

/// A name of at most `size` bytes, to its end.
fn name(element: &Element<'_>, at: usize, size: usize) -> Result<Vec<u8>, ImportError> {
    let bytes = element.slice(at, size, "name")?;
    let end = bytes
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(bytes.len());
    Ok(bytes[..end].to_vec())
}

/// The names of a block's elements.
fn names_of(
    element: &Element<'_>,
    (at, size): (usize, usize),
    name_at: usize,
    field: &str,
) -> Result<Vec<Vec<u8>>, ImportError> {
    element
        .block(at, field, size, MAX_ELEMENTS)?
        .iter()
        .map(|element| name(&element, name_at, NAME_SIZE))
        .collect()
}

/// The HUD globals' waypoint arrows' names, if the game's globals name HUD
/// globals.
fn navpoints(cache: &Cache) -> Result<Option<Vec<Vec<u8>>>, ImportError> {
    let Some(index) = cache
        .find(MATG, GLOBALS_PATH)
        .or_else(|| cache.tags().iter().position(|instance| instance.is(MATG)))
    else {
        return Ok(None);
    };
    let globals = cache.tag(index, MATG, GLOBALS_SIZE)?;
    let interface = globals.block(INTERFACE.0, "interface bitmaps", INTERFACE.1, 1)?;
    let Some(hud_globals) = interface
        .get(0)
        .map(|interface| interface.reference(0x60, "hud globals", &[HUDG]))
        .transpose()?
        .flatten()
    else {
        return Ok(None);
    };
    let hud_globals = cache.tag(hud_globals, HUDG, HUD_GLOBALS_SIZE)?;
    Ok(Some(names_of(
        &hud_globals,
        WAYPOINT_ARROWS,
        0,
        "waypoint arrows",
    )?))
}

/// The names the fix-up looks the nodes' text up in.
fn names(
    scenario: &Element<'_>,
    scripts: &crate::tags::Block<'_>,
    globals: &crate::tags::Block<'_>,
) -> Result<Names, ImportError> {
    let cache = scenario.cache();
    let mut names = Names::default();
    for script in scripts.iter() {
        names.scripts.push((
            name(&script, 0, NAME_SIZE)?,
            script.i16(0x20, "type")?,
            script.i16(0x22, "return type")?,
        ));
    }
    for global in globals.iter() {
        names
            .globals
            .push((name(&global, 0, NAME_SIZE)?, global.i16(0x20, "type")?));
    }
    for (list, &(at, size, name_at)) in names.datums.iter_mut().zip(&DATUM_BLOCKS) {
        *list = names_of(scenario, (at, size), name_at, "named elements")?;
    }
    for encounter in scenario
        .block(ENCOUNTERS.0, "encounters", ENCOUNTERS.1, MAX_ELEMENTS)?
        .iter()
    {
        names.encounters.push(Encounter {
            name: name(&encounter, 0, NAME_SIZE)?,
            squads: names_of(&encounter, SQUADS, 0, "squads")?,
            platoons: names_of(&encounter, PLATOONS, 0, "platoons")?,
        });
    }
    names.navpoints = navpoints(cache)?;
    if let Some(index) = scenario.reference(HUD_MESSAGES_OFFSET, "hud messages", &[HMT])? {
        let messages = cache.tag(index, HMT, HUD_MESSAGE_TEXT_SIZE)?;
        names.hud_messages = Some(names_of(&messages, HMT_MESSAGES, 0, "messages")?);
    }
    for object_name in scenario
        .block(OBJECT_NAMES.0, "object names", OBJECT_NAMES.1, MAX_ELEMENTS)?
        .iter()
    {
        names.object_names.push((
            name(&object_name, 0, NAME_SIZE)?,
            object_name.i16(0x20, "type")?,
        ));
    }
    for reference in scenario
        .block(
            REFERENCES_OFFSET,
            "script references",
            REFERENCE_SIZE,
            MAX_ELEMENTS,
        )?
        .iter()
    {
        // (the name is the path of the tag the handle names)
        let group = reference.u32(0x18, "reference")?;
        let handle = reference.i32(0x18 + 0xC, "reference")?;
        if let Some(index) = cache.handle_index(handle as u32) {
            let path = cache.tags()[index].name.bytes().collect();
            names.references.push((path, group, handle));
        }
    }
    Ok(names)
}

/// The nodes of the syntax data's array, all it has room for.
fn syntax_nodes(scenario: &Element<'_>, nodes: &[u8]) -> Result<Vec<Node>, ImportError> {
    let read = |at: usize| -> Result<Node, reader::Fault> {
        Ok(Node {
            salt: reader::i16(nodes, at)?,
            index: reader::i16(nodes, at + 2)?,
            value_type: reader::i16(nodes, at + 4)?,
            flags: reader::i16(nodes, at + 6)?,
            next: reader::i32(nodes, at + 8)?,
            string: reader::i32(nodes, at + 0xC)?,
            data: reader::i32(nodes, at + 0x10)?,
        })
    };
    (0..MAX_NODES)
        .map(|index| read(index * NODE_SIZE).map_err(|f| scenario.fault("script syntax data", f)))
        .collect()
}

/// The scenario's scripts, or the fault for which the game would not run
/// them (hs_scenario_postprocess) or they are refused.
fn compile(scenario: &Element<'_>) -> Result<Scripts, ImportError> {
    let syntax = scenario.data(
        SYNTAX_DATA_OFFSET,
        "script syntax data",
        DATA_ARRAY_SIZE + MAX_NODES * NODE_SIZE,
    )?;
    let script_count = count(scenario, SCRIPTS_OFFSET, "scripts")?;
    let global_count = count(scenario, GLOBALS_OFFSET, "globals")?;
    if syntax.is_empty() && script_count == 0 && global_count == 0 {
        return Ok(Scripts::default());
    }
    let fault = |reason: &str| scenario.fault("script syntax data", reason);
    let header =
        |at: usize| reader::i16(syntax, at).map_err(|f| scenario.fault("script syntax data", f));
    let strings = scenario.data(STRING_CONSTANTS_OFFSET, "script strings", MAX_STRING_BYTES)?;
    let signature =
        reader::u32(syntax, 0x28).map_err(|f| scenario.fault("script syntax data", f))?;
    let valid = *syntax.get(0x24).ok_or_else(|| fault("has no header"))?;
    let (max, size) = (header(0x20)?, header(0x22)?);
    let (first_free, count, actual) = (header(0x2C)?, header(0x2E)?, header(0x30)?);
    if syntax.len() != DATA_ARRAY_SIZE + MAX_NODES * NODE_SIZE
        || signature != DATA_ARRAY_SIGNATURE
        || usize::try_from(max).ok() != Some(MAX_NODES)
        || usize::try_from(size).ok() != Some(NODE_SIZE)
        || valid == 0
        || !(0..=max).contains(&count)
        || !(0..=count).contains(&actual)
        || !(0..=max).contains(&first_free)
        || strings.len() < DYNAMIC_SOURCE_BYTES
    {
        return Err(fault("is not an array of script nodes"));
    }
    if global_count > MAX_GLOBALS - EXTERNAL_GLOBALS.len() {
        return Err(scenario.fault(
            "globals",
            format!("has {global_count}, more than there is room for"),
        ));
    }
    if script_count > MAX_SCRIPTS {
        return Err(scenario.fault(
            "scripts",
            format!("has {script_count}, more than there is room for"),
        ));
    }
    let script_block = scenario.block(SCRIPTS_OFFSET, "scripts", SCRIPT_SIZE, MAX_SCRIPTS)?;
    let global_block = scenario.block(GLOBALS_OFFSET, "globals", GLOBAL_SIZE, MAX_GLOBALS)?;
    for global in global_block.iter() {
        let kind = global.i16(0x20, "type")?;
        // (hs_type_valid: a value's type, past the special forms' and void)
        if !(4..ValueType::ALL.len() as i16).contains(&kind) {
            return Err(global.fault("type", format!("is {kind}, no value's type")));
        }
    }
    if script_count == 0 && scenario.i32(SOURCE_FILES_OFFSET, "source files")? > 0 {
        return Err(scenario.fault("scripts", "are to be compiled again from their source"));
    }
    let count = usize::try_from(count).unwrap_or_default();
    let mut nodes = syntax_nodes(scenario, &syntax[DATA_ARRAY_SIZE..])?;
    let names = names(scenario, &script_block, &global_block)?;
    postprocess(&mut nodes, count, strings, &names).map_err(fault)?;
    let mut reader = Reader {
        scenario,
        nodes: &nodes,
        count,
        strings,
        globals: global_block.len(),
        scripts: script_block.len(),
        made: Vec::new(),
        by_index: BTreeMap::new(),
    };

    let mut scripts = Vec::with_capacity(script_block.len());
    for element in script_block.iter() {
        let name = element.text(0, "name", 31)?.to_owned();
        let kind = element.i16(0x20, "type")?;
        let kind = ScriptKind::from_index(kind)
            .ok_or_else(|| element.fault("type", format!("is {kind}")))?;
        let return_type = reader.value_type(element.i16(0x22, "return type")?)?;
        let root = reader.expression(element.i32(0x24, "root expression")?)?;
        scripts.push(Script {
            name,
            kind,
            return_type,
            root,
        });
    }
    let mut globals = Vec::with_capacity(global_block.len());
    for element in global_block.iter() {
        let name = element.text(0, "name", 31)?.to_owned();
        let value_type = reader.value_type(element.i16(0x20, "type")?)?;
        let initial = element.i32(0x28, "initialization expression")?;
        let initial = if initial == NONE {
            None
        } else {
            Some(reader.expression(initial)?)
        };
        globals.push(ScriptGlobal {
            name,
            value_type,
            initial,
        });
    }
    let made = reader
        .made
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| scenario.fault("script syntax data", "a node was not made"))?;
    Ok(Scripts {
        nodes: made,
        scripts,
        globals,
        disabled: None,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::cache::CacheKind;
    use crate::fake::FakeCache;
    use crate::level::tests::{block, put};

    type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

    const SALT: i16 = 0x6000;

    /// A node: its type, flags, next link, string and data.
    struct Node(i16, i16, i16, i32, i32, i32);

    fn handle(index: usize) -> i32 {
        i32::from(SALT) << 16 | index as i32
    }

    /// A scenario of one global and one startup script, whose nodes are
    /// `nodes` (the script's root the first) and strings `strings`.
    fn scenario(nodes: &[Node], strings: &[u8]) -> Result<Scripts> {
        Ok(import_scripts(&Cache::open(
            scenario_cache(nodes, strings).build(),
        )?)?)
    }

    /// `(begin (set count 3))` in a cache of its own (the scripts fuzz
    /// target's seed).
    pub(crate) fn script_cache() -> FakeCache {
        scenario_cache(&nodes(), &strings(STRINGS))
    }

    /// The cache of [`scenario`]'s scenario.
    fn scenario_cache(nodes: &[Node], strings: &[u8]) -> FakeCache {
        let mut fake = FakeCache::new(CacheKind::Xbox);
        let mut syntax = vec![0u8; DATA_ARRAY_SIZE + MAX_NODES * NODE_SIZE];
        put(&mut syntax, 0x20, &(MAX_NODES as i16).to_le_bytes());
        put(&mut syntax, 0x22, &(NODE_SIZE as i16).to_le_bytes());
        put(&mut syntax, 0x24, &[1]);
        put(&mut syntax, 0x28, &DATA_ARRAY_SIGNATURE.to_le_bytes());
        put(&mut syntax, 0x2E, &(nodes.len() as i16).to_le_bytes());
        put(&mut syntax, 0x30, &(nodes.len() as i16).to_le_bytes());
        for (index, Node(value_type, function, flags, next, string, data)) in
            nodes.iter().enumerate()
        {
            let at = DATA_ARRAY_SIZE + index * NODE_SIZE;
            put(&mut syntax, at, &SALT.to_le_bytes());
            put(&mut syntax, at + 2, &function.to_le_bytes());
            put(&mut syntax, at + 4, &value_type.to_le_bytes());
            put(&mut syntax, at + 6, &flags.to_le_bytes());
            put(&mut syntax, at + 8, &next.to_le_bytes());
            put(&mut syntax, at + 0xC, &string.to_le_bytes());
            put(&mut syntax, at + 0x10, &data.to_le_bytes());
        }
        let syntax_size = syntax.len() as i32;
        let syntax = fake.add_data(&syntax);
        let strings_address = fake.add_data(strings);
        let mut script = vec![0u8; SCRIPT_SIZE];
        put(&mut script, 0, b"main");
        put(&mut script, 0x22, &4i16.to_le_bytes());
        put(&mut script, 0x24, &handle(0).to_le_bytes());
        let script = fake.add_data(&script);
        let mut global = vec![0u8; GLOBAL_SIZE];
        put(&mut global, 0, b"count");
        put(&mut global, 0x20, &7i16.to_le_bytes());
        put(&mut global, 0x28, &NONE.to_le_bytes());
        let global = fake.add_data(&global);

        let mut data = vec![0u8; SCENARIO_SIZE];
        put(&mut data, SYNTAX_DATA_OFFSET, &syntax_size.to_le_bytes());
        put(&mut data, SYNTAX_DATA_OFFSET + 12, &syntax.to_le_bytes());
        put(
            &mut data,
            STRING_CONSTANTS_OFFSET,
            &(strings.len() as i32).to_le_bytes(),
        );
        put(
            &mut data,
            STRING_CONSTANTS_OFFSET + 12,
            &strings_address.to_le_bytes(),
        );
        block(&mut data, SCRIPTS_OFFSET, 1, script);
        block(&mut data, GLOBALS_OFFSET, 1, global);
        put(&mut data, HUD_MESSAGES_OFFSET + 12, &NONE.to_le_bytes());
        fake.set_scenario_data(&data);
        fake
    }

    /// `(begin (set count 3))`: the strings are "begin\0set\0count\0".
    fn nodes() -> Vec<Node> {
        vec![
            // (begin ...), its name, (set ...), its name, count, 3
            Node(4, 0, 0, NONE, 0, handle(1)),
            Node(2, 0, PRIMITIVE, handle(2), 0, 0),
            Node(4, 4, 0, NONE, 0, handle(3)),
            Node(2, 4, PRIMITIVE, handle(4), 6, 0),
            Node(7, 0, PRIMITIVE | VARIABLE, handle(5), 10, 0),
            Node(7, 7, PRIMITIVE, NONE, 0, 3),
        ]
    }

    const STRINGS: &[u8] = b"begin\0set\0count\0";

    /// `text` and the room after it for the console's expressions.
    fn strings(text: &[u8]) -> Vec<u8> {
        let mut strings = text.to_vec();
        strings.resize(text.len() + DYNAMIC_SOURCE_BYTES, 0);
        strings
    }

    /// Why the scripts of a scenario of `nodes` and `text` do not run.
    fn disabled(nodes: &[Node], text: &[u8]) -> Result<Option<String>> {
        let scripts = scenario(nodes, &strings(text))?;
        if scripts.disabled.is_some() {
            assert_eq!(
                scripts,
                Scripts {
                    disabled: scripts.disabled.clone(),
                    ..Scripts::default()
                }
            );
        }
        Ok(scripts.disabled)
    }

    #[test]
    fn a_script_imports_as_a_tree() -> Result {
        let scripts = scenario(&nodes(), &strings(STRINGS))?;
        assert_eq!(scripts.disabled, None);
        assert_eq!(scripts.scripts[0].name, "main");
        assert_eq!(scripts.scripts[0].kind, ScriptKind::Startup);
        assert_eq!(scripts.scripts[0].return_type, ValueType::Void);
        assert_eq!(scripts.globals[0].value_type, ValueType::Short);
        let NodeKind::Call(begin) = &scripts.nodes[scripts.scripts[0].root as usize].kind else {
            return Err("not a call".into());
        };
        assert_eq!(begin.function, "begin");
        let NodeKind::Call(set) = &scripts.nodes[begin.arguments[0] as usize].kind else {
            return Err("not a call".into());
        };
        assert_eq!(set.function, "set");
        let [global, value] = set.arguments[..] else {
            return Err("not two arguments".into());
        };
        assert!(matches!(
            &scripts.nodes[global as usize].kind,
            NodeKind::Global(GlobalRef { name, external: false, index: 0 }) if name == "count"
        ));
        assert_eq!(
            scripts.nodes[value as usize].kind,
            NodeKind::Constant(ValueType::Short, Constant::Short(3))
        );
        Ok(())
    }

    #[test]
    fn a_loop_among_the_nodes_disables_the_scripts() -> Result {
        // (3 links back to the call it is in)
        let mut looped = nodes();
        looped[5].3 = handle(2);
        assert!(disabled(&looped, STRINGS)?.is_some());
        Ok(())
    }

    #[test]
    fn a_link_to_a_free_node_or_past_the_nodes_disables_the_scripts() -> Result {
        let mut past = nodes();
        past[5].3 = handle(40);
        assert!(disabled(&past, STRINGS)?.is_some());
        let mut salted = nodes();
        salted[1].3 = handle(2) ^ 0x0100_0000;
        assert!(disabled(&salted, STRINGS)?.is_some());
        Ok(())
    }

    #[test]
    fn a_variable_is_found_by_its_name() -> Result {
        // (its stored index is the fix-up's to set)
        let mut stale = nodes();
        stale[4].5 = 1;
        assert_eq!(disabled(&stale, STRINGS)?, None);
        // ("set" names no global)
        let mut missing = nodes();
        missing[4].4 = 6;
        assert!(disabled(&missing, STRINGS)?.is_some());
        Ok(())
    }

    #[test]
    fn a_bad_string_type_or_function_disables_the_scripts() -> Result {
        let mut past = nodes();
        past[3].4 = 0x1000;
        assert!(disabled(&past, STRINGS)?.is_some());
        let mut written = nodes();
        written[5].1 = 6;
        assert!(disabled(&written, STRINGS)?.is_some());
        assert!(disabled(&nodes(), b"begin\0sex\0count\0")?.is_some());
        // (without the console's room after them)
        assert!(scenario(&nodes(), STRINGS)?.disabled.is_some());
        Ok(())
    }
}
