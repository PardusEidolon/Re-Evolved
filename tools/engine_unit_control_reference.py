"""The game's unit control flags, mouth, riders and scripted weapon test,
taken from the sources and run over random units, to record what each
answers, as the native engine's must.

    python3 tools/engine_unit_control_reference.py          # writes the reference
    python3 tools/engine_unit_control_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_controllable`,
`unit_set_controllable`, `unit_set_possessed`,
`unit_set_desired_flashlight_state`, `unit_get_current_flashlight_state`,
`unit_stop_running_blindly`, `unit_set_mouth_aperture`, `unit_driven_by_ai`,
`unit_gunned_by_ai`, `unit_seat_filled`, `unit_is_busy` (with
`unit_animation_busy`) and `unit_has_weapon_definition_index`, and
source/units/unit_scripting_commands.c's `unit_scripting_has_weapon`, found
by name as tools/harness finds them, with units.h's flag and state enums,
and compiled for this machine against a small fake world of six units.

The flags are random words, set or cleared one way at a time, and asked of
a unit or of none. The mouth aperture is moved from a random aperture
towards a random one. Each unit's actor, driver and gunner, and its parent
and seat, are random, and the seat asked of may be one nobody is in. Every
animation state (and a few past them) is asked whether it is busy. The
weapons' definitions are random, and the unit and definition asked of may
be none.

Floats are printed as their bits, so that the engine's must be the same
float, not only print the same.

The trace is written to tools/traces/unit_control_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_control_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

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
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))

#define MAXIMUM_WEAPONS_PER_UNIT 4
#define UNITS 6
#define _object_mask_unit 3
"""

WORLD = r"""
struct unit_animation { short state; };
struct unit_datum
{
	struct { long parent_object_index; } object;
	struct
	{
		unsigned long flags;
		long actor_index;
		long driver_object_index;
		long gunner_object_index;
		short parent_seat_index;
		real mouth_aperture;
		struct unit_animation animation;
		long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT];
	} unit;
};
struct weapon_datum { long definition_index; };

static struct unit_datum units[UNITS];
static struct weapon_datum weapons[8];

static struct unit_datum *unit_get(long unit_index) { return &units[unit_index]; }
static struct weapon_datum *weapon_get(long weapon_index) { return &weapons[weapon_index]; }

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
"""

