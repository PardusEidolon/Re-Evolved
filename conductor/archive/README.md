# Archived tracks

On 27 October 2026 the direction changed: `source/` is compiled unchanged
as wasm32 and run in Wasmtime inside the macOS app, and Rust extends it
through plugins (`docs/retrospective_fork_in_the_road.md`). Every track
before then was moved here. This says what became of each.

## Carried on by a new track

| Archived track | Becomes |
| --- | --- |
| `coverage_audit_20261021` (the map and cutscene audit) | Kept, with a new spec, after the Halo UI rewrite |
| `menu_redesign_20261024` (menus and loading screen) | The Halo UI rewrite (`halo_ui_20261028`), a plugin drawn at 4K from `ui.map`. The launcher part is dropped. |
| `halo_ui_20261021` | The same: its widgets from `ui.map` are the rewrite's start |
| `placed_lights_20261025`, `atmosphere_20261023` | Merged into the lighting extended from the original |
| `native_engine_20261021`, `macos_window_metal_20261018`, `lighting_shaders_20261021` | The native port (`native_port_20261028`) keeps their renderer where it extends the original's |

## Archived for later

| Archived track | When it returns |
| --- | --- |
| `living_world_20261023` (weather, water, a world that reacts) | A feature, after fidelity, as the user chooses |
| `ai_decisions_20261023` (kev-4b) | After fidelity, as a plugin on the AI hook |

The Rust `net` crate (`engine/crates/net`) is archived for reference with
them: the original's own netcode is compiled in as it is, and a later
track reworks netcode from scratch. The crate stays in the workspace until
the native port's removals phase, because the ffi crate still calls it.

## Done, or replaced by the original's code

| Archived track | Why |
| --- | --- |
| `coverage_census_20261026`, `coverage_classify_20261026`, `coverage_evidence_20261026`, `coverage_data_20261026` | Done. The census and the traces are kept and become checks on the wasm build. |
| `coverage_units_20261026`, `coverage_game_loop_20261026`, `coverage_world_animation_20261026`, `coverage_interface_20261026`, `coverage_cameras_20261026`, `coverage_scripts_sounds_20261026`, `coverage_ai_20261026`, `coverage_complete_20261026` | Coverage 4 to 11: their simulation work comes with the C. The user's play feedback in them (the grenade count, the first-person pose, the weapons station's particles) moves to the native port and the Halo UI rewrite. |
| `animation_20261021`, `physics_20261021`, `particles_20261021`, `sound_20261021`, `ragdolls_jolt_20261020`, `dev_tooling_20261021` | Done. The dev tools are kept; the simulation ports are replaced by the original's code in the native port's removals phase. |
| `fidelity_devtools_20261024`, `macos_headless_host_20261018`, `performance_20261023`, `raytracing_metal_20261020`, `rust_tools_20261024` | Archived before 27 October |
