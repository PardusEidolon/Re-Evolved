"""The game's unit running blindly and burning to death, taken from the
sources and run over random units, to record what it does to the world,
what it draws from the game's seed, the way it runs and what it leaves in
the unit, as the native engine's must.

    python3 tools/engine_unit_flaming_reference.py          # writes the reference
    python3 tools/engine_unit_flaming_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_start_running_blindly`,
`unit_running_blind`, `unit_start_flaming_to_death` and
`unit_flame_to_death`, with the helpers they call (real_math's
`euler_angles2d_from_vector3d` and damage.c's `damage_data_new`), found by
name as tools/harness finds them. The game's seed, its helpers and the
maths (`real_seed_random_range`, `seed_random_range`, `sine`, `cosine`,
`arctangent` and `rotate_vector_about_axis`) are
tools/engine_random_reference.py's, so that what is drawn is the game's.
They are compiled for this machine against a fake world of the unit (0)
and seven objects that may set it burning, each there or not, with owners
or none.

What the functions ask of the rest of the game (the actor's way, the
game globals' flaming death, an attacker's owners) is answered from that
world, and what they do to it (drop the weapon in hand, deal the damage,
which kills the unit or not) is written down in order.

Asserts and the game's silent error are compiled out, as in the released
game.

The trace is written to tools/traces/unit_flaming_reference.txt, which
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
import engine_random_reference as random_reference  # noqa: E402
from harness import enum_with, function, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "unit_flaming_reference.txt"

# the random trace's fake world, with no game engine running
SEEDED_WORLD = r"""
/* string.h can't be read after the math world makes long an int */
static void clear(void *data, unsigned size)
{
	char *byte = data;
	while (size--)
		*byte++ = 0;
}
typedef unsigned short word;
/* the C library has a random() of its own */
#define random game_random
#define match_dassert(file, line, condition, message) ((void)0)

static boolean game_engine_running(void)
{
	return FALSE;
}
"""

WORLD = r"""
#define match_assert(file, line, condition) ((void)0);
#define match_assert_valid_real_normal3d(file, line, vector) ((void)0);
#define error(...) ((void)0)
#define csmemset(data, value, size) clear(data, size)
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
typedef unsigned char byte;

#define OBJECTS 8

/* a real as its bits, for what the functions leave and call with */
static unsigned bits(real x)
{
	union { real r; unsigned u; } value;
	value.r = x;
	return value.u;
}

static real_vector3d const forward_value = { 1.f, 0.f, 0.f };
static real_vector3d const up_value = { 0.f, 0.f, 1.f };
static real_vector3d const *global_forward3d = &forward_value;
static real_vector3d const *global_up3d = &up_value;

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) \
	((index) >= 0 && (index) < (block)->count ? (type *)(block)->address + (index) : (type *)NULL)

struct location { long leaf_index; short cluster_index; };

struct damage_data
{
	long definition_index;
	unsigned long flags;
	long owner_player_index;
	long owner_object_index;
	short owner_team_index;
	struct location location;
	real_point3d origin;
	real_point3d epicenter;
	real_vector3d direction;
	real scale;
	real multiplier;
	real material_effect_scale;
	short material_type;
	void const *material_response;
};

struct object_datum
{
	long definition_index;
	struct
	{
		real_vector3d forward;
		unsigned long damage_flags;
		long owner_player_index;
		long owner_object_index;
		short owner_team_index;
	} object;
	boolean exists;
};

struct unit_datum
{
	long definition_index;
	struct
	{
		real_vector3d forward;
		unsigned long damage_flags;
	} object;
	struct
	{
		long actor_index;
		unsigned long flags;
		real run_blindly_angle;
		real run_blindly_angle_delta;
		byte flaming_death_delay;
		long flaming_death_attacker_object_index;
	} unit;
};

struct game_globals_falling_damage { struct { long index; } flaming_death_damage_effect; };
struct game_globals { struct tag_block falling_damage; };

static struct unit_datum the_unit;
static struct object_datum objects[OBJECTS];
static struct game_globals the_globals;
static struct game_globals_falling_damage falling_damage;
static boolean actor_has_way, damage_kills;
static real_vector3d actor_way;

static struct unit_datum *unit_get(long index)
{
	return &the_unit;
}

