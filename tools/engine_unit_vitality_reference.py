"""The game's script commands for a unit's vitality and its animation
lists, taken from the sources and run over random units, to record what
they leave, as the native engine's must.

    python3 tools/engine_unit_vitality_reference.py          # writes the reference
    python3 tools/engine_unit_vitality_reference.py --check  # fails if it is stale

The functions are source/units/unit_scripting_commands.c's
`unit_scripting_set_maximum_vitality`, `unit_scripting_set_current_vitality_of`,
`unit_scripting_get_health`, `unit_scripting_get_shield` and
`unit_scripting_start_user_animation_list`, with source/objects/damage.c's
`object_initialize_vitality`, `object_deplete_shield` and
`object_deplete_body`, found by name as tools/harness finds them and
compiled for this machine against a small fake world of unit datums. The
effects the depletes start, the shield regions and a vehicle's riders are
the object's own and left out; no unit here has riders. Units are drawn at
random: living and dead, depleted or not, with maximums of nothing, below
nothing and above, and vitality set to below nothing, nothing, its maximum
and between. An animation list is a few objects, some of them not units,
each of which would start its animation or not.

The trace is written to tools/traces/unit_vitality_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_vitality_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int
#define __inline static inline

typedef float real;
typedef unsigned short word;
typedef unsigned char boolean;
#define TRUE 1
#define FALSE 0
#define NONE -1

#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))

enum
{
	_object_dead_bit = 2,
	_object_shield_depleted_bit = 3,
};

enum
{
	_object_type_biped = 0,
	_object_type_vehicle,
	_object_type_scenery = 6,
};

struct object_datum
{
	long definition_index;
	struct
	{
		short type;
		word damage_flags;
		real maximum_body_vitality;
		real maximum_shield_vitality;
		real body_vitality;
		real shield_vitality;
		real current_shield_damage;
		long first_child_object_index;
		long next_object_index;
	} object;
	struct
	{
		short player_index;
		short parent_seat_index;
	} unit;
};
#define unit_datum object_datum

struct collision_model
{
	struct
	{
		real maximum_body_vitality;
		real maximum_shield_vitality;
		struct { long index; } body_depleted_effect;
		struct { long index; } shield_depleted_effect;
	} resistance;
};

struct object_definition
{
	struct
	{
		struct { long index; } collision_model;
	} object;
};

static struct { boolean deathless_player; } cheat;

#define MAXIMUM_OBJECTS 8
static struct object_datum objects[MAXIMUM_OBJECTS];
static struct object_definition definition = { { { NONE } } };

static struct object_datum *object_get(long object_index) { return &objects[object_index]; }
static struct object_datum *object_try_and_get(long object_index)
{
	return object_index >= 0 && object_index < MAXIMUM_OBJECTS ? &objects[object_index] : NULL;
}
static struct object_datum *unit_try_and_get(long object_index)
{
	struct object_datum *object = object_try_and_get(object_index);
	return object && object->object.type <= _object_type_vehicle ? object : NULL;
}
#define unit_get object_get
static struct object_definition *object_definition_get(long definition_index) { return &definition; }
static struct collision_model *collision_model_definition_get(long index) { return NULL; }
static void damage_effect_new_on_object(long effect, long object_index) {}
static void unit_kill(long unit_index) {}
static void object_permutation_shield_regions(long object_index, boolean on) {}
static boolean network_coop_set_players_vitality(long unit_index, boolean maximum, real body, real shield)
{
	return FALSE;
}
void object_deplete_shield(long object_index);
void object_deplete_body(long object_index);
void unit_scripting_set_current_vitality_of(long unit_index, real body_vitality, real shield_vitality);

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

/* whether each object would start its animation, and which were asked */
static boolean would_start[MAXIMUM_OBJECTS];
static unsigned asked;
static boolean unit_start_user_animation(long unit_index, long graph, char const *name, boolean interpolate)
{
	asked |= 1u << unit_index;
	return would_start[unit_index];
}
"""

