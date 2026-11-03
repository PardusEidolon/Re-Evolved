# Tech Stack: OpenCE (macOS)

**From 2026-10-27** (track `native_port_20261028`). The reasons are in
`docs/retrospective_fork_in_the_road.md`.

## The Game
- **C (the decompilation, `source/`)**, compiled unchanged as **wasm32**
  with clang into `game.wasm`, from the web build's flags
  (`tools/web_build.py`): the variadic prototypes header (a call must match
  its callee's type exactly), the vendored musl maths, no fused
  multiply-add, and the game's threads on wasm shared memory with atomics.
  wasm32 is ILP32, so the tags' 4-byte pointers and the size asserts hold.
- **Wasmtime**, embedded as a Rust library in the app. `game.wasm` is
  compiled with Cranelift ahead of time to a `.cwasm`, keyed by Wasmtime's
  version and the CPU and cached in the user's cache folder. The game gets
  a 4 GB linear memory with the Xbox's memory window where the web build
  put it. `setjmp` and `longjmp` go through Wasmtime's exception handling
  or the toolchain's lowering.

## The Import List (Rust, `engine/crates/`)
- **Rust (stable, edition 2021), a Cargo workspace.** One typed import
  list: the macOS platform layer and the plugin hooks.
- **The platform layer:** memory and the map cache, files under the user's
  folders, threads, timers and the clock, keyboard and mouse (pad events
  filtered out), sound from the game's mixer, and the render calls as one
  buffer per frame.
- **Plugins:** Rust, on hooks per tick, per frame, rendering, the HUD and
  UI, sound and the AI. They read the game's memory through read-only,
  bounds-checked views. Each plugin's time is logged per frame.
- **Kept:** the importer (`import`, `content`), the census, the dev tools
  (`dev`, `cli`: the `opence` tool, the control socket), and `ffi`.
- **Being removed** in Track A's last phase: the Rust simulation (`sim`,
  `campaign`, `script`, `physics`), `bevy_ecs` and `net`.
- **Dependencies:** crates pinned in `engine/Cargo.lock`, vendored by Nix
  (`importCargoLock`), licences checked by `cargo deny`: MIT, Apache-2.0,
  BSD, Zlib, ISC, Unicode or CC0 only.

## The App (Swift, `engine/app/`)
- AppKit windowing with a `CAMetalLayer`: full screen at any aspect ratio,
  cursor capture and raw mouse input.
- **Metal** draws the rasterizer's buffer: the original's shaders first,
  matched by golden frames, then the lighting extended from the original's
  own, each extension switchable to stock. Ray tracing on M3 and later,
  with the rasterized fallback chosen by the quality setting.
- **Ahead of time:** the shaders' `.air` is linked by Nix into one
  `default.metallib`, loaded with `makeLibrary(URL:)`. The pipelines a map
  needs go into a Metal binary archive behind the loading screen, cached by
  the map's hash, the GPU, the OS and the app's build.
- Sound output to Core Audio, from the game's own mixer.
- UniFFI carries the calls between Swift and Rust.

## Build & Toolchain
- **Nix (flake, cells grown with std in `nix/`):** `nix develop`,
  `nix build .#engine`, `nix run .#engine`, `nix flake check`.
- **Xcode's Swift toolchain and SwiftPM** for the app, XCTest, Metal
  shader compilation and code signing, run under Nix as the host's impure
  Apple toolchain.
- **Python 3:** `tools/engine_build.py`, the reference scripts and the
  tests.

## Testing
- `cargo test` and `cargo fuzz` (the readers, and the views plugins use);
  XCTest for the Swift layer.
- **Reference traces:** `tools/engine_*_reference.py` compile the needed
  functions from `source/` natively against a small fake world; the wasm
  build must match them.
- **The census** (`opence census --check`).
- **Golden frames and image differences** against stock.
- **The benchmark** (`--benchmark 60`): the simulation tick's p99, the CPU
  frame's p99 and the whole frame's, against p99 ≤ 8.33 ms at 4K ultra on
  the M1 Max.

## Removed
- The launcher.
- The Hypervisor.framework VM (`port/macos`) and the native Rust
  simulation (native_port Phase 7).
- Linux, Windows, Android and browser builds. Their hosts stay in `port/`
  as a reference for what the game expects of its platform.
