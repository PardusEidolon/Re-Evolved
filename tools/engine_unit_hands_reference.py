"""The game's unit readying the weapon it wants, dropping an item, making a
grenade in its hand and spawning actors, taken from the sources and run over
random units, to record what it calls on the world's objects, what it draws
from the game's seed and what it leaves in the unit, as the native engine's
must.

    python3 tools/engine_unit_hands_reference.py          # writes the reference
    python3 tools/engine_unit_hands_reference.py --check  # fails if it is stale

The functions are source/units/units.c's `unit_ready_desired_weapon`,
`unit_adjust_for_seat_change`, `unit_drop_item`,
`unit_throw_grenade_move_to_hand` and `unit_test_spawning`, with the
helpers they call (`unit_weapon_next_index`, `unit_can_use_weapon`,
`unit_inventory_get_weapon` and `unit_get_weapon`, and real_math's
`perpendicular3d` and `add_vectors3d`), found by name as tools/harness finds
them. The game's seed and its helpers (`random_range`, `real_random_range`,
`random_vector_in_cone3d`, over the geosphere's direction table) are
tools/engine_random_reference.py's, so that what is drawn is the game's.
They are compiled for this machine against a fake world of sixteen objects:
the unit (0), and weapons riding a parent or not, put away, used by the
unit's seat, allowed by the game engine and to be readied or not.

What the functions ask of the rest of the game is answered from that world,
and what they do to it (detach, disconnect, wake, show or hide, carry or
let go, label, hold, ready, unzoom, throw, push inside the bsp, delete,
make a grenade, spawn actors) is written down in order.

Asserts are compiled out, as in the released game.

The trace is written to tools/traces/unit_hands_reference.txt, which
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
import engine_random_reference as random_reference  # noqa: E402
from harness import enum_with, function, inline, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "unit_hands_reference.txt"

# the random trace's fake world, with a game engine that may be running
SEEDED_WORLD = r"""
/* string.h can't be read after the math world makes long an int */
static void clear(void *data, unsigned size)
{
	char *byte = data;
	while (size--)
		*byte++ = 0;
}
typedef unsigned short word;
/* the C library has a random() of its own */
#define random game_random
#define match_dassert(file, line, condition, message) ((void)0)

static boolean engine_running;
static boolean game_engine_running(void)
{
	return engine_running;
}
"""

WORLD = r"""
#define match_assert(file, line, condition) ((void)0);
#define match_vassert(file, line, condition, message) ((void)0)
#define FLAG(bit) (1u << (bit))
#define TEST_FLAG(flags, bit) (((flags) & FLAG(bit)) != 0)
#define SET_FLAG(flags, bit, value) ((value) ? ((flags) |= FLAG(bit)) : ((flags) &= ~FLAG(bit)))

/* a real as its bits, for the calls the functions make */
static unsigned bits(real x)
{
	union { real r; unsigned u; } value;
	value.r = x;
	return value.u;
}

#define MAXIMUM_WEAPONS_PER_UNIT 4
#define NUMBER_OF_UNIT_GRENADE_TYPES 2
#define SEATS 3
#define WEAPON_CLASSES 3

static real_vector3d const global_zero_vector3d_value = { 0.f, 0.f, 0.f };
#define global_zero_vector3d (&global_zero_vector3d_value)

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) \
	((index) >= 0 && (index) < (block)->count ? (type *)(block)->address + (index) : (type *)NULL)

struct object_datum
{
	long definition_index;
	struct
	{
		long parent_object_index;
		real_vector3d translational_velocity;
		real_vector3d angular_velocity;
	} object;
	struct { long ignore_object_index; } item;
	/* the world's answers about it */
	boolean puts_away, seat_allows, engine_allows, must_be_readied, inside_bsp;
	char label[8];
};
#define weapon_datum object_datum
#define item_datum object_datum

