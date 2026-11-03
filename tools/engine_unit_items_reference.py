"""The game's unit picking up, dropping and deleting its weapons, grenades
and equipment, taken from the sources and run over random units, to record
what it calls on the world's objects and what it leaves in its inventory,
as the native engine's must.

    python3 tools/engine_unit_items_reference.py          # writes the reference
    python3 tools/engine_unit_items_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_delete_all_weapons`,
`unit_add_grenade_to_inventory`, `unit_add_equipment_to_inventory`,
`unit_drop_current_equipment`, `unit_can_use_weapon`,
`unit_drop_inventory_weapons`, `unit_drop_grenades`,
`unit_drop_current_weapon`, `unit_add_weapon_to_inventory` and
`unit_add_initial_weapons`, with the helpers they call
(`unit_weapon_next_index`, `unit_first_free_weapon_index`,
`unit_has_weapon_definition_index`, `unit_inventory_get_weapon` and
`unit_get_weapon`), found by name as tools/harness finds them and compiled
for this machine against a fake world of sixteen objects: the unit (0), and
weapons and equipment each connected to the map or not, invisible or not,
riding a parent or not, able to be fired, put away, used by the unit's seat
and allowed by the game engine or not.

What the functions ask of the rest of the game is answered from that world,
and what they do to it (delete, disconnect, show or hide, hand to the
inventory, drop, create, message the first person weapon, set the player's
desired weapon) is written down in order. `unit_drop_item` is one such
call: its throw (a random cone, the bsp) is the objects' and is not traced
here.

Asserts are compiled out, as in the released game.

The trace is written to tools/traces/unit_items_reference.txt, which
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
from harness import enum_with, function, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "unit_items_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
typedef unsigned short word;
#define TRUE 1
#define FALSE 0
#define NONE -1
#define match_assert(file, line, condition) ((void)0)
#define match_vassert(file, line, condition, message) ((void)0)
#define VALID_INDEX(index, count) ((index) >= 0 && (index) < (count))
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))

#define MAXIMUM_WEAPONS_PER_UNIT 4
#define NUMBER_OF_UNIT_GRENADE_TYPES 2
"""

