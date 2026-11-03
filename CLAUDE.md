# Working on OpenCE

These are the practices the project has settled on. They come before
`AGENTS.md` where the two differ. The direction was set on 27 October
2026; its reasons are in `docs/retrospective_fork_in_the_road.md`.

## What the project is now

- **macOS only.** A native macOS app: Swift for the app and Metal for
  drawing, Rust under it (`engine/crates/`, `engine/app/`).
- **The game is the original's own code.** `source/` is compiled unchanged
  as wasm32 into `game.wasm` and run in Wasmtime inside the app (track
  `native_port_20261028`). It is not a browser build. Wasm's 32-bit
  pointers keep every tag layout and pointer in the maps as they are.
- **One typed import list.** The game calls only the Rust import list: the
  macOS platform layer and the plugin hooks. Plugins are written in Rust,
  and read the game's memory only through read-only, bounds-checked views.
  The Halo UI rewrite is a plugin.
- **Drawing.** Swift and Metal draw what the game's rasterizer asks for.
  The shaders first match the original's; then the lighting is extended
  from the original's own (lightmaps, shader tags, lights), each extension
  switchable back to stock.
- **The bar.** p99 ≤ 8.33 ms at 3840×2160, ultra, on the M1 Max
  (`--benchmark 60`).
- **Built ahead of time.** The metallib and `game.wasm` at build; the
  `.cwasm` and the Metal binary archive behind the loading screen, cached.
- **Keyboard and mouse only.** No gamepad or controller support; pad
  events are filtered out.
- **No launcher.** The game starts on its main menu, or on the map `--map`
  names. It runs through the CLI and Nix: `nix run .#engine` runs it,
  `nix build .#engine` builds it, and `nix develop` is the shell to work
  in.
- **Removed.** The Rust simulation, `bevy_ecs`, the `net` crate and
  `port/macos` (the Hypervisor.framework VM) were removed by Track A's
  last phase; do not bring them back.

## Still in force from `AGENTS.md`

- Never read the Xbox SDK's headers or anything derived from them; do not
  read `xbox/include`.
- `source/` changes as little as possible; each change is marked `port:`.
- The repository is CC0. GPL code (OpenSauce, Reclaimer) is read as
  documentation of formats, never copied. Vendored code goes in
  `port/third_party/<name>` with a README.
- Map, tag and other game files, and the game's memory read by a plugin,
  are untrusted input: read them through bounds-checked readers, and fuzz
  the readers.
- Art: hand-drawn redraws; fonts: Overpass and OpenCE only.
- Nothing from the user's game data (maps, `ui.map`, the xbe, Bungie's
  sounds) goes into the repository, the app bundle or an artifact.
  Generated data derived from it (devgulch, golden frames, bakes) stays on
  the user's machine.
- Do not imitate Bungie's or Microsoft's branding.

## Building and testing

- `nix develop`, then `python tools/engine_build.py` (or the crane
  packages, `nix build .#engine`). Do not leave `nix flake check` broken.
- Rust lints for the workspace (`engine/Cargo.toml`): `unsafe_code` is
  denied, `missing_docs` warns, clippy's `unwrap_used` and `expect_used`
  are denied, and release builds abort on panic. Write code that passes
  them rather than allowing them.
- Tests that need the user's data (`assets/maps`, `Haloce_dump`) skip
  themselves when it is missing; they never fail for want of it.
- Write the test first (`conductor/workflow.md`), and run the whole
  workspace's tests before a commit.

## Matching the original

- **Reference traces.** A `tools/engine_*_reference.py` script compiles
  the needed functions from `source/` natively against a small fake world
  and records a trace; the wasm build must match it. `--check` fails when
  the trace is stale.
- **The census** (`opence census --check`) classes every function of the
  original, and stays current.
- **Determinism.** The simulation runs at 60 Hz and scripts at 30 Hz, as
  the original runs them. `game.wasm` uses musl's maths and no fused
  multiply-add; Rust maths that must repeat goes through `libm`. Maps
  iterated where order matters iterate in a fixed order.
- **Measure, don't assume.** Compare with the stock game by measurement
  (image differences per region, shadow edges, sound, timings), in
  systematic sweeps. A check that passes is not proof that the result
  looks right. The Gearbox PC port's losses are measured against the Xbox
  original, not taken from it.

## Running the game

- Run headless only, never on the user's desktop, and stop only the
  processes you started.
- **devgulch** is the test level: a copy of Blood Gulch made from the
  user's map at import, with a station for each system
  (`--map devgulch --station vehicles`).
- Every dev panel control is also a shell command and a control socket
  call (the `opence` tool), so anything the user can do in the panel can
  be scripted and tested.
- Bakes and probes never run during play: at import, or behind the loading
  screen, cached by the map's hash.

## How the work runs

- **Claude writes all the code.** The user reads it. Do not leave
  `TODO(human)` or ask the user to write code; explanatory insight notes
  are welcome.
- **Conductor tracks** (`conductor/tracks.md`) hold the plans, in order:
  `native_port`, `halo_ui`, `map_audit`, `lighting_original`. A track runs
  all its phases to the end without stopping for manual checks, with
  automated checks at each phase's end. There is no report phase, except
  where a track names one (A1's go/no-go).
- **Designs** (screens, menus, layouts) are drawn as static prototypes in a
  Claude design artifact, not as markdown.
- **Commits.** The subject is a plain sentence saying what changed; the
  body says why and what was checked. Stage only your own files: other
  sessions often have uncommitted work in the same checkout. Do not push
  unless asked.

## Nix

- The flake's outputs are grown with std from the cells in `nix/` (`repo`,
  `macos`, `engine`). std copies each cell into the store on its own, so
  refer to the repository through the toolchain's `root` path, never
  `../..`.
- Pass paths, not `"${inputs.self}/..."` strings, to `importCargoLock` and
  crane: a path copies only the named file, so vendored crates build again
  only when the lock file changes.
