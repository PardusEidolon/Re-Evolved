//! The part of WASI the runtime supplies itself: the game's and the
//! reference harnesses' output (`fd_write`), its empty environment
//! (`environ_sizes_get`, `environ_get`), a command's arguments and standard
//! input (`args_sizes_get`, `args_get`, `fd_read`) and its sleeps
//! (`poll_oneoff`, under `nanosleep`), read and written through checked
//! ranges.

use std::time::{Duration, Instant};

use opence_runtime::{Error, Runtime};

fn module(text: &str) -> Vec<u8> {
    wat::parse_str(text).unwrap_or_default()
}

/// fd_write(fd, iovs, count, written): the iovecs at 0, the bytes at 100
const WRITER: &str = r#"
(module
  (import "wasi_snapshot_preview1" "fd_write" (func $write (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "\64\00\00\00\06\00\00\00\6a\00\00\00\03\00\00\00")
  (data (i32.const 100) "hello\nbye")
  ;; a vector that reaches past the memory
  (data (i32.const 200) "\fa\ff\00\00\64\00\00\00")
  (func (export "both") (result i32) (call $write (i32.const 1) (i32.const 0) (i32.const 2) (i32.const 50)))
  (func (export "error") (result i32) (call $write (i32.const 2) (i32.const 8) (i32.const 1) (i32.const 50)))
  (func (export "outside") (result i32) (call $write (i32.const 1) (i32.const 200) (i32.const 1) (i32.const 50)))
  (func (export "written_outside") (result i32) (call $write (i32.const 1) (i32.const 0) (i32.const 1) (i32.const 65534))))
"#;

#[test]
fn fd_write_is_kept_per_descriptor_with_the_count_written() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(WRITER))?;
    assert_eq!(game.call_i32("both")?, 0);
    assert_eq!(game.read(50, 4)?, 9u32.to_le_bytes());
    assert_eq!(game.call_i32("error")?, 0);
    assert_eq!(game.output(1), b"hello\nbye");
    assert_eq!(game.output(2), b"bye");
    assert_eq!(game.output(3), b"");
    assert_eq!(game.call_count("wasi_snapshot_preview1.fd_write"), 2);
    Ok(())
}

#[test]
fn fd_write_outside_the_memory_is_efault_and_writes_nothing() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(WRITER))?;
    // WASI's errno 21, EFAULT
    assert_eq!(game.call_i32("outside")?, 21);
    assert_eq!(game.call_i32("written_outside")?, 21);
    assert_eq!(game.output(1), b"");
    Ok(())
}

#[test]
fn a_command_s_exit_code_is_its_result() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
          (func (export "_start") (call $exit (i32.const 0))))"#,
    ))?;
    assert_eq!(game.run_command()?, 0);
    let mut returns = runtime.load(&module(r#"(module (func (export "_start")))"#))?;
    assert_eq!(returns.run_command()?, 0);
    let mut fails = runtime.load(&module(
        r#"(module
          (import "wasi_snapshot_preview1" "proc_exit" (func $exit (param i32)))
          (func (export "_start") (call $exit (i32.const 4))))"#,
    ))?;
    assert_eq!(fails.run_command()?, 4);
    Ok(())
}

#[test]
fn the_environment_is_empty() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(
        r#"(module
          (import "wasi_snapshot_preview1" "environ_sizes_get" (func $sizes (param i32 i32) (result i32)))
          (import "wasi_snapshot_preview1" "environ_get" (func $get (param i32 i32) (result i32)))
          (memory (export "memory") 1)
          (data (i32.const 0) "\ff\ff\ff\ff\ff\ff\ff\ff")
          (func (export "sizes") (result i32) (call $sizes (i32.const 0) (i32.const 4)))
          (func (export "sizes_outside") (result i32) (call $sizes (i32.const 65534) (i32.const 4)))
          (func (export "get") (result i32) (call $get (i32.const 16) (i32.const 32))))"#,
    ))?;
    assert_eq!(game.call_i32("sizes")?, 0);
    assert_eq!(game.read(0, 8)?, [0; 8]);
    assert_eq!(game.call_i32("sizes_outside")?, 21);
    assert_eq!(game.call_i32("get")?, 0);
    Ok(())
}

/// args_sizes_get(count, size) to 0 and 4, args_get(pointers, strings) to 16
/// and 32, and fd_read(fd, iovs, count, read): two iovecs at 200 (3 bytes at
/// 300, 10 at 310), the count read at 8
const COMMAND: &str = r#"
(module
  (import "wasi_snapshot_preview1" "args_sizes_get" (func $sizes (param i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "args_get" (func $get (param i32 i32) (result i32)))
  (import "wasi_snapshot_preview1" "fd_read" (func $read (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 200) "\2c\01\00\00\03\00\00\00\36\01\00\00\0a\00\00\00")
  (func (export "sizes") (result i32) (call $sizes (i32.const 0) (i32.const 4)))
  (func (export "get") (result i32) (call $get (i32.const 16) (i32.const 32)))
  (func (export "get_outside") (result i32) (call $get (i32.const 16) (i32.const 65534)))
  (func (export "read") (result i32) (call $read (i32.const 0) (i32.const 200) (i32.const 2) (i32.const 8)))
  (func (export "read_other") (result i32) (call $read (i32.const 3) (i32.const 200) (i32.const 2) (i32.const 8))))
"#;

#[test]
fn a_command_is_given_its_arguments() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(COMMAND))?;
    game.set_command(&["test.wasm", "case"], Vec::new());
    assert_eq!(game.call_i32("sizes")?, 0);
    assert_eq!(game.read(0, 4)?, 2u32.to_le_bytes());
    assert_eq!(game.read(4, 4)?, 15u32.to_le_bytes());
    assert_eq!(game.call_i32("get")?, 0);
    assert_eq!(game.read(16, 4)?, 32u32.to_le_bytes());
    assert_eq!(game.read(20, 4)?, 42u32.to_le_bytes());
    assert_eq!(game.read(32, 15)?, b"test.wasm\0case\0");
    // EFAULT for strings past the memory
    assert_eq!(game.call_i32("get_outside")?, 21);
    Ok(())
}

#[test]
fn a_command_without_arguments_has_none() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(COMMAND))?;
    assert_eq!(game.call_i32("sizes")?, 0);
    assert_eq!(game.read(0, 8)?, [0; 8]);
    Ok(())
}

