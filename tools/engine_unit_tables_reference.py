"""The game's unit name tables and definition lookups, taken from the
sources and run over every name and over random definitions, to record what
each answers, as the native engine's must.

    python3 tools/engine_unit_tables_reference.py          # writes the reference
    python3 tools/engine_unit_tables_reference.py --check  # fails if it is stale

The functions are source/units/dialogue_definitions.c's
`dialogue_get_vocalization_name` and `dialogue_get_vocalization_type_by_name`,
source/units/unit_dialogue.c's `unit_get_speech_priority_name` and
`unit_get_speech_priority_by_name`, source/units/unit_definitions.c's
`unit_definition_get_active_hud_index` and
`unit_definition_get_seat_active_hud_index`, and source/units/units.c's
`unit_seat_is_driver`, `unit_seat_is_gunner`, `unit_seat_allow_noncombatants`
and `unit_has_weapon`, found by name as tools/harness finds them, with the
two name tables as the sources define them, and compiled for this machine
against a small fake world of one unit and its definition.

Every vocalization and priority is asked by number (and a few numbers past
each table) and by name (each name, each abbreviation and a few names the
game doesn't know). Definitions are drawn at random: up to four seats with
random flags, and up to three HUDs on the unit and on each seat. The unit's
weapon slots hold random object indices or none, and the weapon asked for
may be none.

The trace is written to tools/traces/unit_tables_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_tables_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
#define TRUE 1
#define FALSE 0
#define NONE -1
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define csstrcmp strcmp

#define NUMBER_OF_DIALOGUE_VOCALIZATION_TYPES 209
#define _unit_speech_none 0
#define NUMBER_OF_UNIT_SPEECH_PRIORITIES 11
#define MAXIMUM_WEAPONS_PER_UNIT 4

enum
{
	_unit_seat_driver_bit = 2,
	_unit_seat_gunner_bit = 3,
	_unit_seat_allows_noncombatants_bit = 10,
};

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

struct unit_hud_reference { long hud; };
struct unit_seat { unsigned long flags; struct tag_block seat_huds; };
struct unit_definition { struct { struct tag_block seats; struct tag_block huds; } unit; };
struct unit_datum
{
	long definition_index;
	struct { long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT]; } unit;
};

static struct unit_datum the_unit;
static struct unit_definition the_definition;

static struct unit_datum *unit_get(long unit_index) { return &the_unit; }
static struct unit_definition *unit_definition_get(long definition_index) { return &the_definition; }
/* a HUD reference's tag index, as the cache resolves it */
static long verify_tag_reference(long const *reference) { return *reference; }
"""

