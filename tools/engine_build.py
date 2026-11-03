#!/usr/bin/env python3
"""Builds OpenCE Engine.app: game.wasm played in Wasmtime under the Swift
app (conductor/tracks/native_port_20261028).

  engine_build.py [--out build/engine] [--codesign codesign] [--rust folder] [--wasm game.wasm]
  engine_build.py --rust-only folder

The Rust workspace in engine/ is built as a static library (the ffi crate)
and its UniFFI bindings are written (the Rust half, in out/rust), the
bindings are copied into the Swift package in engine/app, Xcode's Swift
builds the package against the library, and the executable is laid out in a
bundle signed ad hoc. --rust-only builds the Rust half alone, as the flake's
engine-rust package does with crane; --rust takes a Rust half built already
(nix build .#engine-rust) instead of running cargo. The game, game.wasm, is
compiled from source/ (tools/engine_wasm.py) into the bundle's resources;
--wasm takes one built already. The app has no shaders of its own: the
game's are translated as it runs and kept in the binary archive behind the
loading screen (GameRenderer.swift). With --test, the tests run
instead: cargo deny and the workspace's tests under nextest, then the
package's XCTests (--suite picks one).
Run it inside nix develop, or from the flake's engine package.
"""

from __future__ import annotations

import argparse
import hashlib
import os
import plistlib
import shutil
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ENGINE = ROOT / "engine"
PACKAGE = ENGINE / "app"
APP_NAME = "OpenCE Engine"
EXECUTABLE = "OpenCEEngine"
LIBRARY = "libopence_ffi.a"
FFI_MODULE = "opence_ffiFFI"


def cargo(*args: str) -> None:
    subprocess.run(["cargo", *args], cwd=ENGINE, check=True)


def deny_command(config: Path = ENGINE / "deny.toml") -> list[str]:
    """cargo deny's licence, ban and source checks (the advisories need the
    network)."""
    return ["cargo", "deny", "--offline", "--config", str(config), "check", "licenses", "bans", "sources"]


def rust_test_commands(profile: str = "test-opt") -> list[list[str]]:
    """The Rust half's test step: the dependencies' licences, then the
    workspace's tests under nextest (engine/.config/nextest.toml), built
    optimised with the debug checks kept."""
    return [
        deny_command(),
        ["cargo", "nextest", "run", "--locked", "--workspace", "--cargo-profile", profile],
    ]


def rust_half(rust: Path) -> None:
    """Builds the library into rust/lib, and writes its UniFFI bindings into
    rust/bindings."""
    cargo("build", "--release", "--locked", "-p", "opence_ffi", "--lib")
    target = Path(os.environ.get("CARGO_TARGET_DIR", ENGINE / "target"))
    bindings = rust / "bindings"
    for folder in (rust / "lib", bindings):
        shutil.rmtree(folder, ignore_errors=True)
        folder.mkdir(parents=True)
    library = rust / "lib" / LIBRARY
    shutil.copy(target / "release" / LIBRARY, library)
    cargo(
        "run",
        "--release",
        "--locked",
        "-p",
        "opence_ffi",
        "--features",
        "bindgen",
        "--bin",
        "uniffi-bindgen",
        "--",
        "generate",
        "--library",
        str(library),
        "--language",
        "swift",
        "--out-dir",
        str(bindings),
    )


def swift_bindings(rust: Path, package: Path = PACKAGE) -> None:
    """Copies the Rust half's bindings where Package.swift expects them:
    every crate's generated Swift into OpenCEEngineCore, and its FFI header
    into the one system library, whose module map declares a module for
    each, since each crate's Swift imports its own (`<crate>FFI`)."""
    bindings = rust / "bindings"
    ffi = package / "Sources" / FFI_MODULE
    core = package / "Sources" / "OpenCEEngineCore"
    for folder in (ffi, core):
        shutil.rmtree(folder, ignore_errors=True)
        folder.mkdir(parents=True)
    modules = []
    for header in sorted(bindings.glob("*FFI.h")):
        shutil.copy(header, ffi / header.name)
        modules.append(f'module {header.stem} {{\n    header "{header.name}"\n    export *\n}}\n')
    (ffi / "module.modulemap").write_text("".join(modules))
    for swift in sorted(bindings.glob("*.swift")):
        shutil.copy(swift, core / swift.name)


