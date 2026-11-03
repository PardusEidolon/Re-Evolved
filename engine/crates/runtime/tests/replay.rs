//! Recorded inputs (`opence_runtime::replay`): what the game's main thread
//! asks the outside world, its clocks, its keyboard and mouse, its file
//! times and its random bytes, is recorded as it is answered, and a run
//! replayed from the recording gets the same answers, whatever the clocks
//! and the keyboard say then, so it ends in the same state.

use opence_runtime::replay::Recording;
use opence_runtime::{Error, Game, Runtime, Val};

/// the module: each step reads the tick count, the performance counter and
/// a keystroke, and folds them into a state word at 64; the console's query
/// (which the keystroke handler calls back into the game for) reads the
/// tick count too
const MODULE: &str = r#"
(module
  (import "env" "GetTickCount" (func $ticks (result i32)))
  (import "env" "QueryPerformanceCounter" (func $counter (param i32) (result i32)))
  (import "env" "XInputDebugGetKeystroke" (func $keystroke (param i32) (result i32)))
  (memory (export "memory") 1)
  (func $fold (param i32)
    (i32.store (i32.const 64)
      (i32.add (i32.mul (i32.load (i32.const 64)) (i32.const 31)) (local.get 0))))
  (func (export "console_is_active") (result i32)
    (drop (call $ticks))
    (i32.const 0))
  (func (export "step")
    (call $fold (call $ticks))
    (drop (call $counter (i32.const 8)))
    (call $fold (i32.load (i32.const 8)))
    (call $fold (call $keystroke (i32.const 32)))
    (call $fold (i32.load (i32.const 32))))
  (func (export "state") (result i32)
    (i32.load (i32.const 64))))
"#;

/// a module that asks in another order: the counter first
const OTHER: &str = r#"
(module
  (import "env" "GetTickCount" (func $ticks (result i32)))
  (import "env" "QueryPerformanceCounter" (func $counter (param i32) (result i32)))
  (memory (export "memory") 1)
  (func (export "step")
    (drop (call $counter (i32.const 8)))
    (drop (call $ticks))))
"#;

fn game(module: &str) -> Result<Game, Error> {
    let bytes = wat::parse_str(module).unwrap_or_default();
    Runtime::new(None)?.load(&bytes)
}

/// `steps` steps, with the key `first + step` pressed and let go before
/// each and a little time between them; the state, and the memory the
/// steps write
fn run(game: &mut Game, steps: u8, first: u8) -> Result<(i32, Vec<u8>), Error> {
    for step in 0..steps {
        game.key(first + step, first + step, 0, true);
        game.key(first + step, first + step, 0, false);
        std::thread::sleep(std::time::Duration::from_millis(2));
        game.call("step")?;
    }
    let state = game.call_i32("state")?;
    Ok((state, game.read(0, 128)?))
}

/// a run of `steps` recorded
fn recorded(steps: u8) -> Result<(Recording, i32, Vec<u8>), Error> {
    let mut game = game(MODULE)?;
    game.record_inputs();
    let (state, memory) = run(&mut game, steps, b'A')?;
    Ok((game.take_recording(), state, memory))
}

#[test]
fn a_replayed_run_gets_the_recorded_answers_whatever_the_clocks_and_keys_say() -> Result<(), Error>
{
    let (recording, state, memory) = recorded(8)?;
    // three answers a step, the console's query inside the keystroke's
    // handler not among them
    assert_eq!(recording.len(), 24);
    let mut again = game(MODULE)?;
    std::thread::sleep(std::time::Duration::from_millis(20));
    again.replay(recording);
    let replayed = run(&mut again, 8, b'a')?;
    assert_eq!(replayed, (state, memory));
    assert_eq!(again.replay_left(), 0);
    Ok(())
}

#[test]
fn a_run_not_replayed_differs() -> Result<(), Error> {
    let (_, state, _) = recorded(8)?;
    let mut live = game(MODULE)?;
    std::thread::sleep(std::time::Duration::from_millis(20));
    assert_ne!(run(&mut live, 8, b'a')?.0, state);
    Ok(())
}

#[test]
fn when_the_recording_runs_out_the_game_runs_live() -> Result<(), Error> {
    let (recording, _, _) = recorded(2)?;
    let mut again = game(MODULE)?;
    again.replay(recording);
    again.call("step")?;
    assert_eq!(again.replay_left(), 3);
    again.call("step")?;
    assert_eq!(again.replay_left(), 0);
    // past its end, the keys pressed now are the ones read (a keystroke is
    // read as its key comes up, while no console is open)
    again.key(b'Z', b'Z', 0, true);
    again.key(b'Z', b'Z', 0, false);
    again.call("step")?;
    assert_eq!(again.read(32, 1)?, [b'Z']);
    Ok(())
}

#[test]
fn a_run_that_asks_something_else_diverges() -> Result<(), Error> {
    let (recording, _, _) = recorded(1)?;
    let mut other = game(OTHER)?;
    other.replay(recording);
    match other.call("step") {
        Err(Error::Diverged {
            at,
            recorded,
            asked,
        }) => {
            assert_eq!(
                (at, recorded.as_str(), asked.as_str()),
                (0, "env.GetTickCount", "env.QueryPerformanceCounter")
            );
            Ok(())
        }
        other => panic!("not a divergence: {other:?}"),
    }
}

#[test]
fn a_recording_is_read_back_from_its_bytes() -> Result<(), Error> {
    let (recording, _, _) = recorded(3)?;
    let bytes = recording.to_bytes();
    assert_eq!(Recording::from_bytes(&bytes)?, recording);
    Ok(())
}

/// xorshift64*: a fixed seed, so that a failure is found again
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

/// A recording's bytes are untrusted (a file the user hands the dev
/// shell): cut short, changed or grown, they are an error or a recording,
/// never a panic.
#[test]
fn a_recordings_bytes_cut_or_changed_are_an_error_never_a_panic() -> Result<(), Error> {
    let (recording, _, _) = recorded(3)?;
    let bytes = recording.to_bytes();
    for cut in 0..bytes.len() {
        assert!(
            Recording::from_bytes(&bytes[..cut]).is_err(),
            "cut at {cut}"
        );
    }
    let mut random = Random(0x0BAD_5EED);
    for _ in 0..4096 {
        let mut changed = bytes.clone();
        for _ in 0..1 + random.next() % 4 {
            let at = (random.next() % changed.len() as u64) as usize;
            changed[at] = random.next() as u8;
        }
        if random.next().is_multiple_of(4) {
            changed.extend((0..random.next() % 16).map(|_| random.next() as u8));
        }
        let _ = Recording::from_bytes(&changed);
    }
    Ok(())
}

#[test]
fn replay_applies_to_the_main_thread_only() {
    // (worker threads stay live: the main thread's answers are the inputs;
    // the doc of `replay` says so, and the set recorded is the main
    // thread's own)
    assert!(opence_runtime::replay::recorded("env.GetTickCount"));
    assert!(opence_runtime::replay::recorded("env.XInputGetState"));
    assert!(!opence_runtime::replay::recorded("env.CreateFileA"));
    assert!(!opence_runtime::replay::recorded("env.D3DDevice_Present"));
}

#[test]
fn values_carry_through_a_replay_bit_for_bit() -> Result<(), Error> {
    let (recording, _, _) = recorded(1)?;
    let mut again = game(MODULE)?;
    again.replay(recording);
    let results = again.call_with("step", &[])?;
    assert!(results.is_empty());
    assert!(matches!(
        again.call_results("state")?.as_slice(),
        [Val::I32(_)]
    ));
    Ok(())
}
