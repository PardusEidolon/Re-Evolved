"""The game's collision BSP queries: where a line first meets a surface, what
surfaces, edges and vertices a sphere touches, where a moving sphere first
meets a surface and which leaf a point is in, recorded for the engine's
(Jolt's, over the mesh made from the same BSP) to match.

    python3 tools/engine_collision_reference.py          # writes the reference
    python3 tools/engine_collision_reference.py --check  # fails if it is stale

The functions are source/physics/collision_bsp.c's `collision_bsp_test_vector`,
`collision_bsp_test_sphere` and `collision_bsp_test_pill`, bsp3d.c's
`bsp3d_test_point` and bsp2d.c's `bsp2d_test_point`, compiled for this
machine against a small fake world: one collision BSP, made here, of a room
(a box cut by three slanted walls, its surfaces facing in) with a floating
pillar (a box with one slanted face, facing out). Its bsp3d is a chain of the
room's planes, solid behind each, ending in a chain of the pillar's, a leaf
in front of each; each leaf's bsp2d references name the room's surfaces and
its own pillar face. Lines, spheres and moving spheres are cast from
anywhere in the room (some from in the pillar, or outside the slanted
walls), in any direction, of any length and radius. The table is written to
tools/traces/collision_reference.txt, which
tools/engine_wasm_traces.py checks game.wasm against.
"""

import argparse
import itertools
import math
import os
import re
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools"))
from harness import enum_with, function, read  # noqa: E402

OUTPUT = ROOT / "tools" / "traces" / "collision_reference.txt"

# The headers collision_bsp.c includes that would bring in the rest of the
# game, as the little they give it.
FAKE_CSERIES = r"""
#ifndef FAKE_CSERIES_H
#define FAKE_CSERIES_H
#include <limits.h>
#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

/* the game's 32-bit long */
#undef LONG_MAX
#undef LONG_MIN
#define LONG_MAX 0x7fffffff
#define LONG_MIN ((int)0x80000000)
typedef struct { int64_t QuadPart; } LARGE_INTEGER;
#define long int

typedef float real;
typedef unsigned char byte;
typedef unsigned short word;
typedef byte boolean;
#define __inline static inline
#define TRUE 1
#define FALSE 0
#define NONE -1
#define FLAG(bit) (1u<<(bit))
#define TEST_FLAG(flags, bit) ((((unsigned)(flags))>>(bit))&1)
#define LONG_BITS 32
#define LONG_BITS_BITS 5
#define BIT_VECTOR_TEST_FLAG(bit_vector, bit) (TEST_FLAG((bit_vector)[(bit) >> LONG_BITS_BITS], ((bit) & (LONG_BITS - 1))))
#define PIN(n,floor,ceiling) ((n)<(floor) ? (floor) : ((n)>(ceiling)?(ceiling):(n)))
#define FLOOR(n,floor) ((n)<(floor) ? (floor) : (n))
#define CEILING(n,ceiling) ((n)>(ceiling) ? (ceiling) : (n))
#define MAX(a,b) ((a)>(b) ? (a) : (b))
#define MIN(a,b) ((a)<(b) ? (a) : (b))
#define match_assert(file, line, condition) ((void)0)
#define assert(condition) ((void)0)
enum { _x = 0, _y, _z };
enum { _error_silent };
static void error(int priority, char const *format, ...) {}

/* (the collision log and the debug drawing) */
static void collision_log_usage(short function) {}
static void collision_log_start_time(LARGE_INTEGER *time) {}
static void collision_log_end_time(short function, int64_t time) {}
#endif
"""

FAKE_RENDER_DEBUG = r"""
#ifndef FAKE_RENDER_DEBUG_H
#define FAKE_RENDER_DEBUG_H
#include "cseries/cseries.h"
#include "math/real_math.h"
static real_argb_color const *global_real_argb_green;
static void render_debug_point(boolean z_buffer, real_point3d const *point, real scale, real_argb_color const *color) {}
static void render_debug_line(boolean z_buffer, real_point3d const *a, real_point3d const *b, real_argb_color const *color) {}
#endif
"""

FAKE_TAG_GROUPS = r"""
#ifndef FAKE_TAG_GROUPS_H
#define FAKE_TAG_GROUPS_H
#include "cseries/cseries.h"
struct tag_block { long count; void *address; void *definition; };
struct tag_field { long type; };
#define TAG_BLOCK_GET_ELEMENT(block, index, type) (((type *)((block)->address))+(index))
#endif
"""

