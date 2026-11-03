"""The game's model nodes and markers: each node's matrix from its
orientation and its parent's, and the matrices of a marker found by name,
recorded for the engine's to match.

    python3 tools/engine_model_reference.py          # writes the reference
    python3 tools/engine_model_reference.py --check  # fails if it is stale

The functions are source/models/models.c's
`model_node_matrices_from_orientations`, `model_find_marker` and
`model_get_marker_by_name`, with source/math/matrix_math.c's matrices,
compiled for this machine against a small fake world: models whose nodes
are trees (some with a parent, sibling or child the model does not have,
or links in a loop, which leave nodes the walk never reaches: the game
leaves their matrices as they were, the engine at the identity), at orientations of any length and scale under roots
facing each way, and markers in the order the tools sort them, on nodes
the model has and does not have, asked for by names in any case and names
the model does not have. A marker is asked for without the object's
region permutations, as the engine asks, and unmirrored. The table is
written to tools/traces/model_reference.txt, which
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
from harness import enum_with, function, inline, read, structure  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "model_reference.txt"

MATRIX_FUNCTIONS = [
    "matrix4x3_rotation_from_quaternion",
    "matrix4x3_from_point_and_quaternion",
    "matrix4x3_from_orientation",
    "matrix4x3_rotation_from_vectors",
    "matrix4x3_from_point_and_vectors",
    "matrix4x3_multiply",
]

MODEL_FUNCTIONS = [
    "model_node_matrices_from_orientations",
    "model_find_marker",
    "model_get_marker_by_name",
]

FAKE_WORLD = r"""
#include <stdio.h>
#include <string.h>
#include <strings.h>

/* the game's 32-bit long */
#define long int

typedef float real;
typedef unsigned char byte;
typedef unsigned short word;
typedef int boolean;

#define TRUE 1
#define FALSE 0
#define NONE -1
#define MIN(a,b) ((a)<(b)?(a):(b))
#define VALID_INDEX(index, count) ((index)>=0 && (index)<(count))
#define match_assert(file, line, condition) ((void)0)
#define _stricmp strcasecmp
#define TAG_STRING_LENGTH 31
#define UNSIGNED_SHORT_MAX 0xffff

typedef struct { real i, j, k; } real_vector3d;
typedef struct { real x, y, z; } real_point3d;
typedef struct real_quaternion { real_vector3d v; real w; } real_quaternion;
typedef struct real_orientation { real_quaternion rotation; real_point3d translation; real scale; } real_orientation;

