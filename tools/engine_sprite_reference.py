"""The game's own sprite basis and fade, taken from the sources and run over
cameras, sprites and their directions, to record how the native engine's
sprites must face.

    python3 tools/engine_sprite_reference.py          # writes the reference
    python3 tools/engine_sprite_reference.py --check  # fails if it is stale

The functions are source/render/render_sprite.c's
`build_sprite_compute_basis` (the axes a sprite's corners are laid along,
facing the screen, parallel to its direction or perpendicular to it) and
`build_sprite_compute_vertex_fade` (how a sprite that does not face the
screen fades as it turns edge on), with source/math/real_math.c's
`rotate_vector_about_axis`, found by name as tools/harness finds them, and
compiled for this machine against a small fake world. The game works in the
camera's space; the fake world takes each case there from the world, as
render_cameras.c's frustum does, and brings the axes back, so the reference
is in the world's. The table is written to
tools/traces/sprite_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "sprite_reference.txt"

SPRITE_FUNCTIONS = ["build_sprite_compute_basis", "build_sprite_compute_vertex_fade"]

INLINE_FUNCTIONS = [
    "dot_product3d",
    "square_root",
    "scale_vector3d",
    "magnitude_squared3d",
    "magnitude3d",
    "normalize3d",
    "cross_product3d",
]

FAKE_WORLD = r"""
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef int boolean;

#define TRUE 1
#define FALSE 0
#define _real_epsilon 0.0001f
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define match_assert(file, line, condition) ((void)0)
#define match_vassert(file, line, condition, message) ((void)0)

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct
{
	real scale;
	real_vector3d forward;
	real_vector3d left;
	real_vector3d up;
	real_point3d position;
} real_matrix4x3;

/* the one field of the sprites being built the basis reads */
struct build_sprite_data
{
	unsigned long flags;
};

/* render_sprite.c's globals, the two the basis reads */
static struct
{
	real_vector3d viewer_space_world_up;
	real_vector3d viewer_space_world_forward;
} build_sprite_globals;
"""

MAIN = r"""
static unsigned int seed = 0x5917e;

/* a component from -2 to 2, in steps the engine reads back exactly */
static real component(void)
{
	seed = seed*1664525u+1013904223u;
	return (real)((int)(seed>>16)%4001-2000)*0.001f;
}

static real_vector3d vector(void)
{
	real_vector3d v;

	v.i = component();
	v.j = component();
	v.k = component();
	return v;
}

/* the camera's axes, as render_cameras.c's frustum makes them */
static void view_axes(real_vector3d const *forward, real_vector3d const *up,
	real_vector3d *x_axis, real_vector3d *y_axis, real_vector3d *z_axis)
{
	cross_product3d(forward, up, x_axis);
	cross_product3d(x_axis, forward, y_axis);
	z_axis->i = -forward->i;
	z_axis->j = -forward->j;
	z_axis->k = -forward->k;
	normalize3d(x_axis);
	normalize3d(y_axis);
	normalize3d(z_axis);
}

/* a world vector in the camera's space, and back */
static real_vector3d to_view(real_vector3d const *axes, real_vector3d v)
{
	real_vector3d out;

	out.i = dot_product3d(&axes[0], &v);
	out.j = dot_product3d(&axes[1], &v);
	out.k = dot_product3d(&axes[2], &v);
	return out;
}

static real_vector3d to_world(real_vector3d const *axes, real_vector3d v)
{
	real_vector3d out;

	out.i = axes[0].i*v.i + axes[1].i*v.j + axes[2].i*v.k;
	out.j = axes[0].j*v.i + axes[1].j*v.j + axes[2].j*v.k;
	out.k = axes[0].k*v.i + axes[1].k*v.j + axes[2].k*v.k;
	return out;
}

static void print(char const *name, real_vector3d v)
{
	printf(" %s %.9g %.9g %.9g", name, v.i, v.j, v.k);
}

