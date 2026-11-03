# Plan: Coverage 8 of 11, The interface and HUD

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: HUD
- [ ] Task: Traces of HUD state.
- [ ] Task: Port `hud_draw_screen`'s logic and the 14 natives.
- [ ] Task: The user's feedback, 27 October: the HUD's elements still need fixing. The frame bloodgulch_hud_1920x1080 drawn then shows the grenade count reading 4 where the first frame had 0, and the first person's rifle and hand moved. Measure every element against the original's HUD (placement, scale, counts, colours) and fix each one that differs.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: HUD' (Protocol in workflow.md) (automated)

## Phase 2: Interface
- [ ] Task: Traces of widget logic.
- [ ] Task: Port `interface_draw_screen` and the widgets.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Interface' (Protocol in workflow.md) (automated)

## Phase 3: Text, input and dialogs
- [ ] Task: Port text and unicode.
- [ ] Task: Match input and dialogs.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Text, input and dialogs' (Protocol in workflow.md) (automated)

## Phase 4: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: The report' (Protocol in workflow.md) (automated)
