//! The real tree's reasons: game.wasm compiles the original's own code, so
//! its systems, its console and its debug functions are Compiled; what it
//! leaves out is Replaced, and nothing is Missing.

mod common;

use opence_census::classes::Class;
use opence_census::Census;

type Failure = Box<dyn std::error::Error>;

/// The functions of `files` that `keep` does not accept.
fn left(census: &Census, files: &[&str], keep: impl Fn(Option<Class>) -> bool) -> Vec<String> {
    census
        .functions
        .iter()
        .filter(|f| files.contains(&f.file.as_str()))
        .filter(|f| !keep(f.class))
        .map(|f| format!("{} ({}): {:?}", f.name, f.file, f.class))
        .collect()
}

/// The original's own code, in game.wasm: compiled, or traced as well.
fn in_the_game(class: Option<Class>) -> bool {
    matches!(class, Some(Class::Compiled | Class::Ported))
}

#[test]
fn the_collision_bsp_walk_is_the_originals() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let census = Census::read(&root)?;
    let left = left(
        &census,
        &[
            "physics/collision_bsp.c",
            "physics/bsp2d.c",
            "physics/bsp3d.c",
        ],
        in_the_game,
    );
    assert!(left.is_empty(), "{left:#?}");
    Ok(())
}

/// The original's debug functions outside its debug files, each a debug
/// draw, a console command or a debug pick.
const DEBUG: &[&str] = &[
    "render_debug_recording",
    "device_render_debug",
    "players_debug_render",
    "debug_player_teleport",
    "hs_parse_debug_string",
    "hs_evaluate_debug_string",
    "render_debug_profile_stall_tick",
    "render_debug_profile",
    "ui_widget_debug_show_path",
    "random_seed_debug_log",
    "damage_render_debug",
    "render_debug_object_damage",
    "object_type_render_debug",
    "debug_pvs",
    "biped_render_debug",
    "units_debug_get_next_unit",
    "units_debug_get_closest_unit",
    "unit_debug_ninja_rope",
    "unit_render_debug",
    "vehicle_render_debug",
    "debug_sound_classes_set_distances_evaluate",
    "ai_debug_vocalize_evaluate",
    "debug_sound_classes_enable_evaluate",
    "ui_debug_targets_enabled",
    "ui_debug_draw_outline",
    "ui_debug_draw_mark",
    "ui_debug_draw_targets",
    "ui_debug_set_mark",
    "main_frame_rate_debug",
    "object_render_debug",
];

#[test]
fn the_console_and_the_debug_functions_are_the_originals() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let census = Census::read(&root)?;
    let mut left = left(
        &census,
        &["main/console.c", "interface/terminal.c", "game/cheats.c"],
        in_the_game,
    );
    left.extend(
        census
            .functions
            .iter()
            .filter(|f| DEBUG.contains(&f.name.as_str()) && !in_the_game(f.class))
            .map(|f| format!("{}: {:?}", f.name, f.class)),
    );
    assert!(left.is_empty(), "{left:#?}");
    Ok(())
}

#[test]
fn what_the_game_leaves_out_is_replaced_and_nothing_is_missing() -> Result<(), Failure> {
    let Some(root) = common::repository() else {
        eprintln!("skipped: no source/");
        return Ok(());
    };
    let census = Census::read(&root)?;
    let wrong: Vec<String> = census
        .functions
        .iter()
        .filter(|f| {
            let compiled = census.game.compiles(&f.file);
            match f.class {
                Some(Class::Compiled) => !compiled,
                Some(Class::Ported) => false,
                Some(Class::Replaced) => compiled,
                _ => true,
            }
        })
        .map(|f| format!("{} ({}): {:?}", f.name, f.file, f.class))
        .collect();
    assert!(wrong.is_empty(), "{} wrong: {wrong:#?}", wrong.len());
    Ok(())
}