INCLUDING_CSERIES = '#include "cseries/cseries.h"\n'

MAIN = r"""
static unsigned int seed = 0xc011;

static unsigned int next(void)
{
	seed = seed*1664525u+1013904223u;
	return seed;
}

/* a real from low to high */
static real between(real low, real high)
{
	return low + (high-low)*(real)(next()>>8)/(real)(1u<<24);
}

static void random_point(real_point3d *point)
{
	point->x = between(-9.5f, 9.5f);
	point->y = between(-9.5f, 9.5f);
	point->z = between(-9.5f, 9.5f);
}

/* a direction, of any length up to `longest` */
static void random_vector(real_vector3d *vector, real longest)
{
	real length;
	do
	{
		vector->i = between(-1.f, 1.f);
		vector->j = between(-1.f, 1.f);
		vector->k = between(-1.f, 1.f);
		length = sqrtf(vector->i*vector->i + vector->j*vector->j + vector->k*vector->k);
	}
	while (length < 0.1f || length > 1.f);
	length = between(0.5f, longest)/length;
	vector->i *= length;
	vector->j *= length;
	vector->k *= length;
}

static void print_list(char const *name, long count, long const *indices)
{
	long index;
	printf(" %s %d", name, count);
	for (index = 0; index<count; index++)
		printf(" %d", indices[index]);
}

int main(void)
{
	static struct collision_bsp bsp;
	int trial;

	bsp.bsp3d.nodes.count = NODE_COUNT;
	bsp.bsp3d.nodes.address = (void *)nodes;
	bsp.bsp3d.planes.count = PLANE_COUNT;
	bsp.bsp3d.planes.address = (void *)planes;
	bsp.leaves.count = LEAF_COUNT;
	bsp.leaves.address = (void *)leaves;
	bsp.bsp2d_references.count = REFERENCE_COUNT;
	bsp.bsp2d_references.address = (void *)references;
	bsp.bsp2d.nodes.count = 0;
	bsp.surfaces.count = SURFACE_COUNT;
	bsp.surfaces.address = (void *)surfaces;
	bsp.edges.count = EDGE_COUNT;
	bsp.edges.address = (void *)edges;
	bsp.vertices.count = VERTEX_COUNT;
	bsp.vertices.address = (void *)vertices;

	for (trial = 0; trial<1000; trial++)
	{
		real_point3d point;
		real_vector3d vector;
		struct collision_bsp_test_vector_result result;
		boolean hit;

		random_point(&point);
		random_vector(&vector, 40.f);
		hit = collision_bsp_test_vector(FLAG(_collision_test_front_facing_surfaces_bit), &bsp, 0, NULL,
			&point, &vector, 1.f, &result);
		printf("vector %d %.9g %.9g %.9g %.9g %.9g %.9g %d", trial, point.x, point.y, point.z,
			vector.i, vector.j, vector.k, hit);
		if (hit)
			printf(" %.9g %d %d", result.t, (int)(result.plane - planes), result.surface_index);
		printf("\n");
	}

	for (trial = 0; trial<1000; trial++)
	{
		real_point3d center;
		real radius = between(0.1f, 3.f);
		struct collision_bsp_test_sphere_result result;
		boolean hit;

		random_point(&center);
		hit = collision_bsp_test_sphere(&bsp, 0, NULL, &center, radius, &result);
		printf("sphere %d %.9g %.9g %.9g %.9g %d", trial, center.x, center.y, center.z, radius, hit);
		print_list("surfaces", result.surface_count, result.surface_indices);
		print_list("edges", result.edge_count, result.edge_indices);
		print_list("vertices", result.vertex_count, result.vertex_indices);
		printf("\n");
	}

	for (trial = 0; trial<1000; trial++)
	{
		real_point3d point;
		real_vector3d vector;
		real radius = between(0.1f, 2.f);
		struct collision_bsp_test_pill_result result;
		boolean hit;

		random_point(&point);
		random_vector(&vector, 30.f);
		hit = collision_bsp_test_pill(&bsp, &point, &vector, radius, 1.f, &result);
		printf("pill %d %.9g %.9g %.9g %.9g %.9g %.9g %.9g %d", trial, point.x, point.y, point.z,
			vector.i, vector.j, vector.k, radius, hit);
		if (hit)
			printf(" %.9g %.9g %.9g %.9g %.9g %d", result.t, result.plane.n.i, result.plane.n.j,
				result.plane.n.k, result.plane.d, result.surface_index);
		printf("\n");
	}

	for (trial = 0; trial<1000; trial++)
	{
		real_point3d point;

		point.x = between(-11.f, 11.f);
		point.y = between(-11.f, 11.f);
		point.z = between(-11.f, 11.f);
		printf("point %d %.9g %.9g %.9g %d\n", trial, point.x, point.y, point.z,
			bsp3d_test_point(&bsp.bsp3d, 0, &point));
	}
	return 0;
}
"""


