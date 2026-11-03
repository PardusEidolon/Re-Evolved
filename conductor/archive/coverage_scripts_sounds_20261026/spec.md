# Spec: Coverage 10 of 11, Scripts and sounds

## Overview
The script system and sound are the last two systems because they call into all the others. The census lists 692 Missing hs functions, 54 allowed script functions and 440 globals unwired, 76 debug script functions, and sound requests that do not carry their object, so the 26 sound classes with a per-object limit cannot apply it and dialogue cannot drive the mouth aperture.

This is track 10 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **The script system.** The 692 Missing hs functions are ported: the evaluator, threads, the type system and the library.
2. **Natives and globals.** The 54 allowed functions and the 440 globals are wired; the 76 debug functions go to the dev shell or are classed Replaced as the editor and tools.
3. **Sounds carry their object**, so the 26 classes with a per-object limit apply it, and dialogue drives the mouth aperture.
4. **Sound** (93 Missing) is ported or classed; Bungie's sounds are read from the user's maps, never shipped.
5. **Play sweeps.** Each campaign map is played through headless with its scripts, and every script that runs is reached and traced.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `hs`, `sound`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- Script and sound traces match.
- The play sweep of each campaign map finishes with no unwired script function or global.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- Sound mixing and the renderer, which are rewritten.