WORLD = r"""
enum { _object_type_biped = 0, _object_type_weapon = 2, _object_type_equipment = 3 };

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) \
	((index) >= 0 && (index) < (block)->count ? (type *)(block)->address + (index) : (type *)NULL)

struct object_datum
{
	long definition_index;
	struct { short type; unsigned long flags; long parent_object_index; } object;
	/* the world's answers about it */
	boolean can_be_fired, puts_away, seat_allows, engine_allows, engine_picks_up;
	short grenade_type;
};
#define weapon_datum object_datum
#define item_datum object_datum

struct unit_datum
{
	long definition_index;
	struct
	{
		short current_weapon_index;
		short desired_weapon_index;
		long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT];
		long weapon_last_used_at_game_time[MAXIMUM_WEAPONS_PER_UNIT];
		char grenade_counts[NUMBER_OF_UNIT_GRENADE_TYPES];
		long equipment_object_index;
		unsigned long flags;
		unsigned long control_flags;
	} unit;
};

struct unit_initial_weapon { struct { long index; } weapon; };
struct unit_definition { struct { struct tag_block initial_weapons; } unit; };
struct equipment_definition { struct { short powerup_type; short grenade_type; } equipment; };
struct game_globals_grenade { short maximum_count; struct { long index; } item; };
struct game_globals { struct tag_block grenades; };
struct player_datum { long local_player_index; };
struct object_placement_data { long definition_index; };

#define MAXIMUM_OBJECTS 16
#define FIRST_NEW_OBJECT 12
static struct object_datum objects[MAXIMUM_OBJECTS];
static struct unit_datum the_unit;
static struct unit_initial_weapon initial_weapons[4];
static struct unit_definition the_unit_definition;
static struct equipment_definition the_equipment_definition;
static struct game_globals_grenade grenades[NUMBER_OF_UNIT_GRENADE_TYPES];
static struct game_globals the_globals;
static struct player_datum the_player;
static long player_index;
static boolean engine_running;
static long next_new;
static unsigned long new_failures;
static long news;

static struct unit_datum *unit_get(long unit_index) { return &the_unit; }
static struct unit_definition *unit_definition_get(long index) { return &the_unit_definition; }
static struct object_datum *weapon_get(long index) { return &objects[index]; }
static struct object_datum *equipment_get(long index) { return &objects[index]; }
static struct object_datum *item_get(long index) { return &objects[index]; }
static struct object_datum *object_get(long index) { return &objects[index]; }
/* an equipment's definition is its own: the grenade type drawn for it */
static struct equipment_definition *equipment_definition_get(long index)
{
	the_equipment_definition.equipment.powerup_type = 6;
	the_equipment_definition.equipment.grenade_type = objects[index - 100].grenade_type;
	return &the_equipment_definition;
}
static struct game_globals *scenario_get_game_globals(void) { return &the_globals; }
static long player_index_from_unit_index(long unit_index) { return player_index; }
static struct player_datum *player_get(long index) { return &the_player; }
static boolean game_engine_running(void) { return engine_running; }
static boolean network_objects_creating_host_object(void) { return FALSE; }
static char const *tag_get_name(long index) { return "tag"; }
static void error(int priority, char const *format, ...) {}
enum { _error_silent = 0 };

static char const *unit_get_seat_label(long unit_index) { return "stand"; }
static long last_weapon_asked;
/* the seat is asked about the weapon whose label it was given */
static char const *weapon_get_label(long weapon_index) { last_weapon_asked = weapon_index; return "rifle"; }
static boolean unit_set_or_test_seat_and_weapon_label(long unit_index, char const *seat, char const *weapon, boolean set);
static boolean game_engine_allow_pick_up(long unit_index, long weapon_index) { return objects[weapon_index].engine_allows; }
static boolean game_engine_picking_up(long unit_index, long weapon_index) { return objects[weapon_index].engine_picks_up; }
static boolean weapon_must_be_readied(long weapon_index) { return FALSE; }
static boolean weapon_can_be_fired(long weapon_index) { return objects[weapon_index].can_be_fired; }

static void object_delete(long index) { printf(" delete %d", index); }
static void object_disconnect_from_map(long index)
{
	printf(" disconnect %d", index);
	SET_FLAG(objects[index].object.flags, _object_connected_to_map_bit, FALSE);
}
static void object_set_visibility(long index, boolean visible) { printf(" visible %d %d", index, visible); }
static void equipment_handle_pickup(long index) { printf(" pickup %d", index); }
static void item_in_unit_inventory(long index, long unit_index) { printf(" carried %d %d", index, unit_index); }
static void unit_drop_item(long unit_index, long index) { printf(" drop %d", index); }
static boolean weapon_put_away(long index, boolean immediate)
{
	printf(" put_away %d %d", index, immediate);
	return objects[index].puts_away;
}
static void first_person_weapon_message_from_unit(long unit_index, long message) { printf(" message %d", message); }
static void player_control_set_desired_weapon(long unit_index, short slot) { printf(" player_desired %d", slot); }
static void object_placement_data_new(struct object_placement_data *data, long definition_index, long owner)
{
	data->definition_index = definition_index;
}
/* the next of the drawn objects, at the place the game's would be, unless
this creation is one drawn to fail */
static long object_new(struct object_placement_data *data)
{
	long index = NONE;
	if (!TEST_FLAG(new_failures, news) && next_new < MAXIMUM_OBJECTS)
	{
		index = next_new++;
		objects[index].definition_index = data->definition_index;
	}
	news++;
	printf(" new %d %d", data->definition_index, index);
	return index;
}

static short unit_weapon_next_index(long unit_index, short current_index, short delta);
static short unit_first_free_weapon_index(long unit_index);
static long unit_get_weapon(struct unit_datum *unit, short index);
boolean unit_add_weapon_to_inventory(long unit_index, long weapon_index, long is_starting_weapon);
boolean unit_can_use_weapon(long unit_index, long weapon_index);
#define unit_get_current_weapon_index(unit_index) unit_inventory_get_weapon((unit_index), unit_get((unit_index))->unit.current_weapon_index)

/* the seat's answer is the weapon's */
static boolean unit_set_or_test_seat_and_weapon_label(long unit_index, char const *seat, char const *weapon, boolean set)
{
	return objects[last_weapon_asked].seat_allows;
}
"""

DRIVER = r"""
static unsigned long seed = 0x17e5;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}
"""

