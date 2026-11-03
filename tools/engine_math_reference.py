"""The game's own vector maths that the native engine ports by itself (the
angle between two vectors, a vector's yaw and pitch in a frame, and a
matrix turning a normal), taken from the sources and run over vectors, to
record what the engine's must give.

    python3 tools/engine_math_reference.py          # writes the reference
    python3 tools/engine_math_reference.py --check  # fails if it is stale

The functions are source/math/real_math.c's `angle_between_vectors3d` and
`euler_angles2d_from_vector3d` and source/math/matrix_math.c's
`matrix4x3_transform_normal` and `matrix4x3_inverse_transform_normal`,
source/math/geometry.c's `geosphere_new` and its helpers, and
source/math/random_math.c's seeded numbers and
`seed_random_vector_in_cone3d`, with real_math.h's inline helpers (and its
one-operation helpers, which the engine leaves to glam and libm, run over
vectors of their own), found by name as tools/harness finds them, and
compiled for this machine against a small fake world. The table is written
to tools/traces/math_reference.txt, which
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
from harness import enum_with, function, inline, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "math_reference.txt"

REAL_MATH_FUNCTIONS = ["angle_between_vectors3d", "euler_angles2d_from_vector3d", "rotate_vector_about_axis"]

MATRIX_FUNCTIONS = ["matrix4x3_transform_normal", "matrix4x3_inverse_transform_normal"]

INLINE_FUNCTIONS = [
    "set_real_point3d",
    "dot_product3d",
    "arctangent",
    "arccosine",
    "square_root",
    "sine",
    "cosine",
    "scale_vector3d",
    "magnitude_squared3d",
    "magnitude3d",
    "normalize3d",
    "cross_product3d",
]

# real_math.h's helpers of one operation each, which the engine does with a
# glam or libm primitive (in the order they call one another)
HELPER_FUNCTIONS = [
    "reciprocal_square_root",
    "arcsine",
    "tangent",
    "power",
    "valid_real",
    "interpolate_scalar",
    "set_real_vector2d",
    "set_real_point2d",
    "set_real_vector3d",
    "set_real_euler_angles2d",
    "dot_product2d",
    "cross_product2d",
    "magnitude_squared2d",
    "magnitude2d",
    "scale_vector2d",
    "negate_vector2d",
    "negate_vector3d",
    "vector_from_points2d",
    "vector_from_points3d",
    "distance_squared2d",
    "distance_squared3d",
    "distance2d",
    "distance3d",
    "midpoint3d",
    "point_from_line2d",
    "point_from_line3d",
    "point_in_circle",
    "point_in_sphere",
    "rotate_vector2d",
    "limit2d",
    "limit3d",
    "plane3d_distance_to_point",
]

GEOMETRY_FUNCTIONS = [
    "geosphere_new",
    "geosphere_dispose",
    "subdivide_triangle",
    "get_face_vertex",
    "get_edge_vertex",
    "calculate_vertex",
]

RANDOM_FUNCTIONS = [
    "seed_random",
    "seed_random_range",
    "real_seed_random",
    "real_seed_random_range",
    "direction3d_from_table",
    "seed_random_vector_in_cone3d",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef int boolean;

#define NONE (-1)
#define TRUE 1
#define FALSE 0
#define MIN(a,b) ((a)<(b)?(a):(b))
#define MAX(a,b) ((a)>(b)?(a):(b))
#define NUMBER_OF_VERTICES_PER_TRIANGLE 3
#define RANDOM_A 1664525u
#define RANDOM_C 1013904223u
#define _real_epsilon 0.0001f
#define match_malloc(file, line, size) malloc(size)
#define match_free(file, line, pointer) free(pointer)
#define match_assert(file, line, condition) ((void)0)

#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define _pi ((real)3.14159265358979323846f)

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct { real yaw, pitch; } real_euler_angles2d;
typedef struct { real i, j; } real_vector2d;
typedef struct { real x, y; } real_point2d;
typedef struct { real_vector3d n; real d; } real_plane3d;
typedef struct
{
	real scale;
	real_vector3d forward;
	real_vector3d left;
	real_vector3d up;
	real_point3d position;
} real_matrix4x3;

/* random_math.c's globals, the direction table's */
static struct
{
	real_vector3d *random_direction_table;
	short random_direction_table_size;
} random_math_globals;
"""

