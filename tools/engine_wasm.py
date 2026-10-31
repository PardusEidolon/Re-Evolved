"""Build the game, source/ unchanged, as build/engine/wasm/game.wasm.

The game runs in Wasmtime inside the macOS app (track native_port_20261028).
The module is compiled from the web build's flags (tools/web_build.py), which
already made source/ correct for wasm32: an ILP32 target, as the game's data
formats need, where a call must match its callee's type exactly (the link's
--fatal-warnings), and where the game files that call variadic functions
without a prototype get one (tools/android_build.py VARIADIC_PROTOTYPE_FILES).

What goes into the module is the game: source/ less what port.json excludes,
the port's own game units (port/linux/game), stb_vorbis and musl's maths, so
every run computes the same results, and the C library's helpers and the
platform's variables and objects (port/wasmtime/crt.c, platform_data.c; the
Linux port's wide strings, pooled globals and sound mixer, dsound_sdl.c, which
hands its mixed frames to the runtime). The C library is wasi-libc. Every
function the game calls that none of these defines is left undefined and
becomes an import from "env": that set is the platform layer, which the Rust
runtime provides (engine/crates/runtime). tools/engine_wasm_imports.txt lists
it; the test (tools/test_engine_wasm.py) fails when the module's imports or
exports differ from the lists, so a change to the platform layer's surface is
always a reviewed one.

Units are not optimised together at the link (no -flto): LLVM's link-time
optimisation turns a call whose parameter types differ only in width into a
trap (tools/web_build.py).

Run inside nix develop, which puts a clang with the wasm32 target, wasm-ld
and wasi-libc (HALO_WASI_LIBC, HALO_WASI_LIBC_DEV) in the environment:

    python3 tools/engine_wasm.py            # build game.wasm
    python3 tools/engine_wasm.py --imports  # print the module's imports
"""

from __future__ import annotations

import argparse
import concurrent.futures
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Dict, List, Optional, Sequence, Tuple

ROOT = Path(__file__).resolve().parent.parent
if __package__ in (None, ""):
    sys.path.insert(0, str(ROOT))

from tools.android_build import VARIADIC_PROTOTYPE_FILES  # noqa: E402
from tools.embed_assets import RAGDOLL_OVERRIDES, SMAA_ASSETS, SMAA_FILES, words  # noqa: E402
from tools.linux_build import (  # noqa: E402
    MUSL_MATH_DIR,
    STB_VORBIS_SOURCE,
    XDK_INCLUDE,
    ZLIB_DEFINES,
    ZLIB_DIR,
    ZLIB_SOURCES,
    game_sources,
    musl_math_sources,
)
from tools.web_build import GAME_FLAGS, PROTOTYPE_FILES, WEB_CODE_FLAGS  # noqa: E402

# the C library helpers the game calls that are not the platform (the printf
# family, the case-blind compares, the D3DX maths and the rest): compiled
# into the module, so that none is a call out to the runtime
CRT_SOURCE = Path("port/wasmtime/crt.c")
# the 16-bit wide string functions (-fshort-wchar), the Linux port's
WIDE_SOURCE = Path("port/linux/src/msvc_wide.c")
# the game's pooled COMMON globals no unit of source/ defines (weak, zeroed:
# the Linux port's)
COMMON_SOURCE = Path("port/linux/src/halo_linker_common.c")
# the Xbox libraries' variables the game uses directly (Direct3D's states,
# the input device types), which can't be imported
PLATFORM_DATA_SOURCE = Path("port/wasmtime/platform_data.c")
# DirectSound: the Linux port's mixer, its voices, streams, reverb and
# resampler, without SDL (HALO_WASMTIME): a thread of the game's mixes into
# the runtime's ring (halo_audio_output)
DSOUND_SOURCE = Path("port/linux/src/dsound_sdl.c")
# the game's log lines and debug output, to standard error
LOG_SOURCE = Path("port/wasmtime/log.c")
# a file read whole through the platform's file imports, for crt.c's fopen
DATA_FILES_SOURCE = Path("port/wasmtime/data_files.c")
# the Direct3D device: the macOS app's Metal device, whose batches the app's
# renderer draws (host_render_submit), with the Linux port's resources,
# textures and shader translators beside it, the web build's memory watch
# (WebAssembly has no page protection, so it hashes pages) and game.wasm's
# own platform for them
METAL_SOURCES = [
    Path("port/wasmtime/metal/d3d8_metal_device.c"),
    Path("port/wasmtime/metal/d3d8_metal.c"),
    Path("port/wasmtime/metal/d3d8_metal_post.c"),
    Path("port/wasmtime/metal/opence_render_writer.c"),
    Path("port/linux/src/xbox_textures.c"),
    Path("port/linux/src/d3d8_resources.c"),
    Path("port/linux/src/nv2a_vsh.c"),
    Path("port/linux/src/nv2a_psh.c"),
    Path("port/linux/src/xgpu_text.c"),
    Path("port/wasmtime/metal_platform.c"),
    Path("port/web/src/web_memory_watch.c"),
    Path("port/android/host/host_watch_hash.c"),
]
# the game's text drawn from its font tags at the display's resolution,
# compiled as the game's units are (it reads the tags through the game's
# headers), with text_hires.h after them
TEXT_TAGS_SOURCE = Path("port/wasmtime/text_tags.c")
# game.wasm's own headers, found before the Linux port's (profile_sections.h)
WASMTIME_INCLUDE = Path("port/wasmtime/include")

