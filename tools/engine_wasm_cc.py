#!/usr/bin/env python3
"""A C compiler for the reference traces that builds them as wasm32 and runs
them in the game's runtime (opence_runtime), so that a trace recorded from
the wasm build can be compared with the native one.

    CC="$PWD/tools/engine_wasm_cc.py" python3 tools/engine_foo_reference.py --check

The reference scripts (tools/engine_*_reference.py) compile their harness
with $CC, as `$CC <flags> source.c -o executable -lm`, and run the
executable. This compiles the same source for wasm32-wasip1 with the game's
ABI flags (tools/engine_wasm.py) into executable.wasm, and writes
executable as a script that runs it with the runtime's `opence-wasm`
($OPENCE_WASM_RUN), whose output is the harness's own. A script whose
--check passes this way has the same trace in the wasm build as natively.

With $OPENCE_WASM_SPEED naming a folder, it also builds the harness
natively ($OPENCE_NATIVE_CC, clang by default) as executable.native, and
the executable times both runs (and an empty process, the baseline) into
the folder's speed.jsonl, the wasm run's load and run apart as opence-wasm
reports them; tools/engine_wasm_traces.py --speed sums them up.
"""

import json
import os
import shlex
import subprocess
import sys
from pathlib import Path
from typing import List, Sequence

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_wasm  # noqa: E402

# the host compiler's flags a wasm build cannot take
HOST_ONLY = {"-lm", "-pthread", "-lpthread"}

# the harnesses' target: a WASI command, single-threaded (the functions they
# trace run on one thread, and the runner gives a command its own memory)
TARGET = "wasm32-wasip1"

# the game's ABI less its optimisation and debug levels, which the
# reference keeps as it asked, and its threads
ABI_FLAGS = [
    f"--target={TARGET}" if flag.startswith("--target=") else flag
    for flag in engine_wasm.ABI_FLAGS
    if not flag.startswith(("-O", "-g")) and flag != "-pthread"
]

# the diagnostics the game's build keeps as warnings, which clang 22 makes
# errors; they change no code, and the harnesses build natively with them as
# warnings (under -w)
DIAGNOSTIC_FLAGS = [flag for flag in engine_wasm.GAME_FLAGS if flag.startswith("-Wno-error=")]


def wasm_command(arguments: Sequence[str], output: Path) -> List[str]:
    """the clang command that builds the harness, with its own flags and the
    game's ABI, as a WASI command module at output"""
    include, lib = engine_wasm.wasi_paths(TARGET)
    builtins = os.environ.get("HALO_WASM_BUILTINS")
    if not builtins:
        raise SystemExit("engine_wasm_cc: run inside nix develop (HALO_WASM_BUILTINS is unset)")
    kept = [argument for argument in arguments if argument not in HOST_ONLY]
    return [
        engine_wasm.clang(),
        *ABI_FLAGS,
        "-isystem",
        str(include),
        *kept,
        *DIAGNOSTIC_FLAGS,
        # (as engine_wasm.link: the unwrapped clang finds neither by itself)
        "-nostdlib",
        str(lib / "crt1-command.o"),
        f"-L{lib}",
        "-Wl,-z,stack-size=8388608",
        "-lc",
        builtins,
        "-o",
        str(output),
    ]


def runner() -> str:
    run = os.environ.get("OPENCE_WASM_RUN")
    if not run:
        raise SystemExit("engine_wasm_cc: OPENCE_WASM_RUN names no runner (tools/engine_wasm_traces.py sets it)")
    return run


# the executable in the speed mode: the native run, an empty process and the
# wasm run, each the fastest of REPEATS after one run unmeasured (macOS checks
# a newly linked binary on its first run, a quarter of a second), as one line
# of speed.jsonl; the output is the wasm run's, as without it
TIMER = """\
import json, os, subprocess, sys, time
native, runner, module, folder, script = {arguments}
REPEATS = 5
def timed(command, **options):
    start = time.perf_counter()
    result = subprocess.run(command, **options)
    return time.perf_counter() - start, result
def fastest(command, **options):
    timed(command, **options)
    return min(timed(command, **options)[0] for _ in range(REPEATS))
native_time = fastest([native], stdout=subprocess.DEVNULL, check=True)
baseline = fastest(["/usr/bin/true"])
times = os.path.join(folder, script + ".times")
loads, runs, walls = [], [], []
for _ in range(REPEATS):
    wall, result = timed([runner, module], stdout=subprocess.DEVNULL, env=dict(os.environ, OPENCE_WASM_TIMES=times))
    with open(times) as file:
        load, run = (int(value) / 1e9 for value in file.read().split())
    os.remove(times)
    loads.append(load); runs.append(run); walls.append(wall)
with open(os.path.join(folder, "speed.jsonl"), "a") as file:
    file.write(json.dumps(dict(script=script, native=native_time, baseline=baseline,
                               wasm=min(walls), load=min(loads), run=min(runs))) + "\\n")
# the output the reference compares, from one more run
sys.exit(subprocess.run([runner, module]).returncode)
"""


def speed_executable(native: Path, module: Path, folder: Path, script: str) -> str:
    arguments = [str(native), runner(), str(module), str(folder), script]
    return f"#!{sys.executable}\n" + TIMER.format(arguments=json.dumps(arguments))


def main(arguments: Sequence[str]) -> int:
    arguments = list(arguments)
    if "-o" not in arguments or arguments.index("-o") + 1 >= len(arguments):
        raise SystemExit("engine_wasm_cc: no -o executable")
    at = arguments.index("-o")
    executable = Path(arguments[at + 1])
    rest = arguments[:at] + arguments[at + 2 :]
    module = executable.with_suffix(".wasm")
    result = subprocess.run(wasm_command(rest, module))
    if result.returncode:
        return result.returncode
    speed = os.environ.get("OPENCE_WASM_SPEED")
    if speed:
        native = executable.with_suffix(".native")
        compiler = os.environ.get("OPENCE_NATIVE_CC", "clang")
        result = subprocess.run([compiler, *arguments[:at], "-o", str(native), *arguments[at + 2 :]])
        if result.returncode:
            return result.returncode
        # (the reference's temporary folder is named for its script)
        executable.write_text(speed_executable(native, module, Path(speed), executable.parent.name))
    else:
        executable.write_text(f"#!/bin/sh\nexec {shlex.quote(runner())} {shlex.quote(str(module))} \"$@\"\n")
    executable.chmod(0o755)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
