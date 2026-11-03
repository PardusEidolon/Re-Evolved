"""The DirectSound buffers of dsound_sdl.c as the loading screen's chant uses them (sound_buffer.c): a looping voice
of the buffer's data at its pitch from 48 kHz and its volume, never one of silence."""

import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from harness import CHECK_FAILED, build, function, mutated, read, run, structure  # noqa: E402
from harness.audio import stream_types  # noqa: E402

SOURCE = "port/linux/src/dsound_sdl.c"
CASES = ["made", "plays", "silent", "pitch", "volume", "released"]
FUNCTIONS = [
    "stream_initialize",
    "buffer_from_interface",
    "buffer_heard",
    "frequency_from_pitch",
    "DirectSoundCreateBuffer",
    "IDirectSoundBuffer_Release",
    "IDirectSoundBuffer_SetBufferData",
    "IDirectSoundBuffer_Play",
    "IDirectSoundBuffer_Stop",
    "IDirectSoundBuffer_SetPitch",
    "IDirectSoundBuffer_SetVolume",
]

# a fault in the buffers, and the case that must catch it
NEGATIVE_CONTROLS = {
    "silence-mixed": (("return FALSE;\n}", "return TRUE;\n}"), "silent"),
    "pitch-from-own-rate": (("OUTPUT_RATE * exp2f", "22050 * exp2f"), "pitch"),
    "not-looping": (("(flags & DSBPLAY_LOOPING) != 0", "FALSE"), "plays"),
    "restarts-playing": (("playing = record->stream.packet_count != 0;", "playing = FALSE;"), "plays"),
}


def generated(fault=None):
    backend = read(SOURCE)
    xdk = read("port/include/xdk/xdk_pdb.h")
    types = stream_types()
    types += "#pragma pack(push, 2)\n" + structure(xdk, "tWAVEFORMATEX") + "\n#pragma pack(pop)\n"
    types += structure(xdk, "_DSBUFFERDESC") + "\n"
    types += "typedef struct tWAVEFORMATEX WAVEFORMATEX;\ntypedef struct _DSBUFFERDESC DSBUFFERDESC;\n"
    code = structure(backend, "sdl_buffer") + "\n" + "\n".join(function(backend, name) for name in FUNCTIONS)
    if fault:
        code = mutated(code, *fault)
    return (("types.inc", types), ("under_test.inc", code))


@pytest.mark.parametrize("case", CASES)
def test_case(case):
    status, output = run(build("sound_buffer", generated()), case)
    assert status == 0, output


@pytest.mark.parametrize("control", NEGATIVE_CONTROLS)
def test_negative_control(control):
    fault, case = NEGATIVE_CONTROLS[control]
    status, output = run(build("sound_buffer", generated(fault)), case)
    assert status == CHECK_FAILED, f"{control} escaped {case}: {output}"