def bindings_digest(bindings: Path) -> str:
    """A digest of the generated bindings' files, names and contents."""
    digest = hashlib.sha256()
    for path in sorted(bindings.iterdir()):
        if path.is_file():
            digest.update(path.name.encode() + b"\0" + path.read_bytes() + b"\0")
    return digest.hexdigest()


def forget_stale_build(scratch: Path, digest: str) -> None:
    """Empties the Swift build folder when the bindings it was built from
    have changed: SwiftPM's incremental build can keep the app's own files
    compiled against a class's old methods (WasmGame gaining stop() left
    main.swift's wait() calling the wrong one, and the app crashed)."""
    stamp = scratch / "bindings.sha256"
    if stamp.is_file() and stamp.read_text() == digest:
        return
    shutil.rmtree(scratch, ignore_errors=True)
    scratch.mkdir(parents=True)
    stamp.write_text(digest)


def xcode_environment() -> dict[str, str]:
    """The environment without nix develop's SDK and compilers, for Xcode's tools."""
    return {
        key: value
        for key, value in os.environ.items()
        if key not in ("SDKROOT", "DEVELOPER_DIR", "MACOSX_DEPLOYMENT_TARGET", "CC", "CXX", "LD")
    }


def tracked_package(out: Path) -> Path:
    """A copy of the package's files git tracks, in `out` (the tests build
    it, so that test files of the checkout's own that git does not track are
    left out); the whole folder but its .build where git cannot say."""
    copy = out / "package"
    shutil.rmtree(copy, ignore_errors=True)
    # the shell finds its fonts by climbing from its sources to
    # port/assets/fonts, so a copy outside the checkout takes them along
    fonts = out / "port" / "assets" / "fonts"
    shutil.rmtree(fonts, ignore_errors=True)
    shutil.copytree(ROOT / "port" / "assets" / "fonts", fonts)
    listed = subprocess.run(
        ["git", "ls-files", "-z", "--", "."], cwd=PACKAGE, capture_output=True, text=True, check=False
    )
    if listed.returncode != 0:
        shutil.copytree(PACKAGE, copy, ignore=shutil.ignore_patterns(".build"))
        return copy
    for name in filter(None, listed.stdout.split("\0")):
        source = PACKAGE / name
        if source.is_file():
            (copy / name).parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(source, copy / name)
    return copy


def swift(command: str, library: Path, out: Path, extra: tuple[str, ...] = (), package: Path = PACKAGE) -> Path:
    """Runs swift build or swift test with Xcode's Swift (nixpkgs' cannot
    link against the macOS 26 SDK) on `package`; returns the build folder."""
    scratch = out / "swift"
    environment = xcode_environment()
    subprocess.run(
        [
            "/usr/bin/xcrun",
            "swift",
            command,
            "-c",
            "release",
            "--quiet",
            "--disable-sandbox",
            "--package-path",
            str(package),
            "--scratch-path",
            str(scratch),
            "--cache-path",
            str(scratch / "cache"),
            "--config-path",
            str(scratch / "config"),
            "--security-path",
            str(scratch / "security"),
            "-Xlinker",
            str(library),
            *extra,
        ],
        env=environment,
        check=True,
    )
    return scratch / "release"


def info_plist(version: str) -> bytes:
    return plistlib.dumps(
        {
            "CFBundleDevelopmentRegion": "en",
            "CFBundleExecutable": EXECUTABLE,
            "CFBundleIdentifier": "org.opence.engine",
            "CFBundleInfoDictionaryVersion": "6.0",
            "CFBundleName": APP_NAME,
            "CFBundleDisplayName": APP_NAME,
            "CFBundlePackageType": "APPL",
            "CFBundleShortVersionString": version,
            "CFBundleVersion": version,
            "LSMinimumSystemVersion": "26.0",
            "NSHighResolutionCapable": True,
            "NSPrincipalClass": "NSApplication",
        }
    )