def f32(value):
    """A double as the nearest float, as the map holds it."""
    return struct.unpack("<f", struct.pack("<f", value))[0]


def plane(normal, distance):
    """A plane with a unit normal: n.p - d is its distance to p."""
    length = math.sqrt(sum(component * component for component in normal))
    return tuple(f32(component / length) for component in normal) + (f32(distance / length),)


# The room: in front of each plane is inside it.
ROOM = [
    plane((0, 0, 1), -10),
    plane((0, 0, -1), -10),
    plane((1, 0, 0), -10),
    plane((-1, 0, 0), -10),
    plane((0, 1, 0), -10),
    plane((0, -1, 0), -10),
    plane((1, 1, 1), -25),
    plane((-1, 0, -1), -16),
    plane((0.3, -1, 0.2), -10.5),
]

# The pillar: behind every plane is inside it.
PILLAR = [
    plane((1, 0, 0), 5),
    plane((-1, 0, 0), -1),
    plane((0, 1, 0), 0),
    plane((0, -1, 0), 4),
    plane((0, 0, 1), 2),
    plane((0, 0, -1), 6),
    plane((1, 0, 1), 6),
]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def corner(planes):
    """Where three planes meet, or None if two are parallel."""
    (n0, d0), (n1, d1), (n2, d2) = ((p[:3], p[3]) for p in planes)
    determinant = dot(n0, cross(n1, n2))
    if abs(determinant) < 1e-9:
        return None
    terms = [tuple(d * c for c in v) for d, v in ((d0, cross(n1, n2)), (d1, cross(n2, n0)), (d2, cross(n0, n1)))]
    return tuple(sum(term[axis] for term in terms) / determinant for axis in range(3))


def faces(planes, inside):
    """Each plane's face of the convex solid `inside` says the planes bound
    (+1: in front of them all, -1: behind them all): its corners, counter-
    clockwise seen from the plane's front."""
    corners = []
    for triple in itertools.combinations(range(len(planes)), 3):
        point = corner([planes[index] for index in triple])
        if point is None:
            continue
        if all(inside * (dot(p[:3], point) - p[3]) >= -1e-4 for p in planes):
            corners.append((triple, point))
    result = []
    for index, p in enumerate(planes):
        on = []
        for triple, point in corners:
            if index in triple and not any(math.dist(point, other) < 1e-4 for other in on):
                on.append(point)
        if len(on) < 3:
            raise SystemExit(f"plane {p} has no face")
        middle = tuple(sum(point[axis] for point in on) / len(on) for axis in range(3))
        normal = p[:3]
        u = tuple(a - b for a, b in zip(on[0], middle))
        v = cross(normal, u)
        on.sort(
            key=lambda point: math.atan2(
                dot(v, [a - b for a, b in zip(point, middle)]), dot(u, [a - b for a, b in zip(point, middle)])
            )
        )
        result.append(on)
    return result


def level():
    """The collision BSP's blocks, as rows."""
    planes = ROOM + PILLAR
    vertices, edges, surfaces = [], [], []
    halves = {}

    def vertex(point):
        point = tuple(f32(component) for component in point)
        for index, (other, _) in enumerate(vertices):
            if math.dist(point, other) < 1e-4:
                return index
        vertices.append((point, None))
        return len(vertices) - 1

    for offset, solid in ((0, ROOM), (len(ROOM), PILLAR)):
        for index, ring in enumerate(faces(solid, 1 if offset == 0 else -1)):
            surface = len(surfaces)
            ring = [vertex(point) for point in ring]
            ring_edges = []
            for a, b in zip(ring, ring[1:] + ring[:1]):
                if (b, a) in halves:
                    edge = halves[(b, a)]
                    edges[edge][2][1] = surface
                    ring_edges.append((edge, 1))
                else:
                    edges.append([[a, b], [None, None], [surface, None]])
                    halves[(a, b)] = len(edges) - 1
                    ring_edges.append((len(edges) - 1, 0))
            for (edge, side), (following, _) in zip(ring_edges, ring_edges[1:] + ring_edges[:1]):
                edges[edge][1][side] = following
            surfaces.append((offset + index, ring_edges[0][0]))
    for edge in edges:
        if None in edge[1] or None in edge[2]:
            raise SystemExit("a surface's edge has no neighbour: the solids are not closed")
    for index, (point, _) in enumerate(vertices):
        vertices[index] = (point, next(e for e, edge in enumerate(edges) if index in edge[0]))

    # the room's planes, solid behind each, then the pillar's, a leaf in front of each
    nodes, leaves, references = [], [], []
    room_count = len(ROOM)
    for index in range(room_count):
        nodes.append((index, -1, index + 1))
    for index in range(len(PILLAR)):
        back = room_count + index + 1 if index + 1 < len(PILLAR) else -1
        nodes.append((room_count + index, back, -0x80000000 | index))
        first = len(references)
        for surface in list(range(room_count)) + [room_count + index]:
            references.append((surfaces[surface][0], -0x80000000 | surface))
        leaves.append((0, len(references) - first, first))
    return planes, vertices, edges, surfaces, nodes, leaves, references