LINUX_DIR = Path("port/linux")
ANDROID_DIR = Path("port/android")
PORT_CONFIG = LINUX_DIR / "port.json"
BUILD = Path("build/engine/wasm")
IMPORTS_LIST = Path("tools/engine_wasm_imports.txt")
EXPORTS_LIST = Path("tools/engine_wasm_exports.txt")

# the game's target: wasm32 with atomics and a shared memory, so its threads
# are its own, each the module instantiated again over the same memory
# (wasi-threads; opence_runtime's threads module answers its imports)
TARGET = "wasm32-wasip1-threads"

# The game's address space. The module starts with 2 GB, which wasi-libc's
# malloc takes as its first region (__heap_base to __heap_end, the end of
# the initial memory). The platform then grows the memory past the Xbox's
# window, 0x80000000 to 0xA0000000 (port/linux/src/platform.h, at the
# desktop sizes), before the game runs, so the heap's later growth lands
# above it and the window's pages are the game's alone. The memory may grow
# to wasm32's 4 GB.
MEMORY_BYTES = 0x80000000
MAXIMUM_MEMORY_BYTES = 0x100000000
# (the game recurses deeply in places: scripts, the BSP's walks)
STACK_BYTES = 8 * 1024 * 1024

# The ABI the game was written for, as the web build reproduces it, with
# its threads (-pthread: atomics and bulk memory).
ABI_FLAGS = [
    f"--target={TARGET}",
    "-pthread",
    "-DHALO_WASMTIME=1",
    "-fshort-wchar",
    "-ffp-contract=off",
    "-msimd128",
    "-mbulk-memory",
    "-mnontrapping-fptoint",
    "-O2",
    "-g2",
]

# what the module exports to the runtime: the game's entry point, the C
# heap, which the platform layer allocates the game's buffers from, and what
# its threads need (CreateThread calls the game's pthread_create, and a new
# thread starts at wasi_thread_start), and the addresses of Direct3D's
# render and texture states, whose defaults the device writes; and for the
# input, the device types the game opens and whether its console or chat
# takes the keys; and the call through which the runtime completes a sound
# packet, to the stream's callback; the menus' widgets, which the Halo UI
# plugin reads; and the loading screen's chant, which the runtime fills from
# the player's xbe
EXPORTS = [
    "halo_game_main",
    "malloc",
    "free",
    "pthread_create",
    "pthread_detach",
    "wasi_thread_start",
    "D3D__RenderState",
    "D3D__TextureState",
    "XDEVICE_TYPE_GAMEPAD_TABLE",
    "XDEVICE_TYPE_DEBUG_KEYBOARD_TABLE",
    "console_is_active",
    "network_chat_typing",
    "network_chat_keyboard_open",
    "ui_widget_globals_storage",
    "virtual_keyboard_port_view",
    "compressed_monk_tone",
]

# wasi-libc for each target, as nix develop names it
WASI_LIBC = {
    "wasm32-wasip1-threads": "HALO_WASI_LIBC_THREADS",
    "wasm32-wasip1": "HALO_WASI_LIBC",
}


# the platform layer: every function the game calls that the module does not
# define is imported from "env". A variable can't be imported, and
# --allow-undefined would place an undefined one at address 0, so the link
# refuses one instead.
UNDEFINED_FLAGS = ["-Wl,--import-undefined"]


