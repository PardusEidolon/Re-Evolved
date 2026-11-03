"""The game's multiplayer rules for a death, taken from the sources and run
over long games of kills, suicides, falls and betrayals, to record the
respawn waits and scores the native engine's must give.

    python3 tools/engine_game_engine_reference.py          # writes the reference
    python3 tools/engine_game_engine_reference.py --check  # fails if it is stale

The functions are source/game/game_engine.c's `game_engine_player_killed`,
`game_engine_get_team_score` and `game_engine_player_added` (with
`initialize_player_multiplayer_data` and `game_engine_has_teams`), and
source/game/game_engine_slayer.c's `slayer_engine_player_added`,
`slayer_engine_update` and scorer (`slayer_engine_player_killed_player`,
`slayer_engine_adjust_score`, `slayer_get_score`), found by name as tools/harness finds them, and compiled
for this machine against a small fake world: a table of players, the
variant's options, and stand-ins for the messages and the network. Each
death goes through source/game/game_statistics.c's own
`game_statistics_record_kill`, which tallies the kill, death, suicide or
friendly-fire kill and decides who is credited and whether it is friendly
fire; no unit remembers an attacker (so there are no assists and the
killer is credited) and each team is an ally of itself only. The players are added by source/game/game.c's own
`game_initial_pulse`, from a new map's `next_team_index`, and Slayer's update runs each
tick. A dead player's respawn timer counts down a game tick at a time, as
`game_engine_update`'s does, and a player is killed only while alive.

The trace is written to tools/traces/game_engine_reference.txt, which
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
from harness import function, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "game_engine_reference.txt"

ENGINE_FUNCTIONS = [
    "game_engine_has_teams",
    "game_engine_get_team_score",
    "game_engine_player_killed",
    "initialize_player_multiplayer_data",
    "game_engine_player_added",
]
SLAYER_FUNCTIONS = [
    "slayer_engine_adjust_score",
    "slayer_get_score",
    "slayer_engine_player_killed_player",
    "slayer_engine_player_added",
    "slayer_engine_update",
]

PLAYERS = 8

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef unsigned char byte;
typedef float real;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define TICKS_PER_SECOND 30
#define HALO_PORT_MAXIMUM_NETWORK_PLAYERS 16
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define VALID_INDEX(i, n) ((i) >= 0 && (i) < (n))
#define PIN(x, lo, hi) MIN(MAX((x), (lo)), (hi))
#define DATUM_INDEX_TO_ABSOLUTE_INDEX(i) (i)
#define match_assert(file, line, condition) ((void)0)
#define csmemset memset

enum get_score_type { _get_score_individual, _get_score_team };

enum
{
	_game_engine_message_killed_by_unknown,
	_game_engine_message_killed_by_biped,
	_game_engine_message_killed_by_vehicle,
	_game_engine_message_killed_by_self,
	_game_engine_message_killed_by_player,
	_game_engine_message_killed_by_friendly_fire,
	_game_engine_message_quit,
	_game_engine_message_killed_friendly,
	_game_engine_message_multi_kill,
	_game_engine_message_triple_kill,
	_game_engine_message_double_kill,
	_game_engine_message_five_kills_in_row,
	_game_engine_message_killed_enemy,
	_game_engine_message_ten_kills_in_a_row,
};

enum { _object_type_biped, _object_type_vehicle };

struct player_datum
{
	long team_index;
	long respawn_timer;
	long respawn_penalty;
	long death_time;
	long quit_out_of_game_time;
	boolean quit_out_of_game;
	long multiplayer_special;
	struct { long player_list_index; char team_index; } network_player_data;
	struct
	{
		short kills_in_a_row;
		short multiple_kills;
		short last_kill_time;
		short kills[4];
		short assists[4];
		short friendly_fire_kills;
		short deaths;
		short suicides;
		long multiplayer_statistics;
	} statistics;
	/* the HUD's, which initialize_player_multiplayer_data sets and the trace
	does not show */
	long state_message;
	long state_message_player_index;
	real speed_multiplier;
	long teleporter_index;
	long player_display_index;
};

static struct player_datum players[HALO_PORT_MAXIMUM_NETWORK_PLAYERS];
static long player_count;
static void *player_data;

static struct player_datum *player_get(long index) { return &players[index]; }
static struct player_datum *player_try_and_get(long index) { return VALID_INDEX(index, player_count) ? &players[index] : NULL; }

struct data_iterator { long datum_index; long next; };

static void data_iterator_new(struct data_iterator *iterator, void *data)
{
	iterator->datum_index = NONE;
	iterator->next = 0;
}

static void *data_iterator_next(struct data_iterator *iterator)
{
	if (iterator->next >= player_count)
		return NULL;
	iterator->datum_index = iterator->next++;
	return &players[iterator->datum_index];
}

struct object_datum { struct { long type; } object; };
static struct object_datum an_object;
static struct object_datum *object_get(long index) { return &an_object; }
static void *unit_try_and_get(long index) { return NULL; }

/* the kill's statistics (game_statistics.c): every player is a unit of the
same index, no unit remembers an attacker, and the teams are allies of
themselves only, as a game's are with no allegiances made */
#define MAXIMUM_ATTACKERS_PER_UNIT 4
#define NUMBER_OF_GAME_TEAMS 16
#define REAL_MIN (-3.4028235e38f)
struct unit_attacker { unsigned long game_time_stamp; real damage_inflicted; long object_index; long player_index; };
struct unit_datum { struct { struct unit_attacker attackers[MAXIMUM_ATTACKERS_PER_UNIT]; } unit; };
static struct unit_datum a_unit;
static struct unit_datum *unit_get(long index)
{
	long attacker;

	memset(&a_unit, 0, sizeof(a_unit));
	for (attacker = 0; attacker < MAXIMUM_ATTACKERS_PER_UNIT; attacker++)
	{
		a_unit.unit.attackers[attacker].object_index = NONE;
		a_unit.unit.attackers[attacker].player_index = NONE;
	}
	return &a_unit;
}
static long player_index_from_unit_index(long unit) { return unit; }
static boolean game_team_is_enemy(short ours, short other)
{
	return !VALID_INDEX(ours, NUMBER_OF_GAME_TEAMS) || !VALID_INDEX(other, NUMBER_OF_GAME_TEAMS) || ours != other;
}
static boolean game_statistics_active = TRUE;

static long game_time;
static long game_time_get(void) { return game_time; }

/* the network: a host of a game with no clients */
static void network_distributed_player_killed(long *killer, long *object, long dead, boolean *friendly) {}
boolean network_game_distributed_client(void) { return FALSE; }
static void *global_network_game_client_get(void) { return NULL; }
struct network_game { struct { char team_index; } players[HALO_PORT_MAXIMUM_NETWORK_PLAYERS]; };
struct network_game_server;
static struct network_game_server *global_network_game_server_get(void) { return NULL; }
static struct network_game *network_game_server_get_game(struct network_game_server *server) { return NULL; }

/* the messages, which the trace does not show */
static void game_show_score_extended(long player, long message, long other) {}
static void multiplayer_message(long player, long message, long other) {}

/* the PC version's gametype options, as stock has them: off */
struct game_variant_options { boolean auto_team_balance; long friendly_fire_penalty; };
static struct game_variant_options options;
static struct game_variant_options *game_variant_options_get(void) { return &options; }
"""

