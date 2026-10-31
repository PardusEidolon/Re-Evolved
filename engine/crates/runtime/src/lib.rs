//! The game's runtime: `source/` built as `game.wasm` (tools/engine_wasm.py),
//! compiled by Wasmtime with Cranelift and run inside the macOS app.
//!
//! A [`Runtime`] holds the engine and, when given a folder, Wasmtime's cache
//! of compiled modules, keyed by Wasmtime's version, the compiler's settings
//! and the CPU's features, so the game compiles once per machine and build.
//! [`Runtime::load`] instantiates a module with a stub for every function it
//! imports: each stub records its call and returns zeros, until the platform
//! supplies the real ones. `proc_exit` ends the call with [`Error::Exit`],
//! and `fd_write` keeps what is written, per descriptor ([`Game::output`]),
//! so the reference harnesses run as WASI commands ([`Game::run_command`]).
//! A call budget ([`Game::set_call_budget`]) ends a game that, on stubs,
//! never returns.
//!
//! The module and its memory are untrusted: a trap is an error, never a
//! panic, and the memory is read and written only through checked ranges.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

mod audio;
mod batches;
pub mod benchmark;
mod command;
mod config;
pub mod control;
mod controls;
mod display;
pub mod files;
mod input;
pub mod keyboard;
pub mod memory;
mod net;
pub mod plugins;
mod profile;
pub mod replay;
mod space;
mod system;
mod threads;
pub mod views;
pub mod widgets;

use space::Space;
use wasmtime::{
    Cache, CacheConfig, Caller, Config, Engine, Extern, ExternType, Instance, Linker, Memory,
    Module, SharedMemory, Store,
};

/// a wasm value, as the game's functions take and return them
pub use wasmtime::Val;

pub use audio::{Speakers, SPEAKER_RATE, SPEAKER_TARGET_FRAMES};
pub use batches::{payload, readback_fits, Readback, RenderCounts, RenderSink};
pub use controls::{keystroke_flags, virtual_key, Controls, Tuning};
pub use display::DisplayRequest;
pub use net::Console;
pub use profile::{Span, SLEEP};

/// the import that ends the game, with its exit code
const PROC_EXIT: &str = "wasi_snapshot_preview1.proc_exit";

/// the import that writes to a descriptor, kept in [`Record::outputs`]
const FD_WRITE: &str = "wasi_snapshot_preview1.fd_write";

/// WASI's errno for an address outside the memory
const EFAULT: i32 = 21;

/// WASI's errno for a count too large for its type
const EOVERFLOW: i32 = 61;

/// the calls a game keeps until [`Game::set_record_limit`] says otherwise
const DEFAULT_RECORD_LIMIT: usize = 65_536;

/// the address space a 32-bit module can reach: the whole of it is reserved,
/// so the Xbox's window (0x80000000 to 0xA0000000) needs no bounds checks
const MEMORY_RESERVATION: u64 = 1 << 32;

/// what went wrong
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// the game called `proc_exit` with this code
    #[error("the game exited with code {0}")]
    Exit(i32),
    /// the game made as many import calls as its budget allowed
    #[error("the game used its budget of {0} import calls")]
    Budget(u64),
    /// the game made an import call after its deadline
    #[error("the game ran past its deadline")]
    Deadline,
    /// the game trapped (or a stub failed)
    #[error("the game trapped: {0}")]
    Trap(String),
    /// the module exports no function of this name
    #[error("the module exports no function {0}")]
    NoExport(String),
    /// the module could not be compiled
    #[error("the module could not be compiled: {0}")]
    Compile(String),
    /// the module imports something the runtime cannot supply
    #[error("the module imports {0}, which the runtime cannot supply")]
    Import(String),
    /// a read or write outside the memory
    #[error("{len} bytes at {address:#x} are outside the memory of {size} bytes")]
    OutOfBounds {
        /// where the access began
        address: u64,
        /// how many bytes it covered
        len: u64,
        /// the memory's size
        size: u64,
    },
    /// the export returned something other than what was asked for
    #[error("{0} returned {1}")]
    Result(String, String),
    /// Wasmtime could not be set up
    #[error("the runtime could not be set up: {0}")]
    Setup(String),
    /// the memory could not grow to this many bytes
    #[error("the memory could not grow to {0:#x} bytes")]
    Grow(u64),
    /// a recording's bytes could not be read ([`replay::Recording::from_bytes`])
    #[error("the recording could not be read: {0}")]
    Recording(String),
    /// a replayed run asked for another import than the recording answers
    #[error(
        "the replay diverged at answer {at}: it answers {recorded}, and {asked} was asked for"
    )]
    Diverged {
        /// the answer's place in the recording
        at: usize,
        /// the import it answers
        recorded: String,
        /// the import the game asked for
        asked: String,
    },
}

/// a value passed to an import; floats are kept as their bits, so that a
/// recorded call compares exactly, NaNs included
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// an i32
    I32(i32),
    /// an i64
    I64(i64),
    /// an f32's bits
    F32(u32),
    /// an f64's bits
    F64(u64),
    /// a v128's bits
    V128(u128),
    /// a reference of any kind
    Ref,
}

impl From<&Val> for Value {
    fn from(value: &Val) -> Self {
        match value {
            Val::I32(v) => Self::I32(*v),
            Val::I64(v) => Self::I64(*v),
            Val::F32(bits) => Self::F32(*bits),
            Val::F64(bits) => Self::F64(*bits),
            Val::V128(v) => Self::V128(v.as_u128()),
            _ => Self::Ref,
        }
    }
}