def bundle(executable: Path, wasm: Path, codesign: str, version: str, app: Path) -> None:
    """Lays out the bundle in a staging folder and renames it into place."""
    staging = app.with_name(app.name + ".partial")
    shutil.rmtree(staging, ignore_errors=True)
    (staging / "Contents" / "MacOS").mkdir(parents=True)
    shutil.copy(executable, staging / "Contents" / "MacOS" / EXECUTABLE)
    (staging / "Contents" / "Info.plist").write_bytes(info_plist(version))
    # the HUD's redraws, for the Halo UI plugin (conductor/tracks/halo_ui_20261028)
    shutil.copytree(ROOT / "port" / "assets" / "hud", staging / "Contents" / "Resources" / "hud")
    # the game: the original's code as wasm32 (GamePlayer.swift)
    shutil.copy(wasm, staging / "Contents" / "Resources" / "game.wasm")
    # the fonts the Halo UI plugin draws with, with their licences: Overpass
    # and OpenCE only
    fonts = staging / "Contents" / "Resources" / "fonts"
    fonts.mkdir()
    for name in (
        "Overpass-750.ttf",
        "Overpass-900.ttf",
        "Overpass-OFL.txt",
        "OpenCE-Regular.ttf",
        "OpenCE-OFL.txt",
        "OverpassMono-Regular.ttf",
        "OverpassMono-OFL.txt",
    ):
        shutil.copy(ROOT / "port" / "assets" / "fonts" / name, fonts / name)
    subprocess.run([codesign, "-s", "-", "-f", str(staging)], check=True)
    shutil.rmtree(app, ignore_errors=True)
    staging.rename(app)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--out", type=Path, default=ROOT / "build" / "engine")
    # (Apple's: nixpkgs' sigtool signs Mach-O files only, not bundles)
    parser.add_argument("--codesign", default=os.environ.get("HALO_APP_CODESIGN", "/usr/bin/codesign"))
    parser.add_argument("--version", default="0.1.0")
    parser.add_argument("--test", action="store_true", help="run the package's XCTests instead of making the app")
    parser.add_argument("--filter", help="with --test, only the tests whose names match this")
    parser.add_argument(
        "--suite",
        choices=("all", "rust", "swift"),
        default="all",
        help="with --test, the Rust half's tests (cargo deny and nextest), the XCTests, or both",
    )
    parser.add_argument("--rust", type=Path, help="the Rust half built already, instead of running cargo")
    parser.add_argument("--rust-only", type=Path, help="build only the Rust half, into this folder")
    parser.add_argument("--wasm", type=Path, help="game.wasm built already, instead of compiling source/")
    arguments = parser.parse_args()
    if arguments.rust_only:
        rust_half(arguments.rust_only.resolve())
        return
    if arguments.test and arguments.suite != "swift":
        for command in rust_test_commands():
            subprocess.run(command, cwd=ENGINE, check=True)
        if arguments.suite == "rust":
            return
    out = arguments.out.resolve()
    out.mkdir(parents=True, exist_ok=True)
    rust = arguments.rust.resolve() if arguments.rust else out / "rust"
    if not arguments.rust:
        rust_half(rust)
    swift_bindings(rust)
    forget_stale_build(out / "swift", bindings_digest(rust / "bindings"))
    library = rust / "lib" / LIBRARY
    if arguments.test:
        package = tracked_package(out)
        swift_bindings(rust, package)
        swift("test", library, out, ("--filter", arguments.filter) if arguments.filter else (), package)
        return
    executable = swift("build", library, out) / EXECUTABLE
    if arguments.wasm:
        wasm = arguments.wasm.resolve()
    else:
        # imported here: --rust-only runs in crane's source, which has only
        # this script of tools/, not engine_wasm and the build scripts it reads
        import engine_wasm

        wasm = engine_wasm.build(os.cpu_count() or 4)
    bundle(executable, wasm, arguments.codesign, arguments.version, out / f"{APP_NAME}.app")


if __name__ == "__main__":
    main()
