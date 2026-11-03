"""The game's sound obstruction: how much a sound's direct path to the
camera is blocked and how much it is shut away, by the BSP's cluster sound
distances, the camera cluster's potentially visible set and a line from the
camera, recorded for the engine's to match.

    python3 tools/engine_sound_obstruction_reference.py          # writes the reference
    python3 tools/engine_sound_obstruction_reference.py --check  # fails if it is stale

The functions are source/sound/game_sound.c's `compute_sound_obstruction`
and source/structures/structure_bsp_definitions.c's
`structure_bsp_get_cluster_encoded_sound_distance` and
`structure_bsp_get_cluster_pvs`, compiled for this machine against a small
fake world: two BSPs of 20 clusters, with distances of any byte and any
visible sets, the second with visible sets for only some of its clusters;
a camera and a source in any of their clusters or outside them, any
distance apart, with the line between them blocked or clear. The table is
written to tools/traces/sound_obstruction_reference.txt, which
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
from harness import constant, function, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "sound_obstruction_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef byte boolean;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define UNSIGNED_CHAR_MAX 0xff
#define LONG_BITS 32
#define LONG_BITS_BITS 5
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define BIT_VECTOR_SIZE_IN_LONGS(bit_count) (((bit_count) + (LONG_BITS - 1)) >> LONG_BITS_BITS)
#define BIT_VECTOR_SIZE_IN_BYTES(bit_count) (sizeof(long) * BIT_VECTOR_SIZE_IN_LONGS(bit_count))
#define BIT_VECTOR_TEST_FLAG(bit_vector, bit) (TEST_FLAG((bit_vector)[(bit) >> LONG_BITS_BITS], ((bit) & (LONG_BITS - 1))))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : ((n)>(ceiling)?(ceiling):(n)))
#define match_assert(file, line, condition) ((void)0)
#define MAXIMUM_CLUSTERS_PER_STRUCTURE 512

typedef struct { real x, y, z; } real_point3d;
typedef struct { real i, j, k; } real_vector3d;

static void vector_from_points3d(real_point3d const *a, real_point3d const *b, real_vector3d *v)
{
	v->i = b->x-a->x;
	v->j = b->y-a->y;
	v->k = b->z-a->z;
}

struct tag_block { long count; void *address; };
struct tag_data { long size; void *address; };

/* the BSP's fields the lookups read */
struct structure_bsp
{
	struct tag_block clusters;
	struct tag_data cluster_data;
	struct tag_data sound_cluster_data;
};

/* a camera's and a sound's fields the obstruction reads */
struct location { short cluster_index; };
struct observer_result { struct location location; real_point3d position; };
enum { _sound_spatialization_mode_absolute = 1 };
struct sound_source
{
	struct { real_point3d position; struct location game_location; } location;
	short spatialization_mode;
	real obstruction;
	real occlusion;
};

/* the collision test the camera's line is cast by */
enum
{
	_collision_test_front_facing_surfaces_bit,
	_collision_test_structure_bit,
	_collision_test_media_bit,
	_collision_test_objects_bit,
	_collision_test_objects_scenery_bit,
	_collision_test_objects_machines_bit,
};
struct collision_result { long unused; };
enum { _collision_user_sounds = 3, MAXIMUM_COLLISION_USER_STACK_DEPTH = 8 };
static long global_current_collision_users[MAXIMUM_COLLISION_USER_STACK_DEPTH];
static short global_current_collision_user_depth = 1;
static boolean blocked;
static int tested;
static boolean collision_test_vector(unsigned long flags, real_point3d const *point, real_vector3d const *vector,
	long ignore, struct collision_result *collision)
{
	tested++;
	return blocked;
}

static struct structure_bsp *bsp;
static struct structure_bsp *global_structure_bsp_get(void) { return bsp; }
static struct observer_result camera;
static struct observer_result const *observer_get_camera(short local_player_index) { return &camera; }
static long now;
static long game_time_get(void) { return now; }
"""

