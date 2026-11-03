/*
CRT.C

The Microsoft C runtime helpers the game calls that are the C library, not
the platform: the printf family with MSVC's length modifiers, the
case-blind string compares, the floating-point control word, the x87
rounding conversion, the compiler barrier, stdio's fopen and winsock's
fd_set test. They are compiled into game.wasm (tools/engine_wasm.py), so a
call to one never leaves the module. Each follows the Linux port's
(port/linux/src/msvc_crt.c, xnet.c), with the structures the public Win32
documentation gives. The D3DX maths is the Metal device's
(port/linux/src/d3d8_resources.c), compiled into game.wasm beside it.

Compiled without the port's shims, so that vsnprintf here is wasi-libc's.
*/

#include <ctype.h>
#include <math.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ---------- printf with MSVC length modifiers */

/* rewrite %I64 as %ll and drop %I32; everything else is shared with musl */
static const char *translate_format(const char *format, char *buffer, size_t size)
{
	const char *cursor;
	size_t length = 0;

	if (!strstr(format, "I64") && !strstr(format, "I32"))
		return format;
	for (cursor = format; *cursor && length + 3 < size; cursor++)
	{
		buffer[length++] = *cursor;
		if (*cursor != '%')
			continue;
		if (cursor[1] == '%')
		{
			buffer[length++] = *++cursor;
			continue;
		}
		/* copy flags, width and precision */
		while (cursor[1] && strchr("-+ #0123456789.*", cursor[1]) && length + 3 < size)
			buffer[length++] = *++cursor;
		if (cursor[1] == 'I' && cursor[2] == '6' && cursor[3] == '4')
		{
			buffer[length++] = 'l';
			buffer[length++] = 'l';
			cursor += 3;
		}
		else if (cursor[1] == 'I' && cursor[2] == '3' && cursor[3] == '2')
		{
			cursor += 3;
		}
	}
	buffer[length] = '\0';
	return buffer;
}

int halo_linux_vsnprintf(char *buffer, size_t count, const char *format, va_list arguments)
{
	char translated_format[1024];

	return vsnprintf(buffer, count, translate_format(format, translated_format, sizeof(translated_format)), arguments);
}

int halo_linux_snprintf(char *buffer, size_t count, const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vsnprintf(buffer, count, format, arguments);
	va_end(arguments);
	return result;
}

int halo_linux_vsprintf(char *buffer, const char *format, va_list arguments)
{
	char translated_format[1024];

	return vsprintf(buffer, translate_format(format, translated_format, sizeof(translated_format)), arguments);
}

int halo_linux_sprintf(char *buffer, const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vsprintf(buffer, format, arguments);
	va_end(arguments);
	return result;
}

int halo_linux_vfprintf(FILE *stream, const char *format, va_list arguments)
{
	char translated_format[1024];

	return vfprintf(stream, translate_format(format, translated_format, sizeof(translated_format)), arguments);
}

int halo_linux_fprintf(FILE *stream, const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vfprintf(stream, format, arguments);
	va_end(arguments);
	return result;
}

int halo_linux_vprintf(const char *format, va_list arguments)
{
	return halo_linux_vfprintf(stdout, format, arguments);
}

int halo_linux_printf(const char *format, ...)
{
	va_list arguments;
	int result;

	va_start(arguments, format);
	result = halo_linux_vfprintf(stdout, format, arguments);
	va_end(arguments);
	return result;
}

/* ---------- stdio files */

int _stricmp(const char *string1, const char *string2);

extern unsigned long halo_init_text(char *buffer, unsigned long size);

/* stdio's fopen. The game opens a few files through the C library, not the
platform: some it writes (gamestate.txt, the rasterizer's profile,
the lruv caches' dumps), which are dropped, and some it reads (the
director's d:\\camera.txt, the automation files, bypass_security.txt, a map
pack's loading picture), each of which every caller goes on without. A file
opened to read is read whole through the platform's file imports
(data_files.c), as the Linux port translates the path and opens it
(port/linux/src/msvc_crt.c), and served from memory. Two are the
platform's own: the error log, d:\\debug.txt, which goes to the module's
standard error with the other log lines (errors.c keeps it open for the
whole run and never closes it), and the console's init script,
d:\\init.txt, which is the text the platform was given (halo_init_text):
console_startup runs it as the game starts, and a map_name line starts it
on that map */
char *halo_data_file_read(const char *path, unsigned long *length);

