//! The game's settings, which it asks the platform for by name
//! (`config_boolean("display.vsync")` and the rest). Each follows the
//! native ports' answer (`port/linux/src/port_config.c`): the same table of
//! settings, copied into `settings.tsv` by `tools/engine_settings.py`, each
//! starting at its default and overridden by its `HALO_*` variable.
//!
//! What the game's menus write (`config_write`) is kept, as the Linux port
//! keeps it in its `config.toml`: in `settings.txt` in the saves folder, a
//! `name<TAB>value` line each, read back over the defaults when the game
//! starts ([`Settings::keep_in`]). Settings set for a run (the tests', the
//! dev panel's) are not kept.

use std::collections::BTreeMap;

use wasmtime::{Caller, Val};

use super::memory::{arg, call_game, give, Handler};
use super::{with_space, Record};

/// the table: name, type, default, variable and how it is read, a line each
const TABLE: &str = include_str!("settings.tsv");

/// the longest value `config_write` takes, and the longest name read
const LONGEST_VALUE: usize = 256;
const LONGEST_NAME: usize = 256;
/// the kept settings' file, in the saves folder
pub(crate) const KEPT: &str = "settings.txt";

/// a setting's type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Boolean,
    Integer,
    Real,
    Text,
}

/// how a setting's variable sets it
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Style {
    /// the variable's text is the value
    Value,
    /// the variable, set, makes it true
    SetIsTrue,
    /// the variable, set, makes it false
    SetIsFalse,
}

/// a setting's value
#[derive(Clone, PartialEq, Debug)]
enum Value {
    Boolean(bool),
    Integer(i32),
    Real(f64),
    Text(String),
}

/// one setting of the table, and its value now
#[derive(Debug)]
struct Setting {
    kind: Kind,
    default: String,
    value: Value,
    /// the copy of a string's value in the game's memory, once asked for
    /// (the copies of old values stay, for whoever holds them)
    copy: Option<u32>,
}

/// the settings, by name
#[derive(Debug, Default)]
pub(crate) struct Settings {
    settings: BTreeMap<String, Setting>,
    /// how many times a setting has been written
    changes: u32,
    /// the file the game's writes are kept in, once there is one
    file: Option<std::path::PathBuf>,
    /// the settings the game wrote, by name, as text
    kept: BTreeMap<String, String>,
}

/// the kept settings in a file's text: each line's name and value, the
/// lines without a tab passed over
fn kept_of(text: &str) -> Vec<(&str, &str)> {
    text.lines()
        .filter_map(|line| line.split_once('\t'))
        .filter(|(name, _)| !name.is_empty())
        .collect()
}

/// the file's text for the kept settings
fn kept_text(kept: &BTreeMap<String, String>) -> String {
    kept.iter()
        .filter(|(_, value)| !value.contains(['\n', '\r']))
        .map(|(name, value)| format!("{name}\t{value}\n"))
        .collect()
}

/// whether `text` reads as false ("0", "false", "no" or "off", in any case)
fn is_false(text: &str) -> bool {
    ["0", "false", "no", "off"]
        .iter()
        .any(|word| text.eq_ignore_ascii_case(word))
}

/// a variable counts as set when it is there, not empty and not false
fn is_set(text: &str) -> bool {
    !text.is_empty() && !is_false(text)
}

/// `strtol(text, NULL, 10)`: the leading integer, or 0, saturated to 32 bits
/// (the game's `long`)
fn integer_of(text: &str) -> i32 {
    let text = text.trim_start();
    let (negative, digits) = match text.as_bytes().first() {
        Some(b'-') => (true, text.get(1..).unwrap_or_default()),
        Some(b'+') => (false, text.get(1..).unwrap_or_default()),
        _ => (false, text),
    };
    let mut value: i64 = 0;
    for digit in digits.bytes().take_while(u8::is_ascii_digit) {
        value = value
            .saturating_mul(10)
            .saturating_add(i64::from(digit - b'0'));
    }
    let value = if negative { -value } else { value };
    i32::try_from(value.clamp(i64::from(i32::MIN), i64::from(i32::MAX))).unwrap_or_default()
}

