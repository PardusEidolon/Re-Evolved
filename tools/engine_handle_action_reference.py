"""The game's choice of how a player turns an overturned vehicle back over,
taken from the sources and run over random vehicles and players, to record
the side or end each is pushed from, as the native engine's
`Approach::of` must choose it.

    python3 tools/engine_handle_action_reference.py          # writes the reference
    python3 tools/engine_handle_action_reference.py --check  # fails if it is stale

The function is source/game/players.c's `player_handle_action`, found by
name as tools/harness finds it and compiled for this machine against a fake
world, with source/math/real_math.h's own cross and dot products. Every run
sets the player's action to flipping the vehicle (`_player_action_result_
flip_vehicle`), with the vehicle's forward axis anywhere on the sphere (a
third of them near upright, on a nose or a tail, and some at the 45 degrees
between) and the player anywhere around it, and records the approach the
game gives the vehicle. The other actions (a device touched, a powerup
swapped, a seat entered) are stand-ins that say they ran.

The trace is written to tools/traces/handle_action_reference.txt, which
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
from harness import enum_with, function, inline, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "handle_action_reference.txt"

FAKE_WORLD = r"""
#include <math.h>
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
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct
{
	real scale;
	real_vector3d forward, left, up;
	real_point3d position;
} real_matrix4x3;

enum { _vehicle_upending_bit = 2 };
enum { _network_pickup_equipment = 1 };

struct player_datum
{
	long unit_index;
	short local_player_index;
	short action_result;
	long action_object_index;
	short action_seat_index;
};
struct unit_datum
{
	struct { real_point3d position; } object;
	struct
	{
		long actor_index;
		long last_vehicle_index;
		long game_time_at_last_vehicle_exit;
	} unit;
};
struct players_vehicle_datum
{
	struct { real_point3d position; real_vector3d forward; } object;
	struct { unsigned long flags; char approach_type; short unknown42A; } vehicle;
};
struct item_datum { long definition_index; };

static struct player_datum the_player;
static struct unit_datum the_unit;
static struct players_vehicle_datum the_vehicle;
static struct item_datum the_equipment;
static real_vector3d const up = { 0.f, 0.f, 1.f };
static real_vector3d const *global_up3d = &up;

static struct player_datum *player_get(long index) { return &the_player; }
static struct unit_datum *unit_get(long index) { return &the_unit; }
static void *vehicle_get(long index) { return &the_vehicle; }
static struct item_datum *equipment_get(long index) { return &the_equipment; }
static long game_time_get(void) { return 1234; }
static real_matrix4x3 *object_get_world_matrix(long index, real_matrix4x3 *matrix)
{
	memset(matrix, 0, sizeof *matrix);
	if (index == 1)
	{
		matrix->position = the_vehicle.object.position;
	}
	else
	{
		matrix->position = the_unit.object.position;
	}
	return matrix;
}
static boolean network_coop_devices_remote(void) { return FALSE; }
static boolean players_decide_pickups(void) { return TRUE; }
static void device_touched(long device, long unit) { printf("assert device\n"); }
static void unit_drop_current_equipment(long unit) { printf("assert drop\n"); }
static boolean unit_add_equipment_to_inventory(long unit, long equipment, boolean x) { printf("assert equipment\n"); return FALSE; }
static void hud_picked_up_powerup(short local, long definition) {}
static void player_network_picked_up(struct player_datum *player, long index, long kind, long definition, long count) {}
static boolean unit_can_enter_seat(long unit, long vehicle, short seat, long *nearby) { printf("assert seat\n"); return FALSE; }
static void unit_enter_seat(long unit, long vehicle, short seat) {}
static void ai_try_vehicle_eviction(long actor, long unit, boolean x) {}
"""

DRIVER = r"""
static unsigned long seed = 0xf11b;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

/* a random real in -range..range, in thousandths of it */
static real next_real(real range)
{
	return range * (real)(next_random(2001) - 1000) / 1000.f;
}

static unsigned bits(real x)
{
	unsigned u;
	memcpy(&u, &x, sizeof u);
	return u;
}

int main(void)
{
	long run;

	for (run = 0; run < 3000; run++)
	{
		long kind = next_random(6);
		real_vector3d forward;
		real length;
		boolean result;

		memset(&the_unit, 0, sizeof the_unit);
		memset(&the_vehicle, 0, sizeof the_vehicle);
		the_player.unit_index = 0;
		the_player.local_player_index = 0;
		the_player.action_result = _player_action_result_flip_vehicle;
		the_player.action_object_index = 1;
		the_player.action_seat_index = NONE;
		if (kind < 2)
		{
			/* on a nose or a tail, leaning */
			forward.i = next_real(0.6f);
			forward.j = next_real(0.6f);
			forward.k = next_random(2) ? 1.f : -1.f;
		}
		else if (kind == 2)
		{
			/* at the 45 degrees between lying and standing */
			forward.i = 0.70710677f;
			forward.j = 0.f;
			forward.k = next_random(2) ? 0.70710677f : -0.70710677f;
		}
		else
		{
			forward.i = next_real(1.f);
			forward.j = next_real(1.f);
			forward.k = next_real(1.f);
		}
		length = sqrtf(forward.i * forward.i + forward.j * forward.j + forward.k * forward.k);
		if (length > 0.f && kind != 2)
		{
			forward.i /= length;
			forward.j /= length;
			forward.k /= length;
		}
		the_vehicle.object.forward = forward;
		the_vehicle.object.position.x = next_real(100.f);
		the_vehicle.object.position.y = next_real(100.f);
		the_vehicle.object.position.z = next_real(10.f);
		the_unit.object.position.x = the_vehicle.object.position.x + next_real(3.f);
		the_unit.object.position.y = the_vehicle.object.position.y + next_real(3.f);
		the_unit.object.position.z = the_vehicle.object.position.z + next_real(1.f);
		if (next_random(20) == 0)
		{
			/* standing where the vehicle is */
			the_unit.object.position = the_vehicle.object.position;
		}
		result = player_handle_action(0);
		if (!result || !(the_vehicle.vehicle.flags & FLAG(_vehicle_upending_bit)) ||
			the_unit.unit.last_vehicle_index != 1)
			printf("assert flip\n");
		printf("flip %08x %08x %08x %08x %08x %08x %08x %08x %08x %d\n",
			bits(forward.i), bits(forward.j), bits(forward.k),
			bits(the_vehicle.object.position.x), bits(the_vehicle.object.position.y),
			bits(the_vehicle.object.position.z),
			bits(the_unit.object.position.x), bits(the_unit.object.position.y),
			bits(the_unit.object.position.z), the_vehicle.vehicle.approach_type);
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    real_math = read("source/math/real_math.h")
    players = read("source/game/players.c")
    players_h = read("source/game/players.h")
    return "\n".join(
        [
            FAKE_WORLD,
            enum_with(players_h, "_player_action_result_flip_vehicle") + ";",
            inline(real_math, "dot_product3d"),
            inline(real_math, "cross_product3d"),
            function(players, "player_handle_action"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="handle-action-reference-") as work:
        source = Path(work) / "handle_action_reference.c"
        executable = Path(work) / "handle_action_reference"
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
        "# The game's approach to an overturned vehicle (tools/engine_handle_action_reference.py).\n"
        "# flip <the vehicle's forward i j k> <its position x y z> <the player's x y z> (f32 bits)"
        " <the approach: 1 right, 2 left, 3 on its tail, 4 on its nose>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_handle_action_reference.py")
        print("the handle action reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
