//! The census as `opence census` runs it.

mod common;

use common::Tree;
use opence_census::{census, Mode};

type Failure = Box<dyn std::error::Error>;

const UNITS: &str = "/*
UNITS.C

symbols in this file:
00010000 0040:
\t_main_loop (0000)
00010040 0020:
\t_unit_kill (0000)
00010060 0010:
\t_unit_orphan (0000)
*/

void main_loop(void)
{
\tunit_kill();
}

void unit_kill(void)
{
}

void unit_orphan(void)
{
}

// port: the old port's
void port_only(void)
{
}
";

const SIM: &str = "//! The units.\n\n/// Does `unit_kill`'s job.\npub fn kill() {}\n";

fn tree(name: &str) -> Tree {
    Tree::new(name)
        .with("source/units/units.c", UNITS)
        .with("engine/crates/sim/src/units.rs", SIM)
}

#[test]
fn the_summary_counts_the_original_and_the_engine() -> Result<(), Failure> {
    let tree = tree("summary");
    let summary = census(tree.root(), Mode::Summary)?;
    assert!(summary.contains("3 functions"), "{summary}");
    assert!(summary.contains("112 bytes"), "{summary}");
    assert!(summary.contains("1 added by the old port"), "{summary}");
    assert!(summary.contains("1 cited"), "{summary}");
    assert!(summary.contains("2 reached from main_loop"), "{summary}");
    Ok(())
}

#[test]
fn a_tree_without_source_is_refused() {
    let tree = Tree::new("nosource").with("engine/crates/sim/src/units.rs", SIM);
    assert!(census(tree.root(), Mode::Check).is_err());
}

#[test]
fn the_real_tree_passes_its_check() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        return Ok(());
    };
    census(&root, Mode::Check)?;
    Ok(())
}