/// one recorded call of an import
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// the import's index in the module's imports ([`Game::import_name`])
    pub import: usize,
    /// its arguments
    pub args: Vec<Value>,
}

/// the exit code `proc_exit` carries out of the game as a trap
#[derive(Debug)]
pub(crate) struct Exit(pub(crate) i32);

impl std::fmt::Display for Exit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "proc_exit({})", self.0)
    }
}

impl std::error::Error for Exit {}

/// the call budget, carried out of the game as a trap
#[derive(Debug)]
struct Budget(u64);

impl std::fmt::Display for Budget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a budget of {} import calls", self.0)
    }
}

impl std::error::Error for Budget {}

/// Ends a game from another thread ([`Game::stopper`]).
#[derive(Clone, Debug)]
pub struct Stopper(Arc<std::sync::atomic::AtomicBool>);

impl Stopper {
    /// Ends the game at its next import call.
    pub fn stop(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// the deadline, carried out of the game as a trap
#[derive(Debug)]
struct Deadline;

impl std::fmt::Display for Deadline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "a deadline")
    }
}

impl std::error::Error for Deadline {}

/// the game's memory: its own, or shared between its threads
#[derive(Clone)]
enum Mem {
    Own(Memory),
    Shared(SharedMemory),
}

/// what the game's threads share
struct Shared {
    /// the Xbox's contiguous window, as `XPhysicalAlloc` hands it out
    window: Mutex<memory::Window>,
    /// the game's folders, open files and last error
    files: Mutex<files::Files>,
    /// the game's threads, the objects they wait on and the clocks
    threads: threads::Threads,
    /// the game's settings, as it reads and writes them
    settings: Mutex<config::Settings>,
    /// the keyboard and mouse, and what the game has opened of them
    input: Mutex<input::Input>,
    /// the display settings applied, on their way to the app's window
    display: Mutex<display::Display>,
    /// the mixed sound, on its way to the speakers
    audio: audio::Output,
    /// the profile sections' spans, while profiling is on
    profile: Mutex<profile::Profile>,
    /// the Metal device's batches: their validator, the app's renderer and
    /// the drawable's size
    batches: Mutex<batches::Batches>,
    /// the game's sockets, and the console's link to the host
    sockets: Mutex<net::Sockets>,
    /// the plugins, called at their hooks
    plugins: plugins::Plugins,
    /// a command's arguments and standard input
    command: Mutex<command::Command>,
}

/// the record the stubs keep, in the store
struct Record {
    /// each import's "module.name", in the module's order
    names: Vec<String>,
    /// how many times each import was called
    counts: Vec<u64>,
    /// the first calls, up to `limit`
    calls: Vec<Call>,
    limit: usize,
    /// the import calls made so far, and how many may be made
    made: u64,
    budget: Option<u64>,
    /// when the game's import calls end ([`Game::set_deadline`])
    deadline: Option<std::time::Instant>,
    /// set from any thread to end the game as its deadline does
    /// ([`Game::stopper`])
    stopped: Arc<std::sync::atomic::AtomicBool>,
    /// what `fd_write` wrote, per descriptor
    outputs: BTreeMap<i32, Vec<u8>>,
    /// the game's memory, once there is one (until then, its export)
    memory: Option<Mem>,
    /// what this thread shares with the game's others
    shared: Arc<Shared>,
    /// this thread's id (0 for the game's first)
    thread: u32,
    /// the handle of the thread `CreateThread` is starting
    pending: Option<u32>,
    /// the main thread's inputs recorded or replayed ([`replay`])
    tape: replay::Tape,
    /// how deep in recorded answers the call is: an answer's own calls back
    /// into the game are not answers of their own
    depth: u32,
    /// the writes of the answer being recorded, moved into each
    /// [`Space`] it writes through
    journal: Option<Vec<(u32, Vec<u8>)>>,
}

impl Record {
    /// a record for the game's thread `thread`, sharing this one's memory
    /// and state (the thread's own calls are not counted or budgeted)
    fn for_thread(&self, thread: u32) -> Self {
        Self {
            names: self.names.clone(),
            counts: vec![0; self.names.len()],
            calls: Vec::new(),
            limit: 0,
            made: 0,
            budget: None,
            deadline: None,
            stopped: Arc::clone(&self.stopped),
            outputs: BTreeMap::new(),
            memory: self.memory.clone(),
            shared: Arc::clone(&self.shared),
            thread,
            pending: None,
            tape: replay::Tape::Off,
            depth: 0,
            journal: None,
        }
    }

