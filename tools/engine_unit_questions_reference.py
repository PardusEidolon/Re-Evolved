"""The game's questions of its units and the small changes it makes to
them: which unit aims for a rider, whether any unit is dangerous, whether
a biped is unsuspecting, where a unit's head and body are, a unit's
equipment deleted, the frames left of its animation, an animation's foot
frames, a biped's sight position, its limp body stopped, a vehicle's dot
product and whether a vehicle moves near a player, taken from the sources
and run over random worlds, to record what each does, as the native
engine's must.

    python3 tools/engine_unit_questions_reference.py          # writes the reference
    python3 tools/engine_unit_questions_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_get_aiming_unit_index`,
`any_unit_is_dangerous`, `unit_unsuspecting`, `unit_get_head_position`,
`unit_get_center_of_mass`, `unit_delete_current_equipment` and
`unit_get_animation_frames_remaining` (with `unit_get_seat_label`),
source/units/bipeds.c's `animation_left_foot_frame_index`,
`animation_right_foot_frame_index`, `biped_get_sight_position` and
`biped_stop_limp_body_physics`, and source/units/vehicles.c's
`vehicle_dot_product3d_test` and `vehicle_moving_near_any_player`, found
by name as tools/harness finds them, with the enums of units.h, bipeds.h,
objects.h, unit_definitions.h and biped_definitions.h and the vector
inlines of real_math.h, compiled for this machine against a small fake
world of six objects.

The world's markers, object iterator, deletion and local players are the
fake world's; a marker lookup and a deletion record what they were asked.

The trace is written to tools/traces/unit_questions_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_questions_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char boolean;
typedef unsigned char byte;
typedef unsigned short word;
typedef union real_point3d { struct { real x, y, z; }; } real_point3d;
typedef union real_vector3d { struct { real i, j, k; }; } real_vector3d;
#define TRUE 1
#define FALSE 0
#define NONE -1
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define match_assert(file, line, condition) ((void)0)
#define csstrcmp strcmp
#define UNSIGNED_LONG_MAX 0xffffffffu
#define MAXIMUM_LOCAL_PLAYERS 4
#define OBJECTS 6
"""

WORLD = r"""
static char const *base_seat_labels[NUMBER_OF_UNIT_BASE_SEATS] =
	{ "asleep", "alert", "stand", "crouch", "flee", "flaming" };
struct unit_seat
{
	unsigned long flags;
	char label[32];
};

struct unit_definition
{
	struct
	{
		unsigned long flags;
		struct { long count; struct unit_seat *address; } seats;
	} unit;
};

struct biped_definition
{
	struct
	{
		unsigned long flags;
		real standing_camera_height;
		real crouching_camera_height;
	} biped;
};

struct animation
{
	short frame_count;
	byte private_left_foot_frame_index;
	byte private_right_foot_frame_index;
};

struct animation_graph
{
	struct { long count; struct animation *address; } animations;
};

struct object_datum
{
	long definition_index;
	struct
	{
		short type;
		unsigned long flags;
		long parent_object_index;
		real_point3d bounding_sphere_center;
		real_vector3d translational_velocity;
		struct
		{
			long animation_graph_index;
			struct { short index; short frame_index; } state;
		} animation;
	} object;
	struct
	{
		unsigned long flags;
		real_vector3d looking_vector;
		char grenade_throw_state;
		short parent_seat_index;
		long equipment_object_index;
		struct
		{
			short state;
			word flags;
			char base_seat_index;
		} animation;
	} unit;
	struct
	{
		unsigned long flags;
		real crouch;
	} biped;
};

#define unit_datum object_datum
#define biped_datum object_datum
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&(block)->address[index])

static struct object_datum objects[OBJECTS];
static boolean exists[OBJECTS];
static struct unit_seat seats[OBJECTS][4];
static struct unit_definition definitions[OBJECTS];
static struct biped_definition biped_definitions[OBJECTS];
static struct animation animations[8];
static struct animation_graph graph = { { 8, animations } };

static struct object_datum *unit_get(long index) { return &objects[index]; }
static struct object_datum *biped_get(long index) { return &objects[index]; }
static struct object_datum *unit_try_and_get(long index)
{
	return index == NONE || !exists[index] || objects[index].object.type > 1 ? NULL : &objects[index];
}
static struct unit_definition *unit_definition_get(long index) { return &definitions[index]; }
static struct biped_definition *biped_definition_get(long index) { return &biped_definitions[index]; }
static struct animation_graph *animation_graph_definition_get(long index) { return &graph; }

/* the origin of an object is its bounding sphere's centre here */
static void object_get_origin(long index, real_point3d *origin) { *origin = objects[index].object.bounding_sphere_center; }
static void object_get_bounding_sphere(long index, real_point3d *center, real *radius)
{
	*center = objects[index].object.bounding_sphere_center;
	*radius = 1.f;
}

/* what the marker lookup and the deletion were asked */
struct object_marker { struct { real_point3d position; } matrix; };
static char const *marker_asked;
static long deleted;

static short object_get_marker_by_name(long index, char const *name, struct object_marker *marker, short count)
{
	marker_asked = name;
	marker->matrix.position.x = (real)index;
	marker->matrix.position.y = (real)strlen(name);
	marker->matrix.position.z = 0.5f;
	return 1;
}
static void object_delete(long index) { deleted = index; }

/* an iterator over the existing objects of a mask's types, in order */
struct object_iterator { unsigned long mask; long index; };
static void object_iterator_new(struct object_iterator *iterator, unsigned long mask, long flags)
{
	iterator->mask = mask;
	iterator->index = NONE;
}
static void *object_iterator_next(struct object_iterator *iterator)
{
	while (++iterator->index < OBJECTS)
	{
		struct object_datum *object = &objects[iterator->index];
		if (exists[iterator->index] && TEST_FLAG(iterator->mask, object->object.type))
			return object;
	}
	return NULL;
}

/* the local players, each with the unit it drives (or none) */
static long local_player_count;
static long local_units[MAXIMUM_LOCAL_PLAYERS];
static boolean local_has_player[MAXIMUM_LOCAL_PLAYERS];
struct player_datum { long unit_index; };
static struct player_datum players[MAXIMUM_LOCAL_PLAYERS];

static short local_player_get_next(short index)
{
	return index + 1 < local_player_count ? index + 1 : NONE;
}
static long local_player_get_player_index(short index) { return local_has_player[index] ? index : NONE; }
static struct player_datum *player_get(long index)
{
	players[index].unit_index = local_units[index];
	return &players[index];
}
"""

