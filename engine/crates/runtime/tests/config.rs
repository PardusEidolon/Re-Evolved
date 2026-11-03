//! The game's settings as the platform answers them (`opence_runtime`'s
//! config module): each setting of the native ports' table
//! (`port/linux/src/port_config.c`, copied by `tools/engine_settings.py`)
//! by name and type, as text and as its default, and written.

use opence_runtime::{answers, Error, Runtime, Val};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// the module: each import re-exported, and a malloc that bumps from 8192
const SETTINGS: &str = r#"
(module
  (import "env" "config_boolean" (func $boolean (param i32) (result i32)))
  (import "env" "config_integer" (func $integer (param i32) (result i32)))
  (import "env" "config_real" (func $real (param i32) (result f64)))
  (import "env" "config_string" (func $string (param i32) (result i32)))
  (import "env" "config_text" (func $text (param i32 i32 i32) (result i32)))
  (import "env" "config_default" (func $default (param i32 i32 i32) (result i32)))
  (import "env" "config_write" (func $write (param i32 i32) (result i32)))
  (import "env" "config_changes" (func $changes (result i32)))
  (import "env" "halo_interpolation_enabled" (func $interpolation (result i32)))
  (memory (export "memory") 1)
  (global $next (mut i32) (i32.const 8192))
  (func (export "malloc") (param i32) (result i32)
    (global.get $next)
    (global.set $next (i32.add (global.get $next) (i32.and (i32.add (local.get 0) (i32.const 15)) (i32.const -16)))))
  (func (export "boolean") (param i32) (result i32) (call $boolean (local.get 0)))
  (func (export "integer") (param i32) (result i32) (call $integer (local.get 0)))
  (func (export "real") (param i32) (result f64) (call $real (local.get 0)))
  (func (export "string") (param i32) (result i32) (call $string (local.get 0)))
  (func (export "text") (param i32 i32 i32) (result i32) (call $text (local.get 0) (local.get 1) (local.get 2)))
  (func (export "default") (param i32 i32 i32) (result i32) (call $default (local.get 0) (local.get 1) (local.get 2)))
  (func (export "write") (param i32 i32) (result i32) (call $write (local.get 0) (local.get 1)))
  (func (export "changes") (result i32) (call $changes))
  (func (export "interpolation") (result i32) (call $interpolation)))
"#;

/// where the test writes a setting's name, a value, and the text it reads
const NAME: u32 = 256;
const VALUE: u32 = 1024;
const TEXT: u32 = 2048;

fn first(results: &[Val]) -> Option<i32> {
    results.first().and_then(Val::i32)
}

struct Settings {
    game: opence_runtime::Game,
}

impl Settings {
    fn new() -> Result<Self, Error> {
        let runtime = Runtime::new(None)?;
        Ok(Self {
            game: runtime.load(&module(SETTINGS))?,
        })
    }

    /// `export(name)`, with the name written as a C string first
    fn ask(&mut self, export: &str, name: &str, more: &[Val]) -> Result<Vec<Val>, Error> {
        let mut bytes = name.as_bytes().to_vec();
        bytes.push(0);
        self.game.write(u64::from(NAME), &bytes)?;
        let mut args = vec![Val::I32(NAME.cast_signed())];
        args.extend_from_slice(more);
        self.game.call_with(export, &args)
    }

    fn number(&mut self, export: &str, name: &str) -> Result<Option<i32>, Error> {
        Ok(first(&self.ask(export, name, &[])?))
    }

    /// the C string at `address`
    fn c_string(&self, address: u32) -> Result<String, Error> {
        let bytes = self.game.read(u64::from(address), 300)?;
        let end = bytes
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(bytes.len());
        Ok(String::from_utf8_lossy(bytes.get(..end).unwrap_or_default()).into_owned())
    }

