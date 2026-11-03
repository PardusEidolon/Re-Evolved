"""The rest of the game's maths that the engine owes evidence for: a vector
from a yaw and a pitch, a rotation matrix from three angles, the move
towards a position that the devices and the first-person weapons make, a
real quantized to the byte below it, and a unit vector unpacked from 32
bits, run over angles, values and states to record what the engine's must
give.

    python3 tools/engine_math_extra_reference.py          # writes the reference
    python3 tools/engine_math_extra_reference.py --check  # fails if it is stale

The functions are source/math/real_math.c's `vector3d_from_euler_angles2d`,
`accelerate_to_position`, `dequantize_byte_to_real` and
`quantize_real_to_byte_lower_bound`, source/math/matrix_math.c's
`matrix4x3_rotation_from_angles` and source/rasterizer/rasterizer_geometry.c's
`uncompress_int32_to_real_vector3d`, with real_math.h's inline helpers,
compiled for this machine against a small fake world. The table is written
to tools/traces/math_extra_reference.txt, which
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
from harness import function, inline, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "math_extra_reference.txt"

INLINE_FUNCTIONS = ["set_real_point3d", "sine", "cosine", "square_root"]

REAL_MATH_FUNCTIONS = [
    "vector3d_from_euler_angles2d",
    "accelerate_to_position",
    "dequantize_byte_to_real",
    "quantize_real_to_byte_lower_bound",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef byte boolean;

#define TRUE 1
#define FALSE 0
#define UNSIGNED_CHAR_MAX 255
#define MIN(a,b) ((a)<(b)?(a):(b))
#define _real_epsilon 0.0001f
#define match_vassert(file, line, condition, message) ((void)0)

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct { real yaw, pitch; } real_euler_angles2d;
typedef struct
{
	real scale;
	union
	{
		real n[4][3];
		struct
		{
			real_vector3d forward;
			real_vector3d left;
			real_vector3d up;
			real_point3d position;
		};
	};
} real_matrix4x3;
"""

MAIN = r"""
static unsigned int seed = 0x5eed;

static unsigned int next(void)
{
	seed = seed*1664525u+1013904223u;
	return seed;
}

/* an angle from -2 pi to 2 pi, in thousandths */
static real angle(void)
{
	return (real)((int)(next()>>16)%12567-6283)*0.001f;
}

/* a real from -scale to scale, in steps of a thousandth of it */
static real between(real scale)
{
	return (real)((int)(next()>>16)%2001-1000)*0.001f*scale;
}

int main(void)
{
	int step;

	for (step = 0; step<300; step++)
	{
		real_euler_angles2d angles;
		real_vector3d v;

		angles.yaw = angle();
		angles.pitch = angle();
		vector3d_from_euler_angles2d(&v, &angles);
		printf("euler %.9g %.9g %.9g %.9g %.9g\n", angles.yaw, angles.pitch, v.i, v.j, v.k);
	}
	for (step = 0; step<300; step++)
	{
		real_matrix4x3 m;
		real yaw = angle(), pitch = angle(), roll = step%5==0 ? 0.0f : angle();
		int i, j;

		matrix4x3_rotation_from_angles(&m, yaw, pitch, roll);
		printf("rotation %.9g %.9g %.9g", yaw, pitch, roll);
		for (i = 0; i<3; i++)
			for (j = 0; j<3; j++)
				printf(" %.9g", m.n[i][j]);
		printf("\n");
	}
	/* runs of ticks towards a target, periodic or not, from rest or moving */
	for (step = 0; step<120; step++)
	{
		real minimum = step%3==0 ? -3.14159274f : 0.0f;
		real maximum = step%3==0 ? 3.14159274f : 1.0f+(real)(step%4);
		real position = minimum+(maximum-minimum)*(between(0.5f)+0.5f);
		real velocity = step%2 ? 0.0f : between(0.05f);
		real target = minimum+(maximum-minimum)*(between(0.6f)+0.5f);
		real maximum_velocity = 0.01f+between(0.04f)+0.04f;
		real acceleration = 0.001f+between(0.004f)+0.004f;
		boolean periodic = step%3==0;
		int tick;

		printf("accelerate %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g %d\n", step, position, velocity, target,
			maximum_velocity, acceleration, minimum, maximum, periodic);
		for (tick = 0; tick<40; tick++)
		{
			boolean arrived = accelerate_to_position(&position, &velocity, target, maximum_velocity, acceleration,
				minimum, maximum, periodic);

			printf("tick %.9g %.9g %d\n", position, velocity, arrived);
		}
	}
	/* each byte's edges, and values between them, over the unit range and others */
	for (step = 0; step<256; step++)
	{
		real value = (real)step/255.0f;
		real below = nextafterf(value, -1.0f);

		printf("quantize 0 1 %.9g %d\n", value, quantize_real_to_byte_lower_bound(0.0f, 1.0f, value));
		if (below>=0.0f)
			printf("quantize 0 1 %.9g %d\n", below, quantize_real_to_byte_lower_bound(0.0f, 1.0f, below));
	}
	for (step = 0; step<400; step++)
	{
		real minimum = step%2 ? 0.0f : -2.5f;
		real maximum = step%2 ? 1.0f : 7.0f;
		real value = minimum+(maximum-minimum)*(between(0.5f)+0.5f);

		printf("quantize %.9g %.9g %.9g %d\n", minimum, maximum, value,
			quantize_real_to_byte_lower_bound(minimum, maximum, value));
	}
	/* packed vectors: at random, and each field's ends */
	for (step = 0; step<300; step++)
	{
		unsigned int packed = next();
		real_vector3d v;

		if (step==0) packed = 0;
		if (step==1) packed = 0xffffffffu;
		if (step==2) packed = 0x3ffu|(0x3ffu<<11)|(0x1ffu<<22);
		if (step==3) packed = 0x400u|(0x400u<<11)|(0x200u<<22);
		v = uncompress_int32_to_real_vector3d(packed);
		printf("unpack %u %.9g %.9g %.9g\n", packed, v.i, v.j, v.k);
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    real_math_c = read("source/math/real_math.c")
    real_math_h = read("source/math/real_math.h")
    matrix_c = read("source/math/matrix_math.c")
    geometry_c = read("source/rasterizer/rasterizer_geometry.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            *(inline(real_math_h, name) for name in INLINE_FUNCTIONS),
            *(function(real_math_c, name) for name in REAL_MATH_FUNCTIONS),
            function(matrix_c, "matrix4x3_rotation_from_angles"),
            function(geometry_c, "uncompress_int32_to_real_vector3d"),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="math-extra-reference-") as work:
        source = Path(work) / "math_extra_reference.c"
        executable = Path(work) / "math_extra_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's remaining owed maths (tools/engine_math_extra_reference.py).\n"
        "# euler <yaw> <pitch> <vector>\n"
        "# rotation <yaw> <pitch> <roll> <forward> <left> <up>\n"
        "# accelerate <run> <position> <velocity> <target> <maximum velocity> <acceleration> <minimum> <maximum> <periodic>\n"
        "# tick <position> <velocity> <arrived>, after each of the run's ticks\n"
        "# quantize <minimum> <maximum> <value> <byte>\n"
        "# unpack <packed> <vector>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_math_extra_reference.py")
        print("the extra math reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
