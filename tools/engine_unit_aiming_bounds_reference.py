"""The game's questions and settings of a unit's aiming and looking vectors
and its clip of a vector to its aiming and looking screens, taken from the
sources and run over random vectors, frames and bounds, to record what they
answer, as the native engine's must.

    python3 tools/engine_unit_aiming_bounds_reference.py          # writes the reference
    python3 tools/engine_unit_aiming_bounds_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_vectors_are_valid`,
`unit_clip_to_aiming_bounds`, `unit_get_facing_vector`,
`unit_get_looking_vector`, `unit_get_aiming_vector`, `unit_align_facing`,
`unit_aim_without_turning` and `unit_is_playing_custom_animation`, with
source/math/real_math.c's `euler_angles2d_from_vector3d` and
`vector3d_from_euler_angles2d`, source/math/matrix_math.c's
`matrix4x3_transform_normal` and `matrix4x3_inverse_transform_normal` and
real_math.h's inline helpers they call, found by name as tools/harness
finds them, compiled for this machine against a small fake unit with the
game's asserts left out, as its release build leaves them.

The vectors are unit normals at random, some a little long or short, some
of no length, infinite or not a number; the frames are random turns of the
world's axes; the screens' bounds are random spans of yaw and pitch, some
of them empty.

Floats are printed as their bits, so that the engine's must be the same
float, not only print the same.

The trace is written to tools/traces/unit_aiming_bounds_reference.txt, which
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
from harness import enum_with, function, inline, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "unit_aiming_bounds_reference.txt"

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
typedef float real;
#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define FLAG(bit) (1u << (bit))
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define match_assert(file, line, condition) ((void)0)
#define match_vassert(file, line, condition, text) ((void)0)
#define match_assert_valid_real(file, line, value) ((void)0)
#define match_assert_valid_real_vector3d(file, line, vector) ((void)0)
#define match_assert_valid_real_normal3d(file, line, vector) ((void)0)
#define match_assert_valid_real_vector3d_axes2(file, line, forward, up) ((void)0)

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct { real i, j; } real_vector2d;
typedef struct { real yaw, pitch; } real_euler_angles2d;
typedef union { real n[4]; struct { real x0; real x1; real y0; real y1; }; } real_rectangle2d;
typedef struct
{
	real scale;
	real_vector3d forward;
	real_vector3d left;
	real_vector3d up;
	real_point3d position;
} real_matrix4x3;

static real_point3d const origin = { 0.f, 0.f, 0.f };
static real_point3d const *global_origin3d = &origin;
static real_vector3d const up = { 0.f, 0.f, 1.f };
static real_vector3d const *global_up3d = &up;
"""

# the fake unit: only the fields these functions read and write
FAKE_UNIT = r"""
struct unit_datum
{
	struct
	{
		real_vector3d forward;
		real_vector3d up;
		long parent_object_index;
	} object;
	struct
	{
		unsigned long flags;
		real_vector3d desired_facing_vector;
		real_vector3d desired_aiming_vector;
		real_vector3d desired_looking_vector;
		real_vector3d aiming_vector;
		real_vector3d looking_vector;
		struct
		{
			short state;
			boolean aiming_with_euler_screen;
			boolean looking_with_euler_screen;
			real_rectangle2d aiming_screen_bounds;
			real_rectangle2d looking_screen_bounds;
		} animation;
	} unit;
};

static struct unit_datum the_unit;

static struct unit_datum *unit_get(long unit_index)
{
	return &the_unit;
}

static void object_get_orientation(long object_index, real_vector3d *forward, real_vector3d *up)
{
	if (forward)
		*forward = the_unit.object.forward;
	if (up)
		*up = the_unit.object.up;
}
"""

