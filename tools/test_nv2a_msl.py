"""The NV2A shader translators' Metal Shading Language (port/linux/src/nv2a_vsh.c,
nv2a_psh.c), which OpenCE.app's renderer draws with.

Every vertex program of the game (source/rasterizer/xbox/
rasterizer_xbox_vertex_shaders_data.inc), and its per-pixel lighting variant
where it has one, and pixel shader keys of each of the translators' paths are
translated by the translators compiled as game.wasm compiles them, into a
WASI command run in its runtime (port/wasmtime/tests/nv2a_print.c), then all
compiled by Metal with the renderer's options (tools/msl_check.swift). The GLSL the GL builds draw with is written by the
same code: its output must not change with the MSL's.

Needs `nix develop`, and a Metal device.
"""

import os
import platform
import re
import shutil
import subprocess
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from tools import engine_wasm, engine_wasm_cc, harness  # noqa: E402

TRANSLATORS = [ROOT / f"port/linux/src/{name}.c" for name in ("nv2a_vsh", "nv2a_psh", "xgpu_text")]
DRIVER = [ROOT / "port/wasmtime/tests/nv2a_print.c", ROOT / "port/wasmtime/tests/nv2a_stubs.c"]
DATA = ROOT / "source/rasterizer/xbox/rasterizer_xbox_vertex_shaders_data.inc"
TABLE = ROOT / "source/rasterizer/xbox/rasterizer_xbox_vertex_shaders.c"
CHECK = ROOT / "tools/msl_check.swift"

pytestmark = [
    pytest.mark.skipif(sys.platform != "darwin" or platform.machine() != "arm64", reason="arm64 macOS only"),
    pytest.mark.skipif(not os.environ.get("HALO_WASM_BUILTINS"), reason="needs nix develop"),
    pytest.mark.skipif(shutil.which("xcrun") is None, reason="needs Xcode's swift"),
]

# the attributes as the game's vertex types feed them: (packed, present)
ATTRIBUTES = [(0x0000, 0xFFFF), (0x000C, 0x001F), (0x0000, 0x0001)]

PIXEL_CASES = [
    "plain",
    "diffuse",
    "texture",
    "fog",
    "fog_exp2",
    "mux",
    "mux_lsb",
    "alpha_test",
    "alpha_test_samples",
    "alpha_test_never",
    "alpha_kill",
    "lighting",
    "point_lighting",
    "meter",
    "blend",
    "count_samples",
    "shadowed",
    "fog_scattering",
    "fog_pass",
    "fog_pass_scattering",
]

# (stage, PSTextureModes' mode, sampler): every mode, each with the samplers
# it is drawn with (_xgpu_sampler_2d 1, 3d 2, cube 3)
TEXTURE_MODES = (
    [(0, mode, sampler) for mode, sampler in ((0x01, 1), (0x02, 2), (0x03, 3), (0x04, 1), (0x05, 1))]
    + [(1, mode, sampler) for mode in (0x06, 0x07, 0x08, 0x09, 0x0F) for sampler in (1,)]
    + [(2, 0x0A, 1), (2, 0x0B, 1), (2, 0x0E, 1), (2, 0x10, 1), (2, 0x11, 1), (2, 0x12, 1)]
    + [(3, 0x0C, 3), (3, 0x0D, 3), (3, 0x0D, 2), (3, 0x01, 2), (3, 0x03, 3)]
)


def vertex_programs():
    """[(index, instruction words)] of the game's vertex shader table"""
    words = [int(word, 16) for word in re.findall(r"0x([0-9A-Fa-f]{8})", DATA.read_text().split("*/", 1)[1])]
    programs = []
    for index, (offset, size) in enumerate(
        re.findall(r"VERTEX_SHADER_ENTRY\((0x[0-9A-Fa-f]+), (0x[0-9A-Fa-f]+)\)", TABLE.read_text())
    ):
        start = int(offset, 16) // 4
        header = words[start]
        count = header >> 16
        assert header & 0xFFFF == 0x2078 and 4 + count * 16 == int(size, 16), index
        programs.append((index, words[start + 1 : start + 1 + count * 4]))
    return programs


