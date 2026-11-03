"""The game's unit seat and current weapon questions, taken from the sources
and run over random units, to record what each answers, as the native
engine's must.

    python3 tools/engine_unit_seats_reference.py          # writes the reference
    python3 tools/engine_unit_seats_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_get_seat_label`,
`unit_scripting_unit_driver`, `unit_scripting_unit_gunner`,
`unit_scripting_unit_riders`, `unit_scripting_can_blink`,
`unit_scripting_vehicle_test_seat`, `vehicle_scripting_find_available_seats`,
`unit_set_actively_controlled`, `unit_get_zoom_magnification`,
`unit_overcharged`, `unit_get_current_weapon_label`,
`unit_integrated_night_vision_is_active`,
`unit_solo_player_integrated_night_vision_is_active`, `unit_unzoom` and
`unarmed_melee_weapon_definition_index` (with `unit_get_weapon`,
`unit_inventory_get_weapon` and `unit_seat_filled`, which they call), found
by name as tools/harness finds them, with units.h's flag enum, units.c's seat
desires and base seat labels, unit_definitions.h's seat flags, objects.h's
damage flags, object_types.h's masks and weapon_definitions.h's flags, and compiled
for this machine against a small fake world of six units and eight weapons.

The units' parents, seats, children, types and flags are random; the seat
labels are drawn from a few labels in mixed case, and the names asked for
from a few names, some of which match none. Each weapon's definition, its
flags and its zoom-out sound are random, and the weapon calls the units make
answer from small formulas the engine's test repeats.

Floats are printed as their bits, so that the engine's must be the same
float, not only print the same.

The trace is written to tools/traces/unit_seats_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_seats_reference.txt"

FAKE_WORLD = r"""
#include <ctype.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
typedef float real;
#define TRUE 1
#define FALSE 0
#define NONE -1
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define match_assert(file, line, condition) ((void)0)
#define _stricmp strcasecmp
#define csstrlen strlen
#define csstrcpy strcpy

static char *strlwr(char *text)
{
	char *at;
	for (at = text; *at; at++)
		*at = (char)tolower((unsigned char)*at);
	return text;
}

#define MAXIMUM_WEAPONS_PER_UNIT 4
#define UNITS 6
#define WEAPONS 8
#define SEATS 5
#define UNSIGNED_LONG_MAX 0xffffffffu
"""

WORLD = r"""
struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

struct unit_seat { unsigned long flags; char label[32]; };
struct unit_definition { struct { struct tag_block seats; } unit; };
struct unit_animation { short base_seat_index; };
struct unit_datum
{
	long definition_index;
	struct
	{
		short type;
		unsigned long damage_flags;
		long parent_object_index;
		long first_child_object_index;
		long next_object_index;
	} object;
	struct
	{
		unsigned long flags;
		long actor_index;
		long swarm_actor_index;
		long player_index;
		long driver_object_index;
		long gunner_object_index;
		short parent_seat_index;
		short current_weapon_index;
		long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT];
		short current_zoom_level;
		short desired_zoom_level;
		real integrated_night_vision_power;
		struct unit_animation animation;
	} unit;
};
#define object_datum unit_datum

struct weapon_datum { long definition_index; };
struct weapon_definition { struct { unsigned long flags; struct { long index; } zoom_out_sound; } weapon; };
struct player_datum { long local_player_index; long unit_index; };
struct game_globals { struct { long count; } weapon_list; };

static struct unit_datum units[UNITS];
static struct unit_seat seats[SEATS];
static struct unit_definition the_definition = { { { 0, seats } } };
static struct weapon_datum weapons[WEAPONS];
static struct weapon_definition weapon_definitions[5];
static struct player_datum players[2];
static long unit_players[UNITS];
static long local_players;
static boolean have_globals;
static struct game_globals the_globals;

