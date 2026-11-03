//! The game's mixed sound on its way to the speakers
//! (`opence_runtime::Speakers`): the mixer (`port/linux/src/dsound_sdl.c`)
//! runs on a thread of the game's and hands its frames, 48 kHz stereo
//! floats, to `halo_audio_output`, which keeps them in a ring and answers
//! how many more to mix; the app pulls them from the ring on Core Audio's
//! thread. While nothing pulls, the ring drains at the output's rate, so
//! the game's streams still play out in time.

use std::time::{Duration, Instant};

use opence_runtime::{Error, Game, Runtime, Val, SPEAKER_RATE, SPEAKER_TARGET_FRAMES};

/// the module: the mixer's call, and a page to mix into
const MODULE: &str = r#"
(module
  (import "env" "halo_audio_output" (func $output (param i32 i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "output") (param i32 i32) (result i32)
    (call $output (local.get 0) (local.get 1))))
"#;

const FRAMES: u32 = 0x1000;

fn game() -> Result<Game, Error> {
    let bytes = wat::parse_str(MODULE).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

fn output(game: &mut Game, address: u32, count: u32) -> Result<u32, Error> {
    let args = [
        Val::I32(address.cast_signed()),
        Val::I32(count.cast_signed()),
    ];
    Ok(match game.call_with("output", &args)?.as_slice() {
        [Val::I32(value)] => value.cast_unsigned(),
        _ => 0,
    })
}

/// writes `frames` stereo frames, left `start + i` and right `-(start + i)`
fn mixed(game: &mut Game, start: usize, frames: usize) -> Result<(), Error> {
    let bytes: Vec<u8> = (start..start + frames)
        .flat_map(|frame| {
            let value = f32::from(u16::try_from(frame).unwrap_or(u16::MAX));
            [value, -value]
        })
        .flat_map(f32::to_le_bytes)
        .collect();
    game.write(u64::from(FRAMES), &bytes)
}

#[test]
fn the_mixer_is_first_asked_for_the_ring_s_target() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(output(&mut game, FRAMES, 0)?, SPEAKER_TARGET_FRAMES);
    Ok(())
}

#[test]
fn the_speakers_play_what_the_mixer_wrote_then_silence() -> Result<(), Error> {
    let mut game = game()?;
    let speakers = game.speakers();
    mixed(&mut game, 1, 4)?;
    output(&mut game, FRAMES, 4)?;
    let mut buffer = [9.0_f32; 12];
    assert_eq!(speakers.pull(&mut buffer), 4);
    assert_eq!(
        buffer,
        [1.0, -1.0, 2.0, -2.0, 3.0, -3.0, 4.0, -4.0, 0.0, 0.0, 0.0, 0.0]
    );
    Ok(())
}

#[test]
fn the_mixer_is_asked_for_what_the_speakers_took() -> Result<(), Error> {
    let mut game = game()?;
    let speakers = game.speakers();
    let target = SPEAKER_TARGET_FRAMES as usize;
    mixed(&mut game, 0, target)?;
    // (the speakers are pulling, so the clock leaves the ring to them)
    speakers.pull(&mut []);
    let puller = std::thread::spawn(move || {
        let mut buffer = vec![0.0_f32; 512 * 2];
        let started = Instant::now();
        // (the game blocks until the ring has room: this takes it once)
        while speakers.pull(&mut buffer) == 0 && started.elapsed() < Duration::from_secs(5) {
            std::thread::yield_now();
        }
    });
    let wanted = output(&mut game, FRAMES, SPEAKER_TARGET_FRAMES)?;
    assert!(puller.join().is_ok());
    assert_eq!(wanted, 512);
    Ok(())
}

#[test]
fn with_nothing_pulling_the_ring_drains_at_the_output_s_rate() -> Result<(), Error> {
    let mut game = game()?;
    let target = SPEAKER_TARGET_FRAMES as usize;
    mixed(&mut game, 0, target)?;
    let started = Instant::now();
    output(&mut game, FRAMES, SPEAKER_TARGET_FRAMES)?;
    let wanted = output(&mut game, FRAMES, 0)?;
    // (a full ring waits for at least a few milliseconds of it to play)
    assert!(started.elapsed() >= Duration::from_millis(2));
    assert!(wanted > 0 && wanted <= SPEAKER_TARGET_FRAMES);
    assert_eq!(SPEAKER_RATE, 48_000);
    Ok(())
}

#[test]
fn frames_outside_the_memory_are_an_error() -> Result<(), Error> {
    let mut game = game()?;
    assert!(output(&mut game, 0xFFF0, 4).is_err());
    Ok(())
}
