//! The game's fix-up of a scenario's compiled scripts as it loads them
//! (hs_compile_postprocess): every call's function is found by its name,
//! every variable's global by its name, and every named constant (a script,
//! a trigger volume, an encounter, an object's name, a tag...) is parsed
//! again from its text into the index it stands for. A node whose type does
//! not fit the value it makes fails the fix-up, and the scenario's scripts
//! do not run.
//!
//! The port is checked against the original's own functions by
//! tools/engine_script_postprocess_reference.py.

use crate::script_tables::{
    CASTS, ENUMS, EXTERNAL_GLOBALS, FUNCTIONS, OBJECT_TYPE_MASKS, TAG_GROUPS,
};

/// A node of the syntax data, as the map holds it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Node {
    /// Its identifier (0 when the node is free).
    pub salt: i16,
    /// A call's function, a script call's script, or a constant's type.
    pub index: i16,
    /// The type its parent takes its value as.
    pub value_type: i16,
    /// Its flags: primitive, script call, variable.
    pub flags: i16,
    /// The next argument.
    pub next: i32,
    /// Where its text is in the string constants.
    pub string: i32,
    /// A call's first child, or a constant's value.
    pub data: i32,
}

/// An encounter's name and its squads' and platoons' names.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Encounter {
    /// Its name.
    pub name: Vec<u8>,
    /// Its squads' names.
    pub squads: Vec<Vec<u8>>,
    /// Its platoons' names.
    pub platoons: Vec<Vec<u8>>,
}

/// The names the fix-up looks a node's text up in: the scenario's, and
/// the tags it refers to.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Names {
    /// Each script's name, kind and return type.
    pub scripts: Vec<(Vec<u8>, i16, i16)>,
    /// Each global's name and type.
    pub globals: Vec<(Vec<u8>, i16)>,
    /// The names of each of [`DATUM_TYPES`]' blocks' elements.
    pub datums: [Vec<Vec<u8>>; 9],
    /// The AI encounters.
    pub encounters: Vec<Encounter>,
    /// The HUD globals' waypoint arrows, if there are HUD globals.
    pub navpoints: Option<Vec<Vec<u8>>>,
    /// The scenario's HUD messages, if it names their tag.
    pub hud_messages: Option<Vec<Vec<u8>>>,
    /// Each object name and the kind of object it was given.
    pub object_names: Vec<(Vec<u8>, i16)>,
    /// Each tag the scripts refer to: its name, group and handle.
    pub references: Vec<(Vec<u8>, u32, i32)>,
}

/// The types whose constants name an element of one of the scenario's
/// blocks (trigger volumes, cutscene flags, camera points, titles,
/// recordings, device groups, command lists, starting profiles and
/// conversations), in [`Names::datums`]' order.
pub const DATUM_TYPES: [i16; 9] = [11, 12, 13, 14, 15, 16, 18, 19, 20];

/// The bytes at the end of the string constants that the console's
/// expressions are written to; the nodes' text is before them.
pub const DYNAMIC_SOURCE_BYTES: usize = 0x400;

const PRIMITIVE: i16 = 1 << 0;
const SCRIPT_CALL: i16 = 1 << 1;
const VARIABLE: i16 = 1 << 2;
const NONE: i32 = -1;

const PASSTHROUGH: i16 = 3;
const VOID: i16 = 4;
const BOOLEAN: i16 = 5;
const STRING: i16 = 9;
const FUNCTION_NAME: i16 = 2;
const FIRST_OBJECT_NAME: i16 = 43;
const OBJECT_TYPES: i16 = 6;

const BEGIN: i16 = 0;
const BEGIN_RANDOM: i16 = 1;
const IF: i16 = 2;
const SET: i16 = 4;

/// The longest encounter name, before an AI name's `/`.
const TAG_STRING_LENGTH: usize = 31;

fn valid(value_type: i16) -> bool {
    (VOID..CASTS.len() as i16).contains(&value_type)
}

/// hs_can_cast: whether a value of type `actual` may be taken as `desired`.
fn can_cast(actual: i16, desired: i16) -> bool {
    let (Ok(actual), Ok(desired)) = (u32::try_from(actual), usize::try_from(desired)) else {
        return false;
    };
    CASTS
        .get(desired)
        .is_some_and(|&mask| actual < 64 && mask >> actual & 1 != 0)
}

/// _stricmp's equality: the same bytes, letters in either case.
fn same_ignoring_case(a: &[u8], b: &[u8]) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// The type of a global, by its index (an external one's has 0x8000).
fn global_type(index: i16, names: &Names) -> i16 {
    let at = (index & 0x7FFF) as usize;
    if index as u16 & 0x8000 != 0 {
        EXTERNAL_GLOBALS.get(at).map_or(0, |&(_, kind)| kind)
    } else {
        names.globals.get(at).map_or(0, |&(_, kind)| kind)
    }
}

