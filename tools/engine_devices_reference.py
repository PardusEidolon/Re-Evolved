"""The game's device groups, taken from the sources and run over random
levels and random script calls, to record each group's value and flags and
each device's power, position and flags after every call, as the native
engine's `Devices` must keep them.

    python3 tools/engine_devices_reference.py          # writes the reference
    python3 tools/engine_devices_reference.py --check  # fails if it is stale

The functions are source/devices/devices.c's: the groups a level begins
with (`create_initial_device_groups`), a device's placement
(`device_new`, `device_add_scenario_information`) and deletion
(`device_delete`), and every function the scripts reach the groups and
devices by (`device_group_set_desired_value` and the change it makes,
`device_group_set_actual_value`, `device_group_get_value`,
`device_group_change_only_once_more_set`, `device_set_desired_position`,
`device_set_actual_position`, `device_set_power`, `device_get_position`,
`device_get_power`, and the three machine setters). They are found by name
as tools/harness finds them and compiled for this machine against a fake
world: the groups' data array, sixteen objects iterated in their order
(half of each level's definitions machines), and an effect stand-in that
records the effect each device starts. A machine is placed by
device_machines.c's own `machine_place`, which copies its placed flags. The co-op client check
answers that the devices are this machine's.

The trace is written to tools/traces/devices_reference.txt, which
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
from harness import enum_with, function, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "devices_reference.txt"

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef float real;
typedef unsigned short word;
typedef unsigned char byte;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

typedef struct { real x, y, z; } real_point3d;

struct _object_datum { unsigned long flags; };
enum { _object_shadowless_bit = 4 };
enum { _object_mask_device = 1 };
/* the placement's object part, which machine_place does not read */
struct scenario_object_datum { short type; };

#define match_assert(file, line, condition) ((condition) ? (void)0 : (void)printf("assert %d\n", line))
#define match_vassert(file, line, condition, message) ((condition) ? (void)0 : (void)printf("assert %s\n", message))
"""

WORLD = r"""
/* the groups' data array */
#define MAXIMUM_GROUPS 1024
struct data_array { short maximum_count; };
static struct data_array groups_array = { MAXIMUM_GROUPS };
static struct data_array *device_groups_data = &groups_array;
static struct device_group_datum groups[MAXIMUM_GROUPS];
static boolean group_used[MAXIMUM_GROUPS];

static void data_make_valid(struct data_array *data) { memset(group_used, 0, sizeof group_used); }
static short datum_new(struct data_array *data)
{
	short index;

	for (index = 0; index < MAXIMUM_GROUPS; index++)
	{
		if (!group_used[index])
		{
			group_used[index] = TRUE;
			memset(&groups[index], 0, sizeof groups[index]);
			return index;
		}
	}
	return NONE;
}
static void *datum_get(struct data_array *data, long index)
{
	if (index < 0 || index >= MAXIMUM_GROUPS || !group_used[index])
	{
		printf("assert datum %d\n", index);
		return &groups[0];
	}
	return &groups[index];
}
static void datum_delete(struct data_array *data, long index) { group_used[index] = FALSE; }

/* the scenario's groups */
struct tag_block { long count; void *address; };
struct scenario { struct tag_block device_groups; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])
static struct scenario_device_group scenario_groups[8];
static struct scenario the_scenario = { { 0, scenario_groups } };
static struct scenario *global_scenario_get(void) { return &the_scenario; }

/* the definitions: the first four machines', the rest controls' */
#define DEFINITIONS 8
struct device_definition
{
	struct
	{
		struct { long index; } depowered_effect, repowered_effect;
	} device;
};
static struct device_definition definitions[DEFINITIONS];
static struct device_definition *device_definition_get(long index) { return &definitions[index]; }

/* the objects, iterated in their order */
#define OBJECTS 16
static struct machine_datum objects[OBJECTS];
static boolean exists[OBJECTS];
static struct device_datum *device_get(long index)
{
	if (index < 0 || index >= OBJECTS || !exists[index])
		printf("assert device %d\n", index);
	return (struct device_datum *)&objects[index & (OBJECTS - 1)];
}
static struct machine_datum *machine_try_and_get(long index)
{
	if (index < 0 || index >= OBJECTS || !exists[index] || objects[index].definition_index >= 4)
		return NULL;
	return &objects[index];
}
#define machine_get(index) machine_try_and_get(index)
struct object_iterator { long index; long next; };
static void object_iterator_new(struct object_iterator *iterator, long mask, long flags)
{
	iterator->index = NONE;
	iterator->next = 0;
}
static struct device_datum *object_iterator_next(struct object_iterator *iterator)
{
	while (iterator->next < OBJECTS)
	{
		long index = iterator->next++;

		if (exists[index])
		{
			iterator->index = index;
			return (struct device_datum *)&objects[index];
		}
	}
	return NULL;
}

static boolean network_coop_devices_remote(void) { return FALSE; }
static void network_coop_note_device_snap(short group_index) {}

/* the effects each device starts */
static void device_effect_new(long device_index, long effect_index)
{
	if (effect_index != NONE)
		printf("effect %d %d\n", device_index, effect_index);
}

void device_group_set_actual_value(short group_index, real actual_value);
static boolean device_group_change(short group_index, real desired_value);
static short device_group_new(real initial_value, unsigned long flags);
static void device_group_delete(short group_index);
static void create_initial_device_groups(void);
"""

