"""The game's particle life, taken from the sources and run over many
particle definitions, to record the sequences, frames, ages and deaths the
native engine's must give.

    python3 tools/engine_particle_reference.py          # writes the reference
    python3 tools/engine_particle_reference.py --check  # fails if it is stale

The functions are source/effects/particles.c's `particle_new`,
`particles_update`, `particle_update_frame_time`, `particle_next_frame`,
`particle_next_sequence`, `particle_die`, `particle_get_radius` and
`new_particle_is_visible`, with source/math/random_math.c's seeded numbers
and source/math/real_math.h's local ones, found by name as tools/harness
finds them, and compiled for this machine against a small fake world: a
table of particle definitions and of their bitmaps' sequences, a data array
of one particle at a time, and stand-ins for the location, the lighting and
the physics. The physics (`particle_update_physics`, traced on its own)
puts the particle at rest or moves it off by a fixed pattern of its own,
which draws nothing from the local seed. Every particle is seen, so none is
deleted for going unrendered.

The trace is written to tools/traces/particle_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "particle_reference.txt"

RANDOM_FUNCTIONS = ["seed_random", "seed_random_range", "real_seed_random", "real_seed_random_range"]
LOCAL_RANDOM_FUNCTIONS = ["local_random", "local_random_range", "real_local_random_range"]
PARTICLE_FUNCTIONS = [
    "new_particle_is_visible",
    "particle_get_radius",
    "particle_die",
    "particle_next_sequence",
    "particle_next_frame",
    "particle_update_frame_time",
    "particle_new",
    "particles_update",
]

CASES = 400
UPDATES = 150

FAKE_WORLD = r"""
#include <float.h>
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef unsigned char byte;
typedef unsigned short word;
typedef float real;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define TICKS_PER_SECOND 30
#define RANDOM_A 1664525
#define RANDOM_C 1013904223
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define PIN(x, lo, hi) ((x) < (lo) ? (lo) : (x) > (hi) ? (hi) : (x))
#define NUMBEROF(a) (sizeof(a) / sizeof((a)[0]))
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])
#define match_assert_valid_real_vector3d(file, line, v) ((void)0)
#define match_assert_valid_real_point3d(file, line, p) ((void)0)
#define match_assert_valid_real_argb_color(file, line, c) ((void)0)
#define match_assert_valid_real_rgb_color(file, line, c) ((void)0)
#define profile_enter(section) ((void)0)
#define profile_exit(section) ((void)0)

enum { MAXIMUM_PARTICLE_UNRENDERED_FRAME_COUNT = 15 };
#define PARTICLE_SHARED_LIFESPAN 0.7f

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct { real red, green, blue; } real_rgb_color;
typedef struct { real alpha; real_rgb_color rgb; } real_argb_color;
typedef struct { real m[4][3]; } real_matrix4x3;
struct location { long cluster_index; long leaf_index; };

struct tag_reference { unsigned long group_tag; long index; };
struct tag_block { long count; void *address; };

struct bitmap_group_sequence { struct tag_block sprites; };
struct bitmap_group { struct tag_block sequences; };

enum
{
	_particle_datum_animates_backwards_bit = 0,
	_particle_datum_at_rest_bit,
	_particle_datum_u_mirror_bit,
	_particle_datum_v_mirror_bit,
	_particle_datum_dont_draw_first_person_bit,
	_particle_datum_dont_draw_third_person_bit,
	_particle_datum_attached_to_local_player_bit,
};

struct particle_datum
{
	word flags;
	long definition_index;
	long object_index;
	short node_index;
	byte state;
	byte local_player_index;
	long last_rendered_frame_index;
	real age;
	real lifespan;
	real frame_time;
	real frame_span;
	short sequence_index;
	short frame_index;
	struct location location;
	real_point3d position;
	real_vector3d direction;
	real_vector3d translational_velocity;
	real rotation;
	real angular_velocity;
	real radius;
	real_argb_color color;
};

struct particle_definition
{
	unsigned long flags;
	struct tag_reference bitmap;
	struct tag_reference physics;
	real life_span_lower_bound;
	real life_span_upper_bound;
	struct tag_reference effect;
	real radius_lower_bound;
	real radius_upper_bound;
	real frames_per_second_lower_bound;
	real frames_per_second_upper_bound;
	short first_sequence_index;
	short initial_sequence_count;
	short looping_sequence_count;
	short final_sequence_count;
};

struct new_particle_data
{
	long definition_index;
	long object_index;
	short node_index;
	short local_player_index;
	boolean attached_to_local_player;
	boolean dont_draw_first_person;
	boolean dont_draw_third_person;
	real_point3d position;
	real_vector3d direction;
	real_vector3d velocity;
	real_vector3d initial_impulse;
	real rotation;
	real angular_velocity;
	real radius;
	real_argb_color color;
};