static struct unit_datum *unit_get(long unit_index) { return &units[unit_index]; }
static struct unit_datum *unit_try_and_get(long unit_index) { return unit_index == NONE ? NULL : &units[unit_index]; }
static struct unit_datum *object_get(long object_index) { return &units[object_index]; }
static struct unit_definition *unit_definition_get(long definition_index) { return &the_definition; }
static struct weapon_datum *weapon_get(long weapon_index) { return &weapons[weapon_index]; }
static struct weapon_definition *weapon_definition_get(long definition_index) { return &weapon_definitions[definition_index]; }
static struct player_datum *player_get(long player_index) { return &players[player_index]; }
static long player_index_from_unit_index(long unit_index) { return unit_players[unit_index]; }
static long local_player_count(void) { return local_players; }
static long local_player_get_next(long local_player_index) { return 0; }
static long local_player_get_player_index(long local_player_index) { return 1; }
static struct game_globals *scenario_get_game_globals(void) { return have_globals ? &the_globals : NULL; }
static long list_index_to_weapon_definition_index(long list_index) { return 77 + list_index; }

/* the weapon calls, answering from formulas the engine's test repeats */
static real weapon_get_zoom_magnification(long weapon_index, short zoom_level)
{
	return (real)(weapon_index + 1) * 1.5f + (real)zoom_level;
}
static boolean weapon_overcharged(long weapon_index) { return weapon_index % 3 == 0; }
static char const *const weapon_labels[WEAPONS] = { "pistol", "rifle", "", "needler", "plasma", "rocket", "sniper", "flag" };
static char const *weapon_get_label(long weapon_index) { return weapon_labels[weapon_index]; }

/* what the units' calls do to the rest of the world, printed */
static void unspatialized_impulse_sound_new(long sound_index, real scale) { printf(" sound %d", sound_index); }
static void player_control_unzoom(long unit_index) { printf(" unzoomed %d", unit_index); }
static void item_in_unit_inventory(long weapon_index, long unit_index) { printf(" held %d", weapon_index); }
static void unit_update_driver_and_gunner(long unit_index) { printf(" seated %d", unit_index); }

/* every unit, in the table's order */
struct object_iterator { long next; };
static void object_iterator_new(struct object_iterator *iterator, unsigned long type_flags, unsigned char flags)
{
	iterator->next = 0;
}
static void *object_iterator_next(struct object_iterator *iterator)
{
	return iterator->next < UNITS ? &units[iterator->next++] : NULL;
}

/* the riders' list */
static long list_count;
static long list[UNITS];
static long object_list_new(void) { list_count = 0; return 0; }
static void object_list_add(long list_index, long object_index) { list[list_count++] = object_index; }

#define unit_get_current_weapon_index(unit_index) unit_inventory_get_weapon((unit_index), unit_get((unit_index))->unit.current_weapon_index)
"""

DRIVER = r"""
static unsigned long seed = 0x5ea75;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static unsigned bits(real value)
{
	unsigned result;
	memcpy(&result, &value, sizeof result);
	return result;
}

static char const *const seat_labels[] = { "driver", "Gunner", "P-riverside", "passenger", "W-GUNNER", "chief" };
static char const *const asked_names[] = { "", "gunner", "Gunner", "river", "p-", "driver", "x" };

/* none, or an index below n */
/* the seat labels in other cases, and names no seat has */
static char const *const tested_names[] = { "driver", "DRIVER", "gunner", "p-riverside", "Passenger", "chief", "x", "" };

/* none, or an index below n */
static long random_index(long n) { return next_random(n + 1) - 1; }

/* random seats for the one definition */
static void random_seats(void)
{
	long index;
	the_definition.unit.seats.count = next_random(SEATS + 1);
	for (index = 0; index < SEATS; index++)
	{
		seats[index].flags = (unsigned long)next_random(1 << 5);
		strcpy(seats[index].label, seat_labels[next_random(6)]);
	}
	for (index = 0; index < the_definition.unit.seats.count; index++)
		printf(" %u:'%s'", seats[index].flags, seats[index].label);
}

