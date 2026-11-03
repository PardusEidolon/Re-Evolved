"""The game's object table, run through a long sequence of objects made and
deleted, to record which slot each new object takes, the order the table is
walked in, when it is full and that a deleted object's handle is refused.

    python3 tools/engine_data_reference.py          # writes the reference
    python3 tools/engine_data_reference.py --check  # fails if it is stale

The functions are source/memory/data.c's (`datum_new`, `datum_delete`,
`data_next_index`, `data_iterator_next`, `datum_try_and_get` and the rest)
and objects.c's header table (`object_header_new`, `object_header_delete`),
compiled for this machine against the shared fake world (tools/harness/world.py).
The table holds the retail game's 2048 objects. The trace is written to
tools/traces/data_reference.txt, which
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
from harness import world  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "data_reference.txt"

MAIN = r"""
#define OBJECTS 2048
#define STEPS 1500

static unsigned long seed = 0x2545f491;

static unsigned long next_random(void)
{
	seed = seed*1664525u + 1013904223u;
	return seed >> 8;
}

static long live(struct data_array *data, long *indices)
{
	long count = 0;
	long index = NONE;

	while ((index = data_next_index(data, index)) != NONE)
		indices[count++] = index;
	return count;
}

static void walk(struct data_array *data)
{
	static long indices[OBJECTS];
	struct data_iterator iterator;
	long count = live(data, indices);
	long at;

	/* the iterator walks the table as data_next_index does */
	data_iterator_new(&iterator, data);
	for (at = 0; at < count; at++)
	{
		if (!data_iterator_next(&iterator) || iterator.datum_index != indices[at])
		{
			printf("iterator differs at %d\n", at);
			exit(1);
		}
	}
	if (data_iterator_next(&iterator))
	{
		printf("iterator goes on\n");
		exit(1);
	}

	printf("walk");
	for (at = 0; at < count; at++)
		printf(" %d", indices[at] & 0xffff);
	printf("\n");
}

int main(void)
{
	static long indices[OBJECTS];
	struct data_array *data = data_new("object", OBJECTS, sizeof(struct object_header_datum));
	long step;
	long made = 0;

	data_make_valid(data);
	for (step = 0; step < STEPS; step++)
	{
		long count = live(data, indices);

		/* more made than deleted, so the table grows, with holes */
		if (count == 0 || next_random() % 100 < 55)
		{
			long index = object_header_new(data, NONE, 64);

			printf("new %d\n", index & 0xffff);
			made++;
		}
		else
		{
			long index = indices[next_random() % count];

			object_header_delete(data, index);
			printf("delete %d\n", index & 0xffff);
			/* its handle is refused, though the slot is free or taken again */
			if (datum_try_and_get(data, index))
			{
				printf("a deleted handle is still taken\n");
				exit(1);
			}
			if (next_random() % 2)
			{
				long again = object_header_new(data, NONE, 64);

				printf("new %d\n", again & 0xffff);
				printf("stale %d %d\n", index & 0xffff, datum_try_and_get(data, index) != NULL);
			}
		}
		walk(data);
	}

	/* the memory pool refuses a block: no object, and the slot is free again */
	world_pool_refuses = TRUE;
	printf("refused %d\n", object_header_new(data, NONE, 64));
	world_pool_refuses = FALSE;
	walk(data);

	/* filled to the end */
	{
		long index;
		long filled = 0;

		while ((index = object_header_new(data, NONE, 64)) != NONE)
			filled++;
		printf("full %d %d\n", filled, live(data, indices));
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    return "\n\n".join([world.prelude(), world.data_functions(), world.object_functions(), MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="data-reference-") as work:
        source = Path(work) / "data_reference.c"
        executable = Path(work) / "data_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable)]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's object table (tools/engine_data_reference.py): 2048 objects.\n"
        "# new <slot, -1 when none is made>; delete <slot>; walk <slots in the table's order>\n"
        "# stale <slot> <whether a handle from before its delete is still taken>\n"
        "# refused <slot when the memory pool refuses>; full <objects then made> <objects held>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_data_reference.py")
        print("the data reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
