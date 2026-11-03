"""The game's slipping effects: where, facing which way and how hard a
vehicle's mass point on the ground slips as it slides, run over random mass
points to record what the engine's must give.

    python3 tools/engine_vehicle_slip_reference.py          # writes the reference
    python3 tools/engine_vehicle_slip_reference.py --check  # fails if it is stale

The function is source/units/vehicles.c's `create_slipping_effects`, with
the maths of source/math/real_math.h, compiled for this machine against a
small fake world whose material_effect_new records what it is asked for.
The table is written to tools/traces/vehicle_slip_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "vehicle_slip_reference.txt"

INLINE_FUNCTIONS = [
    "square_root",
    "dot_product3d",
    "magnitude_squared3d",
    "magnitude3d",
    "scale_vector3d",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef byte boolean;

#define NONE -1
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define FLAG(b) (1<<(b))
#define TEST_FLAG(flags, bit) (((flags)&(unsigned)FLAG(bit))!=0)
"""

WORLD = r"""
struct mass_point_datum
{
	unsigned long flags;
	real_point3d position;
	real_vector3d velocity_relative_to_ground;
	real_plane3d ground_plane;
	short ground_material_type;
	real ground_depth;
};

struct physics_mass_point_definition { unsigned long flags; real unknown68; };
struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[(index)])

struct physics_definition { struct tag_block mass_points; };
struct tag_reference { long index; };
struct vehicle_definition
{
	struct { struct { struct tag_reference physics; } object; } unit;
	struct tag_reference material_effects;
};
struct location { long leaf_index; };
struct vehicle_datum { long definition_index; struct { struct location location; } object; };
struct powered_mass_point_datum { real unused; };

static struct vehicle_datum the_vehicle;
static struct vehicle_definition the_definition;
static struct physics_definition the_physics;
static struct physics_mass_point_definition the_mass_point_definition;

static struct vehicle_datum *vehicle_datum_get(long index) { return &the_vehicle; }
static struct vehicle_definition *vehicle_specific_definition_get(long index) { return &the_definition; }
static struct physics_definition *physics_definition_get(long index) { return &the_physics; }

/* what the slip asks for */
static int asked;

static void material_effect_new(long definition_index, short type, short material_type,
	real_point3d const *position, real_vector3d const *normal, struct location const *location, real scale)
{
	asked = 1;
	printf(" %d %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g", type, material_type, position->x, position->y,
		position->z, normal->i, normal->j, normal->k, scale);
}
"""

MAIN = r"""
static unsigned int seed = 0x511d;

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

int main(void)
{
	int step;

	the_definition.material_effects.index = 0;
	the_physics.mass_points.count = 1;
	the_physics.mass_points.address = &the_mass_point_definition;
	for (step = 0; step<2000; step++)
	{
		struct mass_point_datum point;
		real_vector3d n;
		real length;

		memset(&point, 0, sizeof(point));
		point.flags = step%9==0 ? 0 : FLAG(1);
		point.position.x = between(30.0f);
		point.position.y = between(30.0f);
		point.position.z = between(10.0f);
		/* slides mostly faster than slips, now and then slower or still */
		point.velocity_relative_to_ground.i = between(0.3f);
		point.velocity_relative_to_ground.j = between(0.3f);
		point.velocity_relative_to_ground.k = between(0.05f);
		if (step%7==0)
		{
			point.velocity_relative_to_ground.i *= 0.05f;
			point.velocity_relative_to_ground.j *= 0.05f;
		}
		if (step%31==0) memset(&point.velocity_relative_to_ground, 0, sizeof(real_vector3d));
		do
		{
			n.i = between(0.5f);
			n.j = between(0.5f);
			n.k = between(0.5f)+0.5f;
			length = magnitude3d(&n);
		} while (length==0.0f);
		scale_vector3d(&n, 1.0f/length, &n);
		if (step%5==0) n.i = 0.0f, n.j = 0.0f, n.k = 1.0f;
		point.ground_plane.n = n;
		point.ground_material_type = (short)((next()>>24)%40)-1;
		the_mass_point_definition.unknown68 = 0.05f+between(0.5f)+0.5f;
		point.ground_depth = between(the_mass_point_definition.unknown68)*0.5f+
			the_mass_point_definition.unknown68*0.5f;
		the_mass_point_definition.flags = step%3==0 ? FLAG(0) : 0;

		printf("slip %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %d %.9g %.9g %lu ->", TEST_FLAG(point.flags, 1),
			point.position.x, point.position.y, point.position.z, point.velocity_relative_to_ground.i,
			point.velocity_relative_to_ground.j, point.velocity_relative_to_ground.k, n.i, n.j, n.k,
			point.ground_material_type, point.ground_depth, the_mass_point_definition.unknown68,
			the_mass_point_definition.flags);
		asked = 0;
		create_slipping_effects(0, NULL, &point);
		printf(asked ? "\n" : " none\n");
	}
	return 0;
}
"""


def math_types():
    """real_math.h's vector, quaternion, matrix and plane types."""
    header = read("source/math/real_math.h")
    start = header.index("union real_euler_angles2d")
    end = header.index("typedef struct real_plane3d real_plane3d;")
    return header[start : header.index("\n", end)]


def source_file():
    """The fake world, the game's function and the cases, as one C file."""
    real_math_h = read("source/math/real_math.h")
    vehicles_c = read("source/units/vehicles.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            math_types(),
            WORLD,
            *(inline(real_math_h, name) for name in INLINE_FUNCTIONS),
            function(vehicles_c, "create_slipping_effects"),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="vehicle-slip-reference-") as work:
        source = Path(work) / "vehicle_slip_reference.c"
        executable = Path(work) / "vehicle_slip_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's slipping effects (tools/engine_vehicle_slip_reference.py).\n"
        "# slip <on ground> <position> <velocity relative to the ground> <ground normal> <material>\n"
        "#   <ground depth> <radius> <definition flags>\n"
        "#   -> <effect type> <material> <position> <normal> <scale>, or none\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_vehicle_slip_reference.py")
        print("the vehicle slip reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
