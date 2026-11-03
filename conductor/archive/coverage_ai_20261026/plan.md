# Plan: Coverage 4 of 11, AI core

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: The actor loop
- [ ] Task: Traces for `actors_update`, `actor_update` and `ai_update` on the fake world.
- [ ] Task: Port the loop and the dormancy rules.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: The actor loop' (Protocol in workflow.md) (automated)

## Phase 2: Perception and knowledge
- [ ] Task: Traces for perception, props and knowledge.
- [ ] Task: Finish `actor_perception_update` and port props and knowledge.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Perception and knowledge' (Protocol in workflow.md) (automated)

## Phase 3: Decisions
- [ ] Task: Traces for each `*_decide_action` family.
- [ ] Task: Port them, one commit per family.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Decisions' (Protocol in workflow.md) (automated)

## Phase 4: Actions and positions
- [ ] Task: Traces for each `actor_action_handle_*` and for firing positions and placement.
- [ ] Task: Port them and finish `actor_select_firing_position` and `actor_place`.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Actions and positions' (Protocol in workflow.md) (automated)

## Phase 5: Emotions, swarms and the rest
- [ ] Task: Port emotions and swarms.
- [ ] Task: Port or class the remaining ai functions until the system's counts are zero.
- [ ] Task: Close fidelity Phase 11's divergences.
- [ ] Task: Conductor - User Manual Verification 'Phase 5: Emotions, swarms and the rest' (Protocol in workflow.md) (automated)

## Phase 6: The report
- [ ] Task: Run `census` and record the counts before and after this track, per system and per class.
- [ ] Task: Write the data-backed report and lower the ratchet to the new counts.
- [ ] Task: Send the user the report; it sets up the next track.
- [ ] Task: Conductor - User Manual Verification 'Phase 6: The report' (Protocol in workflow.md) (automated)
