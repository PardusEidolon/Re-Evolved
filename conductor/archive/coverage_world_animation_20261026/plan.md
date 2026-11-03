# Plan: Coverage 7 of 11, Animation and the world's queries

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: Maths
- [ ] Task: Extend the math reference trace to the 84 functions.
- [ ] Task: Port them.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Maths' (Protocol in workflow.md) (automated)

## Phase 2: Animation
- [ ] Task: Traces for frame info, root motion and IK.
- [ ] Task: Port them and the transparent-part fix-up.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Animation' (Protocol in workflow.md) (automated)

## Phase 3: World queries
- [ ] Task: Traces for cluster partitions, PVS and PAS, scenario locations.
- [ ] Task: Port them, and structures and scenario.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: World queries' (Protocol in workflow.md) (automated)

## Phase 4: Models and physics_update
- [ ] Task: Port models.
- [ ] Task: Finish `physics_update`.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Models and physics_update' (Protocol in workflow.md) (automated)

## Phase 5: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: The report' (Protocol in workflow.md) (automated)
