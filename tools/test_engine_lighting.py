"""The lighting sweeps (tools/engine_lighting.py): each lighting extension
switched on alone and its frames measured against stock's, region by
region: brightness, colour, seams (where the change itself jumps) and
shadow edges (stock's edges kept, and how sharp). Pure functions only: the
runs need the user's maps and are made by `engine_lighting.py run`."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))

import engine_audit as audit  # noqa: E402
import engine_lighting as lighting  # noqa: E402
from frame_compare import Image  # noqa: E402


def image(width: int, height: int, colour) -> Image:
    """`colour(x, y)` is an (r, g, b) for each pixel"""
    pixels = bytearray()
    for y in range(height):
        for x in range(width):
            pixels += bytes(colour(x, y)) + b"\xff"
    return Image(width, height, bytes(pixels))


GREY = image(8, 8, lambda x, y: (100, 100, 100))


def test_each_extension_is_switched_on_alone_over_stock() -> None:
    assert list(lighting.EXTENSIONS) == ["placed_lights", "bounce", "atmosphere", "shadows", "reflections"]
    assert lighting.settings("stock") == [*audit.STOCK, *lighting.ALL_STOCK]
    bounce = lighting.settings("bounce")
    assert "lighting.bounce=baked" in bounce
    assert "lighting.bounce=stock" not in bounce
    assert [setting for setting in bounce if setting.startswith("lighting.")].count("lighting.shadows=stock") == 1
    command = lighting.sweep_command(Path("out"), "bloodgulch", "shadows", 25)
    assert command[1:7] == ["--audit", "out", "--map", "bloodgulch", "--seconds", "25"]
    switched = [command[index + 1] for index, word in enumerate(command) if word == "--setting"]
    assert switched == ["debug.frame_clock=true", *lighting.settings("shadows")]


def test_brightness_and_colour_are_measured_per_region() -> None:
    brighter = image(8, 8, lambda x, y: (120, 120, 120) if x < 2 and y < 2 else (100, 100, 100))
    regions = lighting.light_regions(GREY, brighter, grid=4)
    assert len(regions) == 16
    assert regions[0]["brightness"] == 20.0
    assert regions[0]["colour"] == 0.0
    assert all(region == {"brightness": 0.0, "colour": 0.0} for region in regions[1:])
    # a warmer tint with the same luma is colour, not brightness
    warmer = image(8, 8, lambda x, y: (110, 98, 90))
    tinted = lighting.light_regions(GREY, warmer, grid=4)[5]
    assert abs(tinted["brightness"]) < 1.0
    assert tinted["colour"] > 5.0


def test_a_seam_is_where_the_change_itself_jumps() -> None:
    assert lighting.seams(GREY, GREY) == 0
    # light added smoothly across the frame: no seam
    smooth = image(8, 8, lambda x, y: (100 + 4 * x,) * 3)
    assert lighting.seams(GREY, smooth) == 0
    # light added to the left half only: a seam down the middle, a pixel a row
    half = image(8, 8, lambda x, y: (140,) * 3 if x < 4 else (100,) * 3)
    assert lighting.seams(GREY, half) == 8
    # an edge that stock has too is not a seam
    stock_edge = image(8, 8, lambda x, y: (200,) * 3 if x < 4 else (100,) * 3)
    assert lighting.seams(stock_edge, stock_edge) == 0


def test_stock_shadow_edges_are_kept_and_their_sharpness_measured() -> None:
    edge = image(8, 8, lambda x, y: (200,) * 3 if x < 4 else (40,) * 3)
    assert lighting.shadow_edges(edge, edge) == {"edges": 16, "kept": 1.0, "sharpness": 1.0}
    # the same edge softened over four pixels: kept, but less sharp
    soft = image(8, 8, lambda x, y: (max(40, min(200, 200 - 40 * (x - 2))),) * 3)
    softened = lighting.shadow_edges(edge, soft)
    assert softened["kept"] == 1.0
    assert softened["sharpness"] < 0.75
    # the edge gone
    assert lighting.shadow_edges(edge, GREY)["kept"] == 0.0
    # nothing to keep
    assert lighting.shadow_edges(GREY, edge) == {"edges": 0, "kept": 1.0, "sharpness": 1.0}


def test_the_sweep_finds_what_moved_more_than_stock_against_stock() -> None:
    noise = {"brightness": 0.0, "colour": 0.0, "seams": 0, "lost": 0.0}
    limits = lighting.limits(noise)
    assert limits == lighting.FLOORS
    metrics = {"brightness": 6.5, "colour": 1.0, "seams": 3, "lost": 0.02}
    assert [finding["measure"] for finding in lighting.findings("a30", "bounce", metrics, limits)] == [
        "brightness",
        "seams",
    ]