    /// the Xbox's window, held while the guard lives
    fn window(&self) -> MutexGuard<'_, memory::Window> {
        self.shared
            .window
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the game's files, held while the guard lives
    fn files(&self) -> MutexGuard<'_, files::Files> {
        self.shared
            .files
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the game's settings, held while the guard lives
    fn batches(&self) -> MutexGuard<'_, batches::Batches> {
        self.shared
            .batches
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn settings(&self) -> MutexGuard<'_, config::Settings> {
        self.shared
            .settings
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the keyboard and mouse, held while the guard lives
    fn input(&self) -> MutexGuard<'_, input::Input> {
        self.shared
            .input
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn display(&self) -> MutexGuard<'_, display::Display> {
        self.shared
            .display
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the game's sockets, held while the guard lives
    fn sockets(&self) -> MutexGuard<'_, net::Sockets> {
        self.shared
            .sockets
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// the profile sections, held while the guard lives
    fn profile(&self) -> MutexGuard<'_, profile::Profile> {
        self.shared
            .profile
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

/// runs `use_space` on the caller's memory, with its record beside it
///
/// # Errors
/// If the game has no memory yet and exports none.
fn with_space<R>(
    caller: &mut Caller<'_, Record>,
    use_space: impl FnOnce(&mut Space<'_>, &mut Record) -> R,
) -> wasmtime::Result<R> {
    let memory = match caller.data().memory.clone() {
        Some(memory) => memory,
        None => caller
            .get_export("memory")
            .and_then(Extern::into_memory)
            .map(Mem::Own)
            .ok_or_else(|| wasmtime::format_err!("the game has no memory"))?,
    };
    Ok(match memory {
        Mem::Own(memory) => {
            let (data, record) = memory.data_and_store_mut(&mut *caller);
            let mut space = Space::own(data);
            space.journal = record.journal.take();
            let used = use_space(&mut space, record);
            record.journal = space.journal.take();
            used
        }
        Mem::Shared(memory) => {
            let record = caller.data_mut();
            let mut space = Space::shared(memory.data());
            space.journal = record.journal.take();
            let used = use_space(&mut space, record);
            record.journal = space.journal.take();
            used
        }
    })
}

/// `fd_write(fd, iovs, count, written)`: gathers the `count` iovecs at
/// `iovs` from the caller's memory, stores how many bytes they hold at
/// `written` and keeps them; returns WASI's errno, [`EFAULT`] (with nothing
/// written) if any of it is outside the memory
fn fd_write(caller: &mut Caller<'_, Record>, params: &[Val]) -> i32 {
    let [Val::I32(fd), Val::I32(iovs), Val::I32(count), Val::I32(written)] = params else {
        return EFAULT;
    };
    // (the module's pointers are unsigned)
    let (iovs, count, written) = (
        iovs.cast_unsigned(),
        count.cast_unsigned(),
        written.cast_unsigned(),
    );
    let gathered = with_space(caller, |space, _| {
        let mut bytes = Vec::new();
        for index in 0..count {
            let base = index
                .checked_mul(8)
                .and_then(|offset| iovs.checked_add(offset))
                .ok_or(EFAULT)?;
            let chunk = space
                .get(base)
                .zip(base.checked_add(4).and_then(|at| space.get(at)))
                .and_then(|(address, len)| space.read(address, len))
                .ok_or(EFAULT)?;
            bytes.extend(chunk);
        }
        let total = u32::try_from(bytes.len()).map_err(|_| EOVERFLOW)?;
        if !space.put(written, total) {
            return Err(EFAULT);
        }
        Ok(bytes)
    });
    match gathered {
        Ok(Ok(bytes)) => {
            caller
                .data_mut()
                .outputs
                .entry(*fd)
                .or_default()
                .extend(bytes);
            0
        }
        Ok(Err(errno)) => errno,
        Err(_) => EFAULT,
    }
}

/// Whether the platform answers the import "module.name" (rather than
/// leaving it to a stub that returns zeros).
#[must_use]
pub fn answers(name: &str) -> bool {
    name == PROC_EXIT
        || name == FD_WRITE
        || threads::handler(name).is_some()
        || command::handler(name).is_some()
        || memory::handler(name).is_some()
        || system::handler(name).is_some()
        || batches::handler(name).is_some()
        || config::handler(name).is_some()
        || input::handler(name).is_some()
        || display::handler(name).is_some()
        || audio::handler(name).is_some()
        || net::handler(name).is_some()
        || profile::handler(name).is_some()
        || files::service(name).is_some()
}

/// The folder in the cache that holds Wasmtime's compiled modules: a folder
/// of its own, because Wasmtime's cleanup deletes the files it does not
/// know, and the host keeps others in the cache (the bakes, the binary
/// archive).
pub const COMPILED_FOLDER: &str = "compiled";

/// Wasmtime, set up for the game
pub struct Runtime {
    engine: Engine,
    cache: Option<Cache>,
}

impl Runtime {
    /// A runtime that caches compiled modules in `cache` (none if `None`),
    /// in its [`COMPILED_FOLDER`].
    ///
    /// # Errors
    /// [`Error::Setup`] if Wasmtime or its cache cannot be set up.
    pub fn new(cache: Option<&Path>) -> Result<Self, Error> {
        let mut config = Config::new();
        config
            .memory_reservation(MEMORY_RESERVATION)
            .wasm_threads(true)
            .shared_memory(true)
            // the simulation must be deterministic: every NaN the game makes
            // has the same bits on every machine
            .cranelift_nan_canonicalization(true)
            // and relaxed SIMD, whose results may differ by machine (a fused
            // multiply-add or not), is refused: the game is built without it
            .wasm_relaxed_simd(false);
        let cache = match cache {
            Some(folder) => {
                let mut settings = CacheConfig::new();
                settings.with_directory(folder.join(COMPILED_FOLDER));
                Some(Cache::new(settings).map_err(|error| Error::Setup(format!("{error:#}")))?)
            }
            None => None,
        };
        config.cache(cache.clone());
        let engine = Engine::new(&config).map_err(|error| Error::Setup(format!("{error:#}")))?;
        Ok(Self { engine, cache })
    }

    /// Wasmtime's engine, as the runtime set it up: what the platform layer
    /// shares between the game's threads, with their shared memory.
    #[must_use]
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// How many times a compiled module was found in the cache.
    #[must_use]
    pub fn cache_hits(&self) -> usize {
        self.cache.as_ref().map_or(0, Cache::cache_hits)
    }

    /// Compiles `wasm` (or finds it in the cache) and instantiates it with a
    /// recording stub for every function it imports.
    ///
    /// # Errors
    /// [`Error::Compile`] if the module is malformed or invalid,
    /// [`Error::Import`] if it imports anything but functions and one
    /// shared memory, and
    /// [`Error::Trap`] if instantiating it traps.
    pub fn load(&self, wasm: &[u8]) -> Result<Game, Error> {
        let module = Module::new(&self.engine, wasm)
            .map_err(|error| Error::Compile(format!("{error:#}")))?;
        let mut linker = Linker::new(&self.engine);
        let mut names = Vec::new();
        // (a threaded game imports its memory, shared, and exports none)
        let mut shared = None;
        for (index, import) in module.imports().enumerate() {
            let name = format!("{}.{}", import.module(), import.name());
            let ty = match import.ty() {
                ExternType::Func(ty) => ty,
                ExternType::Memory(ty) if ty.is_shared() && shared.is_none() => {
                    let memory = SharedMemory::new(&self.engine, ty)
                        .map_err(|error| Error::Import(format!("{name}: {error:#}")))?;
                    shared = Some((
                        import.module().to_string(),
                        import.name().to_string(),
                        memory,
                    ));
                    names.push(name);
                    continue;
                }
                _ => return Err(Error::Import(name)),
            };
            let exits = name == PROC_EXIT;
            let writes = name == FD_WRITE;
            let threads = threads::handler(&name);
            let serves = memory::handler(&name)
                .or_else(|| command::handler(&name))
                .or_else(|| system::handler(&name))
                .or_else(|| batches::handler(&name))
                .or_else(|| config::handler(&name))
                .or_else(|| input::handler(&name))
                .or_else(|| display::handler(&name))
                .or_else(|| audio::handler(&name))
                .or_else(|| net::handler(&name))
                .or_else(|| profile::handler(&name));
            let files_service = files::service(&name);
            let taped = replay::recorded(&name);
            let asked = name.clone();
            linker
                .func_new(
                    import.module(),
                    import.name(),
                    ty.clone(),
                    move |mut caller: Caller<'_, Record>, params, results| {
                        let record: &mut Record = caller.data_mut();
                        if let Some(budget) = record.budget {
                            if record.made >= budget {
                                return Err(wasmtime::Error::new(Budget(budget)));
                            }
                        }
                        if record
                            .deadline
                            .is_some_and(|deadline| std::time::Instant::now() >= deadline)
                            || record.stopped.load(std::sync::atomic::Ordering::Relaxed)
                        {
                            return Err(wasmtime::Error::new(Deadline));
                        }
                        record.made += 1;
                        if let Some(count) = record.counts.get_mut(index) {
                            *count += 1;
                        }
                        if record.calls.len() < record.limit {
                            record.calls.push(Call {
                                import: index,
                                args: params.iter().map(Value::from).collect(),
                            });
                        }
                        if exits {
                            let code = match params.first() {
                                Some(Val::I32(code)) => *code,
                                _ => -1,
                            };
                            return Err(wasmtime::Error::new(Exit(code)));
                        }
                        let live = |caller: &mut Caller<'_, Record>,
                                    params: &[Val],
                                    results: &mut [Val]|
                         -> wasmtime::Result<()> {
                            if writes {
                                let errno = fd_write(caller, params);
                                if let Some(result) = results.first_mut() {
                                    *result = Val::I32(errno);
                                }
                                return Ok(());
                            }
                            if let Some(serve) = threads {
                                return serve(caller, params, results);
                            }
                            if let Some(serve) = serves {
                                return serve(caller, params, results);
                            }
                            if let Some(service) = files_service {
                                return files::serve(caller, service, params, results);
                            }
                            for (result, ty) in results.iter_mut().zip(ty.results()) {
                                *result = Val::default_for_ty(&ty).ok_or_else(|| {
                                    wasmtime::format_err!("no zero value for a result of type {ty}")
                                })?;
                            }
                            Ok(())
                        };
                        let record = caller.data_mut();
                        let answered = taped && record.thread == 0 && record.depth == 0;
                        if !answered || matches!(record.tape, replay::Tape::Off) {
                            // (an answer's own calls write nothing into it)
                            let journal = record.journal.take();
                            let served = live(&mut caller, params, results);
                            caller.data_mut().journal = journal;
                            return served;
                        }
                        if let replay::Tape::Replaying { recording, next } = &mut record.tape {
                            let Some(answer) = recording.answers.get(*next) else {
                                return live(&mut caller, params, results);
                            };
                            if answer.import != asked {
                                return Err(wasmtime::Error::new(replay::Diverged {
                                    at: *next,
                                    recorded: answer.import.clone(),
                                    asked: asked.clone(),
                                }));
                            }
                            let answer = answer.clone();
                            *next += 1;
                            for ((result, value), ty) in
                                results.iter_mut().zip(&answer.results).zip(ty.results())
                            {
                                let expected = Val::default_for_ty(&ty).ok_or_else(|| {
                                    wasmtime::format_err!("no zero value for a result of type {ty}")
                                })?;
                                *result = replay::val(value, &expected).ok_or_else(|| {
                                    wasmtime::format_err!("{asked}'s recorded result is not a {ty}")
                                })?;
                            }
                            let wrote = with_space(&mut caller, |space, _| {
                                answer
                                    .writes
                                    .iter()
                                    .all(|(address, bytes)| space.write(*address, bytes))
                            })?;
                            if !wrote {
                                return Err(wasmtime::format_err!(
                                    "{asked}'s recorded writes are outside the memory"
                                ));
                            }
                            return Ok(());
                        }
                        record.journal = Some(Vec::new());
                        record.depth += 1;
                        let served = live(&mut caller, params, results);
                        let record = caller.data_mut();
                        record.depth -= 1;
                        let writes = record.journal.take().unwrap_or_default();
                        served?;
                        if let replay::Tape::Recording(recording) = &mut record.tape {
                            recording.answers.push(replay::Answer {
                                import: asked.clone(),
                                results: results.iter().map(Value::from).collect(),
                                writes,
                            });
                        }
                        Ok(())
                    },
                )
                .map_err(|error| Error::Import(format!("{name}: {error:#}")))?;
            names.push(name);
        }
        let counts = vec![0; names.len()];
        let mut store = Store::new(
            &self.engine,
            Record {
                names,
                counts,
                calls: Vec::new(),
                limit: DEFAULT_RECORD_LIMIT,
                made: 0,
                budget: None,
                deadline: None,
                stopped: Arc::default(),
                outputs: BTreeMap::new(),
                memory: shared
                    .as_ref()
                    .map(|(_, _, memory)| Mem::Shared(memory.clone())),
                shared: Arc::new(Shared {
                    window: Mutex::new(memory::Window::new()),
                    files: Mutex::new(files::Files::default()),
                    threads: threads::Threads::new(),
                    settings: Mutex::new(config::Settings::new(|name| std::env::var(name).ok())),
                    input: Mutex::new(input::Input::default()),
                    display: Mutex::new(display::Display::default()),
                    audio: audio::Output::default(),
                    profile: Mutex::new(profile::Profile::default()),
                    batches: Mutex::default(),
                    sockets: Mutex::new(net::Sockets::default()),
                    plugins: plugins::Plugins::default(),
                    command: Mutex::default(),
                }),
                thread: 0,
                pending: None,
                tape: replay::Tape::Off,
                depth: 0,
                journal: None,
            },
        );
        if let Some((module_name, name, memory)) = &shared {
            linker
                .define(&store, module_name, name, memory.clone())
                .map_err(|error| Error::Import(format!("{module_name}.{name}: {error:#}")))?;
        }
        let instance = if shared.is_some() {
            // (each new thread instantiates the game again, over its memory)
            let pre = linker.instantiate_pre(&module).map_err(called)?;
            store.data().shared.threads.set_spawner(pre.clone());
            pre.instantiate(&mut store).map_err(called)?
        } else {
            linker.instantiate(&mut store, &module).map_err(called)?
        };
        let memory = match shared {
            Some((_, _, memory)) => Some(Mem::Shared(memory)),
            None => instance.get_memory(&mut store, "memory").map(Mem::Own),
        };
        store.data_mut().memory.clone_from(&memory);
        let functions = module
            .exports()
            .filter(|export| matches!(export.ty(), ExternType::Func(_)))
            .map(|export| export.name().to_string())
            .collect();
        let data = data_exports(&module, &instance, &mut store);
        store.data().shared.plugins.set_exports(data);
        Ok(Game {
            store,
            instance,
            memory,
            functions,
        })
    }
}

impl Drop for Game {
    /// ends the waits of the game's threads, so they end with it
    fn drop(&mut self) {
        self.store.data().shared.threads.close();
    }
}

/// The name game.wasm exports the loading screen's chant by:
/// `compressed_monk_tone`, its Xbox ADPCM tone ([`Game::set_tone`]).
pub const TONE_EXPORT: &str = "compressed_monk_tone";

/// The size of the chant's tone: 511 Xbox ADPCM blocks of 36 bytes.
pub const TONE_BYTES: usize = 18_396;

/// the data a module exports, each name and its address: the i32 globals
/// wasm-ld exports for each data symbol `--export`ed
fn data_exports(
    module: &Module,
    instance: &Instance,
    store: &mut Store<Record>,
) -> Vec<(String, u32)> {
    module
        .exports()
        .filter(|export| matches!(export.ty(), ExternType::Global(_)))
        .filter_map(|export| {
            let global = instance.get_global(&mut *store, export.name())?;
            match global.get(&mut *store) {
                Val::I32(address) => Some((export.name().to_string(), address.cast_unsigned())),
                _ => None,
            }
        })
        .collect()
}

/// what a failed call means: the game's exit, or a trap
fn called(error: wasmtime::Error) -> Error {
    if let Some(Exit(code)) = error.downcast_ref::<Exit>() {
        return Error::Exit(*code);
    }
    if error.downcast_ref::<Deadline>().is_some() {
        return Error::Deadline;
    }
    if let Some(diverged) = error.downcast_ref::<replay::Diverged>() {
        return Error::Diverged {
            at: diverged.at,
            recorded: diverged.recorded.clone(),
            asked: diverged.asked.clone(),
        };
    }
    match error.downcast_ref::<Budget>() {
        Some(Budget(budget)) => Error::Budget(*budget),
        None => Error::Trap(format!("{error:?}")),
    }
}

/// an instantiated module
pub struct Game {
    store: Store<Record>,
    instance: Instance,
    memory: Option<Mem>,
    /// the names of the functions it exports
    functions: Vec<String>,
}

impl Game {
    /// Calls the exported function `name` with no arguments and returns its
    /// results.
    ///
    /// # Errors
    /// [`Error::NoExport`] if there is no such function, [`Error::Exit`] if
    /// the game exits during the call and [`Error::Trap`] if it traps.
    pub fn call_results(&mut self, name: &str) -> Result<Vec<Val>, Error> {
        self.call_with(name, &[])
    }

