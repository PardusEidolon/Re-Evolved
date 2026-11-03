# Spec: the cleanup after the change of direction (Track B)

## Overview

On 27 October 2026 the project changed direction: `source/` is compiled
unchanged as wasm32 and run in Wasmtime in the native macOS app, under one
typed Rust import list, drawn by Swift and Metal. This track clears the
ground for that work before Track A starts.

## Requirements

- **B1.** A retrospective of the fork in the road
  (`docs/retrospective_fork_in_the_road.md`): what the native Rust engine
  and the VM each delivered, what each cost, and why the wasm32 direction
  was chosen.
- **B2.** Every track from before the change moved to `conductor/archive/`,
  with a README saying what became of each.
- **B3.** The launcher removed: the app starts the game directly, on its
  main menu or the map `--map` names. The Developer tab's controls stay
  reachable as shell commands and socket calls.
- **B4.** `CLAUDE.md`, `AGENTS.md`, `conductor/product.md` and
  `conductor/tech-stack.md` rewritten for the new direction.
- **B5.** Specs and plans for the four tracks: `native_port_20261028`,
  `halo_ui_20261028`, `map_audit_20261028` and
  `lighting_original_20261028`.

## Acceptance

- The whole Rust workspace's tests, the Swift tests and the census
  `--check` pass after B3.
- `conductor/tracks.md` lists only the four new tracks and this one.

## Out of scope

- Any change to the game's behaviour, and removing the Rust simulation
  (Track A's last phase does that).
