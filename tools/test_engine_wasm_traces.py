"""The reference traces in the wasm build: the compiler wrapper
(tools/engine_wasm_cc.py) and the driver that runs every trace through it
(tools/engine_wasm_traces.py).

    python -m pytest -q tools/test_engine_wasm_traces.py

The test that runs a trace needs the wasm toolchain nix develop provides
(HALO_WASM_BUILTINS); without it, it skips.
"""

import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_wasm, engine_wasm_cc, engine_wasm_traces  # noqa: E402


@pytest.fixture
def toolchain(monkeypatch, tmp_path):
    monkeypatch.setattr(
        engine_wasm, "wasi_paths", lambda target=engine_wasm.TARGET: (tmp_path / "include", tmp_path / "lib")
    )
    monkeypatch.setattr(engine_wasm, "clang", lambda: "clang-wasm")
    monkeypatch.setenv("HALO_WASM_BUILTINS", str(tmp_path / "builtins.a"))
    return tmp_path


def test_the_harness_keeps_its_own_optimisation_and_takes_the_game_s_abi(toolchain):
    command = engine_wasm_cc.wasm_command(
        ["-std=gnu99", "-O1", "-ffp-contract=off", "-w", "h.c", "-lm"], toolchain / "h.wasm"
    )
    assert command[0] == "clang-wasm"
    assert f"--target={engine_wasm_cc.TARGET}" in command
    assert "-O1" in command and "-O2" not in command and "-g2" not in command
    assert "-lm" not in command
    assert str(toolchain / "lib" / "crt1-command.o") in command
    assert command[-2:] == ["-o", str(toolchain / "h.wasm")]


def test_the_harness_keeps_the_game_s_diagnostics_as_warnings(toolchain):
    command = engine_wasm_cc.wasm_command(["h.c"], toolchain / "h.wasm")
    assert "-Wno-error=incompatible-pointer-types" in command
    assert engine_wasm_cc.DIAGNOSTIC_FLAGS
    assert all(flag in engine_wasm.GAME_FLAGS for flag in engine_wasm_cc.DIAGNOSTIC_FLAGS)


def test_the_executable_runs_the_module_in_the_runner(toolchain, monkeypatch):
    built = []
    monkeypatch.setattr(
        subprocess, "run", lambda command: built.append(command) or subprocess.CompletedProcess(command, 0)
    )
    monkeypatch.setenv("OPENCE_WASM_RUN", "/opt/opence wasm")
    executable = toolchain / "trace"
    assert engine_wasm_cc.main(["h.c", "-o", str(executable), "-lm"]) == 0
    assert built and built[0][-1] == str(toolchain / "trace.wasm")
    script = executable.read_text()
    assert script.startswith("#!/bin/sh\n")
    assert "'/opt/opence wasm'" in script and str(toolchain / "trace.wasm") in script
    assert os.access(executable, os.X_OK)


def test_the_speed_mode_builds_the_harness_natively_too_and_times_both(toolchain, monkeypatch):
    built = []
    monkeypatch.setattr(
        subprocess, "run", lambda command: built.append(command) or subprocess.CompletedProcess(command, 0)
    )
    monkeypatch.setenv("OPENCE_WASM_RUN", "runner")
    monkeypatch.setenv("OPENCE_WASM_SPEED", str(toolchain / "speed"))
    monkeypatch.setenv("OPENCE_NATIVE_CC", "clang-native")
    executable = toolchain / "facing-reference-x" / "trace"
    executable.parent.mkdir()
    assert engine_wasm_cc.main(["-O1", "h.c", "-o", str(executable), "-lm"]) == 0
    native = [command for command in built if command[0] == "clang-native"]
    assert native == [["clang-native", "-O1", "h.c", "-o", str(executable.with_suffix(".native")), "-lm"]]
    script = executable.read_text()
    assert script.startswith(f"#!{sys.executable}\n")
    assert str(toolchain / "speed") in script and "facing-reference-x" in script