def tables():
    """The level as the fake world's C arrays, and as the reference's rows."""
    planes, vertices, edges, surfaces, nodes, leaves, references = level()
    c = [
        f"#define PLANE_COUNT {len(planes)}",
        f"#define VERTEX_COUNT {len(vertices)}",
        f"#define EDGE_COUNT {len(edges)}",
        f"#define SURFACE_COUNT {len(surfaces)}",
        f"#define NODE_COUNT {len(nodes)}",
        f"#define LEAF_COUNT {len(leaves)}",
        f"#define REFERENCE_COUNT {len(references)}",
        "static real_plane3d const planes[] = {" + ", ".join("{{{%.9g, %.9g, %.9g}}, %.9g}" % p for p in planes) + "};",
        "static struct collision_vertex const vertices[] = {"
        + ", ".join("{{{%.9g, %.9g, %.9g}}, %d}" % (*point, first) for point, first in vertices)
        + "};",
        "static struct collision_edge const edges[] = {"
        + ", ".join("{{%d, %d}, {%d, %d}, {%d, %d}}" % (*v, *e, *s) for v, e, s in edges)
        + "};",
        "static struct collision_surface const surfaces[] = {"
        + ", ".join("{%d, %d, 0, 0, NONE}" % surface for surface in surfaces)
        + "};",
        "static struct bsp3d_node const nodes[] = {" + ", ".join("{%d, {%d, %d}}" % node for node in nodes) + "};",
        "static struct collision_leaf const leaves[] = {" + ", ".join("{%d, %d, %d}" % leaf for leaf in leaves) + "};",
        "static struct bsp2d_reference const references[] = {"
        + ", ".join("{%d, %d}" % reference for reference in references)
        + "};",
    ]
    rows = (
        ["plane %d %.9g %.9g %.9g %.9g" % (index, *p) for index, p in enumerate(planes)]
        + ["vertex %d %.9g %.9g %.9g %d" % (index, *point, first) for index, (point, first) in enumerate(vertices)]
        + ["edge %d %d %d %d %d %d %d" % (index, *v, *e, *s) for index, (v, e, s) in enumerate(edges)]
        + ["surface %d %d %d" % (index, *surface) for index, surface in enumerate(surfaces)]
        + ["node %d %d %d %d" % (index, *node) for index, node in enumerate(nodes)]
        + ["leaf %d %d %d %d" % (index, *leaf) for index, leaf in enumerate(leaves)]
        + ["reference %d %d %d" % (index, *reference) for index, reference in enumerate(references)]
    )
    return "\n".join(c), "\n".join(rows) + "\n"


def collision_bsp_c():
    """collision_bsp.c from its constants on, without its size asserts (they
    are the Xbox's 32-bit pointers')."""
    text = read("source/physics/collision_bsp.c")
    text = text[text.index("/* ---------- constants */") :]
    return re.sub(r"typedef char \w+\[[^;]*;", "", text)


