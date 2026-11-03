//! The classes and the evidence each asks for (`engine/census/census.toml`).

mod common;

use common::Tree;
use opence_census::classes::{Class, Manifest};
use opence_census::{census, Census, Error, Mode};

type Failure = Box<dyn std::error::Error>;

const UNITS: &str = "/*
UNITS.C

symbols in this file:
00010000 0040:
\t_main_loop (0000)
00010040 0020:
\t_unit_kill (0000)
00010060 0010:
\t_unit_think (0000)
00010070 0010:
\t_unit_orphan (0000)
*/

void main_loop(void)
{
\tunit_kill();
\tunit_think();
\trasterizer_frame();
}

void unit_kill(void)
{
}

void unit_think(void)
{
\tunit_kill();
}

void unit_orphan(void)
{
}
";

const TEXT: &str = "/*
RASTERIZER_TEXT.C

symbols in this file:
00021000 0080:
\t_rasterizer_text_draw (0000)
*/

void rasterizer_text_draw(void)
{
}
";

const RASTERIZER: &str = "/*
RASTERIZER.C

symbols in this file:
00020000 0080:
\t_rasterizer_frame (0000)
00020080 0040:
\t_rasterizer_helper (0000)
*/

void rasterizer_frame(void)
{
\trasterizer_helper();
}

void rasterizer_helper(void)
{
}
";

const SIM: &str = "//! The units.\n\n/// Does `unit_kill`'s and `unit_think`'s jobs, with `libm::sinf`.\npub fn kill() { let _ = libm::sinf(0.0); }\n";

/// A manifest that classes every function of the fake tree with the
/// evidence it asks for.
const GOOD: &str = r#"
[[rule]]
class = "missing"

[[rule]]
directory = "rasterizer"
class = "replaced"
because = "renderer"
stands_in = "app/Sources/Render/Frame.swift"

[function.main_loop]
class = "matched"
engine = "sim/src/units.rs"
check = "sim/tests/loop_reference.rs"
against = "trace"

[function.unit_think]
class = "ported"
why = "its one call is unit_kill's"
through = "unit_kill"
"#;

fn tree(name: &str, manifest: &str) -> Tree {
    Tree::new(name)
        .with("source/units/units.c", UNITS)
        .with("source/rasterizer/rasterizer.c", RASTERIZER)
        .with(
            "engine/Cargo.toml",
            "[workspace.dependencies]\nlibm = \"0.2\"\n",
        )
        .with("engine/crates/sim/src/units.rs", SIM)
        .with("engine/crates/sim/tests/loop_reference.rs", "")
        .with("engine/crates/sim/tests/units.rs", "")
        .with("tools/traces/unit_reference.txt", "")
        .with("engine/app/Sources/Render/Frame.swift", "")
        .with("engine/census/census.toml", manifest)
        .with(
            "engine/census/traced.tsv",
            "unit_kill\tengine_unit_reference\n",
        )
}

/// The problems the check finds with `manifest`, with GOOD's lines
/// replaced or added.
fn problems(name: &str, manifest: &str) -> Result<Vec<String>, Failure> {
    let tree = tree(name, manifest);
    Ok(Census::read(tree.root())?.problems())
}

fn has(problems: &[String], text: &str) -> bool {
    problems.iter().any(|problem| problem.contains(text))
}

#[test]
fn a_good_manifest_has_no_problems() -> Result<(), Failure> {
    let found = problems("good", GOOD)?;
    assert!(found.is_empty(), "{found:#?}");
    Ok(())
}

#[test]
fn traced_functions_are_ported_and_the_rest_take_their_rules() -> Result<(), Failure> {
    let tree = tree("resolved", GOOD);
    let census = Census::read(tree.root())?;
    let class = |name: &str| census.function(name).and_then(|f| f.class);
    assert_eq!(class("unit_kill"), Some(Class::Ported));
    assert_eq!(class("unit_think"), Some(Class::Ported));
    assert_eq!(class("main_loop"), Some(Class::Matched));
    assert_eq!(class("rasterizer_frame"), Some(Class::Replaced));
    assert_eq!(class("unit_orphan"), Some(Class::Missing));
    Ok(())
}

