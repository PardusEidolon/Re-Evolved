//! The thread, synchronisation and clock imports the platform answers
//! (`opence_runtime::threads`), on a small module with a shared memory that
//! starts its threads the way wasi-libc does: `CreateThread` calls the
//! module's `pthread_create`, which calls `wasi.thread-spawn`, and the
//! platform runs the module's `wasi_thread_start` on a thread of its own.

use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use opence_runtime::{answers, Error, Game, Runtime, Val};

/// the module: a shared memory, a `pthread_create` and `wasi_thread_start`
/// as wasi-libc's work (a slot of the routine and its argument, handed to
/// `thread-spawn`), the routines the tests start, and each import
/// re-exported as a function the test calls
const MODULE: &str = r#"
(module
  (import "env" "memory" (memory 4 64 shared))
  (import "wasi" "thread-spawn" (func $spawn (param i32) (result i32)))
  (import "env" "CreateThread" (func $create_thread (param i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "ResumeThread" (func $resume (param i32) (result i32)))
  (import "env" "SetThreadPriority" (func $priority (param i32 i32) (result i32)))
  (import "env" "GetExitCodeThread" (func $exit_code (param i32 i32) (result i32)))
  (import "env" "CreateEventA" (func $create_event (param i32 i32 i32 i32) (result i32)))
  (import "env" "SetEvent" (func $set_event (param i32) (result i32)))
  (import "env" "ResetEvent" (func $reset_event (param i32) (result i32)))
  (import "env" "CreateMutexA" (func $create_mutex (param i32 i32 i32) (result i32)))
  (import "env" "ReleaseMutex" (func $release_mutex (param i32) (result i32)))
  (import "env" "WaitForSingleObject" (func $wait (param i32 i32) (result i32)))
  (import "env" "WaitForSingleObjectEx" (func $wait_ex (param i32 i32 i32) (result i32)))
  (import "env" "Sleep" (func $sleep (param i32)))
  (import "env" "SleepEx" (func $sleep_ex (param i32 i32) (result i32)))
  (import "env" "SwitchToThread" (func $switch (result i32)))
  (import "env" "GetTickCount" (func $ticks (result i32)))
  (import "env" "QueryPerformanceCounter" (func $counter (param i32) (result i32)))
  (import "env" "QueryPerformanceFrequency" (func $frequency (param i32) (result i32)))
  (import "env" "GetSystemTime" (func $system_time (param i32)))
  (import "wasi_snapshot_preview1" "clock_time_get" (func $clock (param i32 i64 i32) (result i32)))
  (import "wasi_snapshot_preview1" "sched_yield" (func $yield (result i32)))
  (import "env" "CloseHandle" (func $close (param i32) (result i32)))
  (import "env" "CreateFileA" (func $create_file (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (import "env" "ReadFileEx" (func $read_ex (param i32 i32 i32 i32 i32) (result i32)))
  (type $routine (func (param i32) (result i32)))
  (table (export "__indirect_function_table") 8 funcref)
  (elem (i32.const 1) $store_seven $wait_then_store $take_mutex $completion $trap)

  ;; a bump allocator for the platform's small blocks
  (global $heap (mut i32) (i32.const 0x8000))
  (func (export "malloc") (param $size i32) (result i32)
    (i32.atomic.rmw.add (i32.const 0xE0) (i32.and (i32.add (local.get $size) (i32.const 15))
                                                   (i32.const -16)))
    (i32.const 0x8000) (i32.add))

  (func (export "pthread_create") (param $out i32) (param $attr i32) (param $fn i32) (param $arg i32)
                                  (result i32)
    (local $slot i32) (local $tid i32)
    (local.set $slot (i32.add (i32.const 0x100)
      (i32.mul (i32.atomic.rmw.add (i32.const 0xF0) (i32.const 1)) (i32.const 8))))
    (i32.store (local.get $slot) (local.get $fn))
    (i32.store offset=4 (local.get $slot) (local.get $arg))
    (local.set $tid (call $spawn (local.get $slot)))
    (if (i32.lt_s (local.get $tid) (i32.const 0)) (then (return (i32.const 11))))
    (i32.store (local.get $out) (local.get $tid))
    (i32.const 0))
  (func (export "wasi_thread_start") (param $tid i32) (param $slot i32)
    (drop (call_indirect (type $routine) (i32.load offset=4 (local.get $slot))
                                         (i32.load (local.get $slot)))))

  ;; stores 7 at its argument
  (func $store_seven (param $at i32) (result i32)
    (i32.atomic.store (local.get $at) (i32.const 7))
    (i32.const 3))
  ;; waits for the event whose handle is at its argument, then stores 1
  ;; after it
  (func $wait_then_store (param $at i32) (result i32)
    (drop (call $wait (i32.load (local.get $at)) (i32.const -1)))
    (i32.atomic.store offset=4 (local.get $at) (i32.const 1))
    (i32.const 0))
  ;; waits for the mutex at its argument for the time after it, stores the
  ;; wait's result plus 1, and releases the mutex if it took it (storing
  ;; what ReleaseMutex gave)
  (func $take_mutex (param $at i32) (result i32)
    (local $result i32)
    (local.set $result (call $wait (i32.load (local.get $at)) (i32.load offset=8 (local.get $at))))
    (if (i32.eqz (local.get $result))
      (then (i32.store offset=12 (local.get $at) (call $release_mutex (i32.load (local.get $at))))))
    (i32.atomic.store offset=4 (local.get $at) (i32.add (local.get $result) (i32.const 1)))
    (i32.const 0))
  ;; a ReadFileEx completion routine: stores its arguments after the
  ;; OVERLAPPED
  (func $completion (param $error i32) (param $moved i32) (param $overlapped i32)
    (i32.store offset=20 (local.get $overlapped) (local.get $error))
    (i32.store offset=24 (local.get $overlapped) (local.get $moved))
    (i32.store offset=28 (local.get $overlapped)
      (i32.add (i32.load offset=28 (local.get $overlapped)) (i32.const 1))))
  (func $trap (param i32) (result i32) unreachable)

  (func (export "create_thread") (param i32 i32 i32) (result i32)
    (call $create_thread (i32.const 0) (i32.const 0x4000) (local.get 0) (local.get 1) (local.get 2)
                         (i32.const 0x40)))
  (func (export "resume") (param i32) (result i32) (call $resume (local.get 0)))
  (func (export "priority") (param i32 i32) (result i32) (call $priority (local.get 0) (local.get 1)))
  (func (export "exit_code") (param i32 i32) (result i32) (call $exit_code (local.get 0) (local.get 1)))
  (func (export "create_event") (param i32 i32) (result i32)
    (call $create_event (i32.const 0) (local.get 0) (local.get 1) (i32.const 0)))
  (func (export "set_event") (param i32) (result i32) (call $set_event (local.get 0)))
  (func (export "reset_event") (param i32) (result i32) (call $reset_event (local.get 0)))
  (func (export "create_mutex") (param i32) (result i32)
    (call $create_mutex (i32.const 0) (local.get 0) (i32.const 0)))
  (func (export "release_mutex") (param i32) (result i32) (call $release_mutex (local.get 0)))
  (func (export "wait") (param i32 i32) (result i32) (call $wait (local.get 0) (local.get 1)))
  (func (export "wait_ex") (param i32 i32 i32) (result i32)
    (call $wait_ex (local.get 0) (local.get 1) (local.get 2)))
  (func (export "sleep") (param i32) (call $sleep (local.get 0)))
  (func (export "sleep_ex") (param i32 i32) (result i32) (call $sleep_ex (local.get 0) (local.get 1)))
  (func (export "switch") (result i32) (call $switch))
  (func (export "yield") (result i32) (call $yield))
  (func (export "ticks") (result i32) (call $ticks))
  (func (export "counter") (param i32) (result i32) (call $counter (local.get 0)))
  (func (export "frequency") (param i32) (result i32) (call $frequency (local.get 0)))
  (func (export "system_time") (param i32) (call $system_time (local.get 0)))
  (func (export "clock") (param i32 i32) (result i32)
    (call $clock (local.get 0) (i64.const 1) (local.get 1)))
  (func (export "close") (param i32) (result i32) (call $close (local.get 0)))
  (func (export "create_file") (param i32) (result i32)
    (call $create_file (local.get 0) (i32.const 0x80000000) (i32.const 1) (i32.const 0)
                       (i32.const 3) (i32.const 0) (i32.const 0)))
  (func (export "read_ex") (param i32 i32 i32 i32 i32) (result i32)
    (call $read_ex (local.get 0) (local.get 1) (local.get 2) (local.get 3) (local.get 4))))
"#;

/// the routines, by their index in the table
const STORE_SEVEN: u32 = 1;
const WAIT_THEN_STORE: u32 = 2;
const TAKE_MUTEX: u32 = 3;
const COMPLETION: u32 = 4;
const TRAP: u32 = 5;

const CREATE_SUSPENDED: u32 = 4;
const INFINITE: u32 = u32::MAX;
const WAIT_OBJECT_0: u32 = 0;
const WAIT_TIMEOUT: u32 = 0x102;
const WAIT_IO_COMPLETION: u32 = 0xC0;
const STILL_ACTIVE: u32 = 259;
const THREAD_PRIORITY_HIGHEST: u32 = 2;

/// where the tests put what the routines read and write
const SHARED: u32 = 0x1000;
const OUT: u32 = 0x2000;
const PATH: u32 = 0x3000;
const BUFFER: u32 = 0x3400;
const OVERLAPPED: u32 = 0x3800;

fn game() -> Result<Game, Error> {
    let bytes = wat::parse_str(MODULE).map_err(|error| Error::Compile(error.to_string()))?;
    Runtime::new(None)?.load(&bytes)
}

/// calls the export `name` with `args`, giving its one result (0 if none)
fn call(game: &mut Game, name: &str, args: &[u32]) -> Result<u32, Error> {
    let args: Vec<Val> = args
        .iter()
        .map(|&arg| Val::I32(arg.cast_signed()))
        .collect();
    let results = game.call_with(name, &args)?;
    Ok(match results.first() {
        Some(Val::I32(value)) => value.cast_unsigned(),
        _ => 0,
    })
}

fn word(game: &Game, address: u32) -> Result<u32, Error> {
    let bytes = game.read(u64::from(address), 4)?;
    let bytes = <[u8; 4]>::try_from(bytes.as_slice()).map_err(|_| Error::Trap("short".into()))?;
    Ok(u32::from_le_bytes(bytes))
}

fn long(game: &Game, address: u32) -> Result<u64, Error> {
    let bytes = game.read(u64::from(address), 8)?;
    let bytes = <[u8; 8]>::try_from(bytes.as_slice()).map_err(|_| Error::Trap("short".into()))?;
    Ok(u64::from_le_bytes(bytes))
}

fn put(game: &mut Game, address: u32, value: u32) -> Result<(), Error> {
    game.write(u64::from(address), &value.to_le_bytes())
}

#[test]
fn create_thread_runs_the_routine_on_a_thread_of_its_own() -> Result<(), Error> {
    let mut game = game()?;
    let thread = call(&mut game, "create_thread", &[STORE_SEVEN, SHARED, 0])?;
    assert_ne!(thread, 0);
    assert_eq!(call(&mut game, "wait", &[thread, INFINITE])?, WAIT_OBJECT_0);
    assert_eq!(word(&game, SHARED)?, 7);
    // ended, its exit code is no longer STILL_ACTIVE
    assert_eq!(call(&mut game, "exit_code", &[thread, OUT])?, 1);
    assert_ne!(word(&game, OUT)?, STILL_ACTIVE);
    assert_eq!(call(&mut game, "close", &[thread])?, 1);
    assert!(game.thread_faults().is_empty());
    Ok(())
}

#[test]
fn a_suspended_thread_waits_for_resume_thread() -> Result<(), Error> {
    let mut game = game()?;
    let thread = call(
        &mut game,
        "create_thread",
        &[STORE_SEVEN, SHARED, CREATE_SUSPENDED],
    )?;
    assert_ne!(thread, 0);
    assert_eq!(
        call(&mut game, "priority", &[thread, THREAD_PRIORITY_HIGHEST])?,
        1
    );
    assert_eq!(call(&mut game, "wait", &[thread, 50])?, WAIT_TIMEOUT);
    assert_eq!(call(&mut game, "exit_code", &[thread, OUT])?, 1);
    assert_eq!(word(&game, OUT)?, STILL_ACTIVE);
    assert_eq!(word(&game, SHARED)?, 0);
    // ResumeThread gives the suspend count it had
    assert_eq!(call(&mut game, "resume", &[thread])?, 1);
    assert_eq!(call(&mut game, "wait", &[thread, INFINITE])?, WAIT_OBJECT_0);
    assert_eq!(word(&game, SHARED)?, 7);
    assert_eq!(call(&mut game, "resume", &[thread])?, 0);
    Ok(())
}

#[test]
fn a_thread_waits_for_an_event_another_sets() -> Result<(), Error> {
    let mut game = game()?;
    let event = call(&mut game, "create_event", &[0, 0])?;
    assert_ne!(event, 0);
    put(&mut game, SHARED, event)?;
    let thread = call(&mut game, "create_thread", &[WAIT_THEN_STORE, SHARED, 0])?;
    assert_eq!(call(&mut game, "wait", &[thread, 50])?, WAIT_TIMEOUT);
    assert_eq!(word(&game, SHARED + 4)?, 0);
    assert_eq!(call(&mut game, "set_event", &[event])?, 1);
    assert_eq!(call(&mut game, "wait", &[thread, INFINITE])?, WAIT_OBJECT_0);
    assert_eq!(word(&game, SHARED + 4)?, 1);
    // the thread's wait took the auto-reset event's signal
    assert_eq!(call(&mut game, "wait", &[event, 0])?, WAIT_TIMEOUT);
    Ok(())
}

#[test]
fn a_manual_reset_event_stays_set_until_reset() -> Result<(), Error> {
    let mut game = game()?;
    // manual reset, set from the start
    let manual = call(&mut game, "create_event", &[1, 1])?;
    assert_eq!(call(&mut game, "wait", &[manual, 0])?, WAIT_OBJECT_0);
    assert_eq!(call(&mut game, "wait", &[manual, 0])?, WAIT_OBJECT_0);
    assert_eq!(call(&mut game, "reset_event", &[manual])?, 1);
    assert_eq!(call(&mut game, "wait", &[manual, 0])?, WAIT_TIMEOUT);
    // auto reset: one wait takes the signal
    let auto = call(&mut game, "create_event", &[0, 1])?;
    assert_eq!(call(&mut game, "wait", &[auto, 0])?, WAIT_OBJECT_0);
    assert_eq!(call(&mut game, "wait", &[auto, 10])?, WAIT_TIMEOUT);
    // a handle that is nothing fails the wait
    assert_eq!(call(&mut game, "wait", &[0x1234, 0])?, u32::MAX);
    assert_eq!(call(&mut game, "set_event", &[0x1234])?, 0);
    assert_eq!(call(&mut game, "close", &[auto])?, 1);
    assert_eq!(call(&mut game, "wait", &[auto, 0])?, u32::MAX);
    Ok(())
}

#[test]
fn a_mutex_is_held_by_one_thread_at_a_time() -> Result<(), Error> {
    let mut game = game()?;
    // owned by the main thread from the start, and taken again (it counts)
    let mutex = call(&mut game, "create_mutex", &[1])?;
    assert_ne!(mutex, 0);
    assert_eq!(call(&mut game, "wait", &[mutex, 0])?, WAIT_OBJECT_0);

    // another thread cannot take it
    put(&mut game, SHARED, mutex)?;
    put(&mut game, SHARED + 8, 0)?;
    let first = call(&mut game, "create_thread", &[TAKE_MUTEX, SHARED, 0])?;
    assert_eq!(call(&mut game, "wait", &[first, INFINITE])?, WAIT_OBJECT_0);
    assert_eq!(word(&game, SHARED + 4)?, WAIT_TIMEOUT + 1);

    // released once it is still held; released twice it is free
    assert_eq!(call(&mut game, "release_mutex", &[mutex])?, 1);
    put(&mut game, SHARED + 4, 0)?;
    put(&mut game, SHARED + 8, INFINITE)?;
    let second = call(&mut game, "create_thread", &[TAKE_MUTEX, SHARED, 0])?;
    assert_eq!(call(&mut game, "wait", &[second, 50])?, WAIT_TIMEOUT);
    assert_eq!(call(&mut game, "release_mutex", &[mutex])?, 1);
    assert_eq!(call(&mut game, "wait", &[second, INFINITE])?, WAIT_OBJECT_0);
    assert_eq!(word(&game, SHARED + 4)?, WAIT_OBJECT_0 + 1);
    // the thread released what it took
    assert_eq!(word(&game, SHARED + 12)?, 1);
    // and a thread that does not hold it cannot release it
    assert_eq!(call(&mut game, "release_mutex", &[mutex])?, 0);
    Ok(())
}

#[test]
fn an_alertable_wait_runs_the_completions_waiting() -> Result<(), Error> {
    let folder = std::env::temp_dir().join(format!("opence_threads_{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).map_err(|error| Error::Setup(error.to_string()))?;
    fs::write(folder.join("sounds.map"), b"abcdef")
        .map_err(|error| Error::Setup(error.to_string()))?;
    let mut game = game()?;
    game.set_folders(&folder, &folder);
    game.write(u64::from(PATH), b"d:\\sounds.map\0")?;
    let file = call(&mut game, "create_file", &[PATH])?;
    assert_ne!(file, u32::MAX);
    game.write(u64::from(OVERLAPPED), &[0; 32])?;
    assert_eq!(
        call(
            &mut game,
            "read_ex",
            &[file, BUFFER, 4, OVERLAPPED, COMPLETION]
        )?,
        1
    );

    // a wait that is not alertable leaves it waiting
    assert_eq!(call(&mut game, "sleep_ex", &[0, 0])?, 0);
    assert_eq!(word(&game, OVERLAPPED + 28)?, 0);
    // an alertable one runs it, and says so
    assert_eq!(call(&mut game, "sleep_ex", &[5000, 1])?, WAIT_IO_COMPLETION);
    assert_eq!(word(&game, OVERLAPPED + 20)?, 0);
    assert_eq!(word(&game, OVERLAPPED + 24)?, 4);
    assert_eq!(word(&game, OVERLAPPED + 28)?, 1);
    assert!(game.completions().is_empty());

    // as an alertable WaitForSingleObjectEx does, before it waits
    let event = call(&mut game, "create_event", &[0, 0])?;
    assert_eq!(
        call(
            &mut game,
            "read_ex",
            &[file, BUFFER, 4, OVERLAPPED, COMPLETION]
        )?,
        1
    );
    assert_eq!(
        call(&mut game, "wait_ex", &[event, 5000, 1])?,
        WAIT_IO_COMPLETION
    );
    assert_eq!(word(&game, OVERLAPPED + 28)?, 2);
    assert_eq!(call(&mut game, "wait_ex", &[event, 10, 1])?, WAIT_TIMEOUT);
    let _ = fs::remove_dir_all(&folder);
    Ok(())
}

#[test]
fn a_thread_that_traps_ends_and_is_reported() -> Result<(), Error> {
    let mut game = game()?;
    let thread = call(&mut game, "create_thread", &[TRAP, SHARED, 0])?;
    assert_eq!(call(&mut game, "wait", &[thread, INFINITE])?, WAIT_OBJECT_0);
    let faults = game.thread_faults();
    assert_eq!(faults.len(), 1);
    assert!(
        faults.iter().any(|fault| fault.contains("unreachable")),
        "{faults:?}"
    );
    Ok(())
}

#[test]
fn the_clocks_move_on_with_time() -> Result<(), Error> {
    let mut game = game()?;
    assert_eq!(call(&mut game, "frequency", &[OUT])?, 1);
    assert_eq!(long(&game, OUT)?, 10_000_000);

    let ticks = call(&mut game, "ticks", &[])?;
    assert_eq!(call(&mut game, "counter", &[OUT])?, 1);
    let counter = long(&game, OUT)?;
    assert_eq!(call(&mut game, "clock", &[1, OUT + 8])?, 0);
    let monotonic = long(&game, OUT + 8)?;
    call(&mut game, "sleep", &[20])?;
    assert_eq!(call(&mut game, "switch", &[])?, 1);
    // wasi-libc's spin loops yield, and the platform answers that
    assert!(answers("wasi_snapshot_preview1.sched_yield"));
    assert_eq!(call(&mut game, "yield", &[])?, 0);
    assert!(call(&mut game, "ticks", &[])?.wrapping_sub(ticks) >= 20);
    call(&mut game, "counter", &[OUT])?;
    assert!(long(&game, OUT)? - counter >= 200_000);
    call(&mut game, "clock", &[1, OUT + 8])?;
    assert!(long(&game, OUT + 8)? - monotonic >= 20_000_000);

    // the real time, in nanoseconds since 1970
    assert_eq!(call(&mut game, "clock", &[0, OUT])?, 0);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| Error::Setup(error.to_string()))?;
    let real = Duration::from_nanos(long(&game, OUT)?);
    assert!(now.abs_diff(real) < Duration::from_secs(5));
    // an unknown clock is EINVAL
    assert_eq!(call(&mut game, "clock", &[9, OUT])?, 28);

    // GetSystemTime: the same moment as SYSTEMTIME's eight words
    call(&mut game, "system_time", &[OUT])?;
    let words: Vec<u16> = game
        .read(u64::from(OUT), 16)?
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect();
    let seconds = now.as_secs();
    let days = seconds / 86_400;
    assert!(words[0] >= 2026);
    assert!((1..=12).contains(&words[1]));
    // 1 January 1970 was a Thursday
    assert_eq!(u64::from(words[2]), (days + 4) % 7);
    assert!((1..=31).contains(&words[3]));
    let of_day = u64::from(words[4]) * 3600 + u64::from(words[5]) * 60 + u64::from(words[6]);
    assert!((seconds % 86_400).abs_diff(of_day) < 5);
    assert!(words[7] < 1000);
    Ok(())
}
