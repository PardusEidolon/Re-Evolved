"""The game's frame clock, taken from the sources and run over random frame
times and speeds, to record how many ticks each frame runs and the time it
leaves over, as the native engine's clock must.

    python3 tools/engine_game_time_reference.py          # writes the reference
    python3 tools/engine_game_time_reference.py --check  # fails if it is stale

The function is source/game/game_time.c's `game_time_update`, found by
name as tools/harness finds it and compiled for this machine against a fake
world: a local game (the cap of seven ticks, the time past it thrown away),
a client queue that lets every tick run, and a `game_tick` that counts.
Each run sets the game's speed and plays frames of random lengths through
it: steady frames of common displays, jittered ones, stalls of up to two
seconds and frames of nothing. The film's and the network's caps, the
server's time and what each tick does are left out.

The trace is written to tools/traces/game_time_reference.txt, which
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
from harness import enum_with, function, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "game_time_reference.txt"

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
#define TICKS_PER_SECOND 30
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define match_assert(file, line, expr) if (!(expr)) { printf("assert %d\n", line); }
#define profile_scope(variable, statement) statement

static long ticks_run;
static long client_ticks;

static long game_connection(void) { return _game_connection_local; }
static boolean game_time_held(void) { return FALSE; }
static void update_client_local_ticks(long ticks) { client_ticks += ticks; }
static long update_client_get_maximum_possible_server_time(void) { return client_ticks; }
static void network_game_server_update_ticks(void *server, short ticks) {}
static void *global_network_game_server_get(void) { return 0; }
static void game_tick(void) { ticks_run++; }
static void render_interpolation_tick(void) {}
static void network_distributed_tick(void) {}
static void render_interpolation_frame_begin(void) {}
static void render_interpolation_frame_end(void) {}
static void game_frame(real dt) {}
static boolean network_game_distributed_client(void) { return FALSE; }
"""

DRIVER = r"""
static struct game_time_globals_struct globals;

static unsigned long seed = 0x71c4e5;

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

static const real displays[] = { 1.f/30.f, 1.f/60.f, 1.f/120.f, 1.f/144.f, 1.f/240.f, 1.f/75.f };
static const real speeds[] = { 1.f, 1.f, 1.f, 0.5f, 2.f, 0.25f, 0.f, 1.5f };

int main(void)
{
	long run;

	game_time_globals = &globals;
	for (run = 0; run < 64; run++)
	{
		long frame;
		real display = displays[next_random(6)];
		real speed = run < 8 ? speeds[run] : speeds[next_random(8)];

		if (run >= 16 && next_random(4) == 0)
			speed = (real)next_random(3000) / 1000.f;
		memset(&globals, 0, sizeof globals);
		globals.initialized = TRUE;
		globals.active = TRUE;
		globals.speed = speed;
		client_ticks = 0;
		printf("run %08x\n", bits(speed));
		for (frame = 0; frame < 160; frame++)
		{
			real dt;
			long kind = next_random(20);

			if (kind < 12)
				dt = display;
			else if (kind < 17)
				dt = display * (real)(500 + next_random(1000)) / 1000.f;
			else if (kind < 19)
				dt = (real)next_random(2000) / 1000.f;
			else
				dt = 0.f;
			ticks_run = 0;
			game_time_update(dt);
			printf("frame %08x %d %08x\n", bits(dt), ticks_run, bits(globals.leftover_dt));
		}
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    game_h = read("source/game/game.h")
    game_time = read("source/game/game_time.c")
    return "\n".join(
        [
            enum_with(game_h, "_game_connection_local"),
            "enum { SOME_LARGE_NUMBER_OF_TICKS = 1000 };",
            FAKE_WORLD,
            structure(game_time, "game_time_globals_struct"),
            "static struct game_time_globals_struct *game_time_globals;",
            function(game_time, "game_time_get_speed"),
            function(game_time, "game_time_update"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="game-time-reference-") as work:
        source = Path(work) / "game_time_reference.c"
        executable = Path(work) / "game_time_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's frame clock (tools/engine_game_time_reference.py).\n"
        "# run <the game's speed, f32 bits>\n"
        "# frame <the frame's seconds, f32 bits> <ticks run> <the leftover seconds after, f32 bits>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_game_time_reference.py")
        print("the game time reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
