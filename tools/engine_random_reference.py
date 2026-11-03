"""The game's random helpers (real_math.h's `random_range`,
`real_random_range`, `random_direction3d`, their local seed's twins and the
rest), drawn over long runs of the game's seed and its local seed, to record
the numbers the engine's seeds must draw.

    python3 tools/engine_random_reference.py          # writes the reference
    python3 tools/engine_random_reference.py --check  # fails if it is stale

The helpers are real_math.h's inline wrappers, over source/math/random_math.c's
seeds (`get_global_random_seed_address`, `get_global_local_random_seed_address`,
`seed_random_direction3d` and the seeded numbers), with the direction table
built by source/math/geometry.c's `geosphere_new`, as
tools/engine_math_reference.py builds it, and compiled for this machine
against a small fake world. The trace is written to
tools/traces/random_reference.txt, which
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
import engine_math_reference as math_reference  # noqa: E402
from harness import enum_with, function, inline, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "random_reference.txt"

# random_math.c's seeds and seeded numbers
RANDOM_FUNCTIONS = [
    "lock_global_random_seed",
    "unlock_global_random_seed",
    "get_random_seed",
    "get_global_random_seed_address",
    "get_global_local_random_seed_address",
    "seed_random",
    "seed_random_range",
    "real_seed_random",
    "real_seed_random_range",
    "seed_random_direction3d",
    "seed_random_vector_in_cone3d",
    "direction3d_from_table",
]

# real_math.h's helpers over the game's seed and its local seed
WRAPPERS = [
    "set_random_seed",
    "random",
    "random_range",
    "random_boolean",
    "real_random",
    "real_random_range",
    "random_direction3d",
    "random_vector_in_cone3d",
    "local_random",
    "local_random_range",
    "real_local_random",
    "real_local_random_range",
    "local_random_direction3d",
    "local_random_vector_in_cone3d",
]

FAKE_WORLD = r"""
typedef unsigned short word;
/* the C library has a random() of its own */
#define random game_random
#define match_dassert(file, line, condition, message) do { if (!(condition)) { printf("assert %s %d\n", file, line); exit(1); } } while (0)

/* the game engine (a multiplayer game's rules) is not running */
static boolean game_engine_running(void)
{
	return FALSE;
}
"""

MAIN = r"""
#define STEPS 3000

static unsigned long cases = 0x7a11;

static unsigned long next_case(void)
{
	cases = cases*1103515245u + 12345u;
	return cases >> 8;
}

static void print_vector(real_vector3d const *v)
{
	printf(" %.9g %.9g %.9g", v->i, v->j, v->k);
}

/* one helper, chosen at random, over the seed `local` names; the seed after */
static void draw(boolean local, long step)
{
	short lower = (short)(next_case() % 2001) - 1000;
	short upper = lower + (short)(next_case() % 3000);
	real low = (real)lower * 0.01f;
	real high = (real)upper * 0.01f;
	real_vector3d axis = random_math_globals.random_direction_table[next_case() % random_math_globals.random_direction_table_size];
	real inner = (real)(next_case() % 100) * 0.01f;
	real outer = inner + (real)(next_case() % 100) * 0.01f;
	real_vector3d result;
	unsigned long before = local ? *get_global_local_random_seed_address() : get_random_seed();

	printf("%s %u", local ? "local" : "global", before);
	switch (step % 7)
	{
	case 0:
		printf(" random %u", local ? local_random() : random());
		break;
	case 1:
		printf(" range %d %d %d", lower, upper, local ? local_random_range(lower, upper) : random_range(lower, upper));
		break;
	case 2:
		if (local)
			printf(" real %.9g", real_local_random());
		else
			printf(" boolean %d", random_boolean());
		break;
	case 3:
		printf(" real %.9g", local ? real_local_random() : real_random());
		break;
	case 4:
		printf(" real_range %.9g %.9g %.9g", low, high, local ? real_local_random_range(low, high) : real_random_range(low, high));
		break;
	case 5:
		if (local)
			local_random_direction3d(&result);
		else
			random_direction3d(&result);
		printf(" direction");
		print_vector(&result);
		break;
	case 6:
		if (local)
			local_random_vector_in_cone3d(&axis, inner, outer, &result);
		else
			random_vector_in_cone3d(&axis, inner, outer, &result);
		printf(" cone");
		print_vector(&axis);
		printf(" %.9g %.9g", inner, outer);
		print_vector(&result);
		break;
	}
	printf(" %u\n", local ? *get_global_local_random_seed_address() : get_random_seed());
}

