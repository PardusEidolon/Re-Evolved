"""The game's own vehicle function values, taken from the sources and run
over a spread of vehicles and their states, to record what the native
engine's must give.

    python3 tools/engine_vehicle_function_reference.py          # writes the reference
    python3 tools/engine_vehicle_function_reference.py --check  # fails if it is stale

The function is source/units/vehicles.c's `vehicle_export_function_values`
(with source/math/real_math.c's `component_vectors_from_normal3d`), found by
name as tools/harness finds them, and compiled for this machine against a
small fake world that holds one vehicle and its definition. The table is
written to tools/traces/vehicle_function_reference.txt, which
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
from harness import enum_with, function, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "vehicle_function_reference.txt"

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef float real;

#define TRUE 1
#define FALSE 0
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define MAX(a,b) ((a)>(b)?(a):(b))
#define CEILING(n,ceiling) ((n)>(ceiling)?(ceiling):(n))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : CEILING((n),(ceiling)))
#define NUMBER_OF_INCOMING_OBJECT_FUNCTIONS 4

typedef struct { real i, j, k; } real_vector3d;

static real dot_product3d(real_vector3d const *a, real_vector3d const *b)
{
	return a->i*b->i + a->j*b->j + a->k*b->k;
}

static real magnitude3d(real_vector3d const *v)
{
	return (real)sqrt(dot_product3d(v, v));
}

static real_vector3d *scale_vector3d(real_vector3d const *v, real s, real_vector3d *result)
{
	result->i = v->i*s; result->j = v->j*s; result->k = v->k*s;
	return result;
}

static real_vector3d *subtract_vectors3d(real_vector3d const *a, real_vector3d const *b, real_vector3d *result)
{
	result->i = a->i-b->i; result->j = a->j-b->j; result->k = a->k-b->k;
	return result;
}

struct vehicle_datum
{
	struct
	{
		unsigned long flags;
		real_vector3d translational_velocity, forward, up;
		real incoming_function_values[NUMBER_OF_INCOMING_OBJECT_FUNCTIONS];
	} object;
	struct
	{
		unsigned long flags;
		real speed, slide, turn, left_tread, right_tread, wheel, thrust, hover;
		short airborne_ticks;
	} vehicle;
	long definition_index;
};

struct vehicle_definition
{
	real unknown2f8, unknown2fc, unknown308, unknown30c, unknown330, unknown334;
	real wheel_circumference;
	short function_modes[NUMBER_OF_INCOMING_OBJECT_FUNCTIONS];
};

static struct vehicle_datum the_vehicle;
static struct vehicle_definition the_definition;

static struct vehicle_datum *vehicle_datum_get(long index)
{
	(void)index;
	return &the_vehicle;
}

static struct vehicle_definition *vehicle_specific_definition_get(long index)
{
	(void)index;
	return &the_definition;
}
"""

