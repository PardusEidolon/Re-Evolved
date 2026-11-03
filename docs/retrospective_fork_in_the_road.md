# The fork in the road: a retrospective

Written 27 October 2026, at the start of Track B. It records why the
project stopped rewriting the game in Rust, what the review that followed
decided, and what is kept. The review itself ran in six rounds on a Claude
artifact (https://claude.ai/artifact/VUVoi3qyFTTfTbpiiQRaMo); this file is
the record that lives in the repository.

## Where the project stood

The project has run the original three ways:

| When | How the original ran | What it gave | What it cost |
| --- | --- | --- | --- |
| to 18 Oct | The old ports (Linux, Windows, Android, the web): `source/` compiled as it is, with 32-bit pointers (`arm64_32`, `wasm32`) | The game, unchanged | The original's limits: its resolution, its UI, its renderer |
| 18–21 Oct | `port/macos`: an `arm64_32` guest in a Hypervisor.framework VM, at its Xbox addresses | The game on macOS. 4.17 ms p50, 4.78 ms p99 at 4K on the M1 Max, with the batched render buffer (`docs/macos_host.md`) | An entitlement, a VM exit for every host call, a host in C full of raw pointers, a guest that lldb can't see into |
| 21–27 Oct | A new engine in Rust and Swift (`native_engine_20261021`): `source/` only a reference, maps imported to `.ocpak`, `bevy_ecs` for the simulation | A modern renderer: 4K, HDR, MetalFX, GPU bakes, ray tracing on M3 and later. Ultra holds 6.75–8.3 ms p99 with dynamic resolution (`docs/engine.md`) | The game itself had to be written again, function by function |

The rewrite is large. In the week from 21 to 27 October it reached about
166,000 lines of Rust in 14 crates and 36,000 lines of Swift, in 638
commits, with 87 reference-trace tests checking ported functions against
the original's own code.

## Why the rewrite fell short

The census (`docs/census.md`, written by `opence census --write`) classes
every one of the original's 7,627 functions against the engine. On 27
October:

| Class | Functions | Share | Bytes of the original |
| --- | ---: | ---: | ---: |
| Ported (traced line for line) | 332 | 4.4% | 54,144 |
| Matched (rewritten, behaviour checked) | 525 | 6.9% | 149,456 |
| Native (small helpers) | 332 | 4.4% | 38,896 |
| Partial | 174 | 2.3% | 106,624 |
| Replaced on purpose | 2,774 | 36.4% | 576,504 |
| **Missing** | **3,490** | **45.8%** | **685,498** |

The replaced functions are mostly the parts the engine was meant to
rewrite: the rasterizer (777), networking (288), the cache and bitmaps.
The missing ones are the game:

| System | Functions | Missing |
| --- | ---: | ---: |
| `hs` (scripts) | 639 | 576 (90%) |
| `ai` | 959 | 793 (83%) |
| `game` | 716 | 565 (79%) |
| `interface` | 677 | 452 (67%) |

What that looked like in play:

- **Behaviour drifted where nothing was traced.** The engine played the
  first two missions to their end, but the AI, the scripts and the flow
  were approximations. Each trace that was added found a difference.
- **The user's own play found what the tests missed.** The HUD's grenade
  count read 4 when it should read 0, the first-person rifle and hand sat
  in the wrong place, and the weapons station's particles were incomplete.
  Each was a part of the original that had not been ported yet.
- **The coverage tracks would take months.** Eleven tracks were planned to
  bring the census to zero missing. The first four (the census, the
  classing, the evidence for what existed and the data fidelity) closed,
  and the fifth, units, was part way through when it paused. At that rate the AI and the scripts
  alone were weeks each, and the result would still be a re-creation that
  had to be proved equal to code we already have.

The lesson: **porting is the long way to the original's behaviour when the
original's code is in hand.** A rewrite earns its cost where we want
something new (the renderer, the UI, the lighting), not where we want the
same thing.

## The review

On 27 October the user first proposed a hybrid: `source/`'s simulation
compiled natively, with Rust and Metal owning presentation. The coverage
tracks paused (e42f73e1) and the direction went to a review, round by
round, each section marked Approve, Revise or Decline with notes.

| Round | Asked | Decided |
| --- | --- | --- |
| 1–2 | The hybrid's shape; what to keep | The original engine drives, kept intact; Rust extends it through plugins. Stock rendering is the reference. Drop Bevy. Remove the launcher. Keep every dev tool, the census and the traces. Include what the PC port left out. Track B as the clean-up. |
| 3 | Can 32-bit pointers live in a 64-bit process? Keep the VM, or use a unikernel? | Only as offsets. Compiled for arm64 the tag structs grow and the map's 4-byte pointers no longer fit; macOS 26 refuses memory below 4 GB and kills a small page-zero (`port/macos/spikes/low_memory.c`). `wasm32` is ILP32, so `source/` compiles unchanged. Netcode is left as it is, for a rework later. No separate report phase. |
| 4 | One uniform interface; unikernels | One typed import list in Rust: the macOS platform layer and the plugin hooks. Unikernels rejected: Hermit, Unikraft, OSv and Nanos are all 64-bit, and would run inside the VM anyway. The Halo UI rewrite joins the order as its own track. |
| 5 | The 4K p99 target | p99 ≤ 8.33 ms at 4K on the M1 Max is the A1 go/no-go. The old VM's numbers show the runtime is not the limit (79 exits a frame at about 1 µs each); the GPU is. |
| 5 (talk) | One runtime or two? Desktop or web? | One runtime: Wasmtime. The Hypervisor.framework VM is dropped and goes with `port/macos` in A7; git keeps it. Wasm is portable, but the app is the native macOS desktop app, not a browser build. |
| 6 | Shaders and compilation | Everything compiled ahead of time: a `default.metallib` and `game.wasm` at build; `.cwasm` and a Metal binary archive behind the loading screen, cached in the user's cache folder by the hashes that matter. A pipeline missing during play is a failing test. All approved; final. |

## The direction

- **`source/` compiled unchanged as `wasm32` and run in Wasmtime**,
  embedded in the native macOS app. Wasm's 32-bit pointers are offsets
  into 4 GB of linear memory, so the tag layouts and the map's pointers
  hold. Cranelift compiles the module to arm64 ahead of time. The C is
  sandboxed, its floats are deterministic, and plugins read its memory as
  a plain byte slice in safe Rust.
- **One typed import list in Rust**: the macOS platform layer (learned
  from how the Xbox build behaves and from the old ports' hosts, never from
  the Xbox SDK's headers) and the plugin hooks. Plugins and overlays are
  Rust. The game still decides everything.
- **Swift and Metal draw at the rasterizer.** The shaders match the
  original first; then the original's own lighting is extended (its
  lightmaps, shader tags and lights, with the GPU bakes and ray tracing on
  top). Bevy is dropped.
- **macOS only**, desktop only. The game layer doesn't depend on the OS,
  but nothing else is built, tested or shipped.
- **The bar**: p99 ≤ 8.33 ms at 4K. If A1 misses it, the track stops and
  the data comes to the user; there is no silent fallback.
- **The order**: B (this clean-up) → A, the native port
  (`native_port_20261028`) → the Halo UI rewrite (`halo_ui_20261028`), a
  plugin drawn at 4K from `ui.map` → the map and cutscene audit → the
  lighting extended from the original → later, as the user chooses: the
  living world, kev-4b as a plugin on the AI hook, and netcode reworked
  from scratch.

## What is kept, and what goes

Kept:

- every dev tool: the dev shell, the control socket and the `opence` tool,
  devgulch and its stations;
- the census, the reference traces and the `tools/engine_*_reference.py`
  scripts. They check the engine's Rust ports today, and they become checks
  on the wasm build: the same functions, now run from the original's code;
- the importer's bounds-checked readers and their fuzzing, the GPU bakes,
  the renderer's work where it extends the original rather than replaces it;
- the Gearbox PC port's fixes, catalogued and made in Track A.

Archived for reference (`conductor/archive`): coverage tracks 4 to 11, the
living world, kev-4b, and the Rust `net` crate.

Removed: the launcher (Track B), then in Track A's last phase the Rust
simulation, `bevy_ecs`, the presentation code the plugins replace, and
`port/macos` with its VM.

## What we would do differently

- **Measure the size of a port before starting it.** The census came on
  26 October, five days into the rewrite. Had it come first, the 3,490
  missing functions would have been on the table before the decision.
- **Keep the original running beside anything new.** The old VM met the
  performance bar; the problem was its host, not the idea of running the
  original. Replacing the runtime and the game at once lost the reference
  we needed to check against.
- **Rewrite only what we mean to change.** The renderer, the UI and the
  lighting are worth rewriting. The AI, the scripts and the game's rules
  are worth keeping exactly as they are.
