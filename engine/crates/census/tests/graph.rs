//! The original's definitions and call graph.

mod common;

use common::Tree;
use opence_census::c::{definitions, Kind};
use opence_census::original::Original;

type Failure = Box<dyn std::error::Error>;

const UNITS: &str = r#"/*
UNITS.C

symbols in this file:
00010000 0040:
	_units_update (0000)
00010040 0040:
	_unit_think (0000)
*/

#include "units.h"
#define UNIT_KILL(u) unit_kill(u, "{")

static struct unit_globals { int count; } unit_globals;

// unit_ghost() { is in a comment
static void unit_helper(int x)
{
	if (x) { unit_kill(x, '}'); }
}

void unit_think(int x)
{
	unit_helper(x);
}

void units_update(void)
{
	int i;
	for (i = 0; i < 4; i++) { unit_types[i].think(i); }
	UNIT_KILL(0);
}

static struct unit_type unit_types[] = {
	{ "biped", unit_think },
	{ "vehicle", unit_think },
};

void unit_kill(int x, char *why)
{
}

void unit_orphan(void)
{
	unit_kill(0, "");
}
"#;

#[test]
fn definitions_are_found_at_the_top_level() {
    let found = definitions(UNITS);
    let summary: Vec<_> = found
        .iter()
        .map(|definition| (definition.name.as_str(), definition.kind))
        .collect();
    assert_eq!(
        summary,
        [
            ("UNIT_KILL", Kind::Macro),
            ("unit_helper", Kind::Function),
            ("unit_think", Kind::Function),
            ("units_update", Kind::Function),
            ("unit_types", Kind::Data),
            ("unit_kill", Kind::Function),
            ("unit_orphan", Kind::Function),
        ]
    );
    assert_eq!(found[1].line, 17);
    assert!(found[4].refers_to.contains(&"unit_think".to_owned()));
}

#[test]
fn reach_follows_calls_tables_and_macros() -> Result<(), Failure> {
    let tree = Tree::new("graph").with("units/units.c", UNITS);
    let original = Original::read(tree.root())?;
    let names: Vec<_> = original
        .functions
        .iter()
        .map(|f| (f.name.as_str(), f.size))
        .collect();
    // (the headers' two, then the three the C defines with no symbol)
    assert_eq!(
        names,
        [
            ("units_update", 0x40),
            ("unit_think", 0x40),
            ("unit_helper", 0),
            ("unit_kill", 0),
            ("unit_orphan", 0)
        ]
    );
    let reached = original.graph.reach(&["units_update", "nowhere"], |_| true);
    assert_eq!(
        reached.iter().map(String::as_str).collect::<Vec<_>>(),
        [
            "UNIT_KILL",
            "unit_helper",
            "unit_kill",
            "unit_think",
            "unit_types",
            "units_update"
        ]
    );
    // (walking on only through what is not `unit_think`: its callees are
    // reached only through it)
    let reached = original
        .graph
        .reach(&["units_update"], |name| name != "unit_think");
    assert!(reached.contains("unit_think"));
    assert!(!reached.contains("unit_helper"));
    assert!(reached.contains("unit_kill"));
    Ok(())
}

#[test]
fn functions_the_old_port_added_are_set_apart() -> Result<(), Failure> {
    let tree = Tree::new("graph-port").with(
        "hud/hud.c",
        "int hud_flags(void)\n{\n\treturn 0;\n}\n\n// port: a setter for the hosted shell\nvoid hud_set_flags(int x)\n{\n}\n\nvoid hud_port_reset(void)\n{\n}\n",
    );
    let original = Original::read(tree.root())?;
    let names = |functions: &[opence_census::symbols::Function]| -> Vec<String> {
        functions.iter().map(|f| f.name.clone()).collect()
    };
    assert_eq!(names(&original.functions), ["hud_flags"]);
    assert_eq!(names(&original.added), ["hud_set_flags", "hud_port_reset"]);
    Ok(())
}

#[test]
fn the_real_tree_is_reached_from_its_main_loop() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let original = Original::read(&root.join("source"))?;
    let reached = original.graph.reach(&["main_loop"], |_| true);
    let functions = original
        .functions
        .iter()
        .filter(|f| reached.contains(&f.name))
        .count();
    // (most of the original is reached from its loop; the rest is what
    // nothing calls in the C the census reads, such as debug commands)
    assert!(
        functions * 10 > original.functions.len() * 6,
        "{functions} reached"
    );
    Ok(())
}