#[test]
fn ported_without_a_trace_is_a_problem() -> Result<(), Failure> {
    let found = problems(
        "ported",
        &GOOD.replace("through = \"unit_kill\"", "through = \"unit_orphan\""),
    )?;
    assert!(has(&found, "unit_think"), "{found:#?}");
    assert!(has(&found, "unit_orphan is not traced"), "{found:#?}");
    let found = problems(
        "ported-why",
        &GOOD.replace("why = \"its one call is unit_kill's\"\n", ""),
    )?;
    assert!(has(&found, "unit_think"), "{found:#?}");
    Ok(())
}

#[test]
fn matched_with_only_an_engine_test_is_a_problem() -> Result<(), Failure> {
    let found = problems(
        "matched",
        &GOOD
            .replace("sim/tests/loop_reference.rs", "sim/tests/units.rs")
            .replace("against = \"trace\"\n", ""),
    )?;
    assert!(has(&found, "main_loop"), "{found:#?}");
    assert!(has(&found, "against the original"), "{found:#?}");
    // (a trace must be a reference trace)
    let found = problems(
        "matched-trace",
        &GOOD.replace("sim/tests/loop_reference.rs", "sim/tests/units.rs"),
    )?;
    assert!(has(&found, "not a reference trace"), "{found:#?}");
    // (and a check that is not there is no check)
    let found = problems(
        "matched-gone",
        &GOOD.replace("sim/tests/loop_reference.rs", "sim/tests/gone_reference.rs"),
    )?;
    assert!(
        has(&found, "sim/tests/gone_reference.rs does not exist"),
        "{found:#?}"
    );
    Ok(())
}

#[test]
fn native_without_a_primitive_is_a_problem() -> Result<(), Failure> {
    let native = "\n[function.unit_orphan]\nclass = \"native\"\n";
    let found = problems("native", &format!("{GOOD}{native}"))?;
    assert!(
        has(&found, "unit_orphan: Native names no primitive"),
        "{found:#?}"
    );
    let found = problems(
        "native-good",
        &format!("{GOOD}{native}primitive = \"libm::sinf\"\n"),
    )?;
    assert!(found.is_empty(), "{found:#?}");
    // (a primitive the engine does not use, or a crate it does not have)
    let found = problems(
        "native-unused",
        &format!("{GOOD}{native}primitive = \"libm::cosf\"\n"),
    )?;
    assert!(
        has(&found, "the engine does not use libm::cosf"),
        "{found:#?}"
    );
    let found = problems(
        "native-crate",
        &format!("{GOOD}{native}primitive = \"fastmath::sinf\"\n"),
    )?;
    assert!(has(&found, "fastmath is not a dependency"), "{found:#?}");
    // (and the containers the simulation iterates keep a fixed order)
    let found = problems(
        "native-hash",
        &format!("{GOOD}{native}primitive = \"std::collections::HashMap\"\n"),
    )?;
    assert!(has(&found, "fixed order"), "{found:#?}");
    Ok(())
}

#[test]
fn partial_with_nothing_left_out_is_a_problem() -> Result<(), Failure> {
    let partial = GOOD.replace("class = \"matched\"", "class = \"partial\"");
    let found = problems("partial", &partial)?;
    assert!(
        has(&found, "main_loop: Partial lists nothing it leaves out"),
        "{found:#?}"
    );
    let found = problems(
        "partial-good",
        &partial.replace(
            "against = \"trace\"",
            "against = \"trace\"\nleaves_out = [\"the pause menu\"]",
        ),
    )?;
    assert!(found.is_empty(), "{found:#?}");
    Ok(())
}

#[test]
fn replaced_without_a_listed_reason_is_a_problem() -> Result<(), Failure> {
    let found = problems("replaced", &GOOD.replace("because = \"renderer\"\n", ""))?;
    assert!(
        has(&found, "rasterizer_frame: Replaced gives no reason"),
        "{found:#?}"
    );
    // (a reason off the list is no manifest at all)
    let tree = tree(
        "replaced-list",
        &GOOD.replace("\"renderer\"", "\"too hard\""),
    );
    assert!(matches!(
        Census::read(tree.root()),
        Err(Error::Malformed { .. })
    ));
    let found = problems(
        "replaced-stand-in",
        &GOOD.replace(
            "app/Sources/Render/Frame.swift",
            "app/Sources/Render/Gone.swift",
        ),
    )?;
    assert!(
        has(&found, "app/Sources/Render/Gone.swift does not exist"),
        "{found:#?}"
    );
    Ok(())
}