struct unit_datum
{
	long definition_index;
	struct
	{
		long player_index;
		long actor_index;
		unsigned long flags;
		short current_weapon_index;
		short desired_weapon_index;
		long weapon_object_indices[MAXIMUM_WEAPONS_PER_UNIT];
		long weapon_last_used_at_game_time[MAXIMUM_WEAPONS_PER_UNIT];
		char current_grenade_index;
		char grenade_counts[NUMBER_OF_UNIT_GRENADE_TYPES];
		char current_zoom_level;
		char desired_zoom_level;
		long grenade_object_index;
		char grenade_throw_state;
		real_vector3d aiming_vector;
		struct { short seat_index; short weapon_index; } animation;
	} unit;
};

struct unit_definition
{
	struct { struct { long index; } animation_graph; } object;
	struct
	{
		struct { long index; } spawned_actor_variant;
		short spawn_actor_lower_bound;
		short spawn_actor_upper_bound;
		real spawn_throw_velocity;
	} unit;
};
struct animation_graph_weapon_class { char hand_marker_name[8]; char grip_marker_name[8]; };
struct animation_graph_unit_seat { struct tag_block weapon_classes; };
struct animation_graph { struct tag_block unit_seats; };
struct game_globals_grenade { struct { long index; } projectile; };
struct game_globals { struct tag_block grenades; };
struct object_marker { short node_index; real_matrix4x3 matrix; };
struct object_placement_data
{
	long definition_index;
	unsigned long flags;
	real_vector3d forward;
	real_vector3d up;
	real_point3d position;
};

#define MAXIMUM_OBJECTS 16
static struct object_datum objects[MAXIMUM_OBJECTS];
static struct unit_datum the_unit;
static struct unit_definition the_unit_definition;
static struct animation_graph_weapon_class weapon_classes[SEATS][WEAPON_CLASSES];
static struct animation_graph_unit_seat unit_seats[SEATS];
static struct animation_graph the_graph;
static struct game_globals_grenade grenades[NUMBER_OF_UNIT_GRENADE_TYPES];
static struct game_globals the_globals;
static struct { boolean infinite_ammo; } cheat;
static boolean engine_infinite_grenades, actor_unlimited_grenades, distributed_client;
static long game_time;
static real_vector3d unit_velocity;
static real_point3d camera_position;
static struct object_marker left_hand;
static long new_grenade;
static short spawn_cap;

static struct unit_datum *unit_get(long unit_index) { return &the_unit; }
static struct unit_definition *unit_definition_get(long index) { return &the_unit_definition; }
static struct object_datum *weapon_get(long index) { return &objects[index]; }
static struct object_datum *item_get(long index) { return &objects[index]; }
static struct animation_graph *animation_graph_definition_get(long index) { return &the_graph; }
static struct game_globals *scenario_get_game_globals(void) { return &the_globals; }
static long game_time_get(void) { return game_time; }
static boolean game_engine_infinite_grenades(long player_index) { return engine_infinite_grenades; }
static boolean actor_has_unlimited_grenades(long actor_index) { return actor_unlimited_grenades; }
static boolean network_game_distributed_client(void) { return distributed_client; }

static char const *unit_get_seat_label(long unit_index) { return "stand"; }
static long last_weapon_asked;
/* each weapon's label is its own, and the seat is asked about the weapon
whose label it was given */
static char const *weapon_get_label(long weapon_index)
{
	last_weapon_asked = weapon_index;
	return objects[weapon_index].label;
}
static char const *base_weapon_label_get(short index) { last_weapon_asked = NONE; return "unarmed"; }
/* set, the unit's animation is the seat's weapon class of the weapon (its
index, by three), unarmed's the first */
static boolean unit_set_or_test_seat_and_weapon_label(long unit_index, char const *seat, char const *weapon, boolean set)
{
	if (!set)
		return objects[last_weapon_asked].seat_allows;
	printf(" label %s", weapon);
	the_unit.unit.animation.weapon_index = last_weapon_asked == NONE ? 0 : (short)(last_weapon_asked % WEAPON_CLASSES);
	return TRUE;
}
static boolean game_engine_allow_pick_up(long unit_index, long weapon_index) { return objects[weapon_index].engine_allows; }
static boolean weapon_must_be_readied(long weapon_index) { return objects[weapon_index].must_be_readied; }

