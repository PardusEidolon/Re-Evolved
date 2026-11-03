"""The game's Capture the Flag rules, taken from the sources and run over
long games of players running for the flags, dying, returning and
capturing them, to record the flags and the scores the native engine's
must give.

    python3 tools/engine_ctf_reference.py          # writes the reference
    python3 tools/engine_ctf_reference.py --check  # fails if it is stale

The functions are source/game/game_engine_ctf.c's flag rules (the flag's
update with its untouched return, the player's update that captures, the
game's update that ends it at the score to win, the pickup that grabs or
returns a flag and the one that refuses a team its own flag at home, the
score, the capture's award, the drop and the reset) and
source/game/game_engine.c's `game_engine_flag_reset`, found by name as
tools/harness finds them, and compiled for this machine against a small
fake world: two flags, their stands, a table of players with their units,
and stand-ins for the messages, sounds and goals.

A game tick runs as `game_tick` runs it: the game engine's update (the
flags' updates, then the players', then the game type's), the objects'
update (a carried flag is owned this tick, as `item_update` has it), then
the players' pickups. The game's players pick up by colliding with an item;
the fake world's touch a flag lying within a metre, the lowest player first
and then the lowest flag, and one carrying a flag touches none. The players
walk on a grid of quarter metres toward the places they make for (a stand,
a flag, anywhere), one step on each axis a tick, so that the engine can
walk them the same way; they die now and then, dropping what they carry,
and come back a while later.

The trace is written to tools/traces/ctf_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "ctf_reference.txt"

CTF_FUNCTIONS = [
    "ctf_set_flag_warning",
    "ctf_sound_update_warning",
    "ctf_find_flag_carrier",
    "ctf_reset_flag",
    "ctf_player_drop_flag",
    "ctf_position_near_flag",
    "ctf_award_capture",
    "ctf_flag_failure_sound",
    "ctf_engine_allow_pick_up",
    "ctf_engine_player_update",
    "ctf_engine_weapon_update",
    "ctf_weapon_pickup",
    "ctf_engine_update",
    "ctf_get_score",
]
ENGINE_FUNCTIONS = ["game_engine_flag_reset", "game_engine_player_has_flag"]

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef unsigned char byte;
typedef float real;
typedef unsigned short wchar_t_unused;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define TICKS_PER_SECOND 30
#define MAXIMUM_WEAPONS_PER_UNIT 4
#define FLAG(b) (1u << (b))
#define TEST_FLAG(f, b) (((f) & FLAG(b)) != 0)
#define SET_FLAG(f, b, v) ((v) ? ((f) |= FLAG(b)) : ((f) &= ~FLAG(b)))
#define VALID_INDEX(i, n) ((i) >= 0 && (i) < (n))
#define match_assert(file, line, condition) ((void)0)

enum get_score_type { _get_score_individual, _get_score_team };
enum { _team_red, _team_blue };
enum { _item_attached_to_unit_bit = 0 };
enum { _object_connected_to_map_bit = 0 };
enum { _weapon_runtime_game_engine_active_bit = 3 };

enum
{
	NUMBER_OF_CTF_TEAMS = 2,
	CTF_WARNING_SOUND_INTERVAL = 20 * TICKS_PER_SECOND,
	CTF_FLAG_FAILURE_SOUND_INTERVAL = 4 * TICKS_PER_SECOND,
	CTF_FLAG_RETURN_TIME = 17 * TICKS_PER_SECOND,
};

enum { _ctf_weapon_handled_bit = 6 };

enum
{
	_multiplayer_sound_blue_team_has_the_flag = 0x8,
	_multiplayer_sound_red_team_has_the_flag = 0xB,
	_multiplayer_sound_flag_failure = 0x1C,
	_multiplayer_sound_blue_team_ctf = 0x25,
	_multiplayer_sound_red_team_ctf = 0x26,
};

enum { _ctf_message_time_expired = 0x2B };

typedef struct { real x, y, z; } real_point3d;
typedef struct { real i, j, k; } real_vector3d;
static real_vector3d global_forward3d_ = { 1.f, 0.f, 0.f };
static real_vector3d global_up3d_ = { 0.f, 0.f, 1.f };
/* (the game's are pointers) */
#define global_forward3d (&global_forward3d_)
#define global_up3d (&global_up3d_)

struct scenario_netgame_flag { real_point3d position; };

struct weapon_datum
{
	struct
	{
		unsigned long flags;
		long owner_team_index;
		long parent_object_index;
		real_point3d position;
	} object;
	struct { unsigned long flags; long last_owned_time; long ignore_object_index; } item;
	struct { unsigned long flags; } weapon;
};

struct unit_datum
{
	struct { real_point3d position; } object;
	struct { short current_weapon_index; long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT]; } unit;
};

struct player_datum
{
	long team_index;
	long unit_index;
	long local_player_index;
	struct
	{
		struct
		{
			struct { short flag_grabs; short flag_returns; short flag_scores; } ctf_statistics;
		} multiplayer_statistics;
	} statistics;
};

/* the variant's Capture the Flag options */
static struct
{
	struct
	{
		struct { boolean assault; boolean flag_must_reset; boolean flag_at_home_to_score; long single_flag_time; } ctf;
	} game_engine_variant;
} variant;
static void *game_engine_get_variant(void) { return &variant; }
#define game_engine_get_variant() (&variant)

/* the flags are objects 0 and 1, the players' units 100 and on */
#define UNIT_BASE 100
#define MAXIMUM_PLAYERS 8

static struct weapon_datum flag_weapons[NUMBER_OF_CTF_TEAMS];
static struct unit_datum units[MAXIMUM_PLAYERS];
static struct player_datum players[MAXIMUM_PLAYERS];
static long player_count;
static void *player_data;
static struct scenario_netgame_flag stands[NUMBER_OF_CTF_TEAMS];

static struct weapon_datum *weapon_get(long index) { return &flag_weapons[index]; }
static struct weapon_datum *weapon_try_and_get(long index) { return VALID_INDEX(index, NUMBER_OF_CTF_TEAMS) ? &flag_weapons[index] : NULL; }
static boolean weapon_is_flag(long index) { return VALID_INDEX(index, NUMBER_OF_CTF_TEAMS); }
static struct unit_datum *unit_get(long index) { return &units[index - UNIT_BASE]; }
static struct player_datum *player_get(long index) { return &players[index]; }
static long player_index_from_unit_index(long unit_index) { return unit_index - UNIT_BASE; }

static boolean unit_has_weapon(long unit_index, long weapon_index)
{
	struct unit_datum *unit = unit_get(unit_index);
	long slot;

	for (slot = 0; slot < MAXIMUM_WEAPONS_PER_UNIT; slot++)
	{
		if (unit->unit.weapon_object_indices[slot] == weapon_index)
			return TRUE;
	}
	return FALSE;
}

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

static long game_time;
static long game_time_get(void) { return game_time; }

/* the postgame: the game is over when the game type ends it */
static boolean game_over;
static boolean game_engine_can_score(void) { return !game_over; }
static void game_engine_end_game(void) { game_over = TRUE; }

/* a host with no clients */
static boolean network_game_distributed_client(void) { return FALSE; }
static byte distributed_player_to_byte(long player_index) { return (byte)player_index; }
static void ctf_client_flag_failure_update(long player_index) {}

static struct
{
	byte captures[NUMBER_OF_CTF_TEAMS];
	byte capturers[NUMBER_OF_CTF_TEAMS];
	byte grabs[NUMBER_OF_CTF_TEAMS];
	byte grabbers[NUMBER_OF_CTF_TEAMS];
	byte returns[NUMBER_OF_CTF_TEAMS];
	byte returners[NUMBER_OF_CTF_TEAMS];
} ctf_events;

struct ctf_globals
{
	struct scenario_netgame_flag *flags[NUMBER_OF_CTF_TEAMS];
	long weapon_indices[NUMBER_OF_CTF_TEAMS];
	long scores[NUMBER_OF_CTF_TEAMS];
	long score_to_win;
	boolean flag_warnings[NUMBER_OF_CTF_TEAMS];
	long flag_warning_ticks[NUMBER_OF_CTF_TEAMS];
	long flag_swap_timer;
	long next_flag_failure_time;
};
static struct ctf_globals ctf_globals;

/* what the trace does not show: the messages, the sounds and the goals */
static void game_show_score(long player_index, long message) {}
static void game_engine_play_multiplayer_sound(long sound) {}
static void game_engine_clear_goal_position(long goal) {}
static void game_engine_set_goal_position(long goal, real_point3d const *position, real radius,
	char const *name, long player, long team, long carrier) {}
static void game_engine_player_depower_active_camo(long player_index) {}
static void ctf_show_capture(long player_index, long team_index) {}
static void ctf_show_grab(long player_index) {}
static void ctf_show_touch_return(long player_index) {}
static void ctf_show_untouched_return(long team_index) {}
static void ctf_single_flag_what_is_up_message(long team_index) {}

static void item_get_position_even_if_in_inventory(long index, real_point3d *position)
{
	*position = flag_weapons[index].object.position;
}

static void object_set_position(long index, real_point3d const *position,
	real_vector3d const *forward, real_vector3d const *up)
{
	flag_weapons[index].object.position = *position;
}

static void object_reset(long index) {}

/* a unit drops the flag it holds where it stands */
static void detach(long weapon_index, long unit_index)
{
	struct weapon_datum *weapon = &flag_weapons[weapon_index];
	struct unit_datum *unit = unit_get(unit_index);
	long slot;

	for (slot = 0; slot < MAXIMUM_WEAPONS_PER_UNIT; slot++)
	{
		if (unit->unit.weapon_object_indices[slot] == weapon_index)
			unit->unit.weapon_object_indices[slot] = NONE;
	}
	unit->unit.current_weapon_index = NONE;
	SET_FLAG(weapon->item.flags, _item_attached_to_unit_bit, FALSE);
	SET_FLAG(weapon->object.flags, _object_connected_to_map_bit, TRUE);
	weapon->object.parent_object_index = NONE;
	weapon->object.position = unit->object.position;
}

static void unit_drop_current_weapon(long unit_index, boolean delete_it)
{
	struct unit_datum *unit = unit_get(unit_index);

	if (unit->unit.current_weapon_index != NONE)
		detach(unit->unit.weapon_object_indices[unit->unit.current_weapon_index], unit_index);
}
"""

