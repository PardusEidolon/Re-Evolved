"""game.wasm (tools/engine_wasm.py) and the reader of its imports and exports
(tools/wasm_module.py): modules made here, malformed ones among them, and the
built module's imports and exports against the reviewed lists.

    python -m pytest -q tools/test_engine_wasm.py

The built module's test needs the wasm toolchain nix develop provides
(HALO_WASM_CC); without it, it skips.
"""

import os
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_wasm  # noqa: E402
from tools.wasm_module import MalformedModule, read_module  # noqa: E402

I32, I64, F32, F64 = 0x7F, 0x7E, 0x7D, 0x7C


def leb(value):
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        if value:
            out.append(byte | 0x80)
        else:
            out.append(byte)
            return bytes(out)


def vec(items):
    return leb(len(items)) + b"".join(items)


def name(text):
    raw = text.encode()
    return leb(len(raw)) + raw


def section(ident, body):
    return bytes([ident]) + leb(len(body)) + body


def functype(params, results):
    return b"\x60" + vec([bytes([p]) for p in params]) + vec([bytes([r]) for r in results])


def module(*sections):
    return b"\x00asm\x01\x00\x00\x00" + b"".join(sections)


def sample():
    """two types; imports of a function, a memory and a global; one function
    of the module's own; exports of it, of the imported function and of the
    memory"""
    types = section(1, vec([functype([I32, I32], [I32]), functype([], [F64])]))
    imports = section(
        2,
        vec(
            [
                name("env") + name("halo_platform_tick") + b"\x00" + leb(0),
                name("env") + name("memory") + b"\x02\x00" + leb(1),
                name("env") + name("__stack_pointer") + b"\x03" + bytes([I32]) + b"\x01",
            ]
        ),
    )
    functions = section(3, vec([leb(1)]))
    exports = section(
        7,
        vec(
            [
                name("halo_game_main") + b"\x00" + leb(1),
                name("tick") + b"\x00" + leb(0),
                name("memory") + b"\x02" + leb(0),
            ]
        ),
    )
    return module(types, imports, functions, exports)


def test_a_module_s_imports_and_exports_are_read_with_their_types():
    parsed = read_module(sample())
    assert parsed.imports == [
        "env.halo_platform_tick: func (i32, i32) -> (i32)",
        "env.memory: memory",
        "env.__stack_pointer: global mut i32",
    ]
    assert parsed.exports == [
        "halo_game_main: func () -> (f64)",
        "tick: func (i32, i32) -> (i32)",
        "memory: memory",
    ]


def test_sections_the_reader_does_not_need_are_skipped():
    custom = section(0, name("producers") + b"anything")
    parsed = read_module(module(custom) + sample()[8:])
    assert parsed.exports[0] == "halo_game_main: func () -> (f64)"


@pytest.mark.parametrize(
    "data",
    [
        b"",
        b"\x00asm",
        b"\x7fELF\x01\x00\x00\x00",
        b"\x00asm\x02\x00\x00\x00",
        # a section longer than the module
        module(b"\x01\x10\x00"),
        # an import that ends inside its name
        module(section(2, leb(1) + leb(200) + b"env")),
        # a function import of a type the module does not have
        module(section(2, vec([name("env") + name("f") + b"\x00" + leb(5)]))),
        # an export of a function the module does not have
        module(section(7, vec([name("f") + b"\x00" + leb(9)]))),
        # an LEB128 longer than 32 bits
        module(section(1, b"\xff\xff\xff\xff\xff\x01")),
        # a count far beyond the bytes there are
        module(section(1, leb(0xFFFFFFF))),
    ],
)
def test_malformed_modules_are_refused(data):
    with pytest.raises(MalformedModule):
        read_module(data)


def test_every_truncation_of_a_module_is_refused_or_read():
    """no prefix of a valid module makes the reader fail other than by
    refusing it"""
    data = sample()
    for end in range(len(data)):
        try:
            read_module(data[:end])
        except MalformedModule:
            pass


@pytest.mark.skipif(not os.environ.get("HALO_WASM_CC"), reason="needs the wasm toolchain (nix develop)")
def test_game_wasm_imports_and_exports_are_the_reviewed_lists():
    output = engine_wasm.build(os.cpu_count() or 8)
    imports = (ROOT / engine_wasm.IMPORTS_LIST).read_text().splitlines()
    exports = (ROOT / engine_wasm.EXPORTS_LIST).read_text().splitlines()
    assert engine_wasm.module_imports(output) == imports
    assert engine_wasm.module_exports(output) == exports
    # the game's entry point, the heap the platform allocates from, and
    # the globals the platform reads (the D3D state tables, the pads)
    for export in engine_wasm.EXPORTS:
        assert any(line.startswith(f"{export}: ") for line in exports)
    # the game reaches the platform through "env" and wasi only (wasi's
    # thread-spawn starts its threads), and imports its shared memory
    assert {line.split(".", 1)[0] for line in imports} <= {"env", "wasi", "wasi_snapshot_preview1"}
    assert "env.memory: memory" in imports
    assert not any(line.endswith(": memory") for line in exports)
    # the C library's stdio is the module's own (port/wasmtime/crt.c)
    assert not any(line.startswith("env.halo_linux_fopen:") for line in imports)
    # and so is the reader it opens the game's files to read with
    # (port/wasmtime/data_files.c, through the platform's file imports)
    assert not any(line.startswith("env.halo_data_file_read:") for line in imports)


