"""The game's wind: the noise table it draws at a new map, each palette
entry's wind wandering tick by tick, and the air at points in it under
each pair of flags, recorded for the engine's particle wind to match.

    python3 tools/engine_wind_reference.py          # writes the reference
    python3 tools/engine_wind_reference.py --check  # fails if it is stale

The functions are source/scenario/wind.c's `wind_variance_initialize`,
`wind_variance_get`, `wind_update`,
`scenario_get_current_from_weather_palette`, `wind_initialize_for_new_map` and
`wind_dispose_from_old_map` (a second map's winds started after the first's), with real_math.h's uniform
cubic spline and real_math.c's euler angle helpers, compiled for this
machine against a small fake world: a BSP whose weather palette has three
entries (two winds and one without), and the random draws the game takes
from its seeds replaced by a recorded sequence of directions and coins,
which the trace prints so that the engine can be handed the same ones. The
table is written to tools/traces/wind_reference.txt, which
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
from harness import function, inline, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "wind_reference.txt"

INLINE_FUNCTIONS = [
    "sine",
    "cosine",
    "arctangent",
    "square_root",
    "uniform_cubic_spline",
    "uniform_cubic_spline_vector3d",
]

REAL_MATH_FUNCTIONS = ["euler_angles2d_from_vector3d", "vector3d_from_euler_angles2d"]

WIND_FUNCTIONS = [
    "wind_variance_get",
    "wind_update",
    "scenario_get_current_from_weather_palette",
    "wind_variance_initialize",
    "wind_initialize_for_new_map",
    "wind_dispose_from_old_map",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef unsigned short word;
typedef byte boolean;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define NUMBEROF(a) (sizeof(a)/sizeof((a)[0]))
#define MIN(a,b) ((a)<(b)?(a):(b))
#define PIN(x,lo,hi) ((x)<(lo)?(lo):((x)>(hi)?(hi):(x)))
#define FLAG(b) (1<<(b))
#define TEST_FLAG(f,b) (((f)&FLAG(b))!=0)
#define VALID_INDEX(i,n) ((i)>=0 && (i)<(n))
#define match_assert(file, line, condition) ((void)0)
#define match_vassert(file, line, condition, message) ((void)0)

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d_xyz;
typedef union { struct { real x, y, z; }; real n[3]; } real_point3d;
typedef struct { real yaw, pitch; } real_euler_angles2d;

enum
{
	_scenario_current_simple_bit = 0,
	_scenario_current_damped_bit,
	_scenario_current_force_water_bit,
	_scenario_current_force_no_water_bit,
};

static real_vector3d const zero_vector3d = { 0.f, 0.f, 0.f };
static real_vector3d const *const global_zero_vector3d = &zero_vector3d;

struct tag_reference { long index; };
struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

struct wind_definition
{
	real velocity_lower_bound;
	real velocity_upper_bound;
	real_euler_angles2d variation_area;
	real local_variation_weight;
	real local_variation_rate;
	real damping;
};

/* the palette entry's fields the wind reads */
struct structure_weather_palette_entry
{
	struct tag_reference wind;
	real_vector3d wind_direction;
	real wind_magnitude;
};

struct structure_bsp { struct tag_block weather_palette; };

struct wind_state
{
	boolean valid;
	real velocity_variance;
	real_euler_angles2d angular_variance;
	real velocity;
	real_vector3d velocity3d;
};

struct
{
	boolean initialized;
	real_vector3d variance[3][64];
	short count;
	struct wind_state wind_states[32];
	long time;
} wind_globals;

/* the map: two winds, and a palette of three entries, the middle without */
static struct wind_definition winds[2] =
{
	{ 1.0f, 4.0f, { 0.6f, 0.2f }, 0.4f, 1.5f, 0.25f },
	{ 0.5f, 0.75f, { 0.1f, 0.9f }, 0.0f, 0.5f, 0.6f },
};
static struct structure_weather_palette_entry palette[3] =
{
	{ { 0 }, { 1.0f, 2.0f, 0.5f }, 3.0f },
	{ { NONE }, { 0.0f, 1.0f, 0.0f }, 9.0f },
	{ { 1 }, { -0.3f, 0.1f, -0.8f }, 1.25f },
};
static struct structure_bsp bsp = { { 3, palette } };

static struct structure_bsp *global_structure_bsp_get(void) { return &bsp; }
#define wind_definition_get(index) (&winds[index])

/* the recorded draws: a run of numbers the trace prints as it hands them out */
static unsigned int draws = 0x5eed;

static unsigned int next(void)
{
	draws = draws*1664525u+1013904223u;
	return draws;
}

static long global_seed, local_seed;
static long *get_global_random_seed_address(void) { return &global_seed; }
static long *get_global_local_random_seed_address(void) { return &local_seed; }

/* a direction: a recorded point on the unit sphere */
static void seed_random_direction3d(long *seed, real_vector3d *v)
{
	real i = (real)((int)(next()>>16)%2001-1000)*0.001f;
	real j = (real)((int)(next()>>16)%2001-1000)*0.001f;
	real k = (real)((int)(next()>>16)%2001-1000)*0.001f;
	real length = (real)sqrt(i*i+j*j+k*k);

	if (length<0.01f) { i = 1.0f; j = 0.0f; k = 0.0f; length = 1.0f; }
	v->i = i/length;
	v->j = j/length;
	v->k = k/length;
	printf("direction %.9g %.9g %.9g\n", v->i, v->j, v->k);
}

/* a coin: 0 or 1 */
static short seed_random_range(long *seed, short lower, short upper)
{
	short coin = (short)((next()>>20)&1);

	printf("coin %d\n", coin);
	return coin;
}
"""