def test_the_speed_summary_compares_the_run_alone_with_native():
    rows = [
        {"script": "a", "native": 0.110, "baseline": 0.010, "wasm": 0.400, "load": 0.150, "run": 0.200},
        {"script": "b", "native": 0.012, "baseline": 0.010, "wasm": 0.100, "load": 0.090, "run": 0.004},
    ]
    summary = engine_wasm_traces.speed_summary(rows, floor=0.050)
    assert summary["timed"] == ["a"]
    assert summary["ratios"]["a"] == pytest.approx(2.0)
    assert summary["geometric_mean"] == pytest.approx(2.0)


def test_a_failed_compile_writes_no_executable(toolchain, monkeypatch):
    monkeypatch.setattr(subprocess, "run", lambda command: subprocess.CompletedProcess(command, 1))
    monkeypatch.setenv("OPENCE_WASM_RUN", "runner")
    assert engine_wasm_cc.main(["h.c", "-o", str(toolchain / "trace")]) == 1
    assert not (toolchain / "trace").exists()


def test_no_output_is_an_error(toolchain):
    with pytest.raises(SystemExit):
        engine_wasm_cc.main(["h.c"])
    with pytest.raises(SystemExit):
        engine_wasm_cc.main(["h.c", "-o"])


def test_the_driver_finds_every_reference_script():
    found = engine_wasm_traces.scripts()
    every = sorted(script.name for script in (ROOT / "tools").glob("engine_*_reference.py"))
    assert [script.name for script in found] == every
    assert all(script.name.endswith("_reference.py") for script in found)
    assert [script.name for script in engine_wasm_traces.scripts("facing")] == ["engine_facing_reference.py"]


@pytest.mark.skipif(not os.environ.get("HALO_WASM_BUILTINS"), reason="needs nix develop's wasm toolchain")
def test_a_trace_matches_in_the_wasm_build():
    assert engine_wasm_traces.main(["--only", "facing_reference"]) == 0


def test_every_script_has_its_trace_and_every_trace_its_script():
    assert engine_wasm_traces.unpaired() == []


def test_the_check_names_a_trace_without_a_script_and_a_script_without_a_trace(tmp_path):
    (tmp_path / "traces").mkdir()
    (tmp_path / "engine_facing_reference.py").write_text("")
    (tmp_path / "engine_wind_reference.py").write_text("")
    (tmp_path / "traces" / "facing_reference.txt").write_text("")
    (tmp_path / "traces" / "orphan_reference.txt").write_text("")
    assert engine_wasm_traces.unpaired(tmp_path) == [
        "engine_wind_reference.py has no trace (traces/wind_reference.txt)",
        "traces/orphan_reference.txt has no script (engine_orphan_reference.py)",
    ]


def test_check_fails_before_running_anything_when_a_trace_is_unpaired(monkeypatch, capsys):
    monkeypatch.setenv("HALO_WASM_BUILTINS", "builtins.a")
    monkeypatch.setattr(engine_wasm_traces, "unpaired", lambda tools=None: ["traces/x_reference.txt has no script"])
    monkeypatch.setattr(engine_wasm_traces, "build_runner", lambda: pytest.fail("built the runner"))
    assert engine_wasm_traces.main(["--check"]) == 1
    assert "traces/x_reference.txt has no script" in capsys.readouterr().out


def test_check_runs_every_script_and_fails_when_one_differs(monkeypatch):
    monkeypatch.setenv("HALO_WASM_BUILTINS", "builtins.a")
    monkeypatch.setattr(engine_wasm_traces, "unpaired", lambda tools=None: [])
    monkeypatch.setattr(engine_wasm_traces, "build_runner", lambda: Path("runner"))
    monkeypatch.setattr(engine_wasm_traces, "environment", lambda runner: {})
    ran = []

    def check(script, env):
        ran.append(script.name)
        return {"script": script.name, "passed": "facing" not in script.name, "seconds": 0.0, "detail": "differs"}

    monkeypatch.setattr(engine_wasm_traces, "check", check)
    assert engine_wasm_traces.main(["--check"]) == 1
    assert sorted(ran) == [script.name for script in engine_wasm_traces.scripts()]
    with pytest.raises(SystemExit):
        engine_wasm_traces.main(["--check", "--only", "facing"])
