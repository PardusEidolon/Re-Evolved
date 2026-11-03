# Spec: Coverage 7 of 11, Animation and the world's queries

## Overview
Two groups of shared machinery remain: animation's frame info, root motion, x offsets, root velocity, `inverse_kinematics_adjust_matrices` and the transparent-part link fix-up; and the world queries that AI, sound and objects call (cluster partitions, the PVS and PAS tests, scenario locations, the `real_math` geometry queries, `accelerate_to_*`).

This is track 7 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **Animation.** Frame info, root motion, x offsets, root velocity and inverse kinematics are ported and traced; the transparent-part link fix-up is matched.
2. **Models** (16 Missing) are ported.
3. **World queries.** Cluster partitions, the PVS and PAS tests and scenario locations are ported and traced on the fake world's map.
4. **Maths.** The 84 Missing math functions are ported with `libm`, no fused multiply-add, and checked by the math reference trace.
5. **Structures** (28) and **scenario** (21) are ported or classed.
6. `physics_update` is finished.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `math`, `models`, `structures`, `scenario`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- Traces for animation, the world queries and the maths match.
- devgulch's animation station shows no foot slide or pose differences in a measured sweep.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- Drawing the models, which is the renderer's.
