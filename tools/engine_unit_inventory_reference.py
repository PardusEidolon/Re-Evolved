"""The game's unit inventory, taken from the sources and run over random
units, to record which weapon and grenade it picks and what it answers, as
the native engine's must.

    python3 tools/engine_unit_inventory_reference.py          # writes the reference
    python3 tools/engine_unit_inventory_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_weapon_next_index`,
`unit_inventory_next_weapon`, `unit_inventory_next_grenade`,
`unit_first_free_weapon_index`, `unit_add_grenade_type_to_inventory`,
`unit_has_weapon_definition_index`, `unit_has_weapon_with_flag`,
`unit_get_grenade_count`, `unit_get_current_grenade_type`,
`unit_inventory_get_must_be_readied_weapon`, `unit_inventory_get_weapon`,
`unit_get_weapon`, `base_seat_label_get`, `seat_label_to_base_seat_index`,
`base_weapon_label_get` and `unit_get_zoom_level`, found by name as
tools/harness finds them and compiled for this machine against a small fake
world of one unit datum. Units are drawn at random: weapons in some of their
four slots, each usable or not, needing to be readied or not and last used
at one of a few game times (so that some tie), grenade counts of each kind
from below zero to near the top of a signed byte (so that adding wraps), and
a current weapon, grenade and zoom level that may be none.

Asserts are compiled out, as in the released game, and the cases stay where
the code reads only its own arrays.

The trace is written to tools/traces/unit_inventory_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_inventory_reference.txt"

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
#define _stricmp strcasecmp
#define match_assert(file, line, condition) ((void)0)
#define VALID_INDEX(index, count) ((index) >= 0 && (index) < (count))
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)

#define MAXIMUM_WEAPONS_PER_UNIT 4
#define NUMBER_OF_UNIT_GRENADE_TYPES 2
#define NUMBER_OF_UNIT_BASE_SEATS 6
#define NUMBER_OF_UNIT_BASE_WEAPONS 1

static char const *base_seat_labels[NUMBER_OF_UNIT_BASE_SEATS] =
	{ "asleep", "alert", "stand", "crouch", "flee", "flaming" };

struct unit_datum
{
	struct
	{
		short current_weapon_index;
		short desired_weapon_index;
		long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT];
		long weapon_last_used_at_game_time[MAXIMUM_WEAPONS_PER_UNIT];
		char current_grenade_index;
		char desired_grenade_index;
		char grenade_counts[NUMBER_OF_UNIT_GRENADE_TYPES];
		char current_zoom_level;
		char desired_zoom_level;
	} unit;
};

struct weapon_datum
{
	long definition_index;
	struct { unsigned long flags; } weapon;
	boolean usable;
	boolean must_be_readied;
};

#define MAXIMUM_OBJECTS 8
static struct unit_datum the_unit;
static struct weapon_datum weapons[MAXIMUM_OBJECTS];

static struct unit_datum *unit_get(long unit_index) { return &the_unit; }
static struct weapon_datum *weapon_get(long weapon_index) { return &weapons[weapon_index]; }
static boolean unit_can_use_weapon(long unit_index, long weapon_index) { return weapons[weapon_index].usable; }
static boolean weapon_must_be_readied(long weapon_index) { return weapons[weapon_index].must_be_readied; }
static long unit_get_weapon(struct unit_datum *unit, short index);
"""

