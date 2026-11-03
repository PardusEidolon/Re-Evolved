# Spec: Coverage 1 of 11, Classify and cite

## Overview
The census found 4,762 Missing functions (859,746 bytes), 94 Partial and 675 that owe evidence. Part of that count is work the engine has already done under another name (`unit_update`, `main_loop`, `weapon_ready`), helpers that a Rust primitive replaces, and the Jolt-backed collision BSP, for which the fixed list of Replaced reasons has no entry. A further 1,168 functions (300 KB) are known only by address. This track settles those before any porting starts, so the later tracks work from a true count, and adds the ratchet every later track checks against.

This is track 1 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **Cite the existing equivalents.** Every engine item that does an original function's job carries its citation, so `unit_update`, `main_loop`, `weapon_ready` and the rest move out of Missing into Matched or Partial (or into owed, when the evidence is not yet there).
2. **Native helpers.** The 226 small helpers the report lists (math, cseries, data arrays, lists, memory, strings) are classed Native, each naming its Rust primitive.
3. **A library reason.** The Replaced reasons gain "a library" (the collision BSP served by Jolt), with the Rust code that stands in and the player-visible behaviour it must keep (what collides, where shots stop).
4. **Debug and console functions** are classed Replaced as "the editor and tools", with the dev shell and the `opence` tool as the stand-in.
5. **Address-only functions.** Each of the 1,168 is named from its callers, its strings and the decompilation's context, given its system, and classed.
6. **Unreached functions.** Missing functions the played game never reaches are reviewed: each is either classed Replaced with a reason or kept Missing with a note that a later track owns it.
7. **The ratchet.** `engine/census/ratchet.tsv` holds the Missing, Partial and owed counts per system; `census --check` fails when any count rises, and `census --ratchet` lowers the recorded counts after a fix.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` passes and the ratchet is in place, with a test that a rise in any system's count fails it.
- No function is classed only by address.
- The report's per-system Missing counts before and after are recorded in the track's report.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- Porting any function; the later tracks do that.
