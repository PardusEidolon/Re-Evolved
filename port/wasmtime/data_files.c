/*
DATA_FILES.C

A whole file read through the platform's file imports (CreateFileA,
GetFileSize, ReadFile, opence_runtime's files module), for crt.c's fopen,
which has no files of its own: the stdio files the game reads (the
director's d:\camera.txt and the rest) are read whole into memory, where
fscanf and fgets read them, as the Linux port opens them by their
translated path (port/linux/src/msvc_crt.c). Compiled with the platform's
headers, which crt.c is not.
*/

#include "platform.h"

#include <stdlib.h>

/* the file at path, whole, in a buffer of malloc's the caller frees; NULL
when it can't be opened or read */
char *halo_data_file_read(const char *path, unsigned long *length)
{
	HANDLE file;
	DWORD size;
	DWORD read = 0;
	char *text;

	*length = 0;
	file = CreateFileA(path, GENERIC_READ, 0, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
	if (file == INVALID_HANDLE_VALUE)
		return NULL;
	size = GetFileSize(file, NULL);
	if (size == INVALID_FILE_SIZE)
	{
		CloseHandle(file);
		return NULL;
	}
	/* one more byte, so that an empty file is still a buffer */
	text = malloc((size_t)size + 1);
	if (!text)
	{
		CloseHandle(file);
		return NULL;
	}
	if (size && (!ReadFile(file, text, size, &read, NULL) || read != size))
	{
		free(text);
		CloseHandle(file);
		return NULL;
	}
	CloseHandle(file);
	*length = size;
	return text;
}
