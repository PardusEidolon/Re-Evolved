/*
PLATFORM_DATA.C

The variables of the Xbox's libraries that the game reads and writes
directly, not through a call: the input library's device types, whose
addresses name a kind of device. A variable can't be imported, so these
live in game.wasm. (Direct3D's render and texture states are the Metal
device's, port/wasmtime/metal/d3d8_metal_device.c.) The Linux port defines
the same ones (port/linux/src/xinput_sdl.c), and platform.h includes the
SDK's headers as its platform layer does.
*/

#include "platform.h"

/* ---------- input */

/* only their addresses are used: XGetDevices(XDEVICE_TYPE_GAMEPAD) and the
rest */
XPP_DEVICE_TYPE XDEVICE_TYPE_GAMEPAD_TABLE;
XPP_DEVICE_TYPE XDEVICE_TYPE_MEMORY_UNIT_TABLE;
XPP_DEVICE_TYPE XDEVICE_TYPE_DEBUG_KEYBOARD_TABLE;
