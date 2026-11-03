"""The game's damage to one object's shield and body, taken from the
sources and run over random hits, to record what it leaves, as the native
engine's `cause_damage` must.

    python3 tools/engine_cause_damage_reference.py          # writes the reference
    python3 tools/engine_cause_damage_reference.py --check  # fails if it is stale

The functions are source/objects/damage.c's `object_cause_damage`, with its
`object_damage_shield`, `object_damage_body` and `damage_data_validate_owner`,
found by name as tools/harness finds them and compiled for this machine with
source/math/periodic_functions.c's transitions (as
tools/engine_periodic_reference.py compiles them). The object is a biped of
no player and no parent, struck directly, in the campaign, with no game
engine running, no cheats and every difficulty value 1; the world around it
(its effects, the network, the statistics and the aftermath) is stubbed out.
The hits are drawn at random: shields whole, failing and empty, heads and
bodies, friends and foes, the unsuspecting, the side effects, and the
damage effect's and the resistance's flags that the engine reads.

The trace is written to tools/traces/cause_damage_reference.txt, which
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
import engine_periodic_reference as periodic  # noqa: E402
from harness import function, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "cause_damage_reference.txt"

FAKE_WORLD = r"""

#define NONE (-1)
#define MIN(a, b) ((a)<(b) ? (a) : (b))
#define NUMBEROF(array) (sizeof(array)/sizeof((array)[0]))
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define match_vassert(file, line, condition, message) ((void)0)
#define _real_epsilon 0.0001f
#define TICKS_PER_SECOND 30
#define MAXIMUM_REGIONS_PER_OBJECT 8
#define NUMBER_OF_MATERIAL_TYPES 33
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])
#define unit_datum object_datum

typedef union { struct { real x, y, z; }; real n[3]; } real_point3d;
typedef union { struct { real i, j, k; }; real n[3]; } real_vector3d;
struct location { long cluster_reference; short leaf_index; };
struct tag_block { long count; void *address; };
struct tag_reference { long index; };

/* the damage effect's flags, at the bits the engine reads them from */
enum
{
	_damage_can_cause_headshots_bit = 1,
	_damage_detonates_explosives_bit = 5,
	_damage_only_hurts_shields_bit = 6,
	_damage_skips_shields_bit = 9,
	_damage_can_cause_multiplayer_headshots_bit = 11,
};

/* a hit's own flags */
enum
{
	_damage_area_of_effect_bit,
	_damage_kill_instantly_bit,
	_damage_bypasses_shields_bit,
	_damage_create_localized_effect_bit,
	_damage_no_statistics_bit,
};

/* the resistance's, at the engine's bits */
enum
{
	_damage_resistance_takes_shield_damage_for_children_bit = 0,
	_damage_resistance_takes_body_damage_for_children_bit = 1,
	_damage_resistance_always_shields_friendly_damage_bit = 2,
	_damage_resistance_parent_never_takes_body_damage_for_us_bit = 3,
	_damage_resistance_only_hurt_while_occupied_bit = 4,
	_damage_resistance_only_hurt_by_explosives_bit = 5,
};

enum { _damage_material_head_bit = 0 };
enum { _object_region_dies_when_object_dies_bit = 0 };

enum
{
	_object_shield_depleted_bit,
	_object_dead_bit,
	_object_cannot_take_damage_bit,
	_object_shield_over_charging_bit,
	_object_passed_shield_damage_threshold_bit,
	_object_passed_body_damage_threshold_bit,
};

enum
{
	_object_being_damaged_multiplied_by_difficulty_bit,
	_object_being_damaged_by_friendly_bit,
	_object_being_damaged_body_depleted_bit,
	_object_being_damaged_killed_instantly_bit,
	_object_being_damaged_shield_depleted_bit,
	_object_being_damaged_region_destroyed_bit,
	_object_being_damaged_body_destroyed_bit,
	_object_being_damaged_force_hard_ping_bit,
};

enum
{
	_damage_category_none,
	_damage_category_falling,
	_damage_category_grenade,
	_damage_category_highexplosive,
	_damage_category_flame,
};

enum
{
	_damage_side_effect_none,
	_damage_side_effect_harmless,
	_damage_side_effect_lethal_to_the_unsuspecting,
	_damage_side_effect_emp,
};

enum
{
	_object_type_biped,
	_object_type_vehicle,
	_object_type_weapon,
	_object_type_equipment,
	_object_type_garbage,
	_object_type_projectile,
	_object_type_scenery,
};
#define _object_mask_unit (FLAG(_object_type_biped) | FLAG(_object_type_vehicle))

