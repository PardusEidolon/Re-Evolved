# Spec: Coverage 5 of 11, Units, players' effects and objects

## Overview
Units, objects and their effects are what the player touches most after AI. The census lists `unit_update`, `unit_damage_aftermath` and `biped_update_moving` among the reached Missing functions, all of `player_effects.c` as Missing, and physics with 26 Partial functions. This track ports them from `source/` and traces them.

This is track 5 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **Units.** `unit_update`, `unit_damage_aftermath`, `biped_update_moving` and the rest of units (seats, weapons held, grenades, animation states) are ported.
2. **Player effects.** All of `player_effects.c`: screen shake, flashes, the camera effects that follow damage.
3. **Objects, items and devices.** The Missing object functions (attachments, regions, damage, the object lists), items (weapons, equipment, powerups) and devices (machines, controls, light fixtures) are ported.
4. **Effects.** Effect events, particle systems and contrails as the original runs them; drawing stays the renderer's.
5. **Physics.** The 26 Partial functions finished and the 98 Missing ported or classed under the library reason, keeping the player-visible behaviour.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `units`, `objects`, `items`, `devices`, `effects`, `physics`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- Reference traces of units, objects, items, devices, effects and physics over fake-world scenarios match.
- devgulch's vehicles, weapons and devices stations sweep without regressions.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- The renderer's drawing of effects, which is rewritten.
