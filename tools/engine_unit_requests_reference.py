"""The game's small requests of a unit, by the game and by its scripts: its
custom animation stopped, its animation aborted, opened or closed, its
emotion and its seat set, the base seat of the magic animations, whether a
player may enter it, whether it drops its items and how many grenades it
holds, taken from the sources and run over random units, to record what
each does, as the native engine's must.

    python3 tools/engine_unit_requests_reference.py          # writes the reference
    python3 tools/engine_unit_requests_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_stop_custom_animation`,
`unit_abort_animation`, `unit_open`, `unit_close`, `unit_set_emotion`,
`unit_scripting_set_seat`, `scripting_set_magic_base_seat`,
`unit_set_enterable_by_player`, `unit_scripting_doesnt_drop_items` and
`seat_label_to_base_seat_index`, and
source/units/unit_scripting_commands.c's
`unit_scripting_get_grenade_count`, found by name as tools/harness finds
them, with units.h's flag and state enums, and compiled for this machine
against a small fake world of six objects, some of them units.

The game's animation state change, its interpolation and its network
notes are the fake world's: each records that it was called and how, which
is what these functions decide (the state change itself is checked by the
animation's own tests).

The trace is written to tools/traces/unit_requests_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_requests_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>
#include <strings.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
typedef unsigned short word;
#define TRUE 1
#define FALSE 0
#define NONE -1
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define NUMBEROF(array) (sizeof(array) / sizeof((array)[0]))
#define _stricmp strcasecmp

#define NUMBER_OF_UNIT_GRENADE_TYPES 2
#define NUMBER_OF_UNIT_BASE_SEATS 6
#define OBJECTS 6

static char const *base_seat_labels[NUMBER_OF_UNIT_BASE_SEATS] =
	{ "asleep", "alert", "stand", "crouch", "flee", "flaming" };
"""

WORLD = r"""
struct unit_datum
{
	struct
	{
		unsigned long flags;
		struct
		{
			short state;
			char emotion_index;
		} animation;
		char magic_seat_index;
		char grenade_counts[NUMBER_OF_UNIT_GRENADE_TYPES];
	} unit;
};

static struct unit_datum units[OBJECTS];
static boolean is_unit[OBJECTS];
static short magic_base_animation_seat_index = NONE;

static struct unit_datum *unit_get(long index) { return &units[index]; }
static struct unit_datum *unit_try_and_get(long index)
{
	return index == NONE || !is_unit[index] ? NULL : &units[index];
}

/* what the game's state change, interpolation and network notes were asked */
static short state_set;
static short interpolation_frames;
static long network_notes;

static boolean unit_animation_set_state(long unit_index, short state)
{
	state_set = state;
	return TRUE;
}
static void object_start_interpolation(long object_index, long frames) { interpolation_frames = (short)frames; }
static void network_coop_note_unit_animation(long unit_index, long a, long b, boolean c) { network_notes++; }
static void network_coop_note_unit_open(long unit_index, boolean open) { network_notes++; }

/* an object list of every object, in order */
static long object_list_get_first(long list_index, long *reference_index)
{
	*reference_index = 0;
	return 0;
}
static long object_list_get_next(long list_index, long *reference_index)
{
	return ++*reference_index < OBJECTS ? *reference_index : NONE;
}
"""

