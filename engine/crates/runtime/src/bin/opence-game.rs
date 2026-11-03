//! `opence-game --map <scenario> --benchmark <seconds>`: runs game.wasm
//! headless on the map (its draw calls answered without a screen), times
//! the game's own profile sections for `<seconds>` from its first tick, and
//! prints the CPU frame's and the simulation tick's p50, p99 and longest
//! ([`opence_runtime::benchmark`]). The map's load is given `--load-seconds`
//! (120 by default) before the measured time begins.
//!
//! The game is `build/engine/wasm/game.wasm` (`--wasm` names another), its
//! data `assets/` (`--data`), its saves a folder under `build/engine/`, and
//! its screen 3840 by 2160.
//!
//! `opence-game --map <scenario> --control <socket>` runs the game headless
//! with no time limit and its telnet console on, and answers the dev shell's
//! control socket at `<socket>` from that console
//! ([`opence_runtime::control`]): `opence --socket <socket> run <line>` runs
//! the line in the game.

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use opence_runtime::benchmark::{report, sections, Summary};
use opence_runtime::{control, Error, Game, Runtime};

/// the screen the benchmark is drawn for
const DISPLAY: (u32, u32) = (3840, 2160);

struct Options {
    wasm: PathBuf,
    data: PathBuf,
    map: String,
    seconds: u64,
    load_seconds: u64,
    control: Option<PathBuf>,
}

fn options() -> Result<Options, String> {
    let mut options = Options {
        wasm: PathBuf::from("build/engine/wasm/game.wasm"),
        data: PathBuf::from("assets"),
        map: String::new(),
        seconds: 60,
        load_seconds: 120,
        control: None,
    };
    let mut arguments = std::env::args().skip(1);
    while let Some(flag) = arguments.next() {
        let value = arguments
            .next()
            .ok_or_else(|| format!("{flag} needs a value"))?;
        let number = || {
            value
                .parse::<u64>()
                .map_err(|error| format!("{flag} {value}: {error}"))
        };
        match flag.as_str() {
            "--wasm" => options.wasm = PathBuf::from(&value),
            "--data" => options.data = PathBuf::from(&value),
            "--map" => options.map.clone_from(&value),
            "--benchmark" => options.seconds = number()?,
            "--load-seconds" => options.load_seconds = number()?,
            "--control" => options.control = Some(PathBuf::from(&value)),
            _ => return Err(format!("unknown flag {flag}")),
        }
    }
    if options.map.is_empty() {
        return Err("--map names the scenario".to_owned());
    }
    Ok(options)
}

fn line(name: &str, summary: Option<Summary>) -> String {
    let ms = |ns: u64| Duration::from_nanos(ns).as_secs_f64() * 1e3;
    summary.map_or_else(
        || format!("{name}: none measured"),
        |summary| {
            format!(
                "{name}: {} measured, p50 {:.3} ms, p99 {:.3} ms, max {:.3} ms",
                summary.count,
                ms(summary.p50_ns),
                ms(summary.p99_ns),
                ms(summary.max_ns)
            )
        },
    )
}

fn run(options: &Options) -> Result<(), String> {
    let text = |error: Error| error.to_string();
    let wasm = std::fs::read(&options.wasm)
        .map_err(|error| format!("{}: {error}", options.wasm.display()))?;
    let cache = std::env::current_dir()
        .map_err(|error| error.to_string())?
        .join("build/engine/wasm/cache");
    let runtime = Runtime::new(Some(&cache)).map_err(text)?;
    let mut game = runtime.load(&wasm).map_err(text)?;
    let saves = PathBuf::from("build/engine/benchmark_saves");
    std::fs::create_dir_all(&saves).map_err(|error| format!("{}: {error}", saves.display()))?;
    game.prepare(&options.data, &saves, DISPLAY).map_err(text)?;
    game.set_init_commands(&format!("map_name {}\n", options.map));
    if let Some(path) = &options.control {
        return control(game, path);
    }
    game.set_profiling(true);
    let started = Instant::now();
    game.set_deadline(Some(
        started + Duration::from_secs(options.load_seconds + options.seconds),
    ));
    match game.call("halo_game_main") {
        Err(Error::Deadline) => {}
        Ok(()) => return Err("the game returned before the benchmark's time was up".to_owned()),
        Err(error) => return Err(error.to_string()),
    }
    let mut spans = game.take_profile_spans();
    // the measured time: `seconds` from the first tick
    let first_tick = spans
        .iter()
        .filter(|span| span.name == "game_tick.cheats_enforce")
        .map(|span| span.start_ns)
        .min()
        .ok_or("the game never ticked")?;
    let end = first_tick.saturating_add(options.seconds.saturating_mul(1_000_000_000));
    spans.retain(|span| span.end_ns <= end);
    let numbers = report(&spans);
    println!(
        "map {}, {} by {}, {} s from the first tick",
        options.map, DISPLAY.0, DISPLAY.1, options.seconds
    );
    println!(
        "load: {:.3} s",
        Duration::from_nanos(first_tick).as_secs_f64()
    );
    println!(
        "{}",
        line("frame (less its throttle and sleeps)", numbers.frame)
    );
    println!("{}", line("tick", numbers.tick));
    // (from the first tick on: where the frame's time goes)
    spans.retain(|span| span.start_ns >= first_tick);
    for (name, summary) in sections(&spans) {
        println!("  {}", line(&name, Some(summary)));
    }
    Ok(())
}

/// Runs the game until it quits, its console, keyboard and mouse driven
/// through the control socket.
fn control(mut game: Game, path: &Path) -> Result<(), String> {
    if !game.set_setting("debug.telnet_console", "true") {
        return Err("the game has no setting debug.telnet_console".to_owned());
    }
    let listener = control::bind(path).map_err(|error| format!("{}: {error}", path.display()))?;
    control::serve(listener, control::Driven::of(&mut game));
    eprintln!("opence-game: the control socket is {}", path.display());
    let ran = game
        .call("halo_game_main")
        .map_err(|error| error.to_string());
    let _ = std::fs::remove_file(path);
    ran
}

fn main() -> ExitCode {
    let result = options().and_then(|options| run(&options));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("opence-game: {error}");
            ExitCode::from(70)
        }
    }
}
