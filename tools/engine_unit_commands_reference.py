"""The game's small script commands on a unit and a vehicle's seats, taken
from the sources and run over random units, to record what they answer, as
the native engine's must.

    python3 tools/engine_unit_commands_reference.py          # writes the reference
    python3 tools/engine_unit_commands_reference.py --check  # fails if it is stale

The functions are source/units/unit_scripting_commands.c's
`unit_scripting_get_grenade_count`, `unit_scripting_has_weapon_readied` and
`unit_scripting_impervious`,
and source/units/units.c's `unit_get_custom_animation_time`,
`unit_stop_custom_animation` and `unit_scripting_vehicle_test_seat_list`,
found by name as tools/harness finds them and compiled for this machine
against a small fake world of unit datums. Units are drawn at random: with
grenades of each kind, weapons in some of their four slots and a current
weapon that may be none, a custom animation playing or not at any frame of
it, and vehicles whose seats are labelled from a few names, with riders in
some of them and object lists holding some of the riders.

The trace is written to tools/traces/unit_commands_reference.txt, which
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
from harness import function, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "unit_commands_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>
#include <strings.h>

/* the game's 32-bit long */
#define long int
#define __inline static inline

typedef unsigned char boolean;
typedef unsigned char byte;
#define TRUE 1
#define FALSE 0
#define NONE -1
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define NUMBEROF(array) (sizeof(array) / sizeof((array)[0]))
#define _stricmp strcasecmp
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&(block)->elements[index])

enum
{
	_object_type_biped = 0,
	_object_type_vehicle,
	_object_type_scenery = 6,
};
#define _object_mask_unit 3u

enum
{
	_unit_state_idle = 0,
	_unit_state_user_animation = 7,
};

enum { _unit_impervious_bit = 5 };
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))

#define MAXIMUM_WEAPONS_PER_UNIT 4
#define NUMBER_OF_UNIT_GRENADE_TYPES 2
#define MAXIMUM_SEATS 6

struct unit_datum
{
	long definition_index;
	struct
	{
		short type;
		long parent_object_index;
		struct
		{
			long animation_graph_index;
			struct { short index; short frame_index; } state;
		} animation;
	} object;
	struct
	{
		struct { short state; } animation;
		unsigned long flags;
		short parent_seat_index;
		short current_weapon_index;
		long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT];
		char grenade_counts[NUMBER_OF_UNIT_GRENADE_TYPES];
	} unit;
};

struct weapon_datum { long definition_index; };

struct animation { short frame_count; };
struct animation_graph { struct { long count; struct animation *elements; } animations; };

struct unit_seat { char label[32]; };
struct unit_definition { struct { struct { long count; struct unit_seat *elements; } seats; } unit; };

struct object_iterator
{
	unsigned long type_flags;
	byte flags;
	short absolute_index;
	long index;
	unsigned long signature;
};

#define MAXIMUM_OBJECTS 10
static struct unit_datum objects[MAXIMUM_OBJECTS];
static struct weapon_datum weapons[MAXIMUM_OBJECTS];
static struct animation animations[4];
static struct animation_graph graph = { { 4, animations } };
static struct unit_seat seats[MAXIMUM_SEATS];
static struct unit_definition vehicle_definition = { { { 0, seats } } };

static struct unit_datum *unit_get(long unit_index) { return &objects[unit_index]; }
static struct unit_datum *unit_try_and_get(long unit_index)
{
	return unit_index >= 0 && unit_index < MAXIMUM_OBJECTS && objects[unit_index].object.type <= _object_type_vehicle
		? &objects[unit_index] : NULL;
}
static struct weapon_datum *weapon_get(long weapon_index) { return &weapons[weapon_index]; }
static long unit_inventory_get_weapon(long unit_index, short weapon_index)
{
	return weapon_index >= 0 && weapon_index < MAXIMUM_WEAPONS_PER_UNIT
		? objects[unit_index].unit.weapon_object_indices[weapon_index] : NONE;
}
static struct animation_graph *animation_graph_definition_get(long index) { return &graph; }
static struct unit_definition *unit_definition_get(long index) { return &vehicle_definition; }
static void unit_animation_set_state(long unit_index, short state) { objects[unit_index].unit.animation.state = state; }
static void network_coop_note_unit_animation(long unit_index, long graph, long animation, boolean interpolate) {}

static void object_iterator_new(struct object_iterator *iterator, unsigned long type_flags, byte flags)
{
	iterator->type_flags = type_flags;
	iterator->index = NONE;
}
static void *object_iterator_next(struct object_iterator *iterator)
{
	long index;
	for (index = iterator->index + 1; index < MAXIMUM_OBJECTS; index++)
	{
		if (iterator->type_flags & (1u << objects[index].object.type))
		{
			iterator->index = index;
			return &objects[index];
		}
	}
	iterator->index = NONE;
	return NULL;
}

/* an object list: the objects in it, in order */
static long list[MAXIMUM_OBJECTS];
static long list_count;
static long object_list_get_first(long object_list_index, long *reference_index)
{
	*reference_index = 0;
	return list_count ? list[0] : NONE;
}
static long object_list_get_next(long object_list_index, long *reference_index)
{
	*reference_index += 1;
	return *reference_index < list_count ? list[*reference_index] : NONE;
}
"""

