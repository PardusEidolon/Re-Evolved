"""The game's falling damage: whether a biped's landing or fall hurts it,
with which of the globals' damage and at what scale, run over random
landings and falls to record what the engine's must give.

    python3 tools/engine_biped_fall_reference.py          # writes the reference
    python3 tools/engine_biped_fall_reference.py --check  # fails if it is stale

The function is source/units/bipeds.c's `biped_falling_damage`, compiled
for this machine against a small fake world whose object_cause_damage
records the damage it is asked for (and whose game engine runs, so no biped
is erased). The table is written to
tools/traces/biped_fall_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "biped_fall_reference.txt"

WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char boolean;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define FLAG(b) (1<<(b))
#define TEST_FLAG(flags, bit) (((flags)&(unsigned)FLAG(bit))!=0)

enum { _unit_no_falling_damage_bit = 3 };
enum { _biped_flying_bit = 2, _biped_immune_to_falling_damage_bit = 7 };
enum { _object_dead_bit = 0 };
enum { _object_outside_of_map_bit = 5 };
enum { _error_silent };

typedef struct { real i, j, k; } real_vector3d;
struct tag_reference { long index; };
struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[(index)])

struct game_globals_falling_damage
{
	struct tag_reference falling_damage;
	struct tag_reference maximum_distance_damage;
	real runtime_maximum_falling_velocity;
	real runtime_minimum_damage_velocity;
	real runtime_maximum_damage_velocity;
};
struct game_globals { struct tag_block falling_damage; };

struct biped_definition { struct { unsigned long flags; } biped; };
struct biped_datum
{
	long definition_index;
	struct
	{
		unsigned long flags;
		unsigned long damage_flags;
		real_vector3d translational_velocity;
	} object;
	struct
	{
		unsigned long flags;
		long player_index;
		long actor_index;
		long swarm_actor_index;
	} unit;
};

struct damage_data { long definition_index; real scale; };

static struct biped_datum the_biped;
static struct biped_definition the_definition;
static struct game_globals_falling_damage the_falling_damage;
static struct game_globals the_globals;
static struct { boolean jetpack; } cheat;
static char temporary[256];

static struct biped_datum *biped_get(long index) { return &the_biped; }
static struct biped_definition *biped_definition_get(long index) { return &the_definition; }
static struct game_globals *scenario_get_game_globals(void) { return &the_globals; }
static boolean game_engine_running(void) { return TRUE; }
static long player_index_from_unit_index(long index) { return NONE; }
static boolean network_objects_may_delete(long index) { return FALSE; }
static void object_delete(long index) { printf(" deleted"); }
static char const *tag_get_name(long index) { return ""; }
static char const *tag_name_strip_path(char const *name) { return name; }
static char const *ai_debug_describe_actor(long a, long b, boolean c, char *d, long e) { return ""; }
static void error(int kind, char const *format, ...) { }

static void damage_data_new(struct damage_data *damage, long definition_index)
{
	memset(damage, 0, sizeof(*damage));
	damage->definition_index = definition_index;
	damage->scale = 1.0f;
}

/* what the fall asks for */
static int asked;

static void object_cause_damage(struct damage_data *damage, long object_index, long a, long b, long c, void *d)
{
	asked = 1;
	printf(" %d %.9g", damage->definition_index, damage->scale);
}
"""

MAIN = r"""
static unsigned int seed = 0xfa11;

static unsigned int next(void)
{
	seed = seed*1664525u+1013904223u;
	return seed;
}

/* a real from 0 to scale, in steps of a thousandth of it */
static real upto(real scale)
{
	return (real)((int)(next()>>16)%1001)*0.001f*scale;
}

int main(void)
{
	int step;

	the_falling_damage.falling_damage.index = 0;
	the_falling_damage.maximum_distance_damage.index = 1;
	the_globals.falling_damage.count = 1;
	the_globals.falling_damage.address = &the_falling_damage;
	for (step = 0; step<4000; step++)
	{
		/* (a tick's velocities, as the game's tags and physics keep them) */
		real landing;

		the_falling_damage.runtime_minimum_damage_velocity = upto(0.3f);
		the_falling_damage.runtime_maximum_damage_velocity =
			the_falling_damage.runtime_minimum_damage_velocity+0.001f+upto(0.3f);
		the_falling_damage.runtime_maximum_falling_velocity = upto(0.8f);
		landing = step%3==0 ? 0.0f : upto(0.8f);
		memset(&the_biped, 0, sizeof(the_biped));
		the_biped.object.translational_velocity.k = upto(1.0f)-0.9f;
		the_biped.unit.flags = step%23==0 ? FLAG(_unit_no_falling_damage_bit) : 0;
		the_definition.biped.flags = (step%29==0 ? FLAG(_biped_immune_to_falling_damage_bit) : 0)|
			(step%7==0 ? FLAG(_biped_flying_bit) : 0);
		the_biped.object.damage_flags = step%11==0 ? FLAG(_object_dead_bit) : 0;
		the_biped.unit.player_index = 0;

		printf("fall %.9g %.9g %.9g %.9g %.9g %d %d %d ->", the_falling_damage.runtime_minimum_damage_velocity,
			the_falling_damage.runtime_maximum_damage_velocity, the_falling_damage.runtime_maximum_falling_velocity,
			landing, the_biped.object.translational_velocity.k,
			TEST_FLAG(the_biped.unit.flags, _unit_no_falling_damage_bit)||
				TEST_FLAG(the_definition.biped.flags, _biped_immune_to_falling_damage_bit),
			TEST_FLAG(the_definition.biped.flags, _biped_flying_bit),
			TEST_FLAG(the_biped.object.damage_flags, _object_dead_bit));
		asked = 0;
		biped_falling_damage(0, landing);
		printf(asked ? "\n" : " none\n");
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's function and the cases, as one C file."""
    bipeds_c = read("source/units/bipeds.c")
    return "\n\n".join([WORLD, function(bipeds_c, "biped_falling_damage"), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="biped-fall-reference-") as work:
        source = Path(work) / "biped_fall_reference.c"
        executable = Path(work) / "biped_fall_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's falling damage (tools/engine_biped_fall_reference.py), its velocities a tick.\n"
        "# fall <minimum damage velocity> <maximum damage velocity> <maximum falling velocity>\n"
        "#   <landing velocity> <upward velocity> <immune> <flying> <dead>\n"
        "#   -> <damage: 0 falling, 1 greatest falls'> <scale>, or none\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_biped_fall_reference.py")
        print("the biped fall reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
