# Spec: The lighting, extended from the original's own

## Overview
With the stock rendering matched (`native_port_20261028`, A4) and every
map audited against it (`map_audit_20261028`), the lighting is extended,
built on what the original already has: its lightmaps, its shader tags,
its placed lights and its fog. It merges two archived tracks,
`placed_lights_20261025` and `atmosphere_20261023`, and carries on the
lighting work begun in A5. Each extension can be switched back to stock,
and each is measured against stock.

## Functional Requirements

### Built on the original's data
1. The extensions read the original's lightmaps, shader tags, placed
   lights and fog from the map, through the bounds-checked readers.
2. Each extension is a switch in the settings and the dev panel, with stock
   as one of its values, so any frame can be compared with stock.

### The extensions
3. **Placed lights:** the original's dynamic and placed lights cast
   shadows and light the world and the models with their colour, falloff
   and shape.
4. **Bounce and probes:** baked bounce light and light probes made from
   the lightmaps and the lights, at import or behind the loading screen,
   cached by the map's hash.
5. **The sky and atmosphere:** the fog and sky from the map's tags drawn
   with scattering, matched to the original's colours.
6. **Ray traced shadows and reflections** where the M1 Max's budget
   allows, each with a rasterized fallback chosen by the quality setting,
   never silently.

### Measurement
7. Each extension is measured against stock on every map: lightmap seams,
   shadow edges, brightness and colour per region, in systematic sweeps.
   A result that looks wrong to the user is a failing case even when the
   numbers pass.
8. p99 ≤ 8.33 ms at 4K holds with every extension on at ultra.

## Non-Functional Requirements
- Test first; golden frames, re-approved by the user, check the drawing.
- Bakes and probes never run during play.
- Baked data stays on the user's machine.

## Acceptance Criteria
- Each extension has its switch to stock, its measurements against stock
  on every map, and its golden frames approved by the user.
- p99 ≤ 8.33 ms at 4K ultra on every map.
- A report from the data sets up what follows.

## Out of Scope
- Lights or atmosphere the original's maps don't define.
- The living world, kev-4b and the netcode rework, which come later.
