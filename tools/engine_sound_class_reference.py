"""The game's sound classes (source/sound/sound_classes.c): each class
`sound_class_get` returns, an index out of range among them, and the gains
`sound_class_set_gain` fades and `sound_classes_update` steps, recorded for
the engine's classes to match.

    python3 tools/engine_sound_class_reference.py          # writes the reference
    python3 tools/engine_sound_class_reference.py --check  # fails if it is stale

The table and the functions are sound_classes.c's, compiled for this machine
against the shared fake world (tools/harness/world.py), with a stand-in for
the game state's allocator. The trace is written to
tools/traces/sound_class_reference.txt, which game.wasm's build must
match (tools/engine_wasm_traces.py).
"""

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from harness import enum_with, function, read, structure, world  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "sound_class_reference.txt"

# sound_classes.c's functions, each defined before its first use
FUNCTIONS = [
    "sound_class_index_valid",
    "sound_class_get",
    "sound_classes_initialize",
    "sound_class_datum_get",
    "sound_class_get_gain",
    "sound_classes_initialize_for_new_map",
    "sound_classes_update",
    "sound_class_set_gain",
]

FAKE_WORLD = r"""
typedef float real;
/* the game's boolean is a byte, which the definition's size counts on */
#define boolean byte

#define VALID_INDEX(index, count) ((index) >= 0 && (index) < (count))
#define PIN(x, lower, upper) ((x) < (lower) ? (lower) : ((x) > (upper) ? (upper) : (x)))

/* an error the game logs and goes on past */
enum { _error_silent };
static long errors_said;
#define error(level, ...) (errors_said++)

/* the game state's allocator */
static void *game_state_malloc(char const *name, char const *type, long size)
{
	return calloc(1, size);
}
"""

MAIN = r"""
#define STEPS 400

static unsigned long cases = 0x50c1a55;

static unsigned long next_case(void)
{
	cases = cases*1103515245u + 12345u;
	return cases >> 8;
}

/* the names a script fades: whole names, a part of many, and none */
static char const *const fade_names[] =
{
	"weapon", "music", "ambient", "scripted_dialog", "unit_dialog", "impact",
	"game_event", "device_", "e", "no_such_class",
};

static void print_gains(void)
{
	short index;

	printf("gains");
	for (index = 0; index < NUMBER_OF_SOUND_CLASSES; index++)
		printf(" %.9g", sound_class_get_gain(index));
	printf("\n");
}

int main(void)
{
	short index;
	long step;

	/* each class, and the indices either side of the table, which are
	pinned to its ends (said once a run) */
	for (index = -3; index < NUMBER_OF_SOUND_CLASSES + 3; index++)
	{
		short pinned = sound_class_index_valid(index);
		/* the game asks for no unused class (sound_class_get asserts it has
		a name), so those are read from the table */
		boolean used = sound_class_names[pinned][0] != 0;
		struct sound_class_definition *definition = used ? sound_class_get(index) : &sound_classes[pinned];

		printf(
			"%s %d %d %d %d %d %d %d %d %.9g %.9g %.9g %.9g %.9g %d %s\n",
			used ? "class" : "unused",
			index,
			pinned,
			definition->maximum_number_per_definition,
			definition->maximum_number_per_object,
			definition->preemption_time,
			definition->speech,
			definition->priority,
			definition->cache_miss_mode,
			definition->wet_gain,
			definition->minimum_distance,
			definition->maximum_distance,
			definition->unknown_gain,
			definition->unknown_scale,
			definition->disabled,
			used ? sound_class_names[pinned] : "-");
	}
	printf("errors %d\n", errors_said);

	/* fades set and stepped at random */
	sound_classes_initialize();
	sound_classes_initialize_for_new_map();
	print_gains();
	for (step = 0; step < STEPS; step++)
	{
		if (next_case() % 3 == 0)
		{
			char const *name = fade_names[next_case() % NUMBEROF(fade_names)];
			/* gains either side of 0 to 1, and ticks below 0 */
			real gain = (real)((long)(next_case() % 160) - 30) * 0.01f;
			short ticks = (short)((long)(next_case() % 130) - 10);

			sound_class_set_gain(name, gain, ticks);
			printf("set %s %.9g %d\n", name, gain, ticks);
		}
		else
		{
			long ticks = (long)(next_case() % 12) - 2;

			sound_classes_update(ticks);
			printf("update %d\n", ticks);
		}
		print_gains();
	}
	return 0;
}
"""


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    classes_h = read("source/sound/sound_classes.h")
    classes_c = read("source/sound/sound_classes.c")
    # the class count, the definition macro, the datum, the table and the names
    table = classes_c[classes_c.index("/* ---------- constants") : classes_c.index("/* ---------- private code")]
    return "\n\n".join(
        [
            world.prelude(),
            FAKE_WORLD,
            enum_with(classes_h, "MAXIMUM_SOUND_INSTANCES_PER_DEFINITION"),
            structure(classes_h, "sound_class_definition"),
            table,
            *(function(classes_c, name) for name in FUNCTIONS),
            MAIN,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="sound-class-reference-") as work:
        source = Path(work) / "sound_class_reference.c"
        executable = Path(work) / "sound_class_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's sound classes (tools/engine_sound_class_reference.py).\n"
        "# <class|unused> <index asked> <index given> <per definition> <per object> <preemption ms> <speech> <priority>\n"
        "#   <cache miss mode> <wet gain> <minimum> <maximum> <unknown gain> <unknown scale> <disabled> <name, - if unused>\n"
        "# errors <said>; set <name> <gain> <ticks>; update <ticks>; gains <each class's gain>\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_sound_class_reference.py")
        print("the sound class reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