MAIN = r"""
static unsigned int seed = 0x5eed;

static unsigned int next(void)
{
	seed = seed*1664525u+1013904223u;
	return seed;
}

/* an integer from 0 to below count */
static int below(int count)
{
	return (int)((next()>>16)%(unsigned)count);
}

#define CLUSTERS 20
#define BSPS 2

int main(void)
{
	static byte distances[BSPS][CLUSTERS*(CLUSTERS-1)/2];
	static unsigned long visible[BSPS][CLUSTERS];
	static struct structure_bsp bsps[BSPS];
	int b, i, trial;

	for (b = 0; b<BSPS; b++)
	{
		bsps[b].clusters.count = CLUSTERS;
		bsps[b].sound_cluster_data.size = sizeof(distances[b]);
		bsps[b].sound_cluster_data.address = distances[b];
		/* the second has visible sets for only its first 13 clusters */
		bsps[b].cluster_data.size = (b==1 ? 13 : CLUSTERS)*sizeof(unsigned long);
		bsps[b].cluster_data.address = visible[b];
		printf("bsp %d %d", b, CLUSTERS);
		for (i = 0; i<(int)sizeof(distances[b]); i++)
		{
			/* reachable distances, unreachable ones and the farthest */
			switch (below(4))
			{
			case 0: distances[b][i] = (byte)(0x80|below(128)); break;
			case 1: distances[b][i] = (byte)(120+below(8)); break;
			default: distances[b][i] = (byte)below(120); break;
			}
			printf(" %d", distances[b][i]);
		}
		printf("\n");
		for (i = 0; i<CLUSTERS; i++)
		{
			visible[b][i] = (next()&0xFFFFF000u)>>12 | FLAG(i);
			if (i<bsps[b].cluster_data.size/(int)sizeof(unsigned long))
				printf("visible %d %d %u\n", b, i, visible[b][i]);
		}
	}

	for (trial = 0; trial<3000; trial++)
	{
		struct sound_source source;
		real distance = (real)(1+below(4000))*0.05f;

		b = below(BSPS);
		bsp = &bsps[b];
		camera.location.cluster_index = below(10)==0 ? NONE : (short)below(CLUSTERS);
		source.location.game_location.cluster_index = below(10)==0 ? NONE : (short)below(CLUSTERS);
		source.location.position.x = (real)trial;
		source.location.position.y = distance;
		source.location.position.z = 0.0f;
		source.spatialization_mode = _sound_spatialization_mode_absolute;
		blocked = (boolean)below(2);
		tested = 0;
		now = trial;
		compute_sound_obstruction(0, &source, distance);
		printf("trial %d %d %d %d %.9g %d %d %.9g %.9g\n", trial, b, camera.location.cluster_index,
			source.location.game_location.cluster_index, distance, blocked, tested, source.obstruction, source.occlusion);
	}
	return 0;
}
"""


def maximum_distance(game_sound_c):
    """MAXIMUM_CLUSTER_SOUND_DISTANCE, a real #define."""
    line = next(line for line in game_sound_c.splitlines() if line.startswith("#define MAXIMUM_CLUSTER_SOUND_DISTANCE"))
    return line


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    structure_c = read("source/structures/structure_bsp_definitions.c")
    game_sound_c = read("source/sound/game_sound.c")
    start = game_sound_c.index("#define OBSTRUCTION_CACHE_SLOTS")
    cache = game_sound_c[start : game_sound_c.index("void compute_sound_obstruction(", start)]
    return "\n\n".join(
        [
            FAKE_WORLD,
            maximum_distance(game_sound_c),
            *(
                f"#define {name} {constant(game_sound_c, name)}"
                for name in ("_cluster_sound_distance_unreachable_bit", "CLUSTER_SOUND_DISTANCE_VALUE_MASK")
            ),
            "static unsigned long structure_bsp_empty_cluster_pvs[BIT_VECTOR_SIZE_IN_LONGS(MAXIMUM_CLUSTERS_PER_STRUCTURE)];",
            function(structure_c, "structure_bsp_get_cluster_pvs"),
            function(structure_c, "structure_bsp_get_cluster_encoded_sound_data"),
            function(structure_c, "structure_bsp_get_cluster_encoded_sound_distance"),
            cache,
            function(game_sound_c, "compute_sound_obstruction"),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="sound-obstruction-reference-") as work:
        source = Path(work) / "sound_obstruction_reference.c"
        executable = Path(work) / "sound_obstruction_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's sound obstruction (tools/engine_sound_obstruction_reference.py).\n"
        "# bsp <index> <clusters> <each pair's distance byte...>\n"
        "# visible <bsp> <cluster> <its visible set's bits>: clusters without a line have none\n"
        "# trial <index> <bsp> <camera's cluster or -1> <source's cluster or -1> <distance> <line blocked>"
        " <lines tested> <obstruction> <occlusion>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_sound_obstruction_reference.py")
        print("the sound obstruction reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