DRIVER = r"""
static unsigned long seed = 0x1a7e;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

/* fills the unit's slots at random, and prints each as
usable:must be readied:last used:definition:flags (or -1:last used) */
static void random_slots(void)
{
	long index;
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
	{
		long weapon = 1 + index;
		boolean held = next_random(4) != 0;
		weapons[weapon].usable = next_random(4) != 0;
		weapons[weapon].must_be_readied = next_random(5) == 0;
		weapons[weapon].definition_index = next_random(5);
		weapons[weapon].weapon.flags = (unsigned long)next_random(16);
		the_unit.unit.weapon_object_indices[index] = held ? weapon : NONE;
		the_unit.unit.weapon_last_used_at_game_time[index] = next_random(4) * 30;
		if (held)
			printf(" %d:%d:%d:%d:%u", weapons[weapon].usable, weapons[weapon].must_be_readied,
				the_unit.unit.weapon_last_used_at_game_time[index], weapons[weapon].definition_index,
				weapons[weapon].weapon.flags);
		else
			printf(" -1:%d", the_unit.unit.weapon_last_used_at_game_time[index]);
	}
}

static void random_grenades(void)
{
	long index;
	for (index = 0; index < NUMBER_OF_UNIT_GRENADE_TYPES; index++)
	{
		long kind = next_random(6);
		the_unit.unit.grenade_counts[index] = (char)(kind == 0 ? 120 + next_random(8) : kind == 1 ? -next_random(3) : next_random(5));
		printf(" %d", the_unit.unit.grenade_counts[index]);
	}
}

static char const *const asked_labels[] =
	{ "asleep", "ALERT", "Stand", "crouch", "flee", "flaming", "", "stand ", "crouching", "unarmed", "FlAmInG" };

int main(void)
{
	long run;

	for (run = 0; run < 4000; run++)
	{
		short current = (short)(next_random(5) - 1);
		short delta = (short)(next_random(3) - 1);
		boolean through_inventory = next_random(2);
		short answer;
		memset(&the_unit, 0, sizeof the_unit);
		printf("next_weapon %d %d |", current, delta);
		random_slots();
		answer = through_inventory
			? unit_inventory_next_weapon(0, current, delta)
			: unit_weapon_next_index(0, current, delta);
		printf(" -> %d\n", answer);
	}
	for (run = 0; run < 2000; run++)
	{
		short current = (short)(next_random(3) - 1);
		short delta = (short)(next_random(3) - 1);
		memset(&the_unit, 0, sizeof the_unit);
		printf("next_grenade %d %d |", current, delta);
		random_grenades();
		printf(" -> %d\n", unit_inventory_next_grenade(0, current, delta));
	}
	for (run = 0; run < 1000; run++)
	{
		memset(&the_unit, 0, sizeof the_unit);
		printf("first_free |");
		random_slots();
		printf(" -> %d\n", unit_first_free_weapon_index(0));
	}
	for (run = 0; run < 2000; run++)
	{
		short type = (short)(next_random(5) - 1);
		short count = (short)(next_random(4) == 0 ? next_random(300) : next_random(5));
		short answer;
		memset(&the_unit, 0, sizeof the_unit);
		the_unit.unit.current_grenade_index = (char)(next_random(3) - 1);
		the_unit.unit.desired_grenade_index = (char)(next_random(3) - 1);
		printf("add_grenades %d %d %d %d |", type, count,
			the_unit.unit.current_grenade_index, the_unit.unit.desired_grenade_index);
		random_grenades();
		answer = unit_add_grenade_type_to_inventory(0, type, count);
		printf(" -> %d %d %d %d %d\n", answer, the_unit.unit.grenade_counts[0], the_unit.unit.grenade_counts[1],
			the_unit.unit.current_grenade_index, the_unit.unit.desired_grenade_index);
	}
	for (run = 0; run < 2000; run++)
	{
		long definition = next_random(6);
		long flag = next_random(5);
		memset(&the_unit, 0, sizeof the_unit);
		printf("has %d %d |", definition, flag);
		random_slots();
		printf(" -> %d %d %d\n", unit_has_weapon_definition_index(0, definition),
			unit_has_weapon_with_flag(0, flag), unit_inventory_get_must_be_readied_weapon(0));
	}
	for (run = 0; run < 1000; run++)
	{
		short type = (short)(next_random(5) - 1);
		memset(&the_unit, 0, sizeof the_unit);
		the_unit.unit.current_grenade_index = (char)(next_random(3) - 1);
		printf("grenade_count %d %d |", type, the_unit.unit.current_grenade_index);
		random_grenades();
		printf(" -> %d %d\n", unit_get_grenade_count(0, type), unit_get_current_grenade_type(0));
	}
	for (run = 0; run < 1000; run++)
	{
		short index = (short)(next_random(5) - 1);
		long through_unit;
		memset(&the_unit, 0, sizeof the_unit);
		the_unit.unit.current_zoom_level = (char)(next_random(4) - 1);
		printf("weapon %d %d |", index, the_unit.unit.current_zoom_level);
		random_slots();
		through_unit = unit_get_weapon(&the_unit, index);
		printf(" -> %d %d %d\n", unit_inventory_get_weapon(0, index), through_unit, unit_get_zoom_level(0));
	}
	for (run = 0; run < NUMBER_OF_UNIT_BASE_SEATS; run++)
		printf("seat_label %d -> %s\n", run, base_seat_label_get((short)run));
	for (run = 0; run < (long)(sizeof asked_labels / sizeof asked_labels[0]); run++)
		printf("seat_index '%s' -> %d\n", asked_labels[run], seat_label_to_base_seat_index(asked_labels[run]));
	printf("weapon_label 0 -> %s\n", base_weapon_label_get(0));
	return 0;
}
"""

FUNCTIONS = [
    "unit_get_weapon",
    "unit_weapon_next_index",
    "unit_inventory_next_weapon",
    "unit_inventory_next_grenade",
    "unit_first_free_weapon_index",
    "unit_add_grenade_type_to_inventory",
    "unit_has_weapon_definition_index",
    "unit_has_weapon_with_flag",
    "unit_get_grenade_count",
    "unit_get_current_grenade_type",
    "unit_inventory_get_must_be_readied_weapon",
    "unit_inventory_get_weapon",
    "base_seat_label_get",
    "seat_label_to_base_seat_index",
    "base_weapon_label_get",
    "unit_get_zoom_level",
]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    units = read("source/units/units.c")
    return "\n".join([FAKE_WORLD, *(function(units, name) for name in FUNCTIONS), DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-inventory-reference-") as work:
        source = Path(work) / "unit_inventory_reference.c"
        executable = Path(work) / "unit_inventory_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit inventory (tools/engine_unit_inventory_reference.py).\n"
        "# A held slot is usable:must be readied:last used:definition:flags, or -1:last used.\n"
        "# next_weapon <current> <delta> | <slots>... -> <answer>\n"
        "# next_grenade <current> <delta> | <counts>... -> <answer>\n"
        "# first_free | <slots>... -> <answer>\n"
        "# add_grenades <type> <count> <current> <desired> | <counts>... -> <answer> <counts>... <current> <desired>\n"
        "# has <definition> <flag> | <slots>... -> <has definition> <has flag> <must be readied slot>\n"
        "# grenade_count <type> <current> | <counts>... -> <count> <current>\n"
        "# weapon <index> <zoom> | <slots>... -> <weapon> <weapon> <zoom>\n"
        "# seat_label <index> -> <label>; seat_index '<label>' -> <index>; weapon_label 0 -> <label>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_inventory_reference.py")
        print("the unit inventory reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