/// hs_find_global_by_name: an external global's index with 0x8000, a
/// scenario global's, or none.
fn find_global(name: &[u8], names: &Names) -> i32 {
    if let Some(at) = EXTERNAL_GLOBALS
        .iter()
        .position(|(global, _)| same_ignoring_case(global.as_bytes(), name))
    {
        // (the short is sign-extended into the node's long)
        return i32::from((at as u16 | 0x8000) as i16);
    }
    names
        .globals
        .iter()
        .position(|(global, _)| same_ignoring_case(global, name))
        .map_or(NONE, |at| at as i32)
}

/// ai_index_from_string: an encounter, an encounter's squad or platoon
/// (`encounter/squad`), or none.
fn ai_reference(text: &[u8], names: &Names) -> Option<i32> {
    if same_ignoring_case(text, b"none") {
        return Some(NONE);
    }
    let encounter = |name: &[u8]| {
        names
            .encounters
            .iter()
            .position(|encounter| same_ignoring_case(&encounter.name, name))
    };
    let Some(separator) = text.iter().rposition(|&byte| byte == b'/') else {
        return encounter(text).map(|at| at as i32 & 0xFFFF);
    };
    if separator > TAG_STRING_LENGTH + 1 {
        return None;
    }
    let at = encounter(&text[..separator])?;
    let (name, definition) = (&text[separator + 1..], names.encounters.get(at)?);
    let find = |list: &[Vec<u8>]| {
        list.iter()
            .position(|member| same_ignoring_case(member, name))
    };
    let (kind, member) = if let Some(squad) = find(&definition.squads) {
        (2, squad)
    } else {
        (1, find(&definition.platoons)?)
    };
    Some(kind << 30 | (member as i32 & 0xFF) << 16 | at as i32 & 0xFFFF)
}

/// The fix-up of one scenario's nodes.
struct Fixup<'a> {
    nodes: &'a mut [Node],
    count: usize,
    strings: &'a [u8],
    source_size: i64,
    names: &'a Names,
}

impl<'a> Fixup<'a> {
    /// datum_try_and_get: the node a handle names, if it is one.
    fn get(&self, handle: i32) -> Option<usize> {
        if handle == NONE {
            return None;
        }
        let identifier = (handle >> 16) as i16;
        let at = usize::try_from(handle as i16).ok()?;
        let node = self.nodes.get(at)?;
        (node.salt != 0 && (identifier == 0 || node.salt == identifier)).then_some(at)
    }

    /// The text at a node's source offset, if the offset is in the source.
    fn text(&self, offset: i32) -> Result<&'a [u8], &'static str> {
        if offset < 0 || i64::from(offset) >= self.source_size {
            return Err("bad source offset (you need to recompile.)");
        }
        let strings: &'a [u8] = self.strings;
        let rest = strings.get(offset as usize..).unwrap_or_default();
        let end = rest
            .iter()
            .position(|&byte| byte == 0)
            .ok_or("a string has no end")?;
        Ok(&rest[..end])
    }

    fn set_short(&mut self, at: usize, value: i16) {
        let node = &mut self.nodes[at];
        node.data = (node.data as u32 & 0xFFFF_0000 | u32::from(value as u16)) as i32;
    }

