"""Asset-free regression tests: the engine's real code compiled against a fake world (README.md)."""

import functools
import os
import re
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
HARNESS = Path(__file__).resolve().parent


def read(relative):
    """A source file of the repository, as the compiler reads it."""
    return (ROOT / relative).read_text(encoding="latin-1")


def function(source, name):
    """A function's definition (static, inline or not, its type on the line before or not), taken by matching braces; LookupError if not found."""
    match = re.search(r"^(?:static |__inline )?[\w *]+?\n?\b" + re.escape(name) + r"\(\s*[^;{]*\)\s*\{", source, re.M)
    if not match:
        raise LookupError(f"function not found in the sources: {name}")
    end = source.index("{", match.start()) + 1
    depth = 1
    while depth:
        depth += (source[end] == "{") - (source[end] == "}")
        end += 1
    return source[match.start() : end]


def inline(source, name):
    """An __inline header function, as a static one."""
    return function(source, name).replace("__inline", "static", 1)


def enum_with(source, member):
    """The whole enum that declares a member."""
    at = source.index(member)
    start = source.rindex("enum", 0, at)
    return source[start : source.index("};", at) + 2]


def structure(source, name):
    """An actual named structure's definition, including nested unions/structures."""
    match = re.search(r"^struct\s+" + re.escape(name) + r"\s*\{", source, re.M)
    if not match:
        raise LookupError(f"structure not found in the sources: {name}")
    end = source.index("{", match.start()) + 1
    depth = 1
    while depth:
        depth += (source[end] == "{") - (source[end] == "}")
        end += 1
    return source[match.start() : source.index(";", end) + 1]


def constant(source, name):
    """An enum constant's or #define's integer value, following a name it is set to (as the port's capacities are)."""
    match = re.search(r"\b" + re.escape(name) + r"\s*=\s*(\w+)", source) or re.search(
        r"#define\s+" + re.escape(name) + r"\s+(\w+)", source
    )
    if not match:
        raise LookupError(f"constant not found in the sources: {name}")
    value = match.group(1)
    if re.fullmatch(r"0x[0-9A-Fa-f]+|\d+", value):
        return int(value, 0)
    return constant(source + read("port/linux/include/halo_port_capacity.h"), value)


def mutated(text, before, after):
    """The code with one deliberate fault (a negative control); LookupError once the code no longer has it."""
    if text.count(before) != 1:
        raise LookupError(f"negative control no longer applies (found {text.count(before)} times): {before!r}")
    return text.replace(before, after, 1)


_directory = tempfile.TemporaryDirectory(prefix="halo-harness-")


CHECK_FAILED = 1  # a case's exit status when a CHECK fails (harness.h)


def _as_wasm():
    """On arm64 macOS, which runs no 32-bit code, each test is built as a
    wasm32 WASI command and run in the game's runtime (opence-wasm, as the
    reference traces are; needs `nix develop`): the game's own 32-bit ABI, as
    game.wasm has it."""
    import platform
    import sys

    return sys.platform == "darwin" and platform.machine() == "arm64"


# the harness's own flags, for either build
FLAGS = [
    "-std=gnu99",
    "-O2",
    "-Wall",
    "-Werror",
    "-Wno-unused-function",
    "-Wno-unused-variable",
]


@functools.lru_cache(maxsize=None)
def build(test, generated):
    """tests/<test>.c compiled with the generated includes ((name, text), ...) for the game's 32-bit ABI."""
    work = Path(_directory.name) / f"{test}-{abs(hash(generated)):x}"
    work.mkdir(parents=True, exist_ok=True)
    for name, text in generated:
        (work / name).write_text(text)
    includes = ["-I", str(HARNESS / "include"), "-I", str(work)]
    source = str(HARNESS / "tests" / f"{test}.c")
    if _as_wasm():
        # (wchar_t as libc's, 32 bits, as in the Linux build: the tests call
        # libc's wide-string functions, which the game's 16-bit one breaks)
        return _build_as_wasm(test, work, [*FLAGS, "-fno-short-wchar", *includes, source])
    executable = work / test
    command = [os.environ.get("CC", "clang"), "-m32", *FLAGS, *includes, source, "-o", str(executable), "-lm"]
    result = subprocess.run(command, capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"{test}.c does not compile:\n{result.stderr}")
    return executable


def _build_as_wasm(test, work, arguments):
    import pytest
    import sys

    if not os.environ.get("HALO_WASM_BUILTINS"):
        pytest.skip("on macOS the harness runs as wasm32 in the game's runtime: needs `nix develop`")
    sys.path.insert(0, str(ROOT))
    from tools import engine_wasm_cc

    module = work / f"{test}.wasm"
    result = subprocess.run(engine_wasm_cc.wasm_command(arguments, module), capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(f"{test}.c does not compile:\n{result.stderr}")
    return module


@functools.lru_cache(maxsize=None)
def runner():
    """opence-wasm: $OPENCE_WASM_RUN's, or the workspace's, built once."""
    import sys

    if os.environ.get("OPENCE_WASM_RUN"):
        return os.environ["OPENCE_WASM_RUN"]
    sys.path.insert(0, str(ROOT))
    from tools import engine_wasm_traces

    return str(engine_wasm_traces.build_runner())


def process(executable, arguments, **options):
    """subprocess.run of a built test with these arguments (and subprocess.run's options: input, timeout...); on
    macOS, where the test is a wasm32 module, in the game's runtime"""
    if Path(executable).suffix == ".wasm":
        return subprocess.run([runner(), str(executable), *arguments], **options)
    return subprocess.run([str(executable), *arguments], **options)


def run(executable, case):
    """Run one case: (exit status, output)."""
    result = process(executable, [case], capture_output=True, text=True, timeout=60)
    return result.returncode, (result.stdout + result.stderr).strip()
