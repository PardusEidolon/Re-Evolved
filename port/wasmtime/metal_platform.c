/*
METAL_PLATFORM.C

The platform the Metal device (port/wasmtime/metal/d3d8_metal_device.c and the
units beside it) needs inside game.wasm, where the app's Swift renderer
draws its batches: the drawable's size, the contiguous window's blocks, the
frame's start for the memory watch, and none of the high-res HUD, text or
menu art (as the VM's hires stubs did), so that every texture is
the game's own.

The memory watch is the web build's (port/web/src/web_memory_watch.c),
which hashes pages because WebAssembly has no page protection; its frame
starts here, when a frame is shown (Present calls
platform_clock_frame_shown). The window's input is the app's
(opence_runtime's input imports): the menus coming up and going, and the
menus' pointer, are the runtime's, which keeps the pointer as the Linux
port's SDL platform does.
*/

#include "platform.h"
#include "sdl_platform.h"
#include "hud_hires.h"
#include "menu_files.h"
#include "text_hires.h"

/* ---------- the runtime's imports */

/* the drawable's size in pixels (opence_runtime's video imports) */
__attribute__((import_module("env"), import_name("host_drawable_width"))) unsigned int host_drawable_width(void);
__attribute__((import_module("env"), import_name("host_drawable_height"))) unsigned int host_drawable_height(void);

/* the contiguous window's blocks, as the game's own (opence_runtime's
memory module) */
__attribute__((import_module("env"), import_name("XPhysicalAlloc"))) void *XPhysicalAlloc(unsigned long size,
	unsigned long physical_address, unsigned long alignment, unsigned long protect);
__attribute__((import_module("env"), import_name("XPhysicalFree"))) void XPhysicalFree(void *address);

/* the menus coming up or going, and their pointer in the drawable's pixels
(opence_runtime's input imports) */
__attribute__((import_module("env"), import_name("host_menus_active"))) void host_menus_active(BOOL active);
__attribute__((import_module("env"), import_name("host_ui_pointer_read"))) BOOL host_ui_pointer_read(
	struct platform_ui_pointer *pointer);

/* ---------- the window */

BOOL platform_video_initialize(unsigned long width, unsigned long height)
{
	(void)width;
	(void)height;
	return TRUE;
}

void platform_video_drawable_size(int *width, int *height)
{
	*width = (int)host_drawable_width();
	*height = (int)host_drawable_height();
}

/* the menus' pointer is the app's, so window and drawable are one */
void platform_video_window_size(int *width, int *height)
{
	platform_video_drawable_size(width, height);
}

void platform_pump_events(void)
{
}

/* a frame was shown: the memory watch may hash its pages again */
void platform_clock_frame_shown(void)
{
	memory_watch_begin_frame();
}

void platform_menus_set_active(BOOL active)
{
	host_menus_active(active);
}

BOOL platform_ui_pointer_read(struct platform_ui_pointer *pointer)
{
	return host_ui_pointer_read(pointer);
}

void platform_ui_pointer_set_active(BOOL active)
{
	host_menus_active(active);
}

/* ---------- the contiguous window */

void *platform_contiguous_alloc(unsigned long size, unsigned long alignment, unsigned long physical_address,
	DWORD protect)
{
	return XPhysicalAlloc(size, physical_address, alignment, protect);
}

void platform_contiguous_free(void *address)
{
	XPhysicalFree(address);
}

BOOL platform_is_contiguous(const void *address)
{
	unsigned long at = (unsigned long)address;

	return at >= PLATFORM_CONTIGUOUS_BASE && at - PLATFORM_CONTIGUOUS_BASE < PLATFORM_CONTIGUOUS_SIZE;
}

/* ---------- no high-res HUD, text or menu art */

long hud_hires_override_find(unsigned long address, unsigned long width, unsigned long height,
	unsigned long level0_size)
{
	(void)address;
	(void)width;
	(void)height;
	(void)level0_size;
	return -1;
}

unsigned int hud_hires_override_texture(long asset, unsigned long *levels)
{
	(void)asset;
	(void)levels;
	return 0;
}

unsigned long hud_hires_settings_changed(void)
{
	return 0;
}

int hud_hires_bitmap_affected(unsigned long address, long width, long height, unsigned long changed)
{
	(void)address;
	(void)width;
	(void)height;
	(void)changed;
	return 0;
}

int hud_hires_override_coverage(long asset)
{
	(void)asset;
	return 0;
}

int hud_hires_override_point_threshold(long asset)
{
	(void)asset;
	return 0;
}

unsigned int hud_hires_placeholder_texture(unsigned long data, unsigned long *levels)
{
	(void)data;
	(void)levels;
	return 0;
}

unsigned int hud_hires_upscale_texture(const unsigned char *bgra, unsigned long width, unsigned long height,
	long kind, unsigned long *levels)
{
	(void)bgra;
	(void)width;
	(void)height;
	(void)kind;
	(void)levels;
	return 0;
}

unsigned int menu_art_texture(unsigned long data, unsigned long *levels)
{
	(void)data;
	(void)levels;
	return 0;
}
