//! The Xbox's system calls the game makes on its way to its main loop
//! that no other module answers, starting with the debug monitor
//! (`xbdm`), which the game walks for its modules' sections, and the
//! title's language and launch, and the menu's Quit. Each follows
//! the Linux port's answer (`port/linux/src/`), which `source/` already
//! runs against. (The C library's helpers are compiled into game.wasm,
//! `port/wasmtime/crt.c`.)

use wasmtime::{Caller, Val};

use super::memory::{arg, give, Handler};
use super::{with_space, Record};

/// the debug monitor's end of a list (`port/include/xdk/xdk_xbdm.h`)
const XBDM_ENDOFLIST: u32 = 0x82DB_0104;
/// and its plain success
const XBDM_NOERR: u32 = 0x02DB_0000;

/// XC_LANGUAGE_ENGLISH (`port/include/xdk/xdk_xbox.h`)
const ENGLISH: u32 = 1;
/// the languages `game.language` may choose by its first two letters, and
/// their XC_LANGUAGE_* (`port/linux/src/xbox_xapi.c`)
const LANGUAGES: [(&str, u32); 5] = [("ja", 2), ("de", 3), ("fr", 4), ("es", 5), ("it", 6)];
/// LDT_FROM_DASHBOARD (`port/include/xdk/xdk_xbox.h`)
const FROM_DASHBOARD: u32 = 2;
/// the size of LAUNCH_DATA (`port/include/xdk/xdk_pdb.h`)
const LAUNCH_DATA: usize = 3072;
/// ERROR_SUCCESS
const SUCCESS: u32 = 0;

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "DmWalkLoadedModules" | "DmWalkModuleSections" => end_of_list,
        "DmCloseModuleSections" => no_error,
        "XGetLanguage" => language,
        "XGetLaunchInfo" => launch_info,
        "platform_request_quit" => request_quit,
        _ => return None,
    };
    Some(handler)
}

/// `platform_request_quit()`: the menu's Quit. The Linux port leaves with
/// `exit(EXIT_SUCCESS)`; here the run ends as `proc_exit(0)` ends it.
fn request_quit(_: &mut Caller<'_, Record>, _: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    Err(wasmtime::Error::new(super::Exit(0)))
}

/// `DmWalkLoadedModules` and `DmWalkModuleSections`: there is no debug
/// monitor, so every walk is already at its end (`port/linux/src/xbdm.c`)
fn end_of_list(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, XBDM_ENDOFLIST);
    Ok(())
}

/// `DmCloseModuleSections(walk)`: nothing to close
fn no_error(
    _: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let _walk = arg(params, 0);
    give(results, XBDM_NOERR);
    Ok(())
}

/// `XGetLanguage()`: the language `game.language` names, English unless it
/// starts with one of [`LANGUAGES`]
fn language(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let language = {
        let settings = caller.data().settings();
        let chosen = settings.text("game.language").unwrap_or_default();
        LANGUAGES
            .iter()
            .find(|(prefix, _)| chosen.starts_with(prefix))
            .map_or(ENGLISH, |&(_, language)| language)
    };
    give(results, language);
    Ok(())
}

/// `XGetLaunchInfo(type, data)`: the game was started from the dashboard,
/// with no launch data; either pointer may be null
fn launch_info(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (kind, data) = (arg(params, 0), arg(params, 1));
    with_space(caller, |space, _| {
        if kind != 0 {
            space.write(kind, &FROM_DASHBOARD.to_le_bytes());
        }
        if data != 0 {
            space.write(data, &[0; LAUNCH_DATA]);
        }
    })?;
    give(results, SUCCESS);
    Ok(())
}