    /// `config_text` or `config_default` into a buffer of `size`, and what
    /// it wrote
    fn text(
        &mut self,
        export: &str,
        name: &str,
        size: u32,
    ) -> Result<(Option<i32>, String), Error> {
        self.game.write(u64::from(TEXT), &[0xAA; 300])?;
        let size = Val::I32(size.cast_signed());
        let result = first(&self.ask(export, name, &[Val::I32(TEXT.cast_signed()), size])?);
        Ok((result, self.c_string(TEXT)?))
    }

    fn write(&mut self, name: &str, value: &str) -> Result<Option<i32>, Error> {
        let mut bytes = value.as_bytes().to_vec();
        bytes.push(0);
        self.game.write(u64::from(VALUE), &bytes)?;
        Ok(first(&self.ask(
            "write",
            name,
            &[Val::I32(VALUE.cast_signed())],
        )?))
    }
}

#[test]
fn each_type_of_setting_is_its_default() -> Result<(), Error> {
    let mut settings = Settings::new()?;
    assert_eq!(settings.number("boolean", "network.votekick")?, Some(1));
    assert_eq!(
        settings.number("boolean", "debug.placement_trace")?,
        Some(0)
    );
    assert_eq!(
        settings.number("integer", "debug.telnet_console_port")?,
        Some(2323)
    );
    assert_eq!(
        settings.number("integer", "debug.gpu_trace_frame")?,
        Some(-1)
    );
    let real = settings.ask("real", "network.voice_proximity", &[])?;
    assert_eq!(real.first().and_then(Val::f64), Some(15.0));
    let string = settings
        .number("string", "network.brokers_file")?
        .unwrap_or_default();
    assert_eq!(settings.c_string(string.cast_unsigned())?, "brokers.txt");
    // the same string each time: its callers keep the pointer
    assert_eq!(
        settings.number("string", "network.brokers_file")?,
        Some(string)
    );
    Ok(())
}

#[test]
fn a_setting_of_another_type_or_none_is_zero_or_empty() -> Result<(), Error> {
    let mut settings = Settings::new()?;
    assert_eq!(settings.number("integer", "network.votekick")?, Some(0));
    assert_eq!(settings.number("boolean", "no.such_setting")?, Some(0));
    let real = settings.ask("real", "network.votekick", &[])?;
    assert_eq!(real.first().and_then(Val::f64), Some(0.0));
    let string = settings
        .number("string", "debug.telnet_console_port")?
        .unwrap_or_default();
    assert_ne!(string, 0);
    assert_eq!(settings.c_string(string.cast_unsigned())?, "");
    Ok(())
}

#[test]
fn a_setting_reads_as_text_as_the_native_ports_print_it() -> Result<(), Error> {
    let mut settings = Settings::new()?;
    assert_eq!(
        settings.text("text", "network.votekick", 64)?,
        (Some(1), "true".into())
    );
    assert_eq!(
        settings.text("text", "debug.gpu_trace_frame", 64)?,
        (Some(1), "-1".into())
    );
    assert_eq!(
        settings.text("text", "network.voice_proximity", 64)?,
        (Some(1), "15".into())
    );
    assert_eq!(
        settings.text("text", "display.player_name_scale", 64)?,
        (Some(1), "1".into())
    );
    assert_eq!(
        settings.text("text", "controls.melee", 64)?,
        (Some(1), "F, Mouse 4".into())
    );
    // cut to the buffer, which ends in its NUL
    assert_eq!(
        settings.text("text", "controls.melee", 4)?,
        (Some(1), "F, ".into())
    );
    assert_eq!(settings.text("text", "no.such_setting", 64)?.0, Some(0));
    Ok(())
}

#[test]
fn a_setting_s_default_is_its_text_without_quotes() -> Result<(), Error> {
    let mut settings = Settings::new()?;
    assert_eq!(
        settings.text("default", "controls.crouch", 64)?,
        (Some(1), "Left Ctrl, C".into())
    );
    assert_eq!(
        settings.text("default", "display.player_name_scale", 64)?,
        (Some(1), "1.0".into())
    );
    assert_eq!(settings.text("default", "no.such_setting", 64)?.0, Some(0));
    Ok(())
}

