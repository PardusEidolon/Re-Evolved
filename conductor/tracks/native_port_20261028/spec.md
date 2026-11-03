# Spec: The native port, the original run as wasm32 in Wasmtime

## Overview
The game is the original's own code again. `source/` is compiled unchanged
as wasm32 into `game.wasm` and run in Wasmtime, a Rust library linked into
the native macOS app. Wasm's pointers are 32-bit offsets into its linear
memory, so every tag layout, size assert and 4-byte pointer in the maps
holds, and the C needs no edits. The C is sandboxed: a bug in it cannot
corrupt the host, and the host reads its memory as a plain byte slice, in
safe Rust.

Under the game is one typed import list, in Rust: the macOS platform layer
and the plugin hooks. The game calls only these. Swift and Metal draw what
the game's rasterizer asks for, first matching the original's shaders and
then extending its own lighting. The presentation the engine built since
21 October (the renderer's bakes and probes, the particles, the HUD, the
sound) becomes plugins on the hooks, and the game still decides
everything.

The decision and its reasons are in
`docs/retrospective_fork_in_the_road.md` and the review artifact
(https://claude.ai/artifact/VUVoi3qyFTTfTbpiiQRaMo).

## Functional Requirements

### A1: The spike, then go or no-go
1. **`game.wasm`.** `source/` is built as wasm32 with clang, from the web
   build's flags (`tools/web_build.py`): the variadic prototypes header,
   since wasm needs a call's exact type; musl's maths, for determinism; and
   the game's threads. Nix builds it; `nix build .#engine` carries it.
2. **The runtime crate.** A new crate embeds Wasmtime. It compiles
   `game.wasm` with Cranelift to native code ahead of time (`.cwasm`, keyed
   by Wasmtime's version and the CPU), caches it in the user's cache
   folder, and gives the game a 4 GB linear memory with the Xbox's memory
   window where the web build put it.
3. **Stub imports.** Every import the game asks for exists, as a stub that
   records its calls.
4. **The run.** devgulch runs headless for some thousands of ticks.
5. **The checks.**
   - The reference traces (`tools/engine_*_reference.py`) are recorded from
     the wasm build too, and must match the natively compiled reference.
   - `--benchmark 60` records the simulation tick's p99, which must be
     within about 1 ms, and the CPU frame's p99 (the guest's ticks and its
     rasterizer buffer, with drawing stubbed), which must be at most
     8.33 ms. The GPU draws in parallel; A4 measures the whole frame.
   - The game's sound and file threads run as host threads on wasm shared
     memory, with atomics.
   - `setjmp` and `longjmp` work, through Wasmtime's exception handling or
     the lowering the toolchain offers.
   - Floating point is deterministic: no fused multiply-add, and the same
     results on every run.
6. **The go/no-go.** A data-backed write-up to the user, the track's only
   stop. If the traces match and the bar holds, the track carries on. If
   not, and it cannot be fixed, the track stops with the data. There is no
   silent fallback.

### A2: The macOS platform layer
7. One service at a time, each tested on its own and then on devgulch:
   - memory and the map cache;
   - files: the data folder, saved games, and the copies of maps the game
     keeps, all under the user's folders;
   - threads, timers and the clock;
   - keyboard and mouse, as the game's input; pad events filtered out;
   - sound output, from the game's own mixer to Core Audio;
   - the render calls, as one buffer per frame (A4 draws it).
8. What the game expects from each service is learned from how the Xbox
   build behaves and from the old ports' hosts (`port/android`,
   `port/web`, `port/macos/host`), never from the Xbox SDK's headers.
9. **The dev tools run against the game.** The dev shell, the control
   socket and the `opence` tool reach the wasm game. devgulch is a map the
   original loads, with its stations. Every dev panel control is still a
   shell command and a socket call.

### A3: The plugin interface
10. **Hooks** as imports and exports of the one list: per tick, per frame,
    rendering, the HUD and UI, sound, and the AI (for later plugins).
11. **Views of the game's memory**, read-only, through bounds-checked
    readers of its structures and tags, like the importer's.
12. **Recorded inputs**, so a run can be replayed exactly.
13. **Budgets.** Each plugin's time is logged per frame. A plugin over its
    share is a failing test.
14. With every plugin off, a run matches the reference traces. The readers
    and the hooks' inputs are fuzzed.