enum { _friendly_damage_none, _friendly_damage_shields, _friendly_damage_all };
enum { _game_team_player = 1, _game_team_covenant = 3 };
enum { _game_difficulty_value_enemy_damage };
enum { _error_silent };

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

struct damage_resistance_material
{
	char name[32];
	unsigned long flags;
	short material_type;
	real shield_leak_fraction;
	real shield_damage_multiplier;
	real body_damage_multiplier;
};

struct damage_region
{
	char name[32];
	unsigned long flags;
	real damage_threshold;
};

struct damage_resistance
{
	unsigned long flags;
	short indirect_damage_material_index;
	real friendly_damage_resistance;
	real body_damaged_effect_threshold;
	struct tag_reference body_damaged_effect;
	real body_destroyed_threshold;
	struct tag_reference localized_damage_effect;
	real area_damage_effect_threshold;
	struct tag_reference area_damage_effect;
	short shield_material_type;
	short shield_failure_function;
	real shield_failure_threshold;
	real maximum_shield_failure;
	real minimum_shield_stun_damage;
	real shield_stun_time;
	real shield_damaged_effect_threshold;
	struct tag_reference shield_damaged_effect;
	struct tag_block materials;
	struct tag_block regions;
};

struct collision_node { short runtime_damage_part; };

struct collision_model
{
	struct damage_resistance resistance;
	struct tag_block nodes;
};

struct damage_definition
{
	short side_effect;
	short category;
	unsigned long flags;
	real damage_lower_bound;
	real damage_upper_bound;
	real damage_minimum;
	real vehicle_passthrough_penalty;
	real instantaneous_acceleration;
	real material_modifiers[NUMBER_OF_MATERIAL_TYPES];
};

struct damage_effect_definition { struct damage_definition damage; };

struct object_definition
{
	struct { struct tag_reference collision_model; real acceleration_scale; } object;
};

struct unit_definition { struct { real child_damage_fraction; } unit; };

struct object_datum
{
	long definition_index;
	struct
	{
		short type;
		short owner_team_index;
		unsigned long damage_flags;
		real maximum_body_vitality;
		real maximum_shield_vitality;
		real body_vitality;
		real shield_vitality;
		short shield_stun_ticks;
		short shield_damage_decay_timer;
		short body_damage_decay_timer;
		real current_shield_damage;
		real recent_shield_damage;
		real current_body_damage;
		real recent_body_damage;
		byte region_damage[MAXIMUM_REGIONS_PER_OBJECT];
		unsigned long regions_destroyed_flags;
		long parent_object_index;
		long umbrella_shield_object_index;
		long first_child_object_index;
		long next_object_index;
		long definition_index;
	} object;
	struct
	{
		long player_index;
		long driver_object_index;
		long gunner_object_index;
		long swarm_actor_index;
		long actor_index;
		unsigned long flags;
	} unit;
};

static struct
{
	boolean omnipotent;
	boolean reflexive_damage_effects;
	boolean deathless_player;
} cheat;

static boolean debug_damage;
static long global_debug_damage_object_index;
static boolean distributed_damage_authorized;

static struct object_datum the_object;
static struct object_definition the_definition;
static struct unit_definition the_unit_definition;
static struct collision_model the_model;
static struct damage_effect_definition the_effect;
static struct damage_resistance_material the_materials[2];
static boolean the_unsuspecting;
static boolean the_object_destroyed;

static struct object_datum *object_get(long object_index) { (void)object_index; return &the_object; }
static struct object_datum *unit_get(long object_index) { return object_get(object_index); }
static struct object_datum *unit_try_and_get(long object_index)
{
	return object_index != NONE && TEST_FLAG(_object_mask_unit, the_object.object.type) ? &the_object : NULL;
}
static struct object_definition *object_definition_get(long index) { (void)index; return &the_definition; }
static struct unit_definition *unit_definition_get(long index) { (void)index; return &the_unit_definition; }
static struct collision_model const *collision_model_definition_get(long index) { (void)index; return &the_model; }
static struct damage_effect_definition *damage_effect_definition_get(long index) { (void)index; return &the_effect; }
static void *player_try_and_get(long index) { (void)index; return NULL; }