def source_file(level_c=None):
    """The fake world, the game's functions and the cases, as one C file."""
    if level_c is None:
        level_c = tables()[0]
    real_math_c = read("source/math/real_math.c")
    bsp3d_c = read("source/physics/bsp3d.c")
    bsp2d_c = read("source/physics/bsp2d.c")
    return "\n\n".join(
        [
            '#include "cseries/cseries.h"',
            '#include "collision_bsp_definitions.h"',
            '#include "collision_bsp.h"',
            '#include "collisions.h"',
            '#include "render/render_debug.h"',
            "short const global_projection3d_mappings[3][2][2] = {{{2,1},{1,2}},{{0,2},{2,0}},{{1,0},{0,1}}};",
            "static struct collision_bsp *global_collision_bsp;",
            function(real_math_c, "fast_vector_intersects_sphere"),
            function(read("source/math/matrix_math.c"), "matrix4x3_transform_point"),
            "static boolean warned_about_bsp3d_nodes;",
            function(bsp3d_c, "bsp3d_test_point"),
            "static boolean warned_about_bsp2d_nodes;",
            function(bsp2d_c, "bsp2d_test_point"),
            collision_bsp_c(),
            level_c,
            MAIN,
        ]
    )


def check_projection_mappings():
    """The table the fake world holds is real_math.c's."""
    text = read("source/math/real_math.c")
    start = text.index("short const global_projection3d_mappings[3][2][2]")
    table = text[text.index("=", start) : text.index(";", start)]
    if [int(number) for number in re.findall(r"\d+", table)] != [2, 1, 1, 2, 0, 2, 2, 0, 1, 0, 0, 1]:
        raise SystemExit("real_math.c's global_projection3d_mappings is not the fake world's")


def reference():
    """The reference, as the game's code writes it."""
    check_projection_mappings()
    level_c, rows = tables()
    with tempfile.TemporaryDirectory(prefix="collision-reference-") as work:
        work = Path(work)
        for relative, text in (
            ("cseries/cseries.h", FAKE_CSERIES),
            ("cseries/errors.h", INCLUDING_CSERIES),
            ("tag_files/tag_groups.h", FAKE_TAG_GROUPS),
            ("render/render_debug.h", FAKE_RENDER_DEBUG),
            ("scenario/scenario.h", INCLUDING_CSERIES),
            ("collision_usage.h", INCLUDING_CSERIES),
            ("math/integer_math.h", read("source/math/integer_math.h")),
            (
                "collisions.h",
                INCLUDING_CSERIES
                + enum_with(read("source/physics/collisions.h"), "_collision_test_front_facing_surfaces_bit"),
            ),
        ):
            (work / "fake" / relative).parent.mkdir(parents=True, exist_ok=True)
            (work / "fake" / relative).write_text(text)
        # real_math.h, with its integer_math.h the fake one
        (work / "fake" / "math" / "real_math.h").write_text(
            read("source/math/real_math.h").replace('#include "integer_math.h"', '#include "math/integer_math.h"')
        )
        source = work / "collision_reference.c"
        executable = work / "collision_reference"
        source.write_text(source_file(level_c))
        compiler = os.environ.get("CC", "clang")
        command = [
            compiler, "-std=gnu99", "-O1", "-ffp-contract=off", "-w",
            f"-I{work / 'fake'}", f"-I{ROOT / 'source' / 'physics'}",
            str(source), "-o", str(executable), "-lm",
        ]  # fmt: skip
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode:
            raise SystemExit(f"the reference does not compile:\n{result.stderr}")
        output = subprocess.run([str(executable)], capture_output=True, text=True, check=True).stdout
    return (
        "# The game's collision BSP queries (tools/engine_collision_reference.py).\n"
        "# plane <index> <i j k d>; vertex <index> <x y z> <first edge>\n"
        "# edge <index> <vertices 2> <next edges 2> <surfaces 2>; surface <index> <plane> <first edge>\n"
        "# node <index> <plane> <back> <front>; leaf <index> <flags> <references> <first>\n"
        "# reference <index> <plane> <root>\n"
        "# vector <trial> <point> <vector> <hit> [<t> <plane> <surface>]\n"
        "# sphere <trial> <center> <radius> <hit> surfaces <n> <...> edges <n> <...> vertices <n> <...>\n"
        "# pill <trial> <point> <vector> <radius> <hit> [<t> <plane i j k d> <surface>]\n"
        "# point <trial> <point> <leaf or -1>\n" + rows + output
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail if the written reference is not the code's")
    arguments = parser.parse_args()
    text = reference()
    if arguments.check:
        if not OUTPUT.exists() or OUTPUT.read_text() != text:
            raise SystemExit(f"{OUTPUT.relative_to(ROOT)} is stale: run tools/engine_collision_reference.py")
        print("the collision reference is the game's code's")
        return
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(text)
    print(f"wrote {OUTPUT.relative_to(ROOT)} ({len(text.splitlines())} lines)")


if __name__ == "__main__":
    main()