@pytest.fixture(scope="module")
def nv2a_print(tmp_path_factory):
    """the driver and the translators as one WASI command, the translators
    with game.wasm's flags for them (engine_wasm.metal_flags; wasm_command
    gives the ABI's own)"""
    work = tmp_path_factory.mktemp("nv2a_print")
    engine_wasm.semantics_header(work)
    include, _ = engine_wasm.wasi_paths(engine_wasm_cc.TARGET)
    flags = engine_wasm.metal_flags(work, include)[len(engine_wasm.ABI_FLAGS) + 1 :]
    module = work / "nv2a_print.wasm"
    run = subprocess.run(
        engine_wasm_cc.wasm_command([*flags, *map(str, DRIVER + TRANSLATORS)], module),
        capture_output=True,
        text=True,
        cwd=ROOT,
    )
    assert run.returncode == 0, run.stderr
    return module


def translate(module, folder, *arguments):
    run = harness.process(module, [str(argument) for argument in arguments], capture_output=True, text=True, cwd=folder)
    return run.returncode, run.stdout, run.stderr


@pytest.fixture(scope="module")
def shaders(nv2a_print, tmp_path_factory):
    """{name: (stage, glsl, msl)} of every case"""
    folder = tmp_path_factory.mktemp("nv2a_print_run")
    cases = {}
    for index, words in vertex_programs():
        hex_words = ["%x" % word for word in words]
        for packed, present in ATTRIBUTES:
            name = f"vertex_{index}_{packed:x}_{present:x}"
            outputs = []
            for language in ("glsl", "msl"):
                code, out, err = translate(
                    nv2a_print, folder, "vertex", language, "%x" % packed, "%x" % present, "unlit", *hex_words
                )
                assert code == 0, f"{name} {language}: {err}"
                outputs.append(out)
            cases[name] = ("vertex", *outputs)
        outputs = []
        for language in ("glsl", "msl"):
            code, out, err = translate(nv2a_print, folder, "vertex", language, "0", "ffff", "lit", *hex_words)
            outputs.append(out if code == 0 else None)
        if outputs[1] is not None:
            cases[f"vertex_{index}_lit"] = ("vertex", *outputs)
        # the shadowed light passes' (MSL only: the GL builds draw none)
        code, out, err = translate(nv2a_print, folder, "vertex", "msl", "0", "ffff", "shadowed", *hex_words)
        assert code == 0, f"vertex {index} shadowed: {err}"
        cases[f"vertex_{index}_shadowed"] = ("vertex", None, out)
    for case in PIXEL_CASES:
        outputs = []
        for language in ("glsl", "msl"):
            code, out, err = translate(nv2a_print, folder, "pixel", language, case)
            assert code == 0, f"{case} {language}: {err}"
            outputs.append(out)
        cases[f"pixel_{case}"] = ("fragment", *outputs)
    for stage, mode, sampler in TEXTURE_MODES:
        outputs = []
        for language in ("glsl", "msl"):
            code, out, err = translate(nv2a_print, folder, "pixel", language, "mode", stage, "0x%x" % mode, sampler)
            assert code == 0, f"mode {stage} {mode:#x} {sampler} {language}: {err}"
            outputs.append(out)
        cases[f"pixel_mode_{stage}_{mode:x}_{sampler}"] = ("fragment", *outputs)
    return cases


def metal_compile(paths):
    # Xcode's Swift, outside the Nix toolchain, as the app is built
    # (tools/engine_build.py)
    environment = {
        key: value
        for key, value in os.environ.items()
        if key not in ("SDKROOT", "DEVELOPER_DIR", "MACOSX_DEPLOYMENT_TARGET", "CC", "CXX", "LD")
    }
    run = subprocess.run(
        ["/usr/bin/xcrun", "swift", str(CHECK), *map(str, paths)],
        capture_output=True,
        text=True,
        timeout=600,
        env=environment,
    )
    results = {}
    for line in run.stdout.splitlines():
        if line.startswith("/"):
            path, _, message = line.partition(": ")
            results[Path(path).name] = message
        elif results:
            last = list(results)[-1]
            results[last] += "\n" + line
    if not results:
        pytest.skip("Metal could not be run: " + run.stdout + run.stderr)
    return run.returncode, results


