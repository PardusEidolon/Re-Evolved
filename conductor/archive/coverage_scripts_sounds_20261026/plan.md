# Plan: Coverage 10 of 11, Scripts and sounds

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The script system
- [ ] Task: Traces for the evaluator and threads.
- [ ] Task: Port the remaining hs functions.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: The script system' (Protocol in workflow.md) (automated)

## Phase 2: Natives and globals
- [ ] Task: Wire the 54 functions and 440 globals, one commit per group.
- [ ] Task: Route the 76 debug functions.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Natives and globals' (Protocol in workflow.md) (automated)

## Phase 3: Sound
- [ ] Task: Tests for per-object limits and the mouth aperture.
- [ ] Task: Carry the object on sound requests and port the rest of sound.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Sound' (Protocol in workflow.md) (automated)

## Phase 4: Play sweeps
- [ ] Task: A headless play sweep per campaign map, recording each unwired call.
- [ ] Task: Close what the sweeps find.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Play sweeps' (Protocol in workflow.md) (automated)

## Phase 5: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: The report' (Protocol in workflow.md) (automated)
