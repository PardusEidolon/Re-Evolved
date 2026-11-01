# Plan: A fast loop for the Halo UI

## Phase 1: Measure the loop and open screens directly
- [ ] Task: Time today's loop, step by step: the cargo build, the Swift link and signing, the launch, the walk and the shot. Write the times into the spec as the baseline.
- [ ] Task: Write tests for opening a screen by name. Then add an `open_screen` socket call and `opence ui open <screen>`, both going through the game's own screen-opening function. A name it doesn't know is refused with the names it does know.
- [ ] Task: Conductor - User Manual Verification 'Phase 1' (Protocol in workflow.md) (automated)

## Phase 2: Split the layout from the plugin
- [ ] Task: Write tests for a `Snapshot` type that holds what layout reads from the widget tree (screens, items, focus, strings, pictures), and for its binary encoding, round-tripped and refusing truncated input.
- [ ] Task: Move screens.rs, scene.rs, loading's layout and advances.rs into a new crate, `opence_halo_ui_layout`. It has no runtime dependency, and its entry point is `layout(&Snapshot) -> Scene`. The native plugin calls it directly, and halo_ui's 44 tests pass unchanged.
- [ ] Task: Write tests for the Scene's wire encoding: round-tripped, every count and offset bounds-checked, and a broken scene refused rather than drawn. Add a cargo-fuzz target for its reader.
- [ ] Task: Conductor - User Manual Verification 'Phase 2' (Protocol in workflow.md) (automated)

## Phase 3: Run the layout as wasm
- [ ] Task: Build the layout crate for `wasm32-unknown-unknown` with no imports, exporting `alloc`, `layout` and `scene_ptr`. Add it to engine_build.py and the Nix build, with the module in the bundle.
- [ ] Task: Write tests for a host that loads the module in the app's Wasmtime, writes the snapshot into its memory, calls `layout` with a fuel limit, and reads the scene back through the bounds-checked reader. Its output must match the native layout's on every screen.
- [ ] Task: The plugin uses the wasm layout. Its cost per `ui_update` is timed and must stay at or below 0.1 ms. A failing module leaves the last good scene and logs why. The benchmark's p99 at 4K must stay at or below 8.33 ms.
- [ ] Task: Conductor - User Manual Verification 'Phase 3' (Protocol in workflow.md) (automated)

## Phase 4: Swap the layout live
- [ ] Task: Write tests for swapping in a new module at the next `ui_update` (`swap_layout` on the socket), and for a bad module being refused while the last good one stays.
- [ ] Task: Add `--ui-dev`, headless only. It watches the layout crate, runs an incremental cargo build of the module in a cached profile tuned for build speed, swaps the result in, and writes a shot of the open screen. A failed build is logged and changes nothing.
- [ ] Task: Add `opence ui loop-time`, which touches a layout constant, times the save-to-shot, and puts it back. It fails above 2 s. If needed, tune the build profile, the module's compile and the shot path until it passes on the M1 Max.
- [ ] Task: Conductor - User Manual Verification 'Phase 4' (Protocol in workflow.md) (automated)

## Phase 5: Golden images of every screen
- [ ] Task: Write tests for comparing images by region (from scene rects, with a set tolerance), and for a report that names the screen and the region that changed.
- [ ] Task: Add `opence ui golden [--update|--check]`. It opens every menu screen by name, headless at 1920×1080 and 3840×2160, shoots it, and compares the shot with `~/.cache/opence/golden`. The check skips itself when the images are missing, and it fails on a deliberate 4-pixel move.
- [ ] Task: Write the loop's steps into the `opence` tool's help and a short guide (`docs/ui_fast_loop.md`): `--ui-dev`, `ui open`, `ui golden` and `ui loop-time`.
- [ ] Task: Conductor - User Manual Verification 'Phase 5' (Protocol in workflow.md) (automated) (workspace tests, clippy, census, menu_functions_check, `nix build .#engine`, `nix flake check`; loop-time under 2 s; golden --check on every screen)
