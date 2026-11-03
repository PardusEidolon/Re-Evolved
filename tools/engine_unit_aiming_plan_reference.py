"""The game's planner of a unit's aiming and looking, one euler axis at a
time, taken from the sources and run over random errors and speeds, to
record what it plans and where each plan takes the axis, as the native
engine's must.

    python3 tools/engine_unit_aiming_plan_reference.py          # writes the reference
    python3 tools/engine_unit_aiming_plan_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_euler_axis_buildplan`,
`unit_euler_axis_doplan` and `unit_euler_axis_couple`, found by name as
tools/harness finds them, with units.c's `struct unit_acceleration_plan`,
compiled for this machine with the game's asserts left out, as its release
build leaves them.

Each case's error and speed are drawn from a few edges (zero, either side of
the plan's rest, small and large) and from random values; the speed limit is
none, negative or positive, and the acceleration limit positive. Each plan is
followed for a tick and for random times, and pairs of plans are coupled.

Floats are printed as their bits, so that the engine's must be the same
float, not only print the same.

The trace is written to tools/traces/unit_aiming_plan_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "unit_aiming_plan_reference.txt"

FAKE_WORLD = r"""
#include <float.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
typedef float real;
#define TRUE 1
#define FALSE 0
#define REAL_MAX FLT_MAX
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
/* real_math.h's, which answers a float */
static real square_root(real x) { return sqrt(x); }
#define match_vassert(file, line, condition, text) ((void)0)
"""

DRIVER = r"""
static unsigned long seed = 0xa1373;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static unsigned bits(real value)
{
	unsigned result;
	memcpy(&result, &value, sizeof result);
	return result;
}

/* an error or a speed: an edge, or a random value of a random scale */
static real random_value(void)
{
	static real const edges[] = { 0.f, 0.0005f, -0.0005f, 0.001f, -0.001f, 0.002f, -0.002f, 1.f, -1.f, 3.1415927f, -3.1415927f };
	static real const scales[] = { 0.001f, 0.01f, 0.1f, 1.f, 10.f };
	if (next_random(4) == 0)
		return edges[next_random(sizeof edges / sizeof edges[0])];
	return (real)(next_random(20001) - 10000) / 10000.f * scales[next_random(5)];
}

/* the speed limit: none, negative or positive */
static real random_speed_limit(void)
{
	static real const limits[] = { 0.f, -0.5f, 0.05f, 0.2f, 1.f, 5.f };
	if (next_random(3) == 0)
		return (real)next_random(10001) / 1000.f;
	return limits[next_random(sizeof limits / sizeof limits[0])];
}

/* the acceleration limit, positive */
static real random_acceleration_limit(void)
{
	static real const limits[] = { 0.01f, 0.1f, 0.5f, 1.f, 4.f };
	if (next_random(3) == 0)
		return (real)(next_random(10000) + 1) / 1000.f;
	return limits[next_random(sizeof limits / sizeof limits[0])];
}

static void print_plan(struct unit_acceleration_plan const *plan)
{
	printf(" %d %u %u %u %u %u %u %u", plan->at_rest, bits(plan->initial_position), bits(plan->initial_velocity),
		bits(plan->acceleration), bits(plan->acceleration_time), bits(plan->coast_time), bits(plan->deceleration),
		bits(plan->deceleration_time));
}

static void print_build(real position, real velocity, real speed, real acceleration)
{
	printf("%u %u %u %u", bits(position), bits(velocity), bits(speed), bits(acceleration));
}

int main(void)
{
	long run;
	for (run = 0; run < 12000; run++)
	{
		struct unit_acceleration_plan plan;
		real position = random_value(), velocity = random_value();
		real speed = random_speed_limit(), acceleration = random_acceleration_limit();
		real delta, new_position, new_velocity;
		boolean complete;
		long step;
		memset(&plan, 0, sizeof plan);
		printf("build ");
		print_build(position, velocity, speed, acceleration);
		unit_euler_axis_buildplan(position, velocity, speed, acceleration, &plan);
		printf(" ->");
		print_plan(&plan);
		printf("\n");
		for (step = 0; step < 3; step++)
		{
			static real const deltas[] = { 1.f, 0.f, -1.f, 0.25f };
			delta = step == 0 ? 1.f : next_random(3) ? (real)next_random(4001) / 1000.f : deltas[next_random(4)];
			printf("do");
			print_plan(&plan);
			printf(" %u %u %u ->", bits(delta), bits(position), bits(velocity));
			complete = unit_euler_axis_doplan(&plan, delta, position, &new_position, velocity, &new_velocity);
			printf(" %d %u %u\n", complete, bits(new_position), bits(new_velocity));
		}
	}
	for (run = 0; run < 8000; run++)
	{
		struct unit_acceleration_plan first, second;
		real speed = random_speed_limit(), acceleration = random_acceleration_limit();
		real yaw = random_value(), yaw_velocity = random_value();
		real pitch = random_value(), pitch_velocity = random_value();
		memset(&first, 0, sizeof first);
		memset(&second, 0, sizeof second);
		unit_euler_axis_buildplan(yaw, yaw_velocity, speed, acceleration, &first);
		unit_euler_axis_buildplan(pitch, pitch_velocity, speed, acceleration, &second);
		printf("couple %u %u", bits(speed), bits(acceleration));
		print_plan(&first);
		print_plan(&second);
		unit_euler_axis_couple(&first, &second, speed, acceleration);
		printf(" ->");
		print_plan(&first);
		print_plan(&second);
		printf("\n");
	}
	return 0;
}
"""

# each function, and the file it is defined in
FUNCTIONS = [
    ("source/units/units.c", "unit_euler_axis_buildplan"),
    ("source/units/units.c", "unit_euler_axis_doplan"),
    ("source/units/units.c", "unit_euler_axis_couple"),
]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    units_c = read("source/units/units.c")
    plan = structure(units_c, "unit_acceleration_plan")
    functions = [function(units_c, name) for _, name in FUNCTIONS]
    return "\n".join([FAKE_WORLD, plan, *functions, DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-aiming-plan-reference-") as work:
        source = Path(work) / "unit_aiming_plan_reference.c"
        executable = Path(work) / "unit_aiming_plan_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    plan = "<at rest> <position> <velocity> <acceleration> <its time> <coast time> <deceleration> <its time>"
    return (
        "# The game's euler axis planner (tools/engine_unit_aiming_plan_reference.py), floats as bits.\n"
        f"# build <position> <velocity> <speed limit> <acceleration limit> -> {plan}\n"
        f"# do {plan} <delta> <position> <velocity> -> <complete> <position> <velocity>\n"
        "# couple <speed limit> <acceleration limit> <first plan> <second plan> -> <first plan> <second plan>\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_aiming_plan_reference.py")
        print("the unit aiming plan reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