/* random weapons, and unit 0's inventory and zoom */
static void random_inventory(void)
{
	long index;
	for (index = 0; index < 5; index++)
	{
		weapon_definitions[index].weapon.flags = (unsigned long)next_random(1 << 16);
		weapon_definitions[index].weapon.zoom_out_sound.index = random_index(9);
	}
	for (index = 0; index < WEAPONS; index++)
		weapons[index].definition_index = next_random(5);
	units[0].unit.current_weapon_index = (short)random_index(MAXIMUM_WEAPONS_PER_UNIT);
	units[0].unit.current_zoom_level = (short)random_index(2);
	units[0].unit.desired_zoom_level = (short)random_index(2);
	units[0].unit.integrated_night_vision_power = (real)next_random(4) / 4.f;
	printf(" %d %d %d |", units[0].unit.current_weapon_index, units[0].unit.current_zoom_level,
		units[0].unit.desired_zoom_level);
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
	{
		long weapon = random_index(WEAPONS);
		units[0].unit.weapon_object_indices[index] = weapon;
		if (weapon == NONE)
			printf(" -1");
		else
			printf(" %d:%u:%d", weapon, weapon_definitions[weapons[weapon].definition_index].weapon.flags,
				weapon_definitions[weapons[weapon].definition_index].weapon.zoom_out_sound.index);
	}
}