/* the one definition and bitmap of a case */
static struct particle_definition the_definition;
static struct bitmap_group_sequence the_sequences[8];
static long the_sprites[8];
static struct bitmap_group the_bitmap;
static struct particle_definition *particle_definition_get(long index) { return &the_definition; }
static struct bitmap_group *bitmap_group_get(long index) { return &the_bitmap; }

/* a data array of one particle */
static struct particle_datum the_particle;
static boolean the_particle_lives;
static void *particle_data;
static struct particle_datum *particle_get(long index) { return &the_particle; }
static long datum_new(void *data) { memset(&the_particle, 0, sizeof(the_particle)); the_particle_lives = TRUE; return 0; }
static void datum_delete(void *data, long index) { the_particle_lives = FALSE; }
static long data_next_index(void *data, long index) { return index == NONE && the_particle_lives ? 0 : NONE; }

static struct { long frame_index; } render;
static short players;
static short local_player_count(void) { return players; }

static unsigned long local_seed;
static unsigned long *get_global_local_random_seed_address(void) { return &local_seed; }

/* the world: every point is in a leaf that is seen, lit white */
static void scenario_location_from_point(struct location *location, real_point3d const *point) { location->cluster_index = 0; location->leaf_index = 0; }
static boolean scenario_location_potentially_visible_local(struct location const *location) { return TRUE; }
static real_matrix4x3 *object_get_node_matrix(long object, short node) { return NULL; }
static real_matrix4x3 *first_person_weapon_get_node_matrix(short player, short node) { return NULL; }
static void matrix4x3_transform_point(real_matrix4x3 const *matrix, real_point3d const *in, real_point3d *out) { *out = *in; }
static void light_particle(real_point3d const *point, real_rgb_color *light, real_rgb_color *diffuse, boolean flag)
{
	light->red = light->green = light->blue = 1.0f;
	*diffuse = *light;
}
static void *point_physics_definition_get(long index) { return NULL; }
static real point_physics_definition_get_mass(void *definition, real radius) { return 0.0f; }

/* the death effect, which the trace shows */
static long effects_played;
static void particle_effect_new(struct particle_datum *particle, unsigned long group_tag, long effect_index, real scale) { effects_played++; }

/* the physics, traced on its own: it rests the particle or moves it off by a
fixed pattern, drawing nothing from the local seed */
static unsigned long physics_seed;
static long physics_updates;
static boolean particle_update_physics(long particle_index, real dt)
{
	physics_seed = physics_seed * 69069u + 1u;
	physics_updates++;
	SET_FLAG(the_particle.flags, _particle_datum_at_rest_bit, (physics_seed >> 28) < 5);
	return TRUE;
}

static real particles_leftover_ticks = 0.f;
static long particles_update_ticks = 0;
static long particles_tick_frame_indices[MAXIMUM_PARTICLE_UNRENDERED_FRAME_COUNT + 1];
static long particles_tick_frame_head = 0;

static boolean particle_next_sequence(long particle_index);
static boolean particle_next_frame(long particle_index);
static boolean particle_update_frame_time(long particle_index, real dt);
"""

MAIN = r"""
static unsigned long case_seed = 0x13579bdu;

static long next_case(long n)
{
	case_seed = case_seed * 1664525u + 1013904223u;
	return (long)((case_seed >> 8) % (unsigned long)n);
}

static real const lifespans[] = { 0.0f, 0.1f, 0.25f, 0.7f, 1.0f, 2.5f, 4.0f };
static real const rates[] = { 0.0f, 5.0f, 10.0f, 15.0f, 30.0f, 45.0f, 60.0f, 120.0f };
static real const radii[] = { 0.05f, 0.1f, 0.4f, 1.0f, 2.0f };
static real const steps[] = { 1.0f / 30.0f, 1.0f / 30.0f, 1.0f / 30.0f, 1.0f / 60.0f, 0.05f, 0.01f };
static unsigned long const life_flags[] = { 0, 1, 2, 3, 10, 11 };

static void print_particle(void)
{
	printf(" %d %d %d %.9g %.9g %d %d %.9g\n",
		the_particle.state, the_particle.sequence_index, the_particle.frame_index,
		the_particle.age, the_particle.frame_time, TEST_FLAG(the_particle.flags, _particle_datum_at_rest_bit),
		effects_played, particle_get_radius(0));
}