def wasi_paths(target: str = TARGET) -> Tuple[Path, Path]:
    """wasi-libc's include and lib directories for target (nix develop's
    HALO_WASI_LIBC and HALO_WASI_LIBC_DEV, or their _THREADS forms)"""
    name = WASI_LIBC[target]
    dev = os.environ.get(f"{name}_DEV")
    lib = os.environ.get(name)
    if not dev or not lib:
        raise SystemExit(f"engine_wasm: run inside nix develop ({name} and {name}_DEV are unset)")
    return Path(dev) / "include" / target, Path(lib) / "lib" / target


def clang() -> str:
    return os.environ.get("HALO_WASM_CC") or "clang"


def semantics_header(build: Path) -> Path:
    """MSVC's inline and tag semantics for the game
    (tools/linux_msvc_semantics.py, as the Linux build makes it)"""
    header = build / "halo_msvc_semantics.h"
    scan = ["--all-inlines", "--tags", "source", "--inlines", "source", "--inlines", str(XDK_INCLUDE)]
    subprocess.run(
        [sys.executable, "tools/linux_msvc_semantics.py", "--output", str(header), *scan],
        check=True,
        cwd=ROOT,
    )
    return header


def ragdoll_unit(build: Path) -> Path:
    """the ragdolls' overrides (port/linux/game/ragdoll_bodies.c) as a unit,
    laid out as tools/embed_assets.py lays them out, size 0 where the
    checkout has none; written only when it changes, so it isn't rebuilt"""
    source = build / "ragdoll_overrides.c"
    data = (ROOT / RAGDOLL_OVERRIDES).read_bytes() if (ROOT / RAGDOLL_OVERRIDES).is_file() else b""
    lines = ["const unsigned int ragdoll_overrides_embedded[] = {", *(words(data) if data else ["\t0,"]), "};"]
    lines.append(f"const unsigned long ragdoll_overrides_embedded_size = {len(data)};")
    text = "\n".join(lines) + "\n"
    if not source.exists() or source.read_text() != text:
        source.write_text(text)
    return source


def smaa_unit(build: Path, assets: Path = ROOT / SMAA_ASSETS) -> Path:
    """SMAA's shader and lookup textures (port/wasmtime/metal/d3d8_metal_post.c)
    as a unit, laid out as tools/embed_assets.py lays them out: the shader
    as text, each byte past ASCII a space, with a NUL; size 0 for a file the
    checkout lacks. Written only when it changes, so it isn't rebuilt"""
    source = build / "smaa_assets.c"
    lines: List[str] = []
    for name, symbol in SMAA_FILES:
        data = (assets / name).read_bytes() if (assets / name).is_file() else b""
        if data and name.endswith(".hlsl"):
            data = bytes(byte if byte < 0x80 else 0x20 for byte in data) + b"\0"
        lines += [f"const unsigned int {symbol}[] = {{", *(words(data) if data else ["\t0,"]), "};"]
        lines.append(f"const unsigned long {symbol}_size = {len(data)};")
    text = "\n".join(lines) + "\n"
    if not source.exists() or source.read_text() != text:
        source.write_text(text)
    return source


def platform_flags(build: Path, include: Path) -> List[str]:
    """the flags of game.wasm's platform layer (port/linux/src's units)"""
    prefix = LINUX_DIR / "include" / "halo_linux_prefix.h"
    common = [*ABI_FLAGS, f"-isystem{include}"]
    platform = [
        *common,
        *("-fno-common" if flag == "-fcommon" else flag for flag in WEB_CODE_FLAGS),
        "-std=gnu11",
        "-D_GNU_SOURCE",
        "-DHALO_LINUX_PLATFORM_LAYER",
        # WASI has no temporary names (its tmpnam is declared, never
        # defined): msvc_wide.c's _wtmpnam only needs the buffer's length
        "-DL_tmpnam=20",
        "-w",
        "-include",
        str(prefix),
        "-include",
        str(build / "halo_msvc_semantics.h"),
        f"-I{LINUX_DIR}/src",
        f"-I{LINUX_DIR}/include",
        "-idirafter",
        str(XDK_INCLUDE),
    ]
    return platform


def metal_flags(build: Path, include: Path) -> List[str]:
    """the flags of game.wasm's Metal device and the units beside it
    (METAL_SOURCES)"""
    platform = platform_flags(build, include)
    metal = [
        *platform,
        "-DHALO_METAL",
        "-DHALO_GLES=1",
        f"-I{os.environ.get('HALO_WASM_GLES_INCLUDE', '')}",
        "-Iport/android/host",
        "-Iport/android/include",
        "-Isource",
        "-Isource/cseries",
        f"-I{os.environ.get('HALO_WASM_SDL_INCLUDE', '')}",
        "-Iport/wasmtime/include",
        "-Iport/wasmtime/metal",
        "-Iport/third_party/zlib",
        "-Iport/android/guest/runtime",
    ]
    return metal