int main(void)
{
	long run;
	long index;

	for (run = 0; run < 2000; run++)
	{
		long parent = random_index(1);
		short seat = (short)random_index(SEATS);
		short base = (short)next_random(6);
		units[0].object.parent_object_index = parent;
		units[0].unit.parent_seat_index = seat;
		units[0].unit.animation.base_seat_index = base;
		printf("seat_label %d %d %d |", parent == NONE ? NONE : 1, seat, base);
		random_seats();
		/* a seat past the parent's seats is never asked of */
		if (parent != NONE && seat >= the_definition.unit.seats.count)
			units[0].unit.parent_seat_index = seat = NONE;
		printf(" %d -> '%s'\n", seat, unit_get_seat_label(0));
	}
	for (run = 0; run < 2000; run++)
	{
		long unit = random_index(UNITS);
		printf("riders %d |", unit);
		for (index = 0; index < UNITS; index++)
		{
			units[index].object.type = (short)next_random(5);
			units[index].object.next_object_index = NONE;
			units[index].unit.parent_seat_index = (short)random_index(2);
			units[index].unit.driver_object_index = random_index(UNITS);
			units[index].unit.gunner_object_index = random_index(UNITS);
		}
		/* the unit's children are a random chain of the units after it */
		if (unit != NONE)
		{
			long *link = &units[unit].object.first_child_object_index;
			*link = NONE;
			for (index = unit + 1; index < UNITS; index++)
			{
				if (next_random(2))
				{
					*link = index;
					link = &units[index].object.next_object_index;
				}
			}
		}
		for (index = 0; index < UNITS; index++)
			printf(" %d:%d:%d:%d:%d", units[index].object.type, units[index].unit.parent_seat_index,
				units[index].unit.driver_object_index, units[index].unit.gunner_object_index,
				unit != NONE && index > unit ? units[index].object.next_object_index : NONE);
		printf(" %d -> %d %d", unit == NONE ? NONE : units[unit].object.first_child_object_index,
			unit_scripting_unit_driver(unit), unit_scripting_unit_gunner(unit));
		list_count = NONE;
		unit_scripting_unit_riders(unit);
		printf(" [");
		for (index = 0; index < list_count; index++)
			printf(index ? " %d" : "%d", list[index]);
		printf("]%s\n", list_count == NONE ? " none" : "");
	}
	for (run = 0; run < 2000; run++)
	{
		long vehicle = next_random(4) ? 0 : NONE;
		long unit = next_random(4) ? 1 : NONE;
		char const *name = tested_names[next_random(8)];
		units[1].object.parent_object_index = next_random(4) ? 0 : NONE;
		units[1].unit.parent_seat_index = (short)random_index(SEATS);
		printf("test_seat %d %d '%s' %d %d |", vehicle, unit, name, units[1].object.parent_object_index,
			units[1].unit.parent_seat_index);
		random_seats();
		printf(" -> %d\n", unit_scripting_vehicle_test_seat(vehicle, name, unit));
	}
	for (run = 0; run < 3000; run++)
	{
		char const *name = asked_names[next_random(7)];
		short desire = (short)random_index(5);
		short maximum = (short)next_random(6);
		short found[SEATS];
		short count;
		printf("available '%s' %d %d |", name, desire, maximum);
		random_seats();
		printf(" |");
		for (index = 1; index < UNITS; index++)
		{
			units[index].object.parent_object_index = random_index(1);
			units[index].unit.parent_seat_index = (short)random_index(SEATS);
			printf(" %d:%d", units[index].object.parent_object_index, units[index].unit.parent_seat_index);
		}
		/* the vehicle rides nothing */
		units[0].object.parent_object_index = NONE;
		units[0].unit.parent_seat_index = NONE;
		count = vehicle_scripting_find_available_seats(0, name, desire, found, maximum);
		printf(" ->");
		for (index = 0; index < count; index++)
			printf(" %d", found[index]);
		printf("\n");
	}
	for (run = 0; run < 2000; run++)
	{
		unsigned long flags = ((unsigned long)next_random(1 << 15) << 15) | (unsigned long)next_random(1 << 15);
		boolean wanted = (boolean)next_random(2);
		boolean can_blink = (boolean)next_random(2);
		long blinker = next_random(5) ? 0 : NONE;
		units[0].unit.flags = flags;
		units[0].unit.actor_index = next_random(3) ? NONE : 4;
		units[0].unit.swarm_actor_index = next_random(3) ? NONE : 5;
		units[0].unit.player_index = next_random(3) ? NONE : 0;
		units[0].object.damage_flags = (unsigned long)next_random(16);
		printf("controlled %u %d %d %d %d %u %d %d |", flags, wanted, units[0].unit.actor_index,
			units[0].unit.swarm_actor_index, units[0].unit.player_index, units[0].object.damage_flags, blinker,
			can_blink);
		for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		{
			units[0].unit.weapon_object_indices[index] = random_index(WEAPONS);
			printf(" %d", units[0].unit.weapon_object_indices[index]);
		}
		printf(" ->");
		unit_set_actively_controlled(0, wanted);
		unit_scripting_can_blink(blinker, can_blink);
		printf(" = %u\n", units[0].unit.flags);
	}
	for (run = 0; run < 3000; run++)
	{
		short zoom_level = (short)random_index(2);
		printf("weapon %d", zoom_level);
		random_inventory();
		printf(" -> %u %d '%s' %d\n", bits(unit_get_zoom_magnification(0, zoom_level)), unit_overcharged(0),
			unit_get_current_weapon_label(0), unit_integrated_night_vision_is_active(0));
	}
	for (run = 0; run < 2000; run++)
	{
		long count = next_random(3);
		long unit = random_index(1);
		local_players = count;
		players[1].unit_index = unit;
		printf("solo %d %d", count, unit);
		random_inventory();
		printf(" -> %d\n", unit_solo_player_integrated_night_vision_is_active());
	}
	for (run = 0; run < 2000; run++)
	{
		long player = random_index(1);
		unit_players[0] = player;
		players[0].local_player_index = 0;
		players[1].local_player_index = NONE;
		printf("unzoom %d", player);
		random_inventory();
		printf(" ->");
		unit_unzoom(0);
		printf(" = %d %d %u\n", units[0].unit.current_zoom_level, units[0].unit.desired_zoom_level,
			bits(units[0].unit.integrated_night_vision_power));
	}
	for (index = 0; index < 3; index++)
	{
		have_globals = index > 0;
		the_globals.weapon_list.count = index > 1;
		printf("unarmed %d %d -> %d\n", have_globals, the_globals.weapon_list.count,
			unarmed_melee_weapon_definition_index());
	}
	return 0;
}
"""

# each function, and the file it is defined in
FUNCTIONS = [
    ("source/units/units.c", "unit_get_weapon"),
    ("source/units/units.c", "unit_inventory_get_weapon"),
    ("source/units/units.c", "unit_seat_filled"),
    ("source/units/units.c", "unit_get_seat_label"),
    ("source/units/units.c", "unit_scripting_unit_driver"),
    ("source/units/units.c", "unit_scripting_unit_gunner"),
    ("source/units/units.c", "unit_scripting_unit_riders"),
    ("source/units/units.c", "unit_scripting_can_blink"),
    ("source/units/units.c", "unit_scripting_vehicle_test_seat"),
    ("source/units/units.c", "vehicle_scripting_find_available_seats"),
    ("source/units/units.c", "unit_set_actively_controlled"),
    ("source/units/units.c", "unit_get_zoom_magnification"),
    ("source/units/units.c", "unit_overcharged"),
    ("source/units/units.c", "unit_get_current_weapon_label"),
    ("source/units/units.c", "unit_integrated_night_vision_is_active"),
    ("source/units/units.c", "unit_solo_player_integrated_night_vision_is_active"),
    ("source/units/units.c", "unit_unzoom"),
    ("source/units/units.c", "unarmed_melee_weapon_definition_index"),
]


def table(source, start):
    """A table's definition, from its declaration to its end."""
    at = source.index(start)
    return source[at : source.index(";", at) + 1]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    files = {}

    def text(path):
        if path not in files:
            files[path] = read(path)
        return files[path]

    units_h = text("source/units/units.h")
    units_c = text("source/units/units.c")
    enums = [
        enum_with(text("source/objects/object_types.h"), "_object_mask_unit"),
        enum_with(text("source/objects/objects.h"), "_object_dead_bit"),
        enum_with(text("source/units/unit_definitions.h"), "_unit_seat_driver_bit"),
        enum_with(units_h, "_unit_actively_controlled_bit"),
        enum_with(units_h, "_unit_base_seat_asleep"),
        enum_with(units_c, "_vehicle_seat_desire_not_driver"),
        enum_with(text("source/items/weapon_definitions.h"), "_weapon_enables_unit_integrated_night_vision_bit"),
    ]
    labels = table(units_c, "char const *base_seat_labels[NUMBER_OF_UNIT_BASE_SEATS] = {")
    functions = [function(text(path), name) for path, name in FUNCTIONS]
    return "\n".join([FAKE_WORLD, *enums, WORLD, labels, *functions, DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-seats-reference-") as work:
        source = Path(work) / "unit_seats_reference.c"
        executable = Path(work) / "unit_seats_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit seat and current weapon questions (tools/engine_unit_seats_reference.py).\n"
        "# seat_label <parent> <seat> <base seat> | <flags>:'<label>' of each seat... <seat asked> -> '<label>'\n"
        "# riders <unit> | <type>:<seat>:<driver>:<gunner>:<next> of each unit... <first child>"
        " -> <driver> <gunner> [<riders>]\n"
        "# test_seat <vehicle> <unit> '<name>' <unit's parent> <unit's seat> | <seats>... -> <in the seat>\n"
        "# available '<name>' <desire> <maximum> | <seats>... | <parent>:<seat> of units 1-5 -> <seats>...\n"
        "# controlled <flags> <wanted> <actor> <swarm> <player> <damage flags> <blinker> <can blink> | <weapons>..."
        " -> <calls>... = <flags>\n"
        "# weapon <zoom level> <current weapon> <current zoom> <desired zoom> | <weapon>:<flags>:<zoom-out sound>"
        " of each slot... -> <magnification bits> <overcharged> '<label>' <night vision>\n"
        "# solo <local players> <their unit> <inventory as weapon's> -> <night vision>\n"
        "# unzoom <player> <inventory as weapon's> -> <calls>... = <zoom> <desired zoom> <night vision power bits>\n"
        "# unarmed <globals> <weapons listed> -> <definition>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_seats_reference.py")
        print("the unit seats reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
