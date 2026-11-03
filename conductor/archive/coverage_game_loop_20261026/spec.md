# Spec: Coverage 6 of 11, The game loop, flow and saves

## Overview
The engine has its own main loop and a working Slayer, but the original's flow (`main_loop_iteration`, `main_won_map_private`, `ui_set_next_level`), `game_engine_update`, the players' per-tick update and co-op respawn, and saved games are Missing. Checkpoints, revert and the move from one campaign level to the next depend on them.

This is track 6 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **The loop.** `main_loop_iteration` and the rest of main are ported or matched, with the engine's loop cited.
2. **Flow.** Winning a map, choosing the next level and the transitions between campaign levels.
3. **The game engine.** `game_engine_update` and the game types' rules.
4. **Players.** `players_update_before_game`, `players_respawn_coop` and the rest of the players system.
5. **Saved games.** Checkpoints, revert and the saved-game files, written in a format of the engine's own and checked by round-trip and by the original's checkpoint rules.
6. **The shell.** The 3 shell functions.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `main`, `game`, `saved games`, `players`, `shell`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- A campaign run from the first level to the second goes through the original's flow, checked headless.
- Checkpoint and revert traces match the original's rules.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- Networking (Replaced).
