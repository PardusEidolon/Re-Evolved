"""The game's save and speech predicates, taken from the sources and run
over every combination of the dangers they weigh, to record when it is
quiet, safe to save and safe to speak, as the native engine's must say.

    python3 tools/engine_safety_reference.py          # writes the reference
    python3 tools/engine_safety_reference.py --check  # fails if it is stale

The functions are source/game/game.c's `game_all_quiet`,
`game_safe_to_save`, `game_safe_to_speak` and `game_is_cooperative`, found
by name as tools/harness finds them and compiled for this machine against
a fake world whose eight dangers (`ai_enemies_can_see_player`,
`dangerous_projectiles_near_player` and the rest) are set from the bits of
a number, all 256 of them, and whose `player_spawn_count` runs from none to
four. How the game decides each danger is left out.

The trace is written to tools/traces/safety_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "safety_reference.txt"

# The dangers, by their bit in the trace's number.
DANGERS = [
    "ai_enemies_can_see_player",
    "dangerous_projectiles_near_player",
    "dangerous_items_near_player",
    "dangerous_effects_near_player",
    "any_unit_is_dangerous",
    "any_player_is_in_the_air",
    "any_player_is_dead",
    "vehicle_moving_near_any_player",
]

FAKE_WORLD = r"""
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;

#define TRUE 1
#define FALSE 0

static boolean debug_game_save = FALSE;
static void console_warning(char const *format, ...) {}
static long player_spawn_count;
static long dangers;
"""

DRIVER = r"""
int main(void)
{
	for (dangers = 0; dangers < 256; dangers++)
		printf("dangers %d %d %d %d\n", dangers, game_all_quiet(), game_safe_to_save(), game_safe_to_speak());
	for (player_spawn_count = 0; player_spawn_count <= 4; player_spawn_count++)
		printf("players %d %d\n", player_spawn_count, game_is_cooperative());
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    game = read("source/game/game.c")
    stubs = [f"static boolean {name}(void) {{ return (dangers >> {bit}) & 1; }}" for bit, name in enumerate(DANGERS)]
    names = ["game_all_quiet", "game_safe_to_save", "game_safe_to_speak", "game_is_cooperative"]
    return "\n".join([FAKE_WORLD, *stubs, *(function(game, name) for name in names), DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="safety-reference-") as work:
        source = Path(work) / "safety_reference.c"
        executable = Path(work) / "safety_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    legend = "".join(f"#   bit {bit}: {name}\n" for bit, name in enumerate(DANGERS))
    return (
        "# The game's save and speech predicates (tools/engine_safety_reference.py).\n"
        "# dangers <the dangers' bits> <all quiet> <safe to save> <safe to speak>\n"
        + legend
        + "# players <player_spawn_count> <cooperative>\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_safety_reference.py")
        print("the safety reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
