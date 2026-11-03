/*
LOG.C

The game's log lines and debug output, written to the module's standard
error, which the runtime keeps per descriptor (opence_runtime's
Game::output), as the Linux port writes them (port/linux/src/xbox_kernel.c).
An assertion's message reaches the platform this way before the game
halts.
*/

#include "platform.h"

#include <stdarg.h>
#include <stdio.h>

void platform_log(const char *format, ...)
{
	va_list arguments;

	fputs("opence: ", stderr);
	va_start(arguments, format);
	vfprintf(stderr, format, arguments);
	va_end(arguments);
	fputc('\n', stderr);
}

VOID WINAPI OutputDebugStringA(LPCSTR string)
{
	if (string)
		fputs(string, stderr);
}
