"""The game's sound around the camera: how far sound travels between two of
a BSP's clusters, and the sound environment and background sound the
camera's cluster gives, glided to frame by frame, recorded for the
engine's to match.

    python3 tools/engine_sound_environment_reference.py          # writes the reference
    python3 tools/engine_sound_environment_reference.py --check  # fails if it is stale

The functions are source/structures/structure_bsp_definitions.c's
`structure_bsp_get_cluster_encoded_sound_data` and
`structure_bsp_get_cluster_encoded_sound_distance` (with the distance
game_sound.c reads from the byte) and source/scenario/scenario.c's
`scenario_get_sound_environment`, compiled for this machine against a
small fake world: BSPs of 1 to 40 clusters whose distances are any byte,
one with fewer distances than its pairs, asked for each pair and for
clusters past the last; and one local player's camera walking a BSP's
clusters (and out of them), whose palettes have environments of any
priority, entries without a tag, and indices past their ends. The
scenario has no fog: the engine's has none (a fog's environment is the
scenario's track). The table is written to
tools/traces/sound_environment_reference.txt, which
tools/engine_wasm_traces.py checks game.wasm against.
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from harness import constant, function, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "sound_environment_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef unsigned short word;
typedef byte boolean;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define UNSIGNED_CHAR_MAX 0xff
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : ((n)>(ceiling)?(ceiling):(n)))
#define VALID_INDEX(index, count) ((index)>=0 && (index)<(count))
#define match_assert(file, line, condition) ((void)0)

typedef struct { real x, y, z; } real_point3d;

struct tag_block { long count; void *address; };
struct tag_data { long size; void *address; };
struct tag_reference { long index; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

/* the BSP's fields the lookups read */
struct structure_cluster { short background_sound_palette_index; short sound_environment_palette_index; };
struct structure_sound_environment_palette_entry { struct tag_reference sound_environment; };
struct structure_background_sound_palette_entry { struct tag_reference background_sound; };
struct structure_bsp
{
	struct tag_block clusters;
	struct tag_data sound_cluster_data;
	struct tag_block sound_environment_palette;
	struct tag_block background_sound_palette;
};

/* a camera's fields the environment reads */
struct location { short cluster_index; };
struct observer_result { struct location location; real_point3d position; };

/* a fog's fields the environment reads: the scenario has none */
struct fog_definition { struct tag_reference sound_environment; long background_sound_index; word flags; };
#define scenario_get_fog_region_index(location, position) NONE
#define scenario_fog_region_get_fog_index(region) NONE
static struct fog_definition *fog_definition_get(long index) { return NULL; }

static boolean debug_sound_environment = FALSE;
static char temporary[256];
#define tag_get_name(index) ""
#define render_debug_string(flag, text) ((void)0)
"""