static boolean network_damage_deals(struct damage_data *damage, long object_index, short node_index,
	short region_index, short material_index, real_vector3d const *object_normal, boolean authorized)
{
	return TRUE;
}
static void network_damage_player_effect(long player_index, struct damage_data *damage, real total_damage) {}
static boolean network_damage_replaying_kill(void) { return FALSE; }
static void network_damage_aftermath(long object_index, struct damage_data *damage, unsigned long flags,
	real shield_damage, real body_damage, real body_damage_multiplier, short body_part, short node_index,
	short region_index, short material_index, long victim_player_index) {}
static void player_effect_start(long player_index, struct damage_data *damage, real_vector3d const *direction,
	real scale, real total_damage) {}
static long local_player_get_player_index(short index) { return NONE; }
static void ai_adjust_damage(long actor_index, struct damage_data *damage, real *total_damage) {}

/* the damage's range, drawn: the hits draw nothing from it (its bounds
are equal) */
static real real_random_range(real lower, real upper) { return lower; }

static boolean game_engine_running(void) { return FALSE; }
static real game_engine_get_damage_multiplier(long attacker, long victim) { return 1.f; }
static short game_engine_friendly_damage(long player_index, long object_index, boolean explosive)
{
	return _friendly_damage_all;
}
static long get_player_index_from_object_or_parents(long object_index) { return NONE; }
static boolean game_team_is_enemy(short team, short other) { return team != other; }
static real game_difficulty_get_value(short value) { return 1.f; }
static real game_difficulty_get_team_value(short value, short team) { return 1.f; }
static boolean unit_unsuspecting(long object_index, real_point3d const *epicenter) { return the_unsuspecting; }

static void damage_effect_new_on_object(long definition_index, long object_index) {}
static void damage_effect_new_at_location(long definition_index, long object_index, short node_index,
	real_point3d const *position, real_vector3d const *direction, real_vector3d const *normal) {}
static void object_destroy_region(long object_index, short region_index)
{
	SET_FLAG(the_object.object.regions_destroyed_flags, region_index, TRUE);
}
static void object_destroy(long object_index) { the_object_destroyed = TRUE; }
static void object_delete(long object_index) {}
static void damage_material_type_error(short material_type) {}
static char const *tag_get_name(long index) { return "\\fake"; }
static char const *material_get_name(short material_type) { return "fake"; }
static void error(short priority, char const *format, ...) {}
static void console_printf(boolean clear, char const *format, ...) {}

/* damage.c's, as the unit vitality reference traces them: the flag, and
a body's shield with it */
void object_deplete_shield(long object_index)
{
	SET_FLAG(the_object.object.damage_flags, _object_shield_depleted_bit, TRUE);
}

void object_deplete_body(long object_index)
{
	if (!TEST_FLAG(the_object.object.damage_flags, _object_dead_bit))
	{
		SET_FLAG(the_object.object.damage_flags, _object_dead_bit, TRUE);
		object_deplete_shield(object_index);
	}
}

static unsigned long the_flags_after;

static void object_damage_aftermath(long object_index, struct damage_data *damage,
	unsigned long being_damaged_flags, real shield_damage, real body_damage,
	real body_damage_multiplier, short body_part)
{
	the_flags_after = being_damaged_flags;
}

real object_get_maximum_body_vitality(long object_index, boolean ignore_difficulty)
{
	return object_get(object_index)->object.maximum_body_vitality *
		(ignore_difficulty ? 1.f : game_difficulty_get_team_value(1, object_get(object_index)->object.owner_team_index));
}

real object_get_maximum_shield_vitality(long object_index, boolean ignore_difficulty)
{
	return object_get(object_index)->object.maximum_shield_vitality *
		(ignore_difficulty ? 1.f : game_difficulty_get_team_value(2, object_get(object_index)->object.owner_team_index));
}

real object_get_actual_body_vitality(long object_index, boolean ignore_difficulty)
{
	return object_get(object_index)->object.body_vitality *
		object_get_maximum_body_vitality(object_index, ignore_difficulty);
}
"""

DRIVER = r"""
static unsigned long seed = 0x0dab;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

/* a real in 0..range, in thousandths of it */
static real next_real(real range)
{
	return range * (real)next_random(1001) / 1000.f;
}

static unsigned bits(real x)
{
	unsigned u;
	memcpy(&u, &x, sizeof u);
	return u;
}

/* a vitality: whole, empty, or between */
static real next_vitality(void)
{
	switch (next_random(4))
	{
	case 0: return 1.f;
	case 1: return 0.f;
	default: return next_real(1.f);
	}
}