DRIVER = r"""
static unsigned long seed = 0xde71;

static long next_random(long n)
{
	seed = seed * 1664525u + 1013904223u;
	return (long)((seed >> 8) % (unsigned long)n);
}

static real const values[] = { -0.5f, 0.f, 0.25f, 0.5f, 1.f, 1.5f };

static real next_value(void)
{
	return values[next_random(6)];
}

static unsigned bits(real x)
{
	unsigned u;
	memcpy(&u, &x, sizeof u);
	return u;
}

/* a group in use, or NONE if there is none */
static short next_group(void)
{
	short used[MAXIMUM_GROUPS];
	short count = 0;
	short index;

	for (index = 0; index < MAXIMUM_GROUPS; index++)
	{
		if (group_used[index])
			used[count++] = index;
	}
	return count ? used[next_random(count)] : NONE;
}

/* a device in the world, or NONE if there is none */
static long next_device(void)
{
	long in[OBJECTS];
	long count = 0;
	long index;

	for (index = 0; index < OBJECTS; index++)
	{
		if (exists[index])
			in[count++] = index;
	}
	return count ? in[next_random(count)] : NONE;
}

static void place(long index, long groups_count)
{
	struct scenario_machine_datum scenario_machine;
	struct scenario_device_datum scenario_device;
	long kind = next_random(16);

	memset(&objects[index], 0, sizeof objects[index]);
	exists[index] = TRUE;
	objects[index].definition_index = next_random(DEFINITIONS);
	scenario_device.power_group_index = next_random(groups_count + 1) - 1;
	scenario_device.position_group_index = next_random(groups_count + 1) - 1;
	scenario_device.flags = next_random(32);
	device_new(index);
	if (machine_try_and_get(index) != NULL)
	{
		/* the melee bit is left to the damage system's trace */
		scenario_machine.device = scenario_device;
		scenario_machine.flags = kind & 7;
		machine_place(index, &scenario_machine);
	}
	else
		device_add_scenario_information(index, &scenario_device);
	printf("place %d %d %d %d %d %d\n", index, objects[index].definition_index,
		scenario_device.power_group_index, scenario_device.position_group_index,
		scenario_device.flags, kind);
}

static void state(void)
{
	long index;

	printf("groups");
	for (index = 0; index < MAXIMUM_GROUPS; index++)
	{
		if (group_used[index])
			printf(" %d:%08x:%d", index, bits(groups[index].actual_value), groups[index].flags);
	}
	printf("\ndevices");
	for (index = 0; index < OBJECTS; index++)
	{
		if (exists[index])
		{
			struct _device_datum *device = &objects[index].device;

			printf(" %d:%08x:%08x:%08x:%08x:%d:%d:%d:%d", index,
				bits(device->power), bits(device->power_velocity),
				bits(device->position), bits(device->position_velocity),
				device->power_group_index, device->position_group_index,
				device->flags & (FLAG(_device_position_reversed_bit) | FLAG(_device_not_usable_bit)),
				objects[index].machine.flags);
		}
	}
	printf("\n");
}

int main(void)
{
	long level;

	for (level = 0; level < 300; level++)
	{
		long groups_count = next_random(7);
		long devices_count = 1 + next_random(10);
		long index;
		long call;

		for (index = 0; index < DEFINITIONS; index++)
		{
			definitions[index].device.depowered_effect.index = next_random(3) ? 100 + index : NONE;
			definitions[index].device.repowered_effect.index = next_random(3) ? 200 + index : NONE;
		}
		memset(objects, 0, sizeof objects);
		memset(exists, 0, sizeof exists);
		the_scenario.device_groups.count = groups_count;
		for (index = 0; index < groups_count; index++)
		{
			scenario_groups[index].initial_value = next_value() < 0.f ? 0.f : values[1 + next_random(4)];
			scenario_groups[index].flags = next_random(4);
		}
		printf("level %d", groups_count);
		for (index = 0; index < DEFINITIONS; index++)
		{
			printf(" %d:%d", definitions[index].device.depowered_effect.index,
				definitions[index].device.repowered_effect.index);
		}
		for (index = 0; index < groups_count; index++)
		{
			printf(" %08x:%d", bits(scenario_groups[index].initial_value), scenario_groups[index].flags);
		}
		printf("\n");
		devices_initialize_for_new_map();
		for (index = 0; index < devices_count; index++)
		{
			long at = next_random(OBJECTS);

			/* (each placement a new object) */
			if (!exists[at])
				place(at, groups_count);
		}
		state();
		for (call = 0; call < 40; call++)
		{
			long kind = next_random(14);
			short group = next_group();
			long device = next_device();
			real value = next_value();
			boolean flag = next_random(2);

			if (device == NONE && kind >= 4 && kind != 13)
				kind = 13;
			if (group == NONE && kind < 4)
				kind = 13;
			switch (kind)
			{
			case 0:
				printf("call group_set %d %08x %d\n", group, bits(value), device_group_set_desired_value(group, value));
				break;
			case 1:
				device_group_set_actual_value(group, value);
				printf("call group_set_immediate %d %08x\n", group, bits(value));
				break;
			case 2:
				printf("call group_get %d %08x\n", group, bits(device_group_get_value(group)));
				break;
			case 3:
			{
				short target = next_random(4) ? group : NONE;

				device_group_change_only_once_more_set(target, flag);
				printf("call group_once %d %d\n", target, flag);
			}
				break;
			case 4:
			case 5:
				printf("call set_position %d %08x %d\n", device, bits(value), device_set_desired_position(device, value));
				break;
			case 6:
				device_set_actual_position(device, value);
				printf("call set_position_immediate %d %08x\n", device, bits(value));
				break;
			case 7:
				device_set_power(device, value);
				printf("call set_power %d %08x\n", device, bits(value));
				break;
			case 8:
				printf("call get %d %08x %08x\n", device, bits(device_get_position(device)), bits(device_get_power(device)));
				break;
			case 9:
				device_one_sided_set(device, flag);
				printf("call one_sided %d %d\n", device, flag);
				break;
			case 10:
				device_operates_automatically_set(device, flag);
				printf("call operates_automatically %d %d\n", device, flag);
				break;
			case 11:
				device_set_never_appears_locked(device, flag);
				printf("call never_appears_locked %d %d\n", device, flag);
				break;
			case 12:
				device_delete(device);
				exists[device] = FALSE;
				printf("call delete %d\n", device);
				break;
			default:
				index = next_random(OBJECTS);
				if (exists[index])
				{
					device_delete(index);
					exists[index] = FALSE;
				}
				place(index, groups_count);
				break;
			}
			state();
		}
	}
	return 0;
}
"""