DRIVER = r"""
static unsigned long seed = 0x5ea7;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static unsigned long random_flags(void)
{
	return ((unsigned long)next_random(1 << 16) << 16) | (unsigned long)next_random(1 << 16);
}

static void clear_calls(void)
{
	state_set = NONE;
	interpolation_frames = NONE;
	network_notes = 0;
}

static char const *const asked_labels[] =
	{ "asleep", "ALERT", "Stand", "crouch", "flee", "flaming", "", "crouching", "unarmed", "FlAmInG" };

int main(void)
{
	long run;
	long index;

	for (run = 0; run < 1000; run++)
	{
		long unit = next_random(4) == 0 ? NONE : 0;
		short state = (short)(next_random(40) - 2);
		if (next_random(2))
			state = _unit_state_user_animation;
		units[0].unit.animation.state = state;
		clear_calls();
		unit_stop_custom_animation(unit);
		printf("stop_custom %d %d -> %d %d\n", unit, state, state_set, network_notes);
	}
	clear_calls();
	unit_abort_animation(0);
	printf("abort -> %d %d\n", state_set, network_notes);
	for (run = 0; run < 200; run++)
	{
		long unit = next_random(4) == 0 ? NONE : 0;
		long open = next_random(2);
		clear_calls();
		if (open)
			unit_open(unit);
		else
			unit_close(unit);
		printf("open %d %d -> %d %d\n", unit, open, state_set, network_notes);
	}
	for (run = 0; run < 1000; run++)
	{
		long unit = next_random(4) == 0 ? NONE : 0;
		char old = (char)(next_random(256) - 128);
		word emotion = (word)next_random(1 << 16);
		if (next_random(2))
			emotion = (word)next_random(12);
		units[0].unit.animation.emotion_index = old;
		clear_calls();
		unit_set_emotion(unit, emotion);
		printf("emotion %d %d %u -> %d %d\n", unit, old, emotion, units[0].unit.animation.emotion_index,
			interpolation_frames);
	}
	for (run = 0; run < 200; run++)
	{
		long unit = next_random(4) == 0 ? NONE : 0;
		char old = (char)(next_random(10) - 2);
		char const *label = asked_labels[next_random(NUMBEROF(asked_labels))];
		units[0].unit.magic_seat_index = old;
		unit_scripting_set_seat(unit, label);
		printf("set_seat %d %d '%s' -> %d\n", unit, old, label, units[0].unit.magic_seat_index);
	}
	for (run = 0; run < (long)NUMBEROF(asked_labels); run++)
	{
		magic_base_animation_seat_index = (short)(next_random(10) - 2);
		scripting_set_magic_base_seat(asked_labels[run]);
		printf("magic_base_seat '%s' -> %d\n", asked_labels[run], magic_base_animation_seat_index);
	}
	for (run = 0; run < 1000; run++)
	{
		long unit = next_random(4) == 0 ? NONE : 0;
		unsigned long flags = random_flags();
		boolean enterable = (boolean)next_random(2);
		units[0].unit.flags = flags;
		unit_set_enterable_by_player(unit, enterable);
		printf("enterable %d %u %d -> %u\n", unit, flags, enterable, units[0].unit.flags);
	}
	for (run = 0; run < 500; run++)
	{
		printf("doesnt_drop |");
		for (index = 0; index < OBJECTS; index++)
		{
			is_unit[index] = (boolean)next_random(2);
			units[index].unit.flags = random_flags();
			printf(" %d:%u", is_unit[index], units[index].unit.flags);
		}
		unit_scripting_doesnt_drop_items(7);
		printf(" ->");
		for (index = 0; index < OBJECTS; index++)
			printf(" %u", units[index].unit.flags);
		printf("\n");
	}
	for (run = 0; run < 1000; run++)
	{
		long unit = next_random(5) == 0 ? NONE : next_random(OBJECTS);
		printf("grenade_count %d |", unit);
		for (index = 0; index < OBJECTS; index++)
			is_unit[index] = (boolean)next_random(2);
		for (index = 0; index < NUMBER_OF_UNIT_GRENADE_TYPES; index++)
		{
			char count = (char)(next_random(256) - 128);
			if (next_random(2))
				count = (char)next_random(5);
			if (unit != NONE)
				units[unit].unit.grenade_counts[index] = count;
			printf(" %d", count);
		}
		printf(" %d -> %d\n", unit == NONE ? 0 : is_unit[unit], unit_scripting_get_grenade_count(unit));
	}
	return 0;
}
"""

# each function, and the file it is defined in
FUNCTIONS = [
    ("source/units/units.c", "seat_label_to_base_seat_index"),
    ("source/units/units.c", "unit_stop_custom_animation"),
    ("source/units/units.c", "unit_abort_animation"),
    ("source/units/units.c", "unit_open"),
    ("source/units/units.c", "unit_close"),
    ("source/units/units.c", "unit_set_emotion"),
    ("source/units/units.c", "unit_scripting_set_seat"),
    ("source/units/units.c", "scripting_set_magic_base_seat"),
    ("source/units/units.c", "unit_set_enterable_by_player"),
    ("source/units/units.c", "unit_scripting_doesnt_drop_items"),
    ("source/units/unit_scripting_commands.c", "unit_scripting_get_grenade_count"),
]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    files = {}

    def text(path):
        if path not in files:
            files[path] = read(path)
        return files[path]

    units_h = text("source/units/units.h")
    enums = [enum_with(units_h, member) for member in ("_unit_actively_controlled_bit", "_unit_state_idle")]
    functions = [function(text(path), name) for path, name in FUNCTIONS]
    return "\n".join([FAKE_WORLD, *enums, WORLD, *functions, DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-requests-reference-") as work:
        source = Path(work) / "unit_requests_reference.c"
        executable = Path(work) / "unit_requests_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's small requests of a unit (tools/engine_unit_requests_reference.py).\n"
        "# A state set is -1 for none; network notes are how many the request sent.\n"
        "# stop_custom <unit> <state> -> <state set> <network notes>\n"
        "# abort -> <state set> <network notes>\n"
        "# open <unit> <open> -> <state set> <network notes>\n"
        "# emotion <unit> <emotion> <emotion asked> -> <emotion after> <interpolation frames>\n"
        "# set_seat <unit> <magic seat> '<label>' -> <magic seat after>\n"
        "# magic_base_seat '<label>' -> <magic base seat>\n"
        "# enterable <unit> <flags> <enterable> -> <flags after>\n"
        "# doesnt_drop | <is a unit>:<flags> of each object... -> <flags after> of each object...\n"
        "# grenade_count <unit> | <count of each kind>... <is a unit> -> <count>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_requests_reference.py")
        print("the unit requests reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
