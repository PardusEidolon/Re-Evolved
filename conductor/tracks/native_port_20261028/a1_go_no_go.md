# A1: go or no-go

28 October 2026, on the M1 Max. The spike's question: can `source/`, built
unchanged as wasm32 and run in Wasmtime, be the game? The spec sets two
criteria, the reference traces and the bar, and three checks on what the
build needs (threads, `setjmp`, floating point).

## Decision: go, with the bar measured where it first can be

The traces match, the three checks hold, and every speed figure that can be
measured now points well inside the bar. The bar itself, `--benchmark 60`
on the game, cannot be measured yet: the game does not tick until the
platform layer answers its imports (below). So the track carries on into
A2, and the benchmark runs as soon as the game ticks on devgulch, at the
end of A2's threads task. **If it misses the bar there, the track stops
then, with the measurements, as a no-go would have stopped it here.** There
is no fallback in between.

## The criteria

| Criterion | Bar | Measured | |
|---|---|---|---|
| Reference traces, wasm against native | all match | 63 of 63 match, twice, identical | met |
| Simulation tick p99 | about 1 ms | not measurable before A2 | moved |
| CPU frame p99 at 4K ultra | ≤ 8.33 ms | not measurable before A2 | moved |

## The traces

`tools/engine_wasm_traces.py` builds each of the 63 reference harnesses
with the game's ABI flags as a WASI command, runs it in the runtime
(`opence-wasm`), and compares its trace with the native reference's
`--check`. All 63 match (about 18 s with every core). The wasm build found
one bug in a harness: the physics_variable LCG kept its seed in
`unsigned long`, 32 bits on wasm32 as on the Xbox, and the native harness
had been 64-bit.

## What the build needs (task 4, 3ebac490)

- **Threads.** Two host threads, each with its own store and instance on
  one shared memory and one engine, add 100,000 each with atomics and see
  200,000. The game's threads (the cache, its decompression, input, and
  `thread_win32.c`) are made through Win32 imports, which A2 maps onto
  host threads this way.
- **`setjmp` and `longjmp`.** `source/` uses neither; game.wasm imports
  neither, so it needs no wasm exceptions or unwinding.
- **Floating point.** The game is built with `-ffp-contract=off` and
  musl's maths. Wasmtime canonicalises NaNs (a payload NaN plus one gives
  `0x7fc00000`), and the runtime refuses relaxed SIMD, whose fused
  multiply-add may differ between machines. The traces run twice gave the
  same output.

## Speed

### The harnesses, wasm against native

`tools/engine_wasm_traces.py --speed` builds each harness natively too and
times both, one script at a time: each the fastest of five runs after one
unmeasured (macOS checks a newly linked binary on its first run, which took
a quarter of a second and made the first measurement meaningless). Native
is the process less an empty process (`/usr/bin/true`, 2.4 ms); wasm is the
run alone, the runtime's start and load measured apart. Both are built at
the harnesses' own `-O1`.

The 15 harnesses whose native run takes 10 ms or more:

| Harness | Native | Wasm | Ratio |
|---|---:|---:|---:|
| unit-hands | 73.0 ms | 115.7 ms | 1.58× |
| unit-items | 73.0 ms | 113.3 ms | 1.55× |
| ctf | 56.8 ms | 97.3 ms | 1.71× |
| unit-aiming-bounds | 46.3 ms | 89.3 ms | 1.93× |
| devices | 35.4 ms | 61.8 ms | 1.75× |
| unit-aiming-plan | 34.6 ms | 50.2 ms | 1.45× |
| unit-flaming | 28.7 ms | 39.7 ms | 1.38× |
| vehicle-physics | 26.1 ms | 29.1 ms | 1.11× |
| data | 25.8 ms | 29.8 ms | 1.16× |
| spawn | 25.6 ms | 31.8 ms | 1.24× |
| physics-variable | 16.0 ms | 15.5 ms | 0.97× |
| unit-seats | 15.6 ms | 19.8 ms | 1.27× |
| cause-damage | 13.4 ms | 22.3 ms | 1.67× |
| math | 11.6 ms | 11.5 ms | 0.99× |
| particle | 11.3 ms | 14.6 ms | 1.29× |

Geometric mean **1.38×** (0.97× to 1.93×). Over the 43 harnesses above
the 2 ms floor it is 1.08×, but most of those are a few milliseconds, where
process start dominates; the 15 above are the figure to use. The game
itself builds at `-O2`, where the ratio may differ; the benchmark will say.

