"""tools/halo_ui_check.py: a drawn menu frame measured against the design's
regions (its title and footer rules, the title's ink, and no colour from
outside the design's palette)."""

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import halo_ui_check  # noqa: E402
from frame_compare import Image  # noqa: E402

WIDTH, HEIGHT = 384, 216  # a tenth of the design's 3840 by 2160


def frame(paint=None):
    """the design's ground, with the rules and a title, then `paint`'s
    changes (x, y) -> (r, g, b)"""
    rows = []
    for y in range(HEIGHT):
        row = bytearray()
        for x in range(WIDTH):
            colour = (2, 8, 18)
            if y in (19, 20, 183, 184):  # the rules at 198 and 1836, a tenth
                colour = (29, 106, 180)
            if 4 <= y <= 14 and 12 <= x <= 140 and x % 3:
                colour = (79, 169, 254)  # the title's letters
            if paint:
                colour = paint(x, y) or colour
            row += bytes(colour) + b"\xff"
        rows.append(bytes(row))
    return Image(WIDTH, HEIGHT, b"".join(rows))


class HaloUiCheckTests(unittest.TestCase):
    def test_a_frame_drawn_to_the_design_passes(self):
        results = halo_ui_check.check(frame())
        self.assertEqual([name for name, passed, _ in results if not passed], [])
        self.assertEqual(
            [name for name, _, _ in results],
            ["title rule", "footer rule", "title ink", "foreign colour"],
        )

    def test_a_missing_rule_fails(self):
        image = frame(lambda x, y: (2, 8, 18) if y in (183, 184) else None)
        failed = [name for name, passed, _ in halo_ui_check.check(image) if not passed]
        self.assertEqual(failed, ["footer rule"])

    def test_a_grey_rule_is_not_the_designs(self):
        image = frame(lambda x, y: (120, 120, 120) if y in (19, 20) else None)
        failed = [name for name, passed, _ in halo_ui_check.check(image) if not passed]
        self.assertIn("title rule", failed)

    def test_an_empty_title_fails(self):
        image = frame(lambda x, y: (2, 8, 18) if y < 18 else None)
        failed = [name for name, passed, _ in halo_ui_check.check(image) if not passed]
        self.assertEqual(failed, ["title ink"])

    def test_the_ui_maps_world_above_the_title_rule_passes(self):
        # (the ring scene, bright in the open band over the panel: each rule
        # is measured against the panel it borders)
        image = frame(lambda x, y: (60, 70, 90) if y < 19 and not (4 <= y <= 14 and 12 <= x <= 140) else None)
        failed = [name for name, passed, _ in halo_ui_check.check(image) if not passed]
        self.assertEqual(failed, [])

    def test_stock_art_showing_through_fails(self):
        # (the stock menus' warm and green art, a patch of the content)
        def stock(x, y):
            return (150, 110, 40) if 100 <= x < 180 and 60 <= y < 120 else None

        failed = [name for name, passed, _ in halo_ui_check.check(frame(stock)) if not passed]
        self.assertEqual(failed, ["foreign colour"])

    def test_the_worlds_colour_outside_the_panel_passes(self):
        # (the ring's land, green and brown, over the title rule)
        image = frame(lambda x, y: (40, 120, 30) if y < 3 else None)
        failed = [name for name, passed, _ in halo_ui_check.check(image) if not passed]
        self.assertEqual(failed, [])

    def test_the_main_menu_is_measured_by_its_items(self):
        # (its items' blue in the middle, over the ring in all its colours)
        def main_menu(x, y):
            if 154 <= x < 230 and 123 <= y < 150 and x % 3:
                return (39, 150, 254)
            return (40, 120, 30) if y < 100 else (90, 80, 70)

        results = halo_ui_check.check(frame(main_menu), main=True)
        self.assertEqual([(name, passed) for name, passed, _ in results], [("menu ink", True)])
        empty = halo_ui_check.check(frame(lambda x, y: (90, 80, 70)), main=True)
        self.assertEqual([(name, passed) for name, passed, _ in empty], [("menu ink", False)])

    def test_a_dimmed_frame_still_finds_its_rules(self):
        # (the keyboard's dim over the whole screen, 0.72 of (1, 5, 13))
        def dim(x, y):
            return None

        image = frame(dim)
        pixels = bytearray(image.pixels)
        for at in range(0, len(pixels), 4):
            for channel, over in enumerate((1, 5, 13)):
                pixels[at + channel] = round(pixels[at + channel] * 0.28 + over * 0.72)
        results = halo_ui_check.check(Image(WIDTH, HEIGHT, bytes(pixels)))
        failed = [name for name, passed, _ in results if not passed]
        self.assertEqual(failed, [])


if __name__ == "__main__":
    unittest.main()
