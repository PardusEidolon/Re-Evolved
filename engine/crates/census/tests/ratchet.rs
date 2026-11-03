//! The ratchet (`engine/census/ratchet.tsv`): each system's Missing,
//! Partial and owed counts, which `--check` lets fall but never rise.

mod common;

use common::Tree;
use opence_census::{census, ratchet, Error, Mode};

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
";

const MANIFEST: &str = "[[rule]]
class = \"missing\"
";

fn tree(name: &str, ratchet: &str) -> Tree {
    Tree::new(name)
        .with("source/units/units.c", UNITS)
        .with("engine/census/census.toml", MANIFEST)
        .with("engine/census/ratchet.tsv", ratchet)
}

fn problems(result: Result<String, Error>) -> Vec<String> {
    match result {
        Err(Error::Problems(problems)) => problems,
        _ => Vec::new(),
    }
}

#[test]
fn a_count_that_rises_fails_the_check() {
    let tree = tree("rises", "system\tmissing\tpartial\towed\nunits\t2\t0\t0\n");
    let found = problems(census(tree.root(), Mode::Check));
    assert!(
        found
            .iter()
            .any(|p| p.contains("units") && p.contains("Missing rose from 2 to 3")),
        "{found:?}"
    );
}

#[test]
fn a_count_that_falls_asks_for_the_ratchet() {
    let tree = tree("falls", "system\tmissing\tpartial\towed\nunits\t4\t0\t0\n");
    let found = problems(census(tree.root(), Mode::Check));
    assert!(
        found
            .iter()
            .any(|p| p.contains("units") && p.contains("--ratchet")),
        "{found:?}"
    );
}

#[test]
fn a_system_the_ratchet_does_not_name_starts_at_zero() {
    let tree = tree("unnamed", "system\tmissing\tpartial\towed\n");
    let found = problems(census(tree.root(), Mode::Check));
    assert!(
        found.iter().any(|p| p.contains("Missing rose from 0 to 3")),
        "{found:?}"
    );
}

#[test]
fn the_ratchet_records_the_counts_and_then_the_check_passes() -> Result<(), Failure> {
    let tree = tree(
        "records",
        "system\tmissing\tpartial\towed\nunits\t4\t0\t0\n",
    );
    census(tree.root(), Mode::Ratchet)?;
    let written = std::fs::read_to_string(tree.root().join("engine/census/ratchet.tsv"))?;
    assert_eq!(written, "system\tmissing\tpartial\towed\nunits\t3\t0\t0\n");
    // (the fake tree has no docs, so only the ratchet's problems count)
    let found = problems(census(tree.root(), Mode::Check));
    assert!(
        !found
            .iter()
            .any(|p| p.contains("rose") || p.contains("fell")),
        "{found:?}"
    );
    Ok(())
}

#[test]
fn the_ratchet_refuses_to_record_a_rise() {
    let tree = tree(
        "refuses",
        "system\tmissing\tpartial\towed\nunits\t1\t0\t0\n",
    );
    assert!(census(tree.root(), Mode::Ratchet).is_err());
}

#[test]
fn the_first_ratchet_records_what_there_is() -> Result<(), Failure> {
    let tree = Tree::new("first")
        .with("source/units/units.c", UNITS)
        .with("engine/census/census.toml", MANIFEST);
    census(tree.root(), Mode::Ratchet)?;
    let written = std::fs::read_to_string(tree.root().join("engine/census/ratchet.tsv"))?;
    assert_eq!(written, "system\tmissing\tpartial\towed\nunits\t3\t0\t0\n");
    Ok(())
}

#[test]
fn a_malformed_ratchet_is_refused() {
    assert!(ratchet::read_ratchet("system\tmissing\tpartial\towed\nunits\tmany\t0\t0\n").is_err());
}

#[test]
fn the_real_tree_has_a_ratchet() {
    let Some(root) = common::repository() else {
        return;
    };
    assert!(root.join("engine").join(ratchet::RATCHET).is_file());
}