static boolean weapon_put_away(long index, boolean immediate)
{
	printf(" put_away %d %d", index, immediate);
	return objects[index].puts_away;
}
static void object_detach(long index) { printf(" detach %d", index); }
static void object_disconnect_from_map(long index) { printf(" disconnect %d", index); }
static void object_activate(long index) { printf(" activate %d", index); }
static void object_reconnect_to_map(long index, void *location) { printf(" reconnect %d", index); }
static void object_set_visibility(long index, boolean visible) { printf(" visible %d %d", index, visible); }
static void item_in_unit_inventory(long index, long unit_index) { printf(" carried %d %d", index, unit_index); }
static void object_attach_to_marker(long parent, char const *marker, long index, char const *grip)
{
	printf(" attach %d %s:%s", index, marker, grip);
}
static void weapon_ready(long index) { printf(" ready %d", index); }
static void unit_unzoom(long unit_index)
{
	printf(" unzoom");
	the_unit.unit.current_zoom_level = NONE;
	the_unit.unit.desired_zoom_level = NONE;
}
static void object_delete(long index) { printf(" delete %d", index); }
static void object_get_velocities(long index, real_vector3d *translational, real_vector3d *angular)
{
	*translational = unit_velocity;
}
static void unit_get_camera_position(long unit_index, real_point3d *position) { *position = camera_position; }
/* the item is thrown still, ignoring the unit */
static void item_accelerate(long index, real_vector3d const *velocity, boolean unused)
{
	struct object_datum *item = &objects[index];
	printf(" throw %d %d %d %08x %08x %08x", index, item->item.ignore_object_index,
		item->object.translational_velocity.i == 0.f && item->object.translational_velocity.j == 0.f &&
			item->object.translational_velocity.k == 0.f && item->object.angular_velocity.i == 0.f &&
			item->object.angular_velocity.j == 0.f && item->object.angular_velocity.k == 0.f,
		bits(velocity->i), bits(velocity->j), bits(velocity->k));
}
static boolean object_force_inside_bsp(long index, real_point3d const *position)
{
	printf(" bsp %d %08x %08x %08x", index, bits(position->x), bits(position->y), bits(position->z));
	return objects[index].inside_bsp;
}
static void object_get_marker_by_name(long index, char const *name, struct object_marker *marker, long count)
{
	*marker = left_hand;
}
static void object_placement_data_new(struct object_placement_data *data, long definition_index, long owner)
{
	clear(data, sizeof *data);
	data->definition_index = definition_index;
	data->up.k = 1.f;
}
/* the grenade drawn for this throw, at the place the game's would be */
static long object_new(struct object_placement_data *data)
{
	printf(" grenade %d %u %08x %08x %08x %08x %08x %08x %08x %08x %08x %d", data->definition_index,
		data->flags, bits(data->position.x), bits(data->position.y), bits(data->position.z),
		bits(data->forward.i), bits(data->forward.j), bits(data->forward.k), bits(data->up.i), bits(data->up.j),
		bits(data->up.k), new_grenade);
	return new_grenade;
}
static void object_attach_to_node(long parent, long index, short node) { printf(" node %d %d", index, node); }
static void network_damage_note_grenade(long unit_index, long kind) { printf(" note %d", kind); }
/* as many as asked are spawned, up to the drawn cap */
static short actors_spawn_from_unit(long unit_index, long variant, short count, real velocity)
{
	short spawned = count < spawn_cap ? count : spawn_cap;
	printf(" spawn %d %d %08x %d", variant, count, bits(velocity), spawned);
	return spawned;
}

static short unit_weapon_next_index(long unit_index, short current_index, short delta);
static long unit_get_weapon(struct unit_datum *unit, short index);
boolean unit_can_use_weapon(long unit_index, long weapon_index);
static void unit_ready_desired_weapon(long unit_index, boolean immediate);
#define unit_get_current_weapon_index(unit_index) unit_inventory_get_weapon((unit_index), unit_get((unit_index))->unit.current_weapon_index)
#define unit_get_desired_weapon_index(unit_index) unit_inventory_get_weapon((unit_index), unit_get((unit_index))->unit.desired_weapon_index)
"""

DRIVER = r"""
static unsigned long cases = 0x4a2d;

