"""The game's tick (source/game/game.c): the order `game_tick` updates the
world's systems in, the seconds a tick lasts, and which updates a client of
another's game leaves to the host, recorded for the engine's tick to be
checked against.

    python3 tools/engine_game_tick_reference.py          # writes the reference
    python3 tools/engine_game_tick_reference.py --check  # fails if it is stale

`game_tick` is game.c's, compiled for this machine against the shared fake
world (tools/harness/world.py), with each system it updates a stand-in that
says its name (and the seconds it was given), over a tick of each kind of
game: alone, with the players at double speed, as a client of the
distributed netcode, and as a client of network co-op. The trace is written
to tools/traces/game_tick_reference.txt, which
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
from harness import function, read, world  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "game_tick_reference.txt"

# the systems game_tick updates, each a stand-in that says its name
UPDATES = [
    "game_state_check_data_arrays",
    "lights_port_recover",
    "cheats_network_client_enforce",
    "remove_quitting_players_from_game",
    "game_allegiance_update",
    "units_update",
    "ai_update",
    "network_actors_drive",
    "players_update_before_game",
    "rumble_update",
    "first_person_weapons_update",
    "game_engine_update",
    "editor_update",
    "hs_update",
    "recorded_animations_update",
    "objects_update",
    "players_update_after_game",
    "movement_trace_update",
    "placement_trace_update",
    "hud_update",
    "player_effect_update",
]

# the systems given the tick's seconds
TIMED = ["effects_update", "antennas_update", "physics_jolt_update"]

FAKE_WORLD = r"""
typedef float real;
enum { TICKS_PER_SECOND = 30 };

/* the profiler and the collision log, which change nothing */
#define profile_tick_start()
#define profile_tick_end()
#define profile_enter(section)
#define profile_exit(section)
#define profile_scope(section, ...) { __VA_ARGS__ }
#define collision_log_begin_period(period)
#define collision_log_end_period()
#define real_math_reset_precision()

/* the game's globals, as much of them as a tick reads */
static struct { boolean active; boolean players_are_double_speed; } game_globals_storage;
static void *const game_globals_pointer = &game_globals_storage;
#define game_globals (&game_globals_storage)

/* the kind of game this is */
static boolean distributed_client;
static boolean coop;
static boolean network_game_distributed_client(void) { return distributed_client; }
static boolean network_coop_active(void) { return coop; }

/* the global random seed, which the first-person weapons may not move */
static void lock_global_random_seed(void) { printf("lock_global_random_seed\n"); }
static void unlock_global_random_seed(void) { printf("unlock_global_random_seed\n"); }
"""

MAIN = r"""
static void tick(char const *kind, boolean double_speed, boolean distributed, boolean co_op)
{
	game_globals_storage.active = TRUE;
	game_globals_storage.players_are_double_speed = double_speed;
	distributed_client = distributed;
	coop = co_op;
	printf("tick %s\n", kind);
	game_tick();
	printf("end\n");
}

int main(void)
{
	tick("alone", FALSE, FALSE, FALSE);
	tick("double_speed", TRUE, FALSE, FALSE);
	tick("distributed_client", FALSE, TRUE, FALSE);
	tick("coop_client", FALSE, TRUE, TRUE);
	return 0;
}
"""


def stand_ins():
    """A stand-in for each system, which says its name."""
    lines = [f'static void {name}(void) {{ printf("{name}\\n"); }}' for name in UPDATES]
    lines += [f'static void {name}(real seconds) {{ printf("{name} %.9g\\n", seconds); }}' for name in TIMED]
    return "\n".join(lines)


def source_file():
    """The fake world, the game's function and the cases, as one C file."""
    game_c = read("source/game/game.c")
    return "\n\n".join([world.prelude(), FAKE_WORLD, stand_ins(), function(game_c, "game_tick"), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="game-tick-reference-") as work:
        source = Path(work) / "game_tick_reference.c"
        executable = Path(work) / "game_tick_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's tick (tools/engine_game_tick_reference.py).\n"
        "# tick <kind>, then each system game_tick updates, in order (with the seconds it is given), then end\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_game_tick_reference.py")
        print("the game tick reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
