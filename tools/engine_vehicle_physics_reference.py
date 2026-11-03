"""The game's vehicle controllers: what the jeep's, the tank's, the boat's,
the scout's and the fighter's hand their physics update (each powered mass
point's push, and the magic force and torque), what they change in the
vehicle as they do, how a vehicle counts its ticks off the ground, and a
vehicle at rest slowing to a stop, run over random vehicles to record what
the engine's must give.

    python3 tools/engine_vehicle_physics_reference.py          # writes the reference
    python3 tools/engine_vehicle_physics_reference.py --check  # fails if it is stale

The functions are source/units/vehicles.c's `update_human_jeep_physics`,
`update_human_tank_physics`, `update_human_boat_physics`,
`update_alien_scout_physics`, `update_alien_fighter_physics` (with its
`_new` and `_old`), `compute_acceleration`, `compute_airborne_ticks`,
`slowly_stop_vehicle` and (of each vehicle before) `vehicle_is_flipped`,
with the maths of source/math/, compiled for this machine against a
small fake world whose physics_update records what it is handed. The table is written to
tools/traces/vehicle_physics_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "vehicle_physics_reference.txt"

INLINE_FUNCTIONS = [
    "sine",
    "cosine",
    "arctangent",
    "arccosine",
    "square_root",
    "reciprocal_square_root",
    "set_real_vector2d",
    "set_real_vector3d",
    "set_real_point3d",
    "dot_product2d",
    "cross_product2d",
    "magnitude_squared2d",
    "magnitude2d",
    "scale_vector2d",
    "normalize2d",
    "dot_product3d",
    "magnitude_squared3d",
    "magnitude3d",
    "scale_vector3d",
    "normalize3d",
    "limit3d",
    "point_from_line3d",
    "cross_product3d",
    "add_vectors3d",
    "subtract_vectors3d",
    "negate_vector3d",
]

REAL_MATH_FUNCTIONS = [
    "rotate_vector_about_axis",
    "yaw_vectors",
    "pitch_vectors",
    "quaternion_to_angle_and_vector",
    "angle_between_vectors3d",
]

MATRIX_FUNCTIONS = [
    "matrix3x3_from_forward_and_up",
    "matrix3x3_transpose",
    "matrix3x3_multiply",
    "matrix3x3_rotation_to_quaternion",
    "matrix4x3_rotation_from_vectors",
    "matrix4x3_from_point_and_vectors",
    "matrix4x3_rotation_from_axis_and_angle",
    "matrix4x3_inverse",
    "matrix4x3_multiply",
    "matrix4x3_rotation_to_quaternion",
    "matrix4x3_transform_vector",
    "matrix4x3_inverse_transform_vector",
]

VEHICLE_FUNCTIONS = [
    "vehicle_is_flipped",
    "compute_airborne_ticks",
    "compute_acceleration",
    "update_alien_fighter_physics_new",
    "update_alien_fighter_physics_old",
    "slowly_stop_vehicle",
    "update_human_tank_physics",
    "vehicle_cross_product3d_target",
    "vehicle_dot_product3d_target",
    "vehicle_triple_product3d_target",
    "update_human_boat_physics",
    "update_human_jeep_physics",
    "update_alien_scout_physics",
    "update_alien_fighter_physics",
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

#define TRUE 1
#define FALSE 0
#define NONE -1
#define MIN(a,b) ((a)>(b)?(b):(a))
#define MAX(a,b) ((a)>(b)?(a):(b))
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define FLAG(b) (1<<(b))
#define TEST_FLAG(flags, bit) (((flags)&(unsigned)FLAG(bit))!=0)
#define _real_epsilon 0.0001f
#define _pi ((real)3.14159265358979323846f)
#define VEHICLE_ANGULAR_ACCELERATION 0.0139626344f
#define match_vassert(file, line, condition, message) ((void)0)
#define vassert(condition, message) ((void)0)
"""