DRIVER_MAIN = r"""
/* objects 1 to 15 at random, each printed as
definition:type:flags:parent:fired:puts away:seat:engine allows:picks up:grenade type */
static void random_world(void)
{
	long index;
	memset(objects, 0, sizeof objects);
	for (index = 1; index < MAXIMUM_OBJECTS; index++)
	{
		struct object_datum *object = &objects[index];
		long kind = next_random(8);
		object->definition_index = index >= FIRST_NEW_OBJECT ? NONE : next_random(4);
		object->object.type = kind == 0 ? _object_type_biped : kind < 3 ? _object_type_equipment : _object_type_weapon;
		object->object.flags = (next_random(4) != 0 ? FLAG(_object_connected_to_map_bit) : 0) |
			(next_random(6) == 0 ? FLAG(_object_invisible_bit) : 0);
		if (index >= FIRST_NEW_OBJECT)
		{
			object->object.type = _object_type_weapon;
			object->object.flags = FLAG(_object_connected_to_map_bit);
		}
		object->object.parent_object_index = next_random(5) == 0 ? 0 : NONE;
		object->can_be_fired = next_random(4) != 0;
		object->puts_away = next_random(4) != 0;
		object->seat_allows = next_random(5) != 0;
		object->engine_allows = next_random(6) != 0;
		object->engine_picks_up = next_random(6) != 0;
		object->grenade_type = (short)(next_random(5) - 1);
		printf(" %d:%d:%u:%d:%d:%d:%d:%d:%d:%d", object->definition_index, object->object.type,
			object->object.flags, object->object.parent_object_index, object->can_be_fired,
			object->puts_away, object->seat_allows, object->engine_allows, object->engine_picks_up,
			object->grenade_type);
	}
}

/* the unit's inventory at random, printed as
current desired | slots... | last used... | grenades... equipment flags control flags */
static void random_unit(void)
{
	long index;
	memset(&the_unit, 0, sizeof the_unit);
	the_unit.unit.current_weapon_index = (short)(next_random(5) - 1);
	the_unit.unit.desired_weapon_index = (short)(next_random(5) - 1);
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
	{
		the_unit.unit.weapon_object_indices[index] = next_random(3) == 0 ? NONE : 1 + next_random(11);
		the_unit.unit.weapon_last_used_at_game_time[index] = next_random(4) * 30;
	}
	for (index = 0; index < NUMBER_OF_UNIT_GRENADE_TYPES; index++)
	{
		long kind = next_random(6);
		the_unit.unit.grenade_counts[index] = (char)(kind == 0 ? 125 + next_random(3) : next_random(5));
	}
	the_unit.unit.equipment_object_index = next_random(3) == 0 ? NONE : 1 + next_random(11);
	the_unit.unit.flags = next_random(4) == 0 ? FLAG(_unit_doesnt_drop_items_bit) : 0;
	the_unit.unit.control_flags = next_random(3) == 0 ? FLAG(_unit_control_weapon_primary_trigger_bit) : 0;
	printf(" %d %d", the_unit.unit.current_weapon_index, the_unit.unit.desired_weapon_index);
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		printf(" %d:%d", the_unit.unit.weapon_object_indices[index], the_unit.unit.weapon_last_used_at_game_time[index]);
	printf(" %d %d %d %u %u", the_unit.unit.grenade_counts[0], the_unit.unit.grenade_counts[1],
		the_unit.unit.equipment_object_index, the_unit.unit.flags, the_unit.unit.control_flags);
}

/* the rest of the game: the player, the engine, the grenades of the
globals, the creations that fail and the unit's initial weapons */
static void random_game(void)
{
	long index;
	player_index = next_random(3) == 0 ? NONE : 3;
	the_player.local_player_index = next_random(3) == 0 ? NONE : 0;
	engine_running = next_random(3) == 0;
	the_globals.grenades.count = next_random(5) == 0 ? next_random(2) : NUMBER_OF_UNIT_GRENADE_TYPES;
	the_globals.grenades.address = grenades;
	for (index = 0; index < NUMBER_OF_UNIT_GRENADE_TYPES; index++)
	{
		grenades[index].maximum_count = (short)(next_random(6) == 0 ? 126 + next_random(4) : next_random(6));
		grenades[index].item.index = 40 + index;
	}
	next_new = FIRST_NEW_OBJECT;
	news = 0;
	new_failures = (unsigned long)next_random(16) & (unsigned long)next_random(16);
	the_unit_definition.unit.initial_weapons.count = next_random(5);
	the_unit_definition.unit.initial_weapons.address = initial_weapons;
	for (index = 0; index < 4; index++)
		initial_weapons[index].weapon.index = next_random(5) == 0 ? NONE : next_random(4);
	printf(" | %d %d %d %d %d %d %d %d %u %d", player_index, the_player.local_player_index, engine_running,
		the_globals.grenades.count, grenades[0].maximum_count, grenades[1].maximum_count,
		grenades[0].item.index, grenades[1].item.index, new_failures,
		the_unit_definition.unit.initial_weapons.count);
	for (index = 0; index < 4; index++)
		printf(" %d", initial_weapons[index].weapon.index);
	printf(" |");
}

static void print_unit(void)
{
	long index;
	printf(" %d %d", the_unit.unit.current_weapon_index, the_unit.unit.desired_weapon_index);
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		printf(" %d:%d", the_unit.unit.weapon_object_indices[index], the_unit.unit.weapon_last_used_at_game_time[index]);
	printf(" %d %d %d\n", the_unit.unit.grenade_counts[0], the_unit.unit.grenade_counts[1],
		the_unit.unit.equipment_object_index);
}

static void begin(char const *name, long argument, long second)
{
	printf("%s %d %d |", name, argument, second);
	random_unit();
	printf(" |");
	random_world();
	random_game();
}

int main(void)
{
	long run;
	for (run = 0; run < 12000; run++)
	{
		long which = next_random(10);
		long item = 1 + next_random(11);
		long argument = next_random(3);
		long answer = 0;
		switch (which)
		{
		case 0:
			begin("delete_all_weapons", 0, 0);
			unit_delete_all_weapons(0);
			break;
		case 1:
			begin("add_grenade", item, 0);
			objects[item].definition_index = 100 + item;
			answer = unit_add_grenade_to_inventory(0, item);
			break;
		case 2:
			begin("add_equipment", item, argument);
			objects[item].definition_index = 100 + item;
			answer = unit_add_equipment_to_inventory(0, item, (short)argument);
			break;
		case 3:
			begin("drop_current_equipment", 0, 0);
			unit_drop_current_equipment(0);
			break;
		case 4:
			begin("can_use_weapon", item, 0);
			answer = unit_can_use_weapon(0, item);
			break;
		case 5:
			begin("drop_inventory_weapons", 0, 0);
			unit_drop_inventory_weapons(0);
			break;
		case 6:
			begin("drop_grenades", 0, 0);
			the_globals.grenades.count = NUMBER_OF_UNIT_GRENADE_TYPES;
			unit_drop_grenades(0);
			break;
		case 7:
			begin("drop_current_weapon", argument != 0, 0);
			answer = unit_drop_current_weapon(0, argument != 0);
			break;
		case 8:
			begin("add_weapon", item, argument);
			answer = unit_add_weapon_to_inventory(0, item, argument);
			break;
		default:
			begin("add_initial_weapons", 0, 0);
			unit_add_initial_weapons(0);
			break;
		}
		printf(" -> %d |", answer);
		print_unit();
	}
	return 0;
}
"""