#[test]
fn a_library_stands_in_and_says_what_the_player_sees() -> Result<(), Failure> {
    let library = GOOD.replace("\"renderer\"", "\"library\"");
    let found = problems("library", &library)?;
    assert!(
        has(
            &found,
            "rasterizer_frame: a library stands in but nothing says what the player sees"
        ),
        "{found:#?}"
    );
    let found = problems(
        "library-noted",
        &library.replace(
            "stands_in = \"app/Sources/Render/Frame.swift\"\n",
            "stands_in = \"app/Sources/Render/Frame.swift\"\nlives_on = \"the same frames\"\n",
        ),
    )?;
    assert!(found.is_empty(), "{found:#?}");
    Ok(())
}

#[test]
fn a_function_with_no_class_is_a_problem() -> Result<(), Failure> {
    let found = problems(
        "unclassed",
        &GOOD.replace("[[rule]]\nclass = \"missing\"\n", ""),
    )?;
    assert!(has(&found, "unit_orphan has no class"), "{found:#?}");
    Ok(())
}

#[test]
fn a_cited_function_classed_missing_is_a_problem() -> Result<(), Failure> {
    let found = problems(
        "cited",
        &GOOD.replace("[function.main_loop]", "[function.unused]"),
    )?;
    // (main_loop is not cited, so it falls to the rule; unit_think is, and
    // keeps its entry)
    assert!(!has(&found, "main_loop is cited"), "{found:#?}");
    let found = problems(
        "cited-missing",
        &GOOD.replace(
            "[function.unit_think]\nclass = \"ported\"",
            "[function.unit_think]\nclass = \"missing\"",
        ),
    )?;
    assert!(
        has(
            &found,
            "unit_think is cited (sim/src/units.rs:3) but Missing"
        ),
        "{found:#?}"
    );
    Ok(())
}

#[test]
fn a_reason_left_for_a_traced_function_is_stale() -> Result<(), Failure> {
    let stale =
        "\n[function.unit_kill]\nclass = \"ported\"\nwhy = \"no trace\"\nthrough = \"unit_kill\"\n";
    let found = problems("stale", &format!("{GOOD}{stale}"))?;
    assert!(
        has(&found, "unit_kill is traced by engine_unit_reference"),
        "{found:#?}"
    );
    Ok(())
}

#[test]
fn owed_evidence_is_accepted_and_listed_until_it_is_there() -> Result<(), Failure> {
    let owed = GOOD.replace(
        "check = \"sim/tests/loop_reference.rs\"\nagainst = \"trace\"",
        "owed = \"a trace of the frame's order\"",
    );
    let tree = tree("owed", &owed);
    let census = Census::read(tree.root())?;
    assert!(census.problems().is_empty(), "{:#?}", census.problems());
    assert_eq!(
        census.owed().map(|f| f.name.as_str()).collect::<Vec<_>>(),
        ["main_loop"]
    );
    // (an owed check that is there after all is stale)
    let found = problems(
        "owed-stale",
        &GOOD.replace(
            "against = \"trace\"",
            "against = \"trace\"\nowed = \"a trace\"",
        ),
    )?;
    assert!(has(&found, "main_loop owes nothing"), "{found:#?}");
    Ok(())
}

#[test]
fn what_only_replaced_code_reaches_is_replaced_through_its_caller() -> Result<(), Failure> {
    let tree = tree(
        "through",
        &GOOD.replace(
            "directory = \"rasterizer\"",
            "file = \"rasterizer/rasterizer.c\"\nprefix = \"rasterizer_frame\"",
        ),
    );
    let census = Census::read(tree.root())?;
    let helper = census.function("rasterizer_helper");
    assert_eq!(helper.and_then(|f| f.class), Some(Class::Replaced));
    assert_eq!(
        helper.and_then(|f| f.through_caller.as_deref()),
        Some("rasterizer_frame")
    );
    // (what nothing reaches stays Missing)
    let orphan = census.function("unit_orphan");
    assert_eq!(orphan.and_then(|f| f.class), Some(Class::Missing));
    assert_eq!(orphan.map(|f| f.reached), Some(false));
    Ok(())
}

