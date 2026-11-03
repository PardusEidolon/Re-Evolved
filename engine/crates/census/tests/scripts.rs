//! The original's script functions and globals, in its tables' order, each
//! marked with whether a map may call it and whether the engine wires it.

use opence_census::scripts::{
    functions, globals, read_rows, read_wired, rows, write_rows, Engine, Kind,
};

type Failure = Box<dyn std::error::Error>;

const HS: &str = r#"
static struct hs_function_definition const begin_definition=
{
	_hs_passthrough,
	0,
	"begin",
	hs_parse_begin,
	hs_evaluate_begin,
	"returns the last expression in a sequence.",
	"<expression(s)>",
};

static struct hs_function_definition const cond_definition=
{
	_hs_passthrough, 0, "cond", hs_parse_cond, NULL, "", "",
};

static struct hs_function_definition_with_1_parameter const sound_play_definition=
{
	{
		_hs_type_void,
		0,
		"sound_play",
		hs_macro_function_parse,
		sound_play_evaluate,
		"plays a sound",
	},
	_hs_type_sound,
};

struct hs_function_table_storage hs_function_table=
{
	{
		&begin_definition,
		&cond_definition,
		&sound_play_definition.definition,
	},
	{
		"hs_update",
	},
};

static boolean const hs_function_allowed_in_maps[]=
{
	/* the language */
	TRUE, /* begin */
	FALSE, /* cond: compiled to if */

	TRUE, /* sound_play */
};
"#;

const GLOBALS: &str = r#"
static struct hs_external_global_definition weather_definition = { "weather", _hs_type_boolean, 0, &weather };

static struct hs_external_global_definition fog_definition = { "fog", _hs_type_boolean, 0, &fog };

struct hs_external_global_definition *hs_external_globals[2] =
{
	&fog_definition,
	&weather_definition,
};
"#;

#[test]
fn the_functions_are_the_tables_names_in_order_with_whether_a_map_may_call_them(
) -> Result<(), Failure> {
    assert_eq!(
        functions(HS)?,
        [
            ("begin".to_owned(), true),
            ("cond".to_owned(), false),
            ("sound_play".to_owned(), true),
        ]
    );
    assert_eq!(globals(GLOBALS)?, ["fog".to_owned(), "weather".to_owned()]);
    let short = HS.replace("\tTRUE, /* sound_play */\n", "");
    assert!(functions(&short).is_err(), "a flag for each function");
    Ok(())
}

#[test]
fn each_is_wired_special_or_missing_and_the_engines_own_are_listed() -> Result<(), Failure> {
    let wired = read_wired(
        "# the engine's\n\
         special\tbegin\n\
         function\tsound_play\n\
         function\tengine_only\n\
         global\tweather\n",
    )?;
    let found = rows(&functions(HS)?, &globals(GLOBALS)?, &wired);
    let marks: Vec<(Kind, &str, Option<bool>, Engine)> = found
        .iter()
        .map(|r| (r.kind, r.name.as_str(), r.maps, r.engine))
        .collect();
    assert_eq!(
        marks,
        [
            (Kind::Function, "begin", Some(true), Engine::Special),
            (Kind::Function, "cond", Some(false), Engine::Missing),
            (Kind::Function, "sound_play", Some(true), Engine::Wired),
            (Kind::Global, "fog", None, Engine::Missing),
            (Kind::Global, "weather", None, Engine::Wired),
            (Kind::Function, "engine_only", None, Engine::Extra),
        ]
    );
    assert_eq!(read_rows(&write_rows(&found))?, found);
    assert!(read_wired("method\tx\n").is_err());
    assert!(read_rows("function\tx\tmaybe\twired\n").is_err());
    Ok(())
}