WORLD = r"""
struct powered_mass_point_datum
{
	real ground_friction_velocity;
	real water_friction_velocity;
	real air_friction_velocity;
	real water_lift_ratio;
	real air_lift_ratio;
	real thrust_fraction;
	real antigrav_fraction;
	union
	{
		real_quaternion rotation;
		real rotation_values[4];
		real_vector2d rotation_pairs[2];
	};
	real_matrix4x3 rotation_matrix;
};

struct mass_point_datum { unsigned long flags; };
struct physics_mass_point_definition { short powered_mass_point_index; };
struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[(index)])

struct physics_definition
{
	real radius, mass, xx_moment, yy_moment, zz_moment;
	struct tag_block mass_points, powered_mass_points;
};

struct unit_definition { struct { struct { long index; } physics; } object; };

struct vehicle_definition
{
	struct unit_definition unit;
	real unknown2f8, unknown2fc, unknown300, unknown304, unknown308, unknown314, unknown364;
	real wheel_circumference;
};

struct location { long leaf_index; };

struct vehicle_datum
{
	long definition_index;
	struct
	{
		real_point3d position;
		real_vector3d forward, up, translational_velocity, angular_velocity;
		struct location location;
	} object;
	struct
	{
		real seat_power[2];
		real_vector2d throttle;
		real_vector3d desired_facing_vector;
	} unit;
	struct
	{
		real speed, turn, wheel, left_tread, right_tread, hover, thrust;
		short stop_time;
		byte airborne_ticks, on_ground_ticks;
		unsigned long flags;
	} vehicle;
};

static struct vehicle_datum the_vehicle;
static struct vehicle_definition the_definition;
static struct physics_definition the_physics;
static struct physics_mass_point_definition the_mass_point_definitions[8];
static boolean driven_by_ai;
static real water_depth_here;

static real_vector3d const up3d = {{0.0f, 0.0f, 1.0f}};
static real_vector3d const forward3d = {{1.0f, 0.0f, 0.0f}};
static real_vector3d const zero3d = {{0.0f, 0.0f, 0.0f}};
static real_vector2d const zero2d = {{0.0f, 0.0f}};
static real_quaternion const identity_quaternion = {{{0.0f, 0.0f, 0.0f}}, 1.0f};
#define global_up3d (&up3d)
#define global_forward3d (&forward3d)
#define global_zero_vector3d (&zero3d)
#define global_zero_vector2d (&zero2d)
#define global_identity_quaternion (&identity_quaternion)
static real global_gravity = 0.0035651792f;

static struct vehicle_datum *vehicle_datum_get(long index) { return &the_vehicle; }
static struct vehicle_definition *vehicle_specific_definition_get(long index) { return &the_definition; }
static struct unit_definition *vehicle_definition_get(long index) { return &the_definition.unit; }
static struct physics_definition *physics_definition_get(long index) { return &the_physics; }
static boolean unit_driven_by_ai(long index) { return driven_by_ai; }
static real scenario_location_water_depth(struct location const *location, real_point3d const *position)
{
	return water_depth_here;
}
static void create_ghost_effect(long index) { }

static void object_set_position(long index, real_point3d const *position, real_vector3d const *forward,
	real_vector3d const *up)
{
	the_vehicle.object.position = *position;
	the_vehicle.object.forward = *forward;
	the_vehicle.object.up = *up;
}

/* what the controller hands the physics update */
static int handed_powered;
static struct powered_mass_point_datum handed[3];
static int handed_force, handed_torque;
static real_vector3d handed_forces[2];

static void physics_update(long index, struct powered_mass_point_datum *state, struct mass_point_datum *mass_points,
	real_vector3d *force, real_vector3d *torque)
{
	handed_powered = state!=NULL;
	if (state) memcpy(handed, state, sizeof(handed));
	handed_force = force!=NULL;
	handed_forces[0] = force ? *force : zero3d;
	handed_torque = torque!=NULL;
	handed_forces[1] = torque ? *torque : zero3d;
}
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

static real_vector3d direction(void)
{
	real_vector3d v;

	do
	{
		v.i = between(1.0f);
		v.j = between(1.0f);
		v.k = between(1.0f);
	} while (normalize3d(&v)==0.0f);
	return v;
}

static void print_vector(real_vector3d const *v)
{
	printf(" %.9g %.9g %.9g", v->i, v->j, v->k);
}

static void random_vehicle(int kind, int step)
{
	struct mass_point_datum *unused = NULL;
	int i, count;
	real_vector3d forward = direction(), up = direction();

	memset(&the_vehicle, 0, sizeof(the_vehicle));
	/* forward and up made square, now and then upside down or on end */
	if (step%11==0) up = step%22==0 ? up3d : zero3d;
	if (up.i==0.0f && up.j==0.0f && up.k==0.0f) { up.i = 0.0f; up.j = 0.0f; up.k = -1.0f; }
	point_from_line3d((real_point3d *)&up, &forward, -dot_product3d(&up, &forward), (real_point3d *)&up);
	if (normalize3d(&up)==0.0f) up = step%2 ? forward3d : up3d;
	if (step%13==0) forward = forward3d, up = up3d;
	the_vehicle.object.position.x = between(20.0f);
	the_vehicle.object.position.y = between(20.0f);
	the_vehicle.object.position.z = between(5.0f);
	the_vehicle.object.forward = forward;
	the_vehicle.object.up = up;
	the_vehicle.object.translational_velocity.i = between(0.4f);
	the_vehicle.object.translational_velocity.j = between(0.4f);
	the_vehicle.object.translational_velocity.k = between(0.2f);
	if (step%7==0) the_vehicle.object.translational_velocity = zero3d;
	the_vehicle.object.angular_velocity.i = between(0.05f);
	the_vehicle.object.angular_velocity.j = between(0.05f);
	the_vehicle.object.angular_velocity.k = between(0.05f);
	if (step%9==0) the_vehicle.object.angular_velocity = zero3d;
	the_vehicle.unit.seat_power[0] = step%5==0 ? 1.0f : between(0.5f)+0.5f;
	the_vehicle.unit.throttle.i = step%6==0 ? 0.0f : between(1.0f);
	the_vehicle.unit.throttle.j = step%8==0 ? 0.0f : between(1.0f);
	the_vehicle.unit.desired_facing_vector = direction();
	if (step%17==0) the_vehicle.unit.desired_facing_vector = up3d;
	if (step%19==0) the_vehicle.unit.desired_facing_vector.i = 0.0f, the_vehicle.unit.desired_facing_vector.j = 0.0f,
		the_vehicle.unit.desired_facing_vector.k = -1.0f;
	the_vehicle.vehicle.speed = step%4==0 ? 0.0f : between(0.4f);
	the_vehicle.vehicle.turn = step%3==0 ? 0.0f : between(0.6f);
	the_vehicle.vehicle.wheel = between(1.0f)+1.0f;
	the_vehicle.vehicle.left_tread = between(1.0f)+1.0f;
	the_vehicle.vehicle.right_tread = between(1.0f)+1.0f;
	the_vehicle.vehicle.hover = step%5==1 ? 0.0f : step%5==2 ? 1.0f : between(0.5f)+0.5f;
	the_vehicle.vehicle.thrust = between(0.5f)+0.5f;
	the_vehicle.vehicle.airborne_ticks = (byte)(next()>>24)%40;
	if (step%10==0) the_vehicle.vehicle.airborne_ticks = 255;
	the_vehicle.vehicle.on_ground_ticks = (byte)(next()>>24)%30;
	if (step%10==5) the_vehicle.vehicle.on_ground_ticks = 255;
	the_vehicle.vehicle.flags = step%3==1 ? FLAG(3) : 0;
	the_vehicle.vehicle.stop_time = 1+(short)(next()>>24)%30;

	the_definition.unknown2f8 = 0.1f+between(0.2f)+0.2f;
	the_definition.unknown2fc = step%10==3 ? 0.0f : 0.05f+between(0.1f)+0.1f;
	the_definition.unknown300 = 0.001f+between(0.005f)+0.005f;
	the_definition.unknown304 = 0.001f+between(0.005f)+0.005f;
	the_definition.unknown308 = between(1.0f)+1.0f;
	the_definition.unknown314 = 0.05f+between(0.2f)+0.2f;
	the_definition.unknown364 = between(0.3f);
	the_definition.wheel_circumference = 0.5f+between(2.0f)+2.0f;

	the_physics.radius = (kind==4 && step%2) ? 0.5f+between(1.0f)+1.0f : 0.0f;
	the_physics.mass = 100.0f+between(2000.0f)+2000.0f;
	the_physics.xx_moment = 10.0f+between(500.0f)+500.0f;
	the_physics.yy_moment = 10.0f+between(500.0f)+500.0f;
	the_physics.zz_moment = 10.0f+between(500.0f)+500.0f;

	/* the right number of powered mass points mostly, the wrong now and then */
	{
		int right = kind==2 ? 3 : 2;

		the_physics.powered_mass_points.count = step%12==4 ? right-1 : step%12==8 ? right+1 : right;
		if (the_physics.powered_mass_points.count>3) the_physics.powered_mass_points.count = 3;
	}
	count = 1+(int)(next()>>24)%8;
	the_physics.mass_points.count = count;
	the_physics.mass_points.address = the_mass_point_definitions;
	for (i = 0; i<count; i++)
	{
		int powered = (int)(next()>>24)%(the_physics.powered_mass_points.count+2)-1;

		the_mass_point_definitions[i].powered_mass_point_index = powered>=the_physics.powered_mass_points.count ?
			NONE : (short)powered;
	}
	driven_by_ai = step%4==3;
	water_depth_here = step%6==1 ? 1.0f : step%6==2 ? 0.49f : -1.0f;
}

static void print_case(char const *kind, struct mass_point_datum const *mass_points, real steering)
{
	int i;

	printf("%s", kind);
	print_vector((real_vector3d *)&the_vehicle.object.position);
	print_vector(&the_vehicle.object.forward);
	print_vector(&the_vehicle.object.up);
	print_vector(&the_vehicle.object.translational_velocity);
	print_vector(&the_vehicle.object.angular_velocity);
	printf(" %.9g %.9g %.9g %.9g %.9g %.9g %.9g %d %d %d %d", the_vehicle.vehicle.speed, the_vehicle.vehicle.turn,
		the_vehicle.vehicle.wheel, the_vehicle.vehicle.left_tread, the_vehicle.vehicle.right_tread,
		the_vehicle.vehicle.hover, the_vehicle.vehicle.thrust, the_vehicle.vehicle.airborne_ticks,
		the_vehicle.vehicle.on_ground_ticks, the_vehicle.vehicle.stop_time, TEST_FLAG(the_vehicle.vehicle.flags, 3));
	printf(" %.9g %.9g", the_vehicle.unit.throttle.i, the_vehicle.unit.throttle.j);
	print_vector(&the_vehicle.unit.desired_facing_vector);
	printf(" %.9g %d %.9g %.9g", the_vehicle.unit.seat_power[0], driven_by_ai, water_depth_here, steering);
	printf(" %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g", the_definition.unknown2f8, the_definition.unknown2fc,
		the_definition.unknown300, the_definition.unknown304, the_definition.unknown308, the_definition.unknown314,
		the_definition.unknown364, the_definition.wheel_circumference);
	printf(" %.9g %.9g %.9g %.9g %.9g %d %d", the_physics.radius, the_physics.mass, the_physics.xx_moment,
		the_physics.yy_moment, the_physics.zz_moment, the_physics.powered_mass_points.count,
		the_physics.mass_points.count);
	for (i = 0; i<the_physics.mass_points.count; i++)
		printf(" %d:%lu", the_mass_point_definitions[i].powered_mass_point_index, mass_points[i].flags);
}

static void print_result(boolean flipped)
{
	int i;

	printf(" ->");
	print_vector((real_vector3d *)&the_vehicle.object.position);
	print_vector(&the_vehicle.object.forward);
	print_vector(&the_vehicle.object.up);
	print_vector(&the_vehicle.object.translational_velocity);
	print_vector(&the_vehicle.object.angular_velocity);
	printf(" %.9g %.9g %.9g %.9g %.9g %d %d %d", the_vehicle.vehicle.wheel, the_vehicle.vehicle.left_tread,
		the_vehicle.vehicle.right_tread, the_vehicle.vehicle.hover, the_vehicle.vehicle.thrust,
		the_vehicle.vehicle.airborne_ticks, the_vehicle.vehicle.on_ground_ticks, the_vehicle.vehicle.stop_time);
	printf(" %d", handed_powered);
	for (i = 0; i<3; i++)
		printf(" %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g", handed[i].ground_friction_velocity,
			handed[i].water_friction_velocity, handed[i].water_lift_ratio, handed[i].antigrav_fraction,
			handed[i].rotation.v.i, handed[i].rotation.v.j, handed[i].rotation.v.k, handed[i].rotation.w);
	print_vector(&handed_forces[0]);
	print_vector(&handed_forces[1]);
	printf(" %d\n", flipped);
}

static char const *const kinds[] = {"jeep", "tank", "boat", "scout", "fighter", "airborne", "stop"};

int main(void)
{
	int kind, step;

	for (kind = 0; kind<7; kind++)
	{
		for (step = 0; step<(kind>=5 ? 200 : 400); step++)
		{
			struct mass_point_datum mass_points[8];
			struct powered_mass_point_datum state[32];
			real steering = step%3==0 ? 0.0f : between(3.14159274f);
			boolean flipped;
			int i;

			random_vehicle(kind, step);
			for (i = 0; i<8; i++)
			{
				unsigned int bits = next()>>28;

				mass_points[i].flags = (bits&1 ? FLAG(1) : 0)|(bits&2 ? FLAG(4) : 0);
			}
			/* the physics update's own state starts unturned and pushing with nothing */
			memset(state, 0, sizeof(state));
			for (i = 0; i<32; i++)
				state[i].rotation = identity_quaternion;
			handed_powered = 0;
			memset(handed, 0, sizeof(handed));
			handed_forces[0] = zero3d;
			handed_forces[1] = zero3d;
			print_case(kinds[kind], mass_points, steering);
			flipped = vehicle_is_flipped(0);
			switch (kind)
			{
			case 0: update_human_jeep_physics(0, mass_points, state); break;
			case 1: update_human_tank_physics(0, mass_points, state); break;
			case 2: update_human_boat_physics(0, mass_points, state); break;
			case 3: update_alien_scout_physics(0, steering, state, mass_points); break;
			case 4: update_alien_fighter_physics(0, state, mass_points); break;
			case 5: compute_airborne_ticks(0, mass_points, state); break;
			case 6: slowly_stop_vehicle(0); break;
			}
			print_result(flipped);
		}
	}
	return 0;
}
"""


