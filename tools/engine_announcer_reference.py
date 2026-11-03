"""The game's multiplayer announcer, taken from the sources and run over
games of every type, to record which of its sounds play and when, as the
native engine's announcer must.

    python3 tools/engine_announcer_reference.py          # writes the reference
    python3 tools/engine_announcer_reference.py --check  # fails if it is stale

The functions are source/game/game_engine_multiplayer_sounds.c's queue
(`game_engine_intialize_queued_sounds`, `game_engine_play_multiplayer_sound`,
`game_engine_update_multiplayer_sound` and the static helpers behind them),
source/game/game_engine.c's `game_engine_initialize_for_new_map` (which
empties the queue), `game_engine_game_starting` and
`game_engine_has_teams`, and the game types' `game_starting`: Slayer's,
Capture the Flag's, King of the Hill's and Oddball's. They are found by
name as tools/harness finds them and compiled for this machine against a
fake world: the globals' multiplayer information with a table of sounds,
some missing, of random lengths, and a stand-in for the sound that prints
what plays. Each game starts as a new map and `game_initial_pulse` start it, then plays random sounds over its ticks, the
queue stepping once a tick as `game_engine_update` steps it. Ting (the last
sound), which nothing plays and the queue's flags do not reach, is left
out.

The trace is written to tools/traces/announcer_reference.txt, which
tools/engine_wasm_traces.py checks game.wasm against.
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from harness import enum_with, function, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "announcer_reference.txt"

SOUND_FUNCTIONS = [
    "_game_engine_play_multiplayer_sound",
    "push_queued_sound",
    "game_engine_update_multiplayer_sound",
    "get_sound_length_in_ticks",
    "game_engine_play_multiplayer_sound",
    "game_engine_intialize_queued_sounds",
]
ENGINE_FUNCTIONS = ["game_engine_has_teams", "game_engine_initialize_for_new_map", "game_engine_game_starting"]
STARTS = [
    ("source/game/game_engine_slayer.c", "slayer_engine_game_starting"),
    ("source/game/game_engine_ctf.c", "ctf_engine_game_starting"),
    ("source/game/game_engine_king.c", "king_engine_game_starting"),
    ("source/game/game_engine_oddball.c", "oddball_engine_game_starting"),
]

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
#define csmemset memset

struct tag_reference { long group_tag; char *name; long name_length; long index; };
struct tag_block { long count; void *address; void *definition; };

#define TAG_BLOCK_GET_ELEMENT(block, element, type) \
	((element) >= 0 && (element) < (block)->count ? (type *)(block)->address + (element) : (type *)NULL)

struct sound_definition { long longest_permutation_length; };

#define MAXIMUM_SOUNDS 48
#define SOUND_TAG_BASE 1000

static struct sound_definition sound_definitions[MAXIMUM_SOUNDS];
static long tick;

static struct sound_definition *sound_definition_get(long index)
{
	return &sound_definitions[index - SOUND_TAG_BASE];
}

static void unspatialized_impulse_sound_new(long index, real scale)
{
	printf("play %d %d\n", tick, index - SOUND_TAG_BASE);
}

static void global_scenario_get(void) {}
"""

GLOBALS = r"""
static struct game_globals_multiplayer_sound_view the_game_globals;
static struct game_globals_multiplayer_information the_multiplayer_information;
static struct tag_reference the_sounds[MAXIMUM_SOUNDS];

static void *scenario_get_game_globals(void) { return &the_game_globals; }
"""

VARIANT = r"""
struct universal_variant { boolean teams; };
struct game_variant { struct universal_variant universal_variant; };
static struct game_variant global_variant;

struct game_engine
{
	void (*game_starting)(void);
	boolean (*initialize_for_new_map)(void);
	void (*dispose)(void);
};
static struct game_engine *game_engine;

/* what a new map resets besides the queue, which the trace leaves out */
struct game_engine_globals { long next_team_index; };
static struct game_engine_globals game_engine_globals;
static long global_goal[16];
static short game_engine_betrayal_penalty[16];
static long game_engine_vehicle_home_count;
static long timeout_for_endgame_sound;
static boolean game_engine_network_state_read;
#define _error_silent 0
static void error(long priority, char const *message) {}
static void game_engine_verify_current_map(void) {}
static void game_engine_predict_resources(void) {}

/* the lighting the game builds for its type, which the trace leaves out */
static void game_engine_build_lighting(void) {}
"""