    /// Calls the exported function `name` with `args` and returns its
    /// results.
    ///
    /// # Errors
    /// As [`Game::call_results`], and [`Error::Trap`] if `args` do not fit
    /// the function's parameters.
    pub fn call_with(&mut self, name: &str, args: &[Val]) -> Result<Vec<Val>, Error> {
        let func = self
            .instance
            .get_func(&mut self.store, name)
            .ok_or_else(|| Error::NoExport(name.to_string()))?;
        let ty = func.ty(&self.store);
        let mut results: Vec<Val> = ty
            .results()
            .map(|ty| Val::default_for_ty(&ty).unwrap_or(Val::I32(0)))
            .collect();
        func.call(&mut self.store, args, &mut results)
            .map_err(called)?;
        Ok(results)
    }

    /// Calls the exported function `name`, dropping any results.
    ///
    /// # Errors
    /// As [`Game::call_results`].
    pub fn call(&mut self, name: &str) -> Result<(), Error> {
        self.call_results(name).map(drop)
    }

    /// Calls the exported function `name`, which returns one i32.
    ///
    /// # Errors
    /// As [`Game::call_results`], and [`Error::Result`] if it returns
    /// anything else.
    pub fn call_i32(&mut self, name: &str) -> Result<i32, Error> {
        match self.call_results(name)?.as_slice() {
            [Val::I32(value)] => Ok(*value),
            other => Err(Error::Result(name.to_string(), format!("{other:?}"))),
        }
    }

