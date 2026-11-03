"""The game's own periodic and transition functions and its seeded random
range, taken from the sources and run over their inputs, to record what the
native engine's must give.

    python3 tools/engine_periodic_reference.py          # writes the reference
    python3 tools/engine_periodic_reference.py --check  # fails if it is stale

The functions are source/math/periodic_functions.c's (the tables built from
the game's seed by `periodic_functions_initialize`, then read by
`periodic_function_evaluate` and `transition_function_evaluate`) and
source/math/random_math.c's `seed_random_range`, found by name as
tools/harness finds them, and compiled for this machine against a small fake
world. The table is written to tools/traces/periodic_reference.txt, which
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
from harness import enum_with, function, inline, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "periodic_reference.txt"

PERIODIC_FUNCTIONS = [
    "periodic_function_build_variable_period_x_table",
    "transition_function_build_table",
    "periodic_function_build_table",
    "periodic_functions_initialize",
    "periodic_function_evaluate",
    "transition_function_evaluate",
]

RANDOM_FUNCTIONS = ["seed_random", "seed_random_range", "real_seed_random"]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef float real;
typedef unsigned char byte;
typedef unsigned short word;

#define TRUE 1
#define FALSE 0
#define __fastcall
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define VALID_INDEX(index, count) (index>=0 && index<count)
#define match_assert(file, line, condition) ((void)0)
#define display_assert(message, file, line, fatal) ((void)0)
#define match_malloc(file, line, size) malloc(size)
#define system_exit(code) exit(code)
#define function_type_error(kind, type) ((void)0)
#define _pi ((real)3.14159265358979323846f)
#define RANDOM_A 1664525u
#define RANDOM_C 1013904223u

static unsigned long global_random_seed;

static unsigned long *get_global_random_seed_address(void)
{
	return &global_random_seed;
}
"""

MAIN = r"""
static real const times[] = { -3.7f, -0.01f, 1e7f, 12345.678f };

int main(void)
{
	int step;
	short function_type;
	unsigned long seed;
	static short const ranges[][2] = { { 0, 1 }, { 3, 9 }, { -5, 5 }, { 0, 32767 }, { -32768, 32767 }, { 7, 7 } };

	periodic_functions_initialize();

	/* every function (and one either side of the tables) over 50 periods,
	finer than the tables' entries, and a few times past them */
	for (step = 0; step<1004; step++)
	{
		real time = step<1000 ? step*0.0503f : times[step-1000];

		printf("periodic %.9g", time);
		for (function_type = -1; function_type<=NUMBER_OF_PERIODIC_FUNCTIONS; function_type++)
			printf(" %.9g", periodic_function_evaluate(function_type, time));
		printf("\n");
	}

	/* every transition (and one either side) over 0 to 1 and past each
	end, on the tables' entries and between them */
	for (step = -8; step<=2056; step++)
	{
		real value = step*(1.0f/2048.0f);

		printf("transition %.9g", value);
		for (function_type = -1; function_type<=NUMBER_OF_TRANSITION_FUNCTIONS; function_type++)
			printf(" %.9g", transition_function_evaluate(function_type, value));
		printf("\n");
	}

	/* the seeded range, over ranges of every width */
	seed = 0xdeadbeef;
	for (step = 0; step<300; step++)
	{
		short const *range = ranges[step%(sizeof(ranges)/sizeof(ranges[0]))];
		short value = seed_random_range(&seed, range[0], range[1]);

		printf("range %d %d %d %u\n", range[0], range[1], value, seed);
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    periodic_c = read("source/math/periodic_functions.c")
    periodic_h = read("source/math/periodic_functions.h")
    random_c = read("source/math/random_math.c")
    real_math_h = read("source/math/real_math.h")
    cseries_h = read("source/cseries/cseries.h")
    enums = [
        enum_with(periodic_c, "_periodic_function_one"),
        enum_with(periodic_c, "PERIODIC_FUNCTION_TABLE_SIZE"),
        enum_with(periodic_c, "SLIDE_PERIODIC_FUNCTION_FLAGS"),
        enum_with(periodic_h, "_transition_function_linear"),
    ]
    tables = periodic_c[periodic_c.index("static boolean function_tables_initialized") :]
    tables = tables[: tables.index("/* ---------- public code */")]
    helpers = [inline(cseries_h, "fast_ftol")]
    helpers += [function(random_c, name) for name in RANDOM_FUNCTIONS]
    helpers += [inline(real_math_h, name) for name in ("set_random_seed", "real_random")]
    definitions = [function(periodic_c, name) for name in PERIODIC_FUNCTIONS]
    return "\n\n".join([FAKE_WORLD, *enums, tables, *helpers, *definitions, MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="periodic-reference-") as work:
        source = Path(work) / "periodic_reference.c"
        executable = Path(work) / "periodic_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's periodic and transition functions and its seeded range (tools/engine_periodic_reference.py).\n"
        "# periodic <time> <function -1> <function 0> ... <function 12>\n"
        "# transition <value> <function -1> <function 0> ... <function 6>\n"
        "# range <lower> <upper> <value> <seed after>, from the seed 0xdeadbeef\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_periodic_reference.py")
        print("the periodic reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
