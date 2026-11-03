"""The game's default Slayer, Team Slayer and Capture the Flag variants,
taken from the sources, to record what the engine's `Variant` must hold.

    python3 tools/engine_game_variant_reference.py          # writes the reference
    python3 tools/engine_game_variant_reference.py --check  # fails if it is stale

The functions are source/game/game_engine.c's `build_game_variant_slayer`,
`build_game_variant_team_slayer` and `build_game_variant_ctf`, compiled for
this machine against a fake `struct game_variant` holding the fields they
set. The trace prints, for each, the fields the engine's rules read.

The trace is written to tools/traces/game_variant_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "game_variant_reference.txt"

FUNCTIONS = ["build_game_variant_slayer", "build_game_variant_team_slayer", "build_game_variant_ctf"]

FAKE_WORLD = r"""
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char boolean;

#define TRUE 1
#define FALSE 0
#define FLAG(b) (1u<<(b))
#define SET_FLAG(f,b,v) ((v) ? ((f)|=FLAG(b)) : ((f)&=~FLAG(b)))

enum { game_engine_none, game_engine_ctf, game_engine_slayer };
enum
{
	_game_variant_draw_object_in_motion_sensor_bit,
	_game_variant_allow_friendly_navpoints_bit,
	_game_variant_infinite_grenades_bit,
	_game_variant_no_shields_bit,
	_game_variant_always_invisible_bit,
	_game_variant_generic_starting_equipment_bit,
};
enum { _game_engine_vehicles_warthog = 2 };
enum { _game_engine_weapons_normal = 0 };
enum { _radar_motion_tracker = 0, _radar_nav_point = 1 };

/* the fields the builders set */
struct universal_variant
{
	unsigned long flags;
	long vehicle_set, goal_radar, weapon_set;
	real health;
	long lives;
	boolean odd_man_out, teams;
	long respawn_time, respawn_time_growth, score_to_win, suicide_penalty;
};

struct slayer_variant { boolean no_death_bonus, no_kill_penalty, kill_in_order; };
struct ctf_variant
{
	boolean assault, flag_at_home_to_score, flag_must_reset, reset_on_capture;
	long single_flag_time;
};

struct game_variant
{
	long game_engine_index;
	struct universal_variant universal_variant;
	union { struct slayer_variant slayer; struct ctf_variant ctf; } game_engine_variant;
	unsigned long flags;
};
"""

MAIN = r"""
static void print(char const *name, struct game_variant const *variant)
{
	struct universal_variant const *universal = &variant->universal_variant;
	int ctf = variant->game_engine_index==game_engine_ctf;

	printf("%s %d %d %d %d %d %d %d %d\n", name, ctf, universal->teams, universal->score_to_win,
		universal->respawn_time, universal->respawn_time_growth, universal->suicide_penalty,
		ctf ? variant->game_engine_variant.ctf.flag_must_reset : 0,
		ctf ? variant->game_engine_variant.ctf.flag_at_home_to_score : 0);
}

int main(void)
{
	struct game_variant variant;

	print("slayer", build_game_variant_slayer(&variant));
	print("team_slayer", build_game_variant_team_slayer(&variant));
	print("ctf", build_game_variant_ctf(&variant));
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    engine_c = read("source/game/game_engine.c")
    return "\n\n".join([FAKE_WORLD, *(function(engine_c, name) for name in FUNCTIONS), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="game-variant-reference-") as work:
        source = Path(work) / "game_variant_reference.c"
        executable = Path(work) / "game_variant_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's default variants (tools/engine_game_variant_reference.py).\n"
        "# <variant> <ctf> <teams> <score to win> <respawn> <respawn growth> <suicide penalty>"
        " <flag must reset> <flag at home to score>, times in the game's ticks\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_game_variant_reference.py")
        print("the game variant reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