static struct object_datum *object_try_and_get(long index)
{
	return index > 0 && index < OBJECTS && objects[index].exists ? &objects[index] : NULL;
}

static struct game_globals *scenario_get_game_globals(void)
{
	return &the_globals;
}

static boolean actor_get_running_blind_vector(long actor_index, real_vector3d *run_vector)
{
	if (actor_has_way)
		*run_vector = actor_way;
	return actor_has_way;
}

static void unit_drop_current_weapon(long unit_index, boolean delete)
{
	printf(" drop_current_weapon %d %d", unit_index, delete);
}

static void object_cause_damage(struct damage_data *damage, long object_index, short node_index,
	short region_index, short material_index, real_vector3d const *normal)
{
	printf(" damage %d %u %d %d %d %d %08x %08x %08x %d %d %d %d %d %d", damage->definition_index, damage->flags,
		damage->owner_player_index, damage->owner_object_index, damage->owner_team_index,
		damage->location.cluster_index, bits(damage->scale), bits(damage->multiplier),
		bits(damage->material_effect_scale), damage->material_type, object_index, node_index, region_index,
		material_index, normal != NULL);
	if (damage_kills)
		SET_FLAG(the_unit.object.damage_flags, _object_dead_bit, TRUE);
}
"""

DRIVER = r"""
static unsigned long cases = 0x5b3e;

static long next_case(long n)
{
	cases = cases * 1103515245u + 12345u;
	return (long)((cases >> 8) % (unsigned long)n);
}

/* a component at random: often a whole number, sometimes zero */
static real next_component(void)
{
	long kind = next_case(4);
	return kind == 0 ? 0.f : kind == 1 ? (real)(next_case(3) - 1) : (real)(next_case(2001) - 1000) * 0.001f;
}

static real_vector3d next_vector(void)
{
	real_vector3d v;
	v.i = next_component();
	v.j = next_component();
	v.k = next_component();
	return v;
}

static void print_vector(real_vector3d const *v)
{
	printf(" %.9g %.9g %.9g", v->i, v->j, v->k);
}

static long next_index(void)
{
	return next_case(4) == 0 ? NONE : 1 + (long)next_case(OBJECTS - 1);
}

/* the unit, printed as
flags damage_flags actor has_way <way> <forward> angle delta delay attacker */
static void random_unit(void)
{
	clear(&the_unit, sizeof the_unit);
	the_unit.unit.flags = (unsigned long)next_case(2) << _unit_running_blindly_bit
		| (unsigned long)next_case(2) << _unit_ignore_hard_pings_bit | (unsigned long)next_case(2) << 3;
	the_unit.object.damage_flags = (unsigned long)next_case(2) << _object_dead_bit
		| (unsigned long)next_case(2) << _object_die_act_of_god_bit
		| (unsigned long)next_case(2) << _object_cannot_take_damage_bit | (unsigned long)next_case(2);
	the_unit.unit.actor_index = next_case(3) == 0 ? NONE : 5;
	actor_has_way = next_case(2);
	do
	{
		actor_way = next_vector();
	} while (actor_has_way && normalize3d(&actor_way) == 0.f);
	the_unit.object.forward = next_vector();
	the_unit.unit.run_blindly_angle = next_case(6) == 0 ? 0.f : (real)(next_case(8001) - 4000) * 0.001f;
	the_unit.unit.run_blindly_angle_delta = next_case(6) == 0 ? 0.f : (real)(next_case(801) - 400) * 0.001f;
	the_unit.unit.flaming_death_delay = next_case(2) == 0 ? 0 : (byte)next_case(256);
	the_unit.unit.flaming_death_attacker_object_index = next_index();
	printf(" %u %u %d %d", the_unit.unit.flags, the_unit.object.damage_flags, the_unit.unit.actor_index,
		actor_has_way);
	print_vector(&actor_way);
	print_vector(&the_unit.object.forward);
	printf(" %.9g %.9g %d %d", the_unit.unit.run_blindly_angle, the_unit.unit.run_blindly_angle_delta,
		the_unit.unit.flaming_death_delay, the_unit.unit.flaming_death_attacker_object_index);
}