def write(folder, shaders):
    paths = []
    for name, (stage, _, msl) in shaders.items():
        path = folder / f"{name}.{stage}.metal"
        path.write_text(msl)
        paths.append(path)
    return paths


def test_every_program_was_translated(shaders):
    vertex = [name for name in shaders if name.startswith("vertex_") and not name.endswith(("_lit", "_shadowed"))]
    assert len(vertex) == len(vertex_programs()) * len(ATTRIBUTES) == 67 * len(ATTRIBUTES)
    # the game's four model lighting programs (rasterizer_xbox_vertex_shaders_
    # initialize.c names them to halo_vertex_shader_lighting), which the
    # analysis finds lighting in; it finds it in program 31 too, which the
    # game does not light
    lit = {name for name in shaders if name.endswith("_lit")}
    assert {f"vertex_{index}_lit" for index in (9, 10, 17, 27)} <= lit
    assert len(lit) < 10


def test_the_msl_is_metal_not_gles(shaders):
    for name, (stage, glsl, msl) in shaders.items():
        assert "#version" not in msl and "precision " not in msl, name
        assert "#include <metal_stdlib>" in msl, name
        assert "gl_" not in msl, name
        if stage == "vertex":
            assert "vertex nv2a_vertex_out vertex_main(" in msl, name
            assert "[[position, invariant]]" in msl, name
            # Metal's clip space is GL's with glClipControl(UPPER_LEFT,
            # ZERO_TO_ONE): no flip of y and no remap of z, as the GLES path has
            assert "position.y = -" not in msl, name
            assert "* 2.0 - " not in msl.split("output.position")[-1], name
            assert "static_assert(sizeof(nv2a_vertex_uniforms) == " in msl, name
        else:
            assert "fragment nv2a_fragment_out fragment_main(" in msl, name
            assert "discard;" not in msl, name
            assert "static_assert(sizeof(nv2a_pixel_uniforms) == " in msl, name


def test_the_paths_are_translated_to_metal(shaders):
    msl = {name: source for name, (_, _, source) in shaders.items()}
    # textures sample with the Xbox's LOD bias, which Metal's samplers lack too
    assert ".sample(sampler0, " in msl["pixel_texture"] and "bias(texture_lod_bias[0])" in msl["pixel_texture"]
    assert "texturecube<float> tex0 [[texture(0)]]" in msl["pixel_mode_0_3_3"]
    assert "texture3d<float> tex0 [[texture(0)]]" in msl["pixel_mode_0_2_2"]
    assert "texture2d<float>" not in msl["pixel_plain"]
    for case in ("alpha_test", "alpha_kill", "alpha_test_never"):
        assert "discard_fragment()" in msl[f"pixel_{case}"], case
    # the multisampled alpha test covers samples in proportion
    assert "[[sample_mask]]" in msl["pixel_alpha_test_samples"]
    assert "[[sample_mask]]" not in msl["pixel_alpha_test"]
    # per-pixel lighting: the normal (and the position, by the point lights)
    # from the vertex shader to the pixel shader
    assert "[[user(xWorldNormal)]]" in msl["vertex_9_lit"]
    assert "model_lights" in msl["pixel_lighting"] and "xWorldNormal" in msl["pixel_lighting"]
    assert "xWorldPosition" in msl["pixel_point_lighting"]
    # an absent attribute reads its constant; a packed one is unpacked
    assert "[[attribute(1)]]" not in msl["vertex_0_0_1"] and "u.attributes[1]" in msl["vertex_0_0_1"]
    assert "uint v2_packed [[attribute(2)]]" in msl["vertex_0_c_1f"]
    # the shadowed light passes: the vertex's and the light's clip positions
    # to the pixel shader, which steps between them through the depth copy
    for name in ("vertex_0_shadowed", "vertex_9_shadowed"):
        assert "[[user(xShadowFrom)]]" in msl[name] and "[[user(xShadowTo)]]" in msl[name], name
        assert "c[15].xyz" in msl[name] and "dot(c[3], light)" in msl[name], name
    assert "depth2d<float> shadow_depth [[texture(4)]]" in msl["pixel_shadowed"]
    assert "shadow_visibility(shadow_depth, input.xShadowFrom, input.xShadowTo)" in msl["pixel_shadowed"]
    for name, source in msl.items():
        if not name.endswith("shadowed"):
            assert "xShadow" not in source and "shadow_depth" not in source, name


