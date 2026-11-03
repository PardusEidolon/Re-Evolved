"""The game's choice of an animation's permutation: which of a list of
animations, linked one to the next by their weights, a random number picks,
recorded for the engine's choice to match.

    python3 tools/engine_animation_permutation_reference.py          # writes the reference
    python3 tools/engine_animation_permutation_reference.py --check  # fails if it is stale

The function is source/models/model_animations.c's
`animation_choose_random_permutation_internal`, compiled for this machine
against a small fake world: a graph of ten animations whose lists are a run
of three, one alone, two that link in a loop, one that links past the
graph's last and a run that ends before the last weight is reached, its
random draw replaced by the number the trace asks with. Each list is asked
from each first animation (and indices the graph does not have) at numbers
from 0 to 1, on its weights and between them. The table is written to
tools/traces/animation_permutation_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "animation_permutation_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef float real;

#define NONE -1
#define _error_silent 0
#define VALID_INDEX(i,n) ((i)>=0 && (i)<(n))
#define match_assert(file, line, condition) ((void)0)
#define error(...) ((void)0)

enum { animation_update_kind_render_only = 0, animation_update_kind_affects_game_state };

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

/* the animation's fields the choice reads */
struct animation { real runtime_normalized_weight; short next_animation_index; };
struct animation_graph { struct tag_block animations; };

#define ANIMATIONS 10

/* a run of three, one alone, a loop of two, a link past the last, and a
run that ends before its last weight */
static struct animation animations[ANIMATIONS] =
{
	{ 0.25f, 1 }, { 0.6f, 2 }, { 1.0f, NONE },
	{ 1.0f, NONE },
	{ 0.3f, 5 }, { 0.5f, 4 },
	{ 0.4f, 12 },
	{ 0.5f, 8 }, { 0.5f, 9 }, { 0.9f, NONE },
};
static struct animation_graph graph = { { ANIMATIONS, animations } };

static struct animation_graph const *animation_graph_definition_get(long index) { return &graph; }
static int model_data_report_once(struct animation_graph const *graph) { return 0; }

/* the draw: the number the trace asks with */
static real asked;
static long global_seed, local_seed;
static long *get_global_random_seed_address(void) { return &global_seed; }
static long *get_global_local_random_seed_address(void) { return &local_seed; }
static real real_seed_random(long *seed) { return asked; }
"""

MAIN = r"""
int main(void)
{
	static real const numbers[] = { 0.0f, 0.1f, 0.2f, 0.25f, 0.2500001f, 0.3f, 0.35f, 0.4f, 0.45f, 0.5f,
		0.5000001f, 0.55f, 0.6f, 0.65f, 0.75f, 0.85f, 0.9f, 0.9000001f, 0.95f, 0.99f, 1.0f };
	int animation, first, number;

	for (animation = 0; animation<ANIMATIONS; animation++)
		printf("animation %d %.9g %d\n", animation, animations[animation].runtime_normalized_weight,
			animations[animation].next_animation_index);
	for (first = -1; first<=ANIMATIONS+1; first++)
		for (number = 0; number<(int)(sizeof(numbers)/sizeof(numbers[0])); number++)
		{
			asked = numbers[number];
			printf("choose %d %.9g %d\n", first, asked,
				animation_choose_random_permutation_internal(animation_update_kind_affects_game_state, 0, (short)first));
		}
	return 0;
}
"""


def source_file():
    """The fake world, the game's function and the cases, as one C file."""
    animations_c = read("source/models/model_animations.c")
    return "\n\n".join([FAKE_WORLD, function(animations_c, "animation_choose_random_permutation_internal"), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="animation-permutation-reference-") as work:
        source = Path(work) / "animation_permutation_reference.c"
        executable = Path(work) / "animation_permutation_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's choice of an animation's permutation (tools/engine_animation_permutation_reference.py).\n"
        "# animation <index> <normalized weight> <next or -1>\n"
        "# choose <first or -1> <random> <chosen or -1>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(
                f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_animation_permutation_reference.py"
            )
        print("the animation permutation reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
