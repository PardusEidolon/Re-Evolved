//! `opence scripts MAP`: a map's scripts, a line each with when it runs,
//! so the map audit can find the cutscenes to play.

use opence_cli::{script_lines, Error};
use opence_content::{Script, ScriptKind, Scripts, ValueType};

fn script(name: &str, kind: ScriptKind) -> Script {
    Script {
        name: name.into(),
        kind,
        return_type: ValueType::Void,
        root: 0,
    }
}

#[test]
fn each_script_is_its_name_and_when_it_runs() -> Result<(), Error> {
    let scripts = Scripts {
        scripts: vec![
            script("mission_start", ScriptKind::Startup),
            script("cutscene_insertion", ScriptKind::Dormant),
            script("music_loop", ScriptKind::Continuous),
            script("cinematic_skip", ScriptKind::Static),
            script("placeholder", ScriptKind::Stub),
        ],
        ..Scripts::default()
    };
    assert_eq!(
        script_lines(&scripts)?,
        [
            "mission_start\tstartup",
            "cutscene_insertion\tdormant",
            "music_loop\tcontinuous",
            "cinematic_skip\tstatic",
            "placeholder\tstub",
        ]
    );
    Ok(())
}

#[test]
fn scripts_the_game_would_not_run_are_refused_with_the_reason() {
    let scripts = Scripts {
        disabled: Some("too many nodes".into()),
        ..Scripts::default()
    };
    assert!(
        matches!(script_lines(&scripts), Err(Error::Refused(reason)) if reason.contains("too many nodes"))
    );
}

#[test]
fn a_map_s_scripts_are_read_from_its_file() {
    let map = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/maps/a10.map");
    if !map.is_file() {
        eprintln!("skipped: needs assets/maps/a10.map");
        return;
    }
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_opence"))
        .args(["scripts".as_ref(), map.as_os_str()])
        .output();
    let Ok(output) = output else {
        panic!("opence did not run: {output:?}");
    };
    assert!(output.status.success(), "{output:?}");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.lines().count() > 10, "{text}");
    assert!(
        text.lines().all(|line| line.split('\t').count() == 2),
        "{text}"
    );
}