/* a read-only file over a copy of text, in a buffer of fmemopen's own,
freed when the file is closed */
static FILE *memory_file(const char *text, unsigned long length)
{
	FILE *file = fmemopen(NULL, length + 1, "w+");

	if (!file)
		return NULL;
	fwrite(text, 1, length, file);
	rewind(file);
	return file;
}

FILE *halo_linux_fopen(const char *path, const char *mode)
{
	unsigned long length;
	char *text;
	FILE *file;

	if (!path || !mode)
		return NULL;
	if (_stricmp(path, "d:\\debug.txt") == 0)
		return stderr;
	if (_stricmp(path, "d:\\init.txt") == 0)
	{
		length = halo_init_text(NULL, 0);
		if (!length)
			return NULL;
		text = malloc(length);
		if (!text)
			return NULL;
		halo_init_text(text, length);
		file = memory_file(text, length);
		free(text);
		return file;
	}
	if (mode[0] != 'r' || strchr(mode, '+'))
		return NULL;
	text = halo_data_file_read(path, &length);
	if (!text)
		return NULL;
	file = memory_file(text, length);
	free(text);
	return file;
}

/* ---------- strings */

int _stricmp(const char *string1, const char *string2)
{
	for (;; string1++, string2++)
	{
		int c1 = tolower((unsigned char)*string1);
		int c2 = tolower((unsigned char)*string2);

		if (c1 != c2 || !c1)
			return c1 - c2;
	}
}

int _strnicmp(const char *string1, const char *string2, size_t count)
{
	for (; count; count--, string1++, string2++)
	{
		int c1 = tolower((unsigned char)*string1);
		int c2 = tolower((unsigned char)*string2);

		if (c1 != c2 || !c1)
			return c1 - c2;
	}
	return 0;
}

/* ---------- floating point */

/* MSVC's default control word (float.h's _CW_DEFAULT): rounding to nearest,
53-bit precision, every exception masked */
#define CW_DEFAULT 0x0009001F
#define EM_DENORMAL 0x00080000

/* WebAssembly rounds to nearest and raises no exceptions, always. The MSVC
control word is only remembered, so that the game reads back what it set. */
static unsigned int msvc_control_word = CW_DEFAULT;

unsigned int _control87(unsigned int new_value, unsigned int mask)
{
	if (mask)
		msvc_control_word = (msvc_control_word & ~mask) | (new_value & mask);
	return msvc_control_word;
}

unsigned int _controlfp(unsigned int new_value, unsigned int mask)
{
	/* _controlfp ignores the denormal mask */
	return _control87(new_value, mask & ~EM_DENORMAL);
}

/* round to an integer as the x87 does in its default mode, like the inline
fast_ftol in cseries.h (fld/fistp) */
long fast_ftol_C(float value)
{
	return lrintf(value);
}

/* MSVC's compiler barrier. The game's threads share the module's memory, so
it is a full fence here: a compiler-only barrier would not order the
accesses another thread sees. */
void _ReadWriteBarrier(void)
{
	__atomic_thread_fence(__ATOMIC_SEQ_CST);
}

/* ---------- winsock */

/* winsock's fd_set (winsock.h): a count and an array of sockets, which are
32-bit handles on the Xbox */
#define WINSOCK_FD_SETSIZE 64

struct winsock_fd_set
{
	unsigned int fd_count;
	unsigned int fd_array[WINSOCK_FD_SETSIZE];
};

int __WSAFDIsSet(unsigned int socket, struct winsock_fd_set *set)
{
	unsigned int index;

	for (index = 0; index < set->fd_count && index < WINSOCK_FD_SETSIZE; index++)
	{
		if (set->fd_array[index] == socket)
			return 1;
	}
	return 0;
}