int main(void)
{
	long run;

	periodic_functions_initialize();
	for (run = 0; run < 6000; run++)
	{
		struct damage_data damage;
		struct damage_resistance *resistance = &the_model.resistance;
		struct damage_resistance_material *material = &the_materials[0];
		struct damage_definition *definition = &the_effect.damage;
		short material_index = next_random(6) == 0 ? NONE : 0;
		boolean friendly = next_random(4) == 0;
		short modifier_index;
		real total;

		memset(&the_object, 0, sizeof the_object);
		memset(&the_model, 0, sizeof the_model);
		memset(&the_effect, 0, sizeof the_effect);
		memset(the_materials, 0, sizeof the_materials);
		memset(&damage, 0, sizeof damage);
		the_unsuspecting = next_random(2);
		the_object_destroyed = FALSE;
		the_flags_after = 0;

		the_definition.object.collision_model.index = 0;
		the_object.object.type = _object_type_biped;
		the_object.object.owner_team_index = _game_team_player;
		the_object.object.parent_object_index = NONE;
		the_object.object.umbrella_shield_object_index = NONE;
		the_object.object.first_child_object_index = NONE;
		the_object.unit.player_index = NONE;
		the_object.unit.actor_index = NONE;
		the_object.unit.swarm_actor_index = NONE;
		the_object.unit.gunner_object_index = NONE;
		the_object.object.maximum_body_vitality = next_random(10) == 0 ? 0.f : 10.f + next_real(140.f);
		the_object.object.maximum_shield_vitality = next_random(3) == 0 ? 0.f : 10.f + next_real(140.f);
		the_object.object.body_vitality = the_object.object.maximum_body_vitality > 0.f ? 0.05f + next_real(0.95f) : 0.f;
		the_object.object.shield_vitality = the_object.object.maximum_shield_vitality > 0.f ? next_vitality() : 0.f;
		if (the_object.object.shield_vitality == 0.f && the_object.object.maximum_shield_vitality > 0.f && next_random(2))
			SET_FLAG(the_object.object.damage_flags, _object_shield_depleted_bit, TRUE);
		the_object.object.shield_stun_ticks = -5;

		resistance->flags = next_random(5) == 0 ? FLAG(_damage_resistance_only_hurt_by_explosives_bit) : 0;
		resistance->indirect_damage_material_index = next_random(2) ? 1 : NONE;
		resistance->friendly_damage_resistance = next_real(1.f);
		resistance->body_destroyed_threshold = 0.f;
		resistance->localized_damage_effect.index = NONE;
		resistance->area_damage_effect.index = NONE;
		resistance->shield_material_type = (short)next_random(4);
		resistance->shield_failure_function = (short)next_random(7);
		resistance->shield_failure_threshold = next_random(3) == 0 ? 0.f : next_real(0.6f);
		resistance->maximum_shield_failure = next_real(1.f);
		resistance->minimum_shield_stun_damage = next_real(40.f);
		resistance->shield_stun_time = next_real(3.f);
		resistance->shield_damaged_effect_threshold = 0.f;
		resistance->materials.count = 2;
		resistance->materials.address = the_materials;
		for (modifier_index = 0; modifier_index < 2; modifier_index++)
		{
			struct damage_resistance_material *each = &the_materials[modifier_index];

			each->flags = next_random(4) == 0 ? FLAG(_damage_material_head_bit) : 0;
			each->material_type = (short)(4 + next_random(4));
			each->shield_leak_fraction = next_random(3) == 0 ? next_real(1.f) : 0.f;
			each->shield_damage_multiplier = next_random(4) == 0 ? next_real(3.f) : 1.f;
			each->body_damage_multiplier = next_random(4) == 0 ? next_real(3.f) : 1.f;
		}

		switch (next_random(8))
		{
		case 0: definition->side_effect = _damage_side_effect_lethal_to_the_unsuspecting; break;
		case 1: definition->side_effect = _damage_side_effect_emp; break;
		default: definition->side_effect = _damage_side_effect_none; break;
		}
		definition->category = (short)next_random(5);
		if (next_random(3) == 0) SET_FLAG(definition->flags, _damage_can_cause_headshots_bit, TRUE);
		if (next_random(4) == 0) SET_FLAG(definition->flags, _damage_can_cause_multiplayer_headshots_bit, TRUE);
		if (next_random(4) == 0) SET_FLAG(definition->flags, _damage_detonates_explosives_bit, TRUE);
		if (next_random(8) == 0) SET_FLAG(definition->flags, _damage_only_hurts_shields_bit, TRUE);
		if (next_random(8) == 0) SET_FLAG(definition->flags, _damage_skips_shields_bit, TRUE);
		total = next_random(8) == 0 ? 0.f : next_real(200.f);
		definition->damage_lower_bound = total;
		definition->damage_upper_bound = total;
		definition->damage_minimum = next_real(20.f);
		for (modifier_index = 0; modifier_index < NUMBER_OF_MATERIAL_TYPES; modifier_index++)
			definition->material_modifiers[modifier_index] = next_random(3) == 0 ? next_real(2.f) : 1.f;

		damage.definition_index = 0;
		damage.owner_player_index = NONE;
		damage.owner_object_index = NONE;
		damage.owner_team_index = friendly ? _game_team_player : NONE;
		damage.scale = next_random(3) == 0 ? next_real(1.f) : 1.f;
		damage.multiplier = 1.f;
		damage.material_type = NONE;

		printf("hit %08x %08x %08x %08x %d %08x %08x %d %d",
			bits(the_object.object.maximum_body_vitality), bits(the_object.object.maximum_shield_vitality),
			bits(the_object.object.body_vitality), bits(the_object.object.shield_vitality),
			TEST_FLAG(the_object.object.damage_flags, _object_shield_depleted_bit),
			bits(total), bits(definition->damage_minimum), friendly, the_unsuspecting);
		printf(" | %d %d %d %08x %d %d %08x %08x %08x %08x %08x %d",
			(int)definition->flags, definition->side_effect, definition->category, bits(damage.scale),
			(int)resistance->flags, resistance->shield_material_type,
			bits(resistance->friendly_damage_resistance), bits(resistance->minimum_shield_stun_damage),
			bits(resistance->shield_failure_threshold), bits(resistance->maximum_shield_failure),
			bits(resistance->shield_stun_time), resistance->shield_failure_function);
		printf(" | %d %d", material_index, resistance->indirect_damage_material_index);
		for (modifier_index = 0; modifier_index < 2; modifier_index++)
		{
			struct damage_resistance_material *each = &the_materials[modifier_index];

			printf(" %d %d %08x %08x %08x", (int)each->flags, each->material_type, bits(each->shield_leak_fraction),
				bits(each->shield_damage_multiplier), bits(each->body_damage_multiplier));
		}
		printf(" |");
		for (modifier_index = 0; modifier_index < 8; modifier_index++)
			printf(" %08x", bits(definition->material_modifiers[modifier_index]));

		object_cause_damage(&damage, 0, NONE, NONE, material_index, NULL);

		printf(" -> %08x %08x %d %d %d %d %d %d\n",
			bits(the_object.object.body_vitality), bits(the_object.object.shield_vitality),
			the_object.object.shield_stun_ticks != -5,
			TEST_FLAG(the_object.object.damage_flags, _object_shield_depleted_bit),
			TEST_FLAG(the_object.object.damage_flags, _object_dead_bit),
			TEST_FLAG(the_flags_after, _object_being_damaged_body_depleted_bit),
			TEST_FLAG(the_flags_after, _object_being_damaged_killed_instantly_bit),
			damage.material_type);
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    damage = read("source/objects/damage.c")
    transitions = periodic.source_file().replace(periodic.MAIN, "")
    return "\n\n".join(
        [
            "#include <stdio.h>\n#include <string.h>",
            transitions,
            FAKE_WORLD,
            function(damage, "damage_data_validate_owner"),
            function(damage, "object_damage_shield"),
            function(damage, "object_damage_body"),
            function(damage, "object_cause_damage"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="cause-damage-reference-") as work:
        source = Path(work) / "cause_damage_reference.c"
        executable = Path(work) / "cause_damage_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    if "assert" in output:
        raise SystemExit("the game's code asserted")
    return (
        "# The game's damage to one object (tools/engine_cause_damage_reference.py), f32 bits.\n"
        "# hit <maximum body> <maximum shield> <body> <shield> <shield depleted> <damage> <minimum> <friendly> <unsuspecting>\n"
        "#   | <effect flags> <side effect> <category> <scale> <resistance flags> <shield material type>\n"
        "#     <friendly resistance> <minimum stun damage> <failure threshold> <maximum failure> <stun time> <failure function>\n"
        "#   | <material struck> <indirect material> (<flags> <type> <leak> <shield multiplier> <body multiplier>) x2\n"
        "#   | <material modifiers 0-7>\n"
        "#   -> <body> <shield> <stunned> <shield depleted> <dead> <body depleted> <killed instantly> <material type>\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_cause_damage_reference.py")
        print("the cause damage reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