ENUMS = [
    ("source/objects/objects.h", "_object_connected_to_map_bit"),
    ("source/units/units.h", "_unit_add_weapon_normal"),
    ("source/units/units.h", "_unit_doesnt_drop_items_bit"),
    ("source/units/units.h", "_unit_control_weapon_primary_trigger_bit"),
    ("source/items/equipment_definitions.h", "_equipment_powerup_grenade"),
]

FUNCTIONS = [
    "unit_get_weapon",
    "unit_inventory_get_weapon",
    "unit_weapon_next_index",
    "unit_first_free_weapon_index",
    "unit_has_weapon_definition_index",
    "unit_delete_all_weapons",
    "unit_add_grenade_to_inventory",
    "unit_add_equipment_to_inventory",
    "unit_drop_current_equipment",
    "unit_can_use_weapon",
    "unit_drop_inventory_weapons",
    "unit_drop_grenades",
    "unit_drop_current_weapon",
    "unit_add_weapon_to_inventory",
    "unit_add_initial_weapons",
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
    units = text("source/units/units.c")
    functions = [function(units, name) for name in FUNCTIONS]
    return "\n".join([FAKE_WORLD, *enums, WORLD, *functions, DRIVER, DRIVER_MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-items-reference-") as work:
        source = Path(work) / "unit_items_reference.c"
        executable = Path(work) / "unit_items_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit items (tools/engine_unit_items_reference.py).\n"
        "# <function> <argument> <second> | <unit> | <objects 1-15> | <game> | <calls>... -> <answer> | <unit after>\n"
        "# unit: current desired <slot:last used>x4 <grenades>x2 equipment flags control_flags\n"
        "# object: definition:type:flags:parent:fired:puts away:seat:engine allows:picks up:grenade type\n"
        "# game: player local running grenade_count max0 max1 item0 item1 failures initial_count <initial>x4\n"
        "# unit after: current desired <slot:last used>x4 <grenades>x2 equipment\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_items_reference.py")
        print("the unit items reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
