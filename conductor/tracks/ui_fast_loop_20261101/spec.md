# Spec: A fast loop for the Halo UI

## Overview

A change to the menus (screens.rs's layout, text, titles) today takes a full
`python tools/engine_build.py` (cargo release, the Swift link, signing), a
relaunch of the app, a scripted socket walk to the screen, and a shot. This
track makes a halo_ui layout change show in the running app in under 2
seconds, and adds golden images of each screen to check changes against.

## Functional requirements

1. **The layout runs as wasm.** halo_ui's layout (the widget tree to the
   Scene) moves into its own crate, built for `wasm32-unknown-unknown` with no
   imports. The native plugin keeps the hooks, the shared scene and the
   pointer. At each `ui_update` the plugin copies the tree's snapshot in,
   calls `layout`, and reads the scene back through a bounds-checked reader.
   The module's output is untrusted input, and its reader is fuzzed. The same
   module ships in the app.
2. **Live swap.** `--ui-dev` (headless only) watches the layout crate. On a
   save it runs an incremental cargo build of the module, compiles it in the
   app's Wasmtime, swaps it in at the next `ui_update`, and writes a shot. A
   failure keeps the last good module and logs why.
3. **Opening a screen directly.** A control socket call and
   `opence ui open <screen>` open a screen by name through the game's own
   screen-opening function.
4. **Golden images.** `opence ui golden` draws every screen at 1920×1080 and
   3840×2160 and compares each by region, within a tolerance. `--update`
   stores new images. The images stay local (`~/.cache/opence/golden`), and
   the check skips itself when they are missing.
5. **Loop time.** `opence ui loop-time` measures save-to-shot and fails above
   2 s.

## Non-functional requirements

- The loop takes under 2 s on the M1 Max.
- No unsafe code, `unwrap` or `expect`.
- Layout costs at most 0.1 ms a frame at 4K, and the p99 bar of 8.33 ms
  holds.
- Every control is also an `opence` command.

## Acceptance criteria

- A size or string change shows in under 2 s, measured.
- A layout that fails to build leaves the last good one running.
- Golden `--check` passes, and fails on a 4-pixel move.
- The existing halo_ui tests pass against the wasm layout.
- The workspace's tests, clippy, the census, `nix build .#engine` and
  `nix flake check` all pass.

## Out of scope

- Live reload of Swift and Metal, or of `game.wasm`.
- Making `nix flake check` faster.
- Changes to how the menus look.

## Baseline

Today's loop, timed on the M1 Max on 1 November 2026 (one run, after
touching `engine/crates/halo_ui/src/screens.rs`; the shot was checked to
be the Video Settings screen):

| Step | Seconds |
| --- | ---: |
| Cargo build (`engine_build.py --rust-only`, incremental release) | 7.4 |
| Swift link, bundle and signing (`engine_build.py --rust`) | 10.0 |
| Headless launch, until the focus names a screen | 0.9 |
| Waiting for the ring scene to settle before keys | 6.0 |
| Socket walk to Video Settings (Settings, then Video) | 10.2 |
| The shot (`frame` request, until the PNG is written) | 0.1 |
| **From save to shot** | **34.6** |

`nix flake check`, run before a commit, is not in the loop above. The
walk and the settling make up 16 s of it; opening a screen by name takes
both away. The two builds take 17 s; the wasm layout and its live swap
(Phases 3 and 4) take both away.
