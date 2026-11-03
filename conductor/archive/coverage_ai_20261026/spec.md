# Spec: Coverage 4 of 11, AI core

## Overview
AI is the largest system in the census: 951 Missing functions, 277,818 bytes, of which 199 KB is reached in play. The engine has actors that patrol and fight, but `actor_update`, the `*_decide_action` and `actor_action_handle_*` families, and much of perception, firing positions, placement and `ai_update` (props, knowledge, emotions, swarms, dormancy) are Missing or Partial. Port, match, then extend: this is a straight port from `source/`, traced on the fake world from the evidence track.

This is track 4 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **The actor loop.** `actors_update`, `actor_update` and `ai_update` are ported in the original's order.
2. **Decisions and actions.** Every `*_decide_action` and `actor_action_handle_*` function is ported.
3. **Perception and positions.** `actor_perception_update`, `actor_select_firing_position` and `actor_place` are finished, with the cases the census lists as left out.
4. **Props, knowledge, emotions, swarms and dormancy** are ported.
5. **Fidelity Phase 11's divergences** are closed, each with its regression test.
6. Encounters, squads and the command lists the scripts drive are covered here or classed for the scripts track.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `ai`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- Reference traces of AI decisions over scripted fake-world scenarios match the original tick for tick.
- Play sweeps on devgulch's AI station and the campaign maps show no regressions.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- The decision-model commander layer (`ai_decisions_20261023`), which waits for fidelity.