/// `strtod(text, NULL)`: the longest leading number, or 0
fn real_of(text: &str) -> f64 {
    let text = text.trim_start();
    (1..=text.len())
        .rev()
        .filter_map(|end| text.get(..end))
        .find_map(|prefix| prefix.parse::<f64>().ok())
        .unwrap_or(0.0)
}

/// a value of `kind` read from `text`, as the native ports read the file's
fn value_of(kind: Kind, text: &str) -> Value {
    match kind {
        Kind::Boolean => Value::Boolean(!is_false(text)),
        Kind::Integer => Value::Integer(integer_of(text)),
        Kind::Real => Value::Real(real_of(text)),
        Kind::Text => Value::Text(text.to_string()),
    }
}

/// `printf("%.15g", real)`
fn print_real(real: f64) -> String {
    if !real.is_finite() {
        return if real.is_nan() {
            "nan".into()
        } else if real > 0.0 {
            "inf".into()
        } else {
            "-inf".into()
        };
    }
    if real == 0.0 {
        return if real.is_sign_negative() {
            "-0".into()
        } else {
            "0".into()
        };
    }
    // the exponent of the value rounded to 15 significant digits
    let scientific = format!("{real:.14e}");
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let exponent: i32 = exponent.parse().unwrap_or_default();
    let trim = |digits: String| -> String {
        if digits.contains('.') {
            digits
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            digits
        }
    };
    if !(-4..15).contains(&exponent) {
        let sign = if exponent < 0 { '-' } else { '+' };
        format!(
            "{}e{sign}{:02}",
            trim(mantissa.to_string()),
            exponent.unsigned_abs()
        )
    } else {
        let decimals = usize::try_from(14 - exponent).unwrap_or_default();
        trim(format!("{real:.decimals$}"))
    }
}

/// a value as `config_text` writes it
fn text_of(value: &Value) -> String {
    match value {
        Value::Boolean(true) => "true".into(),
        Value::Boolean(false) => "false".into(),
        Value::Integer(integer) => integer.to_string(),
        Value::Real(real) => print_real(*real),
        Value::Text(text) => text.clone(),
    }
}

impl Settings {
    /// the table's settings at their defaults, each overridden by its
    /// variable as `variable` finds it (`std::env::var` for the game)
    pub(crate) fn new(variable: impl Fn(&str) -> Option<String>) -> Self {
        let mut settings = BTreeMap::new();
        for line in TABLE.lines() {
            let fields: Vec<&str> = line.split('\t').collect();
            let [name, kind, default, environment, style] = fields.as_slice() else {
                continue;
            };
            let kind = match *kind {
                "boolean" => Kind::Boolean,
                "integer" => Kind::Integer,
                "real" => Kind::Real,
                "string" => Kind::Text,
                _ => continue,
            };
            let style = match *style {
                "set_is_true" => Style::SetIsTrue,
                "set_is_false" => Style::SetIsFalse,
                _ => Style::Value,
            };
            let mut value = value_of(kind, default);
            if let Some(text) = Some(*environment)
                .filter(|environment| !environment.is_empty())
                .and_then(&variable)
            {
                match style {
                    Style::Value => value = value_of(kind, &text),
                    Style::SetIsTrue if is_set(&text) => value = Value::Boolean(true),
                    Style::SetIsFalse if is_set(&text) => value = Value::Boolean(false),
                    _ => {}
                }
            }
            let setting = Setting {
                kind,
                default: (*default).to_string(),
                value,
                copy: None,
            };
            settings.insert((*name).to_string(), setting);
        }
        Self {
            settings,
            changes: 0,
            file: None,
            kept: BTreeMap::new(),
        }
    }

    /// keeps the game's writes in `file` from now on, the settings it
    /// already holds read back over the defaults (a missing file: none
    /// yet; a name no longer in the table is dropped)
    pub(crate) fn keep_in(&mut self, file: &std::path::Path) {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        for (name, value) in kept_of(&text) {
            if self.write(name, value) {
                self.kept.insert(name.to_owned(), value.to_owned());
            }
        }
        self.file = Some(file.to_owned());
    }