DRIVER = r"""
static unsigned long seed = 0x9e37;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static real random_real(real low, real high)
{
	return low + (high - low) * (real)next_random(1 << 16) / 65536.f;
}

static unsigned long random_flags(void)
{
	return ((unsigned long)next_random(1 << 16) << 16) | (unsigned long)next_random(1 << 16);
}

static void random_point(real_point3d *point, real range)
{
	point->x = random_real(-range, range);
	point->y = random_real(-range, range);
	point->z = random_real(-range, range);
}

static void random_vector(real_vector3d *vector, real range)
{
	vector->i = random_real(-range, range);
	vector->j = random_real(-range, range);
	vector->k = random_real(-range, range);
}

/* a real as its bits, in hex */
static unsigned bits(real value)
{
	unsigned result;
	memcpy(&result, &value, sizeof(result));
	return result;
}

static void print_point(real_point3d const *point)
{
	printf(" %x %x %x", bits(point->x), bits(point->y), bits(point->z));
}

static void print_vector(real_vector3d const *vector)
{
	printf(" %x %x %x", bits(vector->i), bits(vector->j), bits(vector->k));
}

static char const *const labels[] = { "asleep", "alert", "stand", "gunner", "driver", "" };

/* a random world: each object exists or not, a biped (0), a vehicle (1)
   or another type (2), riding another in a seat or not, with its own
   definition's flags, seats and camera heights */
static void random_world(void)
{
	long index;
	long seat;
	for (index = 0; index < OBJECTS; index++)
	{
		struct object_datum *object = &objects[index];
		exists[index] = next_random(5) != 0;
		object->definition_index = index;
		object->object.type = (short)next_random(3);
		object->object.flags = random_flags();
		object->object.parent_object_index = next_random(2) ? NONE : next_random(OBJECTS);
		random_point(&object->object.bounding_sphere_center, 12.f);
		random_vector(&object->object.translational_velocity, next_random(2) ? 0.05f : 1.f);
		object->object.animation.state.index = (short)next_random(8);
		object->object.animation.state.frame_index = (short)next_random(40);
		object->unit.flags = random_flags();
		random_vector(&object->unit.looking_vector, 1.f);
		object->unit.grenade_throw_state = (char)next_random(4);
		object->unit.parent_seat_index = next_random(3) ? (short)next_random(4) : NONE;
		object->unit.equipment_object_index = next_random(2) ? NONE : next_random(100);
		object->unit.animation.state = (short)next_random(44);
		object->unit.animation.flags = (word)next_random(16);
		object->unit.animation.base_seat_index = (char)next_random(NUMBER_OF_UNIT_BASE_SEATS);
		object->biped.flags = random_flags();
		object->biped.crouch = random_real(0.f, 1.f);
		definitions[index].unit.flags = random_flags();
		definitions[index].unit.seats.count = 4;
		definitions[index].unit.seats.address = seats[index];
		for (seat = 0; seat < 4; seat++)
		{
			seats[index][seat].flags = (unsigned long)next_random(16);
			strcpy(seats[index][seat].label, labels[next_random(6)]);
		}
		biped_definitions[index].biped.flags = random_flags();
		biped_definitions[index].biped.standing_camera_height = random_real(0.f, 1.f);
		biped_definitions[index].biped.crouching_camera_height = random_real(0.f, 1.f);
	}
	for (index = 0; index < 8; index++)
	{
		animations[index].frame_count = (short)next_random(60);
		animations[index].private_left_foot_frame_index = (byte)next_random(256);
		animations[index].private_right_foot_frame_index = (byte)next_random(256);
	}
}

/* each object as the trace writes it */
static void print_world(void)
{
	long index;
	long seat;
	for (index = 0; index < OBJECTS; index++)
	{
		struct object_datum *object = &objects[index];
		printf(" | %d %d %u %d", exists[index], object->object.type, object->object.flags,
			object->object.parent_object_index);
		print_point(&object->object.bounding_sphere_center);
		print_vector(&object->object.translational_velocity);
		printf(" %d %d %u", object->object.animation.state.index, object->object.animation.state.frame_index,
			object->unit.flags);
		print_vector(&object->unit.looking_vector);
		printf(" %d %d %d %d %u %d %u %x %u", object->unit.grenade_throw_state, object->unit.parent_seat_index,
			object->unit.equipment_object_index, object->unit.animation.state, object->unit.animation.flags,
			object->unit.animation.base_seat_index, object->biped.flags, bits(object->biped.crouch),
			definitions[index].unit.flags);
		for (seat = 0; seat < 4; seat++)
			printf(" %u:'%s'", seats[index][seat].flags, seats[index][seat].label);
		printf(" %u %x %x", biped_definitions[index].biped.flags,
			bits(biped_definitions[index].biped.standing_camera_height),
			bits(biped_definitions[index].biped.crouching_camera_height));
	}
	printf(" | animations");
	for (index = 0; index < 8; index++)
		printf(" %d %u %u", animations[index].frame_count, animations[index].private_left_foot_frame_index,
			animations[index].private_right_foot_frame_index);
}

int main(void)
{
	long run;
	long index;

	for (run = 0; run < 600; run++)
	{
		long unit;
		random_world();
		printf("world");
		print_world();
		printf("\n");

		unit = next_random(OBJECTS + 1) - 1;
		/* the questions that take a unit ask only of units */
		if (unit != NONE && (!exists[unit] || objects[unit].object.type > 1))
			unit = NONE;
		printf("aiming %d -> %d\n", unit, unit_get_aiming_unit_index(unit));
		printf("dangerous -> %d\n", any_unit_is_dangerous());
		{
			real_point3d point;
			random_point(&point, 12.f);
			printf("unsuspecting %d", unit);
			print_point(&point);
			printf(" -> %d\n", unit_unsuspecting(unit, &point));
		}
		if (unit != NONE)
		{
			real_point3d head;
			real_point3d body;
			short state;
			short frames;
			unit_get_head_position(unit, &head);
			printf("head %d -> %s", unit, marker_asked);
			print_point(&head);
			printf("\n");
			unit_get_center_of_mass(unit, &body);
			printf("body %d -> %s", unit, marker_asked);
			print_point(&body);
			printf("\n");
			frames = unit_get_animation_frames_remaining(unit, &state);
			printf("frames_remaining %d -> %d %d\n", unit, frames, state);
			deleted = NONE;
			printf("delete_equipment %d %d ->", unit, objects[unit].unit.equipment_object_index);
			unit_delete_current_equipment(unit);
			printf(" %d %d\n", deleted, objects[unit].unit.equipment_object_index);
			if (objects[unit].object.type == 0)
			{
				short mode = (short)next_random(4);
				real_point3d estimated;
				real_vector3d facing;
				real_vector3d offset;
				real_point3d sight;
				random_point(&estimated, 12.f);
				random_vector(&facing, 1.f);
				random_vector(&offset, 0.5f);
				biped_get_sight_position(unit, mode, &estimated, &facing, &offset, &sight);
				printf("sight %d %d", unit, mode);
				print_point(&estimated);
				print_vector(&facing);
				print_vector(&offset);
				printf(" ->");
				print_point(&sight);
				printf("\n");
				printf("stop_limp %d ->", unit);
				biped_stop_limp_body_physics(unit);
				printf(" %u %u\n", objects[unit].object.flags, objects[unit].biped.flags);
			}
		}
		for (index = 0; index < 8; index++)
			printf("feet %d -> %d %d\n", index, animation_left_foot_frame_index(&animations[index]),
				animation_right_foot_frame_index(&animations[index]));
		{
			real_vector3d a;
			real_vector3d b;
			random_vector(&a, 4.f);
			random_vector(&b, 4.f);
			printf("dot");
			print_vector(&a);
			print_vector(&b);
			printf(" -> %x\n", bits(vehicle_dot_product3d_test(&a, &b)));
		}
		local_player_count = next_random(MAXIMUM_LOCAL_PLAYERS + 1);
		printf("moving_near %d |", local_player_count);
		for (index = 0; index < local_player_count; index++)
		{
			local_has_player[index] = next_random(4) != 0;
			local_units[index] = next_random(3) ? next_random(OBJECTS) : NONE;
			printf(" %d %d", local_has_player[index], local_units[index]);
		}
		printf(" -> %d\n", vehicle_moving_near_any_player());
	}
	return 0;
}
"""

