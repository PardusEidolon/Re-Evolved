/*
SOUND_BUFFER.C (test)

The DirectSound buffers of port/linux/src/dsound_sdl.c (test_sound_buffer.py
takes them as under_test.inc) as the loading screen's chant uses them
(progress_bar.c): set up as a stream's voice is, a buffer plays its data as
one looping packet, at its pitch from 48 kHz and its volume; Stop and
Release end it; a buffer of silence (sound_dsound_xbox.c's) is never mixed.
The voice's own functions (process, flush, release) are recorded, not run:
test_mixer.py hears a looping voice.
*/

#include "harness.h"

#include <math.h>

#include "types.inc"

#define OUTPUT_RATE 48000
#define WAVE_FORMAT_PCM 1
#define WAVE_FORMAT_XBOX_ADPCM 0x0069
#define DSBPLAY_LOOPING 0x00000001
#define DSBFREQUENCY_MIN 188
#define DSBFREQUENCY_MAX 191983
#define DSBVOLUME_MIN (-10000)
#define DS3DMODE_NORMAL 0
#define DS3D_DEFAULTMINDISTANCE 1.0f
#define DS3D_DEFAULTMAXDISTANCE 1000000000.0f
#define DS_OK 0
#define E_OUTOFMEMORY ((HRESULT)0x8007000EL)
#define WINAPI
typedef void *LPUNKNOWN;
typedef const struct _DSBUFFERDESC *LPCDSBUFFERDESC;

static struct sdl_stream *streams;
static int lock_depth, resets, processed, flushed, released;
static XMEDIAPACKET last_packet;
static struct IDirectSoundStreamVtbl *const stream_vtable_value = (struct IDirectSoundStreamVtbl *)0x1234;
#define stream_vtable (*stream_vtable_value)
#define mixer_lock 0
#define pthread_mutex_lock(...) ((void)lock_depth++)
#define pthread_mutex_unlock(...) ((void)lock_depth--)

static void resampler_reset(struct sdl_stream *stream)
{
	(void)stream;
	resets++;
}

static float gain_from_millibels(LONG millibels)
{
	return millibels <= DSBVOLUME_MIN ? 0.0f : powf(10.0f, (float)millibels / 2000.0f);
}

static HRESULT stream_process(IDirectSoundStream *object, const XMEDIAPACKET *input, const XMEDIAPACKET *output)
{
	struct sdl_stream *stream = (struct sdl_stream *)object;

	CHECK(lock_depth == 0, "a packet is queued without the mixer lock held");
	CHECK(output == NULL, "a buffer's packet has no output");
	last_packet = *input;
	stream->packet_count++;
	processed++;
	return 0;
}

static HRESULT stream_flush(IDirectSoundStream *object)
{
	((struct sdl_stream *)object)->packet_count = 0;
	flushed++;
	return 0;
}

static ULONG stream_release(IDirectSoundStream *object)
{
	struct sdl_stream *stream = (struct sdl_stream *)object;

	released++;
	if (--stream->reference_count)
		return stream->reference_count;
	stream_flush(object);
	streams = stream->next;
	free(stream);
	return 0;
}

#include "under_test.inc"

/* progress_bar.c's tone: Xbox ADPCM, mono, 22050 Hz */
static WAVEFORMATEX tone_format = { WAVE_FORMAT_XBOX_ADPCM, 1, 22050, 0, 36, 4, 2 };
static unsigned char tone[36 * 4];

static LPDIRECTSOUNDBUFFER tone_buffer(void)
{
	DSBUFFERDESC description;
	LPDIRECTSOUNDBUFFER buffer = NULL;

	memset(&description, 0, sizeof(description));
	description.lpwfxFormat = &tone_format;
	CHECK(DirectSoundCreateBuffer(&description, &buffer) == DS_OK && buffer, "a buffer is made");
	CHECK(IDirectSoundBuffer_SetBufferData(buffer, tone, sizeof(tone)) == DS_OK, "its data is set");
	return buffer;
}

