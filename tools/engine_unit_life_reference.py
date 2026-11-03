"""The game's unit and biped hooks that do nothing, its quiet deaths, its
biped's flight, surfaces and melee, whether a unit speaks and the
equipment it holds, taken from the sources and run over random units, to
record what each answers or leaves, as the native engine's must.

    python3 tools/engine_unit_life_reference.py          # writes the reference
    python3 tools/engine_unit_life_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_delete`, `units_dispose`,
`units_dispose_from_old_map`, `unit_kill_no_statistics`,
`unit_kill_silent`, `unit_get_current_equipment` and
`unit_flying_through_air`; source/units/bipeds.c's `biped_delete`,
`bipeds_initialize`, `bipeds_initialize_for_new_map`, `bipeds_dispose`,
`bipeds_dispose_from_old_map`, `biped_flying_through_air`,
`biped_disconnect_from_structure_bsp` and `biped_stop_melee_attack`;
source/units/vehicles.c's `vehicle_delete`, `vehicles_initialize`,
`vehicles_initialize_for_new_map`, `vehicles_dispose` and
`vehicles_dispose_from_old_map`; source/units/unit_dialogue.c's
`unit_is_speaking`; and source/units/biped_limp_noodle.c's
`biped_limp_noodle_get_max_relaxation_iterations`, found by name as
tools/harness finds them, with objects.h's damage flags, object_types.h's
types and biped_definitions.h's flags, and compiled for this machine
against a small fake world of four objects.

The hooks are called on a world of random bytes, which is compared after
each with the bytes before it. The damage flags are random words. Each
biped's ticks in the air, damage flags, type and definition (one that
flies, one that doesn't) are random, as are its surfaces, its melee ticks,
its unit's speech priority and its equipment, and the biped asked of may
be none where the game allows it.

The trace is written to tools/traces/unit_life_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_life_reference.txt"

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
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))

#define UNSIGNED_LONG_MAX 0xffffffffu
#define OBJECTS 4
"""

WORLD = r"""
/* a biped and its unit are one object, as in the game */
struct object_datum
{
	struct
	{
		short type;
		long damage_flags;
	} object;
	long definition_index;
	struct
	{
		long equipment_object_index;
		struct { struct { short priority; } current; } speech;
	} unit;
	struct
	{
		long support_surface_index;
		long pathfinding_surface_index;
		long last_pathfinding_surface_index;
		char airborne_ticks;
		char player_melee_ticks;
	} biped;
};
#define unit_datum object_datum
#define biped_datum object_datum

struct biped_definition { struct { unsigned long flags; } biped; };

static struct object_datum objects[OBJECTS];
static struct biped_definition definitions[2];

static struct object_datum *unit_get(long index) { return &objects[index]; }
static struct object_datum *biped_get(long index) { return &objects[index]; }
static struct object_datum *biped_try_and_get(long index) { return index == NONE ? NULL : &objects[index]; }
static struct biped_definition *biped_definition_get(long index) { return &definitions[index]; }
"""

DRIVER = r"""
static unsigned long seed = 0x11fe5;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static long random_word(void)
{
	return (long)(((unsigned long)next_random(1 << 16) << 16) | (unsigned long)next_random(1 << 16));
}

static long random_index(void)
{
	return next_random(3) == 0 ? NONE : next_random(5000);
}

static void scramble(void)
{
	unsigned char *bytes = (unsigned char *)objects;
	unsigned long index;
	for (index = 0; index < sizeof objects; index++)
		bytes[index] = (unsigned char)next_random(256);
}

static void hook(const char *name, void (*call)(void))
{
	struct object_datum before[OBJECTS];
	scramble();
	memcpy(before, objects, sizeof objects);
	call();
	printf("hook %s -> %s\n", name, memcmp(before, objects, sizeof objects) ? "changed" : "unchanged");
}

static void hook_on(const char *name, void (*call)(long))
{
	struct object_datum before[OBJECTS];
	long index = next_random(OBJECTS);
	scramble();
	memcpy(before, objects, sizeof objects);
	call(index);
	printf("hook %s -> %s\n", name, memcmp(before, objects, sizeof objects) ? "changed" : "unchanged");
}

int main(void)
{
	long run;

	definitions[0].biped.flags = 0;
	definitions[1].biped.flags = FLAG(_biped_flying_bit);
	for (run = 0; run < 20; run++)
	{
		hook_on("unit_delete", unit_delete);
		hook("units_dispose", units_dispose);
		hook("units_dispose_from_old_map", units_dispose_from_old_map);
		hook_on("biped_delete", biped_delete);
		hook("bipeds_initialize", bipeds_initialize);
		hook("bipeds_initialize_for_new_map", bipeds_initialize_for_new_map);
		hook("bipeds_dispose", bipeds_dispose);
		hook("bipeds_dispose_from_old_map", bipeds_dispose_from_old_map);
		hook_on("vehicle_delete", vehicle_delete);
		hook("vehicles_initialize", vehicles_initialize);
		hook("vehicles_initialize_for_new_map", vehicles_initialize_for_new_map);
		hook("vehicles_dispose", vehicles_dispose);
		hook("vehicles_dispose_from_old_map", vehicles_dispose_from_old_map);
	}
	for (run = 0; run < 2000; run++)
	{
		long flags = random_word();
		long silent = next_random(2);
		objects[0].object.damage_flags = flags;
		if (silent)
			unit_kill_silent(0);
		else
			unit_kill_no_statistics(0);
		printf("kill %d %u -> %u\n", silent, (unsigned)flags, (unsigned)objects[0].object.damage_flags);
	}
	for (run = 0; run < 3000; run++)
	{
		short type = (short)next_random(4);
		long flags = random_word();
		long definition = next_random(2);
		char ticks = (char)(next_random(256) - 128);
		if (next_random(2))
			ticks = (char)next_random(8);
		objects[1].object.type = type;
		objects[1].object.damage_flags = flags;
		objects[1].definition_index = definition;
		objects[1].biped.airborne_ticks = ticks;
		printf("air %d %u %d %d -> %d", type, (unsigned)flags, definition, ticks, unit_flying_through_air(1));
		if (type == _object_type_biped)
			printf(" %d", biped_flying_through_air(1));
		printf("\n");
	}
	for (run = 0; run < 1000; run++)
	{
		long support = random_index(), pathfinding = random_index(), last = random_index();
		objects[2].biped.support_surface_index = support;
		objects[2].biped.pathfinding_surface_index = pathfinding;
		objects[2].biped.last_pathfinding_surface_index = last;
		biped_disconnect_from_structure_bsp(2);
		printf("disconnect %d %d %d -> %d %d %d\n", support, pathfinding, last, objects[2].biped.support_surface_index,
			objects[2].biped.pathfinding_surface_index, objects[2].biped.last_pathfinding_surface_index);
	}
	for (run = 0; run < 1000; run++)
	{
		long biped = next_random(4) == 0 ? NONE : 3;
		char ticks = (char)(next_random(256) - 128);
		objects[3].biped.player_melee_ticks = ticks;
		biped_stop_melee_attack(biped);
		printf("stop_melee %d %d -> %d\n", biped, ticks, objects[3].biped.player_melee_ticks);
	}
	for (run = 0; run < 1000; run++)
	{
		short priority = (short)(next_random(1 << 16) - (1 << 15));
		if (next_random(2))
			priority = (short)(next_random(12) - 2);
		objects[0].unit.speech.current.priority = priority;
		printf("speaking %d -> %d\n", priority, unit_is_speaking(0));
	}
	for (run = 0; run < 1000; run++)
	{
		long equipment = random_index();
		objects[0].unit.equipment_object_index = equipment;
		printf("equipment %d -> %d\n", equipment, unit_get_current_equipment(0));
	}
	printf("relaxation -> %d\n", biped_limp_noodle_get_max_relaxation_iterations(next_random(OBJECTS)));
	return 0;
}
"""