DRIVER = r"""
static unsigned long seed = 0x5ea7;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static char const *const names[] = { "driver", "gunner", "passenger", "P-riderLF", "Gunner" };

int main(void)
{
	long run, index;

	for (run = 0; run < 2000; run++)
	{
		struct unit_datum *unit = &objects[0];
		long unit_index = next_random(8) == 0 ? NONE : 0;
		memset(unit, 0, sizeof *unit);
		unit->object.type = next_random(6) == 0 ? _object_type_scenery : (short)next_random(2);
		printf("grenades %d %d", unit_index, unit->object.type <= _object_type_vehicle);
		for (index = 0; index < NUMBER_OF_UNIT_GRENADE_TYPES; index++)
		{
			unit->unit.grenade_counts[index] = (char)(next_random(5) == 0 ? next_random(128) : next_random(5));
			printf(" %d", unit->unit.grenade_counts[index]);
		}
		printf(" %d\n", unit_scripting_get_grenade_count(unit_index));
	}
	for (run = 0; run < 2000; run++)
	{
		struct unit_datum *unit = &objects[0];
		long asked = next_random(8) == 0 ? NONE : next_random(4);
		long unit_index = next_random(10) == 0 ? NONE : 0;
		memset(unit, 0, sizeof *unit);
		unit->unit.current_weapon_index = (short)(next_random(5) - 1);
		printf("readied %d %d", unit_index, unit->unit.current_weapon_index);
		for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		{
			long weapon = 1 + index;
			boolean held = next_random(3) != 0;
			unit->unit.weapon_object_indices[index] = held ? weapon : NONE;
			weapons[weapon].definition_index = next_random(4);
			printf(" %d", held ? weapons[weapon].definition_index : NONE);
		}
		printf(" %d %d\n", asked, unit_scripting_has_weapon_readied(unit_index, asked));
	}
	for (run = 0; run < 2000; run++)
	{
		struct unit_datum *unit = &objects[0];
		short animation = (short)next_random(4);
		long unit_index = next_random(10) == 0 ? NONE : 0;
		boolean stop = next_random(4) == 0;
		short time;
		memset(unit, 0, sizeof *unit);
		animations[animation].frame_count = (short)(1 + next_random(120));
		unit->object.animation.state.index = animation;
		unit->object.animation.state.frame_index = (short)next_random(animations[animation].frame_count);
		unit->unit.animation.state = next_random(3) == 0 ? _unit_state_idle : _unit_state_user_animation;
		printf("time %d %d %d %d %d", unit_index, unit->unit.animation.state == _unit_state_user_animation,
			animations[animation].frame_count, unit->object.animation.state.frame_index, stop);
		if (stop)
			unit_stop_custom_animation(unit_index);
		time = unit_get_custom_animation_time(unit_index);
		printf(" %d %d\n", unit->unit.animation.state == _unit_state_user_animation, time);
	}
	for (run = 0; run < 3000; run++)
	{
		long vehicle_index = next_random(10) == 0 ? NONE : 0;
		char const *asked = names[next_random(NUMBEROF(names))];
		boolean result;

		memset(objects, 0, sizeof objects);
		objects[0].object.type = _object_type_vehicle;
		objects[0].object.parent_object_index = NONE;
		vehicle_definition.unit.seats.count = 1 + next_random(MAXIMUM_SEATS);
		printf("seats %d %s", vehicle_index, asked);
		for (index = 0; index < vehicle_definition.unit.seats.count; index++)
		{
			strcpy(seats[index].label, names[next_random(NUMBEROF(names))]);
			printf(" %s", seats[index].label);
		}
		printf(" |");
		for (index = 1; index < MAXIMUM_OBJECTS; index++)
		{
			struct unit_datum *unit = &objects[index];
			unit->object.type = next_random(6) == 0 ? _object_type_scenery : (short)next_random(2);
			unit->object.parent_object_index = next_random(2) == 0 ? 0 : NONE;
			unit->unit.parent_seat_index = (short)next_random(vehicle_definition.unit.seats.count);
			printf(" %d:%d:%d", unit->object.type <= _object_type_vehicle,
				unit->object.parent_object_index, unit->unit.parent_seat_index);
		}
		printf(" |");
		list_count = 0;
		for (index = 1; index < MAXIMUM_OBJECTS; index++)
		{
			if (next_random(3) == 0)
			{
				list[list_count++] = index;
				printf(" %d", index);
			}
		}
		result = unit_scripting_vehicle_test_seat_list(vehicle_index, asked, 0);
		printf(" -> %d\n", result);
	}
	for (run = 0; run < 1000; run++)
	{
		boolean on = next_random(2);
		memset(objects, 0, sizeof objects);
		printf("impervious %d", on);
		for (index = 0; index < MAXIMUM_OBJECTS; index++)
		{
			objects[index].object.type = next_random(5) == 0 ? _object_type_scenery : (short)next_random(2);
			SET_FLAG(objects[index].unit.flags, _unit_impervious_bit, next_random(2));
			printf(" %d:%d", objects[index].object.type <= _object_type_vehicle,
				TEST_FLAG(objects[index].unit.flags, _unit_impervious_bit));
		}
		printf(" |");
		list_count = 0;
		for (index = 0; index < MAXIMUM_OBJECTS; index++)
		{
			if (next_random(2) == 0)
			{
				list[list_count++] = index;
				printf(" %d", index);
			}
		}
		unit_scripting_impervious(0, on);
		printf(" ->");
		for (index = 0; index < MAXIMUM_OBJECTS; index++)
			printf(" %d", TEST_FLAG(objects[index].unit.flags, _unit_impervious_bit));
		printf("\n");
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    commands = read("source/units/unit_scripting_commands.c")
    units = read("source/units/units.c")
    return "\n".join(
        [
            FAKE_WORLD,
            function(commands, "unit_scripting_get_grenade_count"),
            function(commands, "unit_scripting_has_weapon_readied"),
            function(commands, "unit_scripting_impervious"),
            function(units, "unit_get_custom_animation_time"),
            function(units, "unit_stop_custom_animation"),
            function(units, "unit_scripting_vehicle_test_seat_list"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-commands-reference-") as work:
        source = Path(work) / "unit_commands_reference.c"
        executable = Path(work) / "unit_commands_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    if "assert" in output:
        raise SystemExit("the game's code asserted")
    return (
        "# The game's small unit commands (tools/engine_unit_commands_reference.py).\n"
        "# grenades <unit> <is a unit> <count of each kind>... <answer>\n"
        "# readied <unit> <current weapon> <definition in each slot, or -1>... <asked> <answer>\n"
        "# time <unit> <playing> <frame count> <frame> <stopped> <playing after> <answer>\n"
        "# impervious <on> <is a unit:impervious>... | <list>... -> <impervious after>...\n"
        "# seats <vehicle> <asked> <seat labels>... | <is a unit:parent:seat>... | <list>... -> <answer>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_commands_reference.py")
        print("the unit commands reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
