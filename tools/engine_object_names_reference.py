"""The game's object names, taken from the sources and run over scenarios
of placements of every kind, to record which placement each name is given,
as the native importer must say.

    python3 tools/engine_object_names_reference.py          # writes the reference
    python3 tools/engine_object_names_reference.py --check  # fails if it is stale

The function is source/objects/object_types.c's `object_names_postprocess`,
found by name as tools/harness finds them and compiled for this machine
against a fake world: a scenario of a few names and, for each of the nine
kinds of object that have placements, a few placements whose name indices
are drawn from a fixed generator (some none, some past the names, many the
same). Each name is given the kind and the index of the placement that
names it last, the kinds in the game's order.

The trace is written to tools/traces/object_names_reference.txt,
which engine/crates/import/tests/object_names_reference.rs reads.
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

OUTPUT = ROOT / "tools" / "traces" / "object_names_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef unsigned char byte;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define SHORT_MAX 32767
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define VALID_INDEX(i, n) ((i) >= 0 && (i) < (n))
#define NUMBER_OF_OBJECT_TYPES 12
#define MAXIMUM_PLACEMENTS 8
#define MAXIMUM_NAMES 8

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

struct scenario_object_datum { short palette_index; short name_index; };
struct scenario_object_name { char name[32]; short runtime_object_type; short runtime_scenario_datum_index; };
struct scenario { struct tag_block object_names; };
struct object_type_definition { long placement_tag_block_offset; long palette_tag_block_offset; };

/* the kinds with no placements: garbage, projectiles, placeholders */
static struct object_type_definition definitions[NUMBER_OF_OBJECT_TYPES] = {
	{0, 0}, {0, 0}, {0, 0}, {0, 0}, {NONE, NONE}, {NONE, NONE},
	{0, 0}, {0, 0}, {0, 0}, {0, 0}, {NONE, NONE}, {0, 0},
};
static struct scenario_object_datum datums[NUMBER_OF_OBJECT_TYPES][MAXIMUM_PLACEMENTS];
static struct tag_block blocks[NUMBER_OF_OBJECT_TYPES];
static struct scenario_object_name names[MAXIMUM_NAMES];

static struct object_type_definition *object_type_definition_get(short object_type)
{
	return &definitions[object_type];
}

static struct tag_block *scenario_get_object_type_scenario_datums(
	struct scenario *scenario, short object_type, long *element_size)
{
	*element_size = sizeof(struct scenario_object_datum);
	return &blocks[object_type];
}

static void *tag_block_get_element_with_size(struct tag_block *block, long index, long size)
{
	return (byte *)block->address + index * size;
}
"""

DRIVER = r"""
static unsigned long seed = 1;

static long next(long n)
{
	seed = seed * 1103515245u + 12345u;
	return (long)((seed >> 16) % (unsigned long)n);
}

int main(void)
{
	struct scenario scenario;
	long trial;

	for (trial = 0; trial < 200; trial++)
	{
		short type;
		long index;
		long count = next(MAXIMUM_NAMES + 1);

		scenario.object_names.count = count;
		scenario.object_names.address = names;
		for (index = 0; index < MAXIMUM_NAMES; index++)
		{
			names[index].runtime_object_type = NONE;
			names[index].runtime_scenario_datum_index = NONE;
		}
		printf("scenario %d names %d\n", trial, count);
		for (type = 0; type < NUMBER_OF_OBJECT_TYPES; type++)
		{
			blocks[type].count = definitions[type].placement_tag_block_offset == NONE ? 0 : next(MAXIMUM_PLACEMENTS + 1);
			blocks[type].address = datums[type];
			for (index = 0; index < blocks[type].count; index++)
			{
				/* none, a name, or past the names */
				datums[type][index].name_index = (short)(next(MAXIMUM_NAMES + 3) - 1);
				printf("place %d %d %d\n", type, index, datums[type][index].name_index);
			}
		}
		object_names_postprocess(&scenario, FALSE);
		for (index = 0; index < count; index++)
			printf("name %d %d %d\n", index, names[index].runtime_object_type, names[index].runtime_scenario_datum_index);
	}
	return 0;
}
"""


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    objects = read("source/objects/object_types.c")
    return "\n".join([FAKE_WORLD, function(objects, "object_names_postprocess"), DRIVER])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="object-names-reference-") as work:
        source = Path(work) / "object_names_reference.c"
        executable = Path(work) / "object_names_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's object names (tools/engine_object_names_reference.py).\n"
        "# scenario <trial> names <how many names>\n"
        "# place <object type> <placement index> <name index>\n"
        "# name <name index> <object type> <placement index> (-1 -1: none)\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_object_names_reference.py")
        print("the object names reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