int main(void)
{
	static real_vector3d const world_up = { 0.f, 0.f, 1.f };
	static real_vector3d const world_left = { 0.f, 1.f, 0.f };
	/* directions the cases also take: straight up and down, nearly so
	(either side of the perpendicular's turn to its other reference),
	none and too short to normalize */
	static real_vector3d const special[] = {
		{ 0.f, 0.f, 1.f }, { 0.f, 0.f, -2.f }, { 0.1f, 0.f, 1.f }, { 0.f, 0.11f, -1.f },
		{ 0.f, 0.f, 0.f }, { 0.00004f, 0.f, 0.00002f }, { 1.f, 0.f, 0.f }, { 0.f, -1.f, 0.f },
	};
	struct build_sprite_data data = { 0 };
	int step;

	for (step = 0; step<600; step++)
	{
		real_vector3d eye = vector();
		real_vector3d forward = vector();
		real_vector3d up = (step%3) ? vector() : world_up;
		real_vector3d position = vector();
		real_vector3d direction = step<(int)(sizeof(special)/sizeof(special[0]))*3
			? special[step/3]
			: vector();
		real_vector3d axes[3];
		real_vector3d offset, view_direction;
		real_matrix4x3 basis;
		short mode = (short)(step%3);
		short fade_mode;

		if (magnitude_squared3d(&forward)<0.01f)
			forward.i += 1.f;
		view_axes(&forward, &up, &axes[0], &axes[1], &axes[2]);
		if (magnitude_squared3d(&axes[0])<0.5f || magnitude_squared3d(&axes[1])<0.5f)
			continue; /* (a camera looking along its up has no frame) */

		build_sprite_globals.viewer_space_world_up = to_view(axes, world_up);
		build_sprite_globals.viewer_space_world_forward = to_view(axes, world_left);
		offset.i = position.i - eye.i;
		offset.j = position.j - eye.j;
		offset.k = position.k - eye.k;
		offset = to_view(axes, offset);
		view_direction = to_view(axes, direction);
		build_sprite_compute_basis(&data, mode, 0, (real_point3d const *)&offset, &view_direction, &basis);

		printf("sprite %d", mode);
		print("eye", eye);
		print("forward", forward);
		print("up", up);
		print("at", position);
		print("direction", direction);
		print("x", to_world(axes, basis.forward));
		print("y", to_world(axes, basis.left));
		/* a sprite not facing the screen fades by its shader's mode */
		cross_product3d(&basis.forward, &basis.left, &basis.up);
		printf(" fade");
		for (fade_mode = 0; fade_mode<=2; fade_mode++)
			printf(" %.9g", build_sprite_compute_vertex_fade(
				fade_mode, (real_point3d const *)&offset, &basis.up));
		printf("\n");
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    sprite_c = read("source/render/render_sprite.c")
    sprite_h = read("source/render/render_sprite.h")
    real_math_h = read("source/math/real_math.h")
    real_math_c = read("source/math/real_math.c")
    enums = [
        enum_with(sprite_h, "_build_sprites_screen_space_bit"),
        enum_with(sprite_h, "_build_sprite_normal"),
    ]
    helpers = [inline(real_math_h, name) for name in INLINE_FUNCTIONS]
    helpers.append(function(real_math_c, "rotate_vector_about_axis"))
    definitions = [function(sprite_c, name) for name in SPRITE_FUNCTIONS]
    return "\n\n".join([FAKE_WORLD, *enums, *helpers, *definitions, MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="sprite-reference-") as work:
        source = Path(work) / "sprite_reference.c"
        executable = Path(work) / "sprite_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's sprite basis and fade (tools/engine_sprite_reference.py).\n"
        "# sprite <orientation> eye <xyz> forward <xyz> up <xyz> at <xyz> direction <xyz>"
        " x <xyz> y <xyz> fade <mode 0> <mode 1> <mode 2>\n"
        "# (the camera's forward and up as given, not yet made square; x and y the\n"
        "# axes the sprite's corners are laid along, in the world)\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_sprite_reference.py")
        print("the sprite reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
