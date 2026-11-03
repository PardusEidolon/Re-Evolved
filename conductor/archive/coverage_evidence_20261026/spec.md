# Spec: Coverage 2 of 11, Evidence for the owed

## Overview
675 functions are classed Ported or Matched but have no check against the original. The most-called are the datum allocators (`data_next_index`, `datum_new`, `object_new`, `object_delete`), `real_random_range`, `hs_evaluate` and `hs_return`, `sound_class_get` and `game_tick`. Many of the rest, and most of the AI, need a fake world richer than the reference harness has today: objects, tag instances and a map. This track builds that world first, then works down the owed list by caller count until it is empty.

This is track 2 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **A richer fake world.** The `tools/engine_*_reference.py` harness gains a shared fake world with an object table, tag instances and a small map (a BSP, clusters and a scenario), built from synthetic data only.
2. **Traces by caller count.** The owed list is worked in order of callers, beginning with the datum allocators, `real_random_range`, `hs_evaluate`, `hs_return`, `sound_class_get` and `game_tick`.
3. **Divergences found are fixed** in the engine, each with a regression test, or recorded with the trace that shows them and handed to the owning system's track.
4. **Measurements** stand in for traces only where the census allows it (Matched rendering-adjacent work), and are systematic sweeps against the stock game.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- The owed count is zero in every system.
- Every new trace has a `--check` that fails when it is stale.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- Functions that are Missing; the system tracks port those with their evidence.
