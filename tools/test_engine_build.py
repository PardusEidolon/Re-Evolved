"""The new engine's build: the bundle tools/engine_build.py lays out, and the
dependencies' licences (cargo deny) and the test step. Run inside nix develop."""

from __future__ import annotations

import plistlib
import shutil
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
ENGINE = ROOT / "engine"

sys.path.insert(0, str(ROOT / "tools"))

import engine_build  # noqa: E402


@pytest.fixture(scope="module")
def app(tmp_path_factory: pytest.TempPathFactory) -> Path:
    if shutil.which("cargo") is None or not Path("/usr/bin/xcrun").exists():
        pytest.skip("needs nix develop's cargo and Xcode's Swift")
    out = tmp_path_factory.mktemp("engine")
    # (game.wasm built already, when it is: compiling source/ takes minutes)
    wasm = ROOT / "build" / "engine" / "wasm" / "game.wasm"
    given = ["--wasm", str(wasm)] if wasm.is_file() else []
    subprocess.run(["python3", str(ROOT / "tools" / "engine_build.py"), "--out", str(out), *given], check=True)
    return out / "OpenCE Engine.app"


def test_the_bundle_holds_the_executable_and_its_plist(app: Path) -> None:
    info = plistlib.loads((app / "Contents" / "Info.plist").read_bytes())
    assert info["CFBundleExecutable"] == "OpenCEEngine"
    assert info["LSMinimumSystemVersion"] == "26.0"
    assert (app / "Contents" / "MacOS" / "OpenCEEngine").is_file()


def test_the_bundle_holds_the_game(app: Path) -> None:
    # (the original's code as wasm32, which the app plays: GamePlayer.swift)
    wasm = app / "Contents" / "Resources" / "game.wasm"
    assert wasm.read_bytes()[:4] == b"\0asm"


def test_the_bundle_holds_no_shaders_of_its_own(app: Path) -> None:
    # (the game's shaders are its own, translated as it runs and compiled into
    # the binary archive behind the loading screen: GameRenderer.swift)
    resources = app / "Contents" / "Resources"
    assert not (resources / "default.metallib").exists()
    assert not (resources / "shaders").exists()


def test_the_executable_links_the_rust_library(app: Path) -> None:
    result = subprocess.run(
        [str(app / "Contents" / "MacOS" / "OpenCEEngine"), "--version"],
        capture_output=True,
        text=True,
        check=True,
        timeout=30,
    )
    assert result.stdout.strip() == "OpenCE Engine 0.1.0"


def test_the_bundle_is_signed(app: Path) -> None:
    subprocess.run(["/usr/bin/codesign", "--verify", "--strict", str(app)], check=True)


def test_the_swift_contract_tests_pass(app: Path, tmp_path: Path) -> None:
    # (after the app fixture, which has checked the tools are there)
    subprocess.run(
        ["python3", str(ROOT / "tools" / "engine_build.py"), "--out", str(tmp_path), "--test", "--suite", "swift"],
        check=True,
    )


def test_the_tests_build_only_the_files_git_tracks(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    package = tmp_path / "app"
    (package / "Tests").mkdir(parents=True)
    (package / "Package.swift").write_text("// package\n")
    (package / "Tests" / "Kept.swift").write_text("// kept\n")
    subprocess.run(["git", "init", "-q"], cwd=package, check=True)
    subprocess.run(["git", "add", "."], cwd=package, check=True)
    # (a test file of the checkout's own, never added)
    (package / "Tests" / "ZZMine.swift").write_text("// mine\n")
    monkeypatch.setattr(engine_build, "PACKAGE", package)
    copy = engine_build.tracked_package(tmp_path / "out")
    assert (copy / "Package.swift").is_file()
    assert (copy / "Tests" / "Kept.swift").is_file()
    assert not (copy / "Tests" / "ZZMine.swift").exists()


def test_the_dependencies_licences_are_permissive() -> None:
    if shutil.which("cargo-deny") is None:
        pytest.skip("needs nix develop's cargo-deny")
    subprocess.run(engine_build.deny_command(), cwd=ENGINE, check=True)


def test_a_forbidden_licence_fails_the_check(tmp_path: Path) -> None:
    if shutil.which("cargo-deny") is None:
        pytest.skip("needs nix develop's cargo-deny")
    allowed = (ENGINE / "deny.toml").read_text()
    # (half the dependencies are under MIT alone)
    assert '    "MIT",\n' in allowed
    forbidding = tmp_path / "deny.toml"
    forbidding.write_text(allowed.replace('    "MIT",\n', ""))
    run = subprocess.run(engine_build.deny_command(forbidding), cwd=ENGINE, capture_output=True, text=True)
    assert run.returncode != 0
    assert "rejected" in run.stderr


def test_the_test_step_checks_the_licences_then_runs_nextest_optimised() -> None:
    deny, nextest = engine_build.rust_test_commands()
    assert deny == engine_build.deny_command()
    assert nextest[:3] == ["cargo", "nextest", "run"]
    assert "--workspace" in nextest
    assert nextest[nextest.index("--cargo-profile") + 1] == "test-opt"


def test_every_crate_s_bindings_reach_the_package(tmp_path: Path) -> None:
    bindings = tmp_path / "rust" / "bindings"
    bindings.mkdir(parents=True)
    for crate in ("opence_ffi", "opence_render"):
        (bindings / f"{crate}FFI.h").write_text(f"// {crate}\n")
        (bindings / f"{crate}.swift").write_text(f"// {crate}\n")
    package = tmp_path / "package"
    engine_build.swift_bindings(tmp_path / "rust", package)
    ffi = package / "Sources" / engine_build.FFI_MODULE
    core = package / "Sources" / "OpenCEEngineCore"
    assert sorted(path.name for path in core.iterdir()) == ["opence_ffi.swift", "opence_render.swift"]
    assert (ffi / "opence_renderFFI.h").read_text() == "// opence_render\n"
    modulemap = (ffi / "module.modulemap").read_text()
    # each crate's generated Swift imports its own FFI module
    for module in ("opence_ffiFFI", "opence_renderFFI"):
        assert f"module {module} {{" in modulemap
    # (and nothing but the crates' own headers: cbindgen's snapshot went with the old renderer)
    assert sorted(path.name for path in ffi.iterdir()) == [
        "module.modulemap",
        "opence_ffiFFI.h",
        "opence_renderFFI.h",
    ]


def test_new_bindings_start_the_swift_build_afresh(tmp_path: Path) -> None:
    # (SwiftPM kept main.swift compiled against WasmGame's old methods once
    # stop() was added, so wait() called the wrong one and the app crashed)
    bindings = tmp_path / "bindings"
    bindings.mkdir()
    (bindings / "opence_ffi.swift").write_text("// one\n")
    scratch = tmp_path / "swift"
    (scratch / "release").mkdir(parents=True)
    (scratch / "release" / "main.swift.o").write_text("old")
    engine_build.forget_stale_build(scratch, engine_build.bindings_digest(bindings))
    assert not (scratch / "release" / "main.swift.o").exists()
    # the same bindings again keep what was built from them
    (scratch / "release").mkdir(parents=True)
    (scratch / "release" / "main.swift.o").write_text("new")
    engine_build.forget_stale_build(scratch, engine_build.bindings_digest(bindings))
    assert (scratch / "release" / "main.swift.o").read_text() == "new"
    # and changed ones start it afresh
    (bindings / "opence_ffi.swift").write_text("// two\n")
    engine_build.forget_stale_build(scratch, engine_build.bindings_digest(bindings))
    assert not (scratch / "release").exists()