VARIANT = r"""
struct game_variant
{
	struct universal_variant universal_variant;
	union { struct slayer_variant slayer; } game_engine_variant;
};

static struct game_variant global_variant;
static struct game_variant *game_engine_get_variant(void) { return &global_variant; }

/* slayer's speed for its score and its next target are kill in order's and
the score's show, which the trace leaves out */
void update_speed_for_score(long dead, long killer) {}
static void find_next_target(long player) {}

struct game_engine
{
	void (*player_killed_player)(long, long, long, boolean);
	long (*get_player_score)(long, enum get_score_type);
	void (*player_added)(long);
	void (*update)(void);
};
static struct game_engine *game_engine;

struct game_engine_globals { long next_team_index; };
static struct game_engine_globals game_engine_globals;

/* the game type's start, which tools/engine_announcer_reference.py traces */
void game_engine_game_starting(void) {}
"""

MAIN = r"""
static unsigned long seed = 0x2468ace;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static struct game_engine slayer = {
	slayer_engine_player_killed_player,
	slayer_get_score,
	slayer_engine_player_added,
	slayer_engine_update,
};

static void play(long teams, long respawn_time, long growth, long suicide_penalty, long count, long kills)
{
	long index;
	long kill;

	memset(players, 0, sizeof(players));
	memset(&slayer_globals, 0, sizeof(slayer_globals));
	memset(game_engine_betrayal_penalty, 0, sizeof(game_engine_betrayal_penalty));
	player_count = count;
	game_engine = &slayer;
	global_variant.universal_variant.teams = teams;
	global_variant.universal_variant.respawn_time = respawn_time;
	global_variant.universal_variant.respawn_time_growth = growth;
	global_variant.universal_variant.suicide_penalty = suicide_penalty;
	game_engine_globals.next_team_index = 0;
	printf("game %d %d %d %d %d\n", teams, respawn_time, growth, suicide_penalty, count);
	game_initial_pulse();
	for (index = 0; index < count; index++)
		printf("join %d %d\n", index, players[index].team_index);
	for (kill = 0; kill < kills; kill++)
	{
		long wait = next_random(4) ? next_random(60) : next_random(600);
		long tick;
		long dead;
		long killer;
		long kind;

		for (tick = 0; tick < wait; tick++)
		{
			game_time++;
			game_engine->update();
			for (index = 0; index < count; index++)
			{
				if (players[index].respawn_timer > 0)
					players[index].respawn_timer--;
			}
		}
		dead = next_random(count);
		if (players[dead].respawn_timer > 0)
		{
			printf("wait %d\n", wait);
			continue;
		}
		kind = next_random(8);
		killer = kind == 0 ? NONE : kind == 1 ? dead : next_random(count);
		if (killer != NONE && killer != dead && players[killer].respawn_timer > 0)
			killer = NONE;
		game_statistics_record_kill(dead, killer, NONE, killer == NONE ? NONE : players[killer].team_index);
		printf("kill %d %d %d %d", wait, dead, killer, players[dead].respawn_timer);
		for (index = 0; index < count; index++)
			printf(" %d/%d", game_engine->get_player_score(index, _get_score_individual), players[index].respawn_penalty);
		printf(" teams");
		for (index = 0; index < (teams ? 2 : count); index++)
			printf(" %d", game_engine_get_team_score(index));
		printf(" stats");
		for (index = 0; index < count; index++)
			printf(" %d/%d/%d/%d", players[index].statistics.kills[0], players[index].statistics.deaths,
				players[index].statistics.suicides, players[index].statistics.friendly_fire_kills);
		printf("\n");
	}
}

int main(void)
{
	static long const respawn_times[] = { 0, 150, 300 };
	static long const growths[] = { 0, 30, 90 };
	static long const suicide_penalties[] = { 0, 150, 300 };
	long teams;
	long a, b, c;

	for (teams = 0; teams < 2; teams++)
		for (a = 0; a < 3; a++)
			for (b = 0; b < 3; b++)
				for (c = 0; c < 3; c++)
					play(teams, respawn_times[a], growths[b], suicide_penalties[c], 2 + next_random(PLAYERS - 1), 60);
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    game_engine_h = read("source/game/game_engine.h")
    game_engine_c = read("source/game/game_engine.c")
    slayer_c = read("source/game/game_engine_slayer.c")
    variants = [structure(game_engine_h, name) for name in ("universal_variant", "slayer_variant")]
    globals_ = [
        structure(slayer_c, "slayer_globals"),
        "static struct slayer_globals slayer_globals;",
        "static short game_engine_betrayal_penalty[HALO_PORT_MAXIMUM_NETWORK_PLAYERS];",
        "boolean game_engine_has_teams(void);",
    ]
    slayer = [function(slayer_c, name) for name in SLAYER_FUNCTIONS]
    engine = [function(game_engine_c, name) for name in ENGINE_FUNCTIONS]
    pulse = function(read("source/game/game.c"), "game_initial_pulse")
    record_kill = function(read("source/game/game_statistics.c"), "game_statistics_record_kill")
    return "\n\n".join(
        [
            FAKE_WORLD,
            *variants,
            VARIANT,
            *globals_,
            *slayer,
            *engine,
            record_kill,
            pulse,
            MAIN.replace("PLAYERS", str(PLAYERS)),
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="game-engine-reference-") as work:
        source = Path(work) / "game_engine_reference.c"
        executable = Path(work) / "game_engine_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's multiplayer rules for a death (tools/engine_game_engine_reference.py).\n"
        "# game <teams> <respawn time> <respawn growth> <suicide penalty> <players> (game ticks)\n"
        "# join <player> <team>\n"
        "# wait <ticks>: the ticks went by, and the one chosen to die was not alive\n"
        "# kill <ticks before> <dead> <killer> <dead's respawn timer> <score/respawn penalty>... teams <team score>... stats <kills/deaths/suicides/friendly-fire kills>...\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_game_engine_reference.py")
        print("the game engine reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