#[test]
fn a_rule_can_keep_its_functions_from_their_replaced_callers() -> Result<(), Failure> {
    let tree = tree(
        "kept",
        &(GOOD.replace(
            "directory = \"rasterizer\"",
            "file = \"rasterizer/rasterizer.c\"\nprefix = \"rasterizer_frame\"",
        ) + "\n[[rule]]\nfile = \"rasterizer/rasterizer.c\"\nprefix = \"rasterizer_helper\"\n\
             class = \"missing\"\nthrough_callers = false\n"),
    );
    let census = Census::read(tree.root())?;
    let helper = census.function("rasterizer_helper");
    assert_eq!(helper.and_then(|f| f.class), Some(Class::Missing));
    assert_eq!(helper.and_then(|f| f.through_caller.as_deref()), None);
    Ok(())
}

#[test]
fn a_rule_can_keep_its_class_over_an_oracles_trace() -> Result<(), Failure> {
    // (a trace that compiles the original only to check a library against
    // it does not make the original's functions Ported)
    let rule = "\n[[rule]]\nfile = \"units/units.c\"\nprefix = \"unit_kill\"\n\
                class = \"replaced\"\nbecause = \"renderer\"\n\
                stands_in = \"app/Sources/Render/Frame.swift\"\n";
    let kept = tree("oracle", &(GOOD.to_owned() + rule + "over_traces = true\n"));
    let census = Census::read(kept.root())?;
    let kill = census.function("unit_kill");
    assert_eq!(kill.and_then(|f| f.class), Some(Class::Replaced));
    assert!(census.problems().is_empty(), "{:#?}", census.problems());
    // (without it, the trace wins over the rule)
    let ported = tree("oracle-not", &(GOOD.to_owned() + rule));
    let census = Census::read(ported.root())?;
    assert_eq!(
        census.function("unit_kill").and_then(|f| f.class),
        Some(Class::Ported)
    );
    Ok(())
}

#[test]
fn rules_are_chosen_by_how_specific_they_are() -> Result<(), Failure> {
    let manifest = Manifest::parse(
        "census.toml",
        r#"
[[rule]]
class = "missing"
[[rule]]
file = "units/units.c"
class = "matched"
[[rule]]
directory = "units"
class = "native"
[[rule]]
directory = "units"
prefix = "unit_"
class = "partial"
[[rule]]
file = "units/units.c"
class = "replaced"
"#,
    )?;
    let class = |name, file| manifest.lookup(name, file).0.and_then(|entry| entry.class);
    // (a file and a prefix weigh the same; the later of two equal rules wins)
    assert_eq!(
        class("units_update", "units/units.c"),
        Some(Class::Replaced)
    );
    assert_eq!(class("unit_kill", "units/units.c"), Some(Class::Partial));
    assert_eq!(
        class("unit_kill", "units/unit_dialogue.c"),
        Some(Class::Partial)
    );
    assert_eq!(
        class("units_x", "units/unit_dialogue.c"),
        Some(Class::Native)
    );
    assert_eq!(class("ai_x", "ai/ai.c"), Some(Class::Missing));
    Ok(())
}

#[test]
fn the_check_mode_fails_with_the_problems() {
    let tree = tree("mode", &GOOD.replace("because = \"renderer\"\n", ""));
    assert!(matches!(
        census(tree.root(), Mode::Check),
        Err(Error::Problems(found)) if found.iter().any(|p| p.contains("rasterizer_frame"))
    ));
}

#[test]
fn a_traced_script_without_its_trace_is_a_problem() -> Result<(), Failure> {
    let tree = tree("trace-file", GOOD);
    std::fs::remove_file(tree.root().join("tools/traces/unit_reference.txt"))?;
    let found = Census::read(tree.root())?.problems();
    assert!(
        has(&found, "engine_unit_reference's trace (unit_reference.txt)"),
        "{found:#?}"
    );
    Ok(())
}

