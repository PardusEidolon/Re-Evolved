#!/usr/bin/env python3
"""Measures a menu frame drawn by the Halo UI plugin against the design's
regions (engine/crates/halo_ui/src/screens.rs, laid out on 3840 by 2160 and
scaled to the frame):

- the title rule at y 198 and the footer rule at y 1836, each 6 high: blue,
  and brighter than the panel they border (below the title rule, above the
  footer rule; over the title is the ui map's world, its ring, as bright as
  it likes);
- the title band (above the rule) has the title's blue letters in it;
- no colour from outside the design's palette, the blues and greys, shows
  in the panel: the stock menus' warm and green art showing through. Out
  of the panel is the ui map's world, its ring in its own colours.

    python tools/halo_ui_check.py frame.png... [--main]

`--main` is for the main menu, which has no title, footer or panel, only
its items over the ring: their blue ink is measured where they stand. Each check is measured, not looked at; a frame that
passes can still be laid out wrong, so the screens are approved by eye too.
Exits 0 when every frame passes, 1 when one does not.
"""

import argparse
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from frame_compare import read_png  # noqa: E402

DESIGN_WIDTH = 3840.0
TITLE_RULE = 198.0
FOOTER_RULE = 1836.0
RULE_HEIGHT = 6.0
# (a rule is measured against its panel's row this far from it)
RULE_CLEARANCE = 12.0
# (the rule's mean luminance over its panel's, and how much bluer than
# red it is: a dim over the whole screen lowers both, not the ratio)
RULE_CONTRAST = 1.5
RULE_BLUE = 20.0
# (a letter's pixel: blue enough, and clear of red; the title band's share)
INK_BLUE = 60
INK_OVER_RED = 30
INK_SHARE = 0.01
# (a colour the design has no use for: warmer than blue, or green over both)
FOREIGN_MARGIN = 24
FOREIGN_SHARE = 0.001
# (where the main menu's items stand, x and y, in the design's pixels)
MENU_ITEMS = (1400.0, 2440.0, 1150.0, 1560.0)


def _luminance(pixel):
    red, green, blue = pixel[:3]
    return 0.2126 * red + 0.7152 * green + 0.0722 * blue


def _row(image, y, step):
    """the row's pixels, every `step`th"""
    return [image.pixel(x, y) for x in range(0, image.width, step)]


def _mean(pixels, measure):
    return sum(measure(pixel) for pixel in pixels) / max(1, len(pixels))


def _rule(image, top, scale, step, panel_below):
    """(passed, what was measured) for a rule `top` design pixels down,
    against the panel below it (`panel_below`) or above it"""
    first = int(top * scale)
    last = max(first, math.ceil((top + RULE_HEIGHT) * scale) - 1)
    clear = max(1, round(RULE_CLEARANCE * scale))
    rows = range(first, min(last, image.height - 1) + 1)
    rule = max((_row(image, y, step) for y in rows), key=lambda row: _mean(row, _luminance))
    panel = last + clear if panel_below else first - clear
    lit = _mean(rule, _luminance)
    ground = _mean(_row(image, panel, step), _luminance) if 0 <= panel < image.height else 0.0
    blue = _mean(rule, lambda pixel: pixel[2] - pixel[0])
    contrast = lit / max(ground, 0.5)
    passed = contrast >= RULE_CONTRAST and blue >= RULE_BLUE
    return passed, f"{contrast:.2f}x its panel, blue over red {blue:.0f}"


def _ink(image, columns, rows):
    """the share of the pixels at `columns` by `rows` that are a letter's"""
    pixels = [image.pixel(x, y) for y in rows for x in columns]
    ink = sum(1 for pixel in pixels if pixel[2] >= INK_BLUE and pixel[2] - pixel[0] >= INK_OVER_RED)
    return ink / max(1, len(pixels))


def _title_ink(image, scale, step):
    bottom = int((TITLE_RULE - RULE_CLEARANCE) * scale)
    share = _ink(image, range(0, image.width, step), range(0, bottom, step))
    return share >= INK_SHARE, f"{share:.2%} of the band"


def _menu_ink(image, scale, step):
    left, right, top, bottom = (int(edge * scale) for edge in MENU_ITEMS)
    share = _ink(image, range(left, min(right, image.width), step), range(top, min(bottom, image.height), step))
    return share >= INK_SHARE, f"{share:.2%} of the items' box"


def _foreign(image, scale, step):
    top = int((TITLE_RULE + RULE_HEIGHT + RULE_CLEARANCE) * scale)
    bottom = min(image.height, int((FOOTER_RULE - RULE_CLEARANCE) * scale))
    pixels = [image.pixel(x, y) for y in range(top, bottom, step) for x in range(0, image.width, step)]

    def foreign(pixel):
        red, green, blue = pixel[:3]
        warm = red > blue + FOREIGN_MARGIN
        greenish = green > blue + FOREIGN_MARGIN and green > red + FOREIGN_MARGIN
        return warm or greenish

    share = sum(1 for pixel in pixels if foreign(pixel)) / max(1, len(pixels))
    return share <= FOREIGN_SHARE, f"{share:.3%} of the panel"


def check(image, main=False):
    """[(name, passed, what was measured)] for the frame"""
    scale = image.width / DESIGN_WIDTH
    # (a 4K frame is sampled every other pixel; a small one, every pixel)
    step = max(1, round(2 * scale))
    if main:
        return [("menu ink", *_menu_ink(image, scale, step))]
    return [
        ("title rule", *_rule(image, TITLE_RULE, scale, step, True)),
        ("footer rule", *_rule(image, FOOTER_RULE, scale, step, False)),
        ("title ink", *_title_ink(image, scale, step)),
        ("foreign colour", *_foreign(image, scale, step)),
    ]


def main(arguments=None):
    parser = argparse.ArgumentParser(description="Measures menu frames against the Halo UI design.")
    parser.add_argument("frames", type=Path, nargs="+")
    parser.add_argument("--main", action="store_true", help="the main menu: its items' ink alone")
    options = parser.parse_args(arguments)
    passed = True
    for path in options.frames:
        for name, ok, measured in check(read_png(path), options.main):
            passed = passed and ok
            print(f"{path.name}: {name}: {'pass' if ok else 'FAIL'} ({measured})")
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