    /// keeps `name` at `text`, written by the game: the file written anew
    /// (beside it, then moved over it, so that it is never half written);
    /// whether it was kept
    fn keep(&mut self, name: &str, text: &str) -> bool {
        self.kept.insert(name.to_owned(), text.to_owned());
        let Some(file) = &self.file else {
            return false;
        };
        let fresh = file.with_extension("txt.new");
        std::fs::write(&fresh, kept_text(&self.kept)).is_ok()
            && std::fs::rename(&fresh, file).is_ok()
    }

    /// the setting `name`'s value, if it is of `kind`
    fn value(&self, name: &str, kind: Kind) -> Option<&Value> {
        self.settings
            .get(name)
            .filter(|setting| setting.kind == kind)
            .map(|setting| &setting.value)
    }

    /// the string setting `name`'s value, if there is one
    pub(crate) fn text(&self, name: &str) -> Option<&str> {
        match self.value(name, Kind::Text)? {
            Value::Text(text) => Some(text),
            _ => None,
        }
    }

    /// the real setting `name`'s value, if there is one
    pub(crate) fn real(&self, name: &str) -> Option<f64> {
        match self.value(name, Kind::Real)? {
            Value::Real(real) => Some(*real),
            _ => None,
        }
    }

    /// the boolean setting `name`'s value, if there is one
    pub(crate) fn boolean(&self, name: &str) -> Option<bool> {
        match self.value(name, Kind::Boolean)? {
            Value::Boolean(boolean) => Some(*boolean),
            _ => None,
        }
    }

    /// the setting `name`'s value as the settings file writes it, whatever
    /// its type, if there is such a setting
    pub(crate) fn shown(&self, name: &str) -> Option<String> {
        self.settings
            .get(name)
            .map(|setting| text_of(&setting.value))
    }

    /// `config_write`: sets `name` from `text`; whether it was set
    pub(crate) fn write(&mut self, name: &str, text: &str) -> bool {
        if !name.contains('.') || text.len() > LONGEST_VALUE {
            return false;
        }
        let Some(setting) = self.settings.get_mut(name) else {
            return false;
        };
        setting.value = value_of(setting.kind, text);
        setting.copy = None;
        self.changes = self.changes.wrapping_add(1);
        true
    }
}

/// The handler for the import "module.name", if this module answers it.
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name.strip_prefix("env.")? {
        "config_boolean" => config_boolean,
        "config_integer" => config_integer,
        "config_real" => config_real,
        "config_string" => config_string,
        "config_text" => config_text,
        "config_default" => config_default,
        "config_write" => config_write,
        "config_changes" => config_changes,
        "halo_interpolation_enabled" => interpolation_enabled,
        _ => return None,
    };
    Some(handler)
}

/// the C string at `address` in the game's memory, up to `longest` bytes
/// (empty if it cannot be read)
pub(crate) fn read_c_string(
    caller: &mut Caller<'_, Record>,
    address: u32,
    longest: usize,
) -> wasmtime::Result<String> {
    with_space(caller, |space, _| {
        let mut bytes = Vec::new();
        for offset in 0..u32::try_from(longest).unwrap_or(u32::MAX) {
            match space
                .read(address.wrapping_add(offset), 1)
                .and_then(|byte| byte.first().copied())
            {
                Some(0) | None => break,
                Some(byte) => bytes.push(byte),
            }
        }
        String::from_utf8_lossy(&bytes).into_owned()
    })
}

/// writes `text` into the game's buffer of `size` at `buffer`, cut to fit
/// and ending in its NUL
fn write_text(
    caller: &mut Caller<'_, Record>,
    buffer: u32,
    size: u32,
    text: &str,
) -> wasmtime::Result<()> {
    let Some(room) = usize::try_from(size)
        .ok()
        .and_then(|size| size.checked_sub(1))
    else {
        return Ok(());
    };
    let mut bytes: Vec<u8> = text.bytes().take(room).collect();
    bytes.push(0);
    with_space(caller, |space, _| {
        space.write(buffer, &bytes);
    })
}

/// the name the game passes first
fn name_of(caller: &mut Caller<'_, Record>, params: &[Val]) -> wasmtime::Result<String> {
    read_c_string(caller, arg(params, 0), LONGEST_NAME)
}

