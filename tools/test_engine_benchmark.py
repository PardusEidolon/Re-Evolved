"""The new engine's benchmark: each benchmark path (devgulch, bloodgulch,
and bloodgulch with sixteen players; crates/ffi/src/benchmark.rs) played
and drawn offscreen at 3840x2160 for 60 seconds, with temporal
anti-aliasing at full scale. Each path's report (mean, p50, p99, p99.9,
hitches, pacing) is kept in build/engine/benchmark_<path>.json; a run
fails when it is more than 5% slower than the report kept before, or
hitches more, and Blood Gulch's p99 is held to 8.33 ms (120 frames a
second). OPENCE_BENCHMARK_REBASE=1 keeps the new reports without
comparing. No window is opened. Run inside nix develop with
assets/maps/bloodgulch.map; OPENCE_BENCHMARK_SECONDS shortens it."""

from __future__ import annotations

import json
import os
import shutil
import subprocess
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
MAPS = ROOT / "assets" / "maps"
OUT = ROOT / "build" / "engine"
P99_MS = 8.33
PATHS = ["devgulch", "bloodgulch", "bloodgulch16"]


@pytest.fixture(scope="module")
def executable() -> Path:
    if shutil.which("cargo") is None or not Path("/usr/bin/xcrun").exists():
        pytest.skip("needs nix develop's cargo and Xcode's Swift")
    if not (MAPS / "bloodgulch.map").exists():
        pytest.skip("no assets/maps/bloodgulch.map")
    subprocess.run(["python3", str(ROOT / "tools" / "engine_build.py"), "--out", str(OUT)], check=True)
    return OUT / "OpenCE Engine.app" / "Contents" / "MacOS" / "OpenCEEngine"


@pytest.mark.parametrize("path", PATHS)
def test_the_path_holds_its_pace(executable: Path, path: str) -> None:
    seconds = float(os.environ.get("OPENCE_BENCHMARK_SECONDS", "60"))
    cache = OUT / "cache"
    cache.mkdir(parents=True, exist_ok=True)
    kept = OUT / f"benchmark_{path}.json"
    command = [
        str(executable),
        "--benchmark",
        str(seconds),
        "--path",
        path,
        "--maps",
        str(MAPS),
        "--size",
        "3840x2160",
        "--anti-aliasing",
        "temporal",
        "--render-scale",
        "1",
        "--cache",
        str(cache),
    ]
    compare = kept.exists() and os.environ.get("OPENCE_BENCHMARK_REBASE") != "1"
    if compare:
        command += ["--baseline", str(kept)]
    result = subprocess.run(command, capture_output=True, text=True, timeout=seconds + 300)
    assert result.returncode in (0, 2), result.stderr
    report = json.loads(result.stdout.strip().splitlines()[-1])
    print(f"\nbenchmark: {report}")
    assert report["path"] == path
    assert report["frames"] > seconds * 60
    # (a regression fails; the report kept stays the one before it)
    assert result.returncode == 0, f"{path} regressed:\n{result.stderr}"
    kept.write_text(json.dumps(report, indent=2) + "\n")
    if path == "bloodgulch":
        assert report["p99_ms"] <= P99_MS, f"p99 {report['p99_ms']} ms is above {P99_MS} ms"
