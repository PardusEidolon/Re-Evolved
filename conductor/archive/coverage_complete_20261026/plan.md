# Plan: Coverage 11 of 11, The 100% gate

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The gate
- [ ] Task: Write the failing test: any non-zero count fails `--check` with no ratchet override.
- [ ] Task: Remove the ratchet's allowances.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: The gate' (Protocol in workflow.md) (automated)

## Phase 2: Sweeps
- [ ] Task: Run the measurement sweeps against the stock game.
- [ ] Task: Turn anything they find into fix tracks, and run those.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Sweeps' (Protocol in workflow.md) (automated)

## Phase 3: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: The report' (Protocol in workflow.md) (automated)
