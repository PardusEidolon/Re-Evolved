"""The game's landings: which landing a biped recovers from, and for how
long, after landing at random speeds with random biped tags, recorded for
the engine's to match.

    python3 tools/engine_biped_landing_reference.py          # writes the reference
    python3 tools/engine_biped_landing_reference.py --check  # fails if it is stale

The function is source/units/bipeds.c's `biped_start_landing`, compiled for
this machine against a small fake world. The table is written to
tools/traces/biped_landing_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "biped_landing_reference.txt"

WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;

#define TICKS_PER_SECOND 30
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))

struct biped_definition
{
	struct
	{
		real maximum_soft_landing_time;
		real maximum_hard_landing_time;
		real minimum_soft_landing_velocity;
		real minimum_hard_landing_velocity;
		real maximum_hard_landing_velocity;
	} biped;
};

struct biped_datum
{
	long definition_index;
	struct
	{
		short landing;
		char landing_recovery_counter;
		char landing_recovery_time;
	} biped;
};

static struct biped_datum the_biped;
static struct biped_definition the_definition;

static struct biped_datum *biped_get(long index) { return &the_biped; }
static struct biped_definition *biped_definition_get(long index) { return &the_definition; }
"""

MAIN = r"""
static unsigned int seed = 0x1a2d;

static unsigned int next(void)
{
	seed = seed*1664525u+1013904223u;
	return seed;
}

/* a real from 0 to scale, in steps of a thousandth of it */
static real upto(real scale)
{
	return (real)((int)(next()>>16)%1001)*0.001f*scale;
}

int main(void)
{
	int step;

	for (step = 0; step<4000; step++)
	{
		real velocity;

		the_definition.biped.maximum_soft_landing_time = upto(1.5f);
		the_definition.biped.maximum_hard_landing_time = upto(2.0f);
		the_definition.biped.minimum_soft_landing_velocity = upto(6.0f);
		the_definition.biped.minimum_hard_landing_velocity = the_definition.biped.minimum_soft_landing_velocity+upto(8.0f);
		the_definition.biped.maximum_hard_landing_velocity = the_definition.biped.minimum_hard_landing_velocity+upto(20.0f);
		/* (now and then an empty range) */
		if (step%17==0) the_definition.biped.minimum_hard_landing_velocity = the_definition.biped.minimum_soft_landing_velocity;
		if (step%19==0) the_definition.biped.maximum_hard_landing_velocity = the_definition.biped.minimum_hard_landing_velocity;
		/* the landing's velocity, in units a second as the engine's, and a tick as the game's */
		velocity = upto(36.0f);
		memset(&the_biped, 0, sizeof(the_biped));
		the_biped.biped.landing = -1;
		printf("landing %.9g %.9g %.9g %.9g %.9g %.9g ->", the_definition.biped.maximum_soft_landing_time,
			the_definition.biped.maximum_hard_landing_time, the_definition.biped.minimum_soft_landing_velocity,
			the_definition.biped.minimum_hard_landing_velocity, the_definition.biped.maximum_hard_landing_velocity,
			velocity);
		biped_start_landing(0, velocity*(1.f/TICKS_PER_SECOND));
		if (the_biped.biped.landing<0) printf(" none\n");
		else printf(" %d %d\n", the_biped.biped.landing, (int)the_biped.biped.landing_recovery_time);
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's function and the cases, as one C file."""
    bipeds_c = read("source/units/bipeds.c")
    return "\n\n".join([WORLD, function(bipeds_c, "biped_start_landing"), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="biped-landing-reference-") as work:
        source = Path(work) / "biped_landing_reference.c"
        executable = Path(work) / "biped_landing_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's landings (tools/engine_biped_landing_reference.py).\n"
        "# landing <maximum soft landing time> <maximum hard landing time> <minimum soft landing velocity>\n"
        "#   <minimum hard landing velocity> <maximum hard landing velocity> <landing velocity, a second>\n"
        "#   -> <landing: 0 soft, 1 hard> <recovery ticks, of the game's>, or none\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_biped_landing_reference.py")
        print("the biped landing reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
