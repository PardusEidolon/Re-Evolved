//! The Video menu's display settings, which the app applies. The game asks
//! with `platform_display_apply` once its settings are saved; the request
//! waits here until the app takes it ([`crate::Controls::take_display`])
//! and sets its window from it: full screen or not, its size, and vsync.
//! The Linux port's SDL path does the same (`port/linux/src/sdl_platform.c`).
//!
//! macOS's full screen is the display's own size and mode, so there are no
//! resolutions to list (`platform_display_resolutions` leaves only the
//! tag's "Native"); the window sizes are the Linux port's, those that fit
//! the screen the app last gave ([`crate::Controls::set_screen_size`]).
//! A message the game shows the player in a box of its own
//! (`platform_show_message`) is logged and waits for the app the same way
//! ([`crate::Controls::take_message`]).

use wasmtime::{Caller, Val};

use super::config::{read_c_string, Settings};
use super::memory::{arg, give, Handler};
use super::{with_space, Record};

/// Video Setup's window sizes, by shape (4:3, 16:10, 16:9, 21:9), each
/// from the smallest (`platform_window_sizes_offered`)
const OFFERED: [(u32, u32); 21] = [
    (640, 480),
    (800, 600),
    (1024, 768),
    (1280, 960),
    (1600, 1200),
    (1920, 1440),
    (2560, 1920),
    (1280, 800),
    (1440, 900),
    (1680, 1050),
    (1920, 1200),
    (2560, 1600),
    (1280, 720),
    (1600, 900),
    (1920, 1080),
    (2560, 1440),
    (3840, 2160),
    (2560, 1080),
    (3440, 1440),
    (3840, 1600),
    (5120, 2160),
];
/// the longest title or message read (`platform_message_text`'s size)
const LONGEST_MESSAGE: usize = 1024;
/// the Xbox's own size, the smallest a setting may name
const XBOX: (u32, u32) = (640, 480);

/// How the app is to show the game, from the display settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisplayRequest {
    /// full screen (`display.mode` fullscreen or borderless, or with no
    /// mode `display.fullscreen`), or a window
    pub full_screen: bool,
    /// the window's size in pixels, out of full screen
    /// (`display.window_size`, else 640x480 by `display.window_scale`)
    pub width: u32,
    /// and its height
    pub height: u32,
    /// whether frames wait for the display's refresh (`display.vsync`)
    pub vsync: bool,
}

/// The display's state between the game and the app.
#[derive(Default)]
pub(crate) struct Display {
    /// the screen's size in pixels, as the app last gave it (0 by 0 until
    /// then: only the Xbox's size fits)
    screen: (u32, u32),
    /// the settings applied, until the app takes them
    pending: Option<DisplayRequest>,
    /// the last message shown, its title and text, until the app takes it
    message: Option<(String, String)>,
}

impl Display {
    pub(crate) fn set_screen(&mut self, width: u32, height: u32) {
        self.screen = (width, height);
    }

    pub(crate) fn take(&mut self) -> Option<DisplayRequest> {
        self.pending.take()
    }

    pub(crate) fn take_message(&mut self) -> Option<(String, String)> {
        self.message.take()
    }
}

/// a size as a setting has it, "<width>x<height>", if it is one and the
/// Xbox's 640x480 or more (`platform_size_parse`)
fn size_of(text: &str) -> Option<(u32, u32)> {
    let (width, height) = text.split_once(['x', 'X'])?;
    let size = (width.parse().ok()?, height.parse().ok()?);
    (size.0 >= XBOX.0 && size.1 >= XBOX.1).then_some(size)
}

/// the window's size the settings name (`platform_window_size_setting`)
fn window_size(settings: &Settings) -> (u32, u32) {
    settings
        .text("display.window_size")
        .and_then(size_of)
        .unwrap_or_else(|| {
            let scale = settings
                .shown("display.window_scale")
                .and_then(|scale| scale.parse::<u32>().ok())
                .unwrap_or(1)
                .clamp(1, 8);
            (XBOX.0 * scale, XBOX.1 * scale)
        })
}

/// what the display settings ask of the app (`platform_display_apply`)
fn request(settings: &Settings) -> DisplayRequest {
    let full_screen = match settings.text("display.mode").unwrap_or_default() {
        "fullscreen" | "borderless" => true,
        "windowed" => false,
        _ => settings.boolean("display.fullscreen").unwrap_or(true),
    };
    let (width, height) = window_size(settings);
    DisplayRequest {
        full_screen,
        width,
        height,
        vsync: settings.boolean("display.vsync").unwrap_or(true),
    }
}