#[test]
fn writing_a_setting_changes_it_and_counts_the_change() -> Result<(), Error> {
    let mut settings = Settings::new()?;
    assert_eq!(first(&settings.game.call_with("changes", &[])?), Some(0));
    assert_eq!(
        settings.write("debug.telnet_console_port", "4242")?,
        Some(1)
    );
    assert_eq!(
        settings.number("integer", "debug.telnet_console_port")?,
        Some(4242)
    );
    let before = settings
        .number("string", "network.brokers_file")?
        .unwrap_or_default();
    assert_eq!(settings.write("network.brokers_file", "mine.txt")?, Some(1));
    let after = settings
        .number("string", "network.brokers_file")?
        .unwrap_or_default();
    assert_ne!(after, before);
    assert_eq!(settings.c_string(after.cast_unsigned())?, "mine.txt");
    // the old string stays, for whoever holds it
    assert_eq!(settings.c_string(before.cast_unsigned())?, "brokers.txt");
    assert_eq!(settings.write("network.votekick", "off")?, Some(1));
    assert_eq!(settings.number("boolean", "network.votekick")?, Some(0));
    assert_eq!(first(&settings.game.call_with("changes", &[])?), Some(3));
    // no such setting, and a value longer than 256, are refused
    assert_eq!(settings.write("no.such_setting", "1")?, Some(0));
    assert_eq!(settings.write("controls.melee", &"F".repeat(257))?, Some(0));
    assert_eq!(first(&settings.game.call_with("changes", &[])?), Some(3));
    Ok(())
}

#[test]
fn the_platform_answers_the_settings() {
    for name in [
        "env.config_boolean",
        "env.config_integer",
        "env.config_real",
        "env.config_string",
        "env.config_text",
        "env.config_default",
        "env.config_write",
        "env.config_changes",
    ] {
        assert!(answers(name), "{name}");
    }
    assert!(!answers("env.config_folder"));
}

#[test]
fn the_main_menus_build_number_is_hidden_by_default() -> Result<(), Error> {
    // set_textbox_to_build_number writes the original's "01.01.14.2342"
    // only while the setting is on
    let mut settings = Settings::new()?;
    assert_eq!(
        settings.text("default", "display.build_number", 64)?,
        (Some(1), "false".into())
    );
    assert_eq!(settings.number("boolean", "display.build_number")?, Some(0));
    assert_eq!(settings.write("display.build_number", "true")?, Some(1));
    assert_eq!(settings.number("boolean", "display.build_number")?, Some(1));
    Ok(())
}

#[test]
fn each_lighting_extension_is_switched_to_stock_by_default() -> Result<(), Error> {
    let mut settings = Settings::new()?;
    for name in [
        "lighting.placed_lights",
        "lighting.bounce",
        "lighting.atmosphere",
        "lighting.shadows",
        "lighting.reflections",
    ] {
        assert_eq!(
            settings.text("default", name, 64)?,
            (Some(1), "stock".into()),
            "{name}"
        );
    }
    Ok(())
}

#[test]
fn interpolation_is_answered_from_its_setting() -> Result<(), Error> {
    // main.c throttles to 30 frames a second and render_interpolation stays
    // off while halo_interpolation_enabled answers 0
    let mut settings = Settings::new()?;
    assert!(answers("env.halo_interpolation_enabled"));
    assert_eq!(
        first(&settings.game.call_with("interpolation", &[])?),
        Some(1)
    );
    assert_eq!(settings.write("display.interpolation", "false")?, Some(1));
    assert_eq!(
        first(&settings.game.call_with("interpolation", &[])?),
        Some(0)
    );
    Ok(())
}