MATH_INLINES = ["vector_from_points3d", "dot_product3d", "magnitude_squared3d", "distance_squared3d"]

# each function, and the file it is defined in
FUNCTIONS = [
    ("source/units/units.c", "unit_get_seat_label"),
    ("source/units/units.c", "unit_get_aiming_unit_index"),
    ("source/units/units.c", "any_unit_is_dangerous"),
    ("source/units/units.c", "unit_unsuspecting"),
    ("source/units/units.c", "unit_get_head_position"),
    ("source/units/units.c", "unit_get_center_of_mass"),
    ("source/units/units.c", "unit_delete_current_equipment"),
    ("source/units/units.c", "unit_get_animation_frames_remaining"),
    ("source/units/bipeds.c", "animation_left_foot_frame_index"),
    ("source/units/bipeds.c", "animation_right_foot_frame_index"),
    ("source/units/bipeds.c", "biped_get_sight_position"),
    ("source/units/bipeds.c", "biped_stop_limp_body_physics"),
    ("source/units/vehicles.c", "vehicle_dot_product3d_test"),
    ("source/units/vehicles.c", "vehicle_moving_near_any_player"),
]

# each enum, by a member, and the header it is in
ENUMS = [
    ("source/objects/objects.h", "_object_dead_bit"),
    ("source/objects/objects.h", "_object_do_not_recompute_node_matrices_bit"),
    ("source/units/units.h", "_unit_state_idle"),
    ("source/units/units.h", "_unit_grenade_throw_idle"),
    ("source/units/units.h", "_unit_animation_ignore_translation_bit"),
    ("source/units/units.h", "_unit_base_seat_asleep"),
    ("source/units/units.h", "_unit_estimate_none"),
    ("source/units/bipeds.h", "_biped_limp_body_physics_active_bit"),
    ("source/units/unit_definitions.h", "_unit_never_unsuspecting_bit"),
    ("source/units/unit_definitions.h", "_unit_seat_invisible_bit"),
    ("source/units/biped_definitions.h", "_biped_uses_limp_body_physics_bit"),
    ("source/objects/object_types.h", "_object_type_biped"),
]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    files = {}

    def text(path):
        if path not in files:
            files[path] = read(path)
        return files[path]

    enums = []
    for path, member in ENUMS:
        enum = enum_with(text(path), member)
        if enum not in enums:
            enums.append(enum)
    math_h = text("source/math/real_math.h")
    inlines = [inline(math_h, name) for name in MATH_INLINES]
    functions = [function(text(path), name) for path, name in FUNCTIONS]
    return "\n".join([FAKE_WORLD, *enums, *inlines, WORLD, *functions, DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-questions-reference-") as work:
        source = Path(work) / "unit_questions_reference.c"
        executable = Path(work) / "unit_questions_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's questions of its units (tools/engine_unit_questions_reference.py).\n"
        "# Reals are their bits in hex.\n"
        "# world | per object: <exists> <type> <object flags> <parent> <centre xyz> <velocity ijk>\n"
        "#   <animation index> <frame> <unit flags> <looking ijk> <grenade throw> <parent seat>\n"
        "#   <equipment> <state> <animation flags> <base seat> <biped flags> <crouch>\n"
        "#   <definition flags> <seat flags>:'<label>' x4 <biped definition flags> <standing> <crouching>\n"
        "#   | animations <frame count> <left foot> <right foot> x8\n"
        "# aiming <unit> -> <aiming unit>\n"
        "# dangerous -> <any unit is dangerous>\n"
        "# unsuspecting <unit> <point xyz> -> <unsuspecting>\n"
        "# head|body <unit> -> <marker> <position xyz>\n"
        "# frames_remaining <unit> -> <frames> <state>\n"
        "# delete_equipment <unit> <equipment> -> <deleted> <equipment after>\n"
        "# sight <biped> <mode> <estimate xyz> <facing ijk> <gun offset ijk> -> <sight xyz>\n"
        "# stop_limp <biped> -> <object flags> <biped flags>\n"
        "# feet <animation> -> <left> <right>\n"
        "# dot <a ijk> <b ijk> -> <dot>\n"
        "# moving_near <local players> | <has a player> <unit> ... -> <a vehicle moves near>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_questions_reference.py")
        print("the unit questions reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