/// the window sizes Video Setup offers on a screen `screen` big: those
/// that fit it (the Xbox's whatever the screen), and the one `set` though
/// it is none of them or too big, so that the menu shows it
fn window_sizes(screen: (u32, u32), set: (u32, u32)) -> Vec<(u32, u32)> {
    let mut sizes: Vec<(u32, u32)> = OFFERED
        .iter()
        .enumerate()
        .filter(|&(index, &(width, height))| {
            index == 0 || (width <= screen.0 && height <= screen.1)
        })
        .map(|(_, &size)| size)
        .collect();
    if !sizes.contains(&set) {
        sizes.push(set);
    }
    sizes
}

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "platform_display_apply" => apply,
        "platform_window_sizes" => sizes,
        "platform_show_message" => show_message,
        _ => return None,
    };
    Some(handler)
}

/// `platform_display_apply()`: the Video menu's settings saved; the app
/// applies them at its next look
fn apply(caller: &mut Caller<'_, Record>, _: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    let wanted = request(&caller.data().settings());
    caller.data().display().pending = Some(wanted);
    Ok(())
}

/// `platform_show_message(title, message)`: a message for the player (the
/// network code's: a host of another version), logged as the Linux port
/// logs it and kept for the app to show
fn show_message(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let title = read_c_string(caller, arg(params, 0), LONGEST_MESSAGE)?;
    let message = read_c_string(caller, arg(params, 1), LONGEST_MESSAGE)?;
    std::io::Write::write_all(
        &mut std::io::stderr(),
        format!("{title}: {message}\n").as_bytes(),
    )
    .ok();
    caller.data().display().message = Some((title, message));
    Ok(())
}

/// `platform_window_sizes(widths, heights, maximum)`: the window sizes
/// Video Setup offers, as two arrays of C `long`s (four bytes in wasm32);
/// their count
fn sizes(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (widths, heights, maximum) = (arg(params, 0), arg(params, 1), arg(params, 2));
    let room = usize::try_from(i32::from_le_bytes(maximum.to_le_bytes()).max(0)).unwrap_or(0);
    let set = window_size(&caller.data().settings());
    let screen = caller.data().display().screen;
    let offered: Vec<(u32, u32)> = window_sizes(screen, set).into_iter().take(room).collect();
    let fits = with_space(caller, |space, _| {
        offered.iter().zip(0u32..).all(|(&(width, height), index)| {
            space.write(widths.wrapping_add(index * 4), &width.to_le_bytes())
                && space.write(heights.wrapping_add(index * 4), &height.to_le_bytes())
        })
    })?;
    if !fits {
        return Err(wasmtime::format_err!(
            "the window sizes' arrays {widths:#x} and {heights:#x} are outside the memory"
        ));
    }
    give(results, u32::try_from(offered.len()).unwrap_or(0));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{request, size_of, window_sizes, DisplayRequest, Settings};

    fn settings(values: &[(&str, &str)]) -> Settings {
        let mut settings = Settings::new(|_| None);
        for (name, value) in values {
            assert!(settings.write(name, value), "{name}");
        }
        settings
    }

    #[test]
    fn the_mode_decides_full_screen_and_the_size_its_window() {
        assert_eq!(
            request(&settings(&[])),
            DisplayRequest {
                full_screen: true,
                width: 1280,
                height: 960,
                vsync: true
            }
        );
        let windowed = settings(&[
            ("display.mode", "windowed"),
            ("display.window_size", "1920x1080"),
            ("display.vsync", "false"),
        ]);
        assert_eq!(
            request(&windowed),
            DisplayRequest {
                full_screen: false,
                width: 1920,
                height: 1080,
                vsync: false
            }
        );
        assert!(
            request(&settings(&[
                ("display.mode", "borderless"),
                ("display.fullscreen", "false")
            ]))
            .full_screen
        );
        assert!(!request(&settings(&[("display.fullscreen", "false")])).full_screen);
    }

    #[test]
    fn a_size_smaller_than_the_xbox_or_malformed_is_no_size() {
        assert_eq!(size_of("1280x720"), Some((1280, 720)));
        assert_eq!(size_of("800X600"), Some((800, 600)));
        for text in [
            "",
            "native",
            "320x240",
            "1280x",
            "x720",
            "1280x720x2",
            "-1x480",
        ] {
            assert_eq!(size_of(text), None, "{text}");
        }
    }

    #[test]
    fn only_the_sizes_that_fit_the_screen_are_offered_and_the_set_one() {
        let sizes = window_sizes((1920, 1080), (640, 480));
        assert_eq!(sizes.first(), Some(&(640, 480)));
        assert!(sizes.contains(&(1920, 1080)));
        assert!(!sizes.contains(&(1920, 1200)));
        assert!(sizes
            .iter()
            .all(|&(width, height)| width <= 1920 && height <= 1080));
        // (no screen yet: the Xbox's alone, and the set one)
        assert_eq!(
            window_sizes((0, 0), (5120, 2160)),
            vec![(640, 480), (5120, 2160)]
        );
        assert_eq!(window_sizes((0, 0), (640, 480)), vec![(640, 480)]);
    }
}
