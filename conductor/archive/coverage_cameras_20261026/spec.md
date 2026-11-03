# Spec: Coverage 9 of 11, Cameras and cutscenes

## Overview
The camera and cutscene systems are what the campaign's scripts lean on most after AI. The director, the observer and its cameras, recorded animations and the letterbox and title are Missing.

This is track 9 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **The director and observer** are ported, with each camera mode (bored, static, orbiting, flying, the first-person and the dead camera).
2. **Recorded animations** play back from the scenario as the original does.
3. **The letterbox and title** are matched.
4. The cutscene script natives are wired.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `camera`, `cutscene`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- Camera traces over the fake world match.
- The campaign's opening cutscenes play with camera paths measured against the stock game.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- Bink videos (Replaced).
