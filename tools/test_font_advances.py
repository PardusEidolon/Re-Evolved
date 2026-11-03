"""The font reader of tools/font_advances.py: the menus' fonts read, and a
font cut short or scrambled refused rather than read past its end."""

import random
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent))

import font_advances  # noqa: E402


def test_the_table_is_current():
    assert font_advances.table() == font_advances.OUTPUT.read_text()


def test_a_capital_is_wider_in_heavy_than_bold():
    fonts = {
        file: font_advances.Font((font_advances.FONTS / file).read_bytes())
        for file in ("Overpass-900.ttf", "Overpass-750.ttf")
    }
    widths = {file: font.advance(font.glyph(ord("M"))) / font.units_per_em() for file, font in fonts.items()}
    assert widths["Overpass-900.ttf"] > widths["Overpass-750.ttf"] > 0.5


@pytest.mark.parametrize("seed", range(64))
def test_a_broken_font_is_refused(seed):
    data = bytearray((font_advances.FONTS / "Overpass-900.ttf").read_bytes())
    chance = random.Random(seed)
    if seed % 2:
        data = data[: chance.randrange(len(data))]
    else:
        for _ in range(32):
            data[chance.randrange(len(data))] = chance.randrange(256)
    try:
        font = font_advances.Font(bytes(data))
        for code in range(font_advances.FIRST, font_advances.LAST + 1):
            font.advance(font.glyph(code))
        font.units_per_em()
    except ValueError:
        pass