FUNCTIONS = [
    "devices_initialize_for_new_map",
    "device_delete",
    "device_new",
    "device_get_position",
    "device_get_power",
    "device_set_never_appears_locked",
    "device_group_set_actual_value",
    "device_group_set_desired_value",
    "device_group_change",
    "device_one_sided_set",
    "device_group_get_value",
    "device_group_change_only_once_more_set",
    "device_operates_automatically_set",
    "device_set_actual_position",
    "device_set_desired_position",
    "device_set_power",
    "device_add_scenario_information",
    "device_group_new",
    "create_initial_device_groups",
    "device_group_delete",
]


def source_file():
    """The game's code and the fake world it runs in, as one C file."""
    devices_h = read("source/devices/devices.h")
    devices = read("source/devices/devices.c")
    machines_h = read("source/devices/device_machines.h")
    machines = read("source/devices/device_machines.c")
    return "\n".join(
        [
            FAKE_WORLD,
            enum_with(devices_h, "_scenario_device_initially_open_bit"),
            enum_with(devices_h, "_machine_one_sided_bit"),
            enum_with(devices, "_device_group_runtime_bit"),
            structure(devices_h, "_device_datum"),
            structure(devices_h, "scenario_device_datum"),
            structure(devices_h, "device_datum"),
            structure(devices_h, "_machine_datum"),
            structure(devices_h, "machine_datum"),
            structure(machines_h, "scenario_machine_datum"),
            structure(devices, "device_group_datum"),
            structure(devices, "scenario_device_group"),
            WORLD,
            *[function(devices, name) for name in FUNCTIONS],
            function(machines, "machine_place"),
            DRIVER,
        ]
    )


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="devices-reference-") as work:
        source = Path(work) / "devices_reference.c"
        executable = Path(work) / "devices_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    if "assert" in output:
        raise SystemExit("the game's code asserted")
    return (
        "# The game's device groups (tools/engine_devices_reference.py).\n"
        "# level <scenario groups> <each definition's depowered:repowered effect> <each group's value:flags>\n"
        "# place <object> <definition> <power group> <position group> <placement flags> <machine flags>\n"
        "# call <function> <its arguments> <what it returned>; effect <object> <effect>\n"
        "# groups <index:value:flags>...; devices <object:power:velocity:position:velocity:"
        "power group:position group:flags:machine flags>... (reals as f32 bits)\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_devices_reference.py")
        print("the devices reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
