//! The original's functions, from the symbol headers of `source/`.

mod common;

use common::Tree;
use opence_census::symbols::{c_name, functions, parse_header, Symbol};
use opence_census::Error;

type Failure = Box<dyn std::error::Error>;

const UNITS: &str = "/*\nUNITS.C\n\nsymbols in this file:\n001977E0 0040:\n\t_units_initialize (0000)\n00197820 0020:\n\t_units_initialize_for_new_map (0000)\n002A4DCC 0006:\n\t??_C@_0CH@KNBNIKNG@unit?9?$DOobject@ (0000)\n00315C08 061c:\n\t_magic_base_animation_seat_index (0000)\n\t_base_seat_labels (0604)\n*/\n\nvoid units_initialize(void)\n{\n}\n";

#[test]
fn a_header_is_parsed_into_addresses_sizes_and_names() -> Result<(), Failure> {
    let symbols = parse_header("units/units.c", UNITS)?;
    assert_eq!(symbols.len(), 4);
    assert_eq!(
        symbols[0],
        Symbol {
            address: 0x0019_77E0,
            size: 0x40,
            names: vec!["_units_initialize".into()],
        }
    );
    assert_eq!(
        symbols[3].names,
        ["_magic_base_animation_seat_index", "_base_seat_labels"]
    );
    Ok(())
}

#[test]
fn a_name_on_the_address_line_is_read_too() -> Result<(), Failure> {
    let text = "/*\nACTION_UNCOVER.C\n\nsymbols in this file:\n00008690 0080: _action_uncover_setup_target\n*/\n";
    let symbols = parse_header("ai/action_uncover.c", text)?;
    assert_eq!(symbols[0].names, ["_action_uncover_setup_target"]);
    assert_eq!(symbols[0].size, 0x80);
    Ok(())
}

#[test]
fn files_without_symbols_have_none() -> Result<(), Failure> {
    assert!(parse_header("a.c", "/*\nA.C\n\nno symbols in this file.\n*/\n")?.is_empty());
    assert!(parse_header("b.c", "#include \"cseries.h\"\n")?.is_empty());
    Ok(())
}

#[test]
fn malformed_headers_are_refused() {
    for text in [
        "/*\nsymbols in this file:\n\t_orphan (0000)\n*/",
        "/*\nsymbols in this file:\n0019ZZE0 0040:\n\t_x (0000)\n*/",
        "/*\nsymbols in this file:\n001977E0 0040\n\t_x (0000)\n*/",
        "/*\nsymbols in this file:\n001977E0 0040:\n\t_x 0000\n*/",
        "/*\nsymbols in this file:\n001977E0 0040:\n*/",
        "/*\nsymbols in this file:\n1977E0 0040:\n\t_x (0000)\n*/",
        "/*\nsymbols in this file:\nunits are here\n*/",
    ] {
        assert!(
            matches!(parse_header("x.c", text), Err(Error::Malformed { .. })),
            "{text:?} was not refused"
        );
    }
}

#[test]
fn names_lose_the_compilers_underscore_and_stdcall_suffix() {
    assert_eq!(c_name("_unit_kill"), "unit_kill");
    assert_eq!(c_name("__rasterizer_frame_end"), "_rasterizer_frame_end");
    assert_eq!(
        c_name("_connect_async_thread_proc@4"),
        "connect_async_thread_proc"
    );
    assert_eq!(c_name("_odd@name"), "odd@name");
}

#[test]
fn the_code_is_what_lies_below_the_first_constant() -> Result<(), Failure> {
    let tree = Tree::new("symbols-code")
        .with("units/units.c", UNITS)
        .with(
            "ai/ai.c",
            "/*\nAI.C\n\nsymbols in this file:\n00010000 0100:\n\t_ai_initialize (0000)\n00243000 0004:\n\t__real@3f800000 (0000)\n*/\n",
        )
        .with("ai/notes.txt", "001977E0 0040:\n");
    let found = functions(tree.root())?;
    let names: Vec<_> = found
        .iter()
        .map(|function| function.name.as_str())
        .collect();
    assert_eq!(
        names,
        [
            "ai_initialize",
            "units_initialize",
            "units_initialize_for_new_map"
        ]
    );
    assert_eq!(found[1].file, "units/units.c");
    assert_eq!(found[1].directory(), "units");
    assert_eq!(
        found.iter().map(|function| function.size).sum::<u32>(),
        0x160
    );
    Ok(())
}

#[test]
fn the_real_tree_has_every_function_of_the_xbe() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let found = functions(&root.join("source"))?;
    // (6,747 blocks of code below the first constant, at 0x242EEC, in
    // 1,611,122 bytes; the 6,962 counted before the census also counted
    // the data the headers list)
    assert_eq!(found.len(), 6_747);
    assert_eq!(
        found
            .iter()
            .map(|function| u64::from(function.size))
            .sum::<u64>(),
        1_611_122
    );
    Ok(())
}