MAIN = r"""
static unsigned long seed = 0x5eed0c7f;

static int next_random(int n)
{
	seed = seed * 1664525u + 1013904223u;
	return (int)((seed >> 8) % (unsigned long)n);
}

/* where each player stands and makes for, in quarter metres */
static int at[MAXIMUM_PLAYERS][2];
static int goal[MAXIMUM_PLAYERS][2];
static int dead_until[MAXIMUM_PLAYERS];
static int stand_at[NUMBER_OF_CTF_TEAMS][2];

static int sign(int value) { return value > 0 ? 1 : value < 0 ? -1 : 0; }

static long carried_by(long weapon_index)
{
	long player_index;

	for (player_index = 0; player_index < player_count; player_index++)
	{
		if (players[player_index].unit_index != NONE &&
			unit_has_weapon(players[player_index].unit_index, weapon_index))
			return player_index;
	}
	return NONE;
}

static void place(long player_index)
{
	struct unit_datum *unit = &units[player_index];

	unit->object.position.x = (real)at[player_index][0] * 0.25f;
	unit->object.position.y = (real)at[player_index][1] * 0.25f;
	unit->object.position.z = 0.f;
}

static void choose_goal(long player_index)
{
	long team = players[player_index].team_index;
	real_point3d *flag;

	switch (next_random(5))
	{
	case 0:
	case 1:
		flag = &flag_weapons[1 - team].object.position;
		break;
	case 2:
		flag = &stands[team].position;
		break;
	case 3:
		flag = &flag_weapons[team].object.position;
		break;
	default:
		goal[player_index][0] = next_random(161) - 40;
		goal[player_index][1] = next_random(81) - 40;
		printf("go %d %d %d\n", player_index, goal[player_index][0], goal[player_index][1]);
		return;
	}
	/* (a little off the mark now and then, to come near and not touch) */
	goal[player_index][0] = (int)(flag->x * 4.f) + next_random(9) - 4;
	goal[player_index][1] = (int)(flag->y * 4.f) + next_random(9) - 4;
	printf("go %d %d %d\n", player_index, goal[player_index][0], goal[player_index][1]);
}

static void print_state(void)
{
	long team;

	printf("state %d", game_time);
	for (team = 0; team < NUMBER_OF_CTF_TEAMS; team++)
	{
		struct weapon_datum *weapon = &flag_weapons[team];
		long carrier = carried_by(team);

		if (carrier != NONE)
			printf(" carried %d", carrier);
		else if (TEST_FLAG(weapon->weapon.flags, _ctf_weapon_handled_bit))
			printf(" dropped %.9g %.9g", weapon->object.position.x, weapon->object.position.y);
		else
			printf(" home");
	}
	for (team = 0; team < NUMBER_OF_CTF_TEAMS; team++)
		printf(" %d", ctf_globals.scores[team]);
	for (team = 0; team < player_count; team++)
		printf(" %d", ctf_get_score(team, _get_score_individual));
	printf(" %d\n", game_over);
}

static void play(int game)
{
	long index;
	long ticks = 3000;
	char last[512] = "";

	memset(flag_weapons, 0, sizeof(flag_weapons));
	memset(units, 0, sizeof(units));
	memset(players, 0, sizeof(players));
	memset(&ctf_globals, 0, sizeof(ctf_globals));
	game_over = FALSE;
	game_time = 0;

	variant.game_engine_variant.ctf.flag_must_reset = next_random(4) == 0;
	variant.game_engine_variant.ctf.flag_at_home_to_score = next_random(3) == 0;
	ctf_globals.score_to_win = 1 + next_random(5);
	stand_at[0][0] = next_random(9) - 4;
	stand_at[0][1] = next_random(9) - 4;
	stand_at[1][0] = 60 + next_random(41);
	stand_at[1][1] = next_random(41) - 20;
	printf("game %d %d %d %d %d %d %d %d\n", game,
		variant.game_engine_variant.ctf.flag_must_reset,
		variant.game_engine_variant.ctf.flag_at_home_to_score,
		ctf_globals.score_to_win,
		stand_at[0][0], stand_at[0][1], stand_at[1][0], stand_at[1][1]);

	for (index = 0; index < NUMBER_OF_CTF_TEAMS; index++)
	{
		struct weapon_datum *weapon = &flag_weapons[index];

		stands[index].position.x = (real)stand_at[index][0] * 0.25f;
		stands[index].position.y = (real)stand_at[index][1] * 0.25f;
		stands[index].position.z = 0.f;
		ctf_globals.flags[index] = &stands[index];
		ctf_globals.weapon_indices[index] = index;
		weapon->object.owner_team_index = index;
		weapon->object.parent_object_index = NONE;
		SET_FLAG(weapon->object.flags, _object_connected_to_map_bit, TRUE);
		weapon->object.position = stands[index].position;
		weapon->item.last_owned_time = 0;
	}

	player_count = 2 + next_random(5);
	for (index = 0; index < player_count; index++)
	{
		long slot;

		players[index].team_index = index % 2;
		players[index].unit_index = UNIT_BASE + index;
		players[index].local_player_index = NONE;
		units[index].unit.current_weapon_index = NONE;
		for (slot = 0; slot < MAXIMUM_WEAPONS_PER_UNIT; slot++)
			units[index].unit.weapon_object_indices[slot] = NONE;
		at[index][0] = stand_at[index % 2][0] + next_random(9) - 4;
		at[index][1] = stand_at[index % 2][1] + next_random(9) - 4;
		place(index);
		dead_until[index] = 0;
		printf("join %d %d %d\n", index % 2, at[index][0], at[index][1]);
		choose_goal(index);
	}

	for (game_time = 1; game_time <= ticks && !game_over; game_time++)
	{
		char now[512];

		/* deaths and returns, between the ticks */
		for (index = 0; index < player_count; index++)
		{
			struct player_datum *player = &players[index];

			if (player->unit_index != NONE && next_random(160) == 0)
			{
				long slot;
				struct unit_datum *unit = unit_get(player->unit_index);

				for (slot = 0; slot < MAXIMUM_WEAPONS_PER_UNIT; slot++)
				{
					if (unit->unit.weapon_object_indices[slot] != NONE)
						detach(unit->unit.weapon_object_indices[slot], player->unit_index);
				}
				player->unit_index = NONE;
				dead_until[index] = game_time + 100 + next_random(200);
				printf("kill %d %d\n", game_time, index);
			}
			else if (player->unit_index == NONE && game_time >= dead_until[index])
			{
				player->unit_index = UNIT_BASE + index;
				at[index][0] = stand_at[index % 2][0] + next_random(9) - 4;
				at[index][1] = stand_at[index % 2][1] + next_random(9) - 4;
				place(index);
				printf("spawn %d %d %d %d\n", game_time, index, at[index][0], at[index][1]);
				choose_goal(index);
			}
		}

		/* the game engine's update */
		for (index = 0; index < NUMBER_OF_CTF_TEAMS; index++)
			ctf_engine_weapon_update(index, &flag_weapons[index]);
		for (index = 0; index < player_count; index++)
			ctf_engine_player_update(index);
		ctf_engine_update();

		/* the objects' update: a carried flag goes with its carrier */
		for (index = 0; index < NUMBER_OF_CTF_TEAMS; index++)
		{
			struct weapon_datum *weapon = &flag_weapons[index];
			long carrier = carried_by(index);

			if (TEST_FLAG(weapon->item.flags, _item_attached_to_unit_bit))
				weapon->item.last_owned_time = game_time;
			if (carrier != NONE)
				weapon->object.position = units[carrier].object.position;
		}

		/* the players' update: they walk, then pick up */
		for (index = 0; index < player_count; index++)
		{
			if (players[index].unit_index == NONE)
				continue;
			if (at[index][0] == goal[index][0] && at[index][1] == goal[index][1])
				choose_goal(index);
			at[index][0] += sign(goal[index][0] - at[index][0]);
			at[index][1] += sign(goal[index][1] - at[index][1]);
			place(index);
		}
		for (index = 0; index < player_count && !game_over; index++)
		{
			struct player_datum *player = &players[index];
			long weapon_index;

			if (player->unit_index == NONE || game_engine_player_has_flag(index))
				continue;
			for (weapon_index = 0; weapon_index < NUMBER_OF_CTF_TEAMS; weapon_index++)
			{
				struct weapon_datum *weapon = &flag_weapons[weapon_index];
				struct unit_datum *unit = unit_get(player->unit_index);
				real dx = weapon->object.position.x - unit->object.position.x;
				real dy = weapon->object.position.y - unit->object.position.y;
				real dz = weapon->object.position.z - unit->object.position.z;
				real distance_squared = dx * dx;

				distance_squared += dy * dy;
				distance_squared += dz * dz;
				if (TEST_FLAG(weapon->item.flags, _item_attached_to_unit_bit) || distance_squared >= 1.f)
					continue;
				if (!ctf_engine_allow_pick_up(player->unit_index, weapon_index))
					continue;
				if (ctf_weapon_pickup(weapon_index, index))
				{
					unit->unit.weapon_object_indices[2] = weapon_index;
					unit->unit.current_weapon_index = 2;
					SET_FLAG(weapon->item.flags, _item_attached_to_unit_bit, TRUE);
					SET_FLAG(weapon->object.flags, _object_connected_to_map_bit, FALSE);
					weapon->object.parent_object_index = player->unit_index;
					weapon->object.position = unit->object.position;
					break;
				}
			}
		}

		/* the flags and the scores, when they change */
		{
			long n = 0;
			long team;

			for (team = 0; team < NUMBER_OF_CTF_TEAMS; team++)
			{
				long carrier = carried_by(team);

				if (carrier != NONE)
					n += snprintf(now + n, sizeof(now) - n, " c%d", carrier);
				else
					n += snprintf(now + n, sizeof(now) - n, " %d:%.9g:%.9g",
						TEST_FLAG(flag_weapons[team].weapon.flags, _ctf_weapon_handled_bit),
						flag_weapons[team].object.position.x, flag_weapons[team].object.position.y);
			}
			for (team = 0; team < player_count; team++)
				n += snprintf(now + n, sizeof(now) - n, " %d", ctf_get_score(team, _get_score_team));
			snprintf(now + n, sizeof(now) - n, " %d", game_over);
			if (strcmp(now, last))
			{
				print_state();
				strcpy(last, now);
			}
		}
	}
	printf("end %d\n", game_time - 1);
}

int main(void)
{
	int game;

	for (game = 0; game < 60; game++)
		play(game);
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    ctf_c = read("source/game/game_engine_ctf.c")
    engine_c = read("source/game/game_engine.c")
    prototypes = "\n".join(
        [
            "static void ctf_reset_flag(long weapon_index);",
            "static void ctf_player_drop_flag(long player_index, long weapon_index);",
            "static boolean ctf_position_near_flag(long team_index, real_point3d const *position, real radius);",
            "static void ctf_award_capture(long player_index, long team_index);",
            "static void ctf_flag_failure_sound(long player_index);",
            "static long ctf_find_flag_carrier(long weapon_index);",
            "static void ctf_set_flag_warning(long team_index, boolean warning);",
            "static void ctf_sound_update_warning(long team_index);",
            "static long ctf_get_score(long player_index, enum get_score_type score_type);",
            "static boolean game_engine_player_has_flag(long player_index);",
            "static void game_engine_flag_reset(long weapon_index, real_point3d const *position);",
        ]
    )
    functions = [function(engine_c, name) for name in ENGINE_FUNCTIONS]
    functions += [function(ctf_c, name) for name in CTF_FUNCTIONS]
    # (the game's are global; the fake world's are the file's own)
    functions = [
        text.replace("boolean game_engine_player_has_flag(", "static boolean game_engine_player_has_flag(", 1).replace(
            "void game_engine_flag_reset(", "static void game_engine_flag_reset(", 1
        )
        if text.lstrip().startswith(("boolean game_engine_player_has_flag", "void game_engine_flag_reset"))
        else text
        for text in functions
    ]
    return "\n\n".join([FAKE_WORLD, prototypes, *functions, MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="ctf-reference-") as work:
        source = Path(work) / "ctf_reference.c"
        executable = Path(work) / "ctf_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-w", "-ffp-contract=off", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's Capture the Flag rules (tools/engine_ctf_reference.py).\n"
        "# game <n> <flag must reset> <flag at home to score> <score to win> <red stand x y> <blue stand x y>\n"
        "#   (the stands and the players' places in quarter metres)\n"
        "# join <team> <x> <y>: a player, standing there\n"
        "# go <player> <x> <y>: where a player makes for now\n"
        "# kill <tick> <player>: a player dies before the tick, dropping what it carries\n"
        "# spawn <tick> <player> <x> <y>: a player comes back there before the tick\n"
        "# state <tick> <red flag> <blue flag> <red score> <blue score> <each player's captures> <over>\n"
        "#   (after the tick, when it changed: a flag is home, carried <player> or dropped <x> <y>)\n"
        "# end <tick>: the game's last tick\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_ctf_reference.py")
        print("the capture the flag reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