def units(config: Dict[str, object], build: Path, include: Path) -> List[Tuple[Path, List[str]]]:
    """every unit of the module, with its flags"""
    prefix = LINUX_DIR / "include" / "halo_linux_prefix.h"
    common = [*ABI_FLAGS, f"-isystem{include}"]
    game = [
        *common,
        # clang 22 crashes emitting a common symbol for wasm32-wasip1 (in
        # AsmPrinter::emitGlobalVariable), so the game's tentative
        # definitions are compiled as definitions, and each must be defined
        # in one unit only
        *("-fno-common" if flag == "-fcommon" else flag for flag in WEB_CODE_FLAGS),
        *GAME_FLAGS,
        "-include",
        str(prefix),
        "-include",
        str(build / "halo_msvc_semantics.h"),
        # game.wasm's own profile_sections.h, before the Linux port's
        f"-I{WASMTIME_INCLUDE}",
        f"-I{LINUX_DIR}/include",
        "-iquote",
        str(config["game_sources"]),
        # (port.json's defines and include directories, as
        # game_defines_and_includes has them for a shell)
        *(f"-D{define}" for define in config["game"].get("defines", [])),  # type: ignore[index]
        *(f"-I{directory}" for directory in config["game"].get("include_dirs", [])),  # type: ignore[index]
        "-idirafter",
        str(XDK_INCLUDE),
    ]
    result: List[Tuple[Path, List[str]]] = []
    for source in game_sources(config):  # type: ignore[arg-type]
        name = source.as_posix()
        flags = list(game)
        if name in VARIADIC_PROTOTYPE_FILES:
            flags += ["-include", f"{ANDROID_DIR}/include/halo_android_variadic_prototypes.h"]
        if name in PROTOTYPE_FILES:
            flags += ["-include", PROTOTYPE_FILES[name]]
        if name == "source/shell/shell_xbox.c":
            # the runtime calls the game's main once the platform is up
            flags += ["-Dmain=halo_game_main"]
        result.append((source, flags))
    for source in sorted(Path(str(config["game_sources"])).glob("*.c")):
        result.append((source, list(game)))
    result.append((STB_VORBIS_SOURCE, list(game)))
    result.append((TEXT_TAGS_SOURCE, [*game, "-idirafter", f"{LINUX_DIR}/src"]))
    result.append((CRT_SOURCE, [*common, "-std=gnu11", "-Wall", "-Werror"]))
    platform = platform_flags(build, include)
    result.append((WIDE_SOURCE, platform))
    result.append((COMMON_SOURCE, platform))
    result.append((PLATFORM_DATA_SOURCE, platform))
    result.append((DSOUND_SOURCE, platform))
    result.append((LOG_SOURCE, platform))
    result.append((DATA_FILES_SOURCE, platform))
    metal = metal_flags(build, include)
    for source in METAL_SOURCES:
        result.append((source, list(metal)))
    result.append((smaa_unit(build).relative_to(ROOT), [*common, "-std=gnu11", "-Wall", "-Werror"]))
    result.append((ragdoll_unit(build).relative_to(ROOT), [*common, "-std=gnu11", "-Wall", "-Werror"]))
    # the saved films' and the mods' inflate (zlib, with its names prefixed)
    for name in ZLIB_SOURCES:
        result.append((ZLIB_DIR / name, [*common, "-std=gnu11", *ZLIB_DEFINES, "-w"]))
    musl = [*common, "-std=gnu11", "-w", f"-I{MUSL_MATH_DIR}/include", "-include", f"{MUSL_MATH_DIR}/include/libm.h"]
    for source in musl_math_sources():
        result.append((source, list(musl)))
    return result