struct tag_block { long count; void *address; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (&((type *)(block)->address)[index])

static void set_real_point3d(real_point3d *point, real x, real y, real z)
{
	point->x = x; point->y = y; point->z = z;
}
"""

WORLD = r"""
/* the model's fields the functions read */
struct model { struct tag_block nodes; struct tag_block markers; };

#define MODELS 6
#define MARKERS 9
static struct model models[MODELS];
static struct model_node nodes[MODELS][MAXIMUM_NODES_PER_MODEL];
static struct model_marker markers[MODELS][MARKERS];
static struct model_marker_instance instances[MODELS][MARKERS][4];

static struct model *model_definition_get(long index) { return &models[index]; }
static void model_data_error(struct model const *model, char const *what) { (void)model; (void)what; }
"""

MAIN = r"""
static unsigned int seed = 0x5eed;

static unsigned int next(void)
{
	seed = seed*1664525u+1013904223u;
	return seed;
}

/* a real from -scale to scale, in steps of a thousandth of it */
static real between(real scale)
{
	return (real)((int)(next()>>16)%2001-1000)*0.001f*scale;
}

/* an index below count */
static int below(int count)
{
	return (int)((next()>>16)%(unsigned)count);
}

static void print_matrix(real_matrix4x3 const *m)
{
	printf(" %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", m->scale,
		m->forward.i, m->forward.j, m->forward.k, m->left.i, m->left.j, m->left.k,
		m->up.i, m->up.j, m->up.k, m->position.x, m->position.y, m->position.z);
}

/* the markers' names, in the order _stricmp sorts them */
static char const *const marker_names[MARKERS] =
{
	"Body", "driver", "Exit", "gunner", "head", "Primary", "seat", "Thruster", "thrusterjet",
};

/* names asked for: the markers' in other cases, and names the models do not have */
static char const *const asked_names[] =
{
	"body", "BODY", "Driver", "exit", "GUNNER", "head", "primary", "SEAT", "thruster", "ThrusterJet",
	"", "aaa", "zzz", "gun", "thrusterjets", "Thrust",
};

int main(void)
{
	static int const node_counts[MODELS] = { 1, 2, 5, 12, 40, 70 };
	int m, i, trial;

	for (m = 0; m<MODELS; m++)
	{
		int count = node_counts[m];
		int stored = MIN(count, MAXIMUM_NODES_PER_MODEL);
		int marker_count = m==0 ? 0 : MARKERS-(m%3);

		models[m].nodes.count = count;
		models[m].nodes.address = nodes[m];
		models[m].markers.count = marker_count;
		models[m].markers.address = markers[m];
		/* a tree: each node after a parent before it, its siblings in turn */
		for (i = 0; i<stored; i++)
		{
			nodes[m][i].parent_node_index = i==0 ? NONE : below(i);
			nodes[m][i].first_child_node_index = NONE;
			nodes[m][i].next_sibling_node_index = NONE;
		}
		for (i = stored-1; i>0; i--)
		{
			struct model_node *parent = &nodes[m][nodes[m][i].parent_node_index];

			nodes[m][i].next_sibling_node_index = parent->first_child_node_index;
			parent->first_child_node_index = (short)i;
		}
		if (m==3)
		{
			/* a parent, a sibling and a child the model does not have, and a loop */
			nodes[m][5].parent_node_index = 90;
			nodes[m][6].next_sibling_node_index = 200;
			nodes[m][7].first_child_node_index = 12;
			nodes[m][8].first_child_node_index = 1;
		}
		for (i = 0; i<stored; i++)
			printf("node %d %d %d %d %d\n", m, i, nodes[m][i].parent_node_index,
				nodes[m][i].first_child_node_index, nodes[m][i].next_sibling_node_index);
		printf("model %d %d\n", m, count);

		for (i = 0; i<marker_count; i++)
		{
			int k, instance_count = 1+below(4);

			strcpy(markers[m][i].name, marker_names[i]);
			markers[m][i].instances.count = instance_count;
			markers[m][i].instances.address = instances[m][i];
			for (k = 0; k<instance_count; k++)
			{
				struct model_marker_instance *instance = &instances[m][i][k];

				instance->region_index = (byte)below(10);
				instance->permutation_index = (byte)below(3);
				/* now and then on a node the model does not have */
				instance->node_index = (byte)(below(9)==0 ? stored+below(3) : below(stored));
				instance->translation.x = between(2.0f);
				instance->translation.y = between(2.0f);
				instance->translation.z = between(2.0f);
				instance->rotation.v.i = between(1.0f);
				instance->rotation.v.j = between(1.0f);
				instance->rotation.v.k = between(1.0f);
				instance->rotation.w = between(1.0f);
				printf("instance %d %s %d %d %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", m, marker_names[i],
					instance->region_index, instance->permutation_index, instance->node_index,
					instance->translation.x, instance->translation.y, instance->translation.z,
					instance->rotation.v.i, instance->rotation.v.j, instance->rotation.v.k, instance->rotation.w);
			}
		}
	}

	for (trial = 0; trial<60; trial++)
	{
		real_orientation orientations[MAXIMUM_NODES_PER_MODEL];
		real_matrix4x3 matrices[MAXIMUM_NODES_PER_MODEL];
		real_point3d origin = { between(50.0f), between(50.0f), between(50.0f) };
		real_vector3d forward = { between(1.0f), between(1.0f), between(1.0f) };
		real_vector3d up = { between(1.0f), between(1.0f), between(1.0f) };
		int stored, a;

		m = trial%MODELS;
		stored = MIN(node_counts[m], MAXIMUM_NODES_PER_MODEL);
		/* a node the walk never reaches keeps what its matrix had */
		for (i = 0; i<MAXIMUM_NODES_PER_MODEL; i++)
			matrices[i].scale = -12345.0f;
		printf("pose %d %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", trial, m,
			origin.x, origin.y, origin.z, forward.i, forward.j, forward.k, up.i, up.j, up.k);
		for (i = 0; i<stored; i++)
		{
			real_orientation *o = &orientations[i];

			o->rotation.v.i = between(1.0f);
			o->rotation.v.j = between(1.0f);
			o->rotation.v.k = between(1.0f);
			o->rotation.w = between(1.0f);
			if (below(13)==0)
				o->rotation.v.i = o->rotation.v.j = o->rotation.v.k = o->rotation.w = 0.0f;
			o->translation.x = between(3.0f);
			o->translation.y = between(3.0f);
			o->translation.z = between(3.0f);
			o->scale = below(4)==0 ? 1.0f+between(0.5f) : 1.0f;
			printf("orientation %d %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g %.9g\n", trial, i,
				o->rotation.v.i, o->rotation.v.j, o->rotation.v.k, o->rotation.w,
				o->translation.x, o->translation.y, o->translation.z, o->scale);
		}
		model_node_matrices_from_orientations(&models[m], matrices, orientations, &origin, &forward, &up);
		for (i = 0; i<stored; i++)
		{
			if (matrices[i].scale==-12345.0f)
			{
				/* left as it was: here the identity, as the engine leaves
				it, so that its markers can be compared */
				static real_matrix4x3 const identity = { 1.0f, { { { 1.0f, 0.0f, 0.0f }, { 0.0f, 1.0f, 0.0f }, { 0.0f, 0.0f, 1.0f }, { 0.0f, 0.0f, 0.0f } } } };

				matrices[i] = identity;
				printf("unreached %d %d\n", trial, i);
				continue;
			}
			printf("matrix %d %d", trial, i);
			print_matrix(&matrices[i]);
		}
		for (a = 0; a<(int)(sizeof(asked_names)/sizeof(asked_names[0])); a++)
		{
			struct object_marker found[16];
			short index = model_find_marker(m, asked_names[a]);
			short count = model_get_marker_by_name(m, asked_names[a], NULL, NULL, 0, matrices, FALSE, found, 16);
			int f;

			printf("ask %d %d %d %d %d\n", trial, m, a, index, count);
			for (f = 0; f<count; f++)
			{
				printf("found %d %d %d %d", trial, a, f, found[f].node_index);
				print_matrix(&found[f].matrix);
			}
		}
	}
	return 0;
}
"""

ASKED = [
    "body", "BODY", "Driver", "exit", "GUNNER", "head", "primary", "SEAT", "thruster", "ThrusterJet",
    "", "aaa", "zzz", "gun", "thrusterjets", "Thrust",
]  # fmt: skip


def source_file():
    """The fake world, the game's functions and the cases, as one C file."""
    real_math_h = read("source/math/real_math.h")
    matrix_c = read("source/math/matrix_math.c")
    definitions_h = read("source/models/model_definitions.h")
    objects_h = read("source/objects/objects.h")
    models_c = read("source/models/models.c")
    parts = [
        FAKE_WORLD,
        structure(real_math_h, "real_matrix4x3") + "\ntypedef struct real_matrix4x3 real_matrix4x3;",
        inline(real_math_h, "cross_product3d"),
        inline(real_math_h, "negate_vector3d"),
        enum_with(definitions_h, "MAXIMUM_NODES_PER_MODEL"),
        structure(definitions_h, "model_marker_instance"),
        structure(definitions_h, "model_marker"),
        structure(definitions_h, "model_node"),
        structure(objects_h, "object_marker"),
        "enum { MAXIMUM_REGIONS_PER_OBJECT = 8 };",
        WORLD,
        *[function(matrix_c, name) for name in MATRIX_FUNCTIONS],
        *[function(models_c, name) for name in MODEL_FUNCTIONS],
        MAIN,
    ]
    return "\n\n".join(parts)


def reference():
    """The reference, as the game's code writes it."""
    with tempfile.TemporaryDirectory(prefix="model-reference-") as work:
        source = Path(work) / "model_reference.c"
        executable = Path(work) / "model_reference"
        source.write_text(source_file())
        compiler = os.environ.get("CC", "clang")
        command = [compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w", str(source), "-o", str(executable), "-lm"]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    names = "".join(f"asked {index} {name or '-'}\n" for index, name in enumerate(ASKED))
    return (
        "# The game's model nodes and markers (tools/engine_model_reference.py).\n"
        "# node <model> <index> <parent> <first child> <next sibling>\n"
        "# model <model> <node count>\n"
        "# instance <model> <marker> <region> <permutation> <node> <translation> <rotation i j k w>\n"
        "# asked <index> <name, - for the empty one>\n"
        "# pose <trial> <model> <origin> <forward> <up>\n"
        "# orientation <trial> <node> <rotation i j k w> <translation> <scale>\n"
        "# matrix <trial> <node> <scale> <forward> <left> <up> <position>\n"
        "# unreached <trial> <node>: a node the walk never reaches\n"
        "# ask <trial> <model> <asked> <marker index> <found>\n"
        "# found <trial> <asked> <index> <node> <matrix>\n" + names + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_model_reference.py")
        print("the model reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