### A4: Stock rendering on Metal
15. Swift and Metal draw the rasterizer's buffer: every shader the original
    has (environment, model, transparent, chicago and the rest) and its
    lighting (lightmaps, dynamic lights, fog), ported onto Metal.
16. Each is measured against the original by golden frames and image
    differences, until it matches. The user re-approves the golden frames.
17. **Compiled ahead of time.**
    - At build, Nix links the shaders' `.air` into one `default.metallib`
      in the bundle, loaded with `makeLibrary(URL:)`; the source is never
      compiled at startup.
    - Behind the loading screen, the pipelines a map needs are built into a
      Metal binary archive, cached by the map's hash, the GPU, the OS and
      the app's build, in the user's cache folder; a stale one is rebuilt.
    - A pipeline asked for during play that isn't in the archive is a
      failing test.
18. The bar holds end to end: at 3840×2160, ultra, on the M1 Max, p99 at
    most 8.33 ms (`--benchmark 60`, `QualityTierTests`).

### A5: The presentation as plugins
19. The engine's presentation moves onto the hooks:
    - the lighting extended from the original: the GPU bakes, probes, and
      ray-traced shadows and reflections on M3 and later, built on the
      original's lightmaps, shader tags and lights, with no second lighting
      system;
    - the particles;
    - the HUD at 4K;
    - the sound.
20. Every extended shader still matches the original where the original has
    that effect, and each can be switched back to stock.
21. **The user's feedback of 27 October is checked here:**
    - the HUD's grenade count reads what the game says (0 in the frame
      `bloodgulch_hud_1920x1080`, not 4), and every HUD element matches the
      original in placement, scale, counts and colours;
    - the first-person rifle and hand sit where the original puts them;
    - the weapons station draws every particle the game makes (firing,
      impacts, contrails).

### A6: What the PC port left out
22. The effects and behaviour the Gearbox PC port lost or broke against the
    Xbox original are catalogued, each measured against the original's own
    code and frames, and fixed where the fix falls to the presentation.

### A7: Removals and checks
23. Removed, guided by the census: the Rust simulation, `bevy_ecs`, the
    presentation code the plugins replace, the `net` crate, and
    `port/macos` with its Hypervisor.framework VM. Git keeps them.
24. The census is reclassed for the new direction: functions run from the
    original's code, the rasterizer functions Metal stands in for, and what
    is still missing. Its `--check` stays current.
25. The traces become checks that the wasm build matches the natively
    compiled reference.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`).
- `source/` is compiled unchanged. Where a change is unavoidable it is the
  smallest one, marked with a `port:` comment, as `AGENTS.md` says.
- Determinism: the simulation at 60 Hz and scripts at 30 Hz as the original
  runs them; no fused multiply-add on either side of the import list.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`,
  `missing_docs`). Wasmtime's own unsafe stays inside Wasmtime.
- Map, tag and other game files are untrusted: read through bounds-checked
  readers, fuzzed.
- Run headless only. Tests that need the user's data skip without it.
- Nothing from the user's game data enters the repository or the bundle:
  `game.wasm` is built from `source/`; maps, traces, golden frames, the
  `.cwasm` and the binary archive stay on the user's machine.
- Each phase ends with its automated checks: the workspace tests, the
  traces, `census --check` and `nix flake check`. There is no report phase
  apart from A1's go/no-go.

## Acceptance Criteria
- A1's go/no-go is written and the user has it; the track went on only on
  a go.
- devgulch and Blood Gulch play in the app from `game.wasm`, through the
  CLI and Nix, with keyboard and mouse.
- The reference traces match from the wasm build, with every plugin off.
- The stock shaders match the original's golden frames, which the user has
  re-approved.
- p99 ≤ 8.33 ms at 4K ultra on the M1 Max, with the presentation plugins
  on; no pipeline is built during play.
- The three items of feedback are fixed and measured.
- The removals are done, the census is reclassed, and `nix flake check`
  passes.

## Out of Scope
- The Halo UI rewrite (`halo_ui_20261028`), next.
- The map and cutscene audit (`map_audit_20261028`).
- The lighting extended further (`lighting_original_20261028`).
- Netcode: the original's is compiled in and left alone; a later track
  reworks it.
- Any target but macOS.
