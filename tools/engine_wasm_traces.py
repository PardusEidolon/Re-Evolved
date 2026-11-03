#!/usr/bin/env python3
"""Checks every reference trace against the wasm build.

    nix develop -c python3 tools/engine_wasm_traces.py [--only NAME] [--json PATH]
    nix develop -c python3 tools/engine_wasm_traces.py --check
    nix develop -c python3 tools/engine_wasm_traces.py --speed FOLDER

Each tools/engine_*_reference.py compiles the functions it traces from
source/ with $CC and checks that their trace is the one recorded natively.
This runs each script's --check with CC set to tools/engine_wasm_cc.py, so
the same functions are built for wasm32 with the game's ABI and run in the
game's runtime (opence-wasm), and reports which traces match. A script that
passes here traces the same in game.wasm as natively.

--check is the gate: every script runs, and it fails if any trace differs,
or if a script has no trace in tools/traces or a trace there no script
(engine_NAME_reference.py records traces/NAME_reference.txt).

--speed also builds each harness natively and times both runs, one script
at a time, and writes FOLDER/speed.json: per script, the native run less an
empty process's, the wasm run alone (opence-wasm's load apart), and their
ratio, with the geometric mean over the scripts whose native run is long
enough to time (SPEED_FLOOR).
"""

import argparse
import json
import math
import os
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Dict, List, Optional

ROOT = Path(__file__).resolve().parent.parent
CARGO_MANIFEST = ROOT / "engine" / "Cargo.toml"
RUNNER = ROOT / "engine" / "target" / "release" / "opence-wasm"
CACHE = ROOT / "build" / "engine" / "wasm" / "cache"
# (the slowest native trace takes a few seconds; a wasm one tens)
TIMEOUT = 600
# a native run (less an empty process's) shorter than this is within the
# timing's noise, too short to give a ratio
SPEED_FLOOR = 0.002


def scripts(only: Optional[str] = None) -> List[Path]:
    """the reference scripts, by name, or the one whose name contains only"""
    found = sorted((ROOT / "tools").glob("engine_*_reference.py"))
    if only:
        found = [script for script in found if only in script.name]
    return found


def unpaired(tools: Path = ROOT / "tools") -> List[str]:
    """each script without its trace in tools/traces, and each trace there
    without its script, as a line saying so"""
    names = {path.name[len("engine_") : -len(".py")] for path in tools.glob("engine_*_reference.py")}
    traces = {path.name[: -len(".txt")] for path in (tools / "traces").glob("*_reference.txt")}
    lines = [f"engine_{name}.py has no trace (traces/{name}.txt)" for name in sorted(names - traces)]
    lines += [f"traces/{name}.txt has no script (engine_{name}.py)" for name in sorted(traces - names)]
    return lines


def build_runner() -> Path:
    subprocess.run(
        [
            "cargo",
            "build",
            "-q",
            "--release",
            "--manifest-path",
            str(CARGO_MANIFEST),
            "-p",
            "opence_runtime",
            "--bin",
            "opence-wasm",
        ],
        check=True,
    )
    return RUNNER


def environment(runner: Path) -> Dict[str, str]:
    CACHE.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ)
    env["CC"] = str(ROOT / "tools" / "engine_wasm_cc.py")
    env["OPENCE_WASM_RUN"] = str(runner)
    env["OPENCE_WASM_CACHE"] = str(CACHE)
    return env


def check(script: Path, env: Dict[str, str]) -> Dict[str, object]:
    start = time.monotonic()
    try:
        result = subprocess.run(
            [sys.executable, str(script), "--check"],
            env=env,
            capture_output=True,
            text=True,
            timeout=TIMEOUT,
        )
        passed, detail = result.returncode == 0, (result.stdout + result.stderr).strip()
    except subprocess.TimeoutExpired:
        passed, detail = False, f"timed out after {TIMEOUT} s"
    return {
        "script": script.name,
        "passed": passed,
        "seconds": round(time.monotonic() - start, 2),
        # (the end of the output says why it failed)
        "detail": detail[-2000:],
    }


def speed_summary(rows: List[Dict[str, object]], floor: float = SPEED_FLOOR) -> Dict[str, object]:
    """per script, the wasm run alone over the native run less its process's
    start, for those whose native run is at least floor seconds"""
    ratios = {}
    for row in rows:
        native = float(row["native"]) - float(row["baseline"])
        if native >= floor:
            ratios[str(row["script"])] = float(row["run"]) / native
    timed = sorted(ratios)
    mean = math.exp(sum(math.log(ratio) for ratio in ratios.values()) / len(ratios)) if ratios else None
    return {"timed": timed, "ratios": ratios, "geometric_mean": mean, "rows": rows}


def main(arguments: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--only", help="run only the scripts whose name contains this")
    parser.add_argument("--json", type=Path, help="write the results here")
    parser.add_argument("--jobs", type=int, help="scripts at once (all the cores; one with --speed)")
    parser.add_argument("--speed", type=Path, help="time the native and wasm runs into this folder")
    parser.add_argument("--check", action="store_true", help="every trace, each paired with its script")
    options = parser.parse_args(arguments)
    if options.check and (options.only or options.speed):
        parser.error("--check runs every trace, without --only or --speed")
    if not os.environ.get("HALO_WASM_BUILTINS"):
        raise SystemExit("engine_wasm_traces: run inside nix develop")
    if options.check:
        lost = unpaired()
        for line in lost:
            print(line)
        if lost:
            return 1
    env = environment(build_runner())
    jobs = options.jobs or (1 if options.speed else os.cpu_count() or 4)
    if options.speed:
        options.speed.mkdir(parents=True, exist_ok=True)
        (options.speed / "speed.jsonl").unlink(missing_ok=True)
        env["OPENCE_WASM_SPEED"] = str(options.speed)
        env["OPENCE_NATIVE_CC"] = os.environ.get("CC", "clang")
    chosen = scripts(options.only)
    with ThreadPoolExecutor(max_workers=jobs) as pool:
        results = list(pool.map(lambda script: check(script, env), chosen))
    for result in results:
        mark = "match" if result["passed"] else "DIFFERS"
        print(f"{mark:8} {result['seconds']:7.2f} s  {result['script']}")
        if not result["passed"]:
            print("         " + str(result["detail"]).splitlines()[-1] if result["detail"] else "")
    passed = sum(1 for result in results if result["passed"])
    print(f"{passed} of {len(results)} traces match in the wasm build")
    if options.json:
        options.json.parent.mkdir(parents=True, exist_ok=True)
        options.json.write_text(json.dumps(results, indent=2) + "\n")
    if options.speed:
        lines = (options.speed / "speed.jsonl").read_text().splitlines()
        summary = speed_summary([json.loads(line) for line in lines])
        (options.speed / "speed.json").write_text(json.dumps(summary, indent=2) + "\n")
        for script, ratio in sorted(summary["ratios"].items(), key=lambda item: item[1]):
            print(f"{ratio:6.2f}x  {script}")
        mean = summary["geometric_mean"]
        print(
            f"wasm over native, geometric mean of {len(summary['timed'])}: "
            + (f"{mean:.2f}x" if mean else "none timed")
        )
    return 0 if passed == len(results) else 1


if __name__ == "__main__":
    sys.exit(main())
