# Plan: Coverage 5 of 11, Units, players' effects and objects

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: Units
- [ ] Task: Traces for `unit_update`, `unit_damage_aftermath` and `biped_update_moving`.
- [~] Task: Port the units system until its counts are zero.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Units' (Protocol in workflow.md) (automated)

## Phase 2: Player effects
- [ ] Task: Traces for `player_effects.c`.
- [ ] Task: Port it.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Player effects' (Protocol in workflow.md) (automated)

## Phase 3: Objects, items and devices
- [ ] Task: Traces and ports for objects, then items, then devices.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Objects, items and devices' (Protocol in workflow.md) (automated)

## Phase 4: Effects
- [ ] Task: Traces and ports for effects and particles.
- [ ] Task: The user's feedback, 27 October: the weapons station's particles are still incomplete. Its frame's plume changed shape after 592bb028; trace each weapon's effects (firing, impacts, contrails) against the original and draw every particle the game makes.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Effects' (Protocol in workflow.md) (automated)

## Phase 5: Physics
- [ ] Task: Finish the 26 Partial physics functions.
- [ ] Task: Port or class the 98 Missing.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Physics' (Protocol in workflow.md) (automated)

## Phase 6: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 6: The report' (Protocol in workflow.md) (automated)