DRIVER = r"""
static unsigned long seed = 0xa1b0d5;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static real random_real(real low, real high)
{
	return low + (high - low) * (real)next_random(1000001) / 1000000.f;
}

static unsigned bits(real value)
{
	unsigned result;
	memcpy(&result, &value, sizeof result);
	return result;
}

static void print_vector(real_vector3d const *v)
{
	printf(" %u %u %u", bits(v->i), bits(v->j), bits(v->k));
}

/* a unit normal at random, built as the engine reads it back */
static real_vector3d random_normal(void)
{
	real_vector3d v;
	real yaw = random_real(-3.1415927f, 3.1415927f), pitch = random_real(-1.5707964f, 1.5707964f);
	v.i = (real)(cos(yaw) * cos(pitch));
	v.j = (real)(sin(yaw) * cos(pitch));
	v.k = (real)sin(pitch);
	return v;
}

/* a vector that is mostly a unit normal, and sometimes not quite or not at all */
static real_vector3d random_vector(void)
{
	real_vector3d v = random_normal();
	switch (next_random(12))
	{
	case 0: { real scale = random_real(0.998f, 1.002f); v.i *= scale; v.j *= scale; v.k *= scale; break; }
	case 1: v.i = v.j = v.k = 0.f; break;
	case 2: v.j = INFINITY; break;
	case 3: v.k = NAN; break;
	case 4: { real scale = random_real(0.9f, 1.1f); v.i *= scale; break; }
	}
	return v;
}

/* a frame: a forward and an up at right angles, or now and then not */
static void random_frame(real_vector3d *forward, real_vector3d *up)
{
	real_vector3d side;
	real length;
	*forward = random_normal();
	side = random_normal();
	/* up is side less its part along forward, made a normal */
	length = side.i * forward->i + side.j * forward->j + side.k * forward->k;
	up->i = side.i - forward->i * length;
	up->j = side.j - forward->j * length;
	up->k = side.k - forward->k * length;
	length = sqrtf(up->i * up->i + up->j * up->j + up->k * up->k);
	if (length < 0.01f)
	{
		*up = *global_up3d;
		forward->i = 1.f; forward->j = 0.f; forward->k = 0.f;
	}
	else
	{
		up->i /= length; up->j /= length; up->k /= length;
	}
	switch (next_random(16))
	{
	case 0: *up = *forward; break;
	case 1: up->k += 0.01f; break;
	case 2: forward->i = NAN; break;
	}
}

/* a screen's bounds, x0 to x1 in yaw and y0 to y1 in pitch */
static real_rectangle2d random_bounds(void)
{
	real_rectangle2d bounds;
	bounds.x0 = random_real(-3.2f, 0.5f);
	bounds.x1 = random_real(-0.5f, 3.2f);
	bounds.y0 = random_real(-1.6f, 0.3f);
	bounds.y1 = random_real(-0.3f, 1.6f);
	if (next_random(10) == 0)
		bounds.x0 = bounds.x1 = random_real(-1.f, 1.f);
	if (next_random(10) == 0)
	{
		real swap = bounds.y0;
		bounds.y0 = bounds.y1;
		bounds.y1 = swap;
	}
	return bounds;
}

static void print_bounds(real_rectangle2d const *bounds)
{
	printf(" %u %u %u %u", bits(bounds->x0), bits(bounds->x1), bits(bounds->y0), bits(bounds->y1));
}

int main(void)
{
	long run;
	for (run = 0; run < 6000; run++)
	{
		memset(&the_unit, 0, sizeof the_unit);
		the_unit.unit.desired_facing_vector = random_vector();
		the_unit.unit.desired_aiming_vector = random_vector();
		the_unit.unit.desired_looking_vector = random_vector();
		random_frame(&the_unit.object.forward, &the_unit.object.up);
		the_unit.unit.aiming_vector = random_vector();
		the_unit.unit.looking_vector = random_vector();
		printf("valid");
		print_vector(&the_unit.unit.desired_facing_vector);
		print_vector(&the_unit.unit.desired_aiming_vector);
		print_vector(&the_unit.unit.desired_looking_vector);
		print_vector(&the_unit.object.forward);
		print_vector(&the_unit.object.up);
		print_vector(&the_unit.unit.aiming_vector);
		print_vector(&the_unit.unit.looking_vector);
		printf(" -> %d\n", unit_vectors_are_valid(0) ? 1 : 0);
	}
	for (run = 0; run < 40000; run++)
	{
		boolean use_aiming_screen = (boolean)next_random(2);
		boolean on = next_random(5) != 0;
		real_vector3d vector = random_normal(), clipped;
		boolean result;
		memset(&the_unit, 0, sizeof the_unit);
		random_frame(&the_unit.object.forward, &the_unit.object.up);
		if (next_random(12) == 0)
			the_unit.object.up.i += 0.002f;
		the_unit.unit.animation.aiming_with_euler_screen = use_aiming_screen && on;
		the_unit.unit.animation.looking_with_euler_screen = !use_aiming_screen && on;
		the_unit.unit.animation.aiming_screen_bounds = random_bounds();
		the_unit.unit.animation.looking_screen_bounds = random_bounds();
		printf("clip %d %d", use_aiming_screen, on);
		print_vector(&the_unit.object.forward);
		print_vector(&the_unit.object.up);
		print_bounds(use_aiming_screen ? &the_unit.unit.animation.aiming_screen_bounds : &the_unit.unit.animation.looking_screen_bounds);
		print_vector(&vector);
		clipped = vector;
		result = unit_clip_to_aiming_bounds(0, &clipped, use_aiming_screen);
		printf(" -> %d", result ? 1 : 0);
		print_vector(&clipped);
		printf("\n");
	}
	for (run = 0; run < 2000; run++)
	{
		real_vector3d facing, looking, aiming;
		memset(&the_unit, 0, sizeof the_unit);
		random_frame(&the_unit.object.forward, &the_unit.object.up);
		the_unit.unit.looking_vector = random_vector();
		the_unit.unit.aiming_vector = random_vector();
		printf("get");
		print_vector(&the_unit.object.forward);
		print_vector(&the_unit.unit.looking_vector);
		print_vector(&the_unit.unit.aiming_vector);
		unit_get_facing_vector(0, &facing);
		unit_get_looking_vector(0, &looking);
		unit_get_aiming_vector(0, &aiming);
		printf(" ->");
		print_vector(&facing);
		print_vector(&looking);
		print_vector(&aiming);
		printf("\n");
	}
	for (run = 0; run < 2000; run++)
	{
		real_vector2d alignment;
		real_vector3d direction = random_normal();
		memset(&the_unit, 0, sizeof the_unit);
		random_frame(&the_unit.object.forward, &the_unit.object.up);
		the_unit.object.parent_object_index = next_random(4) == 0 ? (long)next_random(100) : NONE;
		alignment.i = direction.i;
		alignment.j = direction.j;
		printf("align %d %u %u", the_unit.object.parent_object_index != NONE, bits(alignment.i), bits(alignment.j));
		print_vector(&the_unit.object.forward);
		print_vector(&the_unit.object.up);
		unit_align_facing(0, &alignment);
		printf(" ->");
		print_vector(&the_unit.object.forward);
		print_vector(&the_unit.object.up);
		printf("\n");
	}
	for (run = 0; run < 2000; run++)
	{
		unsigned long flags = (unsigned long)next_random(0x40000000) | ((unsigned long)next_random(2) << 30);
		boolean on = (boolean)next_random(2);
		long unit_index = next_random(8) == 0 ? NONE : 0;
		memset(&the_unit, 0, sizeof the_unit);
		the_unit.unit.flags = flags;
		unit_aim_without_turning(unit_index, on);
		printf("aim %d %lu %d -> %lu\n", unit_index != NONE, flags, on, the_unit.unit.flags);
	}
	for (run = 0; run < NUMBER_OF_UNIT_STATES + 2; run++)
	{
		long unit_index = run == NUMBER_OF_UNIT_STATES + 1 ? NONE : 0;
		memset(&the_unit, 0, sizeof the_unit);
		the_unit.unit.animation.state = (short)(run % NUMBER_OF_UNIT_STATES);
		printf("custom %d %d -> %d\n", unit_index != NONE, the_unit.unit.animation.state,
			unit_is_playing_custom_animation(unit_index) ? 1 : 0);
	}
	return 0;
}
"""