static long next_case(long n)
{
	cases = cases * 1103515245u + 12345u;
	return (long)((cases >> 8) % (unsigned long)n);
}

/* a component at random: often a whole number, sometimes zero */
static real next_component(void)
{
	long kind = next_case(4);
	return kind == 0 ? 0.f : kind == 1 ? (real)(next_case(3) - 1) : (real)(next_case(2001) - 1000) * 0.001f;
}

static real_vector3d next_vector(void)
{
	real_vector3d v;
	v.i = next_component();
	v.j = next_component();
	v.k = next_component();
	return v;
}

static void print_vector(real_vector3d const *v)
{
	printf(" %.9g %.9g %.9g", v->i, v->j, v->k);
}
"""

DRIVER_MAIN = r"""
/* objects 1 to 15 at random, each printed as
parent:puts away:seat:engine allows:must be readied:inside bsp */
static void random_world(void)
{
	long index;
	clear(objects, sizeof objects);
	for (index = 1; index < MAXIMUM_OBJECTS; index++)
	{
		struct object_datum *object = &objects[index];
		object->definition_index = index;
		object->object.parent_object_index = next_case(3) == 0 ? 0 : NONE;
		object->object.translational_velocity = next_vector();
		object->object.angular_velocity = next_vector();
		object->item.ignore_object_index = NONE;
		object->puts_away = next_case(4) != 0;
		object->seat_allows = next_case(5) != 0;
		object->engine_allows = next_case(6) != 0;
		object->must_be_readied = next_case(6) == 0;
		object->inside_bsp = next_case(4) != 0;
		sprintf(object->label, "w%d", index);
		printf(" %d:%d:%d:%d:%d:%d", object->object.parent_object_index, object->puts_away,
			object->seat_allows, object->engine_allows, object->must_be_readied, object->inside_bsp);
	}
}

/* the unit at random, printed as
current desired <slot:last used>x4 grenade <grenades>x2 zoom desired_zoom
player actor flags seat <aim> */
static void random_unit(void)
{
	long index;
	clear(&the_unit, sizeof the_unit);
	the_unit.unit.current_weapon_index = (short)(next_case(5) - 1);
	the_unit.unit.desired_weapon_index = (short)(next_case(5) - 1);
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
	{
		the_unit.unit.weapon_object_indices[index] = next_case(3) == 0 ? NONE : 1 + next_case(15);
		the_unit.unit.weapon_last_used_at_game_time[index] = next_case(4) * 30;
	}
	the_unit.unit.current_grenade_index = (char)next_case(NUMBER_OF_UNIT_GRENADE_TYPES);
	for (index = 0; index < NUMBER_OF_UNIT_GRENADE_TYPES; index++)
	{
		long kind = next_case(6);
		the_unit.unit.grenade_counts[index] = (char)(kind == 0 ? -128 + next_case(2) : next_case(4));
	}
	the_unit.unit.current_zoom_level = (char)(next_case(3) - 1);
	the_unit.unit.desired_zoom_level = (char)(next_case(3) - 1);
	the_unit.unit.player_index = next_case(2) == 0 ? NONE : 2;
	the_unit.unit.actor_index = next_case(2) == 0 ? NONE : 5;
	the_unit.unit.flags = (next_case(4) == 0 ? FLAG(_unit_doesnt_drop_items_bit) : 0) |
		(next_case(4) == 0 ? FLAG(_unit_spawned_actors_bit) : 0);
	the_unit.unit.grenade_object_index = NONE;
	the_unit.unit.animation.seat_index = (short)next_case(SEATS);
	the_unit.unit.aiming_vector = next_vector();
	printf(" %d %d", the_unit.unit.current_weapon_index, the_unit.unit.desired_weapon_index);
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		printf(" %d:%d", the_unit.unit.weapon_object_indices[index], the_unit.unit.weapon_last_used_at_game_time[index]);
	printf(" %d %d %d %d %d %d %d %u %d", the_unit.unit.current_grenade_index, the_unit.unit.grenade_counts[0],
		the_unit.unit.grenade_counts[1], the_unit.unit.current_zoom_level, the_unit.unit.desired_zoom_level,
		the_unit.unit.player_index, the_unit.unit.actor_index, the_unit.unit.flags,
		the_unit.unit.animation.seat_index);
	print_vector(&the_unit.unit.aiming_vector);
}

