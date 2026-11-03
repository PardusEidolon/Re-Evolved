"""When the game's multiplayer rules end a game, taken from the sources, to
check the engine's `Rules` against.

    python3 tools/engine_game_end_reference.py          # writes the reference
    python3 tools/engine_game_end_reference.py --check  # fails if it is stale

The functions are source/game/game_engine.c's `game_engine_should_end_game`,
`multiple_teams_alive` and `players_in_game`, compiled for this machine
against a fake player table. A game ends when fewer than two teams have a
player in it, or when its time limit is up. The cases are every game of up
to five players, with and without teams, each player on either team, alive
or dead, with no time limit or one of a minute, just before and at its end.

The engine has no lives and no odd man out, and no one quits, so the fake
`game_engine_player_is_out_of_lives` and `game_engine_player_is_odd_man_out`
say no and no player has quit: every player counts as in the game.

The trace is written to tools/traces/game_end_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "game_end_reference.txt"

FUNCTIONS = ["players_in_game", "multiple_teams_alive", "game_engine_should_end_game"]

FAKE_WORLD = r"""
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define TICKS_PER_SECOND 30
#define match_assert(file, line, condition) ((void)0)

#define MAXIMUM_PLAYERS 5

struct player_datum
{
	long team_index;
	long unit_index;
	boolean quit_out_of_game;
};

static struct player_datum players[MAXIMUM_PLAYERS];
static long player_count;
static void *player_data;

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

/* (the engine has neither lives nor an odd man out) */
static boolean game_engine_player_is_odd_man_out(long player_index) { return FALSE; }
static boolean game_engine_player_is_out_of_lives(long player_index) { return FALSE; }

struct game_engine { long index; };
static struct game_engine slayer;
static struct game_engine *game_engine = &slayer;

struct game_variant_options { long time_limit; };
static struct game_variant_options options;
static struct game_variant_options *game_variant_options_get(void) { return &options; }

static long game_time;
static long game_time_get(void) { return game_time; }
"""

MAIN = r"""
static void play(boolean teams, long count, long mask, long alive, long limit, long time)
{
	long index;

	player_count = count;
	options.time_limit = limit;
	game_time = time;
	printf("game %d %d %d", teams, limit, time);
	for (index = 0; index < count; index++)
	{
		players[index].team_index = teams ? (mask >> index) & 1 : index;
		players[index].unit_index = (alive >> index) & 1 ? index : NONE;
		players[index].quit_out_of_game = FALSE;
		printf(" %d%c", players[index].team_index, players[index].unit_index == NONE ? 'd' : 'a');
	}
	printf(" end %d\n", game_engine_should_end_game());
}

int main(void)
{
	static long const times[][2] = { { 0, 0 }, { 0, 600 }, { 1, 1799 }, { 1, 1800 } };
	long teams, count, mask, alive, at;

	for (teams = 0; teams < 2; teams++)
		for (count = 0; count <= MAXIMUM_PLAYERS; count++)
			for (mask = 0; mask < (teams ? 1 << count : 1); mask++)
				for (alive = 0; alive < 1 << count; alive++)
					for (at = 0; at < 4; at++)
						play(teams, count, mask, alive, times[at][0], times[at][1]);
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    engine_c = read("source/game/game_engine.c")
    return "\n\n".join([FAKE_WORLD, *(function(engine_c, name) for name in FUNCTIONS), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="game-end-reference-") as work:
        source = Path(work) / "game_end_reference.c"
        executable = Path(work) / "game_end_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# When the game's rules end a game (tools/engine_game_end_reference.py).\n"
        "# game <teams> <time limit, minutes> <game time, the game's ticks> <each player's team,"
        " a(live) or d(ead)>... end <whether the game ends>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_game_end_reference.py")
        print("the game end reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