INLINE_FUNCTIONS = [
    "arctangent",
    "square_root",
    "sine",
    "cosine",
    "magnitude_squared3d",
    "dot_product3d",
    "cross_product3d",
    "set_real_vector3d",
    "valid_real",
    "valid_realcmp",
    "valid_real_normal3d",
    "valid_real_vector3d_axes2",
]

UNIT_FUNCTIONS = [
    "unit_vectors_are_valid",
    "unit_clip_to_aiming_bounds",
    "unit_get_facing_vector",
    "unit_get_looking_vector",
    "unit_get_aiming_vector",
    "unit_align_facing",
    "unit_aim_without_turning",
    "unit_is_playing_custom_animation",
]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    real_math_h = read("source/math/real_math.h")
    real_math_c = read("source/math/real_math.c")
    matrix_math_c = read("source/math/matrix_math.c")
    units_h = read("source/units/units.h")
    units_c = read("source/units/units.c")
    return "\n".join(
        [
            FAKE_WORLD,
            enum_with(units_h, "_unit_aim_without_turning_bit") + ";",
            enum_with(units_h, "_unit_state_user_animation") + ";",
            FAKE_UNIT,
            *[inline(real_math_h, name) for name in INLINE_FUNCTIONS],
            function(real_math_c, "euler_angles2d_from_vector3d"),
            function(real_math_c, "vector3d_from_euler_angles2d"),
            function(matrix_math_c, "matrix4x3_transform_normal"),
            function(matrix_math_c, "matrix4x3_inverse_transform_normal"),
            *[function(units_c, name) for name in UNIT_FUNCTIONS],
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-aiming-bounds-reference-") as work:
        source = Path(work) / "unit_aiming_bounds_reference.c"
        executable = Path(work) / "unit_aiming_bounds_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit aiming vectors and screens (tools/engine_unit_aiming_bounds_reference.py), floats as bits.\n"
        "# valid <desired facing> <desired aiming> <desired looking> <forward> <up> <aiming> <looking> -> <valid>\n"
        "# clip <aiming screen> <with euler screen> <forward> <up> <x0 x1 y0 y1> <vector> -> <clipped> <vector>\n"
        "# get <forward> <looking> <aiming> -> <facing> <looking> <aiming>\n"
        "# align <attached> <alignment i j> <forward> <up> -> <forward> <up>\n"
        "# aim <unit> <flags> <on> -> <flags>\n"
        "# custom <unit> <state> -> <playing a custom animation>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_aiming_bounds_reference.py")
        print("the unit aiming bounds reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
