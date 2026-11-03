# Spec: Coverage 11 of 11, The 100% gate

## Overview
When the system tracks are done, this track closes coverage. It sets the ratchet to zero everywhere and makes `census --check` demand it, runs the systematic measurement sweeps a passing check cannot replace, and writes the report that tells the user whether fidelity is done and the feature tracks can start.

This is track 11 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **Zero everywhere.** `census --check` fails on any Missing, Partial or owed function, any unread or untraced tag field, any differing fix-up or stride, and any unwired allowed script function or global.
2. **Sweeps.** Systematic measurement sweeps against the stock game: lightmap seams, shadow edges, timings, HUD layout, cutscene camera paths and AI encounters on each campaign map.
3. **What remains.** Anything the sweeps find becomes a fix track before the gate closes.
4. **The report.** A data-backed report: the census before and after the coverage tracks, every class's counts, the sweeps' results, and what the feature tracks can rely on.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` passes with every count at zero.
- The sweeps' results are in the report, and the user has the report.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- New features; they start only once the user is satisfied with fidelity.