/// `config_boolean(name)`: the boolean setting, or 0
fn config_boolean(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let name = name_of(caller, params)?;
    let value = caller.data().settings().value(&name, Kind::Boolean) == Some(&Value::Boolean(true));
    give(results, u32::from(value));
    Ok(())
}

/// `halo_interpolation_enabled()`: `display.interpolation`, which main.c
/// reads to draw between the 30 Hz ticks instead of throttling to them
fn interpolation_enabled(
    caller: &mut Caller<'_, Record>,
    _params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let value = caller
        .data()
        .settings()
        .value("display.interpolation", Kind::Boolean)
        == Some(&Value::Boolean(true));
    give(results, u32::from(value));
    Ok(())
}

/// `config_integer(name)`: the integer setting, or 0
fn config_integer(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let name = name_of(caller, params)?;
    let value = match caller.data().settings().value(&name, Kind::Integer) {
        Some(Value::Integer(integer)) => *integer,
        _ => 0,
    };
    give(results, value.cast_unsigned());
    Ok(())
}

/// `config_real(name)`: the real setting, or 0
fn config_real(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let name = name_of(caller, params)?;
    let value = match caller.data().settings().value(&name, Kind::Real) {
        Some(Value::Real(real)) => *real,
        _ => 0.0,
    };
    if let Some(result) = results.first_mut() {
        *result = Val::F64(value.to_bits());
    }
    Ok(())
}

/// `config_string(name)`: the string setting (or "", for a setting of
/// another type or none), copied into the game's memory once per value and
/// kept there, since its callers hold the pointer
fn config_string(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let name = name_of(caller, params)?;
    let (text, copy) = {
        let settings = caller.data().settings();
        match settings
            .settings
            .get(&name)
            .filter(|setting| setting.kind == Kind::Text)
        {
            Some(Setting {
                value: Value::Text(text),
                copy,
                ..
            }) => (text.clone(), *copy),
            _ => (String::new(), None),
        }
    };
    if let Some(copy) = copy {
        give(results, copy);
        return Ok(());
    }
    let mut bytes = text.into_bytes();
    bytes.push(0);
    let size = u32::try_from(bytes.len()).unwrap_or(u32::MAX);
    let copy = call_game(caller, "malloc", &[Val::I32(size.cast_signed())])?;
    if copy == 0 {
        return Err(wasmtime::format_err!(
            "the game's malloc has no room for the setting {name}"
        ));
    }
    with_space(caller, |space, _| {
        space.write(copy, &bytes);
    })?;
    if let Some(setting) = caller.data().settings().settings.get_mut(&name) {
        if setting.kind == Kind::Text {
            setting.copy = Some(copy);
        }
    }
    give(results, copy);
    Ok(())
}

/// `config_text(name, buffer, size)`: the setting as text, and 1; 0 if
/// there is no such setting
fn config_text(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let name = name_of(caller, params)?;
    let text = caller
        .data()
        .settings()
        .settings
        .get(&name)
        .map(|setting| text_of(&setting.value));
    answer_text(caller, params, results, text)
}

/// `config_default(name, buffer, size)`: the setting's default as text
/// (a string's without its quotes), and 1; 0 if there is no such setting
fn config_default(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let name = name_of(caller, params)?;
    let text = caller
        .data()
        .settings()
        .settings
        .get(&name)
        .map(|setting| setting.default.clone());
    answer_text(caller, params, results, text)
}

/// writes `text`, if there is one, into the buffer the game passes second
/// and third
fn answer_text(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
    text: Option<String>,
) -> wasmtime::Result<()> {
    match text {
        Some(text) => {
            write_text(caller, arg(params, 1), arg(params, 2), &text)?;
            give(results, 1);
        }
        None => give(results, 0),
    }
    Ok(())
}

/// `config_write(name, value)`: sets the setting from the text `value`;
/// 1 if it did (no such setting, a name without a section or a value longer
/// than 256 is refused)
fn config_write(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let name = name_of(caller, params)?;
    let text = read_c_string(caller, arg(params, 1), LONGEST_VALUE + 1)?;
    let mut settings = caller.data().settings();
    let written = settings.write(&name, &text);
    if written && !settings.keep(&name, &text) {
        std::io::Write::write_all(
            &mut std::io::stderr(),
            b"settings: cannot keep the settings\n",
        )
        .ok();
    }
    drop(settings);
    give(results, u32::from(written));
    Ok(())
}

