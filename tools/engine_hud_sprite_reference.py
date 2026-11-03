"""The game's HUD bitmap lookups: which image of a bitmap, and which part
of it, a sequence's frame is drawn from, recorded for the engine's HUD to
match.

    python3 tools/engine_hud_sprite_reference.py          # writes the reference
    python3 tools/engine_hud_sprite_reference.py --check  # fails if it is stale

The functions are source/bitmaps/bitmap_group.c's
`bitmap_group_get_bitmap_from_sequence` (which static elements and meters
draw from) and source/interface/hud_draw.c's
`hud_retrieve_bitmap_and_bounding_rect` and `get_sprite_clip_rect` (which
numbers and crosshairs draw from), compiled for this machine against a
small fake world: three bitmaps, one without sequences, one whose
sequences are runs of images, sprites and neither, and one of sprites
alone and a run without a first image, asked for each sequence from below
the first to past the last and each frame from 0 to past the last. The table is written to
tools/traces/hud_sprite_reference.txt, which
tools/engine_wasm_traces.py checks game.wasm against.
"""

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from harness import function, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "hud_sprite_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef byte boolean;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define _error_silent 0
#define match_assert(file, line, condition) ((void)0)
#define match_assert_stack_frame(file, line) ((void)0)
#define error(...) ((void)0)
#define get_return_eip() 0
#define csmemset memset
enum { STACK_BUFFER_LENGTH = 0x80 };

typedef union { real n[4]; struct { real x0; real x1; real y0; real y1; }; } real_rectangle2d;
typedef struct { real x, y; } real_point2d;

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

struct bitmap_data { long id; };

/* the group's fields the lookups read */
struct bitmap_group { struct tag_block sequences; struct tag_block bitmaps; };

static boolean hud_bad_bitmap_reported;
"""

WORLD = r"""
static struct bitmap_data bitmaps[12] = { {0}, {1}, {2}, {3}, {4}, {5}, {6}, {7}, {8}, {9}, {10}, {11} };

/* a bitmap with sprites of the sequences that have them */
static struct bitmap_group_sprite sprites_a[4] =
{
	{ 0, 0, 0, { { 0.0f, 0.25f, 0.0f, 0.5f } }, { 0.0f, 0.0f } },
	{ 5, 0, 0, { { 0.25f, 0.5f, 0.0f, 0.5f } }, { 0.0f, 0.0f } },
	{ 11, 0, 0, { { 0.5f, 0.75f, 0.5f, 1.0f } }, { 0.0f, 0.0f } },
	{ 3, 0, 0, { { 0.75f, 1.0f, 0.125f, 0.875f } }, { 0.0f, 0.0f } },
};
static struct bitmap_group_sprite sprites_b[1] =
{
	{ 2, 0, 0, { { 0.1f, 0.9f, 0.2f, 0.8f } }, { 0.0f, 0.0f } },
};
static struct bitmap_group_sprite sprites_c[3] =
{
	{ 1, 0, 0, { { 0.0f, 0.5f, 0.0f, 0.5f } }, { 0.0f, 0.0f } },
	{ 0, 0, 0, { { 0.5f, 1.0f, 0.0f, 0.5f } }, { 0.0f, 0.0f } },
	{ 2, 0, 0, { { 0.0f, 1.0f, 0.5f, 1.0f } }, { 0.0f, 0.0f } },
};

/* runs of images, sprites, neither and a run to the last image */
static struct bitmap_group_sequence sequences_one[4] =
{
	{ "run", 2, 3, { 0 }, { 0, NULL } },
	{ "sprites", NONE, 0, { 0 }, { 4, sprites_a } },
	{ "neither", NONE, 0, { 0 }, { 0, NULL } },
	{ "to the last", 7, 5, { 0 }, { 0, NULL } },
};
static struct bitmap_group_sequence sequences_two[3] =
{
	{ "a sprite", NONE, 0, { 0 }, { 1, sprites_b } },
	{ "sprites", NONE, 0, { 0 }, { 3, sprites_c } },
	{ "a run without a first", NONE, 2, { 0 }, { 0, NULL } },
};

#define GROUPS 3
static struct bitmap_group groups[GROUPS] =
{
	{ { 0, NULL }, { 5, bitmaps } },
	{ { 4, sequences_one }, { 12, bitmaps } },
	{ { 3, sequences_two }, { 3, bitmaps } },
};