    /// Runs the module as a WASI command: calls `_start`, and returns the
    /// code it gave `proc_exit`, or 0 if it returned.
    ///
    /// # Errors
    /// [`Error::NoExport`] if it has no `_start`, and [`Error::Trap`] if it
    /// traps.
    pub fn run_command(&mut self) -> Result<i32, Error> {
        match self.call("_start") {
            Ok(()) => Ok(0),
            Err(Error::Exit(code)) => Ok(code),
            Err(error) => Err(error),
        }
    }

    /// Gives a command `args` (its own name first) and `stdin`, for
    /// [`Game::run_command`]: until then it has no arguments and its
    /// standard input is at its end.
    pub fn set_command(&mut self, args: &[impl AsRef<[u8]>], stdin: Vec<u8>) {
        *self
            .store
            .data()
            .shared
            .command
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = command::Command::new(args, stdin);
    }

    /// What the module has written to descriptor `fd` with `fd_write`.
    #[must_use]
    pub fn output(&self, fd: i32) -> &[u8] {
        self.store
            .data()
            .outputs
            .get(&fd)
            .map_or(&[], Vec::as_slice)
    }

    /// The calls recorded so far, oldest first, up to the record's limit.
    #[must_use]
    pub fn calls(&self) -> &[Call] {
        &self.store.data().calls
    }

