"""The map audit: every map of the user's, and every cutscene, played
headless by the app's --audit mode and compared with stock.

  run      plays each map for each variant (shipped: the app's defaults;
           stock and stock2: the original's look and sound, twice, so the
           stock runs' own spread sets the thresholds), and each cutscene
           (a static script called, a dormant one woken, at tick 60);
           each run is recorded into build/engine/audit/<variant>/<run>/
  bench    the 4K benchmark of each map, run alone, into bench/<map>.json
  compare  shipped against stock, run by run: frames by region, the
           sound's loudness and bands, the script events in order; the
           findings over threshold go to findings.tsv
  report   compare.json and the benchmarks as a table, one row a run

The keys and mouse are the audit's fixed schedule (crates/ffi/src/
audit.rs), so the same tick shows the same moment in every variant. A map
cannot be played to its end by a schedule; a run is a fixed length
instead. Everything is written under build/engine/audit and stays on this
machine: the frames are drawn from the user's maps."""

from __future__ import annotations

import argparse
import concurrent.futures
import difflib
import json
import math
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))

from frame_compare import Image, read_png  # noqa: E402

OUT = ROOT / "build" / "engine" / "audit"
APP = ROOT / "build" / "engine" / "OpenCE Engine.app" / "Contents" / "MacOS" / "OpenCEEngine"
MAPS = ROOT / "assets" / "maps"
CAMPAIGN = ["a10", "a30", "a50", "b30", "b40", "c10", "c20", "c40", "d20", "d40"]
MULTIPLAYER = [
    "beavercreek",
    "bloodgulch",
    "boardingaction",
    "carousel",
    "chillout",
    "damnation",
    "hangemhigh",
    "longest",
    "prisoner",
    "putput",
    "ratrace",
    "sidewinder",
    "wizard",
]
# the original's look and sound: each of the app's additions switched off
STOCK = [
    "display.interpolation=false",
    "display.high_res_hud=false",
    "display.high_res_text=false",
    "display.per_pixel_lighting=false",
    "display.redraw_menus=false",
    "audio.reverb=false",
    "game.enhanced_animations=false",
]
VARIANTS = {"shipped": [], "stock": STOCK, "stock2": STOCK}
# every run's: the game's clocks step a 60th of a second a frame shown,
# whatever the machine's pace, so that two runs of a variant repeat
EVERY_RUN = ["debug.frame_clock=true"]
# the measures, in the order findings are listed
MEASURES = ["region", "dbfs", "band", "shift", "missing", "extra"]
# the least each threshold may be: a region's mean difference out of 255,
# decibels, ticks, events
FLOORS = {"region": 4.0, "dbfs": 1.0, "band": 2.0, "shift": 2, "missing": 0, "extra": 0}
SOUND_FLOOR = -119.9
CUTSCENE_TICK = 60


def cutscenes(listing: str) -> list[str]:
    """the console lines that play each cutscene of `opence scripts`'
    listing: a static script is called by name and a dormant one woken;
    startup and continuous scripts already run in the map's own run"""
    lines = []
    for row in listing.splitlines():
        name, _, kind = row.partition("\t")
        if not ("cutscene" in name or "cinematic" in name) or name.startswith("cinematic_skip_"):
            continue
        if kind == "static":
            lines.append(name)
        elif kind == "dormant":
            lines.append(f"wake {name}")
    return lines


def parse_tsv(text: str) -> list[dict[str, str]]:
    """a tab-separated table as one dict a row, keyed by its header"""
    rows = text.splitlines()
    if not rows:
        return []
    header = rows[0].split("\t")
    return [dict(zip(header, row.split("\t"))) for row in rows[1:] if row]


def read_tsv(path: Path) -> list[dict[str, str]]:
    return parse_tsv(path.read_text()) if path.is_file() else []


def region_differences(a: Image, b: Image, grid: int = 4) -> list[float]:
    """the mean difference of red, green and blue, out of 255, in each of
    grid x grid regions, row by row from the top left"""
    if (a.width, a.height) != (b.width, b.height):
        return [255.0] * (grid * grid)
    totals = [0] * (grid * grid)
    counts = [0] * (grid * grid)
    for y in range(a.height):
        row = y * grid // a.height
        start = y * a.width * 4
        for column in range(grid):
            x0 = column * a.width // grid
            x1 = (column + 1) * a.width // grid
            left = a.pixels[start + x0 * 4 : start + x1 * 4]
            right = b.pixels[start + x0 * 4 : start + x1 * 4]
            # every fourth byte is alpha, which is left out
            total = sum(abs(p - q) for index, (p, q) in enumerate(zip(left, right)) if index % 4 != 3)
            totals[row * grid + column] += total
            counts[row * grid + column] += (x1 - x0) * 3
    return [total / count if count else 0.0 for total, count in zip(totals, counts)]