#[test]
fn only_the_originals_bodies_are_traced() {
    let original = opence_census::traced::bodies_of(
        "void unit_kill(void)\n{\n\tunit_die();\n}\nstatic int unit_think(int x)\n{\n\treturn x;\n}\nvoid object_get(void)\n{\n\tobject_table();\n}\n",
    );
    // (the first script's `unit_kill` wins; `object_get` is the fake
    // world's stand-in, with another body)
    let traced = opence_census::traced::read_sources(
        "### engine_a_reference\nvoid object_get(void)\n{\n}\nvoid unit_kill(void)\n{\n\tunit_die();\n}\n### engine_b_reference\nvoid unit_kill(void)\n{\n\tunit_die();\n}\nstatic int unit_think(int x)\n{\n\treturn x;\n}\n",
        &original,
    );
    assert_eq!(
        opence_census::traced::write_traced(&traced),
        "unit_kill\tengine_a_reference\nunit_think\tengine_b_reference\n"
    );
}

#[test]
fn a_headers_inline_helpers_are_bodies_a_trace_can_copy() -> Result<(), Failure> {
    // (real_math.h's inlines are compiled into the math trace)
    let tree = Tree::new("inline-bodies").with(
        "source/math/real_math.h",
        "__inline real dot_product3d(real_vector3d const *a, real_vector3d const *b)\n{\n\treturn a->i * b->i;\n}\n",
    );
    let original = opence_census::traced::bodies(&tree.root().join("source"))?;
    let traced = opence_census::traced::read_sources(
        "### engine_math_reference\n__inline real dot_product3d(real_vector3d const *a, real_vector3d const *b)\n{\n\treturn a->i * b->i;\n}\n",
        &original,
    );
    assert_eq!(
        opence_census::traced::write_traced(&traced),
        "dot_product3d\tengine_math_reference\n"
    );
    Ok(())
}

#[test]
fn the_summary_counts_each_class() -> Result<(), Failure> {
    let tree = tree("summary-classes", GOOD);
    let summary = Census::read(tree.root())?.summary();
    assert!(
        summary.contains("the classes: 2 Ported (")
            && summary.contains("1 Matched (")
            && summary.contains("2 Replaced (")
            && summary.contains("1 Missing ("),
        "{summary}"
    );
    Ok(())
}

#[test]
fn the_docs_are_written_and_then_not_stale() -> Result<(), Failure> {
    use opence_census::docs;
    let tree = tree("docs", GOOD).with(
        "docs/engine.md",
        &format!(
            "# Engine\n\n{}\nold table\n{}\n\nAfter.\n",
            docs::BEGIN,
            docs::END
        ),
    );
    let census = Census::read(tree.root())?;
    assert_eq!(docs::stale(tree.root(), &census).len(), 2);
    docs::write(tree.root(), &census)?;
    assert!(docs::stale(tree.root(), &census).is_empty());
    let engine_md = std::fs::read_to_string(tree.root().join("docs/engine.md"))?;
    assert!(
        engine_md.contains("| `units` | 2 (") && engine_md.ends_with("\n\nAfter.\n"),
        "{engine_md}"
    );
    let census_md = std::fs::read_to_string(tree.root().join("docs/census.md"))?;
    assert!(census_md.contains("| `unit_think` | units/units.c | Ported | sim/src/units.rs:3 | through `unit_kill`'s trace;"), "{census_md}");
    assert!(
        census_md.contains("- units/units.c: `unit_orphan`"),
        "{census_md}"
    );
    Ok(())
}

#[test]
fn a_directory_rule_selects_its_subdirectories_and_the_deeper_wins() -> Result<(), Failure> {
    let manifest = Manifest::parse(
        "census.toml",
        r#"
[[rule]]
directory = "sound/sound_import"
class = "replaced"
[[rule]]
directory = "sound"
class = "native"
[[rule]]
file = "sound/sound_import/sound_wave.c"
class = "matched"
"#,
    )?;
    let class = |name, file| manifest.lookup(name, file).0.and_then(|entry| entry.class);
    assert_eq!(
        class("sound_file_is_aiff", "sound/sound_import/sound_aiff.c"),
        Some(Class::Replaced)
    );
    assert_eq!(
        class("sound_stop", "sound/sound_manager.c"),
        Some(Class::Native)
    );
    assert_eq!(
        class("sound_file_is_wave", "sound/sound_import/sound_wave.c"),
        Some(Class::Matched)
    );
    // (a directory is a whole step of the path, not a prefix of a name)
    assert_eq!(class("x", "sound_extra/x.c"), None);
    Ok(())
}

