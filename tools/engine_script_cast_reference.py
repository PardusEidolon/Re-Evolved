"""The game's script casts (source/hs/hs_runtime.c): which types `hs_can_cast`
lets a value be taken as, and what `hs_cast` makes of each value, which is
what `hs_return` hands back to the expression that called a function or a
script, recorded for the engine's casts to match.

    python3 tools/engine_script_cast_reference.py          # writes the reference
    python3 tools/engine_script_cast_reference.py --check  # fails if it is stale

The casting procedures, their table, `hs_object_type_can_cast`,
`hs_can_cast` and `hs_cast` are hs_runtime.c's, with hs.c's object type
masks, compiled for this machine against the shared fake world
(tools/harness/world.py), with stand-ins for the object lists and the
scenario's object names. A string is a long in the game (a 32-bit pointer),
so here it is an index into a small table of strings. The trace is written
to tools/traces/script_cast_reference.txt, which
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
from harness import enum_with, function, read, world  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "script_cast_reference.txt"

# hs_runtime.c's casting procedures, each defined before the table
PROCEDURES = [
    "hs_long_to_boolean",
    "hs_short_to_boolean",
    "hs_string_to_boolean",
    "hs_data_to_void",
    "hs_short_to_real",
    "hs_long_to_real",
    "hs_enum_to_real",
    "hs_real_to_short",
    "hs_real_to_long",
    "hs_long_to_short",
    "hs_object_name_to_object_list",
    "hs_object_to_object_list",
]

FAKE_WORLD = r"""
typedef float real;
typedef long (*hs_typecasting_procedure)(long value);

/* a string is an index into these, where the game's long is its pointer */
static char const *const strings[] = { "", "hello", "0" };
#define csstrlen(p) ((long)strlen(strings[(long)(p)]))

/* the scenario's object names 0 and 1 name objects 50 and 51; the rest none */
static long object_index_from_name_index(short name_index)
{
	return name_index >= 0 && name_index < 2 ? 50 + name_index : NONE;
}

/* the object lists, numbered from 100, each with its objects */
static long lists_made = 0;
static long list_objects[64][4];
static long list_counts[64];

static long object_list_new(void)
{
	long list = lists_made++;

	list_counts[list] = 0;
	return 100 + list;
}

static void object_list_add(long list_index, long object_index)
{
	long list = list_index - 100;

	list_objects[list][list_counts[list]++] = object_index;
}

/* an encounter's actors: encounter 0 has none, the rest two units each */
long object_list_from_ai_reference(long ai_reference)
{
	long list;

	if (ai_reference == 0)
		return NONE;
	list = object_list_new();
	object_list_add(list, 70 + ai_reference);
	object_list_add(list, 80 + ai_reference);
	return list;
}

/* a thread is named only when an assertion fails, which prints first */
static char const *hs_thread_format(long thread_index)
{
	return "";
}
"""

MAIN = r"""
static long real_bits(real r)
{
	long bits;

	memcpy(&bits, &r, sizeof(bits));
	return bits;
}

/* the values of each type a cast is tried on; *count of them */
static long const *values_of(short type, long *count)
{
	static long reals[8];
	static long const booleans[] = { 0, 1 };
	static long const shorts[] = { -32768, -1, 0, 1, 7, 32767 };
	static long const longs[] = { -2147483647 - 1, -70000, -1, 0, 1, 65537, 2147483647 };
	static long const strings_[] = { 0, 1, 2 };
	static long const indices[] = { -1, 0, 1, 3 };
	static long const objects[] = { -1, 5 };
	static long const names[] = { -1, 0, 1, 2 };
	static long const ais[] = { 0, 3 };

	if (type == _hs_type_real)
	{
		real const r[] = { -31999.75f, -2.5f, -0.5f, 0.0f, 0.75f, 3.9f, 40.5f, 31999.0f };
		long i;

		for (i = 0; i < NUMBEROF(r); i++)
			reals[i] = real_bits(r[i]);
		*count = NUMBEROF(r);
		return reals;
	}
	switch (type)
	{
	case _hs_type_boolean: *count = NUMBEROF(booleans); return booleans;
	case _hs_type_short_integer: *count = NUMBEROF(shorts); return shorts;
	case _hs_type_long_integer: *count = NUMBEROF(longs); return longs;
	case _hs_type_string: *count = NUMBEROF(strings_); return strings_;
	case _hs_type_ai: *count = NUMBEROF(ais); return ais;
	}
	if (HS_TYPE_IS_OBJECT(type))
	{
		*count = NUMBEROF(objects);
		return objects;
	}
	if (HS_TYPE_IS_OBJECT_NAME(type))
	{
		*count = NUMBEROF(names);
		return names;
	}
	*count = NUMBEROF(indices);
	return indices;
}

int main(void)
{
	short actual;
	short desired;

	for (actual = _hs_passthrough; actual < NUMBER_OF_HS_TYPES; actual++)
	{
		for (desired = _hs_type_void; desired < NUMBER_OF_HS_TYPES; desired++)
		{
			long count;
			long const *values;
			long i;

			if (!hs_can_cast(actual, desired))
				continue;
			printf("can %d %d\n", actual, desired);
			/* a passthrough is whatever its value already is */
			if (actual == _hs_passthrough)
				continue;
			values = values_of(actual, &count);
			for (i = 0; i < count; i++)
			{
				long before = lists_made;
				long result = hs_cast(0, actual, desired, values[i]);

				printf("cast %d %d %d %d", actual, desired, values[i], result);
				if (lists_made != before)
				{
					long list = result - 100;
					long j;

					printf(" list");
					for (j = 0; j < list_counts[list]; j++)
						printf(" %d", list_objects[list][j]);
				}
				printf("\n");
			}
		}
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    hs_h = read("source/hs/hs.h")
    hs_c = read("source/hs/hs.c")
    runtime_c = read("source/hs/hs_runtime.c")
    macros = hs_h[hs_h.index("#define hs_type_valid") : hs_h.index("/* ---------- structures")]
    masks = function_free_array(hs_c, "word const hs_object_type_masks")
    table = runtime_c[
        runtime_c.index("hs_typecasting_procedure typecasting_procedures") : runtime_c.index(
            "static struct hs_runtime_globals hs_runtime_globals;"
        )
    ]
    match_hs_assert = runtime_c[
        runtime_c.index("#define match_hs_assert") : runtime_c.index("/* the four ordered comparisons")
    ]
    return "\n\n".join(
        [
            world.prelude(),
            FAKE_WORLD,
            enum_with(hs_h, "_hs_unparsed"),
            enum_with(hs_h, "NUMBER_OF_HS_OBJECT_TYPES"),
            macros,
            masks,
            match_hs_assert,
            *(function(runtime_c, name) for name in PROCEDURES),
            table,
            function(runtime_c, "hs_object_type_can_cast"),
            function(runtime_c, "hs_can_cast"),
            function(runtime_c, "hs_cast"),
            MAIN,
        ]
    )


def function_free_array(source, declaration):
    """A table's definition, from its declaration to its closing brace."""
    start = source.index(declaration)
    return source[start : source.index("};", start) + 2]


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="script-cast-reference-") as work:
        source = Path(work) / "script_cast_reference.c"
        executable = Path(work) / "script_cast_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's script casts (tools/engine_script_cast_reference.py).\n"
        "# can <actual type> <desired type>: hs_can_cast lets it\n"
        "# cast <actual type> <desired type> <value as a long> <hs_cast's long> [list <its objects>]\n"
        "# (a real is its bits; a string is an index into \"\", \"hello\", \"0\")\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_script_cast_reference.py")
        print("the script cast reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
