# Plan: Coverage 9 of 11, Cameras and cutscenes

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The director and observer
- [ ] Task: Traces for the director and observer.
- [ ] Task: Port them and each camera mode.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: The director and observer' (Protocol in workflow.md) (automated)

## Phase 2: Recorded animations
- [ ] Task: Traces for recorded-animation playback.
- [ ] Task: Port it.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Recorded animations' (Protocol in workflow.md) (automated)

## Phase 3: Cutscenes
- [ ] Task: Port the letterbox, the title and the cutscene natives.
- [ ] Task: Measure the opening cutscenes against the stock game.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Cutscenes' (Protocol in workflow.md) (automated)

## Phase 4: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: The report' (Protocol in workflow.md) (automated)
