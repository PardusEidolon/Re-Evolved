"""The shared fake world of the reference traces: the original's own data
arrays (source/memory/data.c, compiled as it is) and the object header table
objects.c keeps its objects in, with stand-ins for what they ask of the rest
of the game.

A script takes `prelude()` (the types, macros and stand-ins), then
`data_functions()` and, if it needs the object table, `object_functions()`,
then its own cases. The stand-ins are the least the functions need: a failed
assertion prints `assert <file> <line>` and exits 1, so a trace never goes on
past one.

The scripts written before it (items, vehicle functions and the rest) reach
their one object through a stand-in getter and never walk a table, so they
keep their own stand-ins and their traces as they are; a script that needs a
table of objects builds on this world.
"""

from . import function, read, structure

# The functions of data.c the world compiles, in an order that defines each
# before its first use.
DATA_FUNCTIONS = [
    "data_allocation_size",
    "data_initialize",
    "datum_try_and_get",
    "datum_get",
    "data_verify",
    "data_new",
    "data_dispose",
    "data_make_invalid",
    "datum_initialize",
    "data_usable",
    "datum_new_at_index",
    "datum_new",
    "datum_delete",
    "data_delete_all",
    "data_make_valid",
    "data_iterator_new",
    "data_iterator_next",
    "data_next_index",
]

# objects.c's header table: an object's header datum and its memory.
OBJECT_FUNCTIONS = [
    "object_header_new",
    "object_header_delete",
]

PRELUDE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* the game's 32-bit long */
#define long int

typedef int boolean;
typedef unsigned char byte;
typedef unsigned short word;

#define TRUE 1
#define FALSE 0
#define NONE (-1)
#define SHORT_MAX 32767
#define TAG_STRING_LENGTH 31
#define NUMBEROF(a) ((long)(sizeof(a)/sizeof((a)[0])))

static char temporary[256];

static void world_assert_failed(const char *file, int line)
{
	printf("assert %s %d\n", file, line);
	exit(1);
}

#define match_assert(file, line, condition) do { if (!(condition)) world_assert_failed(file, line); } while (0)
#define match_vassert(file, line, condition, message) do { if (!(condition)) world_assert_failed(file, line); } while (0)
#define csprintf(buffer, ...) (snprintf((buffer), sizeof(temporary), __VA_ARGS__), (buffer))
#define csmemset memset
#define csstrncpy strncpy
#define match_malloc(file, line, size) malloc(size)
#define match_free(file, line, pointer) free(pointer)

static void release_assert_failed(const char *message, const char *file, int line, boolean halt)
{
	world_assert_failed(file, line);
}

void platform_log(const char *format, ...)
{
}
"""

PROTOTYPES = r"""
static void datum_initialize(struct data_array *data, struct datum_header *header);
static boolean data_usable(struct data_array *data, void const *caller);
"""

OBJECT_PRELUDE = r"""
struct object_datum { byte bytes[1]; };

/* the object memory pool: each block its own allocation, which a trace of
the header table cannot tell from the pool's */
static void *object_memory_pool;
static long world_pool_refuses;

static boolean memory_pool_block_allocate(void *pool, void **block, long size)
{
	if (world_pool_refuses)
		return FALSE;
	*block = malloc(size);
	return *block != NULL;
}

static void memory_pool_block_free(void *pool, void **block)
{
	free(*block);
	*block = NULL;
}
"""


def prelude():
    """The types, macros and stand-ins the data arrays need."""
    data_h = read("source/memory/data.h")
    structures = [structure(data_h, name) for name in ("data_array", "datum_header", "data_iterator")]
    return "\n\n".join([PRELUDE, *structures, PROTOTYPES])


def data_functions():
    """data.c's functions, as it defines them."""
    data_c = read("source/memory/data.c")
    return "\n\n".join(function(data_c, name) for name in DATA_FUNCTIONS)


def object_functions():
    """objects.c's object header table, with the pool it allocates from."""
    objects_h = read("source/objects/objects.h")
    objects_c = read("source/objects/objects.c")
    header = structure(objects_h, "object_header_datum")
    return "\n\n".join([OBJECT_PRELUDE, header, *(function(objects_c, name) for name in OBJECT_FUNCTIONS)])
