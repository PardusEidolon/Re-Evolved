//! `opence-wasm module.wasm [args...]`: runs a WASI command module in the
//! game's runtime and exits with its code, its descriptors 1 and 2 copied
//! to this process's. The module is given the module's path and `args` as
//! its arguments, and this process's standard input as its own (none when
//! that is a terminal). The reference traces built as wasm32 run through it
//! (tools/engine_wasm_cc.py), so a trace compares the wasm build with the
//! native one through the same runtime the game uses.
//!
//! `OPENCE_WASM_CACHE` names a folder for Wasmtime's cache of compiled
//! modules; without it each module compiles afresh. `OPENCE_WASM_TIMES`
//! names a file to write the load's and the run's nanoseconds to, apart, so
//! the wasm build's speed is measured without the runtime's start.

use std::ffi::OsString;
use std::io::{IsTerminal, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use opence_runtime::Runtime;

fn run(module: &PathBuf, args: &[OsString]) -> Result<i32, String> {
    let wasm = std::fs::read(module).map_err(|error| format!("{}: {error}", module.display()))?;
    let cache = std::env::var_os("OPENCE_WASM_CACHE").map(PathBuf::from);
    let start = Instant::now();
    let runtime = Runtime::new(cache.as_deref()).map_err(|error| error.to_string())?;
    let mut game = runtime.load(&wasm).map_err(|error| error.to_string())?;
    let mut stdin = Vec::new();
    if !std::io::stdin().is_terminal() {
        std::io::stdin()
            .read_to_end(&mut stdin)
            .map_err(|error| format!("reading standard input: {error}"))?;
    }
    let names: Vec<Vec<u8>> = std::iter::once(module.as_os_str())
        .chain(args.iter().map(OsString::as_os_str))
        .map(|arg| arg.as_encoded_bytes().to_vec())
        .collect();
    game.set_command(&names, stdin);
    let loaded = Instant::now();
    let result = game.run_command();
    let ran = Instant::now();
    if let Some(times) = std::env::var_os("OPENCE_WASM_TIMES") {
        let line = format!(
            "{} {}\n",
            loaded.duration_since(start).as_nanos(),
            ran.duration_since(loaded).as_nanos()
        );
        std::fs::write(&times, line)
            .map_err(|error| format!("{}: {error}", PathBuf::from(times).display()))?;
    }
    // (what it wrote before a trap is kept, to show where it stopped)
    let copied = std::io::stdout()
        .write_all(game.output(1))
        .and_then(|()| std::io::stdout().flush())
        .and_then(|()| std::io::stderr().write_all(game.output(2)));
    copied.map_err(|error| format!("writing the output: {error}"))?;
    result.map_err(|error| error.to_string())
}

fn main() -> ExitCode {
    let mut arguments = std::env::args_os().skip(1);
    let Some(module) = arguments.next() else {
        eprintln!("usage: opence-wasm module.wasm [args...]");
        return ExitCode::from(2);
    };
    let args: Vec<OsString> = arguments.collect();
    match run(&PathBuf::from(module), &args) {
        // (an exit code is a byte, as a process's is)
        Ok(code) => ExitCode::from(code.to_le_bytes()[0]),
        Err(error) => {
            eprintln!("opence-wasm: {error}");
            ExitCode::from(70)
        }
    }
}