def sound_difference(a: list[dict[str, str]], b: list[dict[str, str]]) -> dict:
    """the largest difference in loudness and in any octave band over the
    seconds both runs heard; a second silent in both is left out"""
    by_second = {row["second"]: row for row in b}
    seconds = 0
    dbfs = band = 0.0
    dbfs_at = band_at = None
    for row in a:
        other = by_second.get(row["second"])
        if other is None:
            continue
        seconds += 1
        if float(row["dbfs"]) <= SOUND_FLOOR and float(other["dbfs"]) <= SOUND_FLOOR:
            continue
        second = int(float(row["second"]))
        difference = abs(float(row["dbfs"]) - float(other["dbfs"]))
        if dbfs_at is None or difference > dbfs:
            dbfs, dbfs_at = difference, second
        for key in row:
            if key.startswith("b") and key in other:
                difference = abs(float(row[key]) - float(other[key]))
                if band_at is None or difference > band:
                    band, band_at = difference, second
    return {"seconds": seconds, "dbfs": round(dbfs, 2), "band": round(band, 2), "dbfs_at": dbfs_at, "band_at": band_at}


def event_difference(a: list[dict[str, str]], b: list[dict[str, str]]) -> dict:
    """the script events b lacks or adds, in order, and the most ticks a
    matched event moved"""
    key = lambda row: (row["event"], row["script"], row["type"])  # noqa: E731
    left = [key(row) for row in a]
    right = [key(row) for row in b]
    matcher = difflib.SequenceMatcher(a=left, b=right, autojunk=False)
    matched = 0
    shift = 0
    for block in matcher.get_matching_blocks():
        for offset in range(block.size):
            matched += 1
            moved = abs(int(a[block.a + offset]["tick"]) - int(b[block.b + offset]["tick"]))
            shift = max(shift, moved)
    return {"missing": len(left) - matched, "extra": len(right) - matched, "shift": shift}


def state_difference(a: list[dict[str, str]], b: list[dict[str, str]]) -> dict:
    """the first tick two runs' game states differ at (state.tsv), and the
    data arrays that differ then: none named where only the state outside
    them does"""
    ticks = 0
    for row, other in zip(a, b):
        ticks += 1
        if row.get("state") != other.get("state"):
            arrays = [key for key in row if key not in ("tick", "state") and row[key] != other.get(key)]
            return {"parted": int(row["tick"]), "arrays": arrays, "ticks": ticks}
    return {"parted": None, "arrays": [], "ticks": ticks}


def percentile(values: list[float], fraction: float) -> float:
    """the nearest-rank percentile of values, sorted"""
    if not values:
        return 0.0
    return values[max(0, math.ceil(fraction * len(values)) - 1)]


def frame_stats(times: list[float], skip: int = 60) -> dict:
    """frame times in milliseconds, the first `skip` (the load) left out"""
    kept = sorted(times[skip:])
    if not kept:
        return {"frames": 0, "p50": 0.0, "p99": 0.0, "max": 0.0}
    return {"frames": len(kept), "p50": percentile(kept, 0.5), "p99": percentile(kept, 0.99), "max": kept[-1]}


def thresholds(noise: dict) -> dict:
    """each measure's limit: twice what two stock runs differ by, and no
    less than its floor"""
    return {measure: max(type(FLOORS[measure])(noise.get(measure, 0) * 2), FLOORS[measure]) for measure in MEASURES}


def where(measure: str, metrics: dict) -> str:
    """where a run's measure was worst: a frame's tick and region, or the
    sound's second"""
    worst = metrics.get("worst")
    if measure == "region" and worst:
        return f"tick {int(worst['frame'].removeprefix('tick_').removesuffix('.png'))}, region {worst['region']}"
    at = metrics.get(f"{measure}_at")
    return "" if at is None else f"second {at}"