/* objects 1 to 7, each printed as exists:player:object:team */
static void random_world(void)
{
	long index;
	clear(objects, sizeof objects);
	for (index = 1; index < OBJECTS; index++)
	{
		struct object_datum *object = &objects[index];
		object->exists = next_case(4) != 0;
		object->object.owner_player_index = next_case(3) == 0 ? NONE : (long)next_case(16);
		object->object.owner_object_index = next_case(2) == 0 ? NONE : (long)next_case(OBJECTS);
		object->object.owner_team_index = next_case(3) == 0 ? NONE : (short)next_case(10);
		printf(" %d:%d:%d:%d", object->exists, object->object.owner_player_index,
			object->object.owner_object_index, object->object.owner_team_index);
	}
}

/* the rest of the game, printed as falling_damage_count effect kills seed */
static void random_game(void)
{
	the_globals.falling_damage.count = next_case(5) == 0 ? 0 : 1;
	the_globals.falling_damage.address = &falling_damage;
	falling_damage.flaming_death_damage_effect.index = next_case(5) == 0 ? NONE : 80;
	damage_kills = next_case(2);
	set_random_seed(next_case(1 << 24) * 61 + 1);
	printf(" | %d %d %d %u |", the_globals.falling_damage.count, falling_damage.flaming_death_damage_effect.index,
		damage_kills, get_random_seed());
}

static void print_unit(void)
{
	printf(" %u %u %08x %08x %d %d %u\n", the_unit.unit.flags, the_unit.object.damage_flags,
		bits(the_unit.unit.run_blindly_angle), bits(the_unit.unit.run_blindly_angle_delta),
		the_unit.unit.flaming_death_delay, the_unit.unit.flaming_death_attacker_object_index, get_random_seed());
}

int main(void)
{
	long run;

	for (run = 0; run < 10000; run++)
	{
		long which = next_case(4);
		long attacker = next_index();
		real_vector3d way = { 0.f, 0.f, 0.f };
		char const *name = which == 0 ? "start_running_blindly"
			: which == 1 ? "running_blind"
			: which == 2 ? "start_flaming_to_death"
			: "flame_to_death";
		printf("%s %d |", name, attacker);
		random_unit();
		printf(" |");
		random_world();
		random_game();
		switch (which)
		{
		case 0:
			unit_start_running_blindly(0);
			break;
		case 1:
			unit_running_blind(0, &way);
			break;
		case 2:
			unit_start_flaming_to_death(0, attacker);
			break;
		default:
			unit_flame_to_death(0);
			break;
		}
		printf(" -> %08x %08x %08x |", bits(way.i), bits(way.j), bits(way.k));
		print_unit();
	}
	return 0;
}
"""

ENUMS = [
    ("source/objects/objects.h", "_object_dead_bit"),
    ("source/units/units.h", "_unit_ignore_hard_pings_bit"),
]

FUNCTIONS = [
    "unit_start_running_blindly",
    "unit_running_blind",
    "unit_start_flaming_to_death",
    "unit_flame_to_death",
]


def source_file():
    """The game's seed, its code and the fake world it runs in, as one C file."""
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
    real_math_h = text("source/math/real_math.h")
    helpers = [
        # the header's own degrees, as the functions reckon them
        next(line for line in real_math_h.splitlines() if line.startswith("#define DEGREES_TO_RADIANS")),
        function(text("source/math/real_math.c"), "euler_angles2d_from_vector3d"),
    ]
    units = text("source/units/units.c")
    functions = [function(units, name) for name in FUNCTIONS]
    seeded = random_reference.seeded_source(SEEDED_WORLD)
    damage = function(text("source/objects/damage.c"), "damage_data_new")
    return "\n".join([seeded, *helpers, *enums, WORLD, damage, *functions, DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-flaming-reference-") as work:
        source = Path(work) / "unit_flaming_reference.c"
        executable = Path(work) / "unit_flaming_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit running blindly and burning (tools/engine_unit_flaming_reference.py).\n"
        "# <function> <attacker> | <unit> | <objects 1-7> | <game> | <calls>... -> <way> | <unit after>\n"
        "# unit: flags damage_flags actor has_way <way> <forward> angle delta delay attacker\n"
        "# object: exists:player:object:team\n"
        "# game: falling_damage_count effect kills seed\n"
        "# calls' reals, the way run and the angles after are their bits\n"
        "# unit after: flags damage_flags angle delta delay attacker seed\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_flaming_reference.py")
        print("the unit flaming reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
