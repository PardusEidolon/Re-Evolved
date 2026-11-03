# Spec: Coverage 3 of 11, Data fidelity

## Overview
The census compared the importer with `tag_layout.tsv` and the original's fix-ups. It found strides that differ (the `game_globals` difficulty block is 0x294 in the importer and 0x284 in the original), four fix-ups that differ, `vehicle_definition` and the shader subtypes with no layout from the decompilation, 516 fields the importer keeps but nothing reads, and 151 fields with no trace of being read. Data comes first among the system tracks because every later port reads these fields.

This is track 3 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **Strides.** Every block stride the importer uses equals `tag_layout.tsv`, checked by a test over the whole table; the difficulty block is fixed.
2. **Fix-ups.** The four differing load-time fix-ups match the original, each checked by a reference trace over a synthetic tag.
3. **A second source** for `vehicle_definition` and the shader subtypes (format documentation such as OpenSauce or Reclaimer, read only, never copied), checked against the user's maps where present.
4. **Unused and untraced fields.** Each of the 516 kept-but-unused fields is read by the system that uses it in the original, or recorded as read by a Replaced function; each of the 151 untraced fields is traced.
5. **Cache and bitmaps.** The cache (37 Missing) and bitmaps (3) functions are ported, matched or classed.
6. All readers stay bounds-checked and fuzzed; new readers get fuzz targets.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `cache`, `bitmaps`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- The census's tag-field, fix-up and stride sections report no differences.
- Every tag field is read, traced, or classed with a reason.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- Gameplay that uses the newly read fields; each system's track does that.
