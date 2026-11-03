"""The game's slowing of a projectile, taken from the sources and run over
random definitions, to record the deceleration, damage distance and
deceleration timer it sets, as the native engine's must.

    python3 tools/engine_projectile_deceleration_reference.py          # writes the reference
    python3 tools/engine_projectile_deceleration_reference.py --check  # fails if it is stale

The functions are source/items/projectiles.c's
`projectile_calculate_deceleration` and
`projectile_calculate_deceleration_from_distances`, found by name as
tools/harness finds them and compiled for this machine against a fake
world: one projectile of one definition, in the air or under water, its
timer left at a random value to show where the game leaves it alone. The
definitions draw their speeds and damage ranges at random, with equal
speeds, empty ranges and ranges that start at the muzzle among them.

The trace is written to tools/traces/projectile_deceleration_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "projectile_deceleration_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef float real;

#define TRUE 1
#define FALSE 0
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags)&(unsigned)FLAG(bit))!=0)

enum { _object_wholly_under_media_bit = 3 };

struct projectile_definition
{
	struct
	{
		real air_damage_range_lower_bound;
		real air_damage_range_upper_bound;
		real water_damage_range_lower_bound;
		real water_damage_range_upper_bound;
		real initial_velocity;
		real final_velocity;
	} projectile;
};

struct projectile_datum
{
	long definition_index;
	struct { unsigned long flags; } object;
	struct
	{
		real deceleration;
		real maximum_damage_distance;
		real deceleration_timer;
		real deceleration_timer_delta;
	} projectile;
};

static struct projectile_datum the_projectile;
static struct projectile_definition the_definition;

static struct projectile_datum *projectile_get(long index) { return &the_projectile; }
static struct projectile_definition *projectile_definition_get(long index) { return &the_definition; }
"""

DRIVER = r"""
static unsigned long seed = 0xdece1;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

/* a random real in 0..range, in thousandths of it */
static real next_real(real range)
{
	return range * (real)next_random(1001) / 1000.f;
}

/* a damage range: empty, from the muzzle or further out */
static void next_range(real *lower, real *upper)
{
	long kind = next_random(4);

	*lower = kind == 0 ? 0.f : kind == 1 ? 0.f : next_real(4.f);
	*upper = kind == 0 ? *lower : *lower + next_real(60.f);
}

static unsigned bits(real x)
{
	unsigned u;
	memcpy(&u, &x, sizeof u);
	return u;
}

int main(void)
{
	long run;

	for (run = 0; run < 4000; run++)
	{
		struct projectile_definition *definition = &the_definition;
		boolean underwater = next_random(3) == 0;

		/* speeds a game tick, as the tags hold them */
		definition->projectile.initial_velocity = 0.5f + next_real(20.f);
		definition->projectile.final_velocity = next_random(5) == 0 ?
			definition->projectile.initial_velocity :
			next_real(definition->projectile.initial_velocity);
		next_range(&definition->projectile.air_damage_range_lower_bound,
			&definition->projectile.air_damage_range_upper_bound);
		next_range(&definition->projectile.water_damage_range_lower_bound,
			&definition->projectile.water_damage_range_upper_bound);
		memset(&the_projectile, 0, sizeof the_projectile);
		the_projectile.object.flags = underwater ? FLAG(_object_wholly_under_media_bit) : 0;
		the_projectile.projectile.deceleration_timer = next_real(2.f);
		printf("case %08x %08x %08x %08x %08x %08x %d %08x",
			bits(definition->projectile.initial_velocity), bits(definition->projectile.final_velocity),
			bits(definition->projectile.air_damage_range_lower_bound), bits(definition->projectile.air_damage_range_upper_bound),
			bits(definition->projectile.water_damage_range_lower_bound), bits(definition->projectile.water_damage_range_upper_bound),
			underwater, bits(the_projectile.projectile.deceleration_timer));
		projectile_calculate_deceleration(0);
		printf(" %08x %08x %08x %08x\n",
			bits(the_projectile.projectile.deceleration), bits(the_projectile.projectile.maximum_damage_distance),
			bits(the_projectile.projectile.deceleration_timer), bits(the_projectile.projectile.deceleration_timer_delta));
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    projectiles = read("source/items/projectiles.c")
    return "\n".join(
        [
            FAKE_WORLD,
            function(projectiles, "projectile_calculate_deceleration_from_distances"),
            function(projectiles, "projectile_calculate_deceleration"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="projectile-deceleration-reference-") as work:
        source = Path(work) / "projectile_deceleration_reference.c"
        executable = Path(work) / "projectile_deceleration_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    if "assert" in output:
        raise SystemExit("the game's code asserted")
    return (
        "# The game's slowing of a projectile (tools/engine_projectile_deceleration_reference.py).\n"
        "# case <initial velocity> <final velocity> <air damage range> <water damage range> <underwater>"
        " <the timer before> <deceleration> <maximum damage distance> <timer> <timer delta> (f32 bits)\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(
                f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_projectile_deceleration_reference.py"
            )
        print("the projectile deceleration reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