/// port/linux/port.json for the fake tree: every C file under `source`
/// compiled into game.wasm, but the rasterizer's.
const PORT: &str = r#"{
  "game": {
    "root": "source",
    "exclude": ["source/rasterizer/"]
  }
}
"#;

#[test]
fn functions_compiled_into_the_game_are_compiled() -> Result<(), Failure> {
    // (no entry and no rule: what game.wasm compiles is Compiled, what a
    // trace compiles stays Ported, and an excluded file takes its rule)
    let manifest = "
[[rule]]
directory = \"rasterizer\"
class = \"replaced\"
because = \"renderer\"
stands_in = \"app/Sources/Render/Frame.swift\"
";
    let tree = tree("compiled", manifest).with("port/linux/port.json", PORT);
    let census = Census::read(tree.root())?;
    let class = |name: &str| census.function(name).and_then(|f| f.class);
    assert_eq!(class("unit_orphan"), Some(Class::Compiled));
    assert_eq!(class("main_loop"), Some(Class::Compiled));
    assert_eq!(class("unit_kill"), Some(Class::Ported));
    assert_eq!(class("rasterizer_frame"), Some(Class::Replaced));
    let found = census.problems();
    assert!(found.is_empty(), "{found:#?}");
    Ok(())
}

#[test]
fn compiled_for_a_file_the_game_leaves_out_is_a_problem() -> Result<(), Failure> {
    let manifest = "
[function.rasterizer_frame]
class = \"compiled\"
";
    let tree = tree("compiled-excluded", manifest).with("port/linux/port.json", PORT);
    let found = Census::read(tree.root())?.problems();
    assert!(
        has(
            &found,
            "rasterizer_frame: rasterizer/rasterizer.c is not compiled into game.wasm"
        ),
        "{found:#?}"
    );
    Ok(())
}

#[test]
fn what_stands_in_for_a_file_the_game_leaves_out_may_be_the_ports() -> Result<(), Failure> {
    // (game.wasm's own stand-in is a C file of the port, outside engine/;
    // one that is not there is still a problem)
    let manifest = "
[[rule]]
file = \"rasterizer/rasterizer.c\"
class = \"replaced\"
because = \"renderer\"
stands_in = \"port/linux/game/rasterizer.c\"
[[rule]]
directory = \"rasterizer\"
class = \"replaced\"
because = \"renderer\"
stands_in = \"source/rasterizer/gone.c\"
";
    let tree = tree("compiled-port", manifest)
        .with("port/linux/port.json", PORT)
        .with("port/linux/game/rasterizer.c", "")
        .with("source/rasterizer/rasterizer_text.c", TEXT);
    let found = Census::read(tree.root())?.problems();
    assert!(
        !found
            .iter()
            .any(|p| p.contains("port/linux/game/rasterizer.c")),
        "{found:#?}"
    );
    assert!(
        found
            .iter()
            .any(|p| p.contains("source/rasterizer/gone.c does not exist")),
        "{found:#?}"
    );
    Ok(())
}

#[test]
fn a_traced_function_game_wasm_compiles_reads_fields_as_compiled() -> Result<(), Failure> {
    // (a field a Ported function reads is read in game.wasm, which runs the
    // original's function as the trace checked it)
    let tree = tree("compiled-reader", "").with("port/linux/port.json", PORT);
    let census = Census::read(tree.root())?;
    assert_eq!(
        census.function("unit_kill").and_then(|f| f.class),
        Some(Class::Ported)
    );
    assert_eq!(census.reader_class("unit_kill"), Some(Class::Compiled));
    assert_eq!(census.reader_class("unit_orphan"), Some(Class::Compiled));
    // (what game.wasm leaves out keeps its own class)
    assert_eq!(
        census.reader_class("rasterizer_frame"),
        census.function("rasterizer_frame").and_then(|f| f.class)
    );
    assert_eq!(census.reader_class("not_a_function"), None);
    Ok(())
}