static struct bitmap_group *bitmap_group_get(long index) { return &groups[index]; }
"""

MAIN = r"""
static void print_group(int g)
{
	int s, i;

	printf("group %d %d\n", g, groups[g].bitmaps.count);
	for (s = 0; s<groups[g].sequences.count; s++)
	{
		struct bitmap_group_sequence *sequence = &((struct bitmap_group_sequence *)groups[g].sequences.address)[s];

		printf("sequence %d %d %d %d %d\n", g, s, sequence->first_bitmap_index, sequence->bitmap_count, sequence->sprites.count);
		for (i = 0; i<sequence->sprites.count; i++)
		{
			struct bitmap_group_sprite *sprite = &((struct bitmap_group_sprite *)sequence->sprites.address)[i];

			printf("sprite %d %d %d %d %.9g %.9g %.9g %.9g\n", g, s, i, sprite->bitmap_index,
				sprite->bounds.x0, sprite->bounds.x1, sprite->bounds.y0, sprite->bounds.y1);
		}
	}
}

static void print_clip(real_rectangle2d const *clip)
{
	if (clip)
		printf(" %.9g %.9g %.9g %.9g\n", clip->x0, clip->x1, clip->y0, clip->y1);
	else
		printf(" none\n");
}

int main(void)
{
	static short const frames[] = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 17, 23, 31, 39, 255, 32767 };
	int g, s, f;

	for (g = 0; g<GROUPS; g++)
		print_group(g);
	for (g = 0; g<GROUPS; g++)
		for (s = -2; s<=5; s++)
			for (f = 0; f<(int)(sizeof(frames)/sizeof(frames[0])); f++)
			{
				short frame = frames[f];
				struct bitmap_data const *bitmap = NULL;
				real_rectangle2d const *clip = NULL;
				struct bitmap_data *generic;
				int wraps = groups[g].sequences.count>0 && s>=0;

				/* a sprite past the last of a sequence without a run is read
				from past the block: not asked for */
				if (!(wraps && ((struct bitmap_group_sequence *)groups[g].sequences.address)[s%groups[g].sequences.count].bitmap_count==0 &&
					frame>=((struct bitmap_group_sequence *)groups[g].sequences.address)[s%groups[g].sequences.count].sprites.count &&
					((struct bitmap_group_sequence *)groups[g].sequences.address)[s%groups[g].sequences.count].sprites.count>0))
				{
					generic = bitmap_group_get_bitmap_from_sequence(g, s, frame);
					printf("static %d %d %d %d", g, s, frame, generic ? generic->id : -1);
					print_clip(get_sprite_clip_rect(g, s, frame));
				}
				hud_retrieve_bitmap_and_bounding_rect(g, s, frame, &bitmap, &clip);
				printf("hud %d %d %d %d", g, s, frame, bitmap ? bitmap->id : -1);
				print_clip(bitmap ? clip : NULL);
			}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    bitmap_group_h = read("source/bitmaps/bitmap_group.h")
    bitmap_group_c = read("source/bitmaps/bitmap_group.c")
    hud_draw_c = read("source/interface/hud_draw.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            structure(bitmap_group_h, "bitmap_group_sprite"),
            structure(bitmap_group_h, "bitmap_group_sequence"),
            WORLD,
            function(bitmap_group_c, "bitmap_group_get_bitmap_from_sequence"),
            function(hud_draw_c, "get_sprite_clip_rect"),
            function(hud_draw_c, "hud_retrieve_bitmap_and_bounding_rect"),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="hud-sprite-reference-") as work:
        source = Path(work) / "hud_sprite_reference.c"
        executable = Path(work) / "hud_sprite_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's HUD bitmap lookups (tools/engine_hud_sprite_reference.py).\n"
        "# group <index> <bitmaps>\n"
        "# sequence <group> <index> <first bitmap> <bitmap count> <sprites>\n"
        "# sprite <group> <sequence> <index> <bitmap> <left> <right> <top> <bottom>\n"
        "# static <group> <sequence> <frame> <bitmap or -1> <clip or none>: a static element's or meter's\n"
        "# hud <group> <sequence> <frame> <bitmap or -1> <clip or none>: a number's or crosshair's\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_hud_sprite_reference.py")
        print("the hud sprite reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