def findings(name: str, variant: str, metrics: dict, limits: dict) -> list[dict]:
    """each measure of a run that is over its limit, and where"""
    return [
        {
            "map": name,
            "variant": variant,
            "measure": measure,
            "value": metrics[measure],
            "limit": limits[measure],
            "at": where(measure, metrics),
        }
        for measure in MEASURES
        if measure in metrics and metrics[measure] > limits[measure]
    ]


def compare(a: Path, b: Path) -> dict:
    """two recorded runs compared: the worst region of the frames both
    kept, the sound and the script events"""
    region = 0.0
    worst = None
    frames = 0
    for frame in sorted((a / "frames").glob("tick_*.png")):
        other = b / "frames" / frame.name
        if not other.is_file():
            continue
        frames += 1
        regions = region_differences(read_png(frame), read_png(other))
        if max(regions) > region:
            region = max(regions)
            worst = {"frame": frame.name, "region": regions.index(region)}
    sound = sound_difference(read_tsv(a / "sound.tsv"), read_tsv(b / "sound.tsv"))
    events = event_difference(read_tsv(a / "events.tsv"), read_tsv(b / "events.tsv"))
    return {"frames": frames, "region": round(region, 2), "worst": worst, **sound, **events}


def run_name(name: str, line: str | None) -> str:
    return name if line is None else f"{name}__{line.replace('wake ', '').replace(' ', '_')}"


def scripts_listing(name: str) -> str:
    """`opence scripts` for a map, or nothing when its scripts do not run"""
    done = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "--release",
            "-p",
            "opence_cli",
            "--bin",
            "opence",
            "--",
            "scripts",
            str(MAPS / f"{name}.map"),
        ],
        cwd=ROOT / "engine",
        capture_output=True,
        text=True,
    )
    return done.stdout if done.returncode == 0 else ""


def audit_command(folder: Path, name: str, variant: str, line: str | None, seconds: int) -> list[str]:
    """the app's command line for a run of `name` as `variant`, with the
    console `line` (a cutscene) at CUTSCENE_TICK"""
    command = [str(APP), "--audit", str(folder), "--map", name, "--seconds", str(seconds)]
    for setting in EVERY_RUN + VARIANTS[variant]:
        command += ["--setting", setting]
    if line is not None:
        command += ["--at", f"{CUTSCENE_TICK}:{line}"]
    return command


def empty_folder(folder: Path) -> None:
    """a run's folder made empty for it: what the last run kept (a frame at
    another tick) would otherwise be compared as this run's"""
    shutil.rmtree(folder, ignore_errors=True)
    folder.mkdir(parents=True)


def play(name: str, variant: str, line: str | None, seconds: int, again: bool) -> tuple[str, int]:
    folder = OUT / variant / run_name(name, line)
    if (folder / "summary.json").is_file() and not again:
        return str(folder), 0
    empty_folder(folder)
    command = audit_command(folder, name, variant, line, seconds)
    with open(folder / "app.log", "w") as log:
        done = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT)
    return str(folder), done.returncode


def present(names: list[str]) -> list[str]:
    return [name for name in names if (MAPS / f"{name}.map").is_file()]


def command_run(arguments: argparse.Namespace) -> int:
    names = present(arguments.maps or CAMPAIGN + MULTIPLAYER)
    if not APP.is_file() or not names:
        print("needs the app (tools/engine_build.py) and assets/maps", file=sys.stderr)
        return 2
    jobs = []
    for name in names:
        seconds = arguments.seconds or (120 if name in CAMPAIGN else 40)
        lines: list[str | None] = [None]
        if arguments.cutscenes and name in CAMPAIGN:
            lines += cutscenes(scripts_listing(name))
        for line in lines:
            for variant in arguments.variants:
                jobs.append((name, variant, line, seconds if line is None else arguments.cutscene_seconds))
    failed = 0
    with concurrent.futures.ThreadPoolExecutor(arguments.jobs) as pool:
        futures = [pool.submit(play, *job, arguments.again) for job in jobs]
        for future in concurrent.futures.as_completed(futures):
            folder, code = future.result()
            failed += code != 0
            print(f"{code}\t{Path(folder).relative_to(OUT)}", flush=True)
    print(f"{len(jobs)} runs, {failed} failed")
    return 1 if failed else 0