def math_types():
    """real_math.h's vector, quaternion and matrix types."""
    header = read("source/math/real_math.h")
    start = header.index("union real_euler_angles2d")
    end = header.index("typedef struct real_matrix4x3 real_matrix4x3;")
    return header[start : header.index("\n", end)]


def next_tables(matrix_c):
    """matrix_math.c's tables of the next axis, which its rotations index."""
    start = matrix_c.index("static short data_0030790c")
    end = matrix_c.index("#define matrix3x3_next")
    return matrix_c[start : matrix_c.index("\n", end)]


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    real_math_h = read("source/math/real_math.h")
    real_math_c = read("source/math/real_math.c")
    matrix_c = read("source/math/matrix_math.c")
    vehicles_c = read("source/units/vehicles.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            math_types(),
            WORLD,
            *(inline(real_math_h, name) for name in INLINE_FUNCTIONS),
            *(function(real_math_c, name) for name in REAL_MATH_FUNCTIONS),
            next_tables(matrix_c),
            *(function(matrix_c, name) for name in MATRIX_FUNCTIONS),
            *(function(vehicles_c, name) for name in VEHICLE_FUNCTIONS),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="vehicle-physics-reference-") as work:
        source = Path(work) / "vehicle_physics_reference.c"
        executable = Path(work) / "vehicle_physics_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's vehicle controllers (tools/engine_vehicle_physics_reference.py).\n"
        "# <kind> <position> <forward> <up> <velocity> <angular velocity> <speed> <turn> <wheel> <treads>\n"
        "#   <hover> <thrust> <airborne ticks> <on ground ticks> <stop time> <brake> <throttle> <facing>\n"
        "#   <seat power> <ai> <water depth> <steering> <2f8> <2fc> <300> <304> <308> <314> <364>\n"
        "#   <wheel circumference> <radius> <mass> <xx> <yy> <zz> <powered count> <mass point count>\n"
        "#   <powered index:flags>...\n"
        "#   -> <position> <forward> <up> <velocity> <angular velocity> <wheel> <treads> <hover> <thrust>\n"
        "#   <airborne ticks> <on ground ticks> <stop time> <handed powered> 3 x <ground friction velocity>\n"
        "#   <water friction velocity> <water lift> <antigrav> <rotation> <force> <torque>\n"
        "#   <whether it lay flipped before>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_vehicle_physics_reference.py")
        print("the vehicle physics reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