MAIN = r"""
/* forward, reverse, left and right turn, left and right slide, circumference */
static real const definitions[][7] = {
	{ 0.9f, 0.4f, 0.6f, -0.6f, 0.3f, -0.2f, 1.25f },
	{ 0.3f, 0.5f, 0.25f, -0.4f, 0.0f, 0.0f, 2.0f },
	{ -0.7f, -0.7f, 0.8f, 0.8f, 0.5f, 0.5f, 0.6f },
};

/* speed, slide, turn, wheel, left tread, right tread, thrust, hover,
airborne ticks, vehicle flags, object flags, velocity, forward, up */
static real const states[][20] = {
	{ 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0.0f, 0, 0, 0, 0.0f, 0.0f, 0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f },
	{ 0.45f, 0.1f, -0.2f, 0.7f, 0.3f, 1.1f, 0.5f, 0.25f, 0, 4, 2, 0.4f, 0.05f, -0.02f, 1.0f, 0.0f, 0.0f, 0.0f, 0.0f, 1.0f },
	{ -0.3f, -0.15f, 0.35f, 1.2f, 0.05f, 0.9f, 1.0f, 0.9f, 3, 8, 4, -0.25f, 0.2f, 0.1f, 0.6f, 0.8f, 0.0f, 0.0f, 0.0f, 1.0f },
	{ 1.4f, 0.5f, 1.0f, 0.2f, 1.9f, 0.6f, 0.8f, 0.0f, 12, 12, 26, 1.2f, -0.6f, 0.3f, 0.8f, 0.0f, 0.6f, -0.6f, 0.0f, 0.8f },
	{ -1.1f, 0.0f, -0.9f, 0.55f, 0.4f, 0.4f, 0.1f, 1.0f, 1, 0, 16, 0.02f, 0.01f, -0.9f, 0.0f, 1.0f, 0.0f, 0.0f, 0.0f, 1.0f },
};

static void set(real_vector3d *v, real const *from)
{
	v->i = from[0]; v->j = from[1]; v->k = from[2];
}

int main(void)
{
	int d, s, mode, value;

	for (d = 0; d<(int)(sizeof(definitions)/sizeof(definitions[0])); d++)
	{
		real const *definition = definitions[d];

		the_definition.unknown2f8 = definition[0];
		the_definition.unknown2fc = definition[1];
		the_definition.unknown308 = definition[2];
		the_definition.unknown30c = definition[3];
		the_definition.unknown330 = definition[4];
		the_definition.unknown334 = definition[5];
		the_definition.wheel_circumference = definition[6];
		for (s = 0; s<(int)(sizeof(states)/sizeof(states[0])); s++)
		{
			real const *state = states[s];

			the_vehicle.vehicle.speed = state[0];
			the_vehicle.vehicle.slide = state[1];
			the_vehicle.vehicle.turn = state[2];
			the_vehicle.vehicle.wheel = state[3];
			the_vehicle.vehicle.left_tread = state[4];
			the_vehicle.vehicle.right_tread = state[5];
			the_vehicle.vehicle.thrust = state[6];
			the_vehicle.vehicle.hover = state[7];
			the_vehicle.vehicle.airborne_ticks = (short)state[8];
			the_vehicle.vehicle.flags = (unsigned long)state[9];
			the_vehicle.object.flags = (unsigned long)state[10];
			set(&the_vehicle.object.translational_velocity, state+11);
			set(&the_vehicle.object.forward, state+14);
			set(&the_vehicle.object.up, state+17);
			/* each mode once, four to a pass, against values it must
			leave as they are where it exports nothing */
			for (mode = 0; mode<40; mode += NUMBER_OF_INCOMING_OBJECT_FUNCTIONS)
			{
				for (value = 0; value<NUMBER_OF_INCOMING_OBJECT_FUNCTIONS; value++)
				{
					the_definition.function_modes[value] = (short)(mode+value);
					the_vehicle.object.incoming_function_values[value] = -1.0f;
				}
				vehicle_export_function_values(0);
				for (value = 0; value<NUMBER_OF_INCOMING_OBJECT_FUNCTIONS; value++)
				{
					real result = the_vehicle.object.incoming_function_values[value];

					printf("export");
					for (int at = 0; at<7; at++)
						printf(" %.9g", definition[at]);
					for (int at = 0; at<20; at++)
						printf(" %.9g", state[at]);
					printf(" %d %.9g\n", mode+value, result);
				}
			}
		}
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's function and the cases, as one C file."""
    vehicles_c = read("source/units/vehicles.c")
    real_math_c = read("source/math/real_math.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            enum_with(vehicles_c, "_vehicle_function_none"),
            function(real_math_c, "component_vectors_from_normal3d"),
            function(vehicles_c, "vehicle_export_function_values"),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="vehicle-function-reference-") as work:
        source = Path(work) / "vehicle_function_reference.c"
        executable = Path(work) / "vehicle_function_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's vehicle function values (tools/engine_vehicle_function_reference.py).\n"
        "# export <forward> <reverse> <left turn> <right turn> <left slide> <right slide> <circumference>\n"
        "#   <speed> <slide> <turn> <wheel> <left tread> <right tread> <thrust> <hover> <airborne ticks>\n"
        "#   <vehicle flags> <object flags> <velocity xyz> <forward xyz> <up xyz> <mode> <value, -1 for none>\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_vehicle_function_reference.py")
        print("the vehicle function reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
