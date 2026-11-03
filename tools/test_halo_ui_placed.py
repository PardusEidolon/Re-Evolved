"""The texels of a picture that the Halo UI plugin placed (the campaign's
mission cards, port/linux/include/halo_ui_pointer.h's
halo_ui_placed_texels): a stock level's picture is its widget's bounds of
texels at the top left of a larger bitmap, as the stock menus draw it, so
the placed draw takes those texels, not the whole bitmap, cropped from the
right or the bottom to the shape of the hole it fills."""

import shutil
import subprocess
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent

PROGRAM = r"""
#include <stdio.h>
#include "halo_ui_pointer.h"

int main(void)
{
	static const short cases[][6] = {
		/* picture, bitmap, hole */
		{ 200, 145, 256, 256, 540, 390 },
		{ 200, 100, 256, 256, 540, 390 },
		{ 400, 300, 256, 256, 540, 390 },
		{ 256, 256, 256, 256, 540, 390 },
		{ 200, 145, 256, 256, 0, 390 },
		{ 0, 145, 256, 256, 540, 390 },
	};
	unsigned index;

	for (index = 0; index < sizeof(cases) / sizeof(cases[0]); index++)
	{
		short texels[2];
		halo_ui_placed_texels(cases[index][0], cases[index][1], cases[index][2],
			cases[index][3], cases[index][4], cases[index][5], texels);
		printf("%d %d\n", texels[0], texels[1]);
	}
	return 0;
}
"""


def test_a_placed_picture_takes_its_widget_s_texels_cropped_to_its_hole(tmp_path):
    compiler = shutil.which("clang") or shutil.which("cc")
    if not compiler:
        pytest.skip("needs a C compiler")
    # (the header alone: port/linux/include has its own stdio.h)
    shutil.copy(ROOT / "port/linux/include/halo_ui_pointer.h", tmp_path)
    (tmp_path / "main.c").write_text(PROGRAM)
    program = tmp_path / "placed"
    subprocess.run(
        [compiler, "-std=c99", "-Wall", "-Werror", str(tmp_path / "main.c"), "-o", str(program)],
        check=True,
        timeout=60,
    )
    lines = subprocess.run([program], check=True, capture_output=True, text=True, timeout=10).stdout.split("\n")
    assert lines[:6] == [
        # 200x145 is narrower than 540x390: cropped from the bottom
        "200 144",
        # a wide picture: cropped from the right
        "138 100",
        # larger than its bitmap: the bitmap's 256x256, then cropped
        "256 184",
        "256 184",
        # a hole or picture with no size: the texels as they are, not cropped
        "200 145",
        "0 145",
    ]