int main(void)
{
	long index;

	for (index = 0; index < CASES; index++)
	{
		struct new_particle_data data;
		long sequence;
		long bit;
		long update;
		real dt;
		real lower;
		real upper;

		memset(&the_definition, 0, sizeof(the_definition));
		for (bit = 0; bit < (long)NUMBEROF(life_flags); bit++)
		{
			if (next_case(3) == 0)
				the_definition.flags |= FLAG(life_flags[bit]);
		}
		lower = lifespans[next_case(NUMBEROF(lifespans))];
		upper = lifespans[next_case(NUMBEROF(lifespans))];
		the_definition.life_span_lower_bound = lower < upper ? lower : upper;
		the_definition.life_span_upper_bound = lower < upper ? upper : lower;
		the_definition.frames_per_second_lower_bound = rates[next_case(NUMBEROF(rates))];
		the_definition.frames_per_second_upper_bound = rates[next_case(NUMBEROF(rates))];
		the_definition.radius_lower_bound = radii[next_case(NUMBEROF(radii))];
		the_definition.radius_upper_bound = radii[next_case(NUMBEROF(radii))];
		the_definition.effect.index = next_case(2) ? 0 : NONE;
		the_definition.first_sequence_index = next_case(3);
		the_definition.initial_sequence_count = next_case(3);
		the_definition.looping_sequence_count = next_case(4);
		the_definition.final_sequence_count = next_case(3);
		the_bitmap.sequences.count = next_case(7);
		the_bitmap.sequences.address = the_sequences;
		for (sequence = 0; sequence < the_bitmap.sequences.count; sequence++)
		{
			the_sequences[sequence].sprites.count = next_case(4) ? 1 + next_case(8) : 0;
			the_sequences[sequence].sprites.address = the_sprites;
		}
		players = 1 + next_case(4);
		dt = steps[next_case(NUMBEROF(steps))];
		physics_seed = index;
		effects_played = 0;

		printf("case %u %.9g %.9g %.9g %.9g %.9g %.9g %d %d %d %d %d %d %.9g",
			the_definition.flags,
			the_definition.life_span_lower_bound, the_definition.life_span_upper_bound,
			the_definition.frames_per_second_lower_bound, the_definition.frames_per_second_upper_bound,
			the_definition.radius_lower_bound, the_definition.radius_upper_bound,
			the_definition.first_sequence_index, the_definition.initial_sequence_count,
			the_definition.looping_sequence_count, the_definition.final_sequence_count,
			the_definition.effect.index != NONE, players, dt);
		printf(" sprites");
		for (sequence = 0; sequence < the_bitmap.sequences.count; sequence++)
			printf(" %d", the_sequences[sequence].sprites.count);
		printf("\n");

		memset(&data, 0, sizeof(data));
		data.object_index = NONE;
		data.radius = 1.0f;
		data.color.alpha = 1.0f;
		the_particle_lives = FALSE;
		particle_new(&data);
		if (!the_particle_lives)
		{
			printf("born dead %d\n", effects_played);
			continue;
		}
		printf("born %d %d %d %.9g %.9g", the_particle.flags & 0xf, the_particle.sequence_index,
			the_particle.frame_index, the_particle.lifespan, the_particle.frame_span);
		print_particle();
		for (update = 0; update < UPDATES && the_particle_lives; update++)
		{
			boolean at_rest = TEST_FLAG(the_particle.flags, _particle_datum_at_rest_bit);

			physics_updates = 0;
			particles_update(dt);
			if (!the_particle_lives)
			{
				printf("dies %d %d %.9g %d\n", update, effects_played, the_particle.age, at_rest);
				break;
			}
			printf("update %d %d %d", particles_update_ticks, at_rest, physics_updates);
			print_particle();
		}
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    random_math_c = read("source/math/random_math.c")
    real_math_h = read("source/math/real_math.h")
    particles_c = read("source/effects/particles.c")
    random = [
        function(random_math_c, name).replace("\nunsigned short", "\nstatic unsigned short")
        for name in RANDOM_FUNCTIONS
    ]
    local = [inline(real_math_h, name) for name in LOCAL_RANDOM_FUNCTIONS]
    enums = [
        enum_with(particles_c, member)
        for member in ("_particle_definition_can_animate_backwards_bit", "_particle_state_next_sequence_initial")
    ]
    particles = [function(particles_c, name) for name in PARTICLE_FUNCTIONS]
    main = MAIN.replace("CASES", str(CASES)).replace("UPDATES", str(UPDATES))
    return "\n\n".join([FAKE_WORLD, *enums, *random, *local, *particles, main])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="particle-reference-") as work:
        source = Path(work) / "particle_reference.c"
        executable = Path(work) / "particle_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's particle life (tools/engine_particle_reference.py).\n"
        "# case <flags> <lifespan bounds> <fps bounds> <radius bounds> <first> <initial> <looping> <final>"
        " <has death effect> <players> <dt> sprites <sprites of each sequence>...\n"
        "# born dead <effects>: it had no sequence to show\n"
        "# born <backwards and mirror flags> <sequence> <frame> <lifespan> <frame span> <particle>\n"
        "# update <ticks> <at rest going in> <physics updates> <particle>\n"
        "# dies <update> <effects> <age> <at rest going in>\n"
        "# <particle>: <state> <sequence> <frame> <age> <frame time> <at rest> <effects> <radius>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_particle_reference.py")
        print("the particle reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
