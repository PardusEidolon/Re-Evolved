"""When the game's netgame equipment stands its items and purges the
untaken, taken from the sources, to check the engine's `Game` against.

    python3 tools/engine_item_spawn_reference.py          # writes the reference
    python3 tools/engine_item_spawn_reference.py --check  # fails if it is stale

The functions are source/game/game_engine.c's `game_engine_update_purge`,
`game_engine_update_item_spawn`, `random_item` and
`item_collection_get_total`, with source/math/random_math.c's `seed_random`
and `seed_random_range`, compiled for this machine against a fake scenario:
a few placements with their own spawn times or their collections', and
collections with whole, fractional, zero and empty weights. Each of the
game's ticks purges, then stands, as `game_engine_update` does; now and then
a player takes the oldest item lying there, which the purge then passes by.

Every placement plays (the engine keeps those of the game type when the game
starts), the game type has its map's weapons, and no biped lies dead: the
purge's dead bipeds and `can_delete_item`'s flags are the units' and the
flags' own.

The trace is written to tools/traces/item_spawn_reference.txt, which
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

OUTPUT = ROOT / "tools" / "traces" / "item_spawn_reference.txt"

RANDOM_FUNCTIONS = ["seed_random", "seed_random_range"]
ENGINE_FUNCTIONS = [
    "item_collection_get_total",
    "random_item",
    "game_engine_update_purge",
    "game_engine_update_item_spawn",
]

FAKE_WORLD = r"""
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef unsigned char boolean;
typedef float real;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define TICKS_PER_SECOND 30
#define RANDOM_A 1664525u
#define RANDOM_C 1013904223u
#define NUMBEROF(array) (sizeof(array) / sizeof((array)[0]))
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags)) >> (bit)) & 1)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define TAG_BLOCK_GET_ELEMENT(block, index, type) ((type *)(block)->address + (index))
#define match_assert(file, line, condition) ((void)0)

typedef struct { real x, y, z; } real_point3d;
typedef struct { real i, j, k; } real_vector3d;

struct tag_block { long count; void *address; };
struct tag_reference { long index; };

struct scenario_netgame_equipment
{
	long flags;
	short game_type[4];
	short team_index;
	short spawn_time;
	real_point3d position;
	real facing;
	struct tag_reference item_collection;
};

struct scenario { struct tag_block netgame_equipment; };

struct item_permutation_definition { real weight; struct tag_reference item; };
struct item_collection_definition { struct tag_block permutations; short spawn_time; };

enum { _equipment_created_at_rest_bit = 0 };
enum { _object_at_rest_bit = 1 };
enum { _item_attached_to_unit_bit = 4 };
enum { _object_dead_bit = 0 };
enum { _object_mask_item = 1, _object_mask_biped = 2 };
enum { _object_type_weapon = 2, _object_type_equipment = 3 };

#define PLACEMENTS 6
#define COLLECTIONS 5
#define MAXIMUM_PERMUTATIONS 4
#define MAXIMUM_ITEMS 512

static struct scenario_netgame_equipment placements[PLACEMENTS] = {
	/* (flags, game types, team, spawn time, position, facing, collection) */
	{ 0, { 1 }, 0, 0, { 0 }, 0, { 0 } },
	{ 1, { 1 }, 0, 0, { 0 }, 0, { 1 } },
	{ 0, { 1 }, 0, 45, { 0 }, 0, { 2 } },
	{ 0, { 1 }, 0, 7, { 0 }, 0, { 3 } },
	{ 0, { 1 }, 0, -10, { 0 }, 0, { 4 } },
	{ 0, { 1 }, 0, 0, { 0 }, 0, { 4 } },
};

static struct item_permutation_definition permutations[COLLECTIONS][MAXIMUM_PERMUTATIONS] = {
	{ { 1.0f, { 10 } } },
	{ { 0.5f, { 20 } }, { 1.7f, { 21 } }, { 2.6f, { 22 } } },
	{ { 3.0f, { 30 } }, { 0.0f, { 31 } }, { 1.0f, { NONE } } },
	{ { 10.0f, { 40 } }, { 25.5f, { 41 } }, { 0.25f, { 42 } }, { 7.0f, { 43 } } },
	{ { 0.9f, { 50 } }, { 0.9f, { 51 } } },
};

static long const permutation_counts[COLLECTIONS] = { 1, 3, 3, 4, 2 };
static short const collection_spawn_times[COLLECTIONS] = { 0, 20, 0, 0, -3 };

static struct item_collection_definition collections[COLLECTIONS];
static struct scenario scenario = { { PLACEMENTS, placements } };

static struct scenario *global_scenario_get(void) { return &scenario; }

static struct item_collection_definition *item_collection_definition_get(long index)
{
	return &collections[index];
}

static unsigned long global_random_seed = 0xDEADBEEF;
static unsigned long *get_global_random_seed_address(void) { return &global_random_seed; }

