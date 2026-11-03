//! The layout's sizes against the user's maps: the cache lays each tag's
//! definition out and its blocks and data after it, before the next
//! tag's, so no tag of a group takes less than its structure's size, and
//! one with no blocks or data takes exactly that (skipped where there is
//! no game data).

mod common;

use std::collections::BTreeMap;

use opence_census::layout::{read_layout, LAYOUT};
use opence_import::Cache;

type Failure = Box<dyn std::error::Error>;

const MAPS: [&str; 11] = [
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
    "bloodgulch",
];

/// Each group, its structure, and whether the structure has blocks or
/// data (so that only the least a tag takes is bounded by its size).
const GROUPS: [(&str, &str, bool); 9] = [
    ("senv", "shader_environment_definition", false),
    ("soso", "shader_model_definition", false),
    ("sgla", "shader_transparent_glass_definition", false),
    ("smet", "shader_transparent_meter_definition", false),
    ("spla", "shader_transparent_plasma_definition", false),
    ("schi", "shader_transparent_chicago_definition", true),
    ("sotr", "shader_transparent_generic_definition", true),
    ("swat", "shader_transparent_water_definition", true),
    ("vehi", "vehicle_definition", true),
];

#[test]
fn no_tag_on_the_maps_is_smaller_than_its_laid_out_structure() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        return Ok(());
    };
    let records = read_layout(&std::fs::read_to_string(root.join("engine").join(LAYOUT))?)?;
    // each group's least footprint: the bytes from a tag to the next
    let mut least: BTreeMap<String, u32> = BTreeMap::new();
    for map in MAPS {
        let Ok(bytes) = std::fs::read(root.join("assets/maps").join(format!("{map}.map"))) else {
            continue; // (no game data here)
        };
        let cache = Cache::open(bytes)?;
        let mut tags: Vec<_> = cache
            .tags()
            .iter()
            .filter(|tag| !tag.in_resource_map && tag.address != 0)
            .collect();
        tags.sort_by_key(|tag| tag.address);
        for pair in tags.windows(2) {
            let gap = pair[1].address.saturating_sub(pair[0].address);
            least
                .entry(pair[0].group.to_string())
                .and_modify(|at| *at = (*at).min(gap))
                .or_insert(gap);
        }
    }
    for (group, structure, has_children) in GROUPS {
        let Some(footprint) = least.get(group) else {
            continue;
        };
        let size = records
            .iter()
            .find(|record| record.name == structure)
            .map(|record| record.size);
        if has_children {
            assert!(
                size.is_some_and(|size| size <= *footprint),
                "{group}: {size:?} > {footprint}"
            );
        } else {
            assert_eq!(size, Some(*footprint), "{group}");
        }
    }
    Ok(())
}