#[test]
fn fd_read_reads_standard_input_until_its_end() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(COMMAND))?;
    game.set_command(&["test.wasm"], b"hello world\nbye".to_vec());
    assert_eq!(game.call_i32("read")?, 0);
    assert_eq!(game.read(8, 4)?, 13u32.to_le_bytes());
    assert_eq!(game.read(300, 3)?, b"hel");
    assert_eq!(game.read(310, 10)?, b"lo world\nb");
    assert_eq!(game.call_i32("read")?, 0);
    assert_eq!(game.read(8, 4)?, 2u32.to_le_bytes());
    assert_eq!(game.read(300, 2)?, b"ye");
    // at its end, nothing more
    assert_eq!(game.call_i32("read")?, 0);
    assert_eq!(game.read(8, 4)?, 0u32.to_le_bytes());
    // WASI's errno 8, EBADF, for a descriptor that is not standard input
    assert_eq!(game.call_i32("read_other")?, 8);
    Ok(())
}

/// poll_oneoff(in, out, count, events): at 0 a relative clock subscription
/// of 20 ms on the monotonic clock, at 48 a descriptor's, at 300 one on a
/// clock there is not; the events written at 100, their count at 200
const POLLER: &str = r#"
(module
  (import "wasi_snapshot_preview1" "poll_oneoff" (func $poll (param i32 i32 i32 i32) (result i32)))
  (memory (export "memory") 1)
  (data (i32.const 0) "\88\77\66\55\44\33\22\11\00")
  (data (i32.const 16) "\01\00\00\00\00\00\00\00\00\2d\31\01\00\00\00\00")
  (data (i32.const 48) "\07\00\00\00\00\00\00\00\01")
  (data (i32.const 300) "\09\00\00\00\00\00\00\00\00")
  (data (i32.const 316) "\05\00\00\00")
  (func (export "clock") (result i32) (call $poll (i32.const 0) (i32.const 100) (i32.const 1) (i32.const 200)))
  (func (export "both") (result i32) (call $poll (i32.const 0) (i32.const 100) (i32.const 2) (i32.const 200)))
  (func (export "no_clock") (result i32) (call $poll (i32.const 300) (i32.const 100) (i32.const 1) (i32.const 200)))
  (func (export "none") (result i32) (call $poll (i32.const 0) (i32.const 100) (i32.const 0) (i32.const 200)))
  (func (export "outside") (result i32) (call $poll (i32.const 65500) (i32.const 100) (i32.const 1) (i32.const 200))))
"#;

#[test]
fn poll_oneoff_sleeps_until_its_clock_and_reports_it() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(POLLER))?;
    let start = Instant::now();
    assert_eq!(game.call_i32("clock")?, 0);
    assert!(start.elapsed() >= Duration::from_millis(20));
    assert_eq!(game.read(200, 4)?, 1u32.to_le_bytes());
    // the event: the subscription's userdata, no error, the clock's type
    assert_eq!(game.read(100, 8)?, 0x1122_3344_5566_7788u64.to_le_bytes());
    assert_eq!(game.read(108, 3)?, [0, 0, 0]);
    Ok(())
}

#[test]
fn poll_oneoff_answers_a_descriptor_at_once_with_enotsup() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(POLLER))?;
    let start = Instant::now();
    assert_eq!(game.call_i32("both")?, 0);
    // (the descriptor's event is ready, so the clock is not slept)
    assert!(start.elapsed() < Duration::from_millis(20));
    assert_eq!(game.read(200, 4)?, 1u32.to_le_bytes());
    assert_eq!(game.read(100, 8)?, 7u64.to_le_bytes());
    // WASI's errno 58, ENOTSUP, and the type fd_read
    assert_eq!(game.read(108, 3)?, [58, 0, 1]);
    Ok(())
}

#[test]
fn poll_oneoff_refuses_what_it_cannot_read_or_wait_for() -> Result<(), Error> {
    let runtime = Runtime::new(None)?;
    let mut game = runtime.load(&module(POLLER))?;
    // EINVAL for no subscriptions and for a clock there is not, EFAULT for
    // subscriptions past the memory
    assert_eq!(game.call_i32("none")?, 28);
    assert_eq!(game.call_i32("no_clock")?, 28);
    assert_eq!(game.call_i32("outside")?, 21);
    Ok(())
}
