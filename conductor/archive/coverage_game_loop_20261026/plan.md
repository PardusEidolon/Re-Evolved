# Plan: Coverage 6 of 11, The game loop, flow and saves

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The loop
- [ ] Task: Traces for `main_loop_iteration` and `game_tick` order.
- [ ] Task: Port or match main.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: The loop' (Protocol in workflow.md) (automated)

## Phase 2: Flow
- [ ] Task: Traces for `main_won_map_private` and `ui_set_next_level`.
- [ ] Task: Port the level flow.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Flow' (Protocol in workflow.md) (automated)

## Phase 3: The game engine and players
- [ ] Task: Traces and ports for `game_engine_update` and the players system.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: The game engine and players' (Protocol in workflow.md) (automated)

## Phase 4: Saved games
- [ ] Task: Tests for checkpoint, revert and round-trip.
- [ ] Task: Port the saved-game rules.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Saved games' (Protocol in workflow.md) (automated)

## Phase 5: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: The report' (Protocol in workflow.md) (automated)