MAIN = r"""
int main(void)
{
	real_point3d const points[] =
	{
		{ { 0.0f, 0.0f, 0.0f } },
		{ { 0.3f, 0.6f, 0.2f } },
		{ { -12.5f, 40.25f, -3.0f } },
		{ { 1000.0f, -2500.5f, 77.7f } },
		{ { 1048576.0f, -3000000.0f, 0.0625f } },
	};
	int tick, axis, sample;

	wind_initialize_for_new_map();
	for (axis = 0; axis<3; axis++)
		for (sample = 0; sample<64; sample++)
			printf("variance %d %d %.9g %.9g %.9g\n", axis, sample, wind_globals.variance[axis][sample].i,
				wind_globals.variance[axis][sample].j, wind_globals.variance[axis][sample].k);
	for (tick = 1; tick<=200; tick++)
	{
		short entry;

		wind_update();
		for (entry = 0; entry<wind_globals.count; entry++)
		{
			struct wind_state *state = &wind_globals.wind_states[entry];

			printf("state %d %d %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", tick, entry, state->valid,
				state->velocity_variance, state->angular_variance.pitch, state->angular_variance.yaw,
				state->velocity, state->velocity3d.i, state->velocity3d.j, state->velocity3d.k);
		}
		if (tick%10)
			continue;
		for (entry = -1; entry<=3; entry++)
		{
			long point, flags;

			for (point = 0; point<(long)NUMBEROF(points); point++)
				for (flags = 0; flags<4; flags++)
				{
					real_vector3d current;

					scenario_get_current_from_weather_palette(&points[point], &current, flags, entry);
					printf("current %d %d %d %d %.9g %.9g %.9g\n", tick, entry, point, flags,
						current.i, current.j, current.k);
				}
		}
	}
	/* the next map: the old one's winds are let go, and the new one's start still, with noise of their own */
	wind_dispose_from_old_map();
	printf("disposed %d\n", wind_globals.initialized);
	wind_initialize_for_new_map();
	printf("renewed %d %d %d\n", wind_globals.initialized, wind_globals.count, wind_globals.time);
	for (axis = 0; axis<3; axis++)
		for (sample = 0; sample<64; sample++)
			printf("renewed_variance %d %d %.9g %.9g %.9g\n", axis, sample, wind_globals.variance[axis][sample].i,
				wind_globals.variance[axis][sample].j, wind_globals.variance[axis][sample].k);
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    real_math_c = read("source/math/real_math.c")
    real_math_h = read("source/math/real_math.h")
    wind_c = read("source/scenario/wind.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            *(inline(real_math_h, name) for name in INLINE_FUNCTIONS),
            *(function(real_math_c, name) for name in REAL_MATH_FUNCTIONS),
            *(function(wind_c, name) for name in WIND_FUNCTIONS),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="wind-reference-") as work:
        source = Path(work) / "wind_reference.c"
        executable = Path(work) / "wind_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's wind (tools/engine_wind_reference.py).\n"
        "# direction <i> <j> <k>: a recorded random direction, in the order drawn\n"
        "# coin <0 or 1>: a recorded random coin, in the order drawn\n"
        "# variance <axis> <sample> <vector>\n"
        "# state <tick> <entry> <valid> <velocity variance> <pitch variance> <yaw variance> <velocity> <velocity3d>\n"
        "# current <tick> <entry> <point> <flags> <current>\n"
        "# disposed <initialized>: after the map's winds are let go\n"
        "# renewed <initialized> <entries> <time>: the next map's winds\n"
        "# renewed_variance <axis> <sample> <vector>: the next map's noise\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_wind_reference.py")
        print("the wind reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
