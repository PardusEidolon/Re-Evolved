"""The game's small physics sums, taken from the sources and run over
random values, to record what they give, as the native engine's must.

    python3 tools/engine_physics_math_reference.py          # writes the reference
    python3 tools/engine_physics_math_reference.py --check  # fails if it is stale

The functions are source/physics/physics.c's `pin_fraction` and
`friction_evaluate` (with source/math/real_math.c's
`component_vectors_from_normal3d` and real_math.h's `cross_product3d`) and
source/physics/point_physics.c's `point_physics_definition_get_mass`,
found by name as tools/harness finds them and compiled for this machine.
The other vector helpers are written here as real_math.h writes them.
The values are drawn at random, with values on the ends of a pin's range,
reversed and empty ranges, and unit and stretched axes among them.

The trace is written to tools/traces/physics_math_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "physics_math_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int
#define __inline static inline

typedef float real;

union real_vector3d
{
	real n[3];
	struct { real i, j, k; };
};
typedef union real_vector3d real_vector3d;

struct friction_datum
{
	real_vector3d friction;
	real_vector3d parallel;
	real_vector3d perpendicular;
};

struct point_physics_definition
{
	real runtime_mass_over_radius_cubed;
};

enum
{
	_friction_type_point = 0,
	_friction_type_forward,
	_friction_type_left,
	_friction_type_up,
};

#define match_vassert(file, line, condition, message) printf("assert %s:%d\n", file, line)

/* real_math.h's helpers */
static inline real dot_product3d(real_vector3d const *a, real_vector3d const *b)
{
	return a->i*b->i + a->j*b->j + a->k*b->k;
}

static inline real_vector3d *set_real_vector3d(real_vector3d *v, real i, real j, real k)
{
	v->i = i; v->j = j; v->k = k;
	return v;
}

static inline real_vector3d *scale_vector3d(real_vector3d const *v, real s, real_vector3d *result)
{
	result->i = s*v->i; result->j = s*v->j; result->k = s*v->k;
	return result;
}

static inline real_vector3d *add_vectors3d(real_vector3d const *a, real_vector3d const *b, real_vector3d *result)
{
	result->i = a->i+b->i; result->j = a->j+b->j; result->k = a->k+b->k;
	return result;
}

static inline real_vector3d *subtract_vectors3d(real_vector3d const *a, real_vector3d const *b, real_vector3d *result)
{
	result->i = a->i-b->i; result->j = a->j-b->j; result->k = a->k-b->k;
	return result;
}
"""

DRIVER = r"""
static unsigned long seed = 0x9415;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

/* a random real in -range..range, in thousandths of it */
static real next_signed(real range)
{
	return range * (real)(next_random(2001) - 1000) / 1000.f;
}

static void next_vector(real_vector3d *v, real range)
{
	set_real_vector3d(v, next_signed(range), next_signed(range), next_signed(range));
}

/* an axis: one of the frame's, or stretched */
static void next_axis(real_vector3d *v)
{
	switch (next_random(4))
	{
	case 0: set_real_vector3d(v, 1.f, 0.f, 0.f); break;
	case 1: set_real_vector3d(v, 0.f, 0.f, 1.f); break;
	case 2: set_real_vector3d(v, 0.6f, 0.f, 0.8f); break;
	default: next_vector(v, 1.5f); break;
	}
}

static unsigned bits(real x)
{
	unsigned u;
	memcpy(&u, &x, sizeof u);
	return u;
}

static void print_vector(real_vector3d const *v)
{
	printf(" %08x %08x %08x", bits(v->i), bits(v->j), bits(v->k));
}

int main(void)
{
	long run;

	for (run = 0; run < 3000; run++)
	{
		real begin = next_signed(1.f);
		real end = next_random(8) == 0 ? begin : next_signed(1.f);
		real value;

		switch (next_random(4))
		{
		case 0: value = begin; break;
		case 1: value = end; break;
		default: value = next_signed(1.5f); break;
		}
		printf("pin %08x %08x %08x %08x\n", bits(value), bits(begin), bits(end), bits(pin_fraction(value, begin, end)));
	}
	for (run = 0; run < 3000; run++)
	{
		struct friction_datum friction;
		real_vector3d forward, up;
		short type = (short)next_random(4);
		real parallel_scale = next_signed(2.f);
		real perpendicular_scale = next_signed(2.f);

		memset(&friction, 0, sizeof friction);
		next_vector(&friction.friction, 50.f);
		next_axis(&forward);
		next_axis(&up);
		printf("friction %d %08x %08x", type, bits(parallel_scale), bits(perpendicular_scale));
		print_vector(&friction.friction);
		print_vector(&forward);
		print_vector(&up);
		friction_evaluate(type, parallel_scale, perpendicular_scale, &friction, &forward, &up);
		print_vector(&friction.friction);
		printf("\n");
	}
	for (run = 0; run < 2000; run++)
	{
		struct point_physics_definition definition;
		real radius = next_random(10) == 0 ? 0.f : next_signed(2.f) + 2.f;

		definition.runtime_mass_over_radius_cubed = next_signed(300000.f);
		printf("mass %08x %08x %08x\n", bits(definition.runtime_mass_over_radius_cubed), bits(radius),
			bits(point_physics_definition_get_mass(&definition, radius)));
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    physics = read("source/physics/physics.c")
    math = read("source/math/real_math.c")
    return "\n".join(
        [
            FAKE_WORLD,
            function(read("source/math/real_math.h"), "cross_product3d"),
            function(math, "component_vectors_from_normal3d"),
            function(physics, "pin_fraction"),
            function(physics, "friction_evaluate"),
            function(read("source/physics/point_physics.c"), "point_physics_definition_get_mass"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="physics-math-reference-") as work:
        source = Path(work) / "physics_math_reference.c"
        executable = Path(work) / "physics_math_reference"
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
        "# The game's small physics sums (tools/engine_physics_math_reference.py), f32 bits.\n"
        "# pin <value> <begin> <end> <fraction>\n"
        "# friction <type> <parallel scale> <perpendicular scale> <friction> <forward> <up> <friction after>\n"
        "# mass <mass over radius cubed> <radius> <mass>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_physics_math_reference.py")
        print("the physics math reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
