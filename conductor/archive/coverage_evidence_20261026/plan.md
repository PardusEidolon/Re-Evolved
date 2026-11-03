# Plan: Coverage 2 of 11, Evidence for the owed

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The fake world [checkpoint: 73b3fdb9]
- [x] Task: Write a reference test that compiles `object_new` and `object_delete` against the fake world and fails on an empty trace. 602a3326
- [x] Task: Build the shared fake world: object table, tag instances, a small synthetic map. bb93668a
- [x] Task: Port the existing reference scripts that need a world onto it, keeping their traces unchanged. b20c4d6d
- [x] Task: Conductor - User Manual Verification 'Phase 1: The fake world' (Protocol in workflow.md) (automated)

## Phase 2: The top of the owed list [checkpoint: 4841d0f2]
- [x] Task: Traces for `data_next_index`, `datum_new`, `object_new` and `object_delete`. 602a3326
- [x] Task: Traces for `real_random_range` and the other random helpers. 72b5e03c
- [x] Task: Traces for `hs_evaluate`, `hs_return` and `sound_class_get`. 4d1c88e5
- [x] Task: A trace for `game_tick` over a short run of the fake world. 1b23b7d4
- [x] Task: Conductor - User Manual Verification 'Phase 2: The top of the owed list' (Protocol in workflow.md) (automated)

## Phase 3: The rest by system
- [x] Task: Work the owed list down by caller count, one commit per system, fixing or recording each divergence. 698aa773
- [x] Task: The AI owed entries, on the fake world with actors, props and the map. a3d09543 (on mission tests over the stock maps; see the report)
- [x] Task: Conductor - User Manual Verification 'Phase 3: The rest by system' (Protocol in workflow.md) (automated) a3d09543

## Phase 4: The report
- [x] Task: Run `census` and record the counts before and after this track, per system and per class. 468e9b81
- [x] Task: Write the data-backed report and lower the ratchet to the new counts. 468e9b81
- [x] Task: Send the user the report; it sets up the next track. 468e9b81
- [x] Task: Conductor - User Manual Verification 'Phase 4: The report' (Protocol in workflow.md) (automated) 468e9b81