MAIN = r"""
static unsigned long seed = 0x5eed1e5;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static struct game_engine engines[] = {
	{ slayer_engine_game_starting, NULL, NULL },
	{ ctf_engine_game_starting, NULL, NULL },
	{ king_engine_game_starting, NULL, NULL },
	{ oddball_engine_game_starting, NULL, NULL },
};

/* Halo's numbers for the game types: Slayer, Capture the Flag, King, Oddball */
static long const game_types[] = { 2, 1, 4, 3 };

static void play(long engine, long teams, long count, long ticks)
{
	long index;

	game_engine = &engines[engine];
	global_variant.universal_variant.teams = teams;
	the_game_globals.multiplayer_information.count = next_random(4) ? 1 : 0;
	the_game_globals.multiplayer_information.address = &the_multiplayer_information;
	the_multiplayer_information.sounds.count = count;
	the_multiplayer_information.sounds.address = the_sounds;
	printf("game %d %d %d\n", game_types[engine], teams, the_game_globals.multiplayer_information.count);
	for (index = 0; index < count; index++)
	{
		boolean missing = next_random(8) == 0;
		long length = next_random(3) ? next_random(4000) : next_random(200);

		the_sounds[index].index = missing ? NONE : SOUND_TAG_BASE + index;
		sound_definitions[index].longest_permutation_length = length;
		printf("sound %d %d\n", missing ? NONE : index, length);
	}

	tick = 0;
	game_engine_initialize_for_new_map();
	game_engine_game_starting();
	for (tick = 1; tick <= ticks; tick++)
	{
		game_engine_update_multiplayer_sound();
		if (next_random(40) == 0)
		{
			long calls = 1 + next_random(next_random(3) ? 1 : 7);

			while (calls--)
			{
				long sound = next_random(NUMBER_OF_MULTIPLAYER_INFORMATION_SOUNDS - 1);

				printf("call %d %d\n", tick, sound);
				game_engine_play_multiplayer_sound(sound);
			}
		}
	}
	printf("end %d\n", ticks);
}

int main(void)
{
	long engine;
	long teams;
	long round;

	for (round = 0; round < 60; round++)
		for (engine = 0; engine < 4; engine++)
			for (teams = 0; teams < 2; teams++)
			{
				long count = next_random(3) ? NUMBER_OF_MULTIPLAYER_INFORMATION_SOUNDS : next_random(MAXIMUM_SOUNDS);

				play(engine, teams, count, 300 + next_random(1500));
			}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    sounds_c = read("source/game/game_engine_multiplayer_sounds.c")
    game_globals_h = read("source/game/game_globals.h")
    game_engine_c = read("source/game/game_engine.c")
    constants = [
        enum_with(sounds_c, "_multiplayer_sound_play_ball") + ";",
        enum_with(sounds_c, "MAXIMUM_QUEUED_MULTIPLAYER_SOUNDS") + ";",
    ]
    structures = [
        structure(game_globals_h, "game_globals_multiplayer_information"),
        structure(sounds_c, "queued_multiplayer_sound"),
        structure(sounds_c, "game_globals_multiplayer_sound_view"),
    ]
    flags = sounds_c[sounds_c.index("static boolean sound_is_queueable") :]
    flags = flags[: flags.index("};") + 2]
    queue = sounds_c[sounds_c.index("static long mp_sound_queue_count") :]
    queue = queue[: queue.index("};") + 2]
    prototypes = [
        "static void _game_engine_play_multiplayer_sound(long sound_index);",
        "static void push_queued_sound(long sound_index, long delay_ticks);",
        "static long get_sound_length_in_ticks(long sound_index);",
        "void game_engine_play_multiplayer_sound(long sound_index);",
    ]
    sounds = [function(sounds_c, name) for name in SOUND_FUNCTIONS]
    engine = [function(game_engine_c, name) for name in ENGINE_FUNCTIONS]
    starts = []
    for path, name in STARTS:
        text = read(path)
        start = function(text, name)
        # Each game type's file names the sounds it plays in an enum of its
        # own, with some names the sounds file does not have: those few are
        # taken, with their values, from the game type's file.
        for sound in sorted(set(re.findall(r"_multiplayer_sound_\w+", start))):
            if not re.search(rf"\b{sound}\b", constants[0]):
                value = re.search(rf"\b{sound}\s*=\s*(\w+)", text)
                if value is None:
                    raise SystemExit(f"{path} does not give {sound} a value")
                starts.append(f"#define {sound} {value.group(1)}")
        starts.append(start)
    return "\n\n".join(
        [
            FAKE_WORLD,
            *constants,
            *structures,
            GLOBALS,
            flags,
            queue,
            *prototypes,
            *sounds,
            VARIANT,
            *engine,
            *starts,
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="announcer-reference-") as work:
        source = Path(work) / "announcer_reference.c"
        executable = Path(work) / "announcer_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's multiplayer announcer (tools/engine_announcer_reference.py).\n"
        "# game <game type> <teams> <whether the globals have their multiplayer information>\n"
        "# sound <index, or -1 for a missing sound> <its longest permutation, milliseconds>\n"
        "# call <game tick> <sound>: the game asks for a sound\n"
        "# play <game tick> <sound>: a sound plays\n"
        "# end <game ticks>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_announcer_reference.py")
        print("the announcer reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