/// `config_changes()`: how many times a setting has been written
fn config_changes(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let changes = caller.data().settings().changes;
    give(results, changes);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_games_writes_are_kept_and_read_back_over_the_defaults() {
        let folder = std::env::temp_dir().join(format!("opence-kept-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&folder);
        let file = folder.join(KEPT);
        let _ = std::fs::remove_file(&file);
        let mut first = Settings::new(|_| None);
        first.keep_in(&file);
        assert!(first.write("display.vsync", "false"));
        assert!(first.keep("display.vsync", "false"));
        assert!(first.write("display.window_size", "1920x1080"));
        assert!(first.keep("display.window_size", "1920x1080"));
        // (a setting set for the run alone is not kept)
        assert!(first.write("display.fullscreen", "false"));

        let mut second = Settings::new(|_| None);
        assert_eq!(second.boolean("display.vsync"), Some(true));
        second.keep_in(&file);
        assert_eq!(second.boolean("display.vsync"), Some(false));
        assert_eq!(second.text("display.window_size"), Some("1920x1080"));
        assert_eq!(second.boolean("display.fullscreen"), Some(true));
        let _ = std::fs::remove_dir_all(&folder);
    }

    #[test]
    fn a_kept_file_s_lines_without_a_tab_or_name_are_passed_over() {
        assert_eq!(
            kept_of("a.b\t1\nnonsense\n\tlost\nc.d\tx\ty\n"),
            vec![("a.b", "1"), ("c.d", "x\ty")]
        );
        let mut kept = BTreeMap::new();
        kept.insert("a.b".to_owned(), "1".to_owned());
        kept.insert("c.d".to_owned(), "two\nlines".to_owned());
        assert_eq!(kept_text(&kept), "a.b\t1\n");
    }

    #[test]
    fn a_setting_s_variable_overrides_its_default_by_its_style() {
        let variables = |name: &str| match name {
            "HALO_TELNET_CONSOLE_PORT" => Some("4000".to_string()),
            "HALO_NO_VSYNC" => Some("1".to_string()),
            "HALO_NET_VOTEKICK" => Some("Off".to_string()),
            _ => None,
        };
        let settings = Settings::new(variables);
        assert_eq!(
            settings.value("debug.telnet_console_port", Kind::Integer),
            Some(&Value::Integer(4000))
        );
        assert_eq!(
            settings.value("display.vsync", Kind::Boolean),
            Some(&Value::Boolean(false))
        );
        assert_eq!(
            settings.value("network.votekick", Kind::Boolean),
            Some(&Value::Boolean(false))
        );
        // a set_is_false variable that reads as false leaves the default
        let settings = Settings::new(|name| (name == "HALO_NO_VSYNC").then(|| "no".to_string()));
        assert_eq!(
            settings.value("display.vsync", Kind::Boolean),
            Some(&Value::Boolean(true))
        );
    }

    #[test]
    fn numbers_read_and_print_as_c_reads_and_prints_them() {
        assert_eq!(integer_of("  42abc"), 42);
        assert_eq!(integer_of("-7"), -7);
        assert_eq!(integer_of("x"), 0);
        assert_eq!(integer_of("99999999999"), i32::MAX);
        assert!((real_of("2.5e1x") - 25.0).abs() < f64::EPSILON);
        assert!(real_of("") == 0.0);
        for (real, text) in [
            (15.0, "15"),
            (1.0, "1"),
            (0.1, "0.1"),
            (-2.5, "-2.5"),
            (1e20, "1e+20"),
            (0.00001, "1e-05"),
            (123_456_789.125, "123456789.125"),
        ] {
            assert_eq!(print_real(real), text);
        }
    }

    #[test]
    fn the_table_has_every_setting() {
        let settings = Settings::new(|_| None);
        assert!(settings.settings.len() > 100);
        assert_eq!(
            settings.value("controls.crouch", Kind::Text),
            Some(&Value::Text("Left Ctrl, C".into()))
        );
    }
}
