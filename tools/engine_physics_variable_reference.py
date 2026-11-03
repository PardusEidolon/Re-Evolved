"""The game's physics variables, taken from the sources and run over long
runs of speeds and positions seeking their targets, to record what the
native engine's must give.

    python3 tools/engine_physics_variable_reference.py          # writes the reference
    python3 tools/engine_physics_variable_reference.py --check  # fails if it is stale

The functions are all of source/physics/physics_variables.c's: the speed's
update and seek, the position's update, seek and seek direction, and the
two that move a position by its speed. They are found by name as
tools/harness finds them and compiled for this machine with no fused
multiply-add, over speed parameters, limits, wraps, targets and steps
drawn from a fixed seed (with zeros, equal limits and targets at and past
the limits among them), as the vehicles' speeds, slides and turns use them.
Each line is one call, with what it was given and what it left.

The trace is written to tools/traces/physics_variable_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "physics_variable_reference.txt"

FUNCTIONS = [
    "physics_variable_position_get_seek_direction",
    "physics_variable_speed_update",
    "physics_variable_speed_update_seek",
    "physics_variable_position_update",
    "physics_variable_position_update_seek",
    "physics_variable_update",
    "physics_variable_update_seek",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>

typedef int boolean;
typedef float real;

#define TRUE 1
#define FALSE 0
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

static real physics_variable_position_get_seek_direction(real *limits, real position, boolean wrap, real target);
"""

MAIN = r"""
/* (64 bits on every target, so the wasm32 build draws the same inputs) */
static unsigned long long seed = 0x13579bd;

static int next_random(int n)
{
	seed = seed * 1664525u + 1013904223u;
	return (int)((seed >> 8) % (unsigned long long)n);
}

/* a value from -range to range, a zero or a whole one now and then */
static real value(real range)
{
	switch (next_random(8))
	{
	case 0: return 0.f;
	case 1: return (real)(next_random(5) - 2);
	default: return range * ((real)next_random(20001) / 10000.f - 1.f);
	}
}

static real positive(real range)
{
	return next_random(10) ? range * (real)next_random(10001) / 10000.f : 0.f;
}

static void parameters_of(real *parameters)
{
	parameters[0] = positive(1.f);
	parameters[1] = positive(1.f);
	parameters[2] = positive(0.1f);
	parameters[3] = positive(0.1f);
}

static void limits_of(real *limits)
{
	real a = value(4.f);
	real b = next_random(10) ? value(4.f) : a;
	limits[0] = MAX(a, b);
	limits[1] = MIN(a, b);
}

#define P(x) printf(" %.9g", (double)(x))

int main(void)
{
	int run;
	int step;

	for (run = 0; run < 60; run++)
	{
		real variable[6];
		real speed = value(0.5f);
		real target = value(1.f);
		real position;
		real range;
		boolean wrap = next_random(2);
		real delta = positive(2.f);

		parameters_of(variable + 2);
		limits_of(variable);
		position = variable[1] + (variable[0] - variable[1]) * positive(1.f);
		range = value(0.2f);

		for (step = 0; step < 30; step++)
		{
			real before;
			real before_range;
			real step_delta = next_random(4) ? delta : value(2.f);
			real signed_delta;
			boolean result;

			if (next_random(10) == 0)
				target = value(1.f);

			before = speed;
			physics_variable_speed_update(&speed, (struct physics_variable_speed_parameters *)(variable + 2), step_delta);
			printf("speed"); P(variable[2]); P(variable[3]); P(variable[4]); P(variable[5]); P(before); P(step_delta); P(speed); printf("\n");

			before = speed;
			result = physics_variable_speed_update_seek(&speed, (struct physics_variable_speed_parameters *)(variable + 2), target, step_delta);
			printf("speed_seek"); P(variable[2]); P(variable[3]); P(variable[4]); P(variable[5]); P(before); P(target); P(step_delta); P(speed); printf(" %d\n", result);

			printf("direction"); P(variable[0]); P(variable[1]); printf(" %d", wrap); P(position); P(target);
			P(physics_variable_position_get_seek_direction(variable, position, wrap, target)); printf("\n");

			before = position;
			signed_delta = step_delta * (next_random(2) ? 1.f : -1.f) * 0.25f;
			physics_variable_position_update(&position, variable, wrap, signed_delta);
			printf("position"); P(variable[0]); P(variable[1]); printf(" %d", wrap); P(before); P(signed_delta); P(position); printf("\n");

			before = position;
			result = physics_variable_position_update_seek(&position, variable, wrap, target, step_delta * 0.1f);
			printf("position_seek"); P(variable[0]); P(variable[1]); printf(" %d", wrap); P(before); P(target); P(step_delta * 0.1f); P(position); printf(" %d\n", result);

			before = position;
			before_range = range;
			signed_delta = step_delta * (next_random(2) ? 1.f : -1.f);
			physics_variable_update(&position, &range, variable, wrap, signed_delta);
			printf("update"); P(variable[0]); P(variable[1]); P(variable[2]); P(variable[3]); P(variable[4]); P(variable[5]); printf(" %d", wrap);
			P(before); P(before_range); P(signed_delta); P(position); P(range); printf("\n");

			before = position;
			before_range = range;
			result = physics_variable_update_seek(&position, &range, variable, wrap, target, step_delta);
			printf("update_seek"); P(variable[0]); P(variable[1]); P(variable[2]); P(variable[3]); P(variable[4]); P(variable[5]); printf(" %d", wrap);
			P(before); P(before_range); P(target); P(step_delta); P(position); P(range); printf(" %d\n", result);
		}
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    header = read("source/physics/physics_variables.h")
    code = read("source/physics/physics_variables.c")
    functions = [function(code, name) for name in FUNCTIONS]
    return "\n\n".join([FAKE_WORLD, structure(header, "physics_variable_speed_parameters"), *functions, MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="physics-variable-reference-") as work:
        source = Path(work) / "physics_variable_reference.c"
        executable = Path(work) / "physics_variable_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-w", "-ffp-contract=off", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's physics variables (tools/engine_physics_variable_reference.py).\n"
        "# speed <positive> <negative> <acceleration> <deceleration> <speed> <delta> <speed after>\n"
        "# speed_seek <the parameters> <speed> <target> <delta> <speed after> <there>\n"
        "# direction <high> <low> <wrap> <position> <target> <direction>\n"
        "# position <high> <low> <wrap> <position> <delta> <position after>\n"
        "# position_seek <high> <low> <wrap> <position> <target> <delta> <position after> <there>\n"
        "# update <high> <low> <the parameters> <wrap> <position> <speed> <delta> <position after> <speed after>\n"
        "# update_seek <high> <low> <the parameters> <wrap> <position> <speed> <target> <delta> <position after> <speed after> <there>\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_physics_variable_reference.py")
        print("the physics variable reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