    /// Keeps at most `limit` calls from now on (every call is still
    /// counted).
    pub fn set_record_limit(&mut self, limit: usize) {
        let record = self.store.data_mut();
        record.limit = limit;
        record.calls.truncate(limit);
    }

    /// Ends the game with [`Error::Budget`] when it makes an import call
    /// after `budget` of them (counted from the start; `None`, the default,
    /// for no budget).
    pub fn set_call_budget(&mut self, budget: Option<u64>) {
        self.store.data_mut().budget = budget;
    }

    /// Ends the game with [`Error::Deadline`] when it makes an import call
    /// at or after `deadline` (`None`, the default, for none): a benchmark
    /// runs the game for a time, not a count.
    pub fn set_deadline(&mut self, deadline: Option<std::time::Instant>) {
        self.store.data_mut().deadline = deadline;
    }

    /// A handle that ends the game, from any thread, at its next import
    /// call with [`Error::Deadline`]: a test that drives the game ends it
    /// when it is done rather than when its deadline comes.
    pub fn stopper(&self) -> Stopper {
        Stopper(Arc::clone(&self.store.data().stopped))
    }

    /// Turns the timing of the game's profile sections on (from now) or
    /// off ([`profile`]'s module documentation).
    pub fn set_profiling(&mut self, on: bool) {
        self.store.data().profile().set(on);
    }

    /// Adds `plugin`, called at each of its hooks after the plugins added
    /// before it ([`plugins`]).
    pub fn add_plugin(&mut self, plugin: Box<dyn plugins::Plugin>) {
        self.store.data().shared.plugins.add(plugin);
    }

    /// Each plugin's time in each frame since they were last taken, oldest
    /// first, the plugins in the order they were added (at most
    /// [`plugins::FRAMES_KEPT`] frames); none while no plugin is added.
    #[must_use]
    pub fn take_plugin_frames(&self) -> Vec<Vec<plugins::PluginTime>> {
        self.store.data().shared.plugins.take_frames()
    }

    /// Records the answers to the main thread's inputs from now on (its
    /// clocks, keyboard and mouse, file times and random bytes: see
    /// [`replay`]), ending any replay.
    pub fn record_inputs(&mut self) {
        self.store.data_mut().tape = replay::Tape::Recording(replay::Recording::default());
    }

    /// The answers recorded since [`Game::record_inputs`], ending the
    /// recording; none if nothing is being recorded.
    pub fn take_recording(&mut self) -> replay::Recording {
        let record = self.store.data_mut();
        match std::mem::take(&mut record.tape) {
            replay::Tape::Recording(recording) => recording,
            other => {
                record.tape = other;
                replay::Recording::default()
            }
        }
    }