DRIVER = r"""
static unsigned long seed = 0x7ab1e;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static struct unit_seat seats[4];
static struct unit_hud_reference huds[3];
static struct unit_hud_reference seat_huds[4][3];

/* random HUDs: up to three, each the tag 100 and its place */
static void random_huds(struct tag_block *block, struct unit_hud_reference *references)
{
	long index;
	block->count = next_random(4);
	block->address = references;
	for (index = 0; index < 3; index++)
		references[index].hud = 100 + index + 10 * next_random(3);
	for (index = 0; index < block->count; index++)
		printf(" %d", references[index].hud);
}

static char const *const unknown_names[] = { "", "SHOUT", "Idle", "dead", "idle combat ", "none" };

int main(void)
{
	long run;
	short index;

	for (index = -2; index < NUMBER_OF_DIALOGUE_VOCALIZATION_TYPES + 2; index++)
	{
		printf("vocalization %d 0 -> %s\n", index, dialogue_get_vocalization_name(index, FALSE));
		printf("vocalization %d 1 -> %s\n", index, dialogue_get_vocalization_name(index, TRUE));
	}
	for (index = 0; index < NUMBER_OF_DIALOGUE_VOCALIZATION_TYPES; index++)
	{
		long abbreviated;
		for (abbreviated = 0; abbreviated < 2; abbreviated++)
		{
			char const *name = dialogue_vocalization_type_name[index][abbreviated];
			printf("vocalization_index '%s' -> %d\n", name, dialogue_get_vocalization_type_by_name(name));
		}
	}
	for (index = 0; index < (short)(sizeof unknown_names / sizeof unknown_names[0]); index++)
		printf("vocalization_index '%s' -> %d\n", unknown_names[index],
			dialogue_get_vocalization_type_by_name(unknown_names[index]));
	for (index = -2; index < NUMBER_OF_UNIT_SPEECH_PRIORITIES + 2; index++)
		printf("priority %d -> %s\n", index, unit_get_speech_priority_name(index));
	for (index = 0; index < NUMBER_OF_UNIT_SPEECH_PRIORITIES; index++)
		printf("priority_index '%s' -> %d\n", global_speech_priority_names[index],
			unit_get_speech_priority_by_name(global_speech_priority_names[index]));
	for (index = 0; index < (short)(sizeof unknown_names / sizeof unknown_names[0]); index++)
		printf("priority_index '%s' -> %d\n", unknown_names[index],
			unit_get_speech_priority_by_name(unknown_names[index]));
	for (run = 0; run < 2000; run++)
	{
		short seat = (short)(next_random(8) - 2);
		long count = next_random(5);
		memset(&the_definition, 0, sizeof the_definition);
		the_definition.unit.seats.count = count;
		the_definition.unit.seats.address = seats;
		printf("seat %d |", seat);
		for (index = 0; index < count; index++)
		{
			seats[index].flags = (unsigned long)next_random(1 << 12);
			printf(" %u", seats[index].flags);
		}
		printf(" -> %d %d %d\n", unit_seat_is_driver(0, seat), unit_seat_is_gunner(0, seat),
			unit_seat_allow_noncombatants(0, seat));
	}
	for (run = 0; run < 500; run++)
	{
		boolean multiplayer = (boolean)next_random(3);
		memset(&the_definition, 0, sizeof the_definition);
		printf("hud %d |", multiplayer);
		random_huds(&the_definition.unit.huds, huds);
		printf(" -> %d\n", unit_definition_get_active_hud_index(&the_definition, multiplayer));
	}
	for (run = 0; run < 500; run++)
	{
		boolean multiplayer = (boolean)next_random(3);
		long count = 1 + next_random(4);
		short seat = (short)next_random(count);
		memset(&the_definition, 0, sizeof the_definition);
		the_definition.unit.seats.count = count;
		the_definition.unit.seats.address = seats;
		for (index = 0; index < count; index++)
		{
			seats[index].seat_huds.count = 0;
			seats[index].seat_huds.address = seat_huds[index];
		}
		printf("seat_hud %d %d |", seat, multiplayer);
		random_huds(&seats[seat].seat_huds, seat_huds[seat]);
		printf(" -> %d\n", unit_definition_get_seat_active_hud_index(&the_definition, seat, multiplayer));
	}
	for (run = 0; run < 1000; run++)
	{
		long weapon = next_random(7) - 1;
		printf("has_weapon %d |", weapon);
		for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		{
			the_unit.unit.weapon_object_indices[index] = next_random(3) == 0 ? NONE : next_random(6);
			printf(" %d", the_unit.unit.weapon_object_indices[index]);
		}
		printf(" -> %d\n", unit_has_weapon(0, weapon));
	}
	return 0;
}
"""

# each function, and the file it is defined in
FUNCTIONS = [
    ("source/units/dialogue_definitions.c", "dialogue_get_vocalization_name"),
    ("source/units/dialogue_definitions.c", "dialogue_get_vocalization_type_by_name"),
    ("source/units/unit_dialogue.c", "unit_get_speech_priority_name"),
    ("source/units/unit_dialogue.c", "unit_get_speech_priority_by_name"),
    ("source/units/unit_definitions.c", "unit_definition_get_active_hud_index"),
    ("source/units/unit_definitions.c", "unit_definition_get_seat_active_hud_index"),
    ("source/units/units.c", "unit_seat_is_driver"),
    ("source/units/units.c", "unit_seat_is_gunner"),
    ("source/units/units.c", "unit_seat_allow_noncombatants"),
    ("source/units/units.c", "unit_has_weapon"),
]

# each table, and the file it is defined in
TABLES = [
    ("source/units/dialogue_definitions.c", "char const *dialogue_vocalization_type_name["),
    ("source/units/unit_dialogue.c", "char const *global_speech_priority_names["),
]


def table(source, start):
    """A table's definition, from its declaration to its closing brace."""
    at = source.index(start)
    return source[at : source.index("};", at) + 2]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    files = {}

    def text(path):
        if path not in files:
            files[path] = read(path)
        return files[path]

    tables = [table(text(path), start) for path, start in TABLES]
    functions = [function(text(path), name) for path, name in FUNCTIONS]
    return "\n".join([FAKE_WORLD, *tables, *functions, DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-tables-reference-") as work:
        source = Path(work) / "unit_tables_reference.c"
        executable = Path(work) / "unit_tables_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit name tables and definition lookups (tools/engine_unit_tables_reference.py).\n"
        "# vocalization <type> <abbreviated> -> <name>; vocalization_index '<name>' -> <type>\n"
        "# priority <priority> -> <name>; priority_index '<name>' -> <priority>\n"
        "# seat <seat> | <flags of each seat>... -> <driver> <gunner> <allows noncombatants>\n"
        "# hud <multiplayer> | <huds>... -> <hud>; seat_hud <seat> <multiplayer> | <seat's huds>... -> <hud>\n"
        "# has_weapon <weapon> | <slots>... -> <has>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_tables_reference.py")
        print("the unit tables reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
