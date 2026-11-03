"""The game's choice of what a player on foot does with a weapon lying
within its reach, taken from the sources and run over random inventories,
to record the choice, as the native engine's pickup must make it.

    python3 tools/engine_pickup_reference.py          # writes the reference
    python3 tools/engine_pickup_reference.py --check  # fails if it is stale

The functions are source/game/players.c's `player_examine_nearby_item` and
`unit_should_autopick_weapon` and source/units/units.c's
`unit_approve_weapon_pickup`, `unit_approve_weapon_swap` and
`unit_get_weapon_count`, found by name as tools/harness finds them and
compiled for this machine against a fake world: a player whose unit carries
up to four weapons of five kinds, each kind with random flags (must be
readied, does not count toward the most carried), each weapon with a random
age, one of them perhaps in its hands, its trigger held or not, and one
weapon lying at its feet, perhaps a CTF flag the game forces it to take.
The choice recorded is the unit's taking it, or the action the button is
set to (adding it, or swapping it for the one in its hands), or nothing.
The ammunition, grenades and powerups taken, the seat's weapon labels and
the action's priority over the other objects within reach are left out.

The trace is written to tools/traces/pickup_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "pickup_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef float real;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)

enum { MAXIMUM_WEAPONS_PER_UNIT = 4, KINDS = 5, LYING = 4 };
enum
{
	_unit_control_weapon_primary_trigger_bit = 11,
	_unit_control_weapon_secondary_trigger_bit,
};
enum
{
	_weapon_must_be_readied_bit = 3,
	_weapon_doesnt_count_toward_maximum_bit,
};
enum { _equipment_powerup_none, _equipment_powerup_grenade };
enum
{
	_player_action_result_none = -1,
	_player_action_result_swap_for_powerup = 5,
	_player_action_result_swap_for_weapon,
	_player_action_result_add_weapon_to_inventory,
};
enum { _network_pickup_ammunition, _network_pickup_grenade, _network_pickup_weapon };

struct player_datum { long unit_index; short local_player_index; };
struct unit_datum
{
	struct { long parent_object_index; } object;
	struct
	{
		long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT];
		short current_weapon_index;
		unsigned long control_flags;
	} unit;
};
struct item_datum
{
	long definition_index;
	struct { long parent_object_index; } object;
	struct { long ignore_object_index; } item;
};
struct weapon_datum { long definition_index; struct { real age; } weapon; };
struct weapon_definition { struct { unsigned long flags; } weapon; };
struct equipment_definition { struct { short powerup_type; } equipment; };

static struct player_datum the_player;
static struct unit_datum the_unit;
static struct item_datum items[5];
static struct weapon_datum weapons[5];
static struct weapon_definition definitions[KINDS];
static boolean forced;
static short taken;
static short action_result;

static struct player_datum *player_get(long index) { return &the_player; }
static struct unit_datum *unit_get(long index) { return &the_unit; }
static struct item_datum *item_get(long index) { return &items[index]; }
static struct weapon_datum *weapon_get(long index) { return &weapons[index]; }
static struct weapon_datum *weapon_try_and_get(long index) { return index == NONE ? 0 : &weapons[index]; }
static struct weapon_definition *weapon_definition_get(long index) { return &definitions[index]; }
static struct item_datum *equipment_try_and_get(long index) { return 0; }
static struct item_datum *equipment_get(long index) { return 0; }
static struct equipment_definition *equipment_definition_get(long index) { return 0; }
static boolean players_decide_pickups(void) { return TRUE; }
static boolean weapon_handle_potential_inventory_item(long weapon, long item, short local, short *count) { *count = 0; return FALSE; }
static boolean unit_add_grenade_to_inventory(long unit, long item) { return FALSE; }
static long unit_get_current_equipment(long unit) { return NONE; }
static void player_handle_powerup_equipment(long player, long item) {}
static boolean unit_can_use_weapon(long unit, long weapon) { return TRUE; }
static boolean game_engine_force_autopickup(long unit, long weapon) { return forced; }
static long unit_inventory_get_weapon(long unit, short index)
{
	return index == NONE ? NONE : the_unit.unit.weapon_object_indices[index];
}
static long unit_get_current_weapon_index(long unit)
{
	return unit_inventory_get_weapon(unit, the_unit.unit.current_weapon_index);
}
static boolean unit_add_weapon_to_inventory(long unit, long weapon, boolean ready) { taken = TRUE; return TRUE; }
static void hud_picked_up_ammunition(short local, long definition, short count) {}
static void hud_picked_up_grenade(short local, long definition) {}
static void hud_picked_up_weapon(short local, long definition) {}
static void player_network_picked_up(struct player_datum *player, long index, short kind, long definition, short count) {}
static void player_control_unzoom(long unit) {}
static void player_set_action_result(long player, short result, long object, short seat) { action_result = result; }
"""

