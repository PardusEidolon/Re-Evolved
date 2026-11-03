//! The original's load-time fix-ups: a tag block's default element and
//! postprocess, a tag group's postprocess and the load path's
//! `*_postprocess` functions, each with a row of the manifest saying how
//! the importer compares.

use std::collections::BTreeMap;

use opence_census::fixups::{found, problems, read_fixups, Comparison};

type Failure = Box<dyn std::error::Error>;

const C: &str = r#"
static boolean postprocess_thing(
	struct thing *thing,
	boolean editing);

static struct tag_block_definition thing_block =
{
	"thing_block",
	0,
	16,
	sizeof(struct thing),
	default_thing,
	thing_block_fields,
	NULL,
	postprocess_thing,
	NULL,
	NULL,
	NULL,
};

static struct tag_block_definition plain_block =
{
	"plain_block", 0, 4, sizeof(long), NULL, plain_fields, NULL, NULL, NULL, NULL, NULL,
};

struct tag_group thing_group =
{
	"thing",
	0,
	THING_TAG,
	NONE,
	1,
	postprocess_thing_group,
	&thing_block,
};

static boolean postprocess_thing(
	struct thing *thing,
	boolean editing)
{
	return TRUE;
}

void scenario_names_postprocess(
	void)
{
	return;
}

void unit_postprocess_node_matrices(
	long unit_index)
{
	return;
}
"#;

#[test]
fn the_fix_ups_are_the_initialisers_entries_and_the_postprocess_functions() {
    let pairs = found(C);
    let fixups: Vec<(&str, &str)> = pairs
        .iter()
        .map(|(n, p)| (n.as_str(), p.as_str()))
        .collect();
    assert_eq!(
        fixups,
        [
            ("default_thing", "thing_block's default element"),
            ("postprocess_thing", "thing_block's postprocess"),
            ("postprocess_thing_group", "thing_group's postprocess"),
            ("postprocess_thing", "defined"),
            ("scenario_names_postprocess", "defined"),
        ]
    );
}

#[test]
fn each_fix_up_has_one_row_and_each_row_a_fix_up() -> Result<(), Failure> {
    let rows = read_fixups(
        "# a comment\n\
         postprocess_thing\tnone\treturns TRUE\n\
         postprocess_thing_group\tdiffers\tsets the linear flag\n\
         postprocess_thing_group\tsame\tagain\n\
         gone_postprocess\tsame\tno longer in source/\n",
    )?;
    assert_eq!(rows[0].comparison, Comparison::None);
    assert_eq!(rows[1].comparison, Comparison::Differs);
    let mut fixups = BTreeMap::new();
    for (name, place) in found(C) {
        fixups.entry(name).or_insert_with(Vec::new).push(place);
    }
    assert_eq!(
        problems(&fixups, &rows),
        [
            "census/fixups.tsv: postprocess_thing_group has two rows",
            "census/fixups.tsv: gone_postprocess is no fix-up of source/",
            "census/fixups.tsv: default_thing (thing_block's default element) has no row",
            "census/fixups.tsv: scenario_names_postprocess (defined) has no row",
        ]
    );
    assert!(read_fixups("x\tmaybe\twhy\n").is_err());
    assert!(read_fixups("x\tsame\n").is_err());
    assert!(read_fixups("x\tsame\t\n").is_err());
    Ok(())
}
