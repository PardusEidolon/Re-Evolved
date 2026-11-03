//! The names of the functions the symbol headers know only by address
//! (`code_000ad290`): each is the file's own definition that has no symbol,
//! taken in order, or, where the file defines none, its place.

mod common;

use common::Tree;
use opence_census::c::{definitions, Kind};
use opence_census::original::{stale, write_names, Evidence, Original, NAMES};
use opence_census::Census;

type Failure = Box<dyn std::error::Error>;

/// A file whose header names one function and knows three by address,
/// and whose C defines those three, an inline one, and nothing else.
const PAIRED: &str = "/*
PAIRED.C

symbols in this file:
00010000 0040:
\t_paired_update (0000)
00010040 0020:
\t_code_00010040 (0000)
00010060 0080:
\t_code_00010060 (0000)
000100e0 0040:
\t_code_000100e0 (0000)
*/

void paired_update(void)
{
\tsmall();
}

static void small(void)
{
\ta();
}

static void large(void)
{
\ta(); b(); c(); d(); e(); f(); g(); h();
}

static __inline int tiny(void)
{
\treturn 0;
}

static void middle(void)
{
\ta(); b(); c();
}
";

/// A file whose header knows a function by address that its C does not
/// define.
const UNDEFINED: &str = "/*
UNDEFINED.C

symbols in this file:
00020000 0040:
\t_undefined_update (0000)
00020040 0020:
\t_code_00020040 (0000)
*/

void undefined_update(void)
{
}
";

fn tree(name: &str) -> Tree {
    Tree::new(name)
        .with("source/units/units.c", "")
        .with("source/game/paired.c", PAIRED)
        .with("source/game/undefined.c", UNDEFINED)
        .with(
            "engine/census/census.toml",
            "[[rule]]\nclass = \"missing\"\n",
        )
}

#[test]
fn an_address_only_symbol_is_the_files_definition_in_order() -> Result<(), Failure> {
    let tree = tree("paired");
    let original = Original::read(&tree.root().join("source"))?;
    let size = |name: &str| {
        original
            .functions
            .iter()
            .find(|f| f.name == name)
            .map(|f| (f.address, f.size))
    };
    assert_eq!(size("small"), Some((0x10040, 0x20)));
    assert_eq!(size("large"), Some((0x10060, 0x80)));
    assert_eq!(size("middle"), Some((0x100e0, 0x40)));
    // (an inline function has no symbol of its own)
    assert_eq!(size("tiny"), Some((0, 0)));
    assert!(
        !original
            .functions
            .iter()
            .any(|f| f.name.starts_with("code_") && f.file == "game/paired.c"),
        "{:?}",
        original.functions
    );
    let small = original.names_by_address.get("code_00010040");
    assert_eq!(small.map(|p| p.name.as_str()), Some("small"));
    Ok(())
}

#[test]
fn sizes_that_rise_with_the_bodies_make_the_order_likelier() -> Result<(), Failure> {
    let tree = tree("evidence");
    let original = Original::read(&tree.root().join("source"))?;
    let evidence = original
        .names_by_address
        .get("code_00010060")
        .map(|p| p.evidence);
    assert_eq!(evidence, Some(Evidence::OrderAndSize));
    Ok(())
}

#[test]
fn a_symbol_the_c_does_not_define_is_known_by_its_place() -> Result<(), Failure> {
    let tree = tree("place");
    let original = Original::read(&tree.root().join("source"))?;
    let place = original.names_by_address.get("code_00020040");
    assert_eq!(place.map(|p| p.evidence), Some(Evidence::Place));
    assert_eq!(
        place.map(|p| p.name.as_str()),
        Some("after undefined_update")
    );
    Ok(())
}

#[test]
fn the_names_table_is_written_and_checked() -> Result<(), Failure> {
    let tree = tree("table");
    let original = Original::read(&tree.root().join("source"))?;
    let table = write_names(&original.names_by_address);
    assert!(
        table.contains("code_00010040\tgame/paired.c\tsmall\torder\n")
            || table.contains("code_00010040\tgame/paired.c\tsmall\torder and size\n"),
        "{table}"
    );
    let path = tree.root().join("engine").join(NAMES);
    std::fs::write(&path, &table)?;
    assert!(stale(tree.root())?.is_empty());
    std::fs::write(&path, "address\n")?;
    let found = stale(tree.root())?;
    assert!(found.iter().any(|p| p.contains("names.tsv")), "{found:?}");
    Ok(())
}

#[test]
fn an_evaluator_macro_defines_its_evaluator() {
    let found = definitions(
        "HS_EVALUATE_NO_ARGUMENTS(game_all_quiet_evaluate, game_all_quiet)\n\
         HS_EVALUATE_NO_OP(radiosity_hack_start_evaluate)\n\
         void after(void)\n{\n}\n",
    );
    let functions: Vec<&str> = found
        .iter()
        .filter(|d| d.kind == Kind::Function)
        .map(|d| d.name.as_str())
        .collect();
    assert_eq!(
        functions,
        [
            "game_all_quiet_evaluate",
            "radiosity_hack_start_evaluate",
            "after"
        ]
    );
}

#[test]
fn no_function_of_the_real_tree_is_known_only_by_its_address() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let census = Census::read(&root)?;
    let original = Original::read(&root.join("source"))?;
    let unnamed: Vec<&str> = census
        .functions
        .iter()
        .filter(|f| f.name.starts_with("code_") && !original.names_by_address.contains_key(&f.name))
        .map(|f| f.name.as_str())
        .collect();
    assert!(unnamed.is_empty(), "{} unnamed: {unnamed:?}", unnamed.len());
    Ok(())
}