/* the rest of the game, printed as
time running cheat engine_infinite actor_unlimited client <unit velocity>
<camera> <hand> node projectile0 projectile1 new_grenade variant lower upper
velocity cap seed */
static void random_game(void)
{
	game_time = 30 * next_case(10);
	engine_running = next_case(3) == 0;
	cheat.infinite_ammo = next_case(6) == 0;
	engine_infinite_grenades = next_case(4) == 0;
	actor_unlimited_grenades = next_case(4) == 0;
	distributed_client = next_case(3) == 0;
	unit_velocity = next_vector();
	{
		real_vector3d camera = next_vector();
		camera_position.x = camera.i;
		camera_position.y = camera.j;
		camera_position.z = camera.k;
	}
	{
		real_vector3d hand = next_vector();
		left_hand.matrix.position.x = hand.i;
		left_hand.matrix.position.y = hand.j;
		left_hand.matrix.position.z = hand.k;
		left_hand.node_index = (short)next_case(20);
	}
	grenades[0].projectile.index = next_case(5) == 0 ? NONE : 60;
	grenades[1].projectile.index = next_case(5) == 0 ? NONE : 61;
	the_globals.grenades.count = NUMBER_OF_UNIT_GRENADE_TYPES;
	the_globals.grenades.address = grenades;
	new_grenade = next_case(4) == 0 ? NONE : 1 + next_case(15);
	the_unit_definition.unit.spawned_actor_variant.index = next_case(3) == 0 ? NONE : 70;
	the_unit_definition.unit.spawn_actor_lower_bound = (short)(next_case(5) - 1);
	the_unit_definition.unit.spawn_actor_upper_bound =
		next_case(10) == 0 ? 32767 : the_unit_definition.unit.spawn_actor_lower_bound + (short)(next_case(5) - 1);
	the_unit_definition.unit.spawn_throw_velocity = (real)next_case(200) * 0.1f;
	spawn_cap = (short)next_case(5);
	set_random_seed(next_case(1 << 24) * 61 + 1);
	printf(" | %d %d %d %d %d %d", game_time, engine_running, cheat.infinite_ammo, engine_infinite_grenades,
		actor_unlimited_grenades, distributed_client);
	print_vector(&unit_velocity);
	printf(" %.9g %.9g %.9g", camera_position.x, camera_position.y, camera_position.z);
	printf(" %.9g %.9g %.9g %d", left_hand.matrix.position.x, left_hand.matrix.position.y,
		left_hand.matrix.position.z, left_hand.node_index);
	printf(" %d %d %d %d %d %d %.9g %d %u |", grenades[0].projectile.index, grenades[1].projectile.index,
		new_grenade, the_unit_definition.unit.spawned_actor_variant.index,
		the_unit_definition.unit.spawn_actor_lower_bound, the_unit_definition.unit.spawn_actor_upper_bound,
		the_unit_definition.unit.spawn_throw_velocity, spawn_cap, get_random_seed());
}

static void print_unit(void)
{
	long index;
	printf(" %d %d", the_unit.unit.current_weapon_index, the_unit.unit.desired_weapon_index);
	for (index = 0; index < MAXIMUM_WEAPONS_PER_UNIT; index++)
		printf(" %d:%d", the_unit.unit.weapon_object_indices[index], the_unit.unit.weapon_last_used_at_game_time[index]);
	printf(" %d %d %d %d %d %u %d %u\n", the_unit.unit.grenade_counts[0], the_unit.unit.grenade_counts[1],
		the_unit.unit.current_zoom_level, the_unit.unit.desired_zoom_level, the_unit.unit.grenade_object_index,
		the_unit.unit.flags, the_unit.unit.grenade_throw_state, get_random_seed());
}

static void begin(char const *name, long argument)
{
	printf("%s %d |", name, argument);
	random_unit();
	printf(" |");
	random_world();
	random_game();
}

