"""Where the game respawns a multiplayer player, taken from the sources, to
check the engine's `Game` against.

    python3 tools/engine_spawn_reference.py          # writes the reference
    python3 tools/engine_spawn_reference.py --check  # fails if it is stale

The functions are source/game/players.c's
`find_best_starting_location_index`; source/game/game_engine.c's
`game_engine_get_starting_location_rating`,
`default_starting_location_rate_function`,
`game_engine_get_distance_rating_for_spawn`, `game_engine_get_friendly_bonus`,
`nearby_vehicle`, `match_game_type` and `game_engine_test_flag`; the test
flags of Capture the Flag, Slayer and Oddball and Capture the Flag's
`ctf_engine_starting_location_rating`; and source/math/random_math.c's
`real_seed_random` and `real_seed_random_range`, compiled for this machine
against a fake world of many games: Capture the Flag, Slayer, Oddball and
King of the Hill, with and without teams, a few players with units or none
near a few starts of either team and of several game types, and vehicles
and other objects standing about.

Each game's player 0 (without a unit) is rated at each start, and then a
start is chosen for it from the game's random seed, which runs on from one
game to the next. Capture the Flag is not played as assault (the engine has
no such option), and the objects a sphere touches are the fake world's own:
those whose bounding sphere comes within its radius, in order.

The trace is written to tools/traces/spawn_reference.txt, which
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
from harness import function, inline, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "spawn_reference.txt"

MATH_INLINES = [
    "square_root",
    "vector_from_points3d",
    "magnitude_squared3d",
    "distance_squared3d",
    "distance3d",
    "point_in_sphere",
]
RANDOM_FUNCTIONS = ["real_seed_random", "real_seed_random_range"]
ENGINE_FUNCTIONS = [
    "match_game_type",
    "game_engine_test_flag",
    "nearby_vehicle",
    "game_engine_get_friendly_bonus",
    "game_engine_get_distance_rating_for_spawn",
    "default_starting_location_rate_function",
    "game_engine_get_starting_location_rating",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <stddef.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
typedef unsigned char byte;
typedef float real;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define TICKS_PER_SECOND 30
#define RANDOM_A 1664525u
#define RANDOM_C 1013904223u
#define NUMBEROF(array) (sizeof(array) / sizeof((array)[0]))
#define CEILING(n, ceiling) ((n) > (ceiling) ? (ceiling) : (n))
#define PIN(n, floor, ceiling) ((n) < (floor) ? (floor) : CEILING((n), (ceiling)))
#define TAG_BLOCK_GET_ELEMENT(block, index, type) ((type *)(block)->address + (index))
#define DATUM_INDEX_TO_ABSOLUTE_INDEX(index) ((index) & 0xFFFF)
#define match_assert(file, line, condition) ((void)0)

typedef struct { real x, y, z; } real_point3d;
typedef struct { real i, j, k; } real_vector3d;

struct tag_block { long count; void *address; };

enum game_engine_type
{
	game_engine_none = 0,
	game_engine_ctf,
	game_engine_slayer,
	game_engine_oddball,
	game_engine_king,
	game_engine_race,
};

enum
{
	_game_engine_all = 12,
	_game_engine_all_non_team,
	_game_engine_all_normal,
};

enum { CTF_STARTING_LOCATION_RATING_WARMUP = TICKS_PER_SECOND };
enum { _game_engine_test_flag_rasterize_score = 1 };
enum { _object_type_biped = 0, _object_type_vehicle = 1, _object_type_scenery = 6 };
enum { _object_mask_vehicle = 1 << _object_type_vehicle };

struct player_starting_location
{
	real_point3d position;
	real facing;
	short team_index;
	short bsp_index;
	short game_types[4];
	byte unused[0x18];
};

struct encounter_definition { struct tag_block player_starting_locations; };
struct scenario { struct tag_block players; struct tag_block ai_encounters; };

#define MAXIMUM_PLAYERS 8
#define MAXIMUM_STARTS 10
#define MAXIMUM_OBJECTS 4

static struct player_starting_location starts[MAXIMUM_STARTS];
static struct scenario scenario = { { 0, starts }, { 0, NULL } };
static struct scenario *global_scenario_get(void) { return &scenario; }
static struct player_starting_location *player_get_starting_location(long index) { return &starts[index]; }

struct { long selected_squad_index; } ai_debug = { NONE };

struct player_datum { short team_index; long unit_index; };
static struct player_datum players[MAXIMUM_PLAYERS];
static real_point3d unit_origins[MAXIMUM_PLAYERS];
static long player_count;
static void *player_data;
static struct player_datum *player_get(long index) { return &players[index]; }

struct data_iterator { long next; };
static void data_iterator_new(struct data_iterator *iterator, void *data) { iterator->next = 0; }
static void *data_iterator_next(struct data_iterator *iterator)
{
	return iterator->next < player_count ? &players[iterator->next++] : NULL;
}

static void object_get_origin(long index, real_point3d *origin) { *origin = unit_origins[index]; }

struct object_datum { struct { short type; } object; real_point3d center; real radius; };
static struct object_datum objects[MAXIMUM_OBJECTS];
static long object_count;
static struct object_datum *object_get(long index) { return &objects[index]; }
static void *object_get_and_verify_type(long index, long mask) { return &objects[index]; }

struct location { long cluster; };
static void scenario_location_from_point(struct location *location, real_point3d const *point) {}

struct game_variant
{
	struct { boolean teams; } universal_variant;
	struct { struct { boolean assault; } ctf; } game_engine_variant;
};
static struct game_variant global_variant;
static struct game_variant *game_engine_get_variant(void) { return &global_variant; }
static boolean game_engine_running(void) { return TRUE; }
static boolean game_engine_has_teams(void) { return global_variant.universal_variant.teams; }
static boolean terminator_scoring_rules(void) { return FALSE; }
static long game_time_get(void) { return 0; }

struct scenario_netgame_flag { real_point3d position; };
static struct { struct scenario_netgame_flag *flags[2]; } ctf_globals;

struct game_engine_definition
{
	boolean (*test_flag)(long flag);
	real (*starting_location_rating)(long player_index, struct player_starting_location const *starting_location);
};
static struct game_engine_definition *game_engine;
static short game_type;
static short game_engine_get_type(void) { return game_type; }

static unsigned long global_random_seed = 0xDEADBEEF;
"""