WORLD = r"""
#define ENVIRONMENTS 6
static struct sound_environment_definition environments[ENVIRONMENTS];
static struct sound_environment_definition *sound_environment_definition_get(long index) { return &environments[index]; }

static struct { boolean sound_environment_initialized; struct sound_environment_definition sound_environment; } globals;
static struct { boolean sound_environment_initialized; struct sound_environment_definition sound_environment; } *scenario_globals = &globals;

static struct structure_bsp *global_structure_bsp;
static struct observer_result camera;
static long local_player_get_player_index(short local_player_index) { return local_player_index==0 ? 0 : NONE; }
static struct observer_result const *observer_get_camera(short local_player_index) { return &camera; }
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

/* a real from -scale to scale, in steps of a thousandth of it */
static real between(real scale)
{
	return (real)((int)(next()>>16)%2001-1000)*0.001f*scale;
}

/* the distance game_sound.c reads from the encoded byte */
static real distance(byte encoded)
{
	return (encoded & ~FLAG(_cluster_sound_distance_unreachable_bit))
		* (MAXIMUM_CLUSTER_SOUND_DISTANCE / CLUSTER_SOUND_DISTANCE_VALUE_MASK);
}

static void distances(void)
{
	static short const counts[] = { 1, 2, 5, 17, 40 };
	static byte data[40*39/2];
	static struct structure_cluster clusters[40];
	int b, from, to, i;

	for (b = 0; b<(int)(sizeof(counts)/sizeof(counts[0])); b++)
	{
		struct structure_bsp bsp;
		short count = counts[b];
		long pairs = count*(count-1)/2;

		memset(&bsp, 0, sizeof(bsp));
		bsp.clusters.count = count;
		bsp.clusters.address = clusters;
		/* the fourth has fewer distances than its pairs */
		bsp.sound_cluster_data.size = b==3 ? pairs/2 : pairs;
		bsp.sound_cluster_data.address = data;
		printf("bsp %d %d %d", b, count, bsp.sound_cluster_data.size);
		for (i = 0; i<bsp.sound_cluster_data.size; i++)
		{
			data[i] = (byte)below(256);
			printf(" %d", data[i]);
		}
		printf("\n");
		for (from = 0; from<count; from++)
			for (to = 0; to<count+2; to++)
			{
				byte encoded = structure_bsp_get_cluster_encoded_sound_distance(&bsp, (short)from, (short)to);

				printf("distance %d %d %d %d %.9g\n", b, from, to, encoded, distance(encoded));
			}
	}
}

static void walk(void)
{
	static struct structure_cluster clusters[12];
	static struct structure_sound_environment_palette_entry environment_palette[7];
	static struct structure_background_sound_palette_entry background_palette[4];
	struct structure_bsp bsp;
	int e, c, i, frame, held = 0;

	memset(&bsp, 0, sizeof(bsp));
	bsp.clusters.count = 12;
	bsp.clusters.address = clusters;
	bsp.sound_environment_palette.count = 7;
	bsp.sound_environment_palette.address = environment_palette;
	bsp.background_sound_palette.count = 4;
	bsp.background_sound_palette.address = background_palette;
	global_structure_bsp = &bsp;

	for (e = 0; e<ENVIRONMENTS; e++)
	{
		real *values = &environments[e].room_intensity;

		environments[e].priority = e==2 ? -32768 : (short)(below(200)-100);
		values[0] = between(0.5f)+0.5f;
		values[1] = between(0.5f)+0.5f;
		values[2] = between(5.0f)+5.0f;
		values[3] = between(10.0f)+10.0f;
		values[4] = between(1.0f)+1.0f;
		values[5] = between(0.5f)+0.5f;
		values[6] = between(0.15f)+0.15f;
		values[7] = between(0.5f)+0.5f;
		values[8] = between(0.05f)+0.05f;
		values[9] = between(0.5f)+0.5f;
		values[10] = between(0.5f)+0.5f;
		values[11] = between(10000.0f)+10000.0f;
		printf("environment %d %d", e, environments[e].priority);
		for (i = 0; i<12; i++)
			printf(" %.9g", values[i]);
		printf("\n");
	}
	/* the palette's last entry has no tag */
	for (i = 0; i<7; i++)
	{
		environment_palette[i].sound_environment.index = i<ENVIRONMENTS ? i : NONE;
		printf("environment_palette %d %d\n", i, environment_palette[i].sound_environment.index);
	}
	for (i = 0; i<4; i++)
	{
		background_palette[i].background_sound.index = i==1 ? NONE : 100+i;
		printf("background_palette %d %d\n", i, background_palette[i].background_sound.index);
	}
	/* indices from none to past the palettes' ends */
	for (c = 0; c<12; c++)
	{
		clusters[c].sound_environment_palette_index = (short)(below(9)-1);
		clusters[c].background_sound_palette_index = (short)(below(6)-1);
		printf("cluster %d %d %d\n", c, clusters[c].sound_environment_palette_index, clusters[c].background_sound_palette_index);
	}

	scenario_globals->sound_environment = default_sound_environment;
	scenario_globals->sound_environment_initialized = FALSE;
	camera.location.cluster_index = NONE;
	for (frame = 0; frame<600; frame++)
	{
		long background_sound_index;
		struct sound_environment_definition const *heard;
		boolean crossed = 2;
		real const *values;

		/* the camera stays in a cluster a while, then moves to another or
		out of them all */
		if (held--<=0)
		{
			camera.location.cluster_index = below(8)==0 ? NONE : (short)below(12);
			held = below(40);
		}
		scenario_get_sound_environment(&background_sound_index, &heard, &crossed);
		values = &heard->room_intensity;
		printf("frame %d %d %d %d", frame, camera.location.cluster_index, crossed, background_sound_index);
		for (i = 0; i<12; i++)
			printf(" %.9g", values[i]);
		printf("\n");
	}
}

int main(void)
{
	distances();
	walk();
	return 0;
}
"""


def default_environment(definitions_c):
    """The game's default environment, the global as it is defined."""
    start = definitions_c.index("struct sound_environment_definition const default_sound_environment")
    return definitions_c[start : definitions_c.index("};", start) + 2]


def maximum_distance(game_sound_c):
    """MAXIMUM_CLUSTER_SOUND_DISTANCE, a real #define."""
    match = re.search(r"#define MAXIMUM_CLUSTER_SOUND_DISTANCE ([0-9.]+f)", game_sound_c)
    if not match:
        raise LookupError("MAXIMUM_CLUSTER_SOUND_DISTANCE")
    return f"#define MAXIMUM_CLUSTER_SOUND_DISTANCE {match.group(1)}"


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    definitions_h = read("source/sound/sound_environment_definitions.h")
    definitions_c = read("source/sound/sound_environment_definitions.c")
    structure_c = read("source/structures/structure_bsp_definitions.c")
    scenario_c = read("source/scenario/scenario.c")
    game_sound_c = read("source/sound/game_sound.c")
    constants = [
        maximum_distance(game_sound_c),
        *(
            f"#define {name} {constant(game_sound_c, name)}"
            for name in ("_cluster_sound_distance_unreachable_bit", "CLUSTER_SOUND_DISTANCE_VALUE_MASK")
        ),
    ]
    return "\n\n".join(
        [
            FAKE_WORLD,
            *constants,
            structure(definitions_h, "sound_environment_definition"),
            default_environment(definitions_c),
            WORLD,
            function(structure_c, "structure_bsp_get_cluster_encoded_sound_data"),
            function(structure_c, "structure_bsp_get_cluster_encoded_sound_distance"),
            function(scenario_c, "scenario_get_sound_environment"),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="sound-environment-reference-") as work:
        source = Path(work) / "sound_environment_reference.c"
        executable = Path(work) / "sound_environment_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's cluster sound distances and sound environments (tools/engine_sound_environment_reference.py).\n"
        "# bsp <index> <clusters> <distances> <each distance's byte...>\n"
        "# distance <bsp> <from> <to> <encoded> <distance>\n"
        "# environment <index> <priority> <its twelve values>\n"
        "# environment_palette <index> <environment or -1>\n"
        "# background_palette <index> <background sound or -1>\n"
        "# cluster <index> <environment palette index> <background palette index>\n"
        "# frame <frame> <camera's cluster or -1> <crossed water> <background sound or -1> <the twelve values heard>\n"
        + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_sound_environment_reference.py")
        print("the sound environment reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
