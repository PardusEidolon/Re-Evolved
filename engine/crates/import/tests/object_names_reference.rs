//! Each object name is given the game's placement:
//! tools/engine_object_names_reference.py runs object_types.c's
//! `object_names_postprocess` over 200 made-up scenarios, recording the
//! kind and index of the placement each name is given, and
//! [`object_names`] must give the same. The importer reads what the tool
//! stored in each map, so the stock maps' names must be the rule's too
//! (skipped where there is no game data).

use std::path::Path;

use opence_content::object::ObjectKind;
use opence_import::level::object_names;
use opence_import::{Cache, GroupTag};

const REFERENCE: &str = include_str!("../../../../tools/traces/object_names_reference.txt");

type Failure = Box<dyn std::error::Error>;

/// A scenario of the trace: how many names, each kind's placements' name
/// indices, and the placement the game gives each name.
#[derive(Default)]
struct Scenario {
    names: usize,
    placements: Vec<(ObjectKind, Vec<i16>)>,
    given: Vec<Option<(ObjectKind, usize)>>,
}

fn kind(field: &str) -> Result<ObjectKind, Failure> {
    let index: usize = field.parse()?;
    Ok(*ObjectKind::ALL.get(index).ok_or("no such kind")?)
}

fn scenarios() -> Result<Vec<Scenario>, Failure> {
    let mut scenarios: Vec<Scenario> = Vec::new();
    for line in REFERENCE.lines().filter(|line| !line.starts_with('#')) {
        let fields: Vec<&str> = line.split(' ').collect();
        if let ["scenario", _, "names", names] = fields[..] {
            scenarios.push(Scenario {
                names: names.parse()?,
                ..Scenario::default()
            });
            continue;
        }
        let scenario = scenarios.last_mut().ok_or("a line before a scenario")?;
        match fields[..] {
            ["place", placement_kind, _, name] => {
                let placement_kind = kind(placement_kind)?;
                match scenario.placements.last_mut() {
                    Some((k, names)) if *k == placement_kind => names.push(name.parse()?),
                    _ => scenario
                        .placements
                        .push((placement_kind, vec![name.parse()?])),
                }
            }
            ["name", _, "-1", "-1"] => scenario.given.push(None),
            ["name", _, given_kind, index] => {
                scenario
                    .given
                    .push(Some((kind(given_kind)?, index.parse()?)));
            }
            _ => return Err(format!("an unknown line: {line}").into()),
        }
    }
    Ok(scenarios)
}

#[test]
fn each_name_is_given_the_placement_the_game_gives_it() -> Result<(), Failure> {
    let scenarios = scenarios()?;
    assert_eq!(scenarios.len(), 200);
    for (trial, scenario) in scenarios.iter().enumerate() {
        // (the importer's kinds come in its own order, not the game's)
        let mut placements = scenario.placements.clone();
        placements.reverse();
        assert_eq!(
            object_names(scenario.names, &placements),
            scenario.given,
            "scenario {trial}"
        );
    }
    Ok(())
}

/// The scenario's object names, and its placement blocks: the kind, where
/// the block is and the size of its elements.
const OBJECT_NAMES: (usize, usize) = (0x204, 0x24);
const PLACEMENTS: [(ObjectKind, usize, usize); 9] = [
    (ObjectKind::Scenery, 0x210, 0x48),
    (ObjectKind::Biped, 0x228, 0x78),
    (ObjectKind::Vehicle, 0x240, 0x78),
    (ObjectKind::Equipment, 0x258, 0x28),
    (ObjectKind::Weapon, 0x270, 0x5C),
    (ObjectKind::Machine, 0x294, 0x40),
    (ObjectKind::Control, 0x2AC, 0x40),
    (ObjectKind::LightFixture, 0x2C4, 0x58),
    (ObjectKind::SoundScenery, 0x2DC, 0x28),
];
const MAPS: [&str; 10] = [
    "a10", "a30", "a50", "b30", "b40", "c10", "c20", "c40", "d20", "d40",
];

#[test]
fn the_stock_maps_store_each_name_the_placement_the_rule_gives_it() -> Result<(), Failure> {
    let mut checked = 0;
    for map in MAPS {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../assets/maps")
            .join(format!("{map}.map"));
        let Ok(bytes) = std::fs::read(&path) else {
            continue; // (no game data here)
        };
        let cache = Cache::open(bytes)?;
        let scenario = cache.tag(cache.scenario_index(), GroupTag::new(b"scnr"), 0x5B0)?;
        let names = scenario.block(OBJECT_NAMES.0, "object names", OBJECT_NAMES.1, 1024)?;
        let mut placements = Vec::new();
        for (kind, at, size) in PLACEMENTS {
            let block = scenario.block(at, "placements", size, 2048)?;
            let indices = block
                .iter()
                .map(|placement| placement.i16(2, "name index"))
                .collect::<Result<Vec<_>, _>>()?;
            placements.push((kind, indices));
        }
        let rule = object_names(names.len(), &placements);
        for (index, (name, given)) in names.iter().zip(&rule).enumerate() {
            // (a name no placement names was left as the tool zeroed it)
            let Some((kind, placement)) = given else {
                continue;
            };
            let stored = (name.i16(0x20, "type")?, name.i16(0x22, "placement")?);
            let kind = ObjectKind::ALL.iter().position(|k| k == kind);
            let expected = (
                i16::try_from(kind.ok_or("no such kind")?)?,
                i16::try_from(*placement)?,
            );
            assert_eq!(stored, expected, "{map} name {index}");
            checked += 1;
        }
    }
    println!("{checked} stored names are the rule's");
    Ok(())
}