PROTOTYPES = r"""
static real real_seed_random_range(unsigned long *seed, real lower_bound, real upper_bound);
static real real_random_range(real lower_bound, real upper_bound)
{
	return real_seed_random_range(&global_random_seed, lower_bound, upper_bound);
}
static boolean point_in_sphere(real_point3d const *point, real_point3d const *center, real radius);

static short objects_in_sphere(
	long flags,
	long mask,
	struct location const *location,
	real_point3d const *center,
	real radius,
	long *object_indices,
	short maximum_count)
{
	short count = 0;
	long index;

	for (index = 0; index < object_count && count < maximum_count; index++)
	{
		if (point_in_sphere(center, &objects[index].center, objects[index].radius + radius))
			object_indices[count++] = index;
	}
	return count;
}
"""

MAIN = r"""
static unsigned long world_seed = 12345;

static long draw(long range)
{
	world_seed = world_seed * 1103515245u + 12345u;
	return (long)((world_seed >> 8) % (unsigned long)range);
}

static real coordinate(real half)
{
	return (real)draw(2401) / 2400.0f * 2.0f * half - half;
}

static struct game_engine_definition ctf_engine = { ctf_test_flag, ctf_engine_starting_location_rating };
static struct game_engine_definition slayer_engine = { slayer_test_flag, NULL };
static struct game_engine_definition oddball_engine = { oddball_test_flag, NULL };
static struct game_engine_definition king_engine = { NULL, NULL };

int main(void)
{
	static short const types[] = { game_engine_ctf, game_engine_slayer, game_engine_oddball, game_engine_king };
	static short const start_types[][4] = {
		{ 0, 0, 0, 0 }, { 1, 0, 0, 0 }, { 2, 0, 0, 0 }, { 3, 0, 0, 0 }, { 4, 0, 0, 0 },
		{ 12, 0, 0, 0 }, { 13, 0, 0, 0 }, { 14, 0, 0, 0 }, { 2, 1, 0, 0 }, { 5, 4, 0, 0 },
	};
	long game, index;

	for (game = 0; game < 4000; game++)
	{
		long start_count = 1 + draw(MAXIMUM_STARTS);
		real half = draw(3) ? 12.0f : 5.0f;
		unsigned long seed = global_random_seed;
		long chosen;

		game_type = types[draw(NUMBEROF(types))];
		game_engine = game_type == game_engine_ctf ? &ctf_engine
			: game_type == game_engine_slayer ? &slayer_engine
			: game_type == game_engine_oddball ? &oddball_engine
			: &king_engine;
		global_variant.universal_variant.teams = game_type == game_engine_ctf ? TRUE : (boolean)draw(2);

		player_count = 1 + draw(MAXIMUM_PLAYERS);
		for (index = 0; index < player_count; index++)
		{
			players[index].team_index = global_variant.universal_variant.teams ? (short)draw(2) : (short)index;
			players[index].unit_index = index > 0 && draw(10) < 7 ? index : NONE;
			unit_origins[index].x = coordinate(half);
			unit_origins[index].y = coordinate(half);
			unit_origins[index].z = coordinate(1.0f);
		}
		scenario.players.count = start_count;
		for (index = 0; index < start_count; index++)
		{
			short const *chosen_types = start_types[draw(NUMBEROF(start_types))];
			long type;

			starts[index].team_index = (short)draw(2);
			for (type = 0; type < 4; type++)
				starts[index].game_types[type] = chosen_types[type];
			if (index > 0 && draw(4) == 0)
			{
				/* (on a player now and then) */
				starts[index].position = unit_origins[draw(player_count)];
				starts[index].position.x += coordinate(0.3f);
			}
			else
			{
				starts[index].position.x = coordinate(half);
				starts[index].position.y = coordinate(half);
				starts[index].position.z = coordinate(1.0f);
			}
		}
		object_count = draw(MAXIMUM_OBJECTS);
		for (index = 0; index < object_count; index++)
		{
			objects[index].object.type = draw(3) ? _object_type_vehicle : _object_type_scenery;
			objects[index].center.x = coordinate(half);
			objects[index].center.y = coordinate(half);
			objects[index].center.z = coordinate(1.0f);
			objects[index].radius = 0.25f + (real)draw(400) / 400.0f;
		}

		printf("game %d %d %d %d %u\n", game_type, global_variant.universal_variant.teams, players[0].team_index, (int)player_count, (unsigned)seed);
		for (index = 0; index < player_count; index++)
		{
			if (players[index].unit_index == NONE)
				printf("player %d -\n", players[index].team_index);
			else
				printf("player %d %.9g %.9g %.9g\n", players[index].team_index, unit_origins[index].x, unit_origins[index].y, unit_origins[index].z);
		}
		for (index = 0; index < object_count; index++)
			printf("object %d %.9g %.9g %.9g %.9g\n", objects[index].object.type == _object_type_vehicle, objects[index].center.x, objects[index].center.y, objects[index].center.z, objects[index].radius);
		for (index = 0; index < start_count; index++)
		{
			struct player_starting_location *start = &starts[index];

			printf("start %d %d %d %d %d %.9g %.9g %.9g %.9g\n", start->team_index,
				start->game_types[0], start->game_types[1], start->game_types[2], start->game_types[3],
				start->position.x, start->position.y, start->position.z,
				game_engine_get_starting_location_rating(0, start));
		}
		chosen = find_best_starting_location_index(0);
		printf("chosen %d %u\n", chosen, (unsigned)global_random_seed);
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the run, as one C file."""
    math_h = read("source/math/real_math.h")
    random_c = read("source/math/random_math.c")
    engine_c = read("source/game/game_engine.c")
    return (
        "\n\n".join(
            [
                FAKE_WORLD,
                *(inline(math_h, name) for name in MATH_INLINES),
                PROTOTYPES,
                *(function(random_c, name) for name in RANDOM_FUNCTIONS),
                *(function(engine_c, name) for name in ENGINE_FUNCTIONS),
                function(read("source/game/game_engine_ctf.c"), "ctf_test_flag"),
                function(read("source/game/game_engine_ctf.c"), "ctf_engine_starting_location_rating"),
                function(read("source/game/game_engine_slayer.c"), "slayer_test_flag"),
                function(read("source/game/game_engine_oddball.c"), "oddball_test_flag"),
                function(read("source/game/players.c"), "find_best_starting_location_index"),
                MAIN,
            ]
        )
        .replace("real real_seed_random(", "static real real_seed_random(")
        .replace("real real_seed_random_range(", "static real real_seed_random_range(", 1)
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="spawn-reference-") as work:
        source = Path(work) / "spawn_reference.c"
        executable = Path(work) / "spawn_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# Where the game respawns player 0 of 4000 games (tools/engine_spawn_reference.py).\n"
        "# game <type> <teams> <player 0's team> <players> <random seed>\n"
        "# player <team> <x y z of its unit, or - for none>\n"
        "# object <vehicle> <x y z of its bounding sphere's centre> <radius>\n"
        "# start <team> <game types>... <x y z> <rating>\n"
        "# chosen <start, -1 for none> <random seed after>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_spawn_reference.py")
        print("the spawn reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
