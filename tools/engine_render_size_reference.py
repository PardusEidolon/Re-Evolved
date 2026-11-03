"""The game's size of a sphere on screen and its contrails' fade: how many
pixels across a sphere is drawn at a depth, and how much of a contrail's
strip shows as it turns from the camera, recorded for the engine's to
match.

    python3 tools/engine_render_size_reference.py          # writes the reference
    python3 tools/engine_render_size_reference.py --check  # fails if it is stale

The functions are source/render/render_cameras.c's
`render_frustum_sphere_diameter_in_pixels` (its frustum's vertical scale
built as `render_camera_build_frustum` builds it, from the field of view's
tangent and the viewport's height) and source/render/render_contrails.c's
`contrail_fade`, with source/math/periodic_functions.c's transition tables
and `transition_function_evaluate`, compiled for this machine against a
small fake world: frustums of a range of fields of view and heights, spheres
before, behind and at the eye, and strips at points facing each way under
each fade mode with and without the slow fade. A camera on the strip's
point (its fade divides by zero) is not asked. The table is written to
tools/traces/render_size_reference.txt, which
ffi/src/render_model.rs's and ffi/src/particles/contrails.rs's reference
tests read.
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

OUTPUT = ROOT / "tools" / "traces" / "render_size_reference.txt"

INLINE_FUNCTIONS = [
    "tangent",
    "square_root",
    "magnitude_squared3d",
    "magnitude3d",
    "dot_product3d",
    "vector_from_points3d",
]

PERIODIC_FUNCTIONS = [
    "periodic_function_build_variable_period_x_table",
    "transition_function_build_table",
    "periodic_function_build_table",
    "periodic_functions_initialize",
    "transition_function_evaluate",
]

RANDOM_FUNCTIONS = ["seed_random", "seed_random_range", "real_seed_random"]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef float real;
typedef unsigned char byte;
typedef unsigned short word;

#define TRUE 1
#define FALSE 0
#define __fastcall
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define MAX(a,b) ((a)>(b)?(a):(b))
#define ABS(x) ((x)<0?-(x):(x))
#define VALID_INDEX(index, count) (index>=0 && index<count)
#define match_assert(file, line, condition) ((void)0)
#define display_assert(message, file, line, fatal) ((void)0)
#define match_malloc(file, line, size) malloc(size)
#define system_exit(code) exit(code)
#define function_type_error(kind, type) ((void)0)
#define _pi ((real)3.14159265358979323846f)
#define RANDOM_A 1664525u
#define RANDOM_C 1013904223u

static unsigned long global_random_seed;

static unsigned long *get_global_random_seed_address(void)
{
	return &global_random_seed;
}

typedef struct { real i, j; } real_vector2d;
typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct
{
	real scale;
	real_vector3d forward;
	real_vector3d left;
	real_vector3d up;
	real_point3d position;
} real_matrix4x3;

/* the frustum's fields the size reads */
struct render_frustum
{
	real_matrix4x3 world_to_view;
	real_vector2d projection_world_to_screen;
};

enum { _contrail_first_point_unfaded_bit = 0, _contrail_last_point_unfaded_bit, _contrail_fades_slowly_bit = 6 };

/* the definition's field the fade reads */
struct contrail_definition { word flags; };

static struct { struct { real_point3d position; } camera; } render;
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

/* a unit vector */
static real_vector3d direction(void)
{
	real_vector3d v = { between(1.0f), between(1.0f), between(1.0f) };
	real length = (real)sqrt(v.i*v.i+v.j*v.j+v.k*v.k);

	if (length<0.01f) { v.i = 1.0f; v.j = 0.0f; v.k = 0.0f; length = 1.0f; }
	v.i /= length; v.j /= length; v.k /= length;
	return v;
}

int main(void)
{
	static real const fields_of_view[] = { 0.3f, 0.7f, 1.0f, 1.2217305f, 1.5707964f, 2.5f };
	static long const heights[] = { 480, 720, 1001, 1080, 2160 };
	int f, h, step;

	periodic_functions_initialize();

	/* spheres about each frustum: the depth through its world_to_view, as
	the game takes it, before, behind and at the eye */
	for (f = 0; f<(int)(sizeof(fields_of_view)/sizeof(fields_of_view[0])); f++)
		for (h = 0; h<(int)(sizeof(heights)/sizeof(heights[0])); h++)
		{
			struct render_frustum frustum;
			real viewport_height = (real)heights[h];
			real field_of_view_tangent = tangent(fields_of_view[f]*0.5f);
			real projection_y_scale = 1.0f/(field_of_view_tangent*1.0f);

			frustum.world_to_view.scale = 1.0f;
			frustum.world_to_view.forward = direction();
			frustum.world_to_view.left = direction();
			frustum.world_to_view.up = direction();
			frustum.world_to_view.position.x = between(20.0f);
			frustum.world_to_view.position.y = between(20.0f);
			frustum.world_to_view.position.z = between(20.0f);
			frustum.projection_world_to_screen.i = 0.0f;
			frustum.projection_world_to_screen.j = projection_y_scale*viewport_height*0.5f;
			for (step = 0; step<20; step++)
			{
				real_point3d point = { between(60.0f), between(60.0f), between(60.0f) };
				real radius = step==0 ? 0.0f : 0.01f+between(5.0f)+5.0f;
				real depth;

				if (step<3)
				{
					/* on the eye's plane, and a hair before and behind it */
					point.x = point.y = point.z = 0.0f;
					frustum.world_to_view.position.z = step==0 ? 0.0f : step==1 ? 0.05f : -0.0999f;
				}
				depth =
					frustum.world_to_view.up.k * point.z +
					frustum.world_to_view.left.k * point.y +
					frustum.world_to_view.forward.k * point.x +
					frustum.world_to_view.position.z;
				printf("sphere %.9g %d %.9g %.9g %.9g %.9g\n", fields_of_view[f], heights[h], depth, radius,
					frustum.projection_world_to_screen.j,
					render_frustum_sphere_diameter_in_pixels(&frustum, &point, radius));
			}
		}

	/* strips at points facing each way, under each fade mode, with and
	without the slow fade */
	for (step = 0; step<400; step++)
	{
		struct contrail_definition definition;
		real_point3d point = { between(30.0f), between(30.0f), between(30.0f) };
		real_vector3d normal = direction();
		short mode;

		render.camera.position.x = between(30.0f);
		render.camera.position.y = between(30.0f);
		render.camera.position.z = between(30.0f);
		if (step%25==0)
		{
			/* the camera straight out from the strip, facing it */
			render.camera.position.x = point.x+normal.i*7.5f;
			render.camera.position.y = point.y+normal.j*7.5f;
			render.camera.position.z = point.z+normal.k*7.5f;
		}
		if (render.camera.position.x==point.x && render.camera.position.y==point.y && render.camera.position.z==point.z)
			continue;
		for (definition.flags = 0; definition.flags<=FLAG(_contrail_fades_slowly_bit); definition.flags += FLAG(_contrail_fades_slowly_bit))
			for (mode = 0; mode<=2; mode++)
				printf("fade %d %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", definition.flags, mode,
					point.x, point.y, point.z, normal.i, normal.j, normal.k,
					render.camera.position.x, render.camera.position.y, render.camera.position.z,
					contrail_fade(&definition, mode, &point, &normal));
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    periodic_c = read("source/math/periodic_functions.c")
    periodic_h = read("source/math/periodic_functions.h")
    random_c = read("source/math/random_math.c")
    real_math_h = read("source/math/real_math.h")
    cseries_h = read("source/cseries/cseries.h")
    cameras_c = read("source/render/render_cameras.c")
    contrails_c = read("source/render/render_contrails.c")
    enums = [
        enum_with(periodic_c, "_periodic_function_one"),
        enum_with(periodic_c, "PERIODIC_FUNCTION_TABLE_SIZE"),
        enum_with(periodic_c, "SLIDE_PERIODIC_FUNCTION_FLAGS"),
        enum_with(periodic_h, "_transition_function_linear"),
    ]
    tables = periodic_c[periodic_c.index("static boolean function_tables_initialized") :]
    tables = tables[: tables.index("/* ---------- public code */")]
    helpers = [inline(cseries_h, "fast_ftol")]
    helpers += [function(random_c, name) for name in RANDOM_FUNCTIONS]
    helpers += [inline(real_math_h, name) for name in ("set_random_seed", "real_random")]
    helpers += [inline(real_math_h, name) for name in INLINE_FUNCTIONS]
    definitions = [function(periodic_c, name) for name in PERIODIC_FUNCTIONS]
    definitions += [
        function(cameras_c, "render_frustum_sphere_diameter_in_pixels"),
        function(contrails_c, "contrail_fade"),
    ]
    return "\n\n".join([FAKE_WORLD, *enums, tables, *helpers, *definitions, MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="render-size-reference-") as work:
        source = Path(work) / "render_size_reference.c"
        executable = Path(work) / "render_size_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's sphere sizes and contrail fades (tools/engine_render_size_reference.py).\n"
        "# sphere <vertical field of view> <viewport height> <depth> <radius> <vertical scale> <pixels>\n"
        "# fade <flags> <mode> <point> <normal> <camera> <fade>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_render_size_reference.py")
        print("the render size reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