    /// hs_parse_primitive, its variables predetermined.
    fn parse_primitive(&mut self, at: usize) -> Result<(), &'static str> {
        let node = self.nodes[at];
        if node.value_type == 1 || node.value_type == VOID {
            return Err("a special form or void constant");
        }
        if node.flags & VARIABLE != 0 {
            return self.parse_variable(at);
        }
        if node.value_type == 0 {
            return Err("an unparsed constant");
        }
        self.parse(at)
    }

    fn parse_variable(&mut self, at: usize) -> Result<(), &'static str> {
        let node = self.nodes[at];
        let global = find_global(self.text(node.string)?, self.names);
        self.nodes[at].data = global;
        if global == NONE {
            return Err("this is not a valid variable name.");
        }
        let kind = global_type(global as i16, self.names);
        if node.value_type != 0 && !can_cast(kind, node.value_type) {
            return Err("the variable is not of the type expected");
        }
        if node.value_type == 0 {
            self.nodes[at].value_type = kind;
        }
        self.nodes[at].flags |= VARIABLE;
        Ok(())
    }

    /// The type's own parser (hs_type_primitive_parsers).
    fn parse(&mut self, at: usize) -> Result<(), &'static str> {
        let node = self.nodes[at];
        let names: &'a Names = self.names;
        match node.value_type {
            // (booleans, reals, integers and strings are as the map has them)
            5..=STRING => Ok(()),
            10 => {
                let text = self.text(node.string)?;
                let script = names
                    .scripts
                    .iter()
                    .position(|(name, ..)| name.as_slice() == text)
                    .ok_or("this is not a valid script name.")?;
                self.set_short(at, script as i16);
                Ok(())
            }
            17 => {
                let reference = ai_reference(self.text(node.string)?, names);
                self.nodes[at].data = reference.unwrap_or(NONE);
                match reference {
                    Some(_) => Ok(()),
                    None => Err("this is not a valid ai encounter or squad."),
                }
            }
            21 => match &names.navpoints {
                Some(list) => self.datum(at, list),
                None => Err("there are no hud globals"),
            },
            22 => match &names.hud_messages {
                Some(list) => self.datum(at, list),
                None => Err("the scenario has no hud messages"),
            },
            23 => {
                self.nodes[at].index = FIRST_OBJECT_NAME;
                self.nodes[at].value_type = FIRST_OBJECT_NAME;
                let result = self.parse_object_name(at);
                self.nodes[at].value_type = 23;
                result
            }
            kind @ 24..=31 => {
                let text = self.text(node.string)?;
                let group = u32::from_be_bytes(TAG_GROUPS[(kind - 24) as usize]);
                if let Some(&(_, _, handle)) = names
                    .references
                    .iter()
                    .find(|(name, g, _)| name.as_slice() == text && *g == group)
                {
                    self.nodes[at].data = handle;
                }
                Ok(())
            }
            kind @ 32..=36 => {
                let text = self.text(node.string)?;
                let values = ENUMS[(kind - 32) as usize];
                let found = values
                    .iter()
                    .position(|value| same_ignoring_case(text, value.as_bytes()))
                    .unwrap_or(values.len());
                self.set_short(at, found as i16);
                if found == values.len() {
                    return Err("not one of the enum's values");
                }
                Ok(())
            }
            37..=42 => {
                if self.text(node.string)? == b"none" {
                    self.nodes[at].data = NONE;
                    return Ok(());
                }
                self.nodes[at].value_type += OBJECT_TYPES;
                self.nodes[at].index = self.nodes[at].value_type;
                let result = self.parse_object_name(at);
                self.nodes[at].value_type -= OBJECT_TYPES;
                result
            }
            43..=48 => self.parse_object_name(at),
            kind => match DATUM_TYPES.iter().position(|&datum| datum == kind) {
                Some(block) => self.datum(at, &names.datums[block]),
                None => Err("expressions of this type are unsupported."),
            },
        }
    }

    /// hs_parse_scenario_datum: the element of `list` the text names.
    fn datum(&mut self, at: usize, list: &[Vec<u8>]) -> Result<(), &'static str> {
        let text = self.text(self.nodes[at].string)?;
        let found = list
            .iter()
            .position(|name| same_ignoring_case(name, text))
            .ok_or("this is not a valid name")?;
        self.nodes[at].data = found as i32;
        Ok(())
    }

    fn parse_object_name(&mut self, at: usize) -> Result<(), &'static str> {
        let node = self.nodes[at];
        let text = self.text(node.string)?;
        let found = self
            .names
            .object_names
            .iter()
            .take(i16::MAX as usize)
            .position(|(name, _)| name.as_slice() == text);
        let Some(found) = found else {
            if text == b"none" {
                self.nodes[at].data = NONE;
                return Ok(());
            }
            return Err("this is not a valid object name.");
        };
        let kind = self.names.object_names[found].1;
        let mask = OBJECT_TYPE_MASKS
            .get((node.value_type - FIRST_OBJECT_NAME) as usize)
            .map_or(0, |&mask| i32::from(mask as i16));
        // (TEST_FLAG's shift, as the x86 takes it)
        if mask & 1i32.wrapping_shl(kind as u32) == 0 {
            return Err("this is not an object of the type.");
        }
        self.set_short(at, found as i16);
        Ok(())
    }

    /// The type of the value a node makes, checked against what makes it.
    fn resolve(&mut self, at: usize) -> Result<(), &'static str> {
        let node = self.nodes[at];
        if !valid(node.value_type) {
            if node.value_type != FUNCTION_NAME {
                return Err("missing type (you need to recompile scripts.)");
            }
            return Ok(());
        }
        let resolved = if node.flags & PRIMITIVE != 0 {
            if node.value_type >= STRING || node.flags & VARIABLE != 0 {
                self.text(node.string)?;
                self.parse_primitive(at)?;
            }
            let node = self.nodes[at];
            if node.flags & VARIABLE != 0 {
                global_type(node.data as i16, self.names)
            } else if node.value_type < STRING && node.index != node.value_type {
                return Err(
                    "constant type is inconsistent with its type (you need to recompile scripts.)",
                );
            } else {
                node.index
            }
        } else if node.flags & SCRIPT_CALL != 0 {
            usize::try_from(node.index)
                .ok()
                .and_then(|script| self.names.scripts.get(script))
                .filter(|&&(_, kind, _)| kind == 3 || kind == 4)
                .map(|&(_, _, returns)| returns)
                .ok_or("bad script index (you need to recompile.)")?
        } else {
            let predicate = self
                .get(node.data)
                .filter(|&predicate| self.nodes[predicate].value_type == FUNCTION_NAME)
                .ok_or("corrupt syntax tree (you need to recompile scripts.)")?;
            let name = self.text(self.nodes[predicate].string)?;
            let function = FUNCTIONS
                .iter()
                .position(|(function, _)| same_ignoring_case(function.as_bytes(), name))
                .ok_or("missing function (you need to recompile scripts.)")?;
            self.nodes[at].index = function as i16;
            FUNCTIONS[function].1
        };
        let value_type = self.nodes[at].value_type;
        if (!valid(resolved) && resolved != PASSTHROUGH) || !can_cast(resolved, value_type) {
            return Err("type is inconsistent with usage (you need to recompile scripts.)");
        }
        Ok(())
    }

    fn next(&self, at: usize) -> Option<usize> {
        let next = self.nodes[at].next;
        (next != NONE).then(|| self.get(next)).flatten()
    }

    /// hs_postprocess_passthrough_arguments: whether the values a special
    /// form returns as its own are of its type.
    fn passthrough(&self, at: usize) -> bool {
        let node = self.nodes[at];
        let function = node.index;
        let Some(predicate) = (valid(node.value_type)
            && node.flags & (PRIMITIVE | SCRIPT_CALL) == 0
            && node.data != NONE)
            .then(|| self.get(node.data))
            .flatten()
        else {
            return true;
        };
        if usize::try_from(function)
            .ok()
            .and_then(|function| FUNCTIONS.get(function))
            .is_none_or(|&(_, returns)| returns != PASSTHROUGH)
        {
            return true;
        }
        // (a list linked in a loop ends where the array does)
        let arguments =
            std::iter::successors(self.next(predicate), |&argument| self.next(argument))
                .take(self.nodes.len());
        let of_type = |argument: usize| self.nodes[argument].value_type == node.value_type;
        match function {
            BEGIN => { arguments }.all(|argument| {
                let expected = if self.nodes[argument].next == NONE {
                    node.value_type
                } else {
                    VOID
                };
                self.nodes[argument].value_type == expected
            }),
            BEGIN_RANDOM => { arguments }.all(of_type),
            IF => {
                let mut arguments = arguments;
                arguments
                    .next()
                    .is_some_and(|condition| self.nodes[condition].value_type == BOOLEAN)
                    && arguments.all(of_type)
            }
            SET => {
                let Some(at) = self.next(predicate) else {
                    return false;
                };
                let variable = self.nodes[at];
                if variable.flags & PRIMITIVE == 0
                    || variable.flags & VARIABLE == 0
                    || !valid(variable.value_type)
                    || !can_cast(variable.value_type, node.value_type)
                {
                    return false;
                }
                let value = self.next(at);
                value.is_some_and(|value| {
                    self.nodes[value].value_type == variable.value_type
                        && self.nodes[value].next == NONE
                })
            }
            _ => { arguments }.all(of_type),
        }
    }
}

/// Runs the game's fix-up over a scenario's nodes (all of the array's, of
/// which the first `count` are in use) and its string constants: on success
/// the nodes hold what the game's do, and on failure the error says why the
/// game would not run the scripts.
pub fn postprocess(
    nodes: &mut [Node],
    count: usize,
    strings: &[u8],
    names: &Names,
) -> Result<(), &'static str> {
    let mut fixup = Fixup {
        count: count.min(nodes.len()),
        nodes,
        strings,
        source_size: strings.len() as i64 - DYNAMIC_SOURCE_BYTES as i64,
        names,
    };
    for at in 0..fixup.count {
        if fixup.nodes[at].salt != 0 {
            fixup.resolve(at)?;
        }
    }
    for at in 0..fixup.count {
        if fixup.nodes[at].salt != 0 && !fixup.passthrough(at) {
            return Err(
                "a value is not of the type it is returned as (you need to recompile scripts.)",
            );
        }
    }
    Ok(())
}
