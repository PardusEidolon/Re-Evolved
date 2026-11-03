"""The game's turn of a player's look by the mouse, taken from the sources
and run over random turns, to record the look after each, as the native
engine's facing must.

    python3 tools/engine_facing_reference.py          # writes the reference
    python3 tools/engine_facing_reference.py --check  # fails if it is stale

The function is source/game/player_control.c's
`player_control_modify_desired_angles`, found by name as tools/harness
finds it and compiled for this machine against a fake world: a player on
foot, whose unit has no seat and no camera of its own, with the pitch
limits a new player is given (85.5 degrees up and down) and a tick a step.
Each run turns the look by mouse-sized steps, quick flicks and turns of
several whole turns. A seat's yaw limits, a camera's pitch limits and the
look levelled in a moving vehicle are left out.

The trace is written to tools/traces/facing_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "facing_reference.txt"

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
#define _pi ((real)3.14159265358979323846f)
#define DEGREES_TO_RADIANS(angle) ((real)(((real)angle) * ((real)M_PI) / 180.f))
#define CEILING(n, ceiling) ((n) > (ceiling) ? (ceiling) : (n))
#define PIN(n, floor, ceiling) ((n) < (floor) ? (floor) : CEILING((n), (ceiling)))
#define match_assert(file, line, expr) if (!(expr)) { printf("assert %d\n", line); }
#define match_assert_valid_real(file, line, x) if ((x) != (x)) { printf("assert %d\n", line); }
#define TAG_BLOCK_GET_ELEMENT(block, index, type) ((type *)(block))

typedef union real_euler_angles2d
{
	real n[2];
	struct { real yaw; real pitch; };
} real_euler_angles2d;
typedef struct { real i, j, k; } real_vector3d;
typedef struct { real_vector3d forward; } real_matrix;

struct unit_camera { real pitch_autolevel, pitch_minimum, pitch_maximum; };
struct unit_seat { real yaw_minimum, yaw_maximum; char marker_name[32]; };
struct unit_definition { struct { struct unit_seat seats; } unit; };
struct unit_datum
{
	long definition_index;
	struct { real_vector3d up; real_vector3d translational_velocity; } object;
};
struct object_marker { real_matrix matrix; };
struct game_globals_player_control { real look_autolevel_scale; };
struct game_globals { struct game_globals_player_control player_control; };

struct player_control_unit_camera_info
{
	long unit_index;
	short seat_index;
	struct unit_camera const *camera;
};

struct player_control
{
	real_euler_angles2d desired_angles;
	real pitch_minimum;
	real pitch_maximum;
	boolean use_autolevel;
};

static struct player_control the_player;
static struct game_globals globals;
static struct unit_datum the_unit;
static struct unit_definition the_definition;
static real player_control_angle_step_ticks = 1.f;

static struct player_control *player_control_get(short index) { return &the_player; }
static struct game_globals *scenario_get_game_globals(void) { return &globals; }
static boolean valid_euler_angles2d(real_euler_angles2d const *angles) { return angles->yaw == angles->yaw && angles->pitch == angles->pitch; }
static void player_control_get_unit_camera_info(short index, struct player_control_unit_camera_info *info)
{
	info->unit_index = 0;
	info->seat_index = NONE;
	info->camera = 0;
}
static struct unit_datum *unit_get(long index) { return &the_unit; }
static struct unit_definition *unit_definition_get(long index) { return &the_definition; }
static void object_get_marker_by_name(long object, char const *name, struct object_marker *marker, short count) {}
static real_euler_angles2d *euler_angles2d_from_vector3d(real_euler_angles2d *angles, real_vector3d const *vector) { return angles; }
static real_vector3d *vector3d_from_euler_angles2d(real_vector3d *vector, real_euler_angles2d const *angles) { return vector; }
static real signed_angular_difference(real a, real b) { return b - a; }
static real angle_between_vectors3d(real_vector3d const *a, real_vector3d const *b) { return 0.f; }
static real magnitude3d(real_vector3d const *v) { return 0.f; }
"""

DRIVER = r"""
static unsigned long seed = 0x5ac1e;

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

	for (run = 0; run < 32; run++)
	{
		long step;

		memset(&the_player, 0, sizeof the_player);
		/* a new player's limits (player_control.c) */
		the_player.pitch_maximum = DEGREES_TO_RADIANS(85.5f);
		the_player.pitch_minimum = -DEGREES_TO_RADIANS(85.5f);
		the_player.desired_angles.yaw = next_real(_pi * 2.f);
		if (the_player.desired_angles.yaw < 0.f)
			the_player.desired_angles.yaw = -the_player.desired_angles.yaw;
		printf("run %08x\n", bits(the_player.desired_angles.yaw));
		for (step = 0; step < 200; step++)
		{
			long kind = next_random(20);
			real yaw, pitch;

			if (kind < 14)
			{
				yaw = next_real(0.05f);
				pitch = next_real(0.05f);
			}
			else if (kind < 18)
			{
				yaw = next_real(1.5f);
				pitch = next_real(1.f);
			}
			else
			{
				yaw = next_real(40.f);
				pitch = next_real(4.f);
			}
			player_control_modify_desired_angles(0, yaw, pitch);
			printf("turn %08x %08x %08x %08x\n", bits(yaw), bits(pitch),
				bits(the_player.desired_angles.yaw), bits(the_player.desired_angles.pitch));
		}
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    real_math = read("source/math/real_math.h")
    player_control = read("source/game/player_control.c")
    return "\n".join(
        [
            FAKE_WORLD,
            function(real_math, "interpolate_scalar").replace("__inline", "static"),
            function(player_control, "player_control_modify_desired_angles"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="facing-reference-") as work:
        source = Path(work) / "facing_reference.c"
        executable = Path(work) / "facing_reference"
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
        "# The game's turn of a player's look (tools/engine_facing_reference.py).\n"
        "# run <the starting yaw, f32 bits>\n"
        "# turn <the yaw's delta> <the pitch's delta> <the yaw after> <the pitch after> (f32 bits)\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_facing_reference.py")
        print("the facing reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