    /// Answers the main thread's inputs from `recording`, in order, until
    /// it runs out; then they are answered live again. An input asked for
    /// out of order ends the call with [`Error::Diverged`].
    pub fn replay(&mut self, recording: replay::Recording) {
        self.store.data_mut().tape = replay::Tape::Replaying { recording, next: 0 };
    }

    /// How many answers the replay has still to hand out.
    #[must_use]
    pub fn replay_left(&self) -> usize {
        self.store.data().tape.left()
    }

    /// The profile sections' spans ended since they were last taken, in
    /// the order they ended.
    pub fn take_profile_spans(&mut self) -> Vec<Span> {
        self.store.data().profile().take()
    }

    /// Sets the game's data folder (`d:\`, the maps) and save folder (the
    /// other drives); until then no path is found.
    pub fn set_folders(&mut self, data: &std::path::Path, saves: &std::path::Path) {
        self.store.data().files().set_folders(data, saves);
        self.store
            .data()
            .settings()
            .keep_in(&saves.join(config::KEPT));
    }

    /// Sets the console commands the game runs as it starts, as its
    /// `d:\init.txt` ([`files::Files::set_init_commands`]).
    pub fn set_init_commands(&mut self, text: &str) {
        self.store.data().files().set_init_commands(text);
    }

    /// Makes the game ready to run `halo_game_main`: its data and save
    /// folders ([`Game::set_folders`]), its memory grown past the Xbox's
    /// window, where the game state and the tag cache are placed
    /// (`memory::WINDOW_BASE`), and the drawable its screen is drawn to
    /// ([`Game::set_drawable_size`]).
    ///
    /// # Errors
    /// [`Error::Grow`] if the memory cannot grow past the window.
    pub fn prepare(
        &mut self,
        data: &std::path::Path,
        saves: &std::path::Path,
        display: (u32, u32),
    ) -> Result<(), Error> {
        self.set_folders(data, saves);
        self.grow_memory_to(u64::from(memory::WINDOW_BASE) + u64::from(memory::WINDOW_SIZE))?;
        self.set_drawable_size(display.0, display.1);
        Ok(())
    }

    /// Installs the app's renderer: each render batch the game submits is
    /// handed to it, checked. Without one, a batch is checked and dropped.
    pub fn set_render_sink(&mut self, sink: Box<dyn RenderSink>) {
        self.store.data().batches().set_sink(sink);
    }

    /// Sets the drawable's size in pixels, as the game's Metal device asks
    /// it (`host_drawable_width`); until then, the Xbox's 640 by 480.
    pub fn set_drawable_size(&mut self, width: u32, height: u32) {
        self.store.data().batches().set_drawable_size(width, height);
        self.store.data().input().set_area(width, height);
    }

    /// How many render batches and commands the game has submitted, and
    /// how many were refused.
    #[must_use]
    pub fn render_counts(&self) -> RenderCounts {
        self.store.data().batches().counts()
    }

    /// The first refusals of the game's render batches and commands, each
    /// as text.
    #[must_use]
    pub fn render_refusals(&self) -> Vec<String> {
        self.store.data().batches().refusals()
    }

    /// A key going down or up: its Windows virtual key, its character (0
    /// if none) and its keystroke flags (shift, control, alt and so on).
    pub fn key(&mut self, vk: u8, ascii: u8, flags: u8, down: bool) {
        self.store.data().input().key(vk, ascii, flags, down);
    }

    /// The mouse moving by `dx` and `dy` pixels, right and down.
    pub fn mouse_motion(&mut self, dx: f32, dy: f32) {
        self.store.data().input().motion(dx, dy);
    }

    /// A mouse button going down or up, numbered as SDL numbers them: 1
    /// the left, 2 the middle, 3 the right, 4 and on the side buttons.
    pub fn mouse_button(&mut self, button: u8, down: bool) {
        self.store.data().input().button(button, down);
    }

    /// The window putting the mouse at `x` and `y`, the drawable's pixels
    /// from its top left: the menus' pointer.
    pub fn mouse_position(&mut self, x: f32, y: f32) {
        self.store.data().input().position(x, y);
    }

    /// The mouse's wheel turning by `notches`, up positive.
    pub fn mouse_wheel(&mut self, notches: f32) {
        self.store.data().input().wheel(notches);
    }

    /// The screenshots the player has asked for since the last call.
    pub fn take_screenshots(&mut self) -> u32 {
        self.store.data().input().take_screenshots()
    }

    /// The game's keyboard and mouse, to hand input to from another thread
    /// while `halo_game_main` keeps this game busy.
    #[must_use]
    pub fn controls(&self) -> Controls {
        Controls::new(Arc::clone(&self.store.data().shared))
    }

    /// The game's speakers, to pull its mixed sound from on the audio
    /// device's thread while `halo_game_main` keeps this game busy.
    #[must_use]
    pub fn speakers(&self) -> Speakers {
        Speakers::new(Arc::clone(&self.store.data().shared))
    }

    /// The game's telnet console, to hand it lines and take its replies
    /// from another thread while `halo_game_main` keeps this game busy.
    #[must_use]
    pub fn console(&self) -> Console {
        Console::new(Arc::clone(&self.store.data().shared))
    }

    /// The game's settings, to read and change from another thread while
    /// `halo_game_main` keeps this game busy.
    #[must_use]
    pub fn tuning(&self) -> Tuning {
        Tuning::new(Arc::clone(&self.store.data().shared))
    }

    /// Sets the game's setting `name` (as `section.key`) to `text`, as the
    /// settings file would have it; false if there is no such setting or
    /// the text is too long.
    pub fn set_setting(&mut self, name: &str, text: &str) -> bool {
        self.store.data().settings().write(name, text)
    }

