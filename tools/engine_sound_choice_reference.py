"""The game's own choice of what a sound plays (its pitch range, its
permutation and its scaled values), taken from the sources and run over
scripted plays, to record what the native engine's must give.

    python3 tools/engine_sound_choice_reference.py          # writes the reference
    python3 tools/engine_sound_choice_reference.py --check  # fails if it is stale

The functions are source/sound/sound_definitions.c's
(`sound_definition_find_pitch_range_by_pitch`, `try_to_reset_permutations`,
`sound_definition_next_permutation`) and source/sound/sound_manager.c's
(`sound_scale_value`, `sound_scale_random_value`), found by name as
tools/harness finds them, with source/math/random_math.c's seeded numbers,
and compiled for this machine against a small fake world. The table is
written to tools/traces/sound_choice_reference.txt, which game.wasm's build must
match (tools/engine_wasm_traces.py).
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

OUTPUT = ROOT / "tools" / "traces" / "sound_choice_reference.txt"

DEFINITION_FUNCTIONS = [
    "sound_definition_find_pitch_range_by_pitch",
    "try_to_reset_permutations",
    "sound_definition_next_permutation",
]

MANAGER_FUNCTIONS = ["sound_scale_value", "sound_scale_random_value"]

RANDOM_FUNCTIONS = ["seed_random", "seed_random_range", "real_seed_random", "real_seed_random_range"]

FAKE_WORLD = r"""
#include <float.h>
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef float real;
typedef unsigned char byte;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define SET_FLAG(f, b, v) ((v) ? ((f)|=(unsigned)FLAG(b)) : ((f)&=(unsigned)~FLAG(b)))
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define VALID_INDEX(index, count) (index>=0 && index<count)
#define match_assert(file, line, condition) ((void)0)
#define error(...) ((void)0)
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])
#define RANDOM_A 1664525u
#define RANDOM_C 1013904223u

struct tag_block_definition;
struct tag_data_definition;

struct tag_block { long count; void *address; struct tag_block_definition *definition; };
struct tag_reference { unsigned long group_tag; char *name; long name_length; long index; };
struct tag_data { long size; unsigned long pad; long file_offset; void *address; struct tag_data_definition *definition; };

static unsigned long local_random_seed;

static unsigned long *get_global_local_random_seed_address(void)
{
	return &local_random_seed;
}
"""

MAIN = r"""
enum { MAXIMUM_PERMUTATIONS = 32, MAXIMUM_RANGES = 4 };

static struct sound_permutation permutations[MAXIMUM_RANGES][MAXIMUM_PERMUTATIONS];
static struct sound_pitch_range ranges[MAXIMUM_RANGES];
static struct sound_definition definition;

/* a range of `count` permutations, the first `actual` of which play, each
skipped by its fraction (the fractions repeat) and linked to its next */
static void make_range(
	short index,
	real lower,
	real upper,
	short count,
	short actual,
	real const *skips,
	short skip_count,
	short const *nexts)
{
	short permutation;
	struct sound_pitch_range *range = &ranges[index];

	memset(range, 0, sizeof(*range));
	range->bend_bounds.lower = lower;
	range->bend_bounds.upper = upper;
	range->actual_permutation_count = actual;
	range->previous_permutation_index = NONE;
	range->forced_permutation_index = NONE;
	range->permutations.count = count;
	range->permutations.address = permutations[index];
	for (permutation = 0; permutation<count; permutation++)
	{
		memset(&permutations[index][permutation], 0, sizeof(permutations[index][permutation]));
		permutations[index][permutation].skip_fraction = skip_count ? skips[permutation%skip_count] : 0.0f;
		permutations[index][permutation].next_permutation_index = nexts ? nexts[permutation] : NONE;
	}
}

static void plays(char const *name, short range_index, int count)
{
	int play;

	for (play = 0; play<count; play++)
	{
		short chosen = sound_definition_next_permutation(&definition, range_index, NONE);

		printf("%s %d %u %u\n", name, chosen, ranges[range_index].played_permutation_mask, local_random_seed);
	}
}