# each function, and the file it is defined in
FUNCTIONS = [
    ("source/units/units.c", "unit_delete"),
    ("source/units/units.c", "units_dispose"),
    ("source/units/units.c", "units_dispose_from_old_map"),
    ("source/units/units.c", "unit_kill_no_statistics"),
    ("source/units/units.c", "unit_kill_silent"),
    ("source/units/units.c", "unit_get_current_equipment"),
    ("source/units/bipeds.c", "biped_delete"),
    ("source/units/bipeds.c", "bipeds_initialize"),
    ("source/units/bipeds.c", "bipeds_initialize_for_new_map"),
    ("source/units/bipeds.c", "bipeds_dispose"),
    ("source/units/bipeds.c", "bipeds_dispose_from_old_map"),
    ("source/units/bipeds.c", "biped_flying_through_air"),
    ("source/units/bipeds.c", "biped_disconnect_from_structure_bsp"),
    ("source/units/bipeds.c", "biped_stop_melee_attack"),
    ("source/units/units.c", "unit_flying_through_air"),
    ("source/units/vehicles.c", "vehicle_delete"),
    ("source/units/vehicles.c", "vehicles_initialize"),
    ("source/units/vehicles.c", "vehicles_initialize_for_new_map"),
    ("source/units/vehicles.c", "vehicles_dispose"),
    ("source/units/vehicles.c", "vehicles_dispose_from_old_map"),
    ("source/units/unit_dialogue.c", "unit_is_speaking"),
    ("source/units/biped_limp_noodle.c", "biped_limp_noodle_get_max_relaxation_iterations"),
]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    files = {}

    def text(path):
        if path not in files:
            files[path] = read(path)
        return files[path]

    enums = [
        enum_with(text("source/objects/objects.h"), "_object_dead_bit"),
        enum_with(text("source/objects/object_types.h"), "_object_type_biped"),
        enum_with(text("source/units/biped_definitions.h"), "_biped_flying_bit"),
    ]
    functions = [function(text(path), name) for path, name in FUNCTIONS]
    return "\n".join([FAKE_WORLD, *enums, WORLD, *functions, DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-life-reference-") as work:
        source = Path(work) / "unit_life_reference.c"
        executable = Path(work) / "unit_life_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit and biped hooks, quiet deaths, flight, surfaces, melee, speech and equipment\n"
        "# (tools/engine_unit_life_reference.py).\n"
        "# hook <function> -> <changed|unchanged> (the fake world's bytes after the call)\n"
        "# kill <silent> <damage flags> -> <damage flags after>\n"
        "# air <type> <damage flags> <definition flies> <airborne ticks> -> <unit flying> [<biped flying>]\n"
        "# disconnect <support> <pathfinding> <last pathfinding> -> <each after>\n"
        "# stop_melee <biped> <melee ticks> -> <melee ticks after>\n"
        "# speaking <priority> -> <speaking>\n"
        "# equipment <equipment> -> <current equipment>\n"
        "# relaxation -> <maximum relaxation iterations>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_life_reference.py")
        print("the unit life reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