### What this means for the bar (an estimate, not a measurement)

The original's simulation ran at 30 Hz on the Xbox's 733 MHz Pentium III
inside its frame. A performance core of the M1 Max is many times faster per
clock and runs at over four times the clock, so even at twice native cost
the tick should be a small fraction of the 1 ms target. This is reasoning
from the hardware, and it is why the bar's measurement can wait for A2
without risk to the queue; it is not the measurement.

### Loading

| | game.wasm (14 MB) |
|---|---:|
| Load, no cache (compiled each time) | 313–317 ms |
| Load, first with the cache (compiled and written, 3.4 MB) | 387 ms |
| Load, cached `.cwasm` | 54 ms |

The spec puts the compile behind the loading screen; 54 ms cached is well
inside it.

### A host call

With every import a stub, a host call costs about 51 ns (20 million calls
in 1.03 s, including the recording). The platform layer's imports are
called per frame and per sound buffer, not per instruction, so this is not
on the bar's path; A3's hooks are measured in their own budget task.

## Why the bar cannot be measured yet

With every import stubbed to return 0, `halo_game_main` first calls
`DmWalkLoadedModules` once, then `DmWalkModuleSections` without end: the
stub never reports the end of the list. A call budget stops it. Getting to
the first tick needs A2's memory, files and threads services and an answer
to the debug monitor's walk. The benchmark is A2's measurement to make.

## Not done in A1

- **devgulch for thousands of ticks** moved to A2's dev shell task, for the
  same reason.
- **The known flaky test** `a_frame_of_the_ai_station_keeps_to_its_budget`
  (279.5–279.7 allocations a frame against 200) fails at HEAD as before
  this track. It measures the Rust simulation, which Phase 7 removes.

## The bar, measured in A2 (28 October 2026)

`opence-game --map 'levels\test\bloodgulch\bloodgulch' --benchmark 60`,
release build, on the Apple M1 Max (10 cores, 64 GB), headless, at
3840×2160. The game's own `profile_scope` sections are imports
(`port/wasmtime/include/profile_sections.h`), so each span is the game's
step as `source/` names it. A frame runs from one `input` section to the
next, less the vblank throttle and the first thread's `Sleep`: headless,
the stock game paces itself at 30 Hz with `Sleep(33 - elapsed)` in
`main_update_time` (`source/main/main.c`), which is waiting, not work. A
tick runs from `game_tick.cheats_enforce`'s entry to
`game_tick.player_effect`'s exit. The load (1.09 s) is left out.

| Measure | Count | p50 | p99 | max |
|---|---:|---:|---:|---:|
| CPU frame, less throttle and sleeps | 1,477 | 4.425 ms | **7.113 ms** | 16.529 ms |
| Simulation tick | 1,800 | 0.818 ms | 2.988 ms | 8.556 ms |
| `game_tick.objects` | 1,800 | 0.757 ms | 2.850 ms | 8.488 ms |
| `game_time_update` | 1,476 | 0.993 ms | 3.511 ms | 8.651 ms |
| `Sleep` (30 Hz pacing) | 1,476 | 37.499 ms | 42.044 ms | 56.264 ms |

**Go.** The CPU frame's p99, 7.113 ms, is under the 8.33 ms bar, so the
track continues.

What the number leaves open, each measured again where it is due:

- **The draws are stubbed.** Until Phase 4 the rasterizer's calls go to
  the D3D stubs and no Metal work is done; the end-to-end bar is Phase 4's
  task. About 3.4 ms of the 4.4 ms median frame falls outside every
  profile section; the only step of the frame without one is
  `main_game_render` (between `non_deterministic_update` and `throttle`
  in `main_loop`), so that is the render's CPU work feeding the stubs,
  by elimination rather than by its own section.
- **The tick misses its "about 1 ms" aim at p99**: 2.988 ms, nearly all
  `game_tick.objects`. Its median, 0.818 ms, is inside the aim. The tick
  runs at 30 Hz here, so it fits its 33 ms period many times over; the
  objects step is the place to look when the aim is chased.
- **Blood Gulch, not devgulch.** devgulch is made at import from the
  user's map, and its import into a map the original loads is Phase 2's
  dev shell task; the run is on the stock map it is made from.
- **Wall clock against CPU.** The process used 8.66 s of CPU over 90.7 s,
  about 10 %: most of every frame is the 30 Hz sleep.