    /// What each of the game's threads that trapped said, oldest first.
    #[must_use]
    pub fn thread_faults(&self) -> Vec<String> {
        self.store.data().shared.threads.faults()
    }

    /// The completion routines `ReadFileEx` and `WriteFileEx` queued, oldest
    /// first, for the game's next alertable wait.
    #[must_use]
    pub fn completions(&self) -> Vec<files::Completion> {
        self.store.data().files().completions().to_vec()
    }

    /// The "module.name" of import `index` (empty if there is none).
    #[must_use]
    pub fn import_name(&self, index: usize) -> String {
        self.store
            .data()
            .names
            .get(index)
            .cloned()
            .unwrap_or_default()
    }

    /// How many functions the module imports.
    #[must_use]
    pub fn import_count(&self) -> usize {
        self.store.data().names.len()
    }

    /// How many times the import "module.name" was called.
    #[must_use]
    pub fn call_count(&self, name: &str) -> u64 {
        let record = self.store.data();
        record
            .names
            .iter()
            .position(|import| import == name)
            .and_then(|index| record.counts.get(index))
            .copied()
            .unwrap_or(0)
    }

    /// Whether the module exports a function `name`.
    #[must_use]
    pub fn has_export(&self, name: &str) -> bool {
        self.functions.iter().any(|function| function == name)
    }

    /// The address of the data the module exports as `name` (an i32
    /// global, as wasm-ld exports a data symbol), if it exports it.
    #[must_use]
    pub fn data_export(&self, name: &str) -> Option<u32> {
        self.store.data().shared.plugins.export(name)
    }

    /// The size of the game's memory in bytes (0 if there is none).
    #[must_use]
    pub fn memory_size(&self) -> u64 {
        match &self.memory {
            Some(Mem::Own(memory)) => memory.data_size(&self.store) as u64,
            Some(Mem::Shared(memory)) => memory.data_size() as u64,
            None => 0,
        }
    }

    /// Grows the game's memory to at least `bytes` (rounded up to whole
    /// wasm pages); does nothing if it is that large already.
    ///
    /// # Errors
    /// [`Error::Grow`] if there is no memory, or it cannot grow that far
    /// (its maximum, or the host's).
    pub fn grow_memory_to(&mut self, bytes: u64) -> Result<(), Error> {
        match self.memory.clone().ok_or(Error::Grow(bytes))? {
            Mem::Own(memory) => {
                let want = bytes.div_ceil(memory.page_size(&self.store));
                let have = memory.size(&self.store);
                if want > have {
                    memory
                        .grow(&mut self.store, want - have)
                        .map_err(|_| Error::Grow(bytes))?;
                }
            }
            Mem::Shared(memory) => {
                let want = bytes.div_ceil(memory.page_size());
                let have = memory.size();
                if want > have {
                    memory.grow(want - have).map_err(|_| Error::Grow(bytes))?;
                }
            }
        }
        Ok(())
    }

    /// what an access of `len` bytes at `address` outside the memory is
    fn outside(&self, address: u64, len: u64) -> Error {
        Error::OutOfBounds {
            address,
            len,
            size: self.memory_size(),
        }
    }

    /// Reads `len` bytes of the memory at `address`.
    ///
    /// # Errors
    /// [`Error::OutOfBounds`] if any of them is outside the memory.
    pub fn read(&self, address: u64, len: u64) -> Result<Vec<u8>, Error> {
        let (Ok(at), Ok(count)) = (u32::try_from(address), u32::try_from(len)) else {
            return Err(self.outside(address, len));
        };
        let read = match &self.memory {
            Some(Mem::Own(memory)) => {
                let (start, end) = (at as usize, (at as usize).checked_add(count as usize));
                end.and_then(|end| memory.data(&self.store).get(start..end))
                    .map(<[u8]>::to_vec)
            }
            Some(Mem::Shared(memory)) => Space::shared(memory.data()).read(at, count),
            None => None,
        };
        read.ok_or_else(|| self.outside(address, len))
    }

    /// Writes `bytes` to the memory at `address`.
    ///
    /// # Errors
    /// [`Error::OutOfBounds`] if any of them would be outside the memory.
    pub fn write(&mut self, address: u64, bytes: &[u8]) -> Result<(), Error> {
        let len = bytes.len() as u64;
        let Ok(at) = u32::try_from(address) else {
            return Err(self.outside(address, len));
        };
        let written = match self.memory.clone() {
            Some(Mem::Own(memory)) => Space::own(memory.data_mut(&mut self.store)).write(at, bytes),
            Some(Mem::Shared(memory)) => Space::shared(memory.data()).write(at, bytes),
            None => false,
        };
        if written {
            Ok(())
        } else {
            Err(self.outside(address, len))
        }
    }
    /// Gives the loading screen's chant (progress_bar.c) its tone: the
    /// one in the player's xbe, or with `None` silence. game.wasm is built
    /// with none of its own ([`TONE_EXPORT`]); a buffer of silence is
    /// never mixed. False if the module does not export the tone.
    ///
    /// # Errors
    /// [`Error::OutOfBounds`] if the export is outside the memory.
    pub fn set_tone(&mut self, tone: Option<&[u8; TONE_BYTES]>) -> Result<bool, Error> {
        let Some(address) = self.data_export(TONE_EXPORT) else {
            return Ok(false);
        };
        self.write(u64::from(address), tone.unwrap_or(&[0; TONE_BYTES]))?;
        Ok(true)
    }
}