#[test]
fn a_function_moving_from_missing_to_owed_is_a_fall() {
    // (one of the three Missing functions is now Matched and owes its check)
    let mut was = ratchet::Ratchet::new();
    was.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 3,
            partial: 0,
            owed: 0,
        },
    );
    let mut now = ratchet::Ratchet::new();
    now.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 2,
            partial: 0,
            owed: 1,
        },
    );
    let (rose, fell) = ratchet::compare(&was, &now);
    assert!(rose.is_empty(), "{rose:?}");
    assert!(!fell.is_empty());
}

#[test]
fn a_function_moving_from_missing_to_an_owing_partial_is_a_fall() {
    let mut was = ratchet::Ratchet::new();
    was.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 3,
            partial: 0,
            owed: 0,
        },
    );
    let mut now = ratchet::Ratchet::new();
    now.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 2,
            partial: 1,
            owed: 1,
        },
    );
    let (rose, _) = ratchet::compare(&was, &now);
    assert!(rose.is_empty(), "{rose:?}");
}

#[test]
fn owed_that_rises_more_than_missing_fell_is_a_rise() {
    let mut was = ratchet::Ratchet::new();
    was.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 3,
            partial: 1,
            owed: 0,
        },
    );
    let mut now = ratchet::Ratchet::new();
    now.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 2,
            partial: 1,
            owed: 2,
        },
    );
    let (rose, _) = ratchet::compare(&was, &now);
    assert!(rose.iter().any(|r| r.contains("owed rose")), "{rose:?}");
}

#[test]
fn partial_that_rises_without_missing_falling_is_a_rise() {
    let mut was = ratchet::Ratchet::new();
    was.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 3,
            partial: 1,
            owed: 0,
        },
    );
    let mut now = ratchet::Ratchet::new();
    now.insert(
        "units".to_owned(),
        ratchet::Counts {
            missing: 3,
            partial: 2,
            owed: 0,
        },
    );
    let (rose, _) = ratchet::compare(&was, &now);
    assert!(rose.iter().any(|r| r.contains("Partial rose")), "{rose:?}");
}

#[test]
fn an_owed_function_its_check_finds_partial_is_no_rise() -> Result<(), Failure> {
    let header = "system\tmissing\tpartial\towed\n";
    let was = ratchet::read_ratchet(&format!("{header}units\t3\t0\t3\n"))?;
    // (two owed functions given their checks, which find them short)
    let checked = ratchet::read_ratchet(&format!("{header}units\t3\t2\t1\n"))?;
    let (rose, _) = ratchet::compare(&was, &checked);
    assert!(rose.is_empty(), "{rose:?}");
    // (two found short with no check given them)
    let unchecked = ratchet::read_ratchet(&format!("{header}units\t3\t2\t3\n"))?;
    let (rose, _) = ratchet::compare(&was, &unchecked);
    assert!(
        rose.iter().any(|r| r.contains("Partial rose from 0 to 2")),
        "{rose:?}"
    );
    Ok(())
}

#[test]
fn an_owed_function_its_check_finds_a_stub_is_no_rise() -> Result<(), Failure> {
    let header = "system\tmissing\tpartial\towed\n";
    let was = ratchet::read_ratchet(&format!("{header}units\t3\t0\t3\n"))?;
    // (an owed function found to be a stub, and so Missing)
    let found = ratchet::read_ratchet(&format!("{header}units\t4\t0\t2\n"))?;
    let (rose, _) = ratchet::compare(&was, &found);
    assert!(rose.is_empty(), "{rose:?}");
    // (a Missing function that came from nowhere)
    let new = ratchet::read_ratchet(&format!("{header}units\t4\t0\t3\n"))?;
    let (rose, _) = ratchet::compare(&was, &new);
    assert!(
        rose.iter().any(|r| r.contains("Missing rose from 3 to 4")),
        "{rose:?}"
    );
    // (one stub and one short, with only one owed function checked)
    let both = ratchet::read_ratchet(&format!("{header}units\t4\t1\t2\n"))?;
    let (rose, _) = ratchet::compare(&was, &both);
    assert!(!rose.is_empty(), "{rose:?}");
    Ok(())
}