static boolean network_game_distributed_client(void) { return FALSE; }
static boolean custom_edition_cache_tags_loaded(void) { return FALSE; }
static void vector3d_from_angle(real_vector3d *vector, real angle) {}
static short game_engine_get_type(void) { return 1; }
static boolean match_game_type(short type, long count, short const *types) { return TRUE; }

struct game_variant_options { boolean no_map_weapons; };
static struct game_variant_options options;
static struct game_variant_options *game_variant_options_get(void) { return &options; }

struct object_definition { struct { short type; } object; };
static struct object_definition equipment_definition = { { _object_type_equipment } };
static struct object_definition *object_definition_get(long index) { return &equipment_definition; }

static long game_time;
static long game_time_get(void) { return game_time; }

struct object_placement_data
{
	long definition_index;
	real_point3d position;
	real_vector3d forward;
};

static void object_placement_data_new(struct object_placement_data *data, long definition_index, long owner)
{
	data->definition_index = definition_index;
}

struct item_datum
{
	boolean exists;
	long definition_index;
	struct { unsigned long flags; } object;
	struct { unsigned long flags; long last_owned_time; } item;
};

static struct item_datum items[MAXIMUM_ITEMS];
static long item_count;

static long object_new(struct object_placement_data *data)
{
	struct item_datum *item;

	if (data->definition_index == NONE || item_count >= MAXIMUM_ITEMS)
		return NONE;
	item = &items[item_count];
	item->exists = TRUE;
	item->definition_index = data->definition_index;
	item->object.flags = 0;
	item->item.flags = 0;
	item->item.last_owned_time = game_time;
	return item_count++;
}

static struct item_datum *item_get(long index) { return &items[index]; }
static void object_set_garbage(long index, boolean garbage) {}

static void object_delete(long index)
{
	items[index].exists = FALSE;
	printf("purge %d %d\n", game_time, index);
}

static boolean can_delete_item(long index) { return TRUE; }

struct object_iterator { long index; long next; unsigned long mask; };

static void object_iterator_new(struct object_iterator *iterator, unsigned long mask, long flags)
{
	iterator->index = NONE;
	iterator->next = 0;
	iterator->mask = mask;
}

static boolean object_iterator_next(struct object_iterator *iterator)
{
	/* (no bipeds) */
	if (iterator->mask != _object_mask_item)
		return FALSE;
	while (iterator->next < item_count && !items[iterator->next].exists)
		iterator->next++;
	if (iterator->next >= item_count)
		return FALSE;
	iterator->index = iterator->next++;
	return TRUE;
}

struct object_datum { struct { short idle_ticks; unsigned long damage_flags; } object; };
static struct object_datum *object_get(long index) { return NULL; }
"""

MAIN = r"""
int main(void)
{
	long index, permutation, last_spawned = 0;

	for (index = 0; index < COLLECTIONS; index++)
	{
		collections[index].permutations.count = permutation_counts[index];
		collections[index].permutations.address = permutations[index];
		collections[index].spawn_time = collection_spawn_times[index];
		printf("collection %d %d", index, collections[index].spawn_time);
		for (permutation = 0; permutation < permutation_counts[index]; permutation++)
			printf(" %.9g:%d", permutations[index][permutation].weight, permutations[index][permutation].item.index);
		printf("\n");
	}
	for (index = 0; index < PLACEMENTS; index++)
		printf("placement %d %d %d\n", index, placements[index].spawn_time, placements[index].item_collection.index);

	for (game_time = 0; game_time <= 6000; game_time++)
	{
		game_engine_update_purge();
		last_spawned = item_count;
		game_engine_update_item_spawn();
		for (index = last_spawned; index < item_count; index++)
			printf("spawn %d %d %d\n", game_time, index, items[index].definition_index);

		/* (now and then a player takes the oldest item lying there) */
		if (game_time % 437 == 5)
		{
			for (index = 0; index < item_count; index++)
			{
				if (items[index].exists && !TEST_FLAG(items[index].item.flags, _item_attached_to_unit_bit))
				{
					SET_FLAG(items[index].item.flags, _item_attached_to_unit_bit, TRUE);
					printf("take %d %d\n", game_time, index);
					break;
				}
			}
		}
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the run, as one C file."""
    random_c = read("source/math/random_math.c")
    engine_c = read("source/game/game_engine.c")
    return "\n\n".join(
        [
            FAKE_WORLD,
            *(function(random_c, name) for name in RANDOM_FUNCTIONS),
            *(function(engine_c, name) for name in ENGINE_FUNCTIONS),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="item-spawn-reference-") as work:
        source = Path(work) / "item_spawn_reference.c"
        executable = Path(work) / "item_spawn_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's netgame items over 6000 of its ticks (tools/engine_item_spawn_reference.py).\n"
        "# collection <index> <spawn time> <weight>:<item, -1 for none>...\n"
        "# placement <index> <spawn time> <collection>\n"
        "# purge|spawn|take <game tick> <object> [<item>]\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_item_spawn_reference.py")
        print("the item spawn reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