def command_bench(arguments: argparse.Namespace) -> int:
    (OUT / "bench").mkdir(parents=True, exist_ok=True)
    failed = 0
    for name in present(arguments.maps or CAMPAIGN + MULTIPLAYER):
        done = subprocess.run(
            [str(APP), "--benchmark", str(arguments.seconds), "--map", name], cwd=ROOT, capture_output=True, text=True
        )
        report = done.stdout[done.stdout.find("{") :] if "{" in done.stdout else ""
        (OUT / "bench" / f"{name}.json").write_text(report or json.dumps({"error": done.stderr[-2000:]}))
        failed += done.returncode != 0
        print(f"{done.returncode}\t{name}", flush=True)
    return 1 if failed else 0


def command_compare(_: argparse.Namespace) -> int:
    runs = sorted(path.name for path in (OUT / "shipped").glob("*") if (path / "summary.json").is_file())
    # each run's own limits, from its two stock runs' spread
    noise: dict = {}
    limits: dict = {}
    results = {}
    found: list[dict] = []
    for run in runs:
        if not (OUT / "stock" / run / "summary.json").is_file():
            continue
        if (OUT / "stock2" / run / "summary.json").is_file():
            spread = compare(OUT / "stock" / run, OUT / "stock2" / run)
            noise[run] = {measure: spread[measure] for measure in MEASURES}
        limits[run] = thresholds(noise.get(run, {}))
        metrics = compare(OUT / "stock" / run, OUT / "shipped" / run)
        summary = json.loads((OUT / "shipped" / run / "summary.json").read_text())
        times = [float(line) for line in (OUT / "shipped" / run / "frame_ms.tsv").read_text().split()]
        metrics.update(ticks=summary["ticks"], refused=summary["refused"], error=summary["error"])
        metrics.update(frame_stats(times))
        results[run] = metrics
        found += findings(run, "shipped", metrics, limits[run])
        if summary["error"] or summary["refused"]:
            found.append(
                {"map": run, "variant": "shipped", "measure": "fault", "value": summary["refused"], "limit": 0}
            )
    (OUT / "compare.json").write_text(json.dumps({"noise": noise, "limits": limits, "runs": results}, indent=1))
    with open(OUT / "findings.tsv", "w") as table:
        table.write("map\tvariant\tmeasure\tvalue\tlimit\tat\n")
        for finding in found:
            keys = ["map", "variant", "measure", "value", "limit", "at"]
            table.write("\t".join(str(finding.get(key, "")) for key in keys) + "\n")
    print(f"{len(results)} runs compared, {len(found)} findings")
    return 0


def command_report(_: argparse.Namespace) -> int:
    data = json.loads((OUT / "compare.json").read_text())
    rows = [
        "| run | ticks | region | dB | band | events ± | p99 640 ms | p99 4K ms |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for run, metrics in data["runs"].items():
        bench = OUT / "bench" / f"{run.split('__')[0]}.json"
        p99 = ""
        if bench.is_file() and "__" not in run:
            report = json.loads(bench.read_text() or "{}")
            p99 = str(report.get("p99_ms", report.get("p99", "")))
        events = f"-{metrics['missing']}/+{metrics['extra']} ({metrics['shift']}t)"
        rows.append(
            f"| {run} | {metrics['ticks']} | {metrics['region']} | {metrics['dbfs']} | {metrics['band']} "
            f"| {events} | {metrics['p99']:.2f} | {p99} |"
        )
    print("\n".join(rows))
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    commands = parser.add_subparsers(dest="command", required=True)
    run = commands.add_parser("run")
    run.add_argument("--maps", nargs="*")
    run.add_argument("--variants", nargs="*", default=list(VARIANTS))
    run.add_argument("--seconds", type=int)
    run.add_argument("--cutscenes", action="store_true")
    run.add_argument("--cutscene-seconds", type=int, default=150)
    run.add_argument("--jobs", type=int, default=3)
    run.add_argument("--again", action="store_true")
    run.set_defaults(handler=command_run)
    bench = commands.add_parser("bench")
    bench.add_argument("--maps", nargs="*")
    bench.add_argument("--seconds", type=int, default=30)
    bench.set_defaults(handler=command_bench)
    commands.add_parser("compare").set_defaults(handler=command_compare)
    commands.add_parser("report").set_defaults(handler=command_report)
    arguments = parser.parse_args()
    return arguments.handler(arguments)


if __name__ == "__main__":
    sys.exit(main())
