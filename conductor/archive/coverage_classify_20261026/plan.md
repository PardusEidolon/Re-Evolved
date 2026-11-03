# Plan: Coverage 1 of 11, Classify and cite

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The ratchet [checkpoint: 8398896e]
- [x] Task: Write the failing test: a census whose Missing count for one system rises fails `--check`. 530eef73
- [x] Task: Add `engine/census/ratchet.tsv` and the `--ratchet` command that writes it from the current census. 530eef73
- [x] Task: Wire the ratchet into `census --check` and the flake's census test. 530eef73
- [x] Task: Conductor - User Manual Verification 'Phase 1: The ratchet' (Protocol in workflow.md) (automated)

## Phase 2: Citations and Native [checkpoint: 08019fa9]
- [x] Task: Write tests that the named equivalents (`unit_update`, `main_loop`, `weapon_ready` and the others the report lists) are cited. 1248541f
- [x] Task: Add the citations in the engine crates and the Swift app, each with its class and evidence or its owed entry. 1248541f
- [x] Task: Class the 226 small helpers as Native with their primitives, checked by the math reference trace where they are numeric. cf5646b0
- [x] Task: Conductor - User Manual Verification 'Phase 2: Citations and Native' (Protocol in workflow.md) (automated)

## Phase 3: Reasons [checkpoint: 93b45a4c]
- [x] Task: Add the "library" reason to the census's fixed list, with a test that it needs a stand-in and a player-visible note. 6d6661a1
- [x] Task: Class the collision BSP functions under it, naming the Jolt code. 9f6f01b7
- [x] Task: Class the debug and console functions as "the editor and tools". 8e708c7d
- [x] Task: Conductor - User Manual Verification 'Phase 3: Reasons' (Protocol in workflow.md) (automated)

## Phase 4: Names [checkpoint: c6e73531]
- [x] Task: Write a test that no census entry is known only by its address. f2b2bc0c
- [x] Task: Name the 1,168 address-only functions by system, from callers, strings and context, in a names table the census reads. f2b2bc0c
- [x] Task: Review the unreached Missing functions and class or annotate each. 1f03014e
- [x] Task: Conductor - User Manual Verification 'Phase 4: Names' (Protocol in workflow.md) (automated)

## Phase 5: The report [checkpoint: 28f04103]
- [x] Task: Run `census` and record the counts before and after this track, per system and per class. ac77e257
- [x] Task: Write the data-backed report and lower the ratchet to the new counts. ac77e257
- [x] Task: Send the user the report; it sets up the next track. (with the final report of all eleven tracks)
- [x] Task: Conductor - User Manual Verification 'Phase 5: The report' (Protocol in workflow.md) (automated)
