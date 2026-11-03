//! The game's threads, the objects they wait on and the clocks, as the
//! imports the platform answers.
//!
//! A threaded game imports its memory, shared, and starts its threads the
//! way wasi-libc does: `CreateThread` calls the game's own `pthread_create`,
//! which calls `wasi.thread-spawn`, and the platform instantiates the game
//! again on a thread of its own, over the same memory, and calls its
//! `wasi_thread_start`. Each thread has its own store and record; what they
//! share (the window, the files, these objects) is in [`super::Shared`].
//!
//! Events, mutexes and threads are objects in one table behind one lock,
//! with one condition variable that every change wakes: a wait looks again
//! at what it waits for each time it wakes, and takes it when it can (an
//! auto-reset event resets, a mutex is owned once more). The game has a
//! handful of threads, so one lock is enough.

use std::collections::BTreeMap;
use std::sync::{Condvar, Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use wasmtime::{Caller, Extern, InstancePre, Ref, Store, Val};

use super::files;
use super::memory::{arg, call_game, give, Handler};
use super::{with_space, Record};

/// `CreateThread`'s flag for a thread that waits for `ResumeThread`
const CREATE_SUSPENDED: u32 = 4;
/// a wait without end
const INFINITE: u32 = u32::MAX;
/// what a wait gives: it took what it waited for, it ran out of time, it
/// ran completions, or the handle is nothing it can wait for
const WAIT_OBJECT_0: u32 = 0;
const WAIT_TIMEOUT: u32 = 0x102;
const WAIT_IO_COMPLETION: u32 = 0xC0;
const WAIT_FAILED: u32 = u32::MAX;
/// `GetExitCodeThread`'s code for a thread that has not ended
const STILL_ACTIVE: u32 = 259;
/// the first object handle, and the step between them (apart from the
/// files' handles, which start at 0x100)
const FIRST_HANDLE: u32 = 0x4000_0000;
const HANDLE_STEP: u32 = 4;
/// the stack of a game thread's host thread: room for the wasm stack and
/// the host calls under it
const HOST_STACK: usize = 8 << 20;
/// `QueryPerformanceCounter`'s ticks per second (100 ns each)
const PERFORMANCE_FREQUENCY: u64 = 10_000_000;
/// WASI's errors
const EFAULT: u32 = 21;
const EINVAL: u32 = 28;
/// WASI's errno ENOTSUP
const ENOTSUP: u16 = 58;
/// a WASI subscription's size, and its clock's place in it: the clock's id,
/// its timeout and its flags (`subscription_clock`)
const SUBSCRIPTION: u32 = 48;
const CLOCK_ID: u32 = 16;
const CLOCK_TIMEOUT: u32 = 24;
const CLOCK_FLAGS: u32 = 40;
/// a clock's timeout is a time on the clock, not one from now
const ABSTIME: u8 = 1;
/// a WASI event's size
const EVENT: u32 = 32;

/// something the game's threads wait on
#[derive(Debug)]
enum Object {
    /// a thread: signalled once it has ended
    Thread {
        /// how many `ResumeThread`s it waits for before it starts
        suspend: u32,
        ended: bool,
        /// its handle is closed, and it is removed when it ends
        closed: bool,
        /// its id, once `thread-spawn` gave it one
        tid: u32,
    },
    /// an event: signalled while set
    Event { manual: bool, set: bool },
    /// a mutex: signalled while no thread but the waiter owns it
    Mutex { owner: Option<u32>, count: u32 },
}

impl Object {
    /// takes it for `thread` if it is signalled
    fn take(&mut self, thread: u32) -> bool {
        match self {
            Self::Thread { ended, .. } => *ended,
            Self::Event { manual, set } => {
                let signalled = *set;
                if !*manual {
                    *set = false;
                }
                signalled
            }
            Self::Mutex { owner, count } => match owner {
                Some(holder) if *holder != thread => false,
                _ => {
                    *owner = Some(thread);
                    *count += 1;
                    true
                }
            },
        }
    }
}

/// the objects, by handle
#[derive(Debug)]
struct Table {
    objects: BTreeMap<u32, Object>,
    next: u32,
    /// the next thread id (the game's first thread is 0)
    next_tid: u32,
    /// the game is going: every wait ends
    closing: bool,
}

/// what the game's threads share of this module
pub(crate) struct Threads {
    table: Mutex<Table>,
    /// woken by every change to the table
    changed: Condvar,
    /// what each thread that trapped said
    faults: Mutex<Vec<String>>,
    /// the game, ready to instantiate on a new thread (a threaded game only)
    spawner: OnceLock<InstancePre<Record>>,
    /// when the game was loaded: the clocks count from it
    start: Instant,
    /// `debug.frame_clock`'s steps
    frame_clock: Mutex<FrameClock>,
}

/// a 60th of a second, the frame clock's step
const FRAME_STEP: u64 = 16_666_667;
/// how long without a frame shown before the frame clock steps with the
/// real time (a map loading)
const FRAME_DARK: u64 = 500_000_000;

/// `debug.frame_clock` (as `port/linux`'s): from the first frame shown the
/// game's clocks (`GetTickCount`, `QueryPerformanceCounter`) step a 60th of a second a frame, whatever the time between them,
/// so that frames captured for tests are the same run to run; while no
/// frame is shown for more than half a second they step with the real
/// time, so that the game's waits end.
#[derive(Debug, Default)]
struct FrameClock {
    /// None until the first frame shown reads the setting; then whether on
    on: Option<bool>,
    /// the real time of the first frame shown
    start: u64,
    /// the steps taken since
    steps: u64,
    /// the real time of the last frame shown
    shown_at: u64,
}

impl FrameClock {
    /// a clock on from a first frame shown at `now`, which it counts from
    /// the whole millisecond before (the fraction past it is chance, and
    /// would move the 16 and 17 ms steps of a 60th onto other frames)
    fn starting(now: u64) -> Self {
        let start = now - now % 1_000_000;
        Self {
            on: Some(true),
            start,
            steps: 0,
            shown_at: now,
        }
    }

    /// the steps taken in the dark since the last frame shown, at `now`:
    /// whole threes of them, so that play goes on from the same third of
    /// the clock's 50 ms cycle however long the dark lasted
    fn dark_steps(&self, now: u64) -> u64 {
        now.saturating_sub(self.shown_at)
            .checked_sub(FRAME_DARK)
            .map_or(0, |dark| dark / FRAME_STEP / 3 * 3)
    }

    /// a frame shown at `now`
    fn shown(&mut self, now: u64) {
        let steps = self.dark_steps(now) + 1;
        self.steps = self.steps.saturating_add(steps);
        self.shown_at = now;
    }

    /// the clock's nanoseconds at `now`: a 60th of a second a step, taken
    /// exactly (50 ms each three) so that the steps do not drift
    fn at(&self, now: u64) -> u64 {
        let steps = self.steps.saturating_add(self.dark_steps(now));
        self.start
            .saturating_add(steps.saturating_mul(50_000_000) / 3)
    }
}

impl Threads {
    pub(crate) fn new() -> Self {
        Self {
            table: Mutex::new(Table {
                objects: BTreeMap::new(),
                next: FIRST_HANDLE,
                next_tid: 1,
                closing: false,
            }),
            changed: Condvar::new(),
            faults: Mutex::new(Vec::new()),
            spawner: OnceLock::new(),
            start: Instant::now(),
            frame_clock: Mutex::default(),
        }
    }

    fn table(&self) -> MutexGuard<'_, Table> {
        self.table.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// sets the game that new threads instantiate (once)
    pub(crate) fn set_spawner(&self, spawner: InstancePre<Record>) {
        // (a second game is never loaded into the same record)
        let _ = self.spawner.set(spawner);
    }

    /// what the threads that trapped said, oldest first
    pub(crate) fn faults(&self) -> Vec<String> {
        self.faults
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// ends every wait, now and to come: the game is going
    pub(crate) fn close(&self) {
        self.table().closing = true;
        self.changed.notify_all();
    }

    /// adds `object`, giving its handle (0 if there are no handles left)
    fn add(&self, object: Object) -> u32 {
        let mut table = self.table();
        let handle = table.next;
        let Some(next) = handle.checked_add(HANDLE_STEP) else {
            return 0;
        };
        table.next = next;
        table.objects.insert(handle, object);
        handle
    }

    /// runs `change` on the object `handle` and wakes the waiters; `None`
    /// if there is no such object
    fn change<R>(&self, handle: u32, change: impl FnOnce(&mut Object) -> R) -> Option<R> {
        let result = self.table().objects.get_mut(&handle).map(change);
        self.changed.notify_all();
        result
    }

    /// waits up to `ms` for `handle` and takes it for `thread`
    ///
    /// # Errors
    /// If the game is going.
    fn wait(&self, thread: u32, handle: u32, ms: u32) -> wasmtime::Result<u32> {
        let deadline = (ms != INFINITE).then(|| Instant::now() + Duration::from_millis(ms.into()));
        let mut table = self.table();
        loop {
            if table.closing {
                return Err(wasmtime::format_err!("the game is going"));
            }
            match table.objects.get_mut(&handle) {
                None | Some(Object::Thread { closed: true, .. }) => return Ok(WAIT_FAILED),
                Some(object) => {
                    if object.take(thread) {
                        return Ok(WAIT_OBJECT_0);
                    }
                }
            }
            table = match deadline {
                None => self
                    .changed
                    .wait(table)
                    .unwrap_or_else(PoisonError::into_inner),
                Some(deadline) => {
                    let left = deadline.saturating_duration_since(Instant::now());
                    if left.is_zero() {
                        return Ok(WAIT_TIMEOUT);
                    }
                    self.changed
                        .wait_timeout(table, left)
                        .unwrap_or_else(PoisonError::into_inner)
                        .0
                }
            };
        }
    }

    /// sleeps `ms` (or until the game goes)
    ///
    /// # Errors
    /// If the game is going.
    fn sleep(&self, ms: u32) -> wasmtime::Result<()> {
        if ms == 0 {
            std::thread::yield_now();
            return Ok(());
        }
        self.sleep_for((ms != INFINITE).then(|| Duration::from_millis(ms.into())))
    }

    /// sleeps `time`, or for ever if it is None (until the game goes)
    ///
    /// # Errors
    /// If the game is going.
    fn sleep_for(&self, time: Option<Duration>) -> wasmtime::Result<()> {
        let deadline = time.map(|time| Instant::now() + time);
        let mut table = self.table();
        loop {
            if table.closing {
                return Err(wasmtime::format_err!("the game is going"));
            }
            table = match deadline {
                None => self
                    .changed
                    .wait(table)
                    .unwrap_or_else(PoisonError::into_inner),
                Some(deadline) => {
                    let left = deadline.saturating_duration_since(Instant::now());
                    if left.is_zero() {
                        return Ok(());
                    }
                    self.changed
                        .wait_timeout(table, left)
                        .unwrap_or_else(PoisonError::into_inner)
                        .0
                }
            };
        }
    }

    /// waits until the thread `handle` may start; false if the game went
    fn wait_to_start(&self, handle: u32) -> bool {
        let mut table = self.table();
        loop {
            if table.closing {
                return false;
            }
            match table.objects.get(&handle) {
                Some(Object::Thread { suspend, .. }) if *suspend > 0 => {}
                _ => return true,
            }
            table = self
                .changed
                .wait(table)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }

    /// marks the thread `handle` ended (and removes it if it was closed)
    fn end(&self, handle: u32) {
        let mut table = self.table();
        if let Some(Object::Thread { ended, closed, .. }) = table.objects.get_mut(&handle) {
            *ended = true;
            if *closed {
                table.objects.remove(&handle);
            }
        }
        drop(table);
        self.changed.notify_all();
    }

    /// closes `handle` if it is one of these objects
    fn close_handle(&self, handle: u32) -> bool {
        let mut table = self.table();
        let closed = match table.objects.get_mut(&handle) {
            None | Some(Object::Thread { closed: true, .. }) => false,
            Some(Object::Thread {
                ended: false,
                closed,
                ..
            }) => {
                *closed = true;
                true
            }
            Some(_) => table.objects.remove(&handle).is_some(),
        };
        drop(table);
        self.changed.notify_all();
        closed
    }

    /// the real nanoseconds since the game was loaded
    fn real(&self) -> u64 {
        u64::try_from(self.start.elapsed().as_nanos()).unwrap_or(u64::MAX)
    }

    /// the nanoseconds since the game was loaded, by the frame clock when
    /// it is on
    pub(crate) fn elapsed(&self) -> u64 {
        let now = self.real();
        let clock = self
            .frame_clock
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if clock.on != Some(true) {
            return now;
        }
        clock.at(now)
    }

    /// a frame was shown: steps the frame clock, which the first frame
    /// turns on if `on` says so
    pub(crate) fn frame_shown(&self, on: impl FnOnce() -> bool) {
        let now = self.real();
        let mut clock = self
            .frame_clock
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        match clock.on {
            None => {
                *clock = if on() {
                    FrameClock::starting(now)
                } else {
                    FrameClock {
                        on: Some(false),
                        ..FrameClock::default()
                    }
                };
            }
            Some(true) => clock.shown(now),
            Some(false) => {}
        }
    }
}

/// the thread import named "module.name", if this module answers it
pub(crate) fn handler(name: &str) -> Option<Handler> {
    let handler: Handler = match name {
        "wasi.thread-spawn" => thread_spawn,
        "wasi_snapshot_preview1.clock_time_get" => clock_time_get,
        "wasi_snapshot_preview1.sched_yield" => sched_yield,
        "wasi_snapshot_preview1.poll_oneoff" => poll_oneoff,
        "wasi_snapshot_preview1.environ_sizes_get" => environ_sizes_get,
        "wasi_snapshot_preview1.environ_get" => environ_get,
        "env.CreateThread" => create_thread,
        "env.ResumeThread" => resume_thread,
        "env.SetThreadPriority" => set_thread_priority,
        "env.GetExitCodeThread" => get_exit_code_thread,
        "env.CreateEventA" => create_event,
        "env.SetEvent" => set_event,
        "env.ResetEvent" => reset_event,
        "env.CreateMutexA" => create_mutex,
        "env.ReleaseMutex" => release_mutex,
        "env.WaitForSingleObject" => wait_for_single_object,
        "env.WaitForSingleObjectEx" => wait_for_single_object_ex,
        "env.Sleep" => sleep,
        "env.SleepEx" => sleep_ex,
        "env.SwitchToThread" => switch_to_thread,
        "env.GetTickCount" => get_tick_count,
        "env.QueryPerformanceCounter" => query_performance_counter,
        "env.QueryPerformanceFrequency" => query_performance_frequency,
        "env.GetSystemTime" => get_system_time,
        "env.CloseHandle" => close_handle,
        _ => return None,
    };
    Some(handler)
}

/// `CreateThread(attributes, stack, start, parameter, flags, id)`: a thread
/// running `start(parameter)`, through the game's `pthread_create`; its
/// handle, or NULL
fn create_thread(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (start, parameter, flags, id) = (
        arg(params, 2),
        arg(params, 3),
        arg(params, 4),
        arg(params, 5),
    );
    let threads = &caller.data().shared.threads;
    let handle = threads.add(Object::Thread {
        suspend: u32::from(flags & CREATE_SUSPENDED != 0),
        ended: false,
        closed: false,
        tid: 0,
    });
    if handle == 0 {
        give(results, 0);
        return Ok(());
    }
    // (`thread-spawn`, called under `pthread_create`, takes the handle)
    caller.data_mut().pending = Some(handle);
    let out = call_game(caller, "malloc", &[Val::I32(4)])?;
    let error = if out == 0 {
        u32::MAX
    } else {
        call_game(
            caller,
            "pthread_create",
            &[
                Val::I32(out.cast_signed()),
                Val::I32(0),
                Val::I32(start.cast_signed()),
                Val::I32(parameter.cast_signed()),
            ],
        )?
    };
    caller.data_mut().pending = None;
    if out != 0 {
        // (the platform never joins the game's threads, so their resources
        // go when they end)
        let thread = with_space(caller, |space, _| space.get(out))?;
        if let (0, Some(thread)) = (error, thread) {
            if caller.get_export("pthread_detach").is_some() {
                call_game(caller, "pthread_detach", &[Val::I32(thread.cast_signed())])?;
            }
        }
        if caller.get_export("free").is_some() {
            call_game(caller, "free", &[Val::I32(out.cast_signed())])?;
        }
    }
    let threads = &caller.data().shared.threads;
    if error != 0 {
        threads.table().objects.remove(&handle);
        give(results, 0);
        return Ok(());
    }
    let tid = match threads.table().objects.get(&handle) {
        Some(Object::Thread { tid, .. }) => *tid,
        _ => 0,
    };
    if id != 0 {
        with_space(caller, |space, _| space.put(id, tid))?;
    }
    give(results, handle);
    Ok(())
}

/// `wasi.thread-spawn(start)`: runs the game's `wasi_thread_start(tid,
/// start)` on a thread of its own; the thread's id, or -1
fn thread_spawn(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let start = arg(params, 0);
    let record = caller.data_mut();
    let threads = &record.shared.threads;
    let Some(spawner) = threads.spawner.get().cloned() else {
        give(results, u32::MAX);
        return Ok(());
    };
    // (a thread pthread_create starts by itself has no handle yet)
    let handle = match record.pending.take() {
        Some(handle) => handle,
        None => threads.add(Object::Thread {
            suspend: 0,
            ended: false,
            closed: false,
            tid: 0,
        }),
    };
    let tid = {
        let mut table = threads.table();
        let tid = table.next_tid;
        table.next_tid = tid.wrapping_add(1).max(1);
        if let Some(Object::Thread { tid: given, .. }) = table.objects.get_mut(&handle) {
            *given = tid;
        }
        tid
    };
    let thread = record.for_thread(tid);
    let shared = record.shared.clone();
    let spawned = std::thread::Builder::new()
        .name(format!("game thread {tid}"))
        .stack_size(HOST_STACK)
        .spawn(move || {
            let threads = &shared.threads;
            if threads.wait_to_start(handle) {
                if let Err(error) = run(&spawner, thread, tid, start) {
                    threads
                        .faults
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .push(format!("thread {tid}: {error:?}"));
                }
            }
            threads.end(handle);
        });
    if spawned.is_err() {
        threads.end(handle);
        give(results, u32::MAX);
        return Ok(());
    }
    give(results, tid);
    Ok(())
}

/// instantiates the game again with `record` and runs its thread `tid`
fn run(
    spawner: &InstancePre<Record>,
    record: Record,
    tid: u32,
    start: u32,
) -> wasmtime::Result<()> {
    let mut store = Store::new(spawner.module().engine(), record);
    let instance = spawner.instantiate(&mut store)?;
    let entry = instance.get_typed_func::<(i32, i32), ()>(&mut store, "wasi_thread_start")?;
    entry.call(&mut store, (tid.cast_signed(), start.cast_signed()))
}

/// `ResumeThread(thread)`: the suspend count it had, or -1
fn resume_thread(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let had = caller
        .data()
        .shared
        .threads
        .change(arg(params, 0), |object| match object {
            Object::Thread { suspend, .. } => {
                let had = *suspend;
                *suspend = had.saturating_sub(1);
                had
            }
            _ => u32::MAX,
        });
    give(results, had.unwrap_or(u32::MAX));
    Ok(())
}

/// `SetThreadPriority(thread, priority)`: true for a thread (the host's
/// scheduler keeps its own priorities)
fn set_thread_priority(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let known = matches!(
        caller
            .data()
            .shared
            .threads
            .table()
            .objects
            .get(&arg(params, 0)),
        Some(Object::Thread { .. })
    );
    give(results, u32::from(known));
    Ok(())
}

/// `GetExitCodeThread(thread, code)`: `STILL_ACTIVE` while it runs, 0 once
/// it has ended
fn get_exit_code_thread(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (handle, out) = (arg(params, 0), arg(params, 1));
    let ended = match caller.data().shared.threads.table().objects.get(&handle) {
        Some(Object::Thread { ended, .. }) => Some(*ended),
        _ => None,
    };
    let done = match ended {
        Some(ended) => {
            let code = if ended { 0 } else { STILL_ACTIVE };
            with_space(caller, |space, _| space.put(out, code))?
        }
        None => false,
    };
    give(results, u32::from(done));
    Ok(())
}

/// `CreateEventA(attributes, manual, initial, name)`: an event's handle
fn create_event(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let handle = caller.data().shared.threads.add(Object::Event {
        manual: arg(params, 1) != 0,
        set: arg(params, 2) != 0,
    });
    give(results, handle);
    Ok(())
}

/// sets or resets the event `params[0]`: true if it is one
fn signal(caller: &Caller<'_, Record>, params: &[Val], results: &mut [Val], to: bool) {
    let done = caller
        .data()
        .shared
        .threads
        .change(arg(params, 0), |object| match object {
            Object::Event { set, .. } => {
                *set = to;
                true
            }
            _ => false,
        });
    give(results, u32::from(done.unwrap_or(false)));
}

/// `SetEvent(event)`
fn set_event(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    signal(caller, params, results, true);
    Ok(())
}

/// `ResetEvent(event)`
fn reset_event(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    signal(caller, params, results, false);
    Ok(())
}

/// `CreateMutexA(attributes, owned, name)`: a mutex's handle, owned by the
/// caller if `owned`
fn create_mutex(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let owned = arg(params, 1) != 0;
    let record = caller.data();
    let handle = record.shared.threads.add(Object::Mutex {
        owner: owned.then_some(record.thread),
        count: u32::from(owned),
    });
    give(results, handle);
    Ok(())
}

/// `ReleaseMutex(mutex)`: true if the caller owned it
fn release_mutex(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let record = caller.data();
    let thread = record.thread;
    let done = record
        .shared
        .threads
        .change(arg(params, 0), |object| match object {
            Object::Mutex { owner, count } if *owner == Some(thread) => {
                *count = count.saturating_sub(1);
                if *count == 0 {
                    *owner = None;
                }
                true
            }
            _ => false,
        });
    give(results, u32::from(done.unwrap_or(false)));
    Ok(())
}

/// runs the caller's completions waiting; true if there were any
///
/// # Errors
/// If a routine is not a function in the game's table, or traps.
fn run_completions(caller: &mut Caller<'_, Record>) -> wasmtime::Result<bool> {
    let record = caller.data();
    let thread = record.thread;
    let due = {
        let mut files = record.files();
        if !files.has_completions(thread) {
            return Ok(false);
        }
        files.take_completions(thread)
    };
    let table = caller
        .get_export("__indirect_function_table")
        .and_then(Extern::into_table)
        .ok_or_else(|| wasmtime::format_err!("the game exports no function table"))?;
    for completion in due {
        let Some(Ref::Func(Some(routine))) = table.get(&mut *caller, completion.routine.into())
        else {
            return Err(wasmtime::format_err!(
                "completion routine {} is not a function",
                completion.routine
            ));
        };
        let args = [
            completion.error,
            completion.transferred,
            completion.overlapped,
        ]
        .map(|value| Val::I32(value.cast_signed()));
        let mut results: Vec<Val> = routine
            .ty(&*caller)
            .results()
            .map(|ty| Val::default_for_ty(&ty).unwrap_or(Val::I32(0)))
            .collect();
        routine.call(&mut *caller, &args, &mut results)?;
    }
    Ok(true)
}

/// `WaitForSingleObject(handle, ms)`
fn wait_for_single_object(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let record = caller.data();
    let result = record
        .shared
        .threads
        .wait(record.thread, arg(params, 0), arg(params, 1))?;
    give(results, result);
    Ok(())
}

/// `WaitForSingleObjectEx(handle, ms, alertable)`: an alertable wait runs
/// the caller's completions instead, if it has any
fn wait_for_single_object_ex(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    if arg(params, 2) != 0 && run_completions(caller)? {
        give(results, WAIT_IO_COMPLETION);
        return Ok(());
    }
    wait_for_single_object(caller, params, results)
}

/// `Sleep(ms)`
fn sleep(caller: &mut Caller<'_, Record>, params: &[Val], _: &mut [Val]) -> wasmtime::Result<()> {
    let start = super::profile::sleep_start(caller.data());
    caller.data().shared.threads.sleep(arg(params, 0))?;
    super::profile::sleep_end(caller.data(), start);
    Ok(())
}

/// `SleepEx(ms, alertable)`: an alertable sleep runs the caller's
/// completions instead, if it has any
fn sleep_ex(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    if arg(params, 1) != 0 && run_completions(caller)? {
        give(results, WAIT_IO_COMPLETION);
        return Ok(());
    }
    let start = super::profile::sleep_start(caller.data());
    caller.data().shared.threads.sleep(arg(params, 0))?;
    super::profile::sleep_end(caller.data(), start);
    give(results, 0);
    Ok(())
}

/// `SwitchToThread()`
fn switch_to_thread(
    _: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    std::thread::yield_now();
    give(results, 1);
    Ok(())
}

/// wasi's `sched_yield()`, which wasi-libc's spin loops call: gives way
/// to the host's other threads, and succeeds
fn sched_yield(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    std::thread::yield_now();
    give(results, 0);
    Ok(())
}

/// `GetTickCount()`: milliseconds since the game was loaded (wrapping, as
/// Windows' do after 49 days)
fn get_tick_count(
    caller: &mut Caller<'_, Record>,
    _: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let ms = caller.data().shared.threads.elapsed() / 1_000_000;
    give(results, ms as u32);
    Ok(())
}

/// writes `value` at `params[index]`: true if it fit
fn put64(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    index: usize,
    value: u64,
) -> wasmtime::Result<bool> {
    let out = arg(params, index);
    with_space(caller, |space, _| space.put64(out, value))
}

/// `QueryPerformanceCounter(count)`: ticks of 100 ns since the game was
/// loaded
fn query_performance_counter(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let ticks = caller.data().shared.threads.elapsed() / 100;
    let done = put64(caller, params, 0, ticks)?;
    give(results, u32::from(done));
    Ok(())
}

/// `QueryPerformanceFrequency(frequency)`
fn query_performance_frequency(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let done = put64(caller, params, 0, PERFORMANCE_FREQUENCY)?;
    give(results, u32::from(done));
    Ok(())
}

/// the time since 1970
fn since_1970() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
}

/// `clock_time_get(id, precision, time)`: the real time (0) or the
/// monotonic one (1, and the CPU clocks 2 and 3), in nanoseconds. The
/// monotonic clock is the real one even with `debug.frame_clock` on (as in
/// `port/linux`, where only the game's own clocks follow the frames): the
/// vertical blanks and the mixer are paced by it, and a blank paced by a
/// clock that steps only when a frame is shown would hold back the frame.
fn clock_time_get(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let nanos = match arg(params, 0) {
        0 => u64::try_from(since_1970().as_nanos()).unwrap_or(u64::MAX),
        1..=3 => caller.data().shared.threads.real(),
        _ => {
            give(results, EINVAL);
            return Ok(());
        }
    };
    let done = put64(caller, params, 2, nanos)?;
    give(results, if done { 0 } else { EFAULT });
    Ok(())
}

/// `environ_sizes_get(count, size)`: the game is given no environment
fn environ_sizes_get(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (count, size) = (arg(params, 0), arg(params, 1));
    let done = with_space(caller, |space, _| space.put(count, 0) && space.put(size, 0))?;
    give(results, if done { 0 } else { EFAULT });
    Ok(())
}

/// `environ_get(pointers, strings)`: no variables, so nothing is written
fn environ_get(_: &mut Caller<'_, Record>, _: &[Val], results: &mut [Val]) -> wasmtime::Result<()> {
    give(results, 0);
    Ok(())
}

/// `poll_oneoff(in, out, count, events)`: the subscriptions' events. The
/// game polls only clocks (`nanosleep`, its vertical blanks' pacing), so a
/// descriptor's subscription is ready at once with ENOTSUP; otherwise the
/// first clock due is slept until, and reported.
fn poll_oneoff(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    let (input, output, count, events) = (
        arg(params, 0),
        arg(params, 1),
        arg(params, 2),
        arg(params, 3),
    );
    if count == 0 {
        give(results, EINVAL);
        return Ok(());
    }
    let subscriptions = match count.checked_mul(SUBSCRIPTION) {
        Some(len) => with_space(caller, |space, _| space.read(input, len))?,
        None => None,
    };
    let Some(subscriptions) = subscriptions else {
        give(results, EFAULT);
        return Ok(());
    };
    let monotonic = caller.data().shared.threads.real();
    let realtime = u64::try_from(since_1970().as_nanos()).unwrap_or(u64::MAX);
    // the descriptors' events (userdata, type, error), or else the soonest
    // clock's (userdata, nanoseconds to wait)
    let mut ready: Vec<(u64, u8, u16)> = Vec::new();
    let mut soonest: Option<(u64, u64)> = None;
    for subscription in subscriptions.as_chunks::<{ SUBSCRIPTION as usize }>().0 {
        let userdata = u64::from_le_bytes(field(subscription, 0));
        let [tag] = field(subscription, 8);
        if tag != 0 {
            ready.push((userdata, tag, ENOTSUP));
            continue;
        }
        let clock = match u32::from_le_bytes(field(subscription, CLOCK_ID)) {
            0 => realtime,
            1..=3 => monotonic,
            _ => {
                give(results, EINVAL);
                return Ok(());
            }
        };
        let timeout = u64::from_le_bytes(field(subscription, CLOCK_TIMEOUT));
        let [flags] = field(subscription, CLOCK_FLAGS);
        let wait = if flags & ABSTIME == 0 {
            timeout
        } else {
            timeout.saturating_sub(clock)
        };
        if soonest.is_none_or(|(_, soonest)| wait < soonest) {
            soonest = Some((userdata, wait));
        }
    }
    if let (true, Some((userdata, wait))) = (ready.is_empty(), soonest) {
        let start = super::profile::sleep_start(caller.data());
        caller
            .data()
            .shared
            .threads
            .sleep_for(Some(Duration::from_nanos(wait)))?;
        super::profile::sleep_end(caller.data(), start);
        ready.push((userdata, 0, 0));
    }
    let written = with_space(caller, |space, _| {
        let mut at = output;
        for &(userdata, kind, error) in &ready {
            let mut event = [0u8; EVENT as usize];
            event[..8].copy_from_slice(&userdata.to_le_bytes());
            event[8..10].copy_from_slice(&error.to_le_bytes());
            event[10] = kind;
            if !space.write(at, &event) {
                return false;
            }
            at = at.wrapping_add(EVENT);
        }
        space.put(events, u32::try_from(ready.len()).unwrap_or(u32::MAX))
    })?;
    give(results, if written { 0 } else { EFAULT });
    Ok(())
}

/// the `N` bytes at `at` in a subscription, or zeros past its end
fn field<const N: usize>(subscription: &[u8], at: u32) -> [u8; N] {
    usize::try_from(at)
        .ok()
        .and_then(|at| subscription.get(at..at.checked_add(N)?))
        .and_then(|bytes| <[u8; N]>::try_from(bytes).ok())
        .unwrap_or([0; N])
}

/// the year, month and day of `days` since 1970 (Howard Hinnant's
/// `civil_from_days`, for the proleptic Gregorian calendar)
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let of_era = z.rem_euclid(146_097);
    let year_of_era = (of_era - of_era / 1460 + of_era / 36_524 - of_era / 146_096) / 365;
    let of_year = of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * of_year + 2) / 153;
    let day = of_year - (153 * shifted + 2) / 5 + 1;
    let month = if shifted < 10 {
        shifted + 3
    } else {
        shifted - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    (year, month as u32, day as u32)
}

/// `GetSystemTime(time)`: now, in UTC, as `SYSTEMTIME` (year, month, day of
/// the week from Sunday, day, hour, minute, second, millisecond)
fn get_system_time(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    _: &mut [Val],
) -> wasmtime::Result<()> {
    let now = since_1970();
    let seconds = now.as_secs();
    let days = seconds / 86_400;
    let of_day = seconds % 86_400;
    let (year, month, day) = civil_from_days(i64::try_from(days).unwrap_or(0));
    let words = [
        year as u16,
        month as u16,
        // (1 January 1970 was a Thursday)
        ((days + 4) % 7) as u16,
        day as u16,
        (of_day / 3600) as u16,
        (of_day / 60 % 60) as u16,
        (of_day % 60) as u16,
        now.subsec_millis() as u16,
    ];
    let bytes: Vec<u8> = words.into_iter().flat_map(u16::to_le_bytes).collect();
    let out = arg(params, 0);
    with_space(caller, |space, _| space.write(out, &bytes))?;
    Ok(())
}

/// `CloseHandle(handle)`: closes a thread, event or mutex, and hands any
/// other handle to the files
fn close_handle(
    caller: &mut Caller<'_, Record>,
    params: &[Val],
    results: &mut [Val],
) -> wasmtime::Result<()> {
    if caller.data().shared.threads.close_handle(arg(params, 0)) {
        give(results, 1);
        return Ok(());
    }
    match files::service("env.CloseHandle") {
        Some(service) => files::serve(caller, service, params, results),
        None => {
            give(results, 0);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_since_1970_are_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(-1), (1969, 12, 31));
        // a leap day, and the day after
        assert_eq!(civil_from_days(11_016), (2000, 2, 29));
        assert_eq!(civil_from_days(11_017), (2000, 3, 1));
        assert_eq!(civil_from_days(20_754), (2026, 10, 28));
    }

    /// the clock's milliseconds after each of `frames` frames shown a
    /// 60th of a second apart, from `start`
    fn milliseconds(start: u64, frames: u64) -> Vec<u64> {
        let mut clock = FrameClock::starting(start);
        (1..=frames)
            .map(|frame| {
                let now = start + frame * FRAME_STEP;
                clock.shown(now);
                clock.at(now) / 1_000_000
            })
            .collect()
    }

    #[test]
    fn the_frame_clock_steps_alike_whatever_the_fraction_of_its_start() {
        // (a real start's fraction of a millisecond is chance; were it kept,
        // the 16 and 17 ms steps of a 60th would fall on other frames)
        let early = milliseconds(5_000_001, 30);
        let late = milliseconds(5_999_999, 30);
        assert_eq!(early, late);
        let steps: Vec<u64> = early.windows(2).map(|pair| pair[1] - pair[0]).collect();
        let (threes, _) = steps.as_chunks::<3>();
        assert!(threes.iter().all(|three| three.iter().sum::<u64>() == 50));
    }

    #[test]
    fn a_frame_clock_in_the_dark_keeps_its_steps_in_threes() {
        // (however long a load lasts, play resumes on the same third of
        // the clock's 50 ms cycle)
        for dark in [FRAME_DARK + 1, FRAME_DARK + FRAME_STEP, 3 * FRAME_DARK + 7] {
            let mut clock = FrameClock::starting(0);
            clock.shown(FRAME_STEP);
            let steps = clock.steps;
            clock.shown(FRAME_STEP + dark);
            assert_eq!((clock.steps - steps - 1) % 3, 0, "dark {dark}");
        }
    }

    #[test]
    fn a_recursive_mutex_is_taken_only_by_its_owner() {
        let mut mutex = Object::Mutex {
            owner: None,
            count: 0,
        };
        assert!(mutex.take(1));
        assert!(mutex.take(1));
        assert!(!mutex.take(2));
        assert!(matches!(
            mutex,
            Object::Mutex {
                owner: Some(1),
                count: 2
            }
        ));
    }
}
