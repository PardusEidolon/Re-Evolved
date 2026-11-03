"""The weather palette entry the game's wind blows from in each cluster,
taken from the sources and run over many small made-up BSPs, to record
what the engine's import must choose.

    python3 tools/engine_wind_weather_reference.py          # writes the reference
    python3 tools/engine_wind_weather_reference.py --check  # fails if it is stale

The functions are source/scenario/wind.c's `scenario_get_wind` and
`scenario_get_current`, and source/scenario/scenario.c's
`scenario_get_fog_region_index` and `scenario_fog_region_get_fog_index`,
compiled for this machine against a fake world: BSPs drawn from a fixed
seed, each with fog regions, a fog palette naming mist, water or no fog,
fog planes and clusters whose fog references name none, a region or a
plane (with indices the map does not have among them).
`scenario_get_current_from_weather_palette` is replaced by one that prints
the palette entry it is handed, which is what the trace records for each
cluster at points on both sides of its plane.

The trace is written to tools/traces/wind_weather_reference.txt,
which engine/crates/import/src/level/wind_weather_reference.rs reads.
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

OUTPUT = ROOT / "tools" / "traces" / "wind_weather_reference.txt"

SCENARIO_FUNCTIONS = ["scenario_fog_region_get_fog_index", "scenario_get_fog_region_index"]
WIND_FUNCTIONS = ["scenario_get_current", "scenario_get_wind"]

FAKE_WORLD = r"""
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef unsigned short word;
typedef byte boolean;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define SHORT_MAX 0x7fff
#define FLAG(b) (1<<(b))
#define TEST_FLAG(f,b) (((f)&FLAG(b))!=0)
#define VALID_INDEX(i,n) ((i)>=0 && (i)<(n))

enum
{
	_scenario_current_simple_bit = 0,
	_scenario_current_damped_bit,
	_scenario_current_force_water_bit,
	_scenario_current_force_no_water_bit,
};

typedef struct { real i, j, k; } real_vector3d;
typedef union { struct { real x, y, z; }; real n[3]; } real_point3d;
typedef struct { real_vector3d n; real d; } real_plane3d;

static real plane3d_distance_to_point(real_plane3d const *plane, real_point3d const *point)
{
	return point->x*plane->n.i+point->y*plane->n.j+point->z*plane->n.k-plane->d;
}

struct tag_reference { long index; };
struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

struct location { long leaf_index; short cluster_index; };

/* the fields the wind reads */
struct structure_cluster { short fog_reference; short weather_palette_index; };
struct structure_fog_plane { short region_index; real_plane3d plane; };
struct structure_fog_region { short fog_palette_index; short weather_palette_index; };
struct structure_fog_palette_entry { struct tag_reference fog; };
struct fog_definition { unsigned long flags; real plane_distance; };

struct structure_bsp
{
	struct tag_block clusters;
	struct tag_block fog_planes;
	struct tag_block fog_regions;
	struct tag_block fog_palette;
};

/* the two fogs: mist, and water that begins a little below its plane */
static struct fog_definition fogs[2] = { { 0, 0.75f }, { 1, 0.5f } };
static struct fog_definition *fog_definition_get(long index) { return &fogs[index]; }

static struct structure_cluster clusters[8];
static struct structure_fog_plane planes[3];
static struct structure_fog_region regions[5];
static struct structure_fog_palette_entry palette[3];
static struct structure_bsp bsp =
{
	{ 0, clusters }, { 0, planes }, { 0, regions }, { 0, palette },
};

static struct structure_bsp *global_structure_bsp_get(void) { return &bsp; }

/* the palette entry the wind is drawn from, which the trace records */
static short chosen;

static void scenario_get_current_from_weather_palette(
	real_point3d const *position, real_vector3d *current, long flags, short weather_palette_index)
{
	chosen = weather_palette_index;
	current->i = current->j = current->k = 0.f;
}
"""

MAIN = r"""
static unsigned long seed = 0x2468ace;

static int next_random(int n)
{
	seed = seed*1664525u+1013904223u;
	return (int)((seed>>8)%(unsigned long)n);
}

int main(void)
{
	int world;

	for (world = 0; world<400; world++)
	{
		int index, point;

		bsp.fog_regions.count = next_random(5);
		bsp.fog_palette.count = next_random(4);
		bsp.fog_planes.count = next_random(4);
		bsp.clusters.count = 1+next_random(8);
		printf("world %d\n", world);
		for (index = 0; index<bsp.fog_regions.count; index++)
		{
			regions[index].fog_palette_index = (short)(next_random(5)-1);
			regions[index].weather_palette_index = (short)(next_random(7)-2);
			printf("region %d %d\n", regions[index].fog_palette_index, regions[index].weather_palette_index);
		}
		for (index = 0; index<bsp.fog_palette.count; index++)
		{
			palette[index].fog.index = next_random(3)-1;
			printf("palette %d\n", palette[index].fog.index==NONE ? -1 : (int)fogs[palette[index].fog.index].flags);
		}
		for (index = 0; index<bsp.fog_planes.count; index++)
		{
			planes[index].region_index = (short)(next_random(6)-1);
			planes[index].plane.n.i = 0.f;
			planes[index].plane.n.j = 0.f;
			planes[index].plane.n.k = 1.f;
			planes[index].plane.d = (real)(next_random(5)-2);
			printf("plane %d %.9g\n", planes[index].region_index, planes[index].plane.d);
		}
		for (index = 0; index<bsp.clusters.count; index++)
		{
			struct location location = { 0, (short)index };
			short reference;

			switch (next_random(3))
			{
			case 0: reference = NONE; break;
			case 1: reference = (short)next_random(6); break;
			default: reference = (short)(0x8000|next_random(4)); break;
			}
			clusters[index].fog_reference = reference;
			clusters[index].weather_palette_index = (short)(next_random(6)-1);
			for (point = 0; point<4; point++)
			{
				real_point3d position = { { 0.f, 0.f, (real)point-1.75f } };
				real_vector3d current;

				scenario_get_wind(&location, &position, &current, next_random(4));
				printf("wind %d %d %.9g %d\n", clusters[index].weather_palette_index, reference,
					position.z, chosen);
			}
		}
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    scenario_c = read("source/scenario/scenario.c")
    wind_c = read("source/scenario/wind.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            *(function(scenario_c, name) for name in SCENARIO_FUNCTIONS),
            *(function(wind_c, name) for name in WIND_FUNCTIONS),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="wind-weather-reference-") as work:
        source = Path(work) / "wind_weather_reference.c"
        executable = Path(work) / "wind_weather_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's wind's weather palette entries (tools/engine_wind_weather_reference.py).\n"
        "# world <n>: a new BSP, its tables following\n"
        "# region <fog palette entry> <weather palette entry>\n"
        "# palette <-1: no fog, 0: mist, 1: water>\n"
        "# plane <region> <height>: a level plane\n"
        "# wind <cluster's weather> <fog reference> <point's height> <the entry blown from>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_wind_weather_reference.py")
        print("the wind weather reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