int main(void)
{
	struct geosphere *sphere = geosphere_new(16);
	long step;

	random_math_globals.random_direction_table = (real_vector3d *)sphere->vertices;
	random_math_globals.random_direction_table_size = sphere->vertex_count;

	set_random_seed(0xdeadbeef);
	*get_global_local_random_seed_address() = 0x10ca1;
	for (step = 0; step < STEPS; step++)
	{
		/* the two seeds are drawn in turn at random; neither moves the other */
		unsigned long global = get_random_seed();
		unsigned long local = *get_global_local_random_seed_address();
		boolean drawn_local = next_case() % 2;

		draw(drawn_local, step);
		if ((drawn_local ? get_random_seed() != global : *get_global_local_random_seed_address() != local))
		{
			printf("one seed moved the other\n");
			exit(1);
		}
	}

	/* a locked seed is still the game's while the game engine is not running */
	lock_global_random_seed();
	printf("locked %u", get_random_seed());
	printf(" %d", random_range(0, 100));
	unlock_global_random_seed();
	printf(" %u\n", get_random_seed());
	return 0;
}
"""


def seeded_source(fake_world=FAKE_WORLD):
    """The game's seed and its helpers over the direction table, in the math
    trace's world and `fake_world` (which answers game_engine_running), as C
    another trace can put its own code and cases after."""
    real_math_c = read("source/math/real_math.c")
    real_math_h = read("source/math/real_math.h")
    geometry_c = read("source/math/geometry.c")
    geometry_h = read("source/math/geometry.h")
    random_c = read("source/math/random_math.c")
    # the math trace's fake world, with random_math.c's own globals in place
    # of its direction table alone
    world = math_reference.FAKE_WORLD
    world = world[: world.index("/* random_math.c's globals")]
    globals_ = structure(random_c, "random_math_globals") + "\nstatic struct random_math_globals random_math_globals;"
    helpers = [inline(real_math_h, name) for name in math_reference.INLINE_FUNCTIONS]
    rotate = function(real_math_c, "rotate_vector_about_axis")
    prototypes = geometry_c[
        geometry_c.index("static void subdivide_triangle(") : geometry_c.index("/* ---------- globals")
    ]
    primitives = geometry_c[
        geometry_c.index("static real_point3d const geosphere_primitive_vertices") : geometry_c.index(
            "/* ---------- public code"
        )
    ]
    geosphere = [
        enum_with(geometry_c, "GEOSPHERE_PRIMITIVE_VERTEX_COUNT"),
        structure(geometry_h, "geosphere"),
        prototypes,
        primitives,
    ]
    geosphere += [function(geometry_c, name) for name in math_reference.GEOMETRY_FUNCTIONS]
    random_prototype = "static real_vector3d *direction3d_from_table(real_vector3d *result, short index);"
    randoms = [function(random_c, name) for name in RANDOM_FUNCTIONS]
    wrappers = [inline(real_math_h, name) for name in WRAPPERS]
    return "\n\n".join(
        [world, fake_world, globals_, *helpers, rotate, *geosphere, random_prototype, *randoms, *wrappers]
    )


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    return "\n\n".join([seeded_source(), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="random-reference-") as work:
        source = Path(work) / "random_reference.c"
        executable = Path(work) / "random_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's random helpers (tools/engine_random_reference.py).\n"
        "# <global|local> <seed before> <helper> <its bounds> <what it drew> <seed after>\n"
        "# random <n>; range <lower> <upper> <n>; boolean <0|1>; real <x>; real_range <lower> <upper> <x>\n"
        "# direction <v>; cone <axis> <inner> <outer> <v>; locked <seed> <range 0 100> <seed after>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_random_reference.py")
        print("the random reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
