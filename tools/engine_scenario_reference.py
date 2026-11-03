"""The game's trigger volumes: whether points lie inside boxes along the
world's axes and boxes turned by their own, recorded for the engine's
trigger volumes to match.

    python3 tools/engine_scenario_reference.py          # writes the reference
    python3 tools/engine_scenario_reference.py --check  # fails if it is stale

The function is source/scenario/scenario.c's
`scenario_trigger_volume_test_point`, with matrix_math.c's
`matrix4x3_rotation_from_vectors`, `matrix4x3_from_point_and_vectors` and
`matrix4x3_inverse_transform_point` and real_math.h's cross product,
compiled for this machine against a small fake world: a scenario of
trigger volumes, half of them oriented, tested at points near, inside and
on their faces, and at an index the scenario does not have. The table is
written to tools/traces/scenario_reference.txt, which
engine/crates/content/tests/scenario_reference.rs reads.
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

OUTPUT = ROOT / "tools" / "traces" / "scenario_reference.txt"

INLINE_FUNCTIONS = ["set_real_point3d", "cross_product3d"]

MATRIX_FUNCTIONS = [
    "matrix4x3_rotation_from_vectors",
    "matrix4x3_from_point_and_vectors",
    "matrix4x3_inverse_transform_point",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef unsigned short word;
typedef byte boolean;

#define TRUE 1
#define FALSE 0
#define TAG_STRING_LENGTH 31
#define _error_silent 0
#define match_vassert(file, line, condition, message) ((void)0)
#define error(...) ((void)0)

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct { real x0, x1, y0, y1, z0, z1; } real_rectangle3d;
typedef struct
{
	real scale;
	real_vector3d forward;
	real_vector3d left;
	real_vector3d up;
	real_point3d position;
} real_matrix4x3;

enum
{
	_scenario_trigger_volume_type_axis_aligned = 0,
	_scenario_trigger_volume_type_oriented,
};

struct scenario_trigger_volume
{
	short type;
	word pad;
	char name[TAG_STRING_LENGTH+1];
	byte unused[0xC];
	real_vector3d forward;
	real_vector3d up;
	union
	{
		struct
		{
			real_point3d position;
			real_vector3d extents;
		};
		real_rectangle3d bounds;
	};
};

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

#define VOLUMES 40

static struct scenario_trigger_volume volumes[VOLUMES];
struct scenario { struct tag_block trigger_volumes; };
static struct scenario scenario = { { VOLUMES, volumes } };

static struct scenario *global_scenario_get(void) { return &scenario; }
"""

MAIN = r"""
static unsigned int seed = 0x5eed;

static unsigned int next(void)
{
	seed = seed*1664525u+1013904223u;
	return seed;
}

/* a real from -scale to scale, in steps of a thousandth of it */
static real between(real scale)
{
	return (real)((int)(next()>>16)%2001-1000)*0.001f*scale;
}

/* a unit vector, and another at right angles to it */
static void axes(real_vector3d *forward, real_vector3d *up)
{
	real_vector3d a = { between(1.0f), between(1.0f), between(1.0f)+1.5f };
	real length = (real)sqrt(a.i*a.i+a.j*a.j+a.k*a.k);
	real_vector3d b = { between(1.0f), between(1.0f), between(1.0f) };
	real along;

	forward->i = a.i/length; forward->j = a.j/length; forward->k = a.k/length;
	along = b.i*forward->i+b.j*forward->j+b.k*forward->k;
	b.i -= along*forward->i; b.j -= along*forward->j; b.k -= along*forward->k;
	length = (real)sqrt(b.i*b.i+b.j*b.j+b.k*b.k);
	if (length<0.01f) { b.i = -forward->j; b.j = forward->i; b.k = 0.0f; length = (real)sqrt(b.i*b.i+b.j*b.j); }
	up->i = b.i/length; up->j = b.j/length; up->k = b.k/length;
}

int main(void)
{
	short volume;

	for (volume = 0; volume<VOLUMES; volume++)
	{
		struct scenario_trigger_volume *v = &volumes[volume];
		real_point3d corner = { between(50.0f), between(50.0f), between(10.0f) };
		real_vector3d size = { 0.5f+between(4.0f)+4.0f, 0.5f+between(4.0f)+4.0f, 0.5f+between(2.0f)+2.0f };

		v->type = volume%2 ? _scenario_trigger_volume_type_oriented : _scenario_trigger_volume_type_axis_aligned;
		axes(&v->forward, &v->up);
		if (v->type==_scenario_trigger_volume_type_oriented)
		{
			v->position = corner;
			v->extents = size;
		}
		else
		{
			v->bounds.x0 = corner.x; v->bounds.x1 = corner.x+size.i;
			v->bounds.y0 = corner.y; v->bounds.y1 = corner.y+size.j;
			v->bounds.z0 = corner.z; v->bounds.z1 = corner.z+size.k;
		}
		printf("volume %d %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", volume, v->type,
			v->position.x, v->position.y, v->position.z, v->forward.i, v->forward.j, v->forward.k,
			v->up.i, v->up.j, v->up.k, v->extents.i, v->extents.j, v->extents.k);
	}
	for (volume = -2; volume<VOLUMES+2; volume++)
	{
		int step;
		struct scenario_trigger_volume *v = &volumes[volume<0 || volume>=VOLUMES ? 0 : volume];

		for (step = 0; step<60; step++)
		{
			real_point3d point;

			/* about the volume, a quarter of its size past each face, and
			first on its corner exactly */
			real a = between(0.75f)+0.5f, b = between(0.75f)+0.5f, c = between(0.75f)+0.5f;

			if (step==0) a = b = c = 0.0f;
			if (v->type==_scenario_trigger_volume_type_oriented)
			{
				real_vector3d left;

				cross_product3d(&v->up, &v->forward, &left);
				a *= v->extents.i; b *= v->extents.j; c *= v->extents.k;
				point.x = v->position.x+v->forward.i*a+left.i*b+v->up.i*c;
				point.y = v->position.y+v->forward.j*a+left.j*b+v->up.j*c;
				point.z = v->position.z+v->forward.k*a+left.k*b+v->up.k*c;
			}
			else
			{
				point.x = v->bounds.x0+(v->bounds.x1-v->bounds.x0)*a;
				point.y = v->bounds.y0+(v->bounds.y1-v->bounds.y0)*b;
				point.z = v->bounds.z0+(v->bounds.z1-v->bounds.z0)*c;
			}
			printf("test %d %.9g %.9g %.9g %d\n", volume, point.x, point.y, point.z,
				scenario_trigger_volume_test_point(volume, &point));
		}
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    real_math_h = read("source/math/real_math.h")
    matrix_c = read("source/math/matrix_math.c")
    scenario_c = read("source/scenario/scenario.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            *(inline(real_math_h, name) for name in INLINE_FUNCTIONS),
            *(function(matrix_c, name) for name in MATRIX_FUNCTIONS),
            function(scenario_c, "scenario_trigger_volume_test_point"),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="scenario-reference-") as work:
        source = Path(work) / "scenario_reference.c"
        executable = Path(work) / "scenario_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's trigger volumes (tools/engine_scenario_reference.py).\n"
        "# volume <index> <type> <position> <forward> <up> <extents>\n"
        "# test <index> <point> <inside>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_scenario_reference.py")
        print("the scenario reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
