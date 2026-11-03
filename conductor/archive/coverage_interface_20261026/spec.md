# Spec: Coverage 8 of 11, The interface and HUD

## Overview
The interface has 734 Missing functions. The engine draws the HUD and menus from the user's `ui.map` at native resolution, which stays; what is owed is the original's behaviour: what each HUD element shows and when, the widget logic, messages, text layout and the 14 HUD script natives.

This is track 8 of the eleven that take the coverage census (`coverage_census_20261026`) to 100%: zero Missing, zero Partial and zero owed functions, with tag fields, fix-ups, scripts and sounds complete. The tracks run in order: classify, evidence, data, AI, units, the game loop, animation and the world, the interface, cameras, scripts and sounds, and the gate.

## Functional Requirements
1. **HUD.** `hud_draw_screen` in full: every meter, the motion sensor, waypoints, messages and the nav points, matched in behaviour; drawn at native resolution.
2. **Interface.** `interface_draw_screen` and the widget logic, matched; the redesigned menus (`menu_redesign_20261024`) keep the original's behaviour under them.
3. **The 14 HUD natives** are wired to the scripts.
4. **Text and unicode** layout and the string lists are ported.
5. **Input** (5) is matched for keyboard and mouse; pad paths are Replaced.
6. **Dialogs** (1) is ported or classed.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`): the reference trace or measurement that proves a function is written before the port.
- Ported and Matched work follows the determinism rules: 60 Hz simulation, 30 Hz scripts, the fixed `SimSet` order, `libm` maths with no fused multiply-add, and fixed iteration order.
- The workspace lints hold (`unsafe_code` denied, no `unwrap` or `expect`, `missing_docs`), and `nix flake check` stays green after every commit.
- Tests that need the user's data skip themselves when it is missing.
- Nothing from the user's game data enters the repository; traces, golden frames and sweeps derived from it stay on the user's machine.
- A function leaves Missing or Partial only with the evidence its class asks for (the census spec, `coverage_census_20261026`).

## Acceptance Criteria
- `census --check` reports zero Missing, zero Partial and zero owed functions in `interface`, `text`, `input`, `dialogs`, and `engine/census/ratchet.tsv` records those zeros so they cannot rise again.
- Traces of HUD state over scripted fake-world scenarios match.
- Golden comparisons of the HUD's layout against the stock game, measured at the original's resolution and scaled.
- A data-backed report, with the census counts before and after, sets up the next track.

## Out of Scope
- The visual redesign of the menus, which is its own track.