MAIN = r"""
static unsigned int seed = 0x5eed;

/* a component from -2 to 2, in steps the engine reads back exactly */
static real component(void)
{
	seed = seed*1664525u+1013904223u;
	return (real)((int)(seed>>16)%4001-2000)*0.001f;
}

static real_vector3d vector(void)
{
	real_vector3d v;

	v.i = component();
	v.j = component();
	v.k = component();
	return v;
}

static void print_vector(real_vector3d const *v)
{
	printf(" %.9g %.9g %.9g", v->i, v->j, v->k);
}

int main(void)
{
	int step;

	/* the angle between two vectors: at random, and the zero, the same,
	the opposite and the perpendicular */
	for (step = 0; step<400; step++)
	{
		real_vector3d a = vector();
		real_vector3d b = step%8==1 ? a : vector();

		if (step%8==2) { b.i = -a.i*3.0f; b.j = -a.j*3.0f; b.k = -a.k*3.0f; }
		if (step%8==3) { b.i = -a.j; b.j = a.i; b.k = 0.0f; }
		if (step%40==4) { a.i = 0.0f; a.j = 0.0f; a.k = 0.0f; }
		printf("angle");
		print_vector(&a);
		print_vector(&b);
		printf(" %.9g\n", angle_between_vectors3d(&a, &b));
	}

	/* a vector's yaw and pitch in a frame (its left up x forward), and the
	frame turning it */
	for (step = 0; step<400; step++)
	{
		real_matrix4x3 matrix;
		real_vector3d v = vector();
		real_vector3d local;
		real_vector3d turned;
		real_euler_angles2d angles;

		matrix.scale = 1.0f;
		matrix.forward = vector();
		matrix.up = vector();
		if (step%50==0) { v.i = 0.0f; v.j = 0.0f; v.k = 0.0f; }
		matrix.left.i = matrix.up.j*matrix.forward.k - matrix.up.k*matrix.forward.j;
		matrix.left.j = matrix.up.k*matrix.forward.i - matrix.up.i*matrix.forward.k;
		matrix.left.k = matrix.up.i*matrix.forward.j - matrix.up.j*matrix.forward.i;
		matrix4x3_inverse_transform_normal(&matrix, &v, &local);
		euler_angles2d_from_vector3d(&angles, &local);
		matrix4x3_transform_normal(&matrix, &v, &turned);
		printf("frame");
		print_vector(&matrix.forward);
		print_vector(&matrix.up);
		print_vector(&v);
		printf(" %.9g %.9g", angles.yaw, angles.pitch);
		print_vector(&turned);
		printf("\n");
	}

	/* the random directions, as random_math_initialize makes them */
	{
		struct geosphere *sphere = geosphere_new(16);
		unsigned long cone_seed = 0xc0e;
		static real const cones[][2] = {
			{ 0.0f, 15.0f }, { 0.0f, _pi/12.0f }, { 0.1f, 0.5f }, { 0.0f, 0.0f }, { 1.0f, 3.0f },
		};

		random_math_globals.random_direction_table = (real_vector3d *)sphere->vertices;
		random_math_globals.random_direction_table_size = sphere->vertex_count;
		for (step = 0; step<sphere->vertex_count; step++)
		{
			printf("direction");
			print_vector(&random_math_globals.random_direction_table[step]);
			printf("\n");
		}

		/* a vector in a cone about an axis: at random, along a table
		direction, and zero */
		for (step = 0; step<400; step++)
		{
			real_vector3d axis = vector();
			real_vector3d result;
			real const *cone = cones[step%5];
			unsigned long before = cone_seed;

			if (step%25==7) axis = random_math_globals.random_direction_table[step];
			if (step%50==9) { axis.i = 0.0f; axis.j = 0.0f; axis.k = 0.0f; }
			seed_random_vector_in_cone3d(&cone_seed, &axis, cone[0], cone[1], &result);
			printf("cone %u", before);
			print_vector(&axis);
			printf(" %.9g %.9g", cone[0], cone[1]);
			print_vector(&result);
			printf(" %u\n", cone_seed);
		}
	}
	/* the one-operation helpers, over vectors of their own: at random,
	with zero vectors and a zero length now and then, and the reals that
	are not numbers */
	for (step = 0; step<400; step++)
	{
		static real const special[] = { INFINITY, -INFINITY, NAN, 0.0f };
		real_vector3d a3 = vector();
		real_vector3d b3 = vector();
		real_vector3d v3 = vector();
		real_vector3d r3;
		real_vector2d a2, b2, r2;
		real_point2d p2, q2, s2;
		real_point3d p3, q3, s3;
		real_euler_angles2d angles;
		real_plane3d plane;
		real t = component();
		real x = component()*0.5f;
		real angle = component()*_pi;
		real length = step%40==6 ? 0.0f : fabsf(component())*1.5f;
		real base = fabsf(component())*2.0f;
		real current = component();
		real n = step%10==0 ? special[(step/10)%4] : t;
		real sine_value = sinf(angle);
		real cosine_value = cosf(angle);

		if (step%50==5) { a3.i = 0.0f; a3.j = 0.0f; a3.k = 0.0f; }
		set_real_vector2d(&a2, a3.i, a3.j);
		set_real_vector2d(&b2, b3.i, b3.j);
		set_real_point2d(&p2, a3.k, b3.k);
		set_real_point2d(&q2, v3.i, v3.j);
		p3.x = a3.i; p3.y = a3.j; p3.z = a3.k;
		q3.x = b3.i; q3.y = b3.j; q3.z = b3.k;
		set_real_vector3d(&plane.n, v3.k, v3.i, v3.j);
		plane.d = t;
		set_real_euler_angles2d(&angles, t, angle);
		printf("set_real_euler_angles2d %.9g %.9g %.9g %.9g\n", t, angle, angles.yaw, angles.pitch);
		printf("arcsine %.9g %.9g\n", x, arcsine(x));
		printf("tangent %.9g %.9g\n", angle, tangent(angle));
		printf("power %.9g %.9g %.9g\n", base, t, power(base, t));
		printf("valid_real %.9g %d\n", n, valid_real(n) ? 1 : 0);
		{
			real moved = current;

			interpolate_scalar(&moved, t, length);
			printf("interpolate_scalar %.9g %.9g %.9g %.9g\n", current, t, length, moved);
		}
		printf("dot_product2d %.9g %.9g %.9g %.9g %.9g\n", a2.i, a2.j, b2.i, b2.j, dot_product2d(&a2, &b2));
		printf("cross_product2d %.9g %.9g %.9g %.9g %.9g\n", a2.i, a2.j, b2.i, b2.j, cross_product2d(&a2, &b2));
		printf("magnitude_squared2d %.9g %.9g %.9g\n", a2.i, a2.j, magnitude_squared2d(&a2));
		printf("magnitude2d %.9g %.9g %.9g\n", a2.i, a2.j, magnitude2d(&a2));
		scale_vector2d(&a2, t, &r2);
		printf("scale_vector2d %.9g %.9g %.9g %.9g %.9g\n", a2.i, a2.j, t, r2.i, r2.j);
		negate_vector2d(&a2, &r2);
		printf("negate_vector2d %.9g %.9g %.9g %.9g\n", a2.i, a2.j, r2.i, r2.j);
		negate_vector3d(&a3, &r3);
		printf("negate_vector3d");
		print_vector(&a3);
		print_vector(&r3);
		printf("\n");
		vector_from_points2d(&p2, &q2, &r2);
		printf("vector_from_points2d %.9g %.9g %.9g %.9g %.9g %.9g\n", p2.x, p2.y, q2.x, q2.y, r2.i, r2.j);
		vector_from_points3d(&p3, &q3, &r3);
		printf("vector_from_points3d");
		print_vector(&a3);
		print_vector(&b3);
		print_vector(&r3);
		printf("\n");
		printf("distance_squared2d %.9g %.9g %.9g %.9g %.9g\n", p2.x, p2.y, q2.x, q2.y, distance_squared2d(&p2, &q2));
		printf("distance2d %.9g %.9g %.9g %.9g %.9g\n", p2.x, p2.y, q2.x, q2.y, distance2d(&p2, &q2));
		printf("distance_squared3d");
		print_vector(&a3);
		print_vector(&b3);
		printf(" %.9g\n", distance_squared3d(&p3, &q3));
		printf("distance3d");
		print_vector(&a3);
		print_vector(&b3);
		printf(" %.9g\n", distance3d(&p3, &q3));
		midpoint3d(&p3, &q3, &s3);
		printf("midpoint3d");
		print_vector(&a3);
		print_vector(&b3);
		printf(" %.9g %.9g %.9g\n", s3.x, s3.y, s3.z);
		point_from_line2d(&p2, &a2, t, &s2);
		printf("point_from_line2d %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", p2.x, p2.y, a2.i, a2.j, t, s2.x, s2.y);
		point_from_line3d(&p3, &v3, t, &s3);
		printf("point_from_line3d");
		print_vector(&a3);
		print_vector(&v3);
		printf(" %.9g %.9g %.9g %.9g\n", t, s3.x, s3.y, s3.z);
		printf("point_in_circle %.9g %.9g %.9g %.9g %.9g %d\n", p2.x, p2.y, q2.x, q2.y, length*2.0f, point_in_circle(&p2, &q2, length*2.0f) ? 1 : 0);
		printf("point_in_sphere");
		print_vector(&a3);
		print_vector(&b3);
		printf(" %.9g %d\n", length*2.0f, point_in_sphere(&p3, &q3, length*2.0f) ? 1 : 0);
		rotate_vector2d(&a2, sine_value, cosine_value, &r2);
		printf("rotate_vector2d %.9g %.9g %.9g %.9g %.9g %.9g\n", a2.i, a2.j, sine_value, cosine_value, r2.i, r2.j);
		{
			real_vector2d limited2 = a2;
			real_vector3d limited3 = a3;
			boolean limited = limit2d(&limited2, length);

			printf("limit2d %.9g %.9g %.9g %d %.9g %.9g\n", a2.i, a2.j, length, limited ? 1 : 0, limited2.i, limited2.j);
			limited = limit3d(&limited3, length);
			printf("limit3d");
			print_vector(&a3);
			printf(" %.9g %d", length, limited ? 1 : 0);
			print_vector(&limited3);
			printf("\n");
		}
		printf("plane3d_distance_to_point");
		print_vector(&plane.n);
		printf(" %.9g", plane.d);
		print_vector(&a3);
		printf(" %.9g\n", plane3d_distance_to_point(&plane, &p3));
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    real_math_c = read("source/math/real_math.c")
    real_math_h = read("source/math/real_math.h")
    matrix_c = read("source/math/matrix_math.c")
    geometry_c = read("source/math/geometry.c")
    geometry_h = read("source/math/geometry.h")
    random_c = read("source/math/random_math.c")
    helpers = [inline(real_math_h, name) for name in INLINE_FUNCTIONS + HELPER_FUNCTIONS]
    definitions = [function(real_math_c, name) for name in REAL_MATH_FUNCTIONS]
    definitions += [function(matrix_c, name) for name in MATRIX_FUNCTIONS]
    # the geosphere: its counts, its structure, the private functions'
    # prototypes, the octahedron it subdivides, and its functions
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
    geosphere += [function(geometry_c, name) for name in GEOMETRY_FUNCTIONS]
    randoms = [function(random_c, name) for name in RANDOM_FUNCTIONS]
    return "\n\n".join([FAKE_WORLD, *helpers, *definitions, *geosphere, *randoms, MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="math-reference-") as work:
        source = Path(work) / "math_reference.c"
        executable = Path(work) / "math_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's vector maths (tools/engine_math_reference.py).\n"
        "# angle <a> <b> <angle between>\n"
        "# frame <forward> <up> <vector> <yaw> <pitch> <the frame turning the vector>\n"
        "# direction <the random direction table's vector, in order>\n"
        "# cone <seed before> <axis> <inner> <outer> <the vector drawn> <seed after>\n"
        "# <a one-operation helper's name> <its arguments> <what it gives>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_math_reference.py")
        print("the math reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