def test_the_fog_scatters_by_its_colour_with_lighting_atmosphere(shaders):
    msl = {name: source for name, (_, _, source) in shaders.items()}
    # lighting.atmosphere scattering: the fog's transmittance raised to a
    # power for each channel, from the fog colour's share of its sum (grey
    # fog a power of one: stock), in the final combiner where it blends the
    # fog colour in by a scalar
    scattered = msl["pixel_fog_scattering"]
    assert "vec3 fog_weight = clamp(3.0 * fC / max(fC.r + fC.g + fC.b, 1.0e-4), 0.25, 4.0);" in scattered
    assert "fA = pow(max(fA, vec3(1.0e-30)), fog_weight);" in scattered
    # and in the world's fog pass, premultiplied and blended by its alpha,
    # through the second source of a dual-source blend
    pass_scattered = msl["pixel_fog_pass_scattering"]
    assert "float4 transmittance [[color(0), index(1)]];" in pass_scattered
    assert "output.transmittance = vec4(fog_through, 1.0);" in pass_scattered
    for name, source in msl.items():
        if not name.endswith("_scattering"):
            assert "fog_weight" not in source and "index(1)" not in source, name
    # the GL builds draw it stock
    for name in ("pixel_fog_scattering", "pixel_fog_pass_scattering"):
        _, glsl, _ = shaders[name]
        assert "fog_weight" not in glsl, name
    assert shaders["pixel_fog_pass_scattering"][1] == shaders["pixel_fog_pass"][1]


def test_the_glsl_is_unchanged(shaders):
    # the GL builds' output, made by the same functions, has none of Metal's
    for name, (stage, glsl, _) in shaders.items():
        if glsl is None:
            continue
        assert glsl.startswith("#version 300 es"), name
        assert "metal" not in glsl and "[[" not in glsl and "static_assert" not in glsl, name


def test_metal_compiles_every_shader(shaders, tmp_path):
    paths = write(tmp_path, shaders)
    code, results = metal_compile(paths)
    failed = {name: message for name, message in results.items() if message != "ok"}
    assert not failed, "\n".join(f"{name}: {message}" for name, message in list(failed.items())[:10])
    assert code == 0 and len(results) == len(paths)


def test_metal_refuses_a_mismatched_uniform_layout(shaders, tmp_path):
    # the negative control: the layouts are checked against the C structs'
    # (a shader whose uniforms the C side lays out differently does not compile)
    _, _, msl = shaders["pixel_texture"]
    size = re.search(r"static_assert\(sizeof\(nv2a_pixel_uniforms\) == (\d+)", msl)
    assert size
    path = tmp_path / "mismatched.fragment.metal"
    path.write_text(msl.replace(size.group(0), size.group(0)[: -len(size.group(1))] + str(int(size.group(1)) + 16)))
    code, results = metal_compile([path])
    assert code == 1 and results["mismatched.fragment.metal"] != "ok"


def test_the_renderer_compiles_with_the_checked_options():
    # tools/msl_check.swift compiles as GameRenderer.libraryOptions does
    renderer = (ROOT / "engine/app/Sources/OpenCEEngineRender/GameRenderer.swift").read_text()
    check = CHECK.read_text()
    options = re.findall(r"options\.(\w+) = (\S+)", check)
    assert len(options) == 3
    for name, value in options:
        assert f"options.{name} = {value}" in renderer, name
    assert "makeLibrary(source: source, options: Self.libraryOptions)" in renderer