int main(void)
{
	static real const skips[] = { 0.0f, 0.3f, 0.9f, 0.0f, 1.0f, 0.5f, 0.0f };
	static real const sometimes[] = { 0.25f, 0.75f };
	static real const always[] = { 1.0f };
	static short const nexts[] = { 1, 2, NONE, 9 };
	static real const scales[][5] = {
		{ 1.0f, 1.0f, 0.5f, 1.0f, 0.5f }, { 0.8f, 1.2f, 1.0f, 2.0f, 0.25f },
		{ 0.0f, 3.0f, 0.0f, 1.0f, 1.0f }, { -1.0f, 1.0f, 2.0f, 0.5f, 0.75f },
	};
	short old;
	int step;

	definition.pitch_ranges.address = ranges;

	/* the pitch ranges, one without permutations, over pitches below,
	between, in and above them, from each old range */
	make_range(0, 0.5f, 0.9f, 5, 5, NULL, 0, NULL);
	make_range(1, 0.9f, 1.1f, 0, 0, NULL, 0, NULL);
	make_range(2, 1.2f, 2.0f, 3, 3, NULL, 0, NULL);
	make_range(3, 2.5f, 4.0f, 2, 2, NULL, 0, NULL);
	definition.pitch_ranges.count = 4;
	for (step = 1; step<=100; step++)
	{
		real pitch = step*0.05f;

		for (old = NONE; old<=4; old++)
			printf("pitch %.9g %d %d\n", pitch, old, sound_definition_find_pitch_range_by_pitch(&definition, pitch, old));
	}

	/* the permutations, without repeating and skipped by their fractions:
	seven of which six play; thirty-two; a lone one; and every one skipped */
	make_range(0, 0.0f, 2.0f, 7, 6, skips, 7, NULL);
	make_range(1, 0.0f, 2.0f, 32, 32, sometimes, 2, NULL);
	make_range(2, 0.0f, 2.0f, 1, 1, NULL, 0, NULL);
	make_range(3, 0.0f, 2.0f, 4, 4, always, 1, NULL);
	local_random_seed = 0x1234;
	plays("six", 0, 400);
	plays("all", 1, 200);
	plays("lone", 2, 20);
	plays("skipped", 3, 40);

	/* a forced permutation, once, then the random ones again */
	ranges[0].forced_permutation_index = 4;
	plays("forced", 0, 8);

	/* linked permutations, from each current one */
	definition.flags = FLAG(1);
	make_range(0, 0.0f, 2.0f, 4, 4, NULL, 0, nexts);
	for (old = NONE; old<=4; old++)
	{
		short chosen = sound_definition_next_permutation(&definition, 0, old);

		printf("linked %d %d %u\n", old, chosen, local_random_seed);
	}

	/* the values a scale moves between */
	local_random_seed = 0xdeadbeef;
	for (step = 0; step<40; step++)
	{
		real const *scale = scales[step%4];
		real value = sound_scale_random_value(scale[0], scale[1], scale[2], scale[3], scale[4]);

		printf("scale %.9g %.9g %.9g %.9g %.9g %.9g\n", scale[0], scale[1], scale[2], scale[3], scale[4], value);
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    definitions_c = read("source/sound/sound_definitions.c")
    definitions_h = read("source/sound/sound_definitions.h")
    manager_c = read("source/sound/sound_manager.c")
    random_c = read("source/math/random_math.c")
    structures = [
        structure(definitions_h, name)
        for name in ("real_bounds", "sound_permutation", "sound_pitch_range", "sound_definition")
    ]
    helpers = [function(random_c, name) for name in RANDOM_FUNCTIONS]
    definitions = [function(definitions_c, name) for name in DEFINITION_FUNCTIONS]
    definitions += [function(manager_c, name) for name in MANAGER_FUNCTIONS]
    return "\n\n".join([FAKE_WORLD, *structures, *helpers, *definitions, MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="sound-choice-reference-") as work:
        source = Path(work) / "sound_choice_reference.c"
        executable = Path(work) / "sound_choice_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's choice of what a sound plays (tools/engine_sound_choice_reference.py).\n"
        "# pitch <pitch> <old range> <range>\n"
        "# <six|all|lone|skipped|forced> <permutation> <played mask after> <seed after>, from the seed 0x1234\n"
        "# linked <current> <next> <seed after>\n"
        "# scale <lower> <upper> <zero> <one> <scale> <value>, from the seed 0xdeadbeef\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_sound_choice_reference.py")
        print("the sound choice reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
