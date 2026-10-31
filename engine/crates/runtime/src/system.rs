//! The Xbox's system calls the game makes on its way to its main loop
//! that no other module answers, starting with the debug monitor
//! (`xbdm`), which the game walks for its modules' sections, and the
//! title's language and launch, and the menu's Quit (and the Xbox's,
//! a launch of its dashboard). Each follows
//! the Linux port's answer (`port/linux/src/`), which `source/` already
//! runs against. (The C library's helpers are compiled into game.wasm,
//! `port/wasmtime/crt.c`.)

use wasmtime::{Caller, Val};

use super::config::read_c_string;
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
/// the pasteboard's tools, which read and write the general pasteboard as
/// plain text
const PASTE: &str = "/usr/bin/pbpaste";
const COPY: &str = "/usr/bin/pbcopy";
/// the longest text set (a network link)
const LONGEST_COPY: usize = 4096;
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
        "XLaunchNewImageA" => launch_new_image,
        "platform_clipboard_get" => clipboard_get,
        "platform_clipboard_set" => clipboard_set,
        _ => return None,
    };
    Some(handler)
}

/// `platform_request_quit()`: the menu's Quit. The Linux port leaves with
/// `exit(EXIT_SUCCESS)`; here the run ends as `proc_exit(0)` ends it.
fn request_quit(_: &mut Caller<'_, Record>, _: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    Err(wasmtime::Error::new(super::Exit(0)))
}

/// `XLaunchNewImageA(path, data)`: the Xbox's way back to its dashboard
/// (the main menu's Quit on the Xbox's own menus). There is no other
/// image to launch, so the game ends as at `platform_request_quit`.
fn launch_new_image(
    _: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let (_path, _data) = (arg(params, 0), arg(params, 1));
    Err(wasmtime::Error::new(super::Exit(0)))
}

/// the pasteboard's text as a C string fits `size` bytes: cut at its first
/// line and at a character's end, with its NUL; `None` when there is none
fn pasted(text: &str, size: usize) -> Option<Vec<u8>> {
    let line = text.lines().next().unwrap_or_default();
    let room = size.checked_sub(1)?;
    let mut end = line.len().min(room);
    while !line.is_char_boundary(end) {
        end -= 1;
    }
    let mut bytes = line.as_bytes().get(..end)?.to_vec();
    if bytes.is_empty() {
        return None;
    }
    bytes.push(0);
    Some(bytes)
}

/// `platform_clipboard_get(text, size)`: a text field's paste (Ctrl+V);
/// whether the pasteboard had text, which is copied into `text`
fn clipboard_get(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (text, size) = (arg(params, 0), arg(params, 1));
    let board = std::process::Command::new(PASTE)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .unwrap_or_default();
    let got = match pasted(&board, usize::try_from(size).unwrap_or(0)) {
        Some(bytes) => with_space(caller, |space, _| space.write(text, &bytes))?,
        None => false,
    };
    give(results, u32::from(got));
    Ok(())
}

/// `platform_clipboard_set(text)`: the network menu's copy of a link
fn clipboard_set(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let text = read_c_string(caller, arg(params, 0), LONGEST_COPY)?;
    let copy = std::process::Command::new(COPY)
        .stdin(std::process::Stdio::piped())
        .spawn();
    if let Ok(mut copy) = copy {
        if let Some(mut input) = copy.stdin.take() {
            std::io::Write::write_all(&mut input, text.as_bytes()).ok();
        }
        copy.wait().ok();
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::pasted;

    #[test]
    fn the_paste_is_its_first_line_cut_to_fit_at_a_character() {
        assert_eq!(pasted("Chief\nsecond", 32), Some(b"Chief\0".to_vec()));
        assert_eq!(pasted("abcdef", 4), Some(b"abc\0".to_vec()));
        // (é is two bytes: not split)
        assert_eq!(pasted("aé", 3), Some(b"a\0".to_vec()));
        assert_eq!(pasted("", 32), None);
        assert_eq!(pasted("\nlater", 32), None);
        assert_eq!(pasted("text", 0), None);
        assert_eq!(pasted("text", 1), None);
    }
}
