//! Every stock scenario's scripts import: their nodes, scripts and
//! globals, each node's links within the list.

use std::error::Error;
use std::path::Path;

use opence_content::{NodeKind, ScriptKind, Scripts};
use opence_import::script::fixed_up_nodes;
use opence_import::{import_scripts, Cache};

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

const MAPS: [&str; 24] = [
    "a10",
    "a30",
    "a50",
    "b30",
    "b40",
    "c10",
    "c20",
    "c40",
    "d20",
    "d40",
    "beavercreek",
    "bloodgulch",
    "boardingaction",
    "carousel",
    "chillout",
    "damnation",
    "hangemhigh",
    "longest",
    "prisoner",
    "putput",
    "ratrace",
    "sidewinder",
    "wizard",
    "ui",
];

fn scripts(map: &str) -> Result<Option<Scripts>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../assets/maps")
        .join(format!("{map}.map"));
    let Ok(file) = std::fs::read(&path) else {
        return Ok(None); // (no game data here)
    };
    Ok(Some(import_scripts(&Cache::open(file)?)?))
}

/// Whether every link of the nodes is to a node, and every script call to
/// a script.
fn linked(scripts: &Scripts) -> bool {
    let nodes = scripts.nodes.len();
    let node = |index: u32| (index as usize) < nodes;
    scripts.scripts.iter().all(|script| node(script.root))
        && scripts
            .globals
            .iter()
            .all(|global| global.initial.is_none_or(node))
        && scripts.nodes.iter().all(|n| match &n.kind {
            NodeKind::Call(call) => call.arguments.iter().all(|&a| node(a)),
            NodeKind::ScriptCall(script) => (*script as usize) < scripts.scripts.len(),
            _ => true,
        })
}

#[test]
fn every_scenarios_scripts_import() -> Result {
    for map in MAPS {
        let Some(scripts) = scripts(map)? else {
            continue;
        };
        assert_eq!(scripts.disabled, None, "{map}'s scripts run");
        assert!(linked(&scripts), "{map}'s links");
        eprintln!(
            "{map}: {} scripts, {} globals, {} nodes",
            scripts.scripts.len(),
            scripts.globals.len(),
            scripts.nodes.len()
        );
    }
    Ok(())
}

#[test]
fn the_pillar_of_autumns_scripts_are_its_own() -> Result {
    let Some(scripts) = scripts("a10")? else {
        return Ok(());
    };
    assert!(scripts.scripts.len() > 20);
    assert!(scripts
        .scripts
        .iter()
        .any(|script| script.kind == ScriptKind::Startup));
    assert!(scripts
        .scripts
        .iter()
        .any(|script| script.kind == ScriptKind::Dormant));
    // (every call names a function, as written)
    assert!(scripts.nodes.iter().all(|node| match &node.kind {
        NodeKind::Call(call) => !call.function.is_empty(),
        _ => true,
    }));
    Ok(())
}

/// The tool fixed up each map's nodes as the game does as it builds it, so
/// the game's fix-up of them changes nothing.
#[test]
fn every_scenarios_stored_nodes_are_the_fix_ups() -> Result {
    for map in MAPS {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/maps")
            .join(format!("{map}.map"));
        let Ok(file) = std::fs::read(&path) else {
            continue; // (no game data here)
        };
        let (stored, fixed) = fixed_up_nodes(&Cache::open(file)?)?;
        let differ: Vec<_> = stored
            .iter()
            .zip(&fixed)
            .enumerate()
            .filter(|(_, (stored, fixed))| stored != fixed)
            .collect();
        for (index, (stored, fixed)) in differ.iter().take(8) {
            eprintln!("{map} node {index}: stored {stored:?}, fixed {fixed:?}");
        }
        assert!(differ.is_empty(), "{map}: {} nodes differ", differ.len());
    }
    Ok(())
}