def compile_unit(source: Path, flags: Sequence[str], obj: Path) -> Optional[str]:
    """compile one unit; its errors, or None"""
    obj.parent.mkdir(parents=True, exist_ok=True)
    if obj.exists() and obj.stat().st_mtime >= source.stat().st_mtime and obj.with_suffix(".flags").exists():
        if obj.with_suffix(".flags").read_text() == "\n".join(flags) and not stale_dependencies(obj):
            return None
    done = subprocess.run(
        [clang(), *flags, "-MMD", "-MF", str(obj.with_suffix(".d")), "-c", str(source), "-o", str(obj)],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if done.returncode != 0:
        return f"{source}:\n{done.stderr}"
    obj.with_suffix(".flags").write_text("\n".join(flags))
    return None


def stale_dependencies(obj: Path) -> bool:
    """a header the unit included is newer than its object"""
    depfile = obj.with_suffix(".d")
    if not depfile.exists():
        return True
    built = obj.stat().st_mtime
    text = depfile.read_text().replace("\\\n", " ")
    for name in text.split(":", 1)[-1].split():
        path = ROOT / name if not os.path.isabs(name) else Path(name)
        if path.exists() and path.stat().st_mtime > built:
            return True
    return False


def link(objects: Sequence[Path], lib: Path, output: Path) -> None:
    builtins = os.environ.get("HALO_WASM_BUILTINS")
    if not builtins:
        raise SystemExit("engine_wasm: run inside nix develop (HALO_WASM_BUILTINS is unset)")
    flags = [
        clang(),
        f"--target={TARGET}",
        "-mexec-model=reactor",
        # wasi-libc's start file and compiler-rt's builtins, named here: the
        # unwrapped clang has neither where it would look
        "-nostdlib",
        str(lib / "crt1-reactor.o"),
        f"-L{lib}",
        "-Wl,--fatal-warnings",
        *UNDEFINED_FLAGS,
        f"-Wl,--initial-memory={MEMORY_BYTES}",
        f"-Wl,--max-memory={MAXIMUM_MEMORY_BYTES}",
        f"-Wl,-z,stack-size={STACK_BYTES}",
        # the memory, shared between the game's threads, is the runtime's,
        # and the function table is exported for the completion routines
        "-pthread",
        "-Wl,--shared-memory",
        "-Wl,--import-memory",
        "-Wl,--export-table",
        *(f"-Wl,--export={name}" for name in EXPORTS),
        "-lsetjmp",
        "-lwasi-emulated-mman",
        "-lc",
        builtins,
        "-o",
        str(output),
    ]
    rsp = output.with_suffix(".rsp")
    # (quoted: "saved games" and "saved films" have spaces)
    rsp.write_text("\n".join(f'"{obj}"' for obj in objects))
    subprocess.run([*flags, f"@{rsp}"], cwd=ROOT, check=True)


def build(jobs: int) -> Path:
    config = json.loads((ROOT / PORT_CONFIG).read_text(encoding="utf-8"))
    out = ROOT / BUILD
    out.mkdir(parents=True, exist_ok=True)
    include, lib = wasi_paths()
    semantics_header(out)
    work = units(config, out, include)
    errors: List[str] = []
    objects: List[Path] = []
    with concurrent.futures.ThreadPoolExecutor(max_workers=jobs) as pool:
        futures = []
        for source, flags in work:
            obj = out / "obj" / source.with_suffix(".o")
            objects.append(obj)
            futures.append(pool.submit(compile_unit, source, flags, obj))
        for future in futures:
            error = future.result()
            if error:
                errors.append(error)
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        raise SystemExit(f"engine_wasm: {len(errors)} of {len(work)} units failed")
    output = out / "game.wasm"
    link(objects, lib, output)
    return output


def module_imports(module: Path) -> List[str]:
    """the module's imports, "module.name: type", sorted"""
    return sorted(_section(module, "import"))


def module_exports(module: Path) -> List[str]:
    return sorted(_section(module, "export"))


def _section(module: Path, kind: str) -> List[str]:
    """a module's imports or exports, read from its binary (the wasm
    format's import and export sections, read with bounds checks)"""
    from tools.wasm_module import read_module

    parsed = read_module(module.read_bytes())
    return parsed.imports if kind == "import" else parsed.exports


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--jobs", type=int, default=os.cpu_count() or 8)
    parser.add_argument("--imports", action="store_true", help="print the built module's imports")
    parser.add_argument("--exports", action="store_true", help="print the built module's exports")
    parser.add_argument("--write-lists", action="store_true", help="write the import and export lists")
    args = parser.parse_args()
    if shutil.which(clang()) is None:
        raise SystemExit("engine_wasm: no clang (run inside nix develop)")
    output = build(args.jobs)
    if args.imports:
        print("\n".join(module_imports(output)))
    if args.exports:
        print("\n".join(module_exports(output)))
    if args.write_lists:
        (ROOT / IMPORTS_LIST).write_text("\n".join(module_imports(output)) + "\n")
        (ROOT / EXPORTS_LIST).write_text("\n".join(module_exports(output)) + "\n")
    print(output)


if __name__ == "__main__":
    main()