@pytest.mark.skipif(not os.environ.get("HALO_WASM_CC"), reason="needs the wasm toolchain (nix develop)")
def test_the_link_imports_undefined_functions_and_refuses_undefined_data(tmp_path):
    # a function the module does not define is the platform's, imported
    # from "env"; a variable it does not define has no platform to come
    # from, and left undefined it would be placed at address 0
    include, lib = engine_wasm.wasi_paths()

    def link(source):
        unit = tmp_path / "unit.c"
        unit.write_text(source)
        compiled = subprocess.run(
            [
                engine_wasm.clang(),
                f"--target={engine_wasm.TARGET}",
                f"-isystem{include}",
                "-c",
                str(unit),
                "-o",
                str(tmp_path / "unit.o"),
            ],
            capture_output=True,
            text=True,
        )
        assert compiled.returncode == 0, compiled.stderr
        return subprocess.run(
            [
                engine_wasm.clang(),
                f"--target={engine_wasm.TARGET}",
                "-nostdlib",
                "-Wl,--no-entry",
                "-Wl,--export=entry",
                *engine_wasm.UNDEFINED_FLAGS,
                str(tmp_path / "unit.o"),
                "-o",
                str(tmp_path / "unit.wasm"),
            ],
            capture_output=True,
            text=True,
        )

    imported = link("int platform(int); int entry(int x) { return platform(x); }\n")
    assert imported.returncode == 0, imported.stderr
    assert "env.platform: func (i32) -> (i32)" in engine_wasm.module_imports(tmp_path / "unit.wasm")
    refused = link("extern int missing; int entry(int x) { return missing + x; }\n")
    assert refused.returncode != 0
    assert "undefined symbol: missing" in refused.stderr


def test_the_game_s_units_find_game_wasm_s_own_profile_header_first(tmp_path):
    # profile_sections.h in port/wasmtime/include makes the game's sections
    # imports the runtime times; the Linux port's header of the same name
    # must not be found first
    import json

    config = json.loads((ROOT / engine_wasm.PORT_CONFIG).read_text(encoding="utf-8"))
    work = engine_wasm.units(config, ROOT / engine_wasm.BUILD, tmp_path)
    game = dict(work)[Path("source/shell/shell_xbox.c")]
    wasmtime = game.index(f"-I{engine_wasm.WASMTIME_INCLUDE}")
    linux = game.index(f"-I{engine_wasm.LINUX_DIR}/include")
    assert wasmtime < linux
    assert (ROOT / engine_wasm.WASMTIME_INCLUDE / "profile_sections.h").is_file()


def test_the_smaa_unit_defines_each_file_and_its_size(tmp_path):
    # the Metal device's post-processing (port/wasmtime/metal/d3d8_metal_post.c)
    # declares SMAA's shader and lookup textures; here a shader with a byte
    # past ASCII (the Metal compiler is handed it as text), and no textures
    assets = tmp_path / "smaa"
    assets.mkdir()
    (assets / "SMAA.hlsl").write_bytes(b"float4 x;\xc3\n")
    text = engine_wasm.smaa_unit(tmp_path, assets).read_text()
    assert "const unsigned int xgpu_smaa_shader[] = {" in text
    # its 11 bytes, the one past ASCII a space, and a NUL
    assert "const unsigned long xgpu_smaa_shader_size = 12;" in text
    assert "0x616f6c66, 0x78203474, 0x000a203b," in text
    for symbol in ("xgpu_smaa_area_texture", "xgpu_smaa_search_texture"):
        assert f"const unsigned int {symbol}[] = {{\n\t0,\n}};" in text
        assert f"const unsigned long {symbol}_size = 0;" in text


def test_the_smaa_unit_is_written_only_when_it_changes(tmp_path):
    assets = tmp_path / "smaa"
    assets.mkdir()
    (assets / "area_tex.zlib").write_bytes(b"\x78\x9c")
    written = engine_wasm.smaa_unit(tmp_path, assets).stat().st_mtime_ns
    assert engine_wasm.smaa_unit(tmp_path, assets).stat().st_mtime_ns == written
    (assets / "area_tex.zlib").write_bytes(b"\x78\x9c\x01")
    assert "xgpu_smaa_area_texture_size = 3;" in engine_wasm.smaa_unit(tmp_path, assets).read_text()