DRIVER = r"""
static unsigned long seed = 0x7a11;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

/* a random real in -range..range, in thousandths of it */
static real next_signed(real range)
{
	return range * (real)(next_random(2001) - 1000) / 1000.f;
}

static unsigned bits(real x)
{
	unsigned u;
	memcpy(&u, &x, sizeof u);
	return u;
}

static real next_maximum(void)
{
	switch (next_random(6))
	{
	case 0: return 0.f;
	case 1: return -next_signed(50.f) - 51.f;
	default: return next_signed(100.f) + 101.f;
	}
}

static real next_fraction(void)
{
	switch (next_random(6))
	{
	case 0: return 0.f;
	case 1: return 1.f;
	case 2: return -next_signed(0.5f) - 0.5f;
	default: return next_signed(0.6f) + 0.6f;
	}
}

/* an amount a script would set, against a maximum */
static real next_amount(real maximum)
{
	switch (next_random(6))
	{
	case 0: return 0.f;
	case 1: return maximum;
	case 2: return -next_signed(20.f) - 21.f;
	default: return next_signed(150.f) + 150.f;
	}
}

static void print_unit(struct object_datum const *unit)
{
	printf(" %d %d %08x %08x %08x %08x",
		TEST_FLAG(unit->object.damage_flags, _object_dead_bit),
		TEST_FLAG(unit->object.damage_flags, _object_shield_depleted_bit),
		bits(unit->object.maximum_body_vitality), bits(unit->object.maximum_shield_vitality),
		bits(unit->object.body_vitality), bits(unit->object.shield_vitality));
}

int main(void)
{
	long run;

	for (run = 0; run < 6000; run++)
	{
		struct object_datum *unit = &objects[0];
		long command = next_random(4);
		real body, shield;

		memset(unit, 0, sizeof *unit);
		unit->object.type = (short)next_random(2);
		unit->object.first_child_object_index = NONE;
		unit->object.next_object_index = NONE;
		unit->unit.player_index = NONE;
		unit->unit.parent_seat_index = NONE;
		if (next_random(6) == 0)
			SET_FLAG(unit->object.damage_flags, _object_dead_bit, TRUE);
		if (next_random(4) == 0)
			SET_FLAG(unit->object.damage_flags, _object_shield_depleted_bit, TRUE);
		unit->object.maximum_body_vitality = next_maximum();
		unit->object.maximum_shield_vitality = next_maximum();
		unit->object.body_vitality = next_fraction();
		unit->object.shield_vitality = next_fraction();
		switch (command)
		{
		case 0:
			body = next_amount(unit->object.maximum_body_vitality);
			shield = next_amount(unit->object.maximum_shield_vitality);
			printf("current %08x %08x", bits(body), bits(shield));
			print_unit(unit);
			unit_scripting_set_current_vitality_of(0, body, shield);
			print_unit(unit);
			break;
		case 1:
			body = next_maximum();
			shield = next_maximum();
			printf("maximum %08x %08x", bits(body), bits(shield));
			print_unit(unit);
			unit_scripting_set_maximum_vitality(0, body, shield);
			print_unit(unit);
			break;
		case 2:
			printf("health");
			print_unit(unit);
			printf(" %08x", bits(unit_scripting_get_health(0)));
			break;
		default:
			printf("shield");
			print_unit(unit);
			printf(" %08x", bits(unit_scripting_get_shield(0)));
			break;
		}
		printf("\n");
	}
	for (run = 0; run < 3000; run++)
	{
		long index;
		boolean result;

		list_count = next_random(7);
		asked = 0;
		printf("list");
		for (index = 0; index < list_count; index++)
		{
			objects[index].object.type = next_random(5) == 0 ? _object_type_scenery : (short)next_random(2);
			would_start[index] = next_random(5) != 0;
			list[index] = index;
			printf(" %d:%d", objects[index].object.type <= _object_type_vehicle, would_start[index]);
		}
		result = unit_scripting_start_user_animation_list(0, 0, "", FALSE);
		printf(" -> %x %d\n", asked, result);
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    damage = read("source/objects/damage.c")
    commands = read("source/units/unit_scripting_commands.c")
    return "\n".join(
        [FAKE_WORLD]
        + [
            function(damage, name)
            for name in ("object_initialize_vitality", "object_deplete_body", "object_deplete_shield")
        ]
        + [
            function(commands, name)
            for name in (
                "unit_scripting_set_maximum_vitality",
                "unit_scripting_set_current_vitality_of",
                "unit_scripting_get_health",
                "unit_scripting_get_shield",
                "unit_scripting_start_user_animation_list",
            )
        ]
        + [DRIVER]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-vitality-reference-") as work:
        source = Path(work) / "unit_vitality_reference.c"
        executable = Path(work) / "unit_vitality_reference"
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
        "# The game's unit vitality commands (tools/engine_unit_vitality_reference.py), f32 bits.\n"
        "# A unit is <dead> <shield depleted> <maximum body> <maximum shield> <body> <shield>.\n"
        "# current|maximum <body> <shield> <unit before> <unit after>\n"
        "# health|shield <unit> <answer>\n"
        "# list <is a unit:would start>... -> <mask of those asked> <answer>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_vitality_reference.py")
        print("the unit vitality reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