DRIVER = r"""
static unsigned long seed = 0x91c4;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static unsigned bits(real x)
{
	unsigned u;
	memcpy(&u, &x, sizeof u);
	return u;
}

static real const ages[] = { 0.f, 0.f, 0.25f, 0.5f, 0.75f, 1.f };

int main(void)
{
	long run;

	for (run = 0; run < 4000; run++)
	{
		long slot, kind;
		char const *choice;

		memset(&the_unit, 0, sizeof the_unit);
		for (kind = 0; kind < KINDS; kind++)
		{
			long roll = next_random(10);
			definitions[kind].weapon.flags =
				(roll == 0 ? FLAG(_weapon_must_be_readied_bit) : 0) |
				(roll == 1 ? FLAG(_weapon_doesnt_count_toward_maximum_bit) : 0) |
				(roll == 2 ? FLAG(_weapon_must_be_readied_bit) | FLAG(_weapon_doesnt_count_toward_maximum_bit) : 0);
		}
		for (slot = 0; slot <= LYING; slot++)
		{
			items[slot].object.parent_object_index = NONE;
			items[slot].item.ignore_object_index = NONE;
			weapons[slot].definition_index = next_random(KINDS);
			weapons[slot].weapon.age = ages[next_random(6)];
		}
		for (slot = 0; slot < MAXIMUM_WEAPONS_PER_UNIT; slot++)
		{
			/* the first two places held more often than the last two */
			boolean held = next_random(slot < 2 ? 4 : 8) < 3 - (slot >= 2) * 2;
			the_unit.unit.weapon_object_indices[slot] = held ? slot : NONE;
		}
		the_unit.unit.current_weapon_index = next_random(6) - 1;
		if (the_unit.unit.current_weapon_index >= MAXIMUM_WEAPONS_PER_UNIT)
			the_unit.unit.current_weapon_index = 0;
		the_unit.unit.control_flags = next_random(4) == 0 ? FLAG(_unit_control_weapon_primary_trigger_bit) : 0;
		forced = next_random(16) == 0;
		taken = FALSE;
		action_result = _player_action_result_none;
		player_examine_nearby_item(0, LYING);
		choice = taken ? "take" :
			action_result == _player_action_result_add_weapon_to_inventory ? "add" :
			action_result == _player_action_result_swap_for_weapon ? "swap" : "leave";
		printf("case %d %d %d", the_unit.unit.current_weapon_index,
			the_unit.unit.control_flags != 0, forced);
		for (slot = 0; slot <= LYING; slot++)
		{
			boolean held = slot == LYING || the_unit.unit.weapon_object_indices[slot] != NONE;
			printf(" %d/%08x", held ? weapons[slot].definition_index : -1, bits(weapons[slot].weapon.age));
		}
		for (kind = 0; kind < KINDS; kind++)
			printf(" %x", definitions[kind].weapon.flags);
		printf(" %s\n", choice);
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    players = read("source/game/players.c")
    units = read("source/units/units.c")
    return "\n".join(
        [
            FAKE_WORLD,
            function(units, "unit_approve_weapon_pickup"),
            function(units, "unit_get_weapon_count"),
            function(units, "unit_approve_weapon_swap"),
            function(players, "unit_should_autopick_weapon"),
            function(players, "player_examine_nearby_item"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="pickup-reference-") as work:
        source = Path(work) / "pickup_reference.c"
        executable = Path(work) / "pickup_reference"
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
        "# The game's choice of what a player does with a weapon at its feet (tools/engine_pickup_reference.py).\n"
        "# case <the place in its hands, -1 for none> <its trigger held> <forced>\n"
        "#   <the four places' kind/age, kind -1 for empty> <the lying weapon's kind/age> (ages as f32 bits)\n"
        "#   <the five kinds' flags> <take, add, swap or leave>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_pickup_reference.py")
        print("the pickup reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