int main(int argc, char **argv)
{
	CHECK(argc == 2, "case name");
	const char *case_name = argv[1];

	for (unsigned index = 0; index < sizeof(tone); index++)
		tone[index] = (unsigned char)(index * 37 + 1);

	CASE("made")
	{
		struct sdl_stream *stream = (struct sdl_stream *)tone_buffer();

		CHECK(streams == stream, "the mixer hears the buffer's voice");
		CHECK(stream->object.lpVtbl == stream_vtable_value && stream->reference_count == 1, "set up as a stream");
		CHECK(stream->adpcm && stream->channels == 1, "Xbox ADPCM, mono");
		CHECK(stream->sample_rate == 22050 && stream->frequency == 22050, "at its own rate");
		CHECK(stream->volume == 1.0f && stream->mix_left == 1.0f && stream->mix_right == 1.0f, "full volume, both fronts");
		CHECK(!stream->has_3d && resets == 1 && !stream->packet_count, "2D, its resampler reset, silent");
		return 0;
	}
	CASE("plays")
	{
		LPDIRECTSOUNDBUFFER buffer = tone_buffer();
		struct sdl_stream *stream = (struct sdl_stream *)buffer;

		CHECK(IDirectSoundBuffer_Play(buffer, 0, 0, DSBPLAY_LOOPING) == DS_OK, "it plays");
		CHECK(processed == 1 && stream->packet_count == 1, "one packet queued");
		CHECK(last_packet.pvBuffer == tone && last_packet.dwMaxSize == sizeof(tone), "the buffer's data");
		CHECK(last_packet.pdwStatus == NULL && last_packet.pdwCompletedSize == NULL, "with nothing to complete");
		CHECK(stream->looping, "looping");
		IDirectSoundBuffer_Play(buffer, 0, 0, DSBPLAY_LOOPING);
		CHECK(processed == 1, "a buffer playing goes on playing");
		CHECK(IDirectSoundBuffer_Stop(buffer) == DS_OK && !stream->packet_count, "Stop ends it");
		IDirectSoundBuffer_Play(buffer, 0, 0, 0);
		CHECK(processed == 2 && !stream->looping, "played again from its start, once");
		return 0;
	}
	CASE("silent")
	{
		static unsigned char silence[512];
		LPDIRECTSOUNDBUFFER buffer = tone_buffer();

		IDirectSoundBuffer_SetBufferData(buffer, silence, sizeof(silence));
		IDirectSoundBuffer_Play(buffer, 0, 0, DSBPLAY_LOOPING);
		CHECK(processed == 0, "silence is not mixed");
		IDirectSoundBuffer_SetBufferData(buffer, NULL, 0);
		IDirectSoundBuffer_Play(buffer, 0, 0, DSBPLAY_LOOPING);
		CHECK(processed == 0, "nor no data");
		return 0;
	}
	CASE("pitch")
	{
		/* progress_bar.c's four voices */
		static const LONG pitches[] = { 0, -4681, -6386, -3999, -8773, -4096, 4096 };
		static const DWORD expected[] = { 48000, 21737, 16289, 24397, 10876, 24000, 96000 };
		LPDIRECTSOUNDBUFFER buffer = tone_buffer();
		struct sdl_stream *stream = (struct sdl_stream *)buffer;

		for (unsigned index = 0; index < sizeof(pitches) / sizeof(*pitches); index++)
		{
			IDirectSoundBuffer_SetPitch(buffer, pitches[index]);
			CHECK(labs((long)stream->frequency - (long)expected[index]) <= 1, "a pitch is 4096ths of an octave from 48 kHz");
		}
		IDirectSoundBuffer_SetPitch(buffer, -100000);
		CHECK(stream->frequency == DSBFREQUENCY_MIN, "no lower than DirectSound's floor");
		IDirectSoundBuffer_SetPitch(buffer, 100000);
		CHECK(stream->frequency == DSBFREQUENCY_MAX, "no higher than its ceiling");
		CHECK(lock_depth == 0, "the lock let go");
		return 0;
	}
	CASE("volume")
	{
		LPDIRECTSOUNDBUFFER buffer = tone_buffer();
		struct sdl_stream *stream = (struct sdl_stream *)buffer;

		IDirectSoundBuffer_SetVolume(buffer, -5000);
		CHECK(fabsf(stream->volume - powf(10.0f, -2.5f)) < 1e-6f, "hundredths of a decibel");
		IDirectSoundBuffer_SetVolume(buffer, DSBVOLUME_MIN);
		CHECK(stream->volume == 0.0f, "the floor is silence");
		return 0;
	}
	CASE("released")
	{
		LPDIRECTSOUNDBUFFER buffer = tone_buffer();

		IDirectSoundBuffer_Play(buffer, 0, 0, DSBPLAY_LOOPING);
		CHECK(IDirectSoundBuffer_Release(buffer) == 0, "released");
		CHECK(released == 1 && flushed >= 1 && streams == NULL, "its voice flushed and gone");
		return 0;
	}
	CHECK(FALSE, "unknown case");
	return 0;
}