DRIVER = r"""
static unsigned long seed = 0xc0e7a1;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

/* a float in [low, high), at one of 4096 steps */
static real random_real(real low, real high)
{
	return low + (high - low) * (real)next_random(4096) / 4096.f;
}

static unsigned bits(real value)
{
	unsigned result;
	memcpy(&result, &value, sizeof result);
	return result;
}

int main(void)
{
	long run;
	long index;

	for (run = 0; run < 3000; run++)
	{
		long way = next_random(5);
		boolean value = (boolean)next_random(2);
		long unit = next_random(4) == 0 ? NONE : 0;
		unsigned long flags = ((unsigned long)next_random(1 << 15) << 15) | (unsigned long)next_random(1 << 15);
		units[0].unit.flags = flags;
		switch (way)
		{
		case 0: unit_set_controllable(0, value); break;
		case 1: unit_set_possessed(0, value); break;
		case 2: unit_set_desired_flashlight_state(unit, value); break;
		case 3: unit_stop_running_blindly(0); break;
		default: break;
		}
		printf("flags %u %d %d %d -> %u %d %d\n", flags, way, value, unit, units[0].unit.flags,
			unit_controllable(0), unit_get_current_flashlight_state(unit));
	}
	for (run = 0; run < 2000; run++)
	{
		real current = random_real(-1.f, 2.f);
		real wanted = next_random(8) == 0 ? current : random_real(-1.f, 2.f);
		units[0].unit.mouth_aperture = current;
		unit_set_mouth_aperture(0, wanted);
		printf("mouth %u %u -> %u\n", bits(current), bits(wanted), bits(units[0].unit.mouth_aperture));
	}
	for (run = 0; run < 2000; run++)
	{
		printf("riders |");
		for (index = 0; index < UNITS; index++)
		{
			units[index].unit.actor_index = next_random(2) ? NONE : next_random(20);
			units[index].unit.driver_object_index = next_random(2) ? NONE : next_random(UNITS);
			units[index].unit.gunner_object_index = next_random(2) ? NONE : next_random(UNITS);
			printf(" %d:%d:%d", units[index].unit.actor_index, units[index].unit.driver_object_index,
				units[index].unit.gunner_object_index);
		}
		printf(" ->");
		for (index = 0; index < UNITS; index++)
			printf(" %d%d", unit_driven_by_ai(index), unit_gunned_by_ai(index));
		printf("\n");
	}
	for (run = 0; run < 2000; run++)
	{
		long unit = next_random(UNITS + 1) - 1;
		short seat = (short)(next_random(5) - 1);
		printf("seat_filled %d %d |", unit, seat);
		for (index = 0; index < UNITS; index++)
		{
			units[index].object.parent_object_index = next_random(UNITS + 1) - 1;
			units[index].unit.parent_seat_index = (short)(next_random(5) - 1);
			printf(" %d:%d", units[index].object.parent_object_index, units[index].unit.parent_seat_index);
		}
		printf(" -> %d\n", unit_seat_filled(unit, seat));
	}
	for (index = -2; index < 40; index++)
	{
		units[0].unit.animation.state = (short)index;
		printf("busy %d -> %d\n", index, unit_is_busy(0));
	}
	for (run = 0; run < 2000; run++)
	{
		long unit = next_random(5) == 0 ? NONE : 0;
		long definition = next_random(6) - 1;
		printf("scripting_has_weapon %d %d |", unit, definition);
		for (index = 0; index < 8; index++)
			weapons[index].definition_index = next_random(5);
		for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		{
			long weapon = next_random(3) == 0 ? NONE : next_random(8);
			units[0].unit.weapon_object_indices[index] = weapon;
			printf(" %d", weapon == NONE ? NONE : weapons[weapon].definition_index);
		}
		printf(" -> %d\n", unit_scripting_has_weapon(unit, definition));
	}
	return 0;
}
"""

# each function, and the file it is defined in
FUNCTIONS = [
    ("source/units/units.c", "unit_animation_busy"),
    ("source/units/units.c", "unit_controllable"),
    ("source/units/units.c", "unit_set_controllable"),
    ("source/units/units.c", "unit_set_possessed"),
    ("source/units/units.c", "unit_set_desired_flashlight_state"),
    ("source/units/units.c", "unit_get_current_flashlight_state"),
    ("source/units/units.c", "unit_stop_running_blindly"),
    ("source/units/units.c", "unit_set_mouth_aperture"),
    ("source/units/units.c", "unit_driven_by_ai"),
    ("source/units/units.c", "unit_gunned_by_ai"),
    ("source/units/units.c", "unit_seat_filled"),
    ("source/units/units.c", "unit_is_busy"),
    ("source/units/units.c", "unit_has_weapon_definition_index"),
    ("source/units/unit_scripting_commands.c", "unit_scripting_has_weapon"),
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
    with tempfile.TemporaryDirectory(prefix="unit-control-reference-") as work:
        source = Path(work) / "unit_control_reference.c"
        executable = Path(work) / "unit_control_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit control flags, mouth, riders and scripted weapon test (tools/engine_unit_control_reference.py).\n"
        "# flags <flags> <way> <value> <unit> -> <flags after> <controllable> <flashlight on>\n"
        "#   (way: 0 set controllable, 1 set possessed, 2 desire the flashlight, 3 stop running blindly, 4 none)\n"
        "# mouth <aperture bits> <wanted bits> -> <aperture bits after>\n"
        "# riders | <actor>:<driver>:<gunner> of each unit... -> <driven by ai><gunned by ai> of each unit...\n"
        "# seat_filled <unit> <seat> | <parent>:<seat> of each unit... -> <filled>\n"
        "# busy <state> -> <busy>\n"
        "# scripting_has_weapon <unit> <definition> | <definition in each slot>... -> <has>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_control_reference.py")
        print("the unit control reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