int main(void)
{
	struct geosphere *sphere = geosphere_new(16);
	long run;
	long seat, weapon_class;

	random_math_globals.random_direction_table = (real_vector3d *)sphere->vertices;
	random_math_globals.random_direction_table_size = sphere->vertex_count;
	for (seat = 0; seat < SEATS; seat++)
	{
		for (weapon_class = 0; weapon_class < WEAPON_CLASSES; weapon_class++)
		{
			sprintf(weapon_classes[seat][weapon_class].hand_marker_name, "hand%d%d", seat, weapon_class);
			sprintf(weapon_classes[seat][weapon_class].grip_marker_name, "grip%d%d", seat, weapon_class);
		}
		unit_seats[seat].weapon_classes.count = WEAPON_CLASSES;
		unit_seats[seat].weapon_classes.address = weapon_classes[seat];
	}
	the_graph.unit_seats.count = SEATS;
	the_graph.unit_seats.address = unit_seats;

	for (run = 0; run < 10000; run++)
	{
		long which = next_case(5);
		long item = 1 + next_case(15);
		long argument = next_case(2);
		long answer = 0;
		switch (which)
		{
		case 0:
			begin("ready_desired_weapon", argument);
			unit_ready_desired_weapon(0, (boolean)argument);
			break;
		case 1:
			begin("adjust_for_seat_change", 0);
			unit_adjust_for_seat_change(0);
			break;
		case 2:
			begin("drop_item", item);
			unit_drop_item(0, item);
			break;
		case 3:
			begin("throw_grenade_move_to_hand", 0);
			unit_throw_grenade_move_to_hand(0);
			break;
		default:
			begin("test_spawning", 0);
			answer = unit_test_spawning(0);
			break;
		}
		printf(" -> %d |", answer);
		print_unit();
	}
	return 0;
}
"""

ENUMS = [
    ("source/objects/objects.h", "_new_object_never_automatically_delete_bit"),
    ("source/units/units.h", "_unit_doesnt_drop_items_bit"),
    ("source/units/units.h", "_unit_grenade_throw_idle"),
    ("source/units/units.h", "_unit_base_weapon_none"),
]

FUNCTIONS = [
    "unit_get_weapon",
    "unit_inventory_get_weapon",
    "unit_can_use_weapon",
    "unit_weapon_next_index",
    "unit_ready_desired_weapon",
    "unit_adjust_for_seat_change",
    "unit_drop_item",
    "unit_throw_grenade_move_to_hand",
    "unit_test_spawning",
]


def source_file():
    """The game's seed, its code and the fake world it runs in, as one C file."""
    files = {}

    def text(path):
        if path not in files:
            files[path] = read(path)
        return files[path]

    enums = []
    for path, member in ENUMS:
        enum = enum_with(text(path), member)
        if enum not in enums:
            enums.append(enum)
    math = [
        inline(text("source/math/real_math.h"), "add_vectors3d"),
        function(text("source/math/real_math.c"), "perpendicular3d"),
    ]
    units = text("source/units/units.c")
    functions = [function(units, name) for name in FUNCTIONS]
    seeded = random_reference.seeded_source(SEEDED_WORLD)
    return "\n".join([seeded, *math, *enums, WORLD, *functions, DRIVER, DRIVER_MAIN])


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="unit-hands-reference-") as work:
        source = Path(work) / "unit_hands_reference.c"
        executable = Path(work) / "unit_hands_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's unit hands (tools/engine_unit_hands_reference.py).\n"
        "# <function> <argument> | <unit> | <objects 1-15> | <game> | <calls>... -> <answer> | <unit after>\n"
        "# unit: current desired <slot:last used>x4 grenade <grenades>x2 zoom desired_zoom player actor flags seat <aim>\n"
        "# calls' reals are their bits\n"
        "# object: parent:puts away:seat:engine allows:must be readied:inside bsp\n"
        "# game: time running cheat engine_infinite actor_unlimited client <unit velocity> <camera> <hand> node\n"
        "#   projectile0 projectile1 new_grenade variant lower upper velocity cap seed\n"
        "# unit after: current desired <slot:last used>x4 <grenades>x2 zoom desired_zoom grenade_object flags\n"
        "#   throw_state seed\n" + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_unit_hands_reference.py")
        print("the unit hands reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
