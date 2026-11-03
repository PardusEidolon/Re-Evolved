# The coverage census

Every function of the original (`source/`), by system, with its class and
the evidence for it. `opence census --write` writes this file from
`engine/census/census.toml` and the reference traces; do not edit it.
docs/engine.md's "The coverage census" says what each class needs.

| System | Ported | Compiled | Matched | Native | Partial | Replaced | Missing | Owed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `ai` | 45 (1936 B) | 914 (294330 B) | | | | | | 0 |
| `bink` | | 24 (3968 B) | | | | | | 0 |
| `bitmaps` | 1 (224 B) | 137 (61392 B) | | | | 72 (2512 B) | | 0 |
| `bungie_net` | | 105 (18992 B) | | | | | | 0 |
| `cache` | | 187 (24784 B) | | | | | | 0 |
| `camera` | 5 (528 B) | 116 (36016 B) | | | | | | 0 |
| `cseries` | 1 (16 B) | 139 (21824 B) | | | | | | 0 |
| `cutscene` | | 65 (10528 B) | | | | | | 0 |
| `devices` | 21 (2176 B) | 39 (5008 B) | | | | | | 0 |
| `dialogs` | | 5 (1328 B) | | | | | | 0 |
| `editor` | | 9 (0 B) | | | | | | 0 |
| `effects` | 13 (2864 B) | 177 (53744 B) | | | | | | 0 |
| `game` | 81 (14432 B) | 635 (89648 B) | | | | | | 0 |
| `hs` | 15 (944 B) | 624 (66048 B) | | | | | | 0 |
| `input` | | 47 (7216 B) | | | | | | 0 |
| `interface` | 8 (1904 B) | 671 (154776 B) | | | | | | 0 |
| `items` | 12 (4256 B) | 125 (31792 B) | | | | | | 0 |
| `main` | | 126 (15840 B) | | | | | | 0 |
| `math` | 65 (8016 B) | 144 (15152 B) | | | | | | 0 |
| `memory` | 18 (2080 B) | 202 (33216 B) | | | | 12 (0 B) | | 0 |
| `models` | 21 (10000 B) | 27 (7568 B) | | | | | | 0 |
| `networking` | | 288 (49824 B) | | | | | | 0 |
| `objects` | 10 (5040 B) | 337 (46544 B) | | | | | | 0 |
| `physics` | 44 (12080 B) | 101 (35152 B) | | | | | | 0 |
| `rasterizer` | 1 (144 B) | 779 (185056 B) | | | | | | 0 |
| `render` | 4 (832 B) | 115 (37520 B) | | | | | | 0 |
| `saved games` | | 137 (24992 B) | | | | | | 0 |
| `scenario` | 12 (4048 B) | 45 (5536 B) | | | | | | 0 |
| `shaders` | | 21 (2624 B) | | | | | | 0 |
| `shell` | | 13 (528 B) | | | | | | 0 |
| `sound` | 14 (2416 B) | 158 (24544 B) | | | | | | 0 |
| `structures` | 2 (0 B) | 124 (31712 B) | | | | | | 0 |
| `tag_files` | | 48 (4288 B) | | | | | | 0 |
| `text` | | 112 (16912 B) | | | | | | 0 |
| `tool` | | 27 (8752 B) | | | | | | 0 |
| `units` | 165 (26944 B) | 164 (80576 B) | | | | | | 0 |
| **all** | 558 (100880 B) | 6987 (1507730 B) | | | | 84 (2512 B) | | 0 |

## `ai`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `action_alert_setup` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `action_alert_begin` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `action_alert_control` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `action_alert_flush_position_indices` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `action_alert_flush_structure_indices` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `vector_from_points3d` | ai/action_alert.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude_squared3d` | ai/action_alert.c | Ported |  | `tools/engine_math_reference.py` |
| `distance_squared3d` | ai/action_alert.c | Ported |  | `tools/engine_math_reference.py` |
| `real_random_range` | ai/action_alert.c | Ported |  | `tools/engine_random_reference.py` |
| `action_alert_update` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `action_alert_next_position` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `action_alert_perform` | ai/action_alert.c | Compiled |  | compiled into game.wasm |
| `action_avoid_setup` | ai/action_avoid.c | Compiled |  | compiled into game.wasm |
| `action_avoid_begin` | ai/action_avoid.c | Compiled |  | compiled into game.wasm |
| `action_avoid_end` | ai/action_avoid.c | Compiled |  | compiled into game.wasm |
| `action_avoid_perform` | ai/action_avoid.c | Compiled |  | compiled into game.wasm |
| `action_avoid_update` | ai/action_avoid.c | Compiled |  | compiled into game.wasm |
| `action_avoid_control` | ai/action_avoid.c | Compiled |  | compiled into game.wasm |
| `action_charge_begin` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_charge_find_target_range` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_charge_update` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_charge_control` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_charge_is_leaping` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `square_root` | ai/action_charge.c | Ported |  | `tools/engine_animation_reference.py` |
| `scale_vector2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude_squared2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `normalize2d` | ai/action_charge.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `dot_product2d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `point_from_line3d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `scale_vector3d` | ai/action_charge.c | Ported |  | `tools/engine_math_reference.py` |
| `magnitude3d` | ai/action_charge.c | Ported |  | `tools/engine_item_reference.py` |
| `normalize3d` | ai/action_charge.c | Ported |  | `tools/engine_item_reference.py` |
| `dot_product3d` | ai/action_charge.c | Ported |  | `tools/engine_handle_action_reference.py` |
| `subtract_vectors3d` | ai/action_charge.c | Ported |  | `tools/engine_physics_math_reference.py` |
| `real_random` | ai/action_charge.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `collision_test_line` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_charge_perform` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_charge_valid_melee_destination` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_charge_setup` | ai/action_charge.c | Compiled |  | compiled into game.wasm |
| `action_converse_setup` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `action_converse_begin` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `action_converse_perform` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `action_converse_update` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `action_converse_control` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `action_converse_replace_prop` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `actor_conversation_control` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `actor_conversation_end` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `action_converse_end` | ai/action_converse.c | Compiled |  | compiled into game.wasm |
| `action_fight_setup` | ai/action_fight.c | Compiled |  | compiled into game.wasm |
| `action_fight_begin` | ai/action_fight.c | Compiled |  | compiled into game.wasm |
| `action_fight_end` | ai/action_fight.c | Compiled |  | compiled into game.wasm |
| `action_fight_update` | ai/action_fight.c | Compiled |  | compiled into game.wasm |
| `action_fight_control` | ai/action_fight.c | Compiled |  | compiled into game.wasm |
| `action_fight_perform` | ai/action_fight.c | Compiled |  | compiled into game.wasm |
| `action_flee_end` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_flush_position_indices` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_modify_color` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_find_flee_position` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_current_position_exposed` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_replace_prop` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_blind_panic` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_setup` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_begin` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_update` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_control` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_at_flee_position` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_flee_perform` | ai/action_flee.c | Compiled |  | compiled into game.wasm |
| `action_guard_setup_current_position` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_setup_find_position` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_setup_postcombat` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_begin` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_end` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_speak_post_combat` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_update` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_flush_position_indices` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_flush_structure_indices` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_modify_color` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_replace_prop` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_setup_from_combat_transition` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_setup_from_fleeing` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_perform` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `action_guard_control` | ai/action_guard.c | Compiled |  | compiled into game.wasm |
| `vehicle_possibility_qsort` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_command_end` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individual_setup` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individual_flush_command_indices` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individual_begin` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individual_advance` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individual_end` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individuals_iterate` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_command_list_setup` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_flush_command_indices` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_flush_structure_indices` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_advance_command_list` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_begin` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_end` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_describe_command` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `vector_from_points2d` | ai/action_obey.c | Ported |  | `tools/engine_math_reference.py` |
| `cross_product3d` | ai/action_obey.c | Ported |  | `tools/engine_animation_reference.py` |
| `negate_vector3d` | ai/action_obey.c | Ported |  | `tools/engine_math_reference.py` |
| `random_range` | ai/action_obey.c | Ported |  | `tools/engine_random_reference.py` |
| `action_obey_directmovement_update_facing` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_command_begin` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_command_perform` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individual_perform` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_individual_update` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_update` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_perform` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_obey_control` | ai/action_obey.c | Compiled |  | compiled into game.wasm |
| `action_search_setup_target` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_setup_pursuit` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_setup_undirected` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_update` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_flush_position_indices` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_flush_structure_indices` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_control` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_begin` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_search_perform` | ai/action_search.c | Compiled |  | compiled into game.wasm |
| `action_sleep_control` | ai/action_sleep.c | Compiled |  | compiled into game.wasm |
| `action_uncover_setup_target` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_setup_pursuit` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_perform` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_control` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_flush_position_indices` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_flush_structure_indices` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_modify_color` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_begin` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `action_uncover_update` | ai/action_uncover.c | Compiled |  | compiled into game.wasm |
| `object_get_bounding_sphere` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_begin` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_update` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_flush_structure_indices` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_control` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `point_from_line2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `distance_squared2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `distance2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `negate_vector2d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `distance3d` | ai/action_vehicle.c | Ported |  | `tools/engine_math_reference.py` |
| `action_vehicle_desirable` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_evaluate_seat` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_find_destination` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_setup_specific` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_perform` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_find_impromptu_seat` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_vehicle_setup_impromptu` | ai/action_vehicle.c | Compiled |  | compiled into game.wasm |
| `action_wait_begin` | ai/action_wait.c | Compiled |  | compiled into game.wasm |
| `action_wait_perform` | ai/action_wait.c | Compiled |  | compiled into game.wasm |
| `action_wait_control` | ai/action_wait.c | Compiled |  | compiled into game.wasm |
| `action_wait_setup` | ai/action_wait.c | Compiled |  | compiled into game.wasm |
| `action_wait_update` | ai/action_wait.c | Compiled |  | compiled into game.wasm |
| `encounter_get_squad` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_perform` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_update` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_control` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_replace_prop` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_flush_position_indices` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_flush_structure_indices` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_panic_from_surprise` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_panic_from_damage` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_panic_from_burning_to_death` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_panic_from_attached_projectiles` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_panic_from_attached_melee_attackers` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_berserking_from_attacking_mode` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_berserking_from_proximity` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_berserking_from_damage` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_deny_transition` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_allowed_to_enter_vehicle` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_vehicle_exit` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_allow_cover_seeking` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_determine_pursuit_options` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_can_stop_guarding` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_can_stop_conversing` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_change` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_test_grenade` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_try_to_seek_cover` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_try_to_panic` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_try_to_enter_vehicle` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_get_pursuit_location` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_pursuit_consider_nearby_actor` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_name` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_mode_name` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_debug_color` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_class` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_get_default_state` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `set_real_vector2d` | ai/actions.c | Ported |  | `tools/engine_math_reference.py` |
| `set_real_vector3d` | ai/actions.c | Ported |  | `tools/engine_math_reference.py` |
| `point_to_line_distance3d` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_set_default_state` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_initial_action` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_pending_command_list` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_surprise` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_panic_transition` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_berserking_from_attached_projectiles` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_vehicle_entry` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_find_escape_from_danger` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_active_cover_seeking` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_combat_selection` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_lost_contact` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_done_fleeing` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_combat_status` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_combat_failure` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_exit_pursuit` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_try_to_throw_grenade` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_consider_grenade` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_try_to_evade` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_try_to_dive` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actors_searching_same_position` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_pursuit_find_nearby_actors` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_berserk_transition` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_combat_transition` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_grenade_throwing` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_evasion` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_action_handle_danger_avoidance` | ai/actions.c | Compiled |  | compiled into game.wasm |
| `actor_combat_enable_special_fire_situation` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_fire_wildly` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_disable_bursts` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_firing_blindly` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_currently_firing_burst` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_get_weapon_definition` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_get_weapon_vector` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_get_firing_variant_definition` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_get_firing_parameters` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_get_grenade_definition` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `fast_ftol` | ai/actor_combat.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `sine` | ai/actor_combat.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `cosine` | ai/actor_combat.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `tangent` | ai/actor_combat.c | Ported |  | `tools/engine_math_reference.py` |
| `cross_product2d` | ai/actor_combat.c | Ported |  | `tools/engine_math_reference.py` |
| `add_vectors3d` | ai/actor_combat.c | Ported |  | `tools/engine_item_reference.py` |
| `random_boolean` | ai/actor_combat.c | Ported |  | `tools/engine_random_reference.py` |
| `valid_real` | ai/actor_combat.c | Ported |  | `tools/engine_math_reference.py` |
| `actor_combat_find_nearby_target` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_start_pause` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_start_first_burst_delay` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_reaim_grenade` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_build_grenade_trajectory` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_find_grenade_target` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_check_collateral_damage` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_plan_grenade_trajectory` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `valid_realcmp` | ai/actor_combat.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `valid_real_normal3d` | ai/actor_combat.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `actor_combat_allow_special_fire_situation` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_aim_projectile` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_start_burst` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_retarget_grenade` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_aim_grenade` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `actor_combat_update` | ai/actor_combat.c | Compiled |  | compiled into game.wasm |
| `firing_position_store_evaluation_debug` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `firing_position_reject` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `pre_evaluator_guard` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `pre_evaluator_combatmove` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `post_evaluator_global` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `post_evaluator_pursuit` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `post_evaluator_hide` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `post_evaluator_uncover` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `post_evaluator_attack` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `firing_position_pre_evaluate` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `firing_position_post_evaluate` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `firing_positions_get_post_evaluation_bound` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `firing_position_compare` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_get_firing_position_group` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_clear_discarded_firing_positions` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_discard_firing_position` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_firing_position_discarded` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `pre_evaluator_global` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `pre_evaluator_pursuit` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `pre_evaluator_panic` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `firing_position_compute_line_of_sight` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `firing_position_forced_evaluation` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_nearby_firing_positions` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_select_firing_position` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_active_select_firing_position` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_change_firing_position` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `pre_evaluator_attack` | ai/actor_firing_position.c | Compiled |  | compiled into game.wasm |
| `actor_look_secondary_stop` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_compute_prop_interest` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_get_looking_definition` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_secondary` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_valid_aim_vector` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_valid_look_vector` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_looking_test_validity` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_idle_find_prop` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_idle_timer` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_find_random_vector` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `valid_real_normal2d` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_decode_direction` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_idle_new_major_direction` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_idle_new_minor_direction` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_update` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_look_affect_movement` | ai/actor_looking.c | Compiled |  | compiled into game.wasm |
| `actor_move_keep_moving_past_destination` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_animation_busy` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_path_clear` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_path_has_path` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_path_at_destination` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_path_get_destination_firing_position_index` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_path_input_new` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `arccosine` | ai/actor_moving.c | Ported |  | `tools/engine_math_reference.py` |
| `midpoint3d` | ai/actor_moving.c | Ported |  | `tools/engine_math_reference.py` |
| `actor_test_destination` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_get_stopping_distances` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_animation_impulse` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_force_stop` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_try_evasion_vector` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_try_evasion_direction` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_aim_jump` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_avoidance_setup` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_test_avoidance_vector` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_vector_avoidance_find_direction` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_transform_avoidance_vector` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_get_avoidance_direction` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_initialize` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_path_3d_available` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_vector_avoidance` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_calculate_movement` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_calculate_controlled_by_aiming` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_path_refresh` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_destination_update` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_to_point` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_to_move_position` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_to_firing_position` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_to_prop` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_calculate_free` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_update` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_halt` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_move_halt_at_firing_position` | ai/actor_moving.c | Compiled |  | compiled into game.wasm |
| `actor_perception_acknowledge` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_get_perception_knowledge` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_get_vision_distances` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_emotion_assess_unopposable_danger` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_emotion_get_unopposable_enemy` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_desire_prop` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_find_prop_pathfinding_location` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_find_killer_prop_index` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `arctangent` | ai/actor_perception.c | Ported |  | `tools/engine_math_reference.py` |
| `actor_perception_find_recent_damaging_prop_index` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_forget_recent_damage` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_retreat_successful` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_compute_prop_unopposable` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_compute_prop_target_weight` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_situation_update_target_status` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_situation_combat_status_update` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_situation_update` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_situation_try_new_target` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_friend_prop_is_attacking` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_aiming_vector_test_blockage` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_emotion_flee_with_friends` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_emotion_unopposable_retreat` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_unit_from_swarm` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_assess_vehicle_danger` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_berserk` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_visibility_at_point` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_audibility_at_point` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_find_sense_position` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_refresh_test_object` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `prop_position_refresh` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_refresh` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_refresh_danger_zone` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_expected_acknowledgement` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_unreachable` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_tried_to_uncover` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_tried_to_search` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_abandoned_search` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_emotion_update` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_become_acknowledged` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `prop_status_refresh` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_qsort_compare_optional_props` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_create_orphan_from_friend` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_assess_suicide_danger` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_perception_update` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_clear` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_combat` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_surprise` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_suspicion` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_prop_sighted` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_enter_combat_found_body` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_enter_combat_perceived_enemy` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_enter_combat_friend_in_combat` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_bumped` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_environmental_noise` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_heard_shooting` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_was_surprised` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_maneuvering` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_vehicle_eviction` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_abandon_stationary_facing` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_prop_acknowledged` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_damage` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_weapon_impact` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_weapon_detonation` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_prop_just_killed` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_prop_fleeing` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `actor_stimulus_noticed_danger_zone` | ai/actor_stimulus.c | Compiled |  | compiled into game.wasm |
| `carrier_decide_action` | ai/actor_type_carrier.c | Compiled |  | compiled into game.wasm |
| `crew_decide_action` | ai/actor_type_crew.c | Compiled |  | compiled into game.wasm |
| `elite_decide_action` | ai/actor_type_elite.c | Compiled |  | compiled into game.wasm |
| `engineer_decide_action` | ai/actor_type_engineer.c | Compiled |  | compiled into game.wasm |
| `flood_decide_action` | ai/actor_type_flood.c | Compiled |  | compiled into game.wasm |
| `actor_type_flood_desire_shamble` | ai/actor_type_flood.c | Compiled |  | compiled into game.wasm |
| `grunt_decide_action` | ai/actor_type_grunt.c | Compiled |  | compiled into game.wasm |
| `hunter_decide_action` | ai/actor_type_hunter.c | Compiled |  | compiled into game.wasm |
| `infection_decide_action` | ai/actor_type_infection.c | Compiled |  | compiled into game.wasm |
| `infection_wander_pause_time` | ai/actor_type_infection.c | Compiled |  | compiled into game.wasm |
| `infection_wander_move_time` | ai/actor_type_infection.c | Compiled |  | compiled into game.wasm |
| `infection_swarm_control` | ai/actor_type_infection.c | Compiled |  | compiled into game.wasm |
| `infection_swarm_aim_jump` | ai/actor_type_infection.c | Compiled |  | compiled into game.wasm |
| `jackal_decide_action` | ai/actor_type_jackal.c | Compiled |  | compiled into game.wasm |
| `marine_decide_action` | ai/actor_type_marine.c | Compiled |  | compiled into game.wasm |
| `mounted_weapon_decide_action` | ai/actor_type_mounted_weapon.c | Compiled |  | compiled into game.wasm |
| `sentinel_decide_action` | ai/actor_type_sentinel.c | Compiled |  | compiled into game.wasm |
| `actor_type_definition_get` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_types_initialize` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_get_name` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_get_race` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_get_when_to_search_at_target` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_get_when_to_pursue` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_get_when_to_search_pursuit` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_get_pursuit_controller` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_get_swarm` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_initialize` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_decide_action` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_swarm_control` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actor_type_swarm_aim_jump` | ai/actor_types.c | Compiled |  | compiled into game.wasm |
| `actors_initialize` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_dispose` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_initialize_for_new_map` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_dispose_from_old_map` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_set_team` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_activation_debug_color` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_verify_unit_activation` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_verify_activation` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_detach_from_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_swarm_detach_from_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_swarm_cache_delete` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_swarm_component_refresh` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_is_noncombat` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_in_combat` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_is_fighting` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_attacking_target` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_is_leaping` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_get_weapon` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_has_ranged_weapon` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_target_unit_index` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_derive_target_information` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_switch_props` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_flush_position_indices` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_flush_structure_indices` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_has_unlimited_grenades` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_handle_damage` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_freeze_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_freeze` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_freeze` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_update_begin` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_move_randomly` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_change_encounter` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_update_end` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_get_timeslice` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_clear_output` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_input_update` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_find_pathfinding_location` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_destination_tolerance` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_input_sample_position` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_clear_orders` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_decision_loop` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_handle_spatial_effect` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_handle_communication` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control_crouch` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control_jump` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control_primary_trigger` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control_secondary_trigger` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control_throw_grenade` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control_exact_facing` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control_stop_animation_impulse` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_new` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_customize_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_set_dormant` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_swarm_component_setup` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_delete_props` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_delete` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_swarm_cache_new` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_get_running_blind_vector` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_kill` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_died` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_swarm_unit_died` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_braindead` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_handle_unit_effect` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_set_active` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_swarm_attach_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_erase` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_randomly_control_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_general_update` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_handle_unit_effect` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_unit_control` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_attach_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_update` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_create_for_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actor_place` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_spawn_from_unit` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `actors_update` | ai/actors.c | Compiled |  | compiled into game.wasm |
| `ai_initialize` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_dispose` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_dispose_from_old_map` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_place` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_globals_ai_active` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_globals_dialogue_triggers_enabled` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_globals_grenades_enabled` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_get_major_upgrade_chance` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_adjust_damage` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_erase` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_release_inactive_swarms` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `compare_potentially_releasable_entities` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_find_inactive_encounters` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_release_inactive_encounters` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_try_vehicle_eviction` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_get_responsible_unit` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_death` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_killing_spree` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_allegiance_status_changed` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_allegiance_broken_notification` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_update_team_status` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_bump` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_damage` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_flush_spatial_effects` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_create_mounted_weapons_for_unit` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_deleted_object` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_unit_effect` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_enter_vehicle` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_exit_vehicle` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_generate_line_of_fire_pill` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_disconnect_from_structure_bsp` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_reconnect_to_structure_bsp` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_get_race_from_team_index` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_initialize_for_new_map` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_update` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_consider_major_upgrade` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_find_line_of_fire_friend_pills` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_place_pending_mounted_weapons` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_test_line_of_fire` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_test_line_of_sight` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_test_ballistic_line_of_fire` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_editing` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `code_000309a0` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_enemies_can_see_player` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_enemies_attacking_player` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_handle_spatial_effect` | ai/ai.c | Compiled |  | compiled into game.wasm |
| `ai_communication_initialize` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_dispose` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_initialize_for_new_map` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_dispose_from_old_map` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_get_type_name` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_get_type_by_name` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_packet_new` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_close` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_not_close` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_searching` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_same_platoon` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_fighting` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_fighting_close` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_same_target` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_no_certain_target` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `reply_filter_flee_leader` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_consider_speech` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `actor_communication_team` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_look_secondary_at_unit` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_look_secondary_at_object` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_status` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_line` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_advance` | ai/ai_communication.c | Compiled | content/src/campaign.rs:103 | compiled into game.wasm |
| `ai_conversation_finish` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_new` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_line_begin` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_line_perform` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_line_end` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `actor_reset_idle_vocalization_timer` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `actor_communication_update` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_handle_received_looking` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_update_speech_timers` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_get_player_rating` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_stop` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_actor_deleted` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_unit_died` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_find_participant` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_started` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_notify` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_actor_talk_weight` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_find_specific_actor_to_talk` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_find_global_actor_to_talk` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation_begin` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_find_actor_to_reply_to_player` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_finished` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_conversation` | ai/ai_communication.c | Compiled | content/src/campaign.rs:63 | compiled into game.wasm |
| `ai_conversation_update` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_communication_event` | ai/ai_communication.c | Compiled |  | compiled into game.wasm |
| `ai_profile_initialize` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_dispose` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_initialize_for_new_map` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_dispose_from_old_map` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_update` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_display` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_sample_encounters` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_change_render_spray` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_sample_actors` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_sample_units` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_sample_swarms` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_sample_props` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_sample_swarm_actors` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_sample_swarm_components` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_draw_string` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render_encounters` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render_actors` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render_swarms` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render_props` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render_collisions` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render_paths` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render_spray` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `ai_profile_render` | ai/ai_profile.c | Compiled |  | compiled into game.wasm |
| `scenario_get_encounter_by_name` | ai/ai_scenario_definitions.c | Compiled |  | compiled into game.wasm |
| `encounter_definition_get_squad_by_name` | ai/ai_scenario_definitions.c | Compiled |  | compiled into game.wasm |
| `encounter_definition_get_platoon_by_name` | ai/ai_scenario_definitions.c | Compiled |  | compiled into game.wasm |
| `choose_random_array_element` | ai/ai_scenario_definitions.c | Compiled |  | compiled into game.wasm |
| `encounter_get_platoon` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_script_initialize` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_script_dispose` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_script_initialize_for_new_map` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_script_dispose_from_old_map` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_from_string` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_to_string` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_platoon_iterator_new` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_platoon_iterator_next` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_squad_iterator_new` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_squad_iterator_next` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_actor_iterator_new` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_index_actor_iterator_next` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `object_list_from_ai_reference` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attach_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attach_units` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_detach_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_detach_units` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_place` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_kill_internal` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_kill` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_kill_silent` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_erase` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_erase_all` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_deselect` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_select` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_spawn_actor` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_respawn` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_deaf` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_blind` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_units` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_timer_start` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_timer_expire` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_count_internal` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_swarm_count` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_nonswarm_count` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_living_count` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_living_fraction` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_strength` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_is_attacking` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attack` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_defend` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_maneuver` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_maneuver_enable` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_find_target_squad` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_internal` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_by_unit_internal` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_by_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_and_speak` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allegiance` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allegiance_remove` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allegiance_broken` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_candidate_qsort` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_going_to_vehicle` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_exit_vehicle` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_braindead` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_braindead_by_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_ignore` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_prefer_target` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_teleport_starting_location_private` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_teleport_starting_location_if_unsupported` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_teleport_starting_location` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_try_to_fight_nothing` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_try_to_fight` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_try_to_fight_player` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allow_charge` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_by_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_advance` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_advance_by_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_status_internal` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_status` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_free` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_free_units` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attach_free` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_force_active` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_force_active_by_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_return_state` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_current_state` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_assess_status` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_status` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_reconnect` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_playfight` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_encounter` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_find_vehicle_enterable` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_distance` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_team` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_actor_type` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_actors` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_disable` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_look_at_object` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_stop_looking` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_automatic_migration_target` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_disable` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_players` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_unit` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_ai` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_distance` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_stop` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_advance` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_line` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_status` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_link_activation` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_berserk` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_team` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allow_dormant` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_encounter` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_players` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_retreat` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_go_to_vehicle_internal` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_go_to_vehicle` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_go_to_vehicle_override` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_renew` | ai/ai_script.c | Compiled |  | compiled into game.wasm |
| `encounters_initialize` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_dispose` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_dispose_from_old_map` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_compute_activation_cluster_bit_vector` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_detach_actor` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_attach_unit` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounterless_attach_actor` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounterless_detach_actor` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_get_by_name` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_iterator_new` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_iterator_next` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_actor_iterator_new` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_actor_iterator_next` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_actor_iterator_prev` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `actor_iterator_new` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `actor_iterator_next` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_clear_pursuit` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_find_pursuit` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_modify_pursuit_desires` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_determine_pursuit_availability` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `squad_reset_starting_locations` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_definition_fit` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_new` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounterless_activate` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_activate` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_deactivate` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_link_activation` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `squad_get_actor_type` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_post_combat_add_possibility` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_stand_down` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_post_combat_select_random_behavior` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_post_combat` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_set_blind` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_set_deaf` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_squad_timer_expire` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_update_timers` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_test_rule` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_initialize_for_new_map` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_unit_died` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_verify_firing_position_owner_actor_indices` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_build_firing_position_owner_actor_indices` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_mark_examined_pursuit_position` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_pursuit_position_already_examined` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_get_actor_starting_location` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_force_activate` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_force_deactivate` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_update_respawn` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_update_platoons` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_update_follow` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_spawn_actor` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_set_respawn` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_control_actors` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_update_squads` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounterless_deactivate` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_attach_actor` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_update_status` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_update_dirty_status` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounter_create` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_test_activation` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_create_for_new_map` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `encounters_update` | ai/encounters.c | Compiled |  | compiled into game.wasm |
| `paths_initialize` | ai/path.c | Compiled |  | compiled into game.wasm |
| `paths_dispose` | ai/path.c | Compiled |  | compiled into game.wasm |
| `paths_initialize_for_new_map` | ai/path.c | Compiled |  | compiled into game.wasm |
| `paths_dispose_from_old_map` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_input_new` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_input_set_target_object` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_input_set_start` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_input_set_attractor` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_input_set_search_bounds` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_new` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_destination` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_reset` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_heap_verify` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_heap_bubble_up` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_heap_bubble_down` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_heap_pop_cheapest_node` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_heap_insert` | ai/path.c | Compiled |  | compiled into game.wasm |
| `surface_is_broken` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_get_node` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_node_from_hash_table` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_3d_available` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_3d_build_path` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_approach_point` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_build_path` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_begin` | ai/path.c | Compiled |  | compiled into game.wasm |
| `closest_available_point_on_surface` | ai/path.c | Compiled |  | compiled into game.wasm |
| `build_path_edges_for_surface` | ai/path.c | Compiled |  | compiled into game.wasm |
| `closest_point_to_attractor` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_attractor_weight` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_estimated_distance` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_traverse` | ai/path.c | Compiled |  | compiled into game.wasm |
| `path_state_find` | ai/path.c | Compiled |  | compiled into game.wasm |
| `obstacles_get_disc` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `obstacle_from_disc` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_get_step` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_get_step_index` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_parent_index` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_left_index` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_right_index` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_cost` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `error_heap` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_verify` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_up` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_down` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_insert` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `heap_remove` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `render_debug_path` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `valid_real_point2d` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_add_step` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_new` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_test_pill2d` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_add_steps` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_iterate` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_find` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `path_avoid_obstacles` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `render_debug_obstacle_path` | ai/path_obstacle_avoidance.c | Compiled |  | compiled into game.wasm |
| `obstacles_new` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `set_real_point2d` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `rotate_vector2d` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `project_point3d` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `point_in_circle` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `point_in_sphere` | ai/path_obstacles.c | Ported |  | `tools/engine_math_reference.py` |
| `circle_intersect_ray` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `circle_tangents` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `obstacles_add_disc` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `obstacles_get_discs_in_sphere` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `obstacles_test_circle` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `obstacles_test_pill` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `obstacles_disc_tangents` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `obstacles_disc_neighborhood` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `obstacles_recompute` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `render_debug_obstacles` | ai/path_obstacles.c | Compiled |  | compiled into game.wasm |
| `surface_is_walkable` | ai/path_smoothing.c | Compiled |  | compiled into game.wasm |
| `find_tangent_point` | ai/path_smoothing.c | Compiled |  | compiled into game.wasm |
| `find_avoidance_point` | ai/path_smoothing.c | Compiled |  | compiled into game.wasm |
| `choose_turning_point` | ai/path_smoothing.c | Compiled |  | compiled into game.wasm |
| `find_turning_point` | ai/path_smoothing.c | Compiled |  | compiled into game.wasm |
| `path_smooth` | ai/path_smoothing.c | Compiled |  | compiled into game.wasm |
| `structure_test_ray2d` | ai/path_structure_bsp.c | Compiled |  | compiled into game.wasm |
| `structure_surfaces_are_equivalent` | ai/path_structure_bsp.c | Compiled |  | compiled into game.wasm |
| `clip_empty_interval_by_solid_interval` | ai/path_structure_bsp.c | Compiled |  | compiled into game.wasm |
| `structure_test_line2d` | ai/path_structure_bsp.c | Compiled |  | compiled into game.wasm |
| `structure_surface_index_from_point` | ai/path_structure_bsp.c | Compiled |  | compiled into game.wasm |
| `structure_test_pill2d` | ai/path_structure_bsp.c | Compiled |  | compiled into game.wasm |
| `props_initialize` | ai/props.c | Compiled |  | compiled into game.wasm |
| `props_dispose` | ai/props.c | Compiled |  | compiled into game.wasm |
| `props_initialize_for_new_map` | ai/props.c | Compiled |  | compiled into game.wasm |
| `props_dispose_from_old_map` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_add` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_new_blank` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_remove` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_iterator_new` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_iterator_next` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_new_unacknowledged` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_setup_orphan` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_orphan_transition` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_orphan_from_friend` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_orphan_update_information` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_delete` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_get_active_by_unit_index` | ai/props.c | Compiled |  | compiled into game.wasm |
| `prop_get_base_by_unit_index` | ai/props.c | Compiled |  | compiled into game.wasm |
| `actor_perception_midpoint3d` | ai/actor_perception.c | Compiled |  | compiled into game.wasm |
| `ai_debug_initialize` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_dispose` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_dispose_from_old_map` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_clear_storage` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_actor_deleted` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_get_last_path` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_highlight_cluster` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_drawstack_setup` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_drawstack` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_path_node` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_highlight_unit` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_surface` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_actor` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_path_nodes` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_path_storage` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_sound_point_set` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_lineoffire_new` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_lineoffire_success` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_lineofsight_reset` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_idle_look_clear` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_lineoffire_addpill` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_idle_look_addprop` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_communication_toggle_bits` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_communication_suppress` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_communication_ignore` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_communication_focus` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_lineofsight_findpoint` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_lineofsight_storeray` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_lineofsight` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_vocalize` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_speak_list` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_speak` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_describe_actor` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_select_encounter` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_select_actor` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_initialize_for_new_map` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_update` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_change_selected_actor` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_change_selected_encounter` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_teleport_to` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_get_path_storage` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_lineoffire` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_ballistic_lineoffire` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_lineofsight` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_aiming_validity` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_idle_look` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_spatial_effects` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_select_this_actor` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_speech` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_vehicles_enterable` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_path` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_paths_failed` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_all_actors` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_encounter` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_speech_update` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_path_storage_update` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_render_path_line` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `ai_debug_get_this_actor` | ai/ai_debug.c | Compiled |  | compiled into game.wasm |
| `path_pathfinding_surface` | ai/path_structure_bsp.c | Compiled |  | compiled into game.wasm |

## `bink`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `bink_draw_frame` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_active` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_ui_rendering_inhibited` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_in_progress` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_free_texture_cache` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_get_memory_available` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_alloc_permanent` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_alloc` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_decompress_audio_frame` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_free` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `is_all_bink_memory_free` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_decompress_video_frame` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_query_analog_controller_buttons` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `code_001b5c60` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `code_001b5c80` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_initialize` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `code_001b5cc0` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `code_001b5d20` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_stop` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_dispose` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_update__internal` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_render` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_start` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |
| `bink_playback_update` | bink/bink_playback.c | Compiled |  | compiled into game.wasm |

## `bitmaps`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `ColorToFcolor` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `FcolorToColor` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `ColorToRGB` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `RGBToColor` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `Square3x3` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `Quantize` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `ClipExtrema` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `AllSame` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `EncodeBlockRGBColorKey` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `DecodeBlockRGB` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `DecodeBlockRGB__single_pixel` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `DecodeBlockAlpha4` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `DecodeBlockAlpha4__single_pixel` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `DecodeBlockAlpha3` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `DecodeBlockAlpha3__single_pixel` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `EncodeBlockRGB` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `EncodeBlockAlpha4` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `EncodeBlockAlpha3` | bitmaps/s3tc/s3tc.c | Compiled |  | compiled into game.wasm |
| `bitmap_initialize_line` | bitmaps/bitmap_drawing.c | Compiled |  | compiled into game.wasm |
| `bitmap_step_line` | bitmaps/bitmap_drawing.c | Compiled |  | compiled into game.wasm |
| `bitmap_fill_rectangle` | bitmaps/bitmap_drawing.c | Compiled |  | compiled into game.wasm |
| `bitmap_copy` | bitmaps/bitmap_drawing.c | Compiled |  | compiled into game.wasm |
| `bitmap_tile_and_bevel_rectangle` | bitmaps/bitmap_drawing.c | Compiled |  | compiled into game.wasm |
| `bitmap_draw_line` | bitmaps/bitmap_drawing.c | Compiled |  | compiled into game.wasm |
| `bitmap_frame_rectangle` | bitmaps/bitmap_drawing.c | Compiled |  | compiled into game.wasm |
| `extract_initialize` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_find_sequence_bounds` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_warn_about_horizontal_border` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_plateless_cube_map` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_find_bitmap_bounds` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_get_bitmap_format` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_pixels_to_mipmap` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_pixels_from_mipmap` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_build_texture_pages_by_sequence` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_build_debug_plate` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_mipmaps_to_bitmap` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_add_bitmap` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_3d_textures` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_cube_maps` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_sprites` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_bitmap` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_without_sequences` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_sequence` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `extract_sequences` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `bitmaps_extract` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `bitmaps_extract_from_plate` | bitmaps/bitmap_extract.c | Compiled |  | compiled into game.wasm |
| `postprocess_bitmap` | bitmaps/bitmap_group.c | Compiled |  | compiled into game.wasm |
| `delete_bitmap` | bitmaps/bitmap_group.c | Compiled |  | compiled into game.wasm |
| `postprocess_bitmap_group` | bitmaps/bitmap_group.c | Compiled |  | compiled into game.wasm |
| `bitmap_group_try_and_get_bitmap` | bitmaps/bitmap_group.c | Compiled |  | compiled into game.wasm |
| `bitmap_group_get_bitmap_from_sequence` | bitmaps/bitmap_group.c | Ported | content/src/bitmap.rs:30 | `tools/engine_hud_sprite_reference.py` |
| `bitmap_group_add_bitmap` | bitmaps/bitmap_group.c | Compiled |  | compiled into game.wasm |
| `bitmap_fill` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_alpha_to_rgb` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_clone` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_shrink` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_shrink` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_shrink` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_fade` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_smooth` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_smooth` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_smooth` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_sharpen` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_sharpen` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_sharpen` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_alpha_bleed` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_alpha_bleed` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_alpha_bleed` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_height_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `real_rgb_color_brightness` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `rgb_color_to_hsv_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `hsv_color_to_rgb_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `real_rgb_color_to_real_hsv_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `real_hsv_color_to_real_rgb_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `argb_color_to_real_argb_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `rgb_color_to_real_rgb_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `pixel32_to_real_argb_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `pixel32_to_real_rgb_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `valid_real_rgb_color` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_shrink` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_smooth` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_sharpen` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_alpha_bleed` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_height_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_height_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_vector_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_vector_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_cm_vector_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_compress_to_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_uncompress_from_mipmap` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `rgb_colors_interpolate` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `rgb_colors_interpolate_and_scale` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_height_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_vector_map` | bitmaps/bitmap_utilities.c | Compiled |  | compiled into game.wasm |
| `bitmap_type_get_string` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_get_string` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_get_bits_per_pixel` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_changed` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_delete` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_address` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_address` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_cube_map_address` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_mipmap_address` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_to_a8r8g8b8` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_byte_swap_pixels` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `palette_find_closest_match` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_type_valid_width` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_type_valid_height` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_type_valid_depth` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_verify` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_rebuild` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_get_max_mipmap_count` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_mipmap_get_width` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_mipmap_get_height` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_mipmap_get_depth` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_mipmap_get_pixel_count` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_mipmap_get_pixel_data_size` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_mipmap_get_row_pitch` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_get_pixel` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_get_pixel_count` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_get_pixel_data_size` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_2d_new` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_new` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_cube_map_new` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_slice_extract` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_3d_slice_insert` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_cube_map_face_extract` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_cube_map_face_insert` | bitmaps/bitmaps.c | Compiled |  | compiled into game.wasm |
| `bitmap_quantitize_read_row` | bitmaps/bitmaps_quantitize.c | Compiled |  | compiled into game.wasm |
| `row_dither` | bitmaps/bitmaps_quantitize.c | Compiled |  | compiled into game.wasm |
| `bitmap_quantitize` | bitmaps/bitmaps_quantitize.c | Compiled |  | compiled into game.wasm |
| `targa_export` | bitmaps/targa_file.c | Compiled |  | compiled into game.wasm |
| `tiff_get_bounds` | bitmaps/tiff_file.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `tiff_export` | bitmaps/tiff_file.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `tiff_import` | bitmaps/tiff_file.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `DECLARE1` | bitmaps/libtiff/mkg3states.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `DECLARE3` | bitmaps/libtiff/mkg3states.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `DECLARE2` | bitmaps/libtiff/mkg3states.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `build_null_mode_tables` | bitmaps/libtiff/mkg3states.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `build_horiz_mode_tables` | bitmaps/libtiff/mkg3states.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `build_uncomp_mode_tables` | bitmaps/libtiff/mkg3states.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `findScheme` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoEncode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoRowEncode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoStripEncode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoTileEncode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoDecode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoRowDecode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoStripDecode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFNoTileDecode` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFSetCompressionScheme` | bitmaps/libtiff/tif_compress.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteDirectory` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteNormalTag` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFSetupShortLong` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteRational` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWritePerSampleShorts` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFSetupShortPair` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteShortTable` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteString` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteShortArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteLongArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteRationalArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteFloatArray` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteJPEGQTables` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteJPEGCTables` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteTransferFunction` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWriteData` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFLinkDirectory` | bitmaps/libtiff/tif_dirwrite.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `defaultHandler` | bitmaps/libtiff/tif_error.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFSetErrorHandler` | bitmaps/libtiff/tif_error.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFError` | bitmaps/libtiff/tif_error.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFInitOrder` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `getMode` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFOpen` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFFdOpen` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFScanlineSize` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFFileName` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFFileno` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFGetMode` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFIsTiled` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFCurrentRow` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFCurrentDirectory` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFCurrentStrip` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFCurrentTile` | bitmaps/libtiff/tif_open.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadScanline` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFSeek` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadEncodedStrip` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadRawStrip` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadRawStrip1` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFFillStrip` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadTile` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadEncodedTile` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadRawTile` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadRawTile1` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFFillTile` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFReadBufferSetup` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFStartStrip` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFStartTile` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFCheckRead` | bitmaps/libtiff/tif_read.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `ThunderDecode` | bitmaps/libtiff/tif_thunder.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `ThunderDecodeRow` | bitmaps/libtiff/tif_thunder.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFInitThunderScan` | bitmaps/libtiff/tif_thunder.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFSetWarningHandler` | bitmaps/libtiff/tif_warning.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |
| `TIFFWarning` | bitmaps/libtiff/tif_warning.c | Replaced | port/linux/game/tiff_file.c | the editor and tools |

## `bungie_net`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `add64` | bungie_net/common/64bit_math.c | Compiled |  | compiled into game.wasm |
| `negate64` | bungie_net/common/64bit_math.c | Compiled |  | compiled into game.wasm |
| `subtract64` | bungie_net/common/64bit_math.c | Compiled |  | compiled into game.wasm |
| `multiply64` | bungie_net/common/64bit_math.c | Compiled |  | compiled into game.wasm |
| `divide64` | bungie_net/common/64bit_math.c | Compiled |  | compiled into game.wasm |
| `key_agreement_get_packet_type` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `key_agreement_decode_packet` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `key_agreement_encode_packet` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `key_agreement_build_message` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `build_initiate_key_agreement_message` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `build_finalize_key_agreement_message` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `is_message_encryption_key_message` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `initiate_key_exchange` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `complete_key_exchange` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `initialize_key_agreement_packets` | bungie_net/common/key_agreement.c | Compiled |  | compiled into game.wasm |
| `reversible_crypt` | bungie_net/common/message_encryption.c | Compiled |  | compiled into game.wasm |
| `tea_encipher` | bungie_net/common/message_encryption.c | Compiled |  | compiled into game.wasm |
| `tea_decipher` | bungie_net/common/message_encryption.c | Compiled |  | compiled into game.wasm |
| `message_encrypt` | bungie_net/common/message_encryption.c | Compiled |  | compiled into game.wasm |
| `message_decrypt` | bungie_net/common/message_encryption.c | Compiled |  | compiled into game.wasm |
| `build_message_header` | bungie_net/common/message_header.c | Compiled |  | compiled into game.wasm |
| `byte_swap_message_header` | bungie_net/common/message_header.c | Compiled |  | compiled into game.wasm |
| `create_message` | bungie_net/common/message_header.c | Compiled |  | compiled into game.wasm |
| `compare_ulongs_descending` | bungie_net/common/prime_numbers.c | Compiled |  | compiled into game.wasm |
| `primegen` | bungie_net/common/prime_numbers.c | Compiled |  | compiled into game.wasm |
| `randomprime` | bungie_net/common/prime_numbers.c | Compiled |  | compiled into game.wasm |
| `probable_prime64` | bungie_net/common/prime_numbers.c | Compiled |  | compiled into game.wasm |
| `x_exp_y_mod_n` | bungie_net/common/public_key_crypt.c | Compiled |  | compiled into game.wasm |
| `generate_diffie_hellman_public_key` | bungie_net/common/public_key_crypt.c | Compiled |  | compiled into game.wasm |
| `generate_diffie_hellman_private_key` | bungie_net/common/public_key_crypt.c | Compiled |  | compiled into game.wasm |
| `generate_key_parameters` | bungie_net/common/public_key_crypt.c | Compiled |  | compiled into game.wasm |
| `generate_public_key` | bungie_net/common/public_key_crypt.c | Compiled |  | compiled into game.wasm |
| `generate_private_key` | bungie_net/common/public_key_crypt.c | Compiled |  | compiled into game.wasm |
| `randomrange` | bungie_net/common/random_numbers.c | Compiled |  | compiled into game.wasm |
| `randomrange64` | bungie_net/common/random_numbers.c | Compiled |  | compiled into game.wasm |
| `get_thread_from_pool` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `get_mutex_from_pool` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `create_thread` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `thread_has_exited` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `dispose_thread` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `create_mutex` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `take_mutex` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `release_mutex` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `dispose_mutex` | bungie_net/common/thread_win32.c | Compiled |  | compiled into game.wasm |
| `create_transport_address` | bungie_net/network/transport_address.c | Compiled |  | compiled into game.wasm |
| `delete_transport_address` | bungie_net/network/transport_address.c | Compiled |  | compiled into game.wasm |
| `transport_address_equivalent` | bungie_net/network/transport_address.c | Compiled |  | compiled into game.wasm |
| `transport_address_to_string` | bungie_net/network/transport_address.c | Compiled |  | compiled into game.wasm |
| `transport_error_to_string` | bungie_net/network/transport_address.c | Compiled |  | compiled into game.wasm |
| `net_startup_debug` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `get_next_available_set_array_index` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_push_key` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_pop_key` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_get_nonce` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_nonce_is_equal` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_is_nonce` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_client_stop` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_get_xnaddr` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_get_key_id` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_get_key` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_initialize` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_dispose` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_network_available` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `create_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `delete_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `poll_ep_array_compare_proc` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `poll_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `add_endpoint_to_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `remove_endpoint_from_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `rewind_endpoint_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `get_next_endpoint_from_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `count_endpoints_in_set` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_server_initialize` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_server_terminate` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `transport_client_start` | bungie_net/network/transport_endpoint_set_winsock.c | Compiled |  | compiled into game.wasm |
| `add_connect_thread` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `mark_connection_thread_as_terminated` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `connection_thread_list_maintenance` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `create_transport_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `get_endpoint_type` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `read_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `write_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `endpoint_readable` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `endpoint_writeable` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `endpoint_connected` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `endpoint_listening` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `endpoint_blocking` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `get_endpoint_error` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `endpoint_equivalent` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `winsock_error_to_string` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `create_socket` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `get_endpoint_address` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `set_endpoint_blocking` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `bind_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `connect_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `disconnect_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `connect_async_thread_proc` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `connect_endpoint_async` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `cancel_connect_process` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `listen_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `accept_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `read_from_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `write_to_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `delete_transport_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |
| `reject_endpoint` | bungie_net/network/transport_endpoint_winsock.c | Compiled |  | compiled into game.wasm |

## `cache`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `cache_files_map_directory` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `scenario_tags_unload` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_files_open` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_files_close` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_groups_checksum` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_get_checksum` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_loaded` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_enable_writes` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_disable_writes` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_block_resize` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_data_resize` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_block_add_element` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_block_delete_element` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_load` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_unload` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_file_get_path` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_reference_set` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_iterator_new` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_iterator_next` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_get_tag_instance` | cache/cache_files.c | Compiled | runtime/src/views.rs:9 | compiled into game.wasm |
| `cache_file_header_verify` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_give_time_to_precache` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `scenario_tags_load` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `scenario_structure_bsp_load` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `scenario_structure_bsp_unload` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_get` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_get_name` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_get_group_tag` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_copy_stop_requested` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_buffer_size` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_set_priority` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_compressed_file_complete` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_begin` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_queue_end` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_initialize_zlib` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_dispose_zlib` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_compressed_alloc` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_compressed_free` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_initialize_read_buffers` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_initialize_file_data` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_block_on_raw_read` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_block_on_raw_write` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `acquire_read_request` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_read_buffer_size` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_get_read_buffer` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_get_write_buffer` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_write_buffer_size` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `any_bit_vector_flag_set` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_wait_for_async_io` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_set_flag` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_get_flags` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `decompressor_reset_timing` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `decompressor_timer_start` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `decompressor_timer_stop` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_print_timing` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_yield` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_get_status` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_end` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_FileIOCompletionRoutine` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_read` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_write` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_read_raw` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_write_raw` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_read_internal` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_read_request` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_read_by_index` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_release_read_request` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_write_internal` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_initialize_read_data` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_issue_initial_reads` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_update_write_buffers` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_run_decompression` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `simple_cache_copy_thread` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_copy_initialize` | cache/cache_files_decompress_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_dispose` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_issue_async_request` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_block_on_async_request` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_request_get` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_request_next_free_index` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_requests_flush` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_set_priority` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_in_progress` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_is_copying_map` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_map_queue_end` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_file_get` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_file_invalidate` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_file_get_handle` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_file_get_size` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_file_get_path` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_windows_thread_wake` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_read_io_completion_routine` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_blocking_io_completion_routine` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_get_map_path` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_IsBusy` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_BlockUntilNotBusy` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Register` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DIndexBuffer8_IsBusy` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DIndexBuffer8_BlockUntilNotBusy` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_close` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_read` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_promote_read` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_read_header_from_dvd` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_issue_async_read` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_issue_async_write` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_block_until_not_busy` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `tags_header_register_vertex_and_index_buffers` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `tags_header_deregister_vertex_and_index_buffers` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `structure_bsp_header_register_vertex_buffers` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `structure_bsp_header_deregister_vertex_buffers` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_files_delete` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_verify_language` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_map_status` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_file_set_modification_date` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_file_read_header` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_files_find_map` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cached_map_files_find_free_map` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_windows_thread_proc` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_open` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_open_cache_files` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_map_loaded` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_map_begin` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_precache_map_end` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_file_windows_thread_create` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `cache_files_initialize` | cache/cache_files_windows.c | Compiled |  | compiled into game.wasm |
| `physical_memory_allocate` | cache/physical_memory_map.c | Compiled |  | compiled into game.wasm |
| `physical_memory_verify` | cache/physical_memory_map.c | Compiled | runtime/src/memory.rs:13 | compiled into game.wasm |
| `physical_memory_free` | cache/physical_memory_map.c | Compiled |  | compiled into game.wasm |
| `physical_memory_get_game_state_base_address` | cache/physical_memory_map.c | Compiled |  | compiled into game.wasm |
| `physical_memory_get_tag_cache_base_address` | cache/physical_memory_map.c | Compiled |  | compiled into game.wasm |
| `physical_memory_get_texture_cache_base_address` | cache/physical_memory_map.c | Compiled |  | compiled into game.wasm |
| `physical_memory_get_sound_cache_base_address` | cache/physical_memory_map.c | Compiled |  | compiled into game.wasm |
| `predicted_resources_sound_precache` | cache/predicted_resources.c | Compiled |  | compiled into game.wasm |
| `predicted_resources_precache` | cache/predicted_resources.c | Compiled |  | compiled into game.wasm |
| `sound_cache_delete` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_open` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_idle` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_sound_new` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_sound_delete` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_sound_finished` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_sound_hardware_lock` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_sound_hardware_unlock` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_locked_block_proc` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_delete_block_proc` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `cache_block_get_sound_permutation_name` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_start_loading_sound` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_new` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_flush` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_close` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `_sound_cache_sound_request` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `render_inverse_transform_screen_point` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `sound_cache_debug_render` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_delete` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_open` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_idle` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_bitmap_new` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_bitmap_delete` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_steal_memory` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_return_memory` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_name_block_proc` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_to_d3d_format` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `bitmap_format_to_d3d_linear_format` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `compare` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_IsBusy` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_KickPushBuffer` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `IDirect3DBaseTexture8_IsBusy` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `IDirect3DBaseTexture8_Register` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_flush` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_locked_block_proc` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_delete_block_proc` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_initialize_hardware_format` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `render_inverse_transform_screen_point` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_new` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_close` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_start_loading_bitmap` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_debug_render` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `_texture_cache_bitmap_get_hardware_format` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |
| `cache_empty_tag_instance` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_set_tag_instances` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_build_region` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_multiplayer_region` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_map_plays_multiplayer` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_show_multiplayer_unavailable` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_map_version` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `cache_files_map_present` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `tag_index_is_group` | cache/cache_files.c | Compiled |  | compiled into game.wasm |
| `sound_cache_sound_get_cache_sound` | cache/xbox_sound_cache.c | Compiled |  | compiled into game.wasm |
| `texture_cache_bitmap_valid` | cache/xbox_texture_cache.c | Compiled |  | compiled into game.wasm |

## `camera`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `bored_camera_new` | camera/bored_camera.c | Compiled |  | compiled into game.wasm |
| `bored_camera_shot_threshold_milliseconds` | camera/bored_camera.c | Compiled |  | compiled into game.wasm |
| `bored_camera_shot_duration_milliseconds` | camera/bored_camera.c | Compiled |  | compiled into game.wasm |
| `is_bored` | camera/bored_camera.c | Compiled |  | compiled into game.wasm |
| `is_still_bored` | camera/bored_camera.c | Compiled |  | compiled into game.wasm |
| `real_local_random_range` | camera/bored_camera.c | Ported |  | `tools/engine_particle_reference.py` |
| `valid_real_vector3d` | camera/bored_camera.c | Compiled |  | compiled into game.wasm |
| `valid_real_vector3d_axes2` | camera/bored_camera.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `bored_camera_update` | camera/bored_camera.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_enable` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_animation` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_first_person` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_dead` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_object_is_first_person_camera` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_absolute` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_camera_point_relative` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_camera_point_absolute` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_next_camera_point` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_object_relative_to` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_time` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_update` | camera/camera_scripting.c | Compiled |  | compiled into game.wasm |
| `player_has_allies` | camera/dead_camera.c | Compiled |  | compiled into game.wasm |
| `player_get_next_player_with_a_unit` | camera/dead_camera.c | Compiled |  | compiled into game.wasm |
| `dead_camera_new` | camera/dead_camera.c | Compiled |  | compiled into game.wasm |
| `dead_camera_update` | camera/dead_camera.c | Compiled |  | compiled into game.wasm |
| `director_get` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_initialize` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_dispose` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_inhibit_facing` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_inhibit_input` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_inhibited_facing` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_inhibited_input` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_set_mode` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_save_camera` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_get_perspective` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_desired_perspective` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_set_camera` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_initialize_variables` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_process_variables` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_dispose_from_old_map` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_load_camera` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_rotate_cameras` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_camera_deterministic` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_choose_game_perspective` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_script_camera` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_choose_camera_game` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_choose_camera_editor` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_choose_camera_script_camera_record` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_update_controls` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_choose_camera` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_initialize_for_new_map` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_update` | camera/director.c | Compiled |  | compiled into game.wasm |
| `director_initialize_for_saved_game` | camera/director.c | Compiled |  | compiled into game.wasm |
| `editor_camera_new` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_get_focus` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_set_focus` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_set_position` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_bump_speed` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_get_speed` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_use_roll` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_get_unit_focus` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_set_mode` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_get_mode` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_get_scripted` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_flying_update` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_get_field_of_view` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_move_to_point` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_set_position_and_roll` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_set_unit_focus` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_update` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_set_scripted` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `editor_camera_orbiting_update` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `translate_orbiting_to_flying` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `translate_flying_to_orbiting` | camera/editor_flying_camera.c | Compiled |  | compiled into game.wasm |
| `first_person_camera_new` | camera/first_person_camera.c | Compiled |  | compiled into game.wasm |
| `first_person_camera_deterministic` | camera/first_person_camera.c | Compiled |  | compiled into game.wasm |
| `first_person_camera_for_unit_and_vector` | camera/first_person_camera.c | Compiled |  | compiled into game.wasm |
| `first_person_camera_fake` | camera/first_person_camera.c | Compiled |  | compiled into game.wasm |
| `first_person_camera_update` | camera/first_person_camera.c | Compiled |  | compiled into game.wasm |
| `flying_camera_new` | camera/flying_camera.c | Compiled |  | compiled into game.wasm |
| `flying_camera_new_from_point_and_vector` | camera/flying_camera.c | Compiled |  | compiled into game.wasm |
| `flying_camera_update` | camera/flying_camera.c | Compiled |  | compiled into game.wasm |
| `following_camera_new` | camera/following_camera.c | Compiled |  | compiled into game.wasm |
| `unit_camera_get` | camera/following_camera.c | Compiled |  | compiled into game.wasm |
| `arcsine` | camera/following_camera.c | Ported |  | `tools/engine_math_reference.py` |
| `uniform_cubic_spline` | camera/following_camera.c | Ported |  | `tools/engine_wind_reference.py` |
| `uniform_cubic_spline_vector3d` | camera/following_camera.c | Ported |  | `tools/engine_wind_reference.py` |
| `camera_track_splut` | camera/following_camera.c | Compiled |  | compiled into game.wasm |
| `following_camera_deterministic` | camera/following_camera.c | Compiled |  | compiled into game.wasm |
| `following_camera_update` | camera/following_camera.c | Compiled |  | compiled into game.wasm |
| `observer_get` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_initialize` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_clear` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_initialize_for_new_map` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_dispose_from_old_map` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_get_camera` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_command_has_finished` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_reconnect_to_structure_bsp` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_update_accelerations` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_update_velocities` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_obsolete_position` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_up_from_forward` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_collision_test_with_t` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_collision_test_differential` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_update_command` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_set_camera` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_update_polynomial` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_apply_rotational_displacement` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_update_positions` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_rotational_displacement` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_find_displacement` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_check_penetration` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_postcheck` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_update_displacements` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_pass_time` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `observer_update` | camera/observer.c | Compiled |  | compiled into game.wasm |
| `orbiting_camera_new` | camera/orbiting_camera.c | Compiled |  | compiled into game.wasm |
| `orbiting_camera_update` | camera/orbiting_camera.c | Compiled |  | compiled into game.wasm |
| `static_camera_new` | camera/static_camera.c | Compiled |  | compiled into game.wasm |
| `static_camera_update` | camera/static_camera.c | Compiled |  | compiled into game.wasm |

## `cseries`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `cseries_initialize` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `cseries_dispose` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `string_to_tag` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `tag_to_string` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `strnlen` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `strnupr` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `strnlwr` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `strupr` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `strlwr` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csprintf` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `display_assert` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csmemcmp` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csmemmove` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csmemset` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrcat` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrcmp` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrncat` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrncmp` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrncpy` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrtok` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrlen` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrcpy` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csmemcpy` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `csstrcasecmp` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `stristr` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `string_hash` | cseries/cseries.c | Compiled |  | compiled into game.wasm |
| `display_debug_string` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_exit` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_unique_identifier_get` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_unique_identifiers_equal` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_milliseconds` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_seconds` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_get_user_name` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_calloc` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_malloc` | cseries/cseries_windows.c | Compiled | runtime/src/memory.rs:3 | compiled into game.wasm |
| `system_free` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_realloc` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_get_used_memory_size` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_memory_information_get` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_show_wait_cursor` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_alert` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `system_kill_screen_saver` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `exception_code_get_string` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `generic_exception_filter` | cseries/cseries_windows.c | Compiled |  | compiled into game.wasm |
| `debug_memory_manager_initialize` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_check_memory_globals` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_memory_header_checksum` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `compare_file_pointer_totals` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_check_pointer_header` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `check_memory_status` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `local_random` | cseries/debug_memory.c | Ported |  | `tools/engine_particle_reference.py` |
| `debug_check_pointer_overrun` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_memory_fill_with_random` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_memory_add_pointer` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_memory_remove_pointer` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_check_memory` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_dump_memory_for_file` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_dump_memory_by_file` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_malloc` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_free` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_realloc` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `debug_dump_memory` | cseries/debug_memory.c | Compiled |  | compiled into game.wasm |
| `errors_dispose` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `errors_output_to_debug_file` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `errors_overflow_suppression_enable` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `error_get` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `write_to_error_file` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `write_to_debug_file` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `errors_initialize` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `error` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `errors_handle` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `errors_clear` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `profile_initialize` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_internal_step` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_rasterizer_stats` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_timesection_inherit` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_seconds_elapsed` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_lapsed_frames` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_lapsed_msec` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `find_profile_section` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_enter_private` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_exit_private` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_describe_frame` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_timesection_begin` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_timesection_end` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `compare_profile_sections` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_dump` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_dump_to_file` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_dump_frame` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_dump_frame_stop` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `string_has_prefix` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_sections_activation` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_sections_activate` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_sections_deactivate` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_find_frame_value` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_find_game_value` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_frame_get_value` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_frame_iterator_new` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_frame_iterator_next` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_frame_get_messages` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_frame_get_stalls` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_rasterizer_stalls` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_timesection_begin_now` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_timesection_end_now` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_tick_start` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_tick_end` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_render_start` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_render_end` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_render_window_start` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_render_window_end` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_texture_start` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_texture_end` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_frame_start` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_frame_end` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_idle_start` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `profile_idle_end` | cseries/profile.c | Compiled |  | compiled into game.wasm |
| `progress_new` | cseries/progress.c | Compiled |  | compiled into game.wasm |
| `progress_update` | cseries/progress.c | Compiled |  | compiled into game.wasm |
| `shortsort_2byte` | cseries/sort.c | Compiled |  | compiled into game.wasm |
| `shortsort_4byte` | cseries/sort.c | Compiled |  | compiled into game.wasm |
| `qsort_2byte` | cseries/sort.c | Compiled |  | compiled into game.wasm |
| `qsort_4byte` | cseries/sort.c | Compiled |  | compiled into game.wasm |
| `stack_walk_global_function_offset` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `stack_walk_disregard_symbol_names` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `symbol_sort_proc` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `free_symbol_table` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `symbol_name_from_address` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `base_address_from_symbol_name` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `is_valid_ebp` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `walk_up` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `initialize_stack_walk` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `walk_stack_context` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `walk_stack` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `stack_walk_dispose` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `stack_walk_with_context` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `load_symbol_table` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `stack_walk_initialize` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `stack_walk` | cseries/stack_walk_windows.c | Compiled |  | compiled into game.wasm |
| `reset_error_state` | cseries/errors.c | Compiled |  | compiled into game.wasm |
| `profile_append` | cseries/profile.c | Compiled |  | compiled into game.wasm |

## `cutscene`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `cinematic_initialize` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_dispose` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_initialize_for_new_map` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_dispose_from_old_map` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_start` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_can_be_skipped` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_skip_start` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_skip_stop` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_show_letterbox` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `draw_quad` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_force_title` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_suppress_bsp_object_creation` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_stop` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_in_progress` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_set_title_delayed` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_render` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `cinematic_set_title` | cutscene/cinematics.c | Compiled |  | compiled into game.wasm |
| `byte_swap_recording` | cutscene/recorded_animation_definitions.c | Compiled |  | compiled into game.wasm |
| `scenario_get_animation_by_name` | cutscene/recorded_animation_definitions.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_byteswap_unit_control` | cutscene/recorded_animation_initialize.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_initialize_unit_control` | cutscene/recorded_animation_initialize.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_write_unit_control` | cutscene/recorded_animation_initialize.c | Compiled |  | compiled into game.wasm |
| `apply_animation_state` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `apply_aiming_speed` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `apply_control_flags` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `apply_weapon_index` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `apply_throttle` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `update_controller_char` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `update_controller_short` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `uncompress_vector_from_controller` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `apply_vector_char_difference` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `apply_vector_short_difference` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_initialize_event_stream` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_initialize_event_stream_with_size` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_apply_event_stream` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `byte_swap_recording_stream` | cutscene/recorded_animation_playback.c | Compiled |  | compiled into game.wasm |
| `apply_facing_vector` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `apply_aiming_vector` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `apply_looking_vector` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `apply_angle_vector` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `apply_multi_vector` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `code_00082c90` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `code_00082d50` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `code_00082e10` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `code_00082ed0` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `code_00082fe0` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_initialize_event_stream_v1` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_apply_event_stream_v1` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `byte_swap_recording_stream_v1` | cutscene/recorded_animation_playback_v1.c | Compiled |  | compiled into game.wasm |
| `recorded_animations_initialize` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animations_dispose` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animations_dispose_from_old_map` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animations_clear_debug_storage` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animations_update` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_verify` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_controlling_unit` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `get_controlling_thread` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `render_debug_recording` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animations_initialize_for_new_map` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_kill` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_play_internal` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_get_time_left` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_play` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_play_and_delete` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_play_and_hover` | cutscene/recorded_animations.c | Compiled |  | compiled into game.wasm |

## `devices`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `controls_initialize` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `controls_dispose` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `controls_initialize_for_new_map` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `controls_dispose_from_old_map` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `control_place` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `control_new` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `control_delete` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `control_update` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `control_toggle` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `control_touched` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `control_destroyed` | devices/device_controls.c | Compiled |  | compiled into game.wasm |
| `light_fixtures_initialize` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `light_fixtures_dispose` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `light_fixtures_initialize_for_new_map` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `light_fixtures_dispose_from_old_map` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `light_fixture_place` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `light_fixture_new` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `light_fixture_delete` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `light_fixture_update` | devices/device_light_fixtures.c | Compiled |  | compiled into game.wasm |
| `machines_initialize` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machines_dispose` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machines_initialize_for_new_map` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machines_dispose_from_old_map` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machine_place` | devices/device_machines.c | Ported |  | `tools/engine_devices_reference.py` |
| `machine_new` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machine_delete` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machine_bumped` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machine_try_to_open_with_damage` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `machine_update` | devices/device_machines.c | Compiled |  | compiled into game.wasm |
| `devices_initialize` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `devices_dispose` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `devices_dispose_from_old_map` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_new` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_export_function_values` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_preprocess_node_orientations` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_get_position` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_get_power` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_set_never_appears_locked` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_group_set_actual_value` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_one_sided_set` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_operates_automatically_set` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_group_change_only_once_more_set` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_group_get_value` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_touched` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_can_change_position` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_effect_new` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_group_new` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_group_delete` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `create_initial_device_groups` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `devices_initialize_for_new_map` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_delete` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_update` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_render_debug` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_group_set_desired_value` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_set_actual_position` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_add_scenario_information` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_frontfacing` | devices/devices.c | Compiled |  | compiled into game.wasm |
| `device_set_desired_position` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_set_power` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |
| `device_group_change` | devices/devices.c | Ported |  | `tools/engine_devices_reference.py` |

## `dialogs`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `edit_text_fix_selection` | dialogs/edit_text.c | Compiled |  | compiled into game.wasm |
| `edit_text_selection_reset` | dialogs/edit_text.c | Compiled |  | compiled into game.wasm |
| `edit_text_get_selection_indices` | dialogs/edit_text.c | Compiled |  | compiled into game.wasm |
| `edit_text_new` | dialogs/edit_text.c | Compiled |  | compiled into game.wasm |
| `edit_text_handle_key` | dialogs/edit_text.c | Compiled |  | compiled into game.wasm |

## `editor`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `editor_render` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `game_in_editor` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `editor_preprocess_rendered_object` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `editor_should_exit` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `editor_initialize` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `editor_dispose` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `editor_update` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `editor_initialize_for_new_map` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |
| `editor_dispose_from_old_map` | editor/editor_stubs.c | Compiled |  | compiled into game.wasm |

## `effects`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `contrail_scale_value` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrails_initialize_for_new_map` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrails_dispose_from_old_map` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrails_dispose` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_delete` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrails_disconnect_from_structure_bsp` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrails_reconnect_to_structure_bsp` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_compute_new_point_count` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_verify` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `local_random_range` | effects/contrails.c | Ported |  | `tools/engine_particle_reference.py` |
| `local_random_vector_in_cone3d` | effects/contrails.c | Ported |  | `tools/engine_random_reference.py` |
| `contrail_scale_random_value` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrails_initialize` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_next_frame` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_add_points` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_update_points` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_new` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrail_owner_collision` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `contrails_update` | effects/contrails.c | Compiled |  | compiled into game.wasm |
| `decal_check` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_set_first_decal_index` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_sprite_get_bounds` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_initialize` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_initialize_for_new_map` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_dispose_from_old_map` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_dispose` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_unlock` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_get_first_decal_index` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_new_from_media_collision` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `render_debug_decals` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `projection_from_vector3d` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `projection_sign_from_vector3d` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `project_point2d` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `triple_product3d` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `plane2d_from_points` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `plane3d_from_point_and_normal` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `plane3d_negate` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `plane3d_distance_to_point` | effects/decals.c | Ported |  | `tools/engine_math_reference.py` |
| `real_a_rgb_color_to_pixel32` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `bsp3d_get_plane_from_designator` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_update` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_reinsert` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_insert` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_reconnect_to_structure_bsp` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_disconnect_from_structure_bsp` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_update` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decals_delete_permanent_from_cluster` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_delete` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_projection_create` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_clip_to_surface` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_new_from_collision` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `decal_new` | effects/decals.c | Compiled |  | compiled into game.wasm |
| `effects_initialize` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_initialize_for_new_map` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_dispose_from_old_map` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_dispose` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_object_is_corpse` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_delete` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_stop_on_first_person_weapon` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_information_get` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_disconnect_from_structure_bsp` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_scale` | effects/effects.c | Compiled | content/src/weapon.rs:602 | compiled into game.wasm |
| `effect_get_random_seed` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_real_random_range` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_random_angular_velocity` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_allowed_by_environment` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_part_allowed_by_disposition` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_set_event` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_build_location` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_location_get_next_instance` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_get_node_matrix` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_evaluate_function_integral` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `real_local_random` | effects/effects.c | Ported |  | `tools/engine_random_reference.py` |
| `local_random_direction3d` | effects/effects.c | Ported |  | `tools/engine_random_reference.py` |
| `effect_stop` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_reconnect_to_structure_bsp` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `dangerous_effects_near_player` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_random_translational_velocity` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_allocate` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `impulse_effect_initialize` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_build_locations` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_generate_particles` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_generate_part` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_start_on_first_person_weapon` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_marker_list_get_marker` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_generate_parts` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_marker_list_get_markers_by_name` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_update` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_new_looping` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_new_from_object` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_new_attached_from_markers` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effect_new_unattached_from_markers` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `effects_update` | effects/effects.c | Compiled |  | compiled into game.wasm |
| `material_effect_visible` | effects/material_effects.c | Compiled |  | compiled into game.wasm |
| `material_effect_new` | effects/material_effects.c | Compiled |  | compiled into game.wasm |
| `material_effect_new_from_point` | effects/material_effects.c | Compiled |  | compiled into game.wasm |
| `particle_systems_initialize` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_systems_initialize_for_new_map` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_orphan` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_delete` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_systems_dispose` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_systems_disconnect_from_structure_bsp` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_systems_reconnect_to_structure_bsp` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_next_type_state_index` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_next_particle_state_index` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_update_default` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_new_particle_default` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_update_particle_default` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_update_explosion` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_systems_dispose_from_old_map` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_new_particles` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `randomize_particle_variables` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_update` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_render` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_new_particle_explosion` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_new_particle_jet` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_initialize` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_systems_update` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_systems_render` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_new_unattached` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particle_system_new_attached` | effects/particle_systems.c | Compiled |  | compiled into game.wasm |
| `particles_initialize` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particles_initialize_for_new_map` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particles_dispose_from_old_map` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particles_dispose` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particle_delete` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particles_stop_on_first_person_weapon` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particles_disconnect_from_structure_bsp` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particles_reconnect_to_structure_bsp` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `new_particle_is_visible` | effects/particles.c | Ported |  | `tools/engine_particle_reference.py` |
| `particle_get_radius` | effects/particles.c | Ported | content/src/particle.rs:155 | `tools/engine_particle_reference.py` |
| `valid_real_point3d` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `valid_real_argb_color` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particle_effect_new` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particle_die` | effects/particles.c | Ported |  | `tools/engine_particle_reference.py` |
| `particle_next_sequence` | effects/particles.c | Ported |  | `tools/engine_particle_reference.py` |
| `particle_next_frame` | effects/particles.c | Ported |  | `tools/engine_particle_reference.py` |
| `particle_update_frame_time` | effects/particles.c | Ported |  | `tools/engine_particle_reference.py` |
| `particle_update_physics` | effects/particles.c | Compiled |  | compiled into game.wasm |
| `particle_new` | effects/particles.c | Ported |  | `tools/engine_particle_reference.py` |
| `particles_update` | effects/particles.c | Ported |  | `tools/engine_particle_reference.py` |
| `player_effect_get` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_initialize` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_dispose` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_initialize_for_new_map` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_dispose_from_old_map` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_add_continuous_effect` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_set_rotation` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_set_rumble` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_telefrag_effect_stop` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_screen_fade_in` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_screen_fade_out` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_get_damage_indicators` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_clear_damage_indicators` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `effect_scale_factor` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_update_screen_flash` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_update_camera_shake` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `effect_scale_value` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_update` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_continuous_refresh` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_set_translation` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_start` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_stop` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_screen_flash` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_telefrag_effect_start` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_get_screen_flash` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `get_shake_matrix` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_get_camera_effect_matrix` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_update_camera_impulse` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `player_effect_start` | effects/player_effects.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_get` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_get_type` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_systems_initialize` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_systems_initialize_for_new_map` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_systems_dispose_from_old_map` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_systems_dispose` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_type_delete_particle` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_wrap_point` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_new` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_delete` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_new_particle` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_box_offset_from_point3d` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_update_physics` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_build_clipping_planes` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_transform_clip_planes_to_box` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_polyhedra_find` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_update_particle_count` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_update` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_system_render` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |
| `weather_particle_systems_render` | effects/weather_particle_systems.c | Compiled |  | compiled into game.wasm |

## `game`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `compute_attenuation` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `compute_composite_attenuation` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `unit_get_aim_assist_parameters` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `compare_targets` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `reciprocal_square_root` | game/aim_assist.c | Ported |  | `tools/engine_animation_reference.py` |
| `limit3d` | game/aim_assist.c | Ported |  | `tools/engine_math_reference.py` |
| `set_real_euler_angles2d` | game/aim_assist.c | Ported |  | `tools/engine_math_reference.py` |
| `aim_assist_clear_line_of_sight` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `object_compute_autoaim_target` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `aim_assist_compute_target` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `autoaim_compute_target` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `find_aim_assist_targets_recursive` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `find_aim_assist_targets` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `aim_assist` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `player_aim_projectile` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `local_player_aim_assist` | game/aim_assist.c | Compiled |  | compiled into game.wasm |
| `cheats_initialize` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheats_dispose` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheats_dispose_from_old_map` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheats_update` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheats_load` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_active_camouflage_local_player` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_player_index` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheats_initialize_for_new_map` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_teleport_to_camera` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_active_camouflage` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_objects` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_all_weapons` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_all_powerups` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `cheat_all_vehicles` | game/cheats.c | Compiled |  | compiled into game.wasm |
| `game_initialize` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_dispose` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_precache_new_map` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_map_loading_in_progress` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_unload` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_dispose_from_old_map` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_frame` | game/game.c | Compiled |  | compiled into game.wasm |
| `remove_quitting_players_from_game` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_tick` | game/game.c | Ported | runtime/src/benchmark.rs:6 | `tools/engine_game_tick_reference.py` |
| `game_options_new` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_options_verify` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_initial_pulse` | game/game.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_set_players_are_double_speed` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_players_are_double_speed` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_level_set` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_level_get` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_level_get_ignore_easy` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_set_game_variant` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_set_game_engine_index` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_all_quiet` | game/game.c | Ported |  | `tools/engine_safety_reference.py` |
| `game_safe_to_save` | game/game.c | Ported |  | `tools/engine_safety_reference.py` |
| `game_safe_to_speak` | game/game.c | Ported |  | `tools/engine_safety_reference.py` |
| `game_is_cooperative` | game/game.c | Ported |  | `tools/engine_safety_reference.py` |
| `set_random_seed` | game/game.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `game_load` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_initialize_for_new_map` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_set_game_variant_from_name` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_initialize` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_dispose` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_initialize_for_new_map` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_dispose_from_old_map` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_team_is_enemy` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_team_is_ally` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_team_ally_status_changed` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_get_incidents` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_provoke` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_notify_change` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_broken` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_update` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_create` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_remove` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_incident` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `game_globals_get_weapon` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_team_score` | game/game_engine.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `linear_to_non_linear_alpha` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_dispose` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `initialize_player_multiplayer_data` | game/game_engine.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_engine_build_lighting` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_dispose_from_old_map` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_game_ending` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_game_starting` | game/game_engine.c | Ported |  | `tools/engine_announcer_reference.py` |
| `players_in_game` | game/game_engine.c | Ported |  | `tools/engine_game_end_reference.py` |
| `game_engine_statistics_append` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_handle_client_message` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_handle_server_message` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `sort_statistic_buffer` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `sort_statistic_buffer_ranking` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `is_place_tied` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `place_get_position` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `get_place_string` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `drawline` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `get_selected_color` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `get_postgame_hilite_colors` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `find_closest_player_callback` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_post_rasterize_objects` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `can_delete_item` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_update_purge` | game/game_engine.c | Ported |  | `tools/engine_item_spawn_reference.py` |
| `update_weapon_inventory` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_update_weapons` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `item_collection_get_total` | game/game_engine.c | Ported |  | `tools/engine_item_spawn_reference.py` |
| `game_engine_load_stage` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_playlist_begin` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_current_stage` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_end_game` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_allow_pick_up` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_damaged_player` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_is_odd_man_out` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_is_out_of_lives` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_should_spawn_player` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_get_team_index` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_prespawn_player_update` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_running` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_can_score` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_force_single_screen` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `get_blink_alpha` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_press_start_to_begin` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `nearby_vehicle` | game/game_engine.c | Ported |  | `tools/engine_spawn_reference.py` |
| `game_engine_rasterize_message` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_picking_up` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `goal_matches_player` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_get_custom_motion_sensor_positions` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `get_flag_definition_index` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `get_ball_definition_index` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_switch_to_postgame` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_variant` | game/game_engine.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_engine_get_goal_in_use` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_goal_position` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_set_goal_position` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_clear_goal_position` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_render_nav_points` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_infinite_grenades_internal` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_infinite_grenades` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_has_teams` | game/game_engine.c | Ported |  | `tools/engine_announcer_reference.py` |
| `game_engine_display_team_indicators` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_has_shield` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `weapon_definition_index_to_list_index` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `list_index_to_weapon_definition_index` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_remap_vehicle` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_remap_weapon` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_man_out` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_state_message` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_get_change_color` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_has_flag` | game/game_engine.c | Ported |  | `tools/engine_ctf_reference.py` |
| `game_engine_player_depower_active_camo` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `get_place_name` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_place` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_hud_draw_motion_sensor` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_test_flag` | game/game_engine.c | Ported |  | `tools/engine_spawn_reference.py` |
| `game_engine_test_trait` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `netgame_flag_verify_no_team_duplicates` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `netgame_flag_verify_team_range` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_playlist_next` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_slayer` | game/game_engine.c | Ported |  | `tools/engine_game_variant_reference.py` |
| `build_game_variant_slayer_pro` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_elimination` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_phantoms` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_endurance` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_rockets` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_snipers` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_team_slayer` | game/game_engine.c | Ported |  | `tools/engine_game_variant_reference.py` |
| `build_game_variant_oddball` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_team_oddball` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_reverse_tag` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_accumulation` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_juggernaut` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_stalker` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_king` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_king_pro` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_crazy_king` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_team_king` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_ctf` | game/game_engine.c | Ported |  | `tools/engine_game_variant_reference.py` |
| `build_game_variant_ctf_pro` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_invasion` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_iron_ctf` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_race` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_rally` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_team_race` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `build_game_variant_team_rally` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_override_map_name` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_override_game_variant` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `rasterize_in_game_score_draw_line` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_hud_draw_messages` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_has_stealth_weapon` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_weapon_fired` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `test_any_gamepad_button` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `ticks_to_unicode_time_string` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_flag_reset` | game/game_engine.c | Ported |  | `tools/engine_ctf_reference.py` |
| `game_engine_variant_cleanup` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_allow_pause` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_predict_resources` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_draw_object_in_motion_sensor` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_allow_dynamic_lighting` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_allow_integrated_lights` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_force_autopickup` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_initialize` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `multiple_teams_alive` | game/game_engine.c | Ported |  | `tools/engine_game_end_reference.py` |
| `team_has_players` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_should_end_game` | game/game_engine.c | Ported |  | `tools/engine_game_end_reference.py` |
| `adjust_score_for_ranking` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `populate_statistic_buffer` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_player_place` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `postgame_statistic_get_rating` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `select_players_to_display` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `find_closest_player_index` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `internal_rasterize_target_name` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `internal_rasterize_score` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `random_item` | game/game_engine.c | Ported |  | `tools/engine_item_spawn_reference.py` |
| `game_engine_get_type` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `match_game_type` | game/game_engine.c | Ported | content/src/level.rs:464 | `tools/engine_spawn_reference.py` |
| `game_engine_update_item_spawn` | game/game_engine.c | Ported |  | `tools/engine_item_spawn_reference.py` |
| `game_engine_update_player_no_shield` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_update_player_always_invis` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_update_non_deterministic` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `multiplayer_message_internal` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `multiplayer_message` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_show_score_one_player` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_show_score_team` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_show_score_you_ally_enemy` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_show_score_extended` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_show_score` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `find_netgame_flags` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `find_netgame_flag` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `handle_custom_starting_equipment` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_postspawn_player_update` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_damage_multiplier` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_update_teleporter` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_distance_rating_for_spawn` | game/game_engine.c | Ported |  | `tools/engine_spawn_reference.py` |
| `game_engine_get_friendly_bonus` | game/game_engine.c | Ported |  | `tools/engine_spawn_reference.py` |
| `default_starting_location_rate_function` | game/game_engine.c | Ported |  | `tools/engine_spawn_reference.py` |
| `game_engine_get_starting_location_rating` | game/game_engine.c | Ported |  | `tools/engine_spawn_reference.py` |
| `game_engine_get_variant_by_name` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_remap_equipment` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_remap_object_definition` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_get_state_message` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_did_player_win_default` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_did_player_win` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_did_team_win` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `netgame_flag_verify_team_exists` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `netgame_verify_spawn_points` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `netgame_verify_equipment` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_verify_current_map` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_playlist_initialize` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_initialize_for_new_map` | game/game_engine.c | Ported |  | `tools/engine_announcer_reference.py` |
| `game_engine_player_added` | game/game_engine.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_engine_generate_title_string` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_post_rasterize_post_game` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_update` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_player_killed` | game/game_engine.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_engine_nonplayer_post_rasterize` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_rasterize_in_game_score` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_post_rasterize_in_game` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_post_rasterize` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `ctf_create_flag_object` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_dispose` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `create_the_flag` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_single_flag_what_is_up_message` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_dispose_from_old_map` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_player_added` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_game_ending` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_game_starting` | game/game_engine_ctf.c | Ported |  | `tools/engine_announcer_reference.py` |
| `ctf_engine_statistics_append` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_handle_client_message` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_handle_server_message` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_pregame_post_rasterize` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_post_rasterize` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_award_capture` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_flag_failure_sound` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_find_flag_carrier` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_engine_allow_pick_up` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_engine_player_damaged_player` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_player_killed_player` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_display_score` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_prespawn_player_update` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_state_message_update_warning` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_sound_update_warning` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_set_flag_warning` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_weapon_drop` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_get_score` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_test_flag` | game/game_engine_ctf.c | Ported |  | `tools/engine_spawn_reference.py` |
| `ctf_get_score_string` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_get_score_header_string` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_get_team_score_string` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_engine_initialize_for_new_map` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_reset_flag` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_player_drop_flag` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_position_near_flag` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_engine_player_update` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_engine_weapon_update` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_engine_update` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_weapon_pickup` | game/game_engine_ctf.c | Ported |  | `tools/engine_ctf_reference.py` |
| `ctf_engine_starting_location_rating` | game/game_engine_ctf.c | Ported |  | `tools/engine_spawn_reference.py` |
| `king_engine_dispose` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `point3d_to_point2d` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `find_hill` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_dispose_from_old_map` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_player_added` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_game_ending` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_game_starting` | game/game_engine_king.c | Ported |  | `tools/engine_announcer_reference.py` |
| `king_engine_statistics_append` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_handle_client_message` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_handle_server_message` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_pregame_post_rasterize` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `player_inside_hill` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_player_update` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_calculate_hill_state` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_player_damaged_player` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_player_killed_player` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_display_score` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_prespawn_player_update` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_get_score` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `render_dynamic_quad_initialize` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `render_dynamic_quad` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_get_score_string` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_get_score_header_string` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_get_team_score_string` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_goal_matches_player` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `find_next_hill` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_initialize_for_new_map` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_post_rasterize` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_engine_update` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `_game_engine_play_multiplayer_sound` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `push_queued_sound` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `game_engine_update_multiplayer_sound` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `get_sound_length_in_ticks` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `game_engine_play_multiplayer_sound` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `game_engine_intialize_queued_sounds` | game/game_engine_multiplayer_sounds.c | Ported |  | `tools/engine_announcer_reference.py` |
| `oddball_engine_dispose` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_dispose_from_old_map` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_player_added` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_game_ending` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_game_starting` | game/game_engine_oddball.c | Ported |  | `tools/engine_announcer_reference.py` |
| `oddball_engine_statistics_append` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_handle_client_message` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_handle_server_message` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_pregame_post_rasterize` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_post_rasterize` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_add_score` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_add_time_with_ball` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `player_ball_count` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_player_damaged_player` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `player_has_ball` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `ball_available` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_display_score` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_prespawn_player_update` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_weapon_drop` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_get_score` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_ball_transfer_by_killing` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `accumulate_score_by_time` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `terminator_scoring_rules` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_test_flag` | game/game_engine_oddball.c | Ported |  | `tools/engine_spawn_reference.py` |
| `oddball_test_trait` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_get_score_string` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_get_score_header_string` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_get_team_score_string` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `find_position_for_ball` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `create_the_ball` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_initialize_for_new_map` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `reset_ball` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `update_ball_ownership` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_player_update` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_weapon_update` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_update` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_engine_player_killed_player` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `oddball_weapon_pickup` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `race_engine_dispose` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `delete_race_vehicles` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_get_vehicle_to_spawn` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_flags_make_unique` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_dispose_from_old_map` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_player_added` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_game_ending` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_game_starting` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_statistics_append` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_handle_client_message` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_handle_server_message` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_pregame_post_rasterize` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_post_rasterize` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_complete_lap` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `can_touch_team` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_weapon_update` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_team_can_win_game` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `build_player_speeds` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_player_damaged_player` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_player_killed_player` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_display_score` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_prespawn_player_update` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_goal_matches_player` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `count_bits_32` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_get_score` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_get_score_string` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_get_score_header_string` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_get_team_score_string` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_did_player_win` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `find_closest_vehicle` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `spawn_race_vehicles` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `new_rally_flag` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_touch_flag` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_player_update` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_update` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `race_engine_initialize_for_new_map` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `target_is_eligible` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_dispose` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_initialize_for_new_map` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_dispose_from_old_map` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_player_added` | game/game_engine_slayer.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `slayer_engine_game_ending` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_game_starting` | game/game_engine_slayer.c | Ported |  | `tools/engine_announcer_reference.py` |
| `slayer_engine_statistics_append` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_handle_client_message` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_handle_server_message` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_pregame_post_rasterize` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_post_rasterize` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_update` | game/game_engine_slayer.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `slayer_engine_allow_pick_up` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_player_damaged_player` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `update_speed_for_score` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_adjust_score` | game/game_engine_slayer.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `slayer_engine_prespawn_player_update` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_get_score` | game/game_engine_slayer.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `slayer_test_flag` | game/game_engine_slayer.c | Ported |  | `tools/engine_spawn_reference.py` |
| `slayer_get_score_string` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_get_score_header_string` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_get_team_score_string` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `find_next_target` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_engine_player_killed_player` | game/game_engine_slayer.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `slayer_engine_display_score` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `slayer_player_update` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `stub_engine_dispose` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_initialize_for_new_map` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_dispose_from_old_map` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_player_added` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_game_ending` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_game_starting` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_statistics_append` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_handle_client_message` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_handle_server_message` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_pregame_post_rasterize` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_post_rasterize` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_update` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_allow_pick_up` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_player_damaged_player` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `stub_engine_player_killed_player` | game/game_engine_stub.c | Compiled |  | compiled into game.wasm |
| `material_get_name` | game/game_globals.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_get_value_by_difficulty` | game/game_globals.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_get_value` | game/game_globals.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_get_team_value` | game/game_globals.c | Compiled |  | compiled into game.wasm |
| `game_statistics_start` | game/game_statistics.c | Compiled |  | compiled into game.wasm |
| `game_statistics_stop` | game/game_statistics.c | Compiled |  | compiled into game.wasm |
| `game_statistics_record_damage` | game/game_statistics.c | Compiled |  | compiled into game.wasm |
| `game_statistics_record_kill` | game/game_statistics.c | Ported |  | `tools/engine_game_engine_reference.py` |
| `game_time_initialized` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_initialize` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_initialize_for_new_map` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_dispose_from_old_map` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_dispose` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_end` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_get` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_get_elapsed` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `local_time_get` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `local_time_get_elapsed` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_predicting` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_in_progress` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_get_paused` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_set_paused` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_get_speed` | game/game_time.c | Ported |  | `tools/engine_game_time_reference.py` |
| `game_time_set_speed` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_statistics_new` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_statistics_frame` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_start` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_update` | game/game_time.c | Ported |  | `tools/engine_game_time_reference.py` |
| `player_control_get` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_initialize` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_dispose` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_dispose_from_old_map` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_camera_control_is_active` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `scripted_player_control_set_camera_control` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_inhibit_buttons` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `evaluate_piecewise_linear_function` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_action_clear` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_aiming_unit_index` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_target_object_index` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_field_of_view` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_unit_camera_info` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_unit_index` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_desired_weapon` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_autoaim_level` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `unit_get_local_player_index` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `players_unzoom_all` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_unzoom` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_zoom_level` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_reset` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_accept` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_back` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_action` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_jump` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_primary_trigger` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_grenade_trigger` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_zoom` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_move_relative_all_directions` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_all_directions` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_left` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_right` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_up` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_down` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_check_reset_input_blob` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `signed_angular_difference` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `limit2d` | game/player_control.c | Ported |  | `tools/engine_math_reference.py` |
| `interpolate_scalar` | game/player_control.c | Ported |  | `tools/engine_facing_reference.py` |
| `player_control_set_facing` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_new_unit` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `get_local_player_input_blob` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_facing_angles` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_get_facing_direction` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_set_desired_weapon` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_modify_desired_angles` | game/player_control.c | Ported |  | `tools/engine_facing_reference.py` |
| `player_control_initialize_for_new_map` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `handle_one_player_input` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_update` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `player_control_permanent_impulse` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `update_server_add_player` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_new` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_delete` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_start` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_add_player` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_queue` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_queue_push` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_get_maximum_actions` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_build_client_update` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `player_new_queue` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_get_update` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_get_update` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_new` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_delete` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_start` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_build_server_update` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_dequeue` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_get_maximum_possible_server_time` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_handle_client_update` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_handle_server_update` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_queues_reset_and_fill_with_lies` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_next_update` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_local_ticks` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `rumble_initialize` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_dispose` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_initialize_for_new_map` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_player_set_scripted_values` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_player_set_scale` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_player_impulse` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_player_clear` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_clear_all_now` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_player_continuous` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_dispose_from_old_map` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_calculate` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `rumble_update` | game/player_rumble.c | Compiled |  | compiled into game.wasm |
| `players_initialize` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_initialize_for_new_map` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_dispose_from_old_map` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_dispose` | game/players.c | Compiled |  | compiled into game.wasm |
| `machine_get_player_list` | game/players.c | Compiled |  | compiled into game.wasm |
| `machine_add_player` | game/players.c | Compiled |  | compiled into game.wasm |
| `local_player_exists` | game/players.c | Compiled |  | compiled into game.wasm |
| `find_unused_local_player_index` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_delete` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_get_respawn_failure` | game/players.c | Compiled |  | compiled into game.wasm |
| `local_player_get_player_index` | game/players.c | Compiled |  | compiled into game.wasm |
| `local_player_set_player_index` | game/players.c | Compiled |  | compiled into game.wasm |
| `local_player_count` | game/players.c | Compiled |  | compiled into game.wasm |
| `local_player_get_next` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_index_from_unit_index` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_died` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_are_all_dead` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_set_local_player_unit` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_get_combined_pvs_local` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_get_combined_pvs` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_input_enable` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_input_enabled` | game/players.c | Compiled |  | compiled into game.wasm |
| `any_player_is_in_the_air` | game/players.c | Compiled |  | compiled into game.wasm |
| `any_player_is_dead` | game/players.c | Compiled |  | compiled into game.wasm |
| `is_player_in_trigger` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_pseudo_kill` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_control_fix_for_loaded_game_state` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_get_starting_location_count` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_get_starting_location` | game/players.c | Compiled |  | compiled into game.wasm |
| `placement_data_set_change_color` | game/players.c | Compiled |  | compiled into game.wasm |
| `create_weapon` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_compute_combined_pvs` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_compute_local_player_count` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_examine_nearby_unit` | game/players.c | Compiled |  | compiled into game.wasm |
| `unit_should_autopick_weapon` | game/players.c | Ported |  | `tools/engine_pickup_reference.py` |
| `player_reset_action_result` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_handle_weapon_swap` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_over_shield_screen_effect` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_active_camo_screen_effect` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_health_pack_screen_effect` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_powerup_on` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_powerup_additional` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_powerup_off` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_handle_deleted_object` | game/players.c | Compiled |  | compiled into game.wasm |
| `random_direction3d` | game/players.c | Ported |  | `tools/engine_random_reference.py` |
| `valid_real_vector2d` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_new` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_add_equipment` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_aiming_vector_from_facing` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_teleport_internal` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_teleport` | game/players.c | Compiled |  | compiled into game.wasm |
| `find_best_starting_location_index` | game/players.c | Ported |  | `tools/engine_spawn_reference.py` |
| `player_spawn` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_set_action_result` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_handle_action` | game/players.c | Ported |  | `tools/engine_handle_action_reference.py` |
| `player_handle_powerup` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_handle_powerup_minor` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_update_powerups` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_debug_render` | game/players.c | Compiled |  | compiled into game.wasm |
| `debug_player_teleport` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_respawn_coop` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_teleport_on_bsp_switch` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_reconnect_to_structure_bsp` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_examine_nearby_vehicle` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_examine_nearby_device` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_handle_powerup_equipment` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_update_before_game` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_examine_nearby_item` | game/players.c | Ported |  | `tools/engine_pickup_reference.py` |
| `player_examine_nearby_objects` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_update_after_game` | game/players.c | Compiled |  | compiled into game.wasm |
| `game_set_game_variant_options` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_variant_options_get` | game/game.c | Compiled |  | compiled into game.wasm |
| `game_allegiance_set_teams` | game/game_allegiance.c | Compiled |  | compiled into game.wasm |
| `scoreboard_background_color` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `scoreboard_draw_row` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `scoreboard_team_columns` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_scoreboard_closed` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `scoreboard_menu_items` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `scoreboard_menu_first_row` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `scoreboard_rectangle` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `scoreboard_point_in` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `scoreboard_menu_pick` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_rasterize_scoreboard` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_rasterize_voice_speakers` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_variant_options_default` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_vehicle_empty` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_vehicle_home_on_pad` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_vehicle_home_follow` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_vehicle_home_current` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_update_vehicle_respawn` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_predict_multiplayer_vehicle` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_variant_vehicle_type` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_nearest_team` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_vehicle_placement_allowed` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_network_player_score` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_write_network_state` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `game_engine_read_network_state` | game/game_engine.c | Compiled |  | compiled into game.wasm |
| `ctf_show_grab` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_show_touch_return` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_show_untouched_return` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `ctf_client_flag_failure_update` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `game_engine_ctf_write_network_state` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `game_engine_ctf_read_network_state` | game/game_engine_ctf.c | Compiled |  | compiled into game.wasm |
| `game_engine_king_write_network_state` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `king_client_score_sounds` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `game_engine_king_read_network_state` | game/game_engine_king.c | Compiled |  | compiled into game.wasm |
| `oddball_client_score_sounds` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `game_engine_oddball_write_network_state` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `game_engine_oddball_read_network_state` | game/game_engine_oddball.c | Compiled |  | compiled into game.wasm |
| `game_engine_race_write_network_state` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `game_engine_race_read_network_state` | game/game_engine_race.c | Compiled |  | compiled into game.wasm |
| `game_engine_slayer_write_network_state` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `game_engine_slayer_read_network_state` | game/game_engine_slayer.c | Compiled |  | compiled into game.wasm |
| `game_time_set_distributed` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_get_tick_fraction` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `game_time_held` | game/game_time.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_note` | game/player_control.c | Compiled |  | compiled into game.wasm |
| `update_client_local_player_index` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_dequeue_distributed` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_queues_distributed_reset` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_take_local_actions` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `distributed_action_valid` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `distributed_add_new_ticks` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_handle_distributed_input` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_ticked_update_number` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_server_update_actions` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_handle_relayed_action` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `update_client_distributed_input` | game/player_queues_new.c | Compiled |  | compiled into game.wasm |
| `network_player_attach_unit` | game/players.c | Compiled |  | compiled into game.wasm |
| `network_player_show_pickup` | game/players.c | Compiled |  | compiled into game.wasm |
| `network_player_log_idle_action` | game/players.c | Compiled |  | compiled into game.wasm |
| `network_player_detach_unit` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_extras_held` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_may_spawn` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_waiting_to_start` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_danger_near` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_unit_in_structure` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_unit_grounded` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_unit_safe` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_unit_where` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_session_count` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_use_majority` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_unit_spawnable` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_near_count` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_respawn_unit` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_spawn_location` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_respawn_network_coop` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_spawnable_beside` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_checkpoint_position` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_unit_grounded_anywhere` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_unit_stranded` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_rescue_stranded` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_note_on_foot` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_room_to_spawn` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_place_beside_teammate` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_respawn_beside` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_note_checkpoint` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_respawn_at_checkpoint` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_position_switches_bsp` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_bsp_switch_ready` | game/players.c | Compiled |  | compiled into game.wasm |
| `players_coop_bsp_switch_allowed` | game/players.c | Compiled |  | compiled into game.wasm |
| `player_name_clean` | game/players.c | Compiled |  | compiled into game.wasm |

## `hs`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `hs_sv_say` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_sv_end_game` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_sv_say_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_sv_end_game_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_mods_reload_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_mods_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_mods_list_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_scenario_syntax_data_valid` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_syntax_node_refusal` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_scenario_functions_check` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_scenario_script_disabled` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_scenario_global_initializer_disabled` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_allocate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_compile_source_error` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_compile_source` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_rebuild_source_file` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_rebuild_source` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_get_function_parameters_string` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_get_function_documentation_string` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_tokens_enumerate_add_string` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_block_data` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_scenario_data` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_from_string_list` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_special_form_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_script_type_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_type_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_function_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_script_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_variable_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_ai_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_ai_command_list_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_starting_profile_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_conversation_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_object_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_trigger_volume_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_cutscene_flag_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_cutscene_camera_point_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_cutscene_title_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_cutscene_recording_names` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_navpoints` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_enumerate_hud_messages` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `alphabetize_file_references` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `alphabetize` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_saving_map_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `game_state_reverted_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_destroy_all_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_restart_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `objects_dump_memory_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `garbage_collect_now_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_pvs_clear_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_reset_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cheat_all_powerups_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cheat_all_weapons_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cheat_all_vehicles_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cheat_teleport_to_camera_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cheat_active_camouflage_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripting_magic_melee_attack_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cheats_load_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_erase_all_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_deselect_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_reconnect_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `director_save_camera_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `director_load_camera_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `players_unzoom_all_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_reset_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_reset_map_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_print_version_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_set_game_connection_to_film_playback_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `texture_cache_flush_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `sound_cache_flush_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `debug_dump_memory_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `debug_dump_memory_by_file_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `profile_initialize_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_profile_change_render_spray_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_debug_sound_point_set_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_skip_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_skip_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `attract_mode_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_won_map_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_lost_map_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_save_map_safe_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_save_cancel_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_save_map_no_timeout_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_save_map_nonsafe_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_revert_map_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_load_core_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_load_core_at_startup_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_save_core_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_restart_flashing_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `terminal_clear_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `structure_lens_flares_place_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_messages_clear_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_time_code_reset_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_flush_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_fps_accumulate_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lights_reset_for_new_map_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `enumerate_memory_units_test_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_delete_all_custom_profiles_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_ui_fast_setup_network_server_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_ui_activate_all_solo_levels_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `network_game_client_request_immediate_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_doc_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `tag_groups_dump_memory_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_players_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `game_time_get_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_time_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_level_get_ignore_easy_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `game_difficulty_level_get_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `global_structure_bsp_index_get_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_get_timer_ticks_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_list_count_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_get_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_get_time_left_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scenery_get_animation_time_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_get_custom_animation_time_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_get_grenade_count_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_status_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_going_to_vehicle_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_living_count_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_swarm_count_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_nonswarm_count_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_status_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_line_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_status_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `vehicle_scripting_load_magic_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `vehicle_scripting_unload_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_unit_riders_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_unit_driver_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_unit_gunner_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_list_from_ai_reference_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_sound_time_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_destroy_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_kill_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_cannot_take_damage_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_can_take_damage_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_objects_predict_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_definition_predict_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_pvs_set_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_pvs_activate_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_open_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_close_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_kill_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_kill_silent_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_stop_custom_animation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_exit_vehicle_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_doesnt_drop_items_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_free_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_free_units_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_detach_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_detach_units_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_place_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_kill_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_kill_silent_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_erase_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_select_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_spawn_actor_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_players_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_timer_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_timer_expire_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attack_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_defend_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_retreat_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_create_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_create_anew_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_pvs_set_camera_point_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cheat_active_camouflage_local_player_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `render_effects_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_globals_ai_active_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_globals_dialogue_triggers_enabled_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_globals_grenades_enabled_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_print_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_create_containing_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_create_anew_containing_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_destroy_containing_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_objects_delete_by_definition_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripting_set_magic_base_seat_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_set_ranged_attack_inhibited_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_set_melee_attack_inhibited_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_scripting_set_collideable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_can_blink_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_aim_without_turning_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_set_enterable_by_player_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_impervious_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_suspended_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `units_set_desired_flashlight_state_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_set_desired_flashlight_state_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_set_never_appears_locked_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_one_sided_set_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_operates_automatically_set_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_respawn_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_deaf_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_blind_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_damage_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `objects_scripting_detach_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attach_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attach_units_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_attach_free_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_encounter_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_magically_see_units_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_teleport_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_set_facing_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_effect_new_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_damage_new_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_set_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_set_emotion_animation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_set_seat_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_group_change_only_once_more_set_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_set_emotion_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_enter_vehicle_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_teleport_players_not_in_trigger_volume_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_set_shield_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_set_permutation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_effect_new_from_object_marker_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_objects_can_see_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_objects_can_see_flag_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_sound_set_gain_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `objects_scripting_set_scale_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `objects_scripting_attach_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `object_beautify_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scenery_animation_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scenery_animation_start_at_frame_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_set_maximum_vitality_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `units_scripting_set_maximum_vitality_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_set_current_vitality_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `units_scripting_set_current_vitality_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_set_power_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_set_desired_position_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_set_actual_position_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_group_set_desired_value_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_group_set_actual_value_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_distance_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_distance_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_effect_screen_fade_in_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_effect_screen_fade_out_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_set_title_delayed_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_sound_new_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_set_scale_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `debug_sound_classes_set_distances_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `debug_sound_classes_set_wet_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `sound_class_set_gain_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_unit_activate_nav_point_with_flag_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_unit_activate_nav_point_with_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_activate_team_nav_point_with_flag_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_activate_team_nav_point_with_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_set_translation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_set_rotation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_set_rumble_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_model_ambient_reflection_tint_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_script_screen_effect_set_value_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_convolution_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_filter_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_filter_desaturation_tint_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_video_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_object_list_get_element_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_sound_get_gain_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_get_health_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_get_shield_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_get_power_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_get_position_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `device_group_get_value_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_living_fraction_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_strength_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_maneuver_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_maneuver_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `radiosity_hack_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `radiosity_hack_save_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `radiosity_hack_find_point_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_and_speak_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_migrate_by_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allegiance_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allegiance_remove_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_go_to_vehicle_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_go_to_vehicle_override_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_exit_vehicle_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_braindead_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_braindead_by_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_ignore_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_prefer_target_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_teleport_starting_location_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_teleport_starting_location_if_unsupported_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_renew_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_try_to_fight_nothing_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_try_to_fight_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_try_to_fight_player_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_by_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_advance_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_command_list_advance_by_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_force_active_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_force_active_by_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_return_state_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_current_state_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_playfight_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_encounter_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_team_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_actor_type_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_actors_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_vehicle_enterable_disable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_look_at_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_stop_looking_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_automatic_migration_target_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_disable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_players_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_unit_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_follow_target_ai_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_advance_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_link_activation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_berserk_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_set_team_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allow_charge_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allow_dormant_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `director_script_camera_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_absolute_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_animation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_first_person_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_camera_set_dead_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `game_time_set_speed_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `game_set_game_variant_from_name_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_input_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_jump_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_primary_trigger_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_grenade_trigger_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_zoom_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_action_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_accept_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_back_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_up_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_down_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_left_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_right_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_look_relative_all_directions_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player_control_action_test_move_relative_all_directions_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_set_difficulty_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_crash_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `debug_dump_memory_for_file_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `profile_dump_to_file_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `profile_sections_activate_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `profile_sections_deactivate_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `profile_graph_toggle_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `debug_pvs_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_debug_vocalize_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_debug_teleport_to_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_debug_speak_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_debug_speak_list_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_show_letterbox_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_set_title_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `cinematic_suppress_bsp_object_creation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_load_core_name_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_load_core_name_at_startup_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_save_core_name_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `main_skip_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_sound_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_foley_predict_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_set_alternate_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `debug_sound_classes_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `sound_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `vehicle_hover_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_flashing_state_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_unit_deactivate_nav_point_with_flag_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_unit_deactivate_nav_point_with_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_team_nav_point_with_flag_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_team_nav_point_with_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `errors_overflow_suppression_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_player_effect_stop_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_health_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_blink_health_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_shield_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_blink_shield_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_motion_sensor_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_blink_motion_sensor_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_crosshair_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_state_message_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_objective_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_timer_time_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_timer_warning_cutoff_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_timer_position_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_timer_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_scenario_merge` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_pause_timer_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_dispose` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_recompile` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_update` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_node_gc` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_function_get` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_find_script_by_name` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_find_tag_reference_by_index` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_global_external_get` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_global_get_type` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_global_get_name` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_find_global_by_name` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_find_function_by_name` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_time_code_show_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_time_code_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_start_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_near_clip_distance_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player0_look_invert_pitch_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player0_look_pitch_is_inverted_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `player0_joystick_set_is_normal_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ui_widget_debug_show_path_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `display_scenario_help_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `xbox_set_machine_name_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_help_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_not_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scenario_trigger_volume_test_object_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_trigger_volume_test_objects_any_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_trigger_volume_test_objects_all_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_play_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_play_and_delete_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `recorded_animation_play_and_hover_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `lights_enable_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_start_user_animation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_start_user_animation_list_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_custom_animation_at_frame_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_is_playing_custom_animation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_tokens_enumerate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_vehicle_test_seat_list_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_vehicle_test_seat_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_has_weapon_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_has_weapon_readied_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_solo_player_integrated_night_vision_is_active_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `unit_get_current_flashlight_state_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_is_attacking_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_by_name` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_conversation_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `ai_scripting_allegiance_broken_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_player_control_set_camera_control_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_scenario_postprocess` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_initialize_for_new_map` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_dispose_from_old_map` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_help` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_doc` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_hack` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_show_hud_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `scripted_show_hud_help_text_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_initialize` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_compile_and_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_parse_cond_recursive` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_get_parameter_indices` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_compile_initialize` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_compile_add_source` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_verify_source_offset` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_find_string_constant` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `character_in_list` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_variable` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_call_predicate` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_boolean` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_real` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_integer` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_string` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_script` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_tag_reference` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_enum` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_scenario_datum` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_trigger_volume` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_cutscene_flag` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_cutscene_camera_point` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_cutscene_title` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_cutscene_recording` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_device_group` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_ai` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_ai_command_list` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_starting_profile` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_conversation` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_object_name` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_object` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_navpoint` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_hud_message` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_object_list` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `string_list_find` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_concatenate_string_constant` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_add_global` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_add_script` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_tokenize_primitive` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_primitive` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_nonprimitive` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_postprocess_passthrough_arguments` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_compile_postprocess` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_concatenate_expression` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_tokenize` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_tokenize_nonprimitive` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_macro_function_parse` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_begin` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_if` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_cond` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_set` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_logical` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_arithmetic` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_equality` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_inequality` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_sleep` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_sleep_until` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_wake` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_inspect` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_object_cast_up` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_parse_debug_string` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_compile_expression` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_compile` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_compile_finish` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_compile_dispose` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_not` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_print` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_players` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_trigger_volume_test_objects` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_unit_can_see_object` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_objects_can_see_object` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_unit_can_see_flag` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_objects_can_see_flag` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `object_is_or_contains_player` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_create` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_destroy` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_destroy_by_name` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_destroy_all` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_iterate_names_containing` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_create_containing` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_destroy_containing` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_list_get_element` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_set_shield` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_set_permutation` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_objects_predict` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_objects_delete_by_definition` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_effect_new` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_effect_new_from_object_marker` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_damage_new` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_damage_object` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_sound_get_gain_reference` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_sound_get_gain` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_sound_set_gain` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_trigger_volume_test_objects_all` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_trigger_volume_test_objects_any` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_create_anew` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_create_anew_containing` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_orient` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_teleport` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_object_set_facing` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_teleport_players_not_in_trigger_volume` | hs/hs_library_external.c | Compiled |  | compiled into game.wasm |
| `hs_syntax_nth` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_inspect_boolean` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_inspect_real` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_inspect_short_integer` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_inspect_long_integer` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_inspect_string` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_inspect_enum` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_initialize` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_dispose_from_old_map` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_dispose` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `expression_get_function_name` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_thread_new` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_thread_delete` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_thread_format` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_stack_push` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_stack_pop` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_stack_allocate` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_wake` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_find_thread_by_script` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_find_thread_by_name` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_long_to_boolean` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_short_to_boolean` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_string_to_boolean` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_data_to_void` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_short_to_real` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_long_to_real` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_enum_to_real` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_real_to_short` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_real_to_long` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_long_to_short` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_object_name_to_object_list` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_object_to_object_list` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_object_type_can_cast` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_can_cast` | hs/hs_runtime.c | Ported | import/src/script_tables.rs:881 | `tools/engine_script_cast_reference.py` |
| `hs_cast` | hs/hs_runtime.c | Ported |  | `tools/engine_script_cast_reference.py` |
| `hs_global_reconcile_read` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_global_reconcile_write` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `script_error` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_get_executing_thread_name` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_wake_by_name` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `render_debug_scripting` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `render_debug_trigger_volumes` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_return` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_global_evaluate` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_wake` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_script_evaluate` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_arguments_evaluate` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_macro_function_evaluate` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_begin` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_begin_random` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_if` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_set` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_logical` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_arithmetic` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_equality` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_inequality` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_sleep` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_sleep_until` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_inspect` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_object_cast_up` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_evaluate_debug_string` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_thread_main` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_initialize_for_new_map` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_update` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_evaluate` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_syntax_data_byte_swap` | hs/hs_scenario_definitions.c | Compiled |  | compiled into game.wasm |
| `reference_list_new` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `reference_list_get_next_datum_index` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `reference_list_add` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `reference_list_delete` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_lists_initialize` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_lists_dispose` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_lists_initialize_for_new_map` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_lists_dispose_from_old_map` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_new` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_delete` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_add` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_get_next` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_add_reference` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_remove_reference` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_gc` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_count` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `object_list_get_first` | hs/object_lists.c | Compiled |  | compiled into game.wasm |
| `hs_recompile_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `random_range_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `real_random_range_evaluate` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `hs_compile_and_evaluate_command` | hs/hs.c | Compiled |  | compiled into game.wasm |
| `skip_whitespace` | hs/hs_compile.c | Compiled |  | compiled into game.wasm |
| `hs_runtime_waiting_on_call` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |
| `hs_type_default_value` | hs/hs_runtime.c | Compiled |  | compiled into game.wasm |

## `input`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `input_abstraction_dispose` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_abstraction_reset_controller_detection_timer` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_abstraction_get_local_player_preferences` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_abstraction_update_local_player_preferences` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_abstraction_get_input_state` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_abstraction_update_device_changes` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `keyboard_hold_ticks` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `keyboard_controls_update` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_abstraction_initialize` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_abstraction_update` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `fix_dead_zone` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `update_ticks` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `update_hold_ticks` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_dispose` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_activate` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_deactivate` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_flush` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_key_is_down` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_get_key` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_get_mouse_state` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_suppress` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_mouse_button_is_down` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_has_gamepad` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_get_gamepad_state` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_set_gamepad_rumbler_state` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_vertical_blank_interrupt` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `acquire_input_mutex` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `release_input_mutex` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_update_analog_button_state` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_get_device_states` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_update_keyboard_devices` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_get_raw_data_string` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_update` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_frame_begin` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_frame_end` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_flush_rumble` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `input_initialize` | input/input_xbox.c | Compiled |  | compiled into game.wasm |
| `HATCleanup` | input/HaloAutoTest.c | Compiled |  | compiled into game.wasm |
| `HATReadMain` | input/HaloAutoTest.c | Compiled |  | compiled into game.wasm |
| `HATRawRead` | input/HaloAutoTest.c | Compiled |  | compiled into game.wasm |
| `HATRawLoopRead` | input/HaloAutoTest.c | Compiled |  | compiled into game.wasm |
| `HATRawWrite` | input/HaloAutoTest.c | Compiled |  | compiled into game.wasm |
| `HATInit` | input/HaloAutoTest.c | Compiled |  | compiled into game.wasm |
| `HATRun` | input/HaloAutoTest.c | Compiled |  | compiled into game.wasm |
| `local_player_is_piloting_aircraft` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `set_default_game_input_preferences` | input/input_abstraction.c | Compiled |  | compiled into game.wasm |
| `input_keyboard_thread` | input/input_xbox.c | Compiled |  | compiled into game.wasm |

## `interface`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `weapon_state_is_depleted` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_initialize` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_dispose` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_initialize_for_new_map` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_dispose_from_old_map` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `scripted_show_hud` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `scripted_show_hud_help_text` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_update` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `get_object_icon_text_index` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_show_action_response` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_get_item_string` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_picked_up_grenade` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_picked_up_ammunition` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_picked_up_weapon` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_picked_up_powerup` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `temporary_hud_draw_reticle` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_load` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_autosave` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_draw_friendly_indicator` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_draw_players` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_player_names_setting` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_draw_screen` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `get_return_eip` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `check_stack_buffer` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_multitexture_overlay_get_current_weapon_definition` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_globals_get_scale` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_retrieve_bitmap_and_bounding_rect` | interface/hud_draw.c | Ported |  | `tools/engine_hud_sprite_reference.py` |
| `hud_multitexture_overlay_blend_function` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_multitexture_overlay` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `fast_ftol_C` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `real_argb_color_to_pixel32` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `real_rgb_color_to_pixel32` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `real_alpha_intensity_to_pixel32` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_calculate_point` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `get_flash_duration` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `get_flash_color` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_multitexture_overlays_follow_zoom` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_number_shows_only_when_zoomed` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_zoomed_layout_begin` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_bitmap_direct` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_meter` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_numbers` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_bitmap` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_static_element` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_weapon_overlays` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `render_state_text` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `render_state_bitmap` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_messaging_initialize` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_messaging_initialize_for_new_map` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_messaging_dispose_from_old_map` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_messaging_dispose` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_state_message` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_flashing_state` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_restart_flashing` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_objective` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_timer_time` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_timer_warning_cutoff` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_set_timer_position` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_timer` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_pause_timer` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_get_timer_ticks` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_time_code_show` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_time_code_start` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_time_code_reset` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_render_timer` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_set_state_message` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_set_state_message_icon` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_set_state_message_text` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_enable_custom_state_message` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_set_state_text` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_messaging_get_objective` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `find_free_hud_message` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `compare_messages` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_messages_clear` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_get_font_index` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_get_text_color` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_messaging_globals_update` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_print_message` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_add_item_message` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_broadcast_team_message` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `hud_messaging_update` | interface/hud_messaging.c | Compiled |  | compiled into game.wasm |
| `find_nav_point` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `get_nav_point_datum` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_initialize_nav_points` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_initialize_nav_points_for_new_map` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_dispose_nav_points_from_old_map` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_dispose_nav_points` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_nav_point` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_nav_point_with_game_engine_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_nav_point_with_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_nav_point_with_object` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_team_nav_point` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_team_nav_point_with_game_engine_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_team_nav_point_with_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_team_nav_point_with_object` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_global_nav_point` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_activate_global_nav_point_with_game_engine_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_nav_point` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_nav_point_with_game_engine_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_nav_point_with_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_nav_point_with_object` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_team_nav_point` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_team_nav_point_with_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_deactivate_team_nav_point_with_object` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_unit_activate_nav_point_with_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_unit_activate_nav_point_with_object` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_unit_deactivate_nav_point_with_flag` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_unit_deactivate_nav_point_with_object` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_get_nav_point_render_type` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `custom_render_nav_point` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_render_nav_points` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_update_nav_point_local_player` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_update_nav_points` | interface/hud_nav_points.c | Compiled |  | compiled into game.wasm |
| `hud_play_sound` | interface/hud_sounds.c | Compiled |  | compiled into game.wasm |
| `unit_hud_outline_mapper_tick` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `unit_hud_shield_meter_mapper_tick` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `unit_hud_shield_meter_mapper_init` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `initialize_hud_state` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `get_hud_state` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_initialize_unit_interface` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_initialize_unit_interface_for_new_map` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_dispose_unit_interface_from_old_map` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_dispose_unit_interface` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_health` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_blink_health` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_shield` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_blink_shield` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_motion_sensor` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_blink_motion_sensor` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_play_unit_sounds` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_fix_unit_data` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_update_unit_local_player` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_render_damage_indicators` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_tick_shield` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_update_unit` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_render_unit_interface` | interface/hud_unit.c | Compiled |  | compiled into game.wasm |
| `hud_initialize_weapon_interface` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `hud_initialize_weapon_interface_for_new_map` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `hud_dispose_weapon_interface_from_old_map` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `hud_dispose_weapon_interface` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `scripted_hud_show_crosshair` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `get_hud_state` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `get_crosshair_state` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `play_weapon_hud_sounds` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `render_grenade_hud` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `strip_path_name` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `hud_update_weapon_local_player` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `hud_fix_weapon_data` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `crosshairs_draw` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `render_weapon_hud` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `hud_update_weapon` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `hud_render_weapon_interface` | interface/hud_weapon.c | Compiled |  | compiled into game.wasm |
| `tiny_point2d_set` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `tiny_point2d_get` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `blip_type_get` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_blip_set_type_and_size` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `blip_size_get` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `get_motion_sensor_data` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_initialize` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_dispose` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_initialize_for_new_map` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_dispose_from_old_map` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `should_track_object` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `blip_begin` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `should_draw_object` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `render_blip` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_update` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `update_motion_sensor` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `render_motion_sensor` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_draw_screen` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `motion_sensor_tick` | interface/motion_sensor.c | Compiled |  | compiled into game.wasm |
| `event_manager_initialize` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `event_manager_dispose` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `event_manager_flush` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `event_manager_suppress` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `get_next_event` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `event_manager_time_of_last_event` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `event_manager_post_button` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `event_manager_update` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `animation_update_render_only` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapons_initialize` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapons_dispose` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapons_initialize_for_new_map` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapons_dispose_from_old_map` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_set_visibility` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_predict` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_start_interpolation` | interface/first_person_weapons.c | Ported |  | `tools/engine_first_person_reference.py` |
| `first_person_weapon_set_state` | interface/first_person_weapons.c | Ported |  | `tools/engine_first_person_reference.py` |
| `first_person_weapon_animation_fits` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_build_node_matrices` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_message` | interface/first_person_weapons.c | Ported |  | `tools/engine_first_person_reference.py` |
| `first_person_weapon_switch_weapons` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_new_unit` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_next_state` | interface/first_person_weapons.c | Ported |  | `tools/engine_first_person_reference.py` |
| `first_person_weapon_draw` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_get_local_index` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_get_marker_by_name` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_center_flashlight` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_adjust_light` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_get_node_matrix` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_update` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `model_build_remapping_table_for_animation_graph` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_render_update` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_get_marker_by_name_render` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_play_first_person_weapon_sound` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_state_from_weapon_message` | interface/first_person_weapons.c | Ported |  | `tools/engine_first_person_reference.py` |
| `first_person_animation_type_from_weapon_state` | interface/first_person_weapons.c | Ported |  | `tools/engine_first_person_reference.py` |
| `model_remap_node_matrices_to_match_animation_graph` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_message_from_unit` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_message_from_weapon` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_get` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_index_from_weapon_index` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `first_person_weapons_update` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `interface_initialize` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_dispose_from_old_map` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_dispose` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_get_tag_index` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_get_real_argb_color` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_set_bitmap_text_draw_mode` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_get_weapon_hud_index` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_draw_screen` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `profile_graph_toggle` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `render_debug_profile_stall_tick` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `render_debug_profile` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_splitscreen_render` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_initialize_for_new_map` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_get_rgb_color` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_draw_fullscreen_overlays` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_draw_bitmap` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_draw_bitmap_modulated` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `interface_draw_bitmap_modulated_p32` | interface/interface.c | Compiled |  | compiled into game.wasm |
| `overhead_map_initialize` | interface/overhead_map.c | Compiled |  | compiled into game.wasm |
| `overhead_map_initialize_for_new_map` | interface/overhead_map.c | Compiled |  | compiled into game.wasm |
| `overhead_map_dispose_from_old_map` | interface/overhead_map.c | Compiled |  | compiled into game.wasm |
| `overhead_map_post_rasterize` | interface/overhead_map.c | Compiled |  | compiled into game.wasm |
| `player_ui_dispose` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_reset_single_player_local_player_controllers` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_set_single_player_local_player_controller` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_single_player_local_player_controller` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_single_player_local_player_from_controller` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_local_player_joined_multiplayer_game` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_local_player_wants_to_play_multiplayer` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_clear_multiplayer_autojoin_for_local_player` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_autojoin_players_to_next_multiplayer_game` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_clear_multiplayer_variant` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_active_player_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_active_player_profile_index` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_last_single_player_level_played` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_set_game_variant` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_game_variant_specified` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_rumble_disabled` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_autolevel_enabled` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_path_to_local_player_profile_directory` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_remember_player1_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_player1_last_used_profile_index` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_fast_setup_network_server` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_edit_profile_is_default_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_edit_profile_name_is_dirty` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_prompt_user_to_rename_edit_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_edit_player_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_edit_playlist_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_edit_profile_is_dirty` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_activate_all_solo_levels` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `hud_message_to_all` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player0_look_pitch_is_inverted` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player0_joystick_set_is_normal` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `generate_default_player_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `set_local_player_controls_from_player_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `clear_profile_edit_data` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `reset_local_player_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_initialize` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_clear_multiplayer_joins` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_set_active_player_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_begin_editing_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_save_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_end_editing_profile` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player0_look_invert_pitch` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `progress_bar_load_loading_texture` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d0ec0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d0f10` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d0f20` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d0f40` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `tgaLoadHeader` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1020` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1030` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `tgaLoadImageData` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d10e0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d10f0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `tgaLoad` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_is_stuff_ready` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1160` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1380` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1390` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_initialize` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_dispose` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_begin` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d14c0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_end` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_is_active` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `to_screen` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `generate_gravy_rect` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1660` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1670` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1680` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d16a0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_create_noise_texture` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d16e0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d16f0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `gen_cloud_coord` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `gen_mask_coord` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `draw_layer_int` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `draw_layer` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `draw_initial_layer` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `this_is_awful` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `do_convoluation_coords` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `draw_fade_layer` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_enable` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1ba0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `IDirect3DTexture8_Release` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1bc0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1bd0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `code_000d1bf0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `D3DXMatrixIdentity` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `SetRenderStateSmart` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `SetTextureStageStateSmart` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_render` | interface/progress_bar.c | Compiled | ui/src/loading.rs:211 | compiled into game.wasm |
| `code_000d23b0` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_create_mask_texture` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_eachframe` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_make_stuff_ready` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `progress_bar_display` | interface/progress_bar.c | Compiled |  | compiled into game.wasm |
| `terminal_initialize` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_dispose` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_remove_line` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_clear` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_gets_active` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_gets_begin` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_gets_end` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_update_input` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_update_output` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_draw` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_new_line` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_update` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `terminal_printf` | interface/terminal.c | Compiled |  | compiled into game.wasm |
| `display_error_text_when_main_menu_loaded` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `pool_alloc` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `event_controller_index_compatible_with_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `set_ui_plasma_effect_color` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_initialize` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_safe_to_load` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_set_fade_value` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_debug_show_path` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_count_children` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_get_nth_child` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_realloc` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_free` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_active` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_active_for_local_player` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_inhibit_processing` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `pool_free` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `draw_bitmap_in_rect` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_get_topmost_parent` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_get_child_index_from_parent` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_set_visibility_recursive` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `main_menu_active` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `main_menu_is_active` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `main_menu_screen_is_active` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_set_next_level` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_load_progress_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `filesystem_check_thread_is_active` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `pop_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_error_when_main_menu_loaded` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_error_deferred` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_error_abort_to_dashboard_deferred` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_start_main_menu_music` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_stop_main_menu_music` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_main_menu_music_active` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_disable_pause_game` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `dispose_widget_stack` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_delete_children_recursive` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_launch_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_find_by_tag_index_recursive` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_give_focus_directly` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_give_focus_by_tag` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `play_sound_tag` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_add_child` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `push_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_can_receive_events` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_set_focused_child_by_index` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_go_back_to_previous` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `event_handler_dispatch` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_load_children_recursive` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_initialize` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `remap_sticks_for_local_player` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `transition_to_game_in_progress` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `get_icon_type` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `should_flip_sticks_for_local_player` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `filesystem_initialization_thread_proc` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `perform_filesystem_initialization` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_reload_recursive` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_reload_by_tag` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_text_box_is_focused` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `string_has_icons_to_draw` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `search_and_replace` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_render_text_box` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_render_spinner_list` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `get_ui_rgb_white` | interface/ui_widget.c | Compiled | ui/src/draw.rs:16 | compiled into game.wasm |
| `get_ui_argb_white` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_debug_targets_enabled` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `modulate_pixel32_by_real_alpha` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_delete` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_close_all` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_close_all_for_local_player` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_delete_history` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_pop_stack` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `main_screen_shell_begin_fade` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_play_audio_feedback_sound` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_debug_draw_outline` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_debug_draw_mark` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `draw_string_and_hack_in_icons` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_debug_draw_targets` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_debug_set_mark` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_press` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_child_index` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_list_shows_several` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_dispose` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_event_function_list_widget_goto_next_item` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_event_function_list_widget_goto_previous_item` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_key_button` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_widget_is_item` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_note_list_step` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `render_ui_widgets_postgame` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `render_ui_widgets` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_note_slot_band` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_note_target` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_load_by_name_or_tag` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `main_screen_shell_load` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `network_game_reset_to_pregame_ui` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_error` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_error_abort_to_dashboard` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_error_damaged_media` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_errors_deferred_until_cinematic_stop` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `display_scenario_help` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_merge_setting_rows` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_widen_values` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `process_ui_widgets` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_event_handler_function_invoke` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `widget_event_function_null` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `reset_last_player1_profile_index` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `new_campaign_decision` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `create_and_begin_editing_new_gametype_profile` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_join_game_from_server_list` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `pause_game_restart_at_checkpoint` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `pause_game_restart_level` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `pause_game_quit_to_main_menu` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `coop_game_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_type_menu_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `cancel_profile_delete` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `run_xdemos` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `single_player_reset_controller_choices` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `exit_to_xbox_dashboard` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `clear_multiplayer_player_joins` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_server_list_dispose` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_cancel` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_pick_quick_start_play_stage` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_level_list_dispose` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_end_editing` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_end_editing` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `switch_from_main_menu_to_single_player` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_server_accept_connections` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_server_defer_game_start` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_server_allow_game_start` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `disable_widget_if_no_xdemos` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `pop_history_stack_once` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `begin_music_fade_out` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `solo_level_dispose_list` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `start_new_game` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_profiles_list_dispose` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profiles_list_dispose` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_color_picker_menu_dispose` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_server_list_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `main_menu_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `delete_playlist_profile_final` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_wants_to_join_multiplayer_game` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_name` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_change_name` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_remove_local_player` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `delete_player_profile_final` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `single_player_set_player1_controller_choice` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `netgame_unjoin_player` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `close_calling_widget_if_not_editing_profile` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `new_campaign_if_no_custom_player_profiles_exist` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `difficulty_set` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `display_error_if_no_network_connection` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `split_screen_game_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `single_player_set_player2_controller_choice` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_save_changes` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `start_network_game_if_no_advertised_servers` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `delete_player_profile_request` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_color_picker_select_color` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_set_game_engine` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_swap_teams` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_start_faster` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_start_slower` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `create_and_begin_editing_new_player_profile` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_level_list_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_begin_editing` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_begin_editing` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `delete_playlist_profile_request` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_color_picker_menu_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `difficulty_menu_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_game_engine` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_save_changes` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_profiles_list_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `ui_widget_function_denied` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `new_campaign_chosen` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `network_game_start_new_server` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_name` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_item_options` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_indicator_options` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_ctf_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_racing_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_koth_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_slayer_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_set_for_game_1wide` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_initialize_controller_settings` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `solo_level_initialize_list_coop` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_change_advanced_controller_settings` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `solo_level_set_next_map_name` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `netgame_join_player` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profiles_list_initialize` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_set_for_game_3wide` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_indicator_options` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_item_options` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_initialize_advanced_controller_settings` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_change_controller_settings` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_racing_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_slayer_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_ctf_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_koth_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_oddball_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_initialize_player_options` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_oddball_rules` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_change_player_options` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_level_select` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_profile_set_for_game` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `ui_widget_game_data_function_invoke` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `widget_function_null` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `settings_menu_update_extended_description` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_settings_menu_update_extended_description` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `playlist_gametype_select_menu_update_extended_description` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_type_menu_update_extended_description` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `difficulty_select_menu_update_extended_description` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `server_list_menu_update` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `network_pregame_status_screen_update` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `splitscreen_pregame_status_screen_update` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `netgame_prejoin_players` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `set_textbox_to_build_number` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `solo_game_objective_text` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `main_menu_animation_fakery` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `get_active_player_profile_display_name` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `get_editable_player_profile_display_name` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `get_editable_playlist_profile_display_name` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `get_active_player_profile_color_index` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_edit_select_menu_update_extended_description` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `game_options_menu_update_text_desc` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `game_options_menu_update_pic_desc` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_text_box_for_map_name` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_text_box_for_game_ruleset` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_text_box_for_teams_noteams` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_text_box_for_score_limit` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_text_box_for_score_limit_type` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_bitmap_for_map` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_bitmap_for_ruleset` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_set_text_box_for_number_of_players` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_edit_profile_set_ruleset_textbox_string_index` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `system_link_status_check` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `multiplayer_game_directions` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `teams_no_teams_mp_game_bitmap_update` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `warn_if_difficulty_will_nuke_saved_game` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `dim_if_no_system_link_cable` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `spinner_list_3wide_determine_displayed_item_indices` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_update_cache_for_nwide_list` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `variant_profile_update_cache_for_nwide_list` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `mutliplayer_settings_select_list_update_displayed_items` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_3wide_list_update` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_1wide_list_update` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `solo_level_select_list_update_displayed_items` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `player_profile_color_picker_update` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `mp_level_select_list_update_displayed_items` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `list_indices_sort_proc` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `filter_invalid_list_indices` | interface/ui_widget_game_data_input_functions.c | Compiled |  | compiled into game.wasm |
| `ui_load_tags_for_scenario` | interface/ui_widget_group.c | Compiled |  | compiled into game.wasm |
| `ui_widget_search_and_replace_invoke` | interface/ui_widget_text_search_and_replace_functions.c | Compiled |  | compiled into game.wasm |
| `widget_replace_function_null` | interface/ui_widget_text_search_and_replace_functions.c | Compiled |  | compiled into game.wasm |
| `widget_controller` | interface/ui_widget_text_search_and_replace_functions.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_initialize` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_dispose` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_launch` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_active` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_last_exit_saved_text` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_tab_left` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_tab_right` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_tab_up` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_tab_down` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_cancel` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_get_character` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_get_current_character` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_render_internal` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_free_space_in_text_buffer` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_backspace` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_close` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_render` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_select` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_process_internal` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_process` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `attract_mode_should_start` | interface/attract_mode.c | Compiled |  | compiled into game.wasm |
| `attract_mode_reset_timer` | interface/attract_mode.c | Compiled |  | compiled into game.wasm |
| `attract_mode_get_localized_movie_path` | interface/attract_mode.c | Compiled |  | compiled into game.wasm |
| `attract_mode_start` | interface/attract_mode.c | Compiled |  | compiled into game.wasm |
| `game_end_credits_start` | interface/attract_mode.c | Compiled |  | compiled into game.wasm |
| `queue_event` | interface/event_manager.c | Compiled |  | compiled into game.wasm |
| `first_person_weapon_index_from_unit_index` | interface/first_person_weapons.c | Compiled |  | compiled into game.wasm |
| `hud_player_name_scale` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_player_name_in_sight` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_player_name_enemy_range` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_draw_player_name` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_draw_player_names` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `temporary_hud_draw` | interface/hud.c | Compiled |  | compiled into game.wasm |
| `hud_zoomed_layout_end` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `hud_draw_bitmap_with_meter` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `get_sprite_clip_rect` | interface/hud_draw.c | Ported |  | `tools/engine_hud_sprite_reference.py` |
| `hud_calculate_bitmap_bounds` | interface/hud_draw.c | Compiled |  | compiled into game.wasm |
| `xbox_demos_available` | interface/marketing_and_strategic_business_development.c | Compiled |  | compiled into game.wasm |
| `xbox_demos_launch` | interface/marketing_and_strategic_business_development.c | Compiled |  | compiled into game.wasm |
| `xbox_dashboard_launch` | interface/marketing_and_strategic_business_development.c | Compiled |  | compiled into game.wasm |
| `clean_up_for_image_launch` | interface/marketing_and_strategic_business_development.c | Compiled |  | compiled into game.wasm |
| `player_ui_get_game_variant_options` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `player_ui_saved_game_file_removed` | interface/player_ui.c | Compiled |  | compiled into game.wasm |
| `compute_offset_coordinate` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_can_handle_events` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_get_tail_child_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_find_by_tag_index` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_get_cumulative_alpha_modifier` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_label_text_right` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_fit_button_targets` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_target_at` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_widget_has_focus` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_give_focus` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_list_directions` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_step_list_to_slot` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_wheel_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_wheel_skips_child` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_wheel_room` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_menu` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_menus_active` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_debug_log_click` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_debug_log_keyboard_tap` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_point_redrawn` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_mouse_keyboard_point_redrawn` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widgets_process_mouse` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_render_recursive` | interface/ui_widget.c | Compiled | ui/src/draw.rs:3 | compiled into game.wasm |
| `spinner_string_list_extra_count` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_spinner_own_item_count` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_widget_spinner_extra_description` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `spinner_string_list_get_string` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_render_column_list` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_update_animation_parameters` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `spinner_list_update` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `column_list_update` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_tab_to_next_valid_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_tab_to_previous_valid_widget` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_takes_any_controller` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_takes_events_of_controller` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `widget_instance_process_one_event_recursive` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `ui_check_for_pause_game` | interface/ui_widget.c | Compiled |  | compiled into game.wasm |
| `solo_level_initialize_list_single_player` | interface/ui_widget_event_handler_functions.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_target_rectangles` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |
| `virtual_keyboard_click` | interface/virtual_keyboard.c | Compiled |  | compiled into game.wasm |

## `items`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `equipment_place` | items/equipment.c | Compiled | import/src/level.rs:1046 | compiled into game.wasm |
| `equipment_handle_pickup` | items/equipment.c | Compiled |  | compiled into game.wasm |
| `equipment_definition_handle_pickup` | items/equipment.c | Compiled |  | compiled into game.wasm |
| `garbage_update` | items/garbage.c | Compiled |  | compiled into game.wasm |
| `garbage_new` | items/garbage.c | Compiled |  | compiled into game.wasm |
| `object_get_type` | items/items.c | Compiled |  | compiled into game.wasm |
| `items_initialize` | items/items.c | Compiled |  | compiled into game.wasm |
| `items_initialize_for_new_map` | items/items.c | Compiled |  | compiled into game.wasm |
| `items_dispose_from_old_map` | items/items.c | Compiled |  | compiled into game.wasm |
| `items_dispose` | items/items.c | Compiled |  | compiled into game.wasm |
| `item_new` | items/items.c | Compiled |  | compiled into game.wasm |
| `item_delete` | items/items.c | Compiled |  | compiled into game.wasm |
| `verify_item_location` | items/items.c | Compiled |  | compiled into game.wasm |
| `dangerous_items_near_player` | items/items.c | Compiled |  | compiled into game.wasm |
| `item_in_unit_inventory` | items/items.c | Compiled |  | compiled into game.wasm |
| `item_get_position_even_if_in_inventory` | items/items.c | Compiled |  | compiled into game.wasm |
| `item_detonate` | items/items.c | Compiled |  | compiled into game.wasm |
| `item_adjust_for_angular_velocity_change` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `valid_real_vector3d_axes3` | items/items.c | Compiled |  | compiled into game.wasm |
| `valid_real_matrix4x3` | items/items.c | Compiled |  | compiled into game.wasm |
| `item_accelerate` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_align_to_normal_and_point` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_update` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `projectiles_initialize` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectiles_initialize_for_new_map` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectiles_dispose_from_old_map` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectiles_dispose` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_kill_tracer` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectiles_delete_all` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_delete` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_set_target_object_index` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_make_tracer` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_get_ballistic_acceleration` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_estimate_time_to_target` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `dangerous_projectiles_near_player` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_handle_deleted_object` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_calculate_deceleration_from_distances` | items/projectiles.c | Ported |  | `tools/engine_projectile_deceleration_reference.py` |
| `projectile_adjust_for_angular_velocity_change` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_export_function_values` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_calculate_deceleration` | items/projectiles.c | Ported |  | `tools/engine_projectile_deceleration_reference.py` |
| `projectile_handle_parent_destroyed` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `random_vector_in_cone3d` | items/projectiles.c | Ported |  | `tools/engine_random_reference.py` |
| `projectile_aim_ballistic` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_aim_linear` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_aim` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_set_action` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_effect_new` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_collision_test_line` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_detonate` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_new` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_accelerate` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_collision` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `projectile_update` | items/projectiles.c | Compiled |  | compiled into game.wasm |
| `weapon_definition_get_damage_potential` | items/weapon_definitions.c | Compiled |  | compiled into game.wasm |
| `animation_convert_frame_to_pal` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `animation_key_frame_index` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `animation_update` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `animation_choose_random_permutation` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapons_initialize` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapons_initialize_for_new_map` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapons_dispose_from_old_map` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapons_dispose` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_place` | items/weapons.c | Compiled | import/src/level.rs:1046 | compiled into game.wasm |
| `weapon_preprocess_node_orientations` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_get_label` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_set_integrated_light_power` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_estimate_time_to_target` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_can_be_fired` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_useful` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_compute_movement_penalty` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_melee_attack` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_must_be_readied` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_is_flag` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_prevents_grenade_throwing` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_get_first_person_animation_time` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_overcharged` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_precache_projectile_trails` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_finish_reload` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_start_chamber` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_finish_chamber` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_fully_charged` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_idle` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_locked` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_recover` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_finish_tracking` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_start_reload` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_state_next` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_get` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_get` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_count` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_get_charged_fraction` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_busy` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_state_change_ok` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_get_effect_object_index` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_get_owner_object_index` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_set_total_rounds` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `power` | items/weapons.c | Ported |  | `tools/engine_math_reference.py` |
| `random` | items/weapons.c | Ported |  | `tools/engine_random_reference.py` |
| `weapon_new` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_delete` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_export_function_values` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_handle_potential_inventory_item` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_owner_update` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_build_weapon_interface_state` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_reloading` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_rotate_zoom_level` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_get_zoom_magnification` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_get_field_of_view` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_prevents_melee_attack` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_get_projectile_owner_object_index` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_can_fire_again` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_idle` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_effect_looping_new` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_detonate` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_change_state` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_state_key_frame` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_magazine_state_interruptable` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_effect_new` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_reset` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_state_interruptable` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_set_current_amount` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_ready` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_put_away` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_aim` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_stop_reload` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_set_state` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `projectile_distribute` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `trigger_create_projectiles` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_fire` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_begin_firing` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_overload` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_trigger_release_charge` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `weapon_update` | items/weapons.c | Compiled |  | compiled into game.wasm |
| `item_rested_surface_index` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_rested_surface` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `item_set_rested_surface` | items/items.c | Ported |  | `tools/engine_item_reference.py` |
| `weapon_trigger_overcharged` | items/weapons.c | Compiled |  | compiled into game.wasm |

## `main`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `console_initialize` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_initialize_for_new_map` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_open` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_close` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_is_active` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_printf` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_warning` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_process_command` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_get_text_to_autocomplete` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_complete` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_startup` | main/console.c | Compiled | runtime/src/files.rs:189 | compiled into game.wasm |
| `console_dispose` | main/console.c | Compiled |  | compiled into game.wasm |
| `console_update` | main/console.c | Compiled |  | compiled into game.wasm |
| `main_get_seconds_elapsed` | main/main.c | Compiled |  | compiled into game.wasm |
| `gamepad_button_is_down` | main/main.c | Compiled |  | compiled into game.wasm |
| `game_connection_set` | main/main.c | Compiled |  | compiled into game.wasm |
| `game_connection` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_disallow_persistent_storage` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_set_map_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_defer_map_map_change` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_set_multiplayer_map_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_get_map_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_get_multiplayer_map_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_set_difficulty` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_get_difficulty` | main/main.c | Compiled |  | compiled into game.wasm |
| `sort_desired_local_player_controllers` | main/main.c | Compiled |  | compiled into game.wasm |
| `create_local_players` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_reset_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_revert_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_skip_cinematic` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_map_nonsafe` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_saving_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_cancel` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_map_safe` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_won_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_lost_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_respawn` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_core` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_core_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_load_core` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_load_core_at_startup` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_load_core_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_load_core_name_at_startup` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_switch_structure_bsp` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_skip` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_queue_map_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_queue_map_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_goto_main_menu` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_menu_precache_resources` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_menu_unload` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_menu_ensure_player_queues_exist` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_menu_fade_active` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_menu_switch_to_single_player` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_set_game_connection_to_film_playback` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_get_solo_level_from_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_get_current_solo_level` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_get_solo_level_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_run_demos` | main/main.c | Compiled |  | compiled into game.wasm |
| `screenshot_record` | main/main.c | Compiled |  | compiled into game.wasm |
| `compute_window_bounds` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_get_window_count` | main/main.c | Compiled |  | compiled into game.wasm |
| `compute_subframe_counts` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_new_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_change_map_name` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_native_build_label` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_native_error_tail` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_coop_host` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_revert_map_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_skip_cinematic_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_skip_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_saving_map_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_map_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_switch_to_structure_bsp_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_death_timer_expired` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_lost_map_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_respawn_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_load_core_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_core_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_run_demos_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_framerate_throttle_enabled` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_frame_rate_debug` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_rasterizer_throttle` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_won_map_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_taking_screenshot` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_movie_start` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_movie_stop` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_stop_time` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_start_time` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_crash` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_print_version` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_vertical_blank_interrupt_handler` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_current_solo_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_load_last_solo_map` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_save_map_no_timeout` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_load_ui_scenario` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_menu_load` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_roll_credits` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_pregame_render` | main/main.c | Compiled |  | compiled into game.wasm |
| `set_window_camera_values` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_present_frame` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_exit` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_reset_time` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_update_time_unthrottled` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_framerate_render` | main/main.c | Compiled |  | compiled into game.wasm |
| `halt_and_catch_fire` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_loop_of_death` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_update_time` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_loop` | main/main.c | Compiled |  | compiled into game.wasm |
| `debug_keys_initialize` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_keys_dispose` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_keys_update` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_select_actor` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_select_next_encounter` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_select_prev_encounter` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_select_next_actor` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_select_prev_actor` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_show_actor_spray` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_profile_reset` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_erase_all_actors` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `debug_key_play_animation` | main/debug_keys.c | Compiled |  | compiled into game.wasm |
| `main_setup_connection` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_initialize_time` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_reset_map_private` | main/main.c | Compiled |  | compiled into game.wasm |
| `screenshot_render` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_game_render` | main/main.c | Compiled |  | compiled into game.wasm |
| `main_loop_iteration` | main/main.c | Compiled |  | compiled into game.wasm |

## `math`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `geosphere_dispose` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `convex_hull3d_verify` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `plane2d_distance_to_point` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `vector_intersect_plane2d` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `vector_intersect_plane3d` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `subdivide_triangle` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `build_torus` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `get_face_vertex` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `convex_hull2d` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull2d_verify` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull2d_test_circle` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull2d_test_point` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull2d_test_point_indexed` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull2d_area` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull2d_perimeter` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull2d_test_vector` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_polygon2d_clip_to_plane` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_polygon2d_verify` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_polygon3d_clip_to_plane` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_polygon3d_verify` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull3d_begin` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull3d_expand` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull3d` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull3d_test_point` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `convex_hull3d_test_vector` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `get_edge_vertex` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `convex_hull2d_intersect` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `calculate_vertex` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `points_dimension2d` | math/geometry.c | Compiled |  | compiled into game.wasm |
| `geosphere_new` | math/geometry.c | Ported |  | `tools/engine_math_reference.py` |
| `matrix4x3_identity` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_transpose` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_inverse` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix4x3_scale` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_translation` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_rotation_from_axis_and_angle` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix4x3_rotation_from_quaternion` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix4x3_from_point_and_quaternion` | math/matrix_math.c | Ported |  | `tools/engine_model_reference.py` |
| `matrix4x3_from_orientation` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix4x3_to_point_and_vectors` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_transform_point` | math/matrix_math.c | Ported |  | `tools/engine_collision_reference.py` |
| `matrix4x3_transform_vector` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix4x3_transform_normal` | math/matrix_math.c | Ported |  | `tools/engine_math_reference.py` |
| `matrix4x3_inverse_transform_point` | math/matrix_math.c | Ported |  | `tools/engine_scenario_reference.py` |
| `matrix4x3_inverse_transform_vector` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix4x3_inverse_transform_normal` | math/matrix_math.c | Ported |  | `tools/engine_math_reference.py` |
| `matrix4x3_multiply` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix3x3_determinant` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix3x3_transpose` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix3x3_inverse` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix3x3_from_axis_and_angle` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix3x3_multiply` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix3x3_transform_vector` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_rotation_from_vectors` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `matrix4x3_rotation_from_angles` | math/matrix_math.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `matrix4x3_rotation_to_angles` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_rotation_to_quaternion` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix4x3_from_point_and_vectors` | math/matrix_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `vector_from_matrices4x3` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_transform_plane` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_inverse_transform_plane` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix3x3_from_forward_and_up` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `matrix3x3_rotation_to_quaternion` | math/matrix_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `valid_real_plane3d` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `matrix4x3_from_plane` | math/matrix_math.c | Compiled |  | compiled into game.wasm |
| `periodic_functions_dispose` | math/periodic_functions.c | Compiled |  | compiled into game.wasm |
| `periodic_function_evaluate` | math/periodic_functions.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `transition_function_evaluate` | math/periodic_functions.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `@periodic_function_build_variable_period_x_table` | math/periodic_functions.c | Compiled |  | compiled into game.wasm |
| `transition_function_build_table` | math/periodic_functions.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `periodic_function_build_table` | math/periodic_functions.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `periodic_functions_initialize` | math/periodic_functions.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `factorial` | math/probability.c | Compiled |  | compiled into game.wasm |
| `permutations` | math/probability.c | Compiled |  | compiled into game.wasm |
| `combinations` | math/probability.c | Compiled |  | compiled into game.wasm |
| `permute` | math/probability.c | Compiled |  | compiled into game.wasm |
| `combine` | math/probability.c | Compiled |  | compiled into game.wasm |
| `lock_global_random_seed` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `unlock_global_random_seed` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `get_global_random_seed_address` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `get_random_seed` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `get_global_local_random_seed_address` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `random_seed_debug_log` | math/random_math.c | Compiled |  | compiled into game.wasm |
| `get_number_suitable_for_initializing_random_seed` | math/random_math.c | Compiled |  | compiled into game.wasm |
| `random_math_initialize` | math/random_math.c | Compiled |  | compiled into game.wasm |
| `random_math_dispose` | math/random_math.c | Compiled |  | compiled into game.wasm |
| `real_seed_random` | math/random_math.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `real_seed_random_range` | math/random_math.c | Ported |  | `tools/engine_math_reference.py` |
| `seed_random` | math/random_math.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `seed_random_range` | math/random_math.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `direction3d_from_table` | math/random_math.c | Ported |  | `tools/engine_math_reference.py` |
| `seed_random_direction3d` | math/random_math.c | Ported |  | `tools/engine_random_reference.py` |
| `seed_random_orientation` | math/random_math.c | Compiled |  | compiled into game.wasm |
| `seed_random_vector_in_cone3d` | math/random_math.c | Ported |  | `tools/engine_math_reference.py` |
| `vector_tree_new` | math/vector_tree.c | Compiled |  | compiled into game.wasm |
| `vector_tree_delete` | math/vector_tree.c | Compiled |  | compiled into game.wasm |
| `vectors_equal` | math/vector_tree.c | Compiled |  | compiled into game.wasm |
| `vector_tree_add_node` | math/vector_tree.c | Compiled |  | compiled into game.wasm |
| `vector_tree_find` | math/vector_tree.c | Compiled |  | compiled into game.wasm |
| `set_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `set_point2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `offset_point2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `rectangle2d_width` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `rectangle2d_height` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `inset_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `offset_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `adjust_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `intersect_rectangles2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `rectangle2d_hull_from_rectangles2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `point2d_in_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `interior_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `equal_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `equal_point2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `floor_log2` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `ceiling_log2` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `floor_power2` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `ceiling_power2` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `integer_square_root` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `bit_vector_and` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `bit_vector_or` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `bit_vector_not` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `scale_rectangle2d` | math/integer_math.c | Compiled |  | compiled into game.wasm |
| `periodic_function_build_variable_period_x_table` | math/periodic_functions.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `magnitude_squared3d_right_associated` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `real_math_initialize` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `real_math_dispose` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `real_math_reset_precision` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `perpendicular2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `perpendicular3d` | math/real_math.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `perpendicular4d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `rotate_vector_about_axis` | math/real_math.c | Ported |  | `tools/engine_item_reference.py` |
| `vectors_interpolate` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `points_interpolate` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `scalars_interpolate` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `scalars_interpolate_and_clamp_0_to_1` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `component_vectors_from_normal3d` | math/real_math.c | Ported |  | `tools/engine_physics_math_reference.py` |
| `component_vectors_from_direction3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `quaternions_multiply` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `quaternions_interpolate` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `quaternion_transform_point` | math/real_math.c | Ported |  | `tools/engine_item_reference.py` |
| `vectors3d_from_euler_angles3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `fast_vector_intersects_sphere` | math/real_math.c | Ported |  | `tools/engine_collision_reference.py` |
| `fast_vector_intersection_with_sphere` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_rectangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_rectangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `circle_intersects_rectangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_cone2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_cone3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_sector2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_sector3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `circle_intersects_cone2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `circle_intersects_sector2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `sphere_intersects_cone3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `sphere_intersects_sector3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_triangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_in_triangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `circle_intersects_triangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `pill_intersects_triangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `pill_intersects_rectangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `sphere_intersects_rectangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `sphere_intersects_triangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `pill_intersects_triangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector_intersects_rectangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector_intersects_pill2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector_intersects_pill3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector_intersects_triangle2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector_intersects_triangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector_intersects_rectangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `pill_intersects_rectangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `fast_normalize2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `fast_normalize3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `cross_product_magnitude3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `dequantize_byte_to_real` | math/real_math.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `quantize_real_to_byte_lower_bound` | math/real_math.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `quantize_real_to_byte_upper_bound` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `quantize_real_to_byte_rectangle3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `signed_angle_between_vectors2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `angle_between_vectors2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `angle_between_vectors3d` | math/real_math.c | Ported |  | `tools/engine_math_reference.py` |
| `angle_between_normals3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `yaw_vectors` | math/real_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `pitch_vectors` | math/real_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `fast_normals_interpolate` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `normals_interpolate` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `pin_normal_to_cone3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `reflect_vector3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `refract_vector3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `quaternion_normalize` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `quaternion_from_angle_and_vector` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `quaternion_to_angle_and_vector` | math/real_math.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `quaternions_interpolate_and_normalize` | math/real_math.c | Ported |  | `tools/engine_animation_reference.py` |
| `orientations_interpolate` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `euler_angles2d_from_vector3d` | math/real_math.c | Ported |  | `tools/engine_math_reference.py` |
| `vector3d_from_angle` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector3d_from_euler_angles2d` | math/real_math.c | Ported | content/src/level.rs:375 | `tools/engine_math_extra_reference.py` |
| `point_in_pill2d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_to_line_distance_squared3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `vector_to_line_distance_squared3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `sphere_test_vector3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `pill_test_vector3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `point_from_planes3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `line_from_planes3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `angular_accelerate_to_position` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `accelerate_to_velocity3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `accerate_to_position3d` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `accelerate_to_position` | math/real_math.c | Ported |  | `tools/engine_math_extra_reference.py` |
| `accelerate_to_velocity` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `valid_real_sine_cosine` | math/real_math.c | Compiled |  | compiled into game.wasm |
| `set_real_point3d` | math/real_math.h | Ported |  | `tools/engine_animation_reference.py` |

## `memory`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `dynamic_array_new` | memory/array.c | Compiled |  | compiled into game.wasm |
| `dynamic_array_resize` | memory/array.c | Compiled |  | compiled into game.wasm |
| `dynamic_array_delete` | memory/array.c | Compiled |  | compiled into game.wasm |
| `dynamic_array_add_element` | memory/array.c | Compiled |  | compiled into game.wasm |
| `dynamic_array_get_element` | memory/array.c | Compiled |  | compiled into game.wasm |
| `dynamic_array_delete_element` | memory/array.c | Compiled |  | compiled into game.wasm |
| `static_array_new` | memory/array.c | Compiled |  | compiled into game.wasm |
| `static_array_resize` | memory/array.c | Compiled |  | compiled into game.wasm |
| `static_array_add_element` | memory/array.c | Compiled |  | compiled into game.wasm |
| `static_array_get_element` | memory/array.c | Compiled |  | compiled into game.wasm |
| `static_array_delete_element` | memory/array.c | Compiled |  | compiled into game.wasm |
| `byte_swap_memory` | memory/byte_swapping.c | Compiled |  | compiled into game.wasm |
| `_byte_swap_data` | memory/byte_swapping.c | Compiled |  | compiled into game.wasm |
| `byte_swap_codes_size` | memory/byte_swapping.c | Compiled |  | compiled into game.wasm |
| `byte_swap_data` | memory/byte_swapping.c | Compiled |  | compiled into game.wasm |
| `byte_swap_data_explicit` | memory/byte_swapping.c | Compiled |  | compiled into game.wasm |
| `circular_queue_reset` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `circular_queue_verify` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `circular_queue_new` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `circular_queue_delete` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `circular_queue_size` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `circular_queue_free_space` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `circular_queue_queue_data` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `circular_queue_dequeue_data` | memory/circular_queue.c | Compiled |  | compiled into game.wasm |
| `crc_new` | memory/crc.c | Compiled |  | compiled into game.wasm |
| `build_crc_table` | memory/crc.c | Compiled |  | compiled into game.wasm |
| `crc_checksum_buffer` | memory/crc.c | Compiled |  | compiled into game.wasm |
| `data_allocation_size` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_initialize` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `datum_try_and_get` | memory/data.c | Ported | runtime/src/views.rs:9 | `tools/engine_data_reference.py` |
| `datum_get` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_verify` | memory/data.c | Ported | ffi/src/audit.rs:33 | `tools/engine_data_reference.py` |
| `datum_initialize` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_new` | memory/data.c | Ported | runtime/src/views.rs:194 | `tools/engine_data_reference.py` |
| `data_dispose` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_make_invalid` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `datum_new_at_index` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `datum_new` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `datum_delete` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_delete_all` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_iterator_new` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_iterator_next` | memory/data.c | Ported | runtime/src/views.rs:75 | `tools/engine_data_reference.py` |
| `data_next_index` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_prev_index` | memory/data.c | Compiled |  | compiled into game.wasm |
| `data_compact` | memory/data.c | Compiled |  | compiled into game.wasm |
| `data_make_valid` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `data_compress` | memory/data_compress.c | Compiled |  | compiled into game.wasm |
| `data_decompressed_size` | memory/data_compress.c | Compiled |  | compiled into game.wasm |
| `data_decompress` | memory/data_compress.c | Compiled |  | compiled into game.wasm |
| `data_encode_new` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_encode_memory` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_encode_integer` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_encode_structures` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_encode_array` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_encode_string` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_new` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_structures` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_memory` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_byte` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_short` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_long` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_int64` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_integer` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_array` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_decode_string` | memory/data_encoding.c | Compiled |  | compiled into game.wasm |
| `data_packet_group_initialize` | memory/data_packet_groups.c | Compiled |  | compiled into game.wasm |
| `data_packet_groups_get_error` | memory/data_packet_groups.c | Compiled |  | compiled into game.wasm |
| `data_packet_group_decode_packet` | memory/data_packet_groups.c | Compiled |  | compiled into game.wasm |
| `data_packet_group_append_packet_header` | memory/data_packet_groups.c | Compiled |  | compiled into game.wasm |
| `data_packet_group_encode_packet` | memory/data_packet_groups.c | Compiled |  | compiled into game.wasm |
| `_data_packet_verify` | memory/data_packets.c | Compiled |  | compiled into game.wasm |
| `_data_packet_encode` | memory/data_packets.c | Compiled |  | compiled into game.wasm |
| `_data_packet_decode` | memory/data_packets.c | Compiled |  | compiled into game.wasm |
| `data_packet_verify` | memory/data_packets.c | Compiled |  | compiled into game.wasm |
| `data_packet_encode` | memory/data_packets.c | Compiled |  | compiled into game.wasm |
| `data_packet_decode` | memory/data_packets.c | Compiled |  | compiled into game.wasm |
| `hashtable_new` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_set_user_data` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_delete` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `default_hash_function` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_search` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_get` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_remove` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_capacious_put` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_grow` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `hashtable_put` | memory/hashtable.c | Compiled |  | compiled into game.wasm |
| `lra_full` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_default_update_proc` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_default_delete_proc` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_block_delete` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `verify_lra_cache_block` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `verify_lra_cache` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_new` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_dispose` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_flush` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_free` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_lock` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_unlock` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_block_offset` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lra_allocate` | memory/lra_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_default_lock_proc` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_default_unlock_proc` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_unlock_block` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `verify_lrar_cache_block` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `verify_lrar_cache` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_new` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_dispose` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `get_lrar_cache_block` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_flush` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_allocate` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_block_address` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lrar_deallocate` | memory/lrar_cache.c | Compiled |  | compiled into game.wasm |
| `lru_default_new_block_proc` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_default_purge_block_proc` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `get_lru_cache_block_offset` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `verify_lru_cache_block` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `verify_lru_cache` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_new` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_dispose` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_flush` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_free_blocks` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_allocate` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_lock` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_unlock` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lru_touch` | memory/lru_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_allocation_size` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_update_function_pointers` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_has_locked_proc` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_cache_bytes_to_pages` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_cache_verify` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_initialize` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_delete` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_idle` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_block_delete` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_block_touch` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_block_get_address` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_block_touched` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_cache_get_page_usage` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_resize` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_debug_to_file` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_new` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_flush` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `lruv_block_new` | memory/lruv_cache.c | Compiled |  | compiled into game.wasm |
| `memory_pool_allocation_size` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_initialize` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_get_free_size` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_get_used_size` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_get_contiguous_free_size` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_block_compute_actual_size` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_block_new` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_verify` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_block_get` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_new` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_delete` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_block_allocate` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_block_free` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_compact` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_pool_block_reallocate` | memory/memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_reset` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_set_size_and_index` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_get_real_size` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_get_user_size` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_get_pool_index` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_free_space_at_end_of_pool` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_find_first_unused_memory_block` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_set_next_block_index` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_find_space_between_blocks` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_valid` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_lock` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_unlock` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_is_locked` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `memory_block_get_user_address` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_compact` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_valid_block` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_dispose_block` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_lock_block` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_unlock_block` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_allocate` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `dispose_handle` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `lock_handle` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `unlock_handle` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `dispose_pointer` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_new_block` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_new_block_clear` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `stack_memory_pool_resize_block` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `pool_new_handle` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `pool_new_handle_clear` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `pool_resize_handle` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `pool_new_pointer` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `pool_new_pointer_clear` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `pool_resize_pointer` | memory/stack_memory_pool.c | Compiled |  | compiled into game.wasm |
| `texture_page_fraction_used` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_verify` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_new` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_delete` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_textures_begin` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_texture_get` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `qsort_texture_indexes` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_resort` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_texture_new` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_textures_cancel` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_textures_end` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_texture_delete` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `texture_page_resize` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `data_usable` | memory/data.c | Ported |  | `tools/engine_data_reference.py` |
| `texture_page_verify_and_return` | memory/texture_page.c | Compiled |  | compiled into game.wasm |
| `match_init` | memory/zlib/contrib/asm386/gvmat32c.c | Replaced | source/memory/zlib/deflate.c | the editor and tools |
| `longest_match` | memory/zlib/contrib/asm386/gvmat32c.c | Replaced | source/memory/zlib/deflate.c | the editor and tools |
| `setcompressionlevel` | memory/zlib/contrib/iostream/zfstream.h | Compiled |  | compiled into game.wasm |
| `setcompressionstrategy` | memory/zlib/contrib/iostream/zfstream.h | Compiled |  | compiled into game.wasm |
| `read` | memory/zlib/contrib/iostream2/zstream.h | Compiled |  | compiled into game.wasm |
| `zstringlen` | memory/zlib/contrib/iostream2/zstream.h | Compiled |  | compiled into game.wasm |
| `write` | memory/zlib/contrib/iostream2/zstream.h | Compiled |  | compiled into game.wasm |
| `allocate_new_datablock` | memory/zlib/contrib/minizip/zip.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `OF` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `getoct` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `strtime` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `ExprMatch` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `makedir` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `matchname` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `tar` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `help` | memory/zlib/contrib/untgz/untgz.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `make_crc_table` | memory/zlib/crc32.c | Compiled |  | compiled into game.wasm |
| `get_crc_table` | memory/zlib/crc32.c | Compiled |  | compiled into game.wasm |
| `crc32` | memory/zlib/crc32.c | Compiled |  | compiled into game.wasm |
| `maketree` | memory/zlib/maketree.c | Replaced | source/memory/zlib/zlib.h | the editor and tools |
| `send_bits` | memory/zlib/trees.c | Compiled |  | compiled into game.wasm |
| `tr_static_init` | memory/zlib/trees.c | Compiled |  | compiled into game.wasm |
| `gen_trees_header` | memory/zlib/trees.c | Compiled |  | compiled into game.wasm |
| `zlibVersion` | memory/zlib/zutil.c | Compiled |  | compiled into game.wasm |
| `zcalloc` | memory/zlib/zutil.c | Compiled |  | compiled into game.wasm |
| `zcfree` | memory/zlib/zutil.c | Compiled |  | compiled into game.wasm |

## `models`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `animation_get_frame_data` | models/model_animation_definitions.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_get_frame_info` | models/model_animation_definitions.c | Compiled |  | compiled into game.wasm |
| `animation_list_get_string` | models/model_animation_definitions.c | Compiled |  | compiled into game.wasm |
| `animation_loop_frame_index` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_second_key_frame_index` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_sound_frame_index` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_is_compressed` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `build_damage_animation_index` | models/model_animations.c | Compiled | content/src/object.rs:522 | compiled into game.wasm |
| `animation_get_x_offsets` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_set_frame_size` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `quaternion_decompress_8byte` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `quaternion_decompress_6byte` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `quaternion_decompress_6byte_renormalized` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `quaternion_compress_8byte` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `quaternion_compress_6byte` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_graph_node_matrices_from_orientations` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `interpolate_node_orientations` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_graph_get_animation_by_name` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_keyframe_search` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_frame_get_xy_translation` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_choose_random_permutation_internal` | models/model_animations.c | Ported |  | `tools/engine_animation_permutation_reference.py` |
| `inverse_kinematics_adjust_matrices` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_get_keyframe_rotation` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_get_keyframe_translation` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_get_keyframe_scale` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_update_internal` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_get_node_orientations` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `replacement_animation_apply` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `overlay_animation_apply` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_scaled` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_continuous` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `overlay_animation_apply_continuous_scaled` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `aiming_screen_apply` | models/model_animations.c | Ported |  | `tools/engine_animation_reference.py` |
| `animation_get_root_matrix` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `animation_get_root_velocity` | models/model_animations.c | Compiled |  | compiled into game.wasm |
| `render_model_parts` | models/models.c | Compiled |  | compiled into game.wasm |
| `model_interpolate_node_orientations` | models/models.c | Compiled |  | compiled into game.wasm |
| `model_get_node_orientations` | models/models.c | Ported |  | `tools/engine_animation_reference.py` |
| `model_get_node_matrices` | models/models.c | Compiled |  | compiled into game.wasm |
| `model_node_matrices_from_orientations` | models/models.c | Ported |  | `tools/engine_model_reference.py` |
| `model_find_marker` | models/models.c | Ported |  | `tools/engine_model_reference.py` |
| `model_get_default_inverse_matrix` | models/models.c | Compiled |  | compiled into game.wasm |
| `model_find_node` | models/models.c | Compiled |  | compiled into game.wasm |
| `model_geometry_part_build_tangent_matrices` | models/models.c | Compiled |  | compiled into game.wasm |
| `render_model` | models/models.c | Compiled |  | compiled into game.wasm |
| `model_get_marker_by_name` | models/models.c | Ported |  | `tools/engine_model_reference.py` |
| `model_build_tangent_matrices` | models/models.c | Compiled |  | compiled into game.wasm |
| `model_geometry_fix_transparent_part_links` | models/models.c | Compiled |  | compiled into game.wasm |

## `networking`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `check_networking_and_generate_error` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_dispose` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_keep_alive` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_state` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_initiate_join_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_set_machine` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_machine` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_machine_index` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_available_games` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_error` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_seconds_to_game_start` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_write` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_address_matches_server` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_game_out_of_sync` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_ponged` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_accepted_into_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_game_settings_updated` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `unstrip_player_index` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_game_has_started` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_game_update` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_add_player_to_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_switch_to_postgame` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_switch_to_pregame` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_connection` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_remote_server_address` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_server_has_started_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_next_update_number` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_client_get_oos` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_add_player` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_update_local_player_data` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_request_start_time_change` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_countdown_timer_update` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_advertised_game_is_valid` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `add_advertised_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_set_error` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_update_precache_status` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_process_incoming_messages` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_leave_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_request_remove_player` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_remove_player` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_new_advertised_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_game_shutdown` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_reset` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_idle_searching` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_idle_joining` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_idle_pregame` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_idle_ingame` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_idle_postgame` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_create` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_idle` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_rejected_by_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_game_advertise` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_pong` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_machine_accepted` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_machine_rejected` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_game_settings_update` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_pregame_countdown` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_pregame_keep_alive` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_postgame_keep_alive` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_begin_game` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_graceful_game_exit_pregame` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_game_update` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_add_player_ingame` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_remove_player_ingame` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_game_over` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_switch_to_pregame` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message_server_graceful_game_exit_postgame` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_client_handle_message` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_connection_initialize` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_server_allow_client_connections` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_connected` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_get_address` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_connect` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_set_connection_rejection_procedure` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_server_accept_client_connection` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_active` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_going_stale` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_client_unreliable_connection_read` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_log_traffic_event` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_keep_alive` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_delete` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_write` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_server_close_client_connection` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_create_client_from_endpoint` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_client_reliable_connection_read` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_idle_client_reliable_endpoint` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_new` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_read` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_disconnect` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_idle_server_reliable_endpoint` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_idle` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_game_is_active` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_set_number_of_games_played` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_set_random_seed` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_get_game` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_player_is_local` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_accept_remote_connections` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_should_accept_remote_connections` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_is_splitscreen_local` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_set_quickstart_local` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_is_quickstart_local` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `global_network_game_server_get` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `dispose_global_network_game_server` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_server_start_frame` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `global_network_game_client_get` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `create_global_network_game_client` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `dispose_global_network_game_client` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_client_start_frame` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_client_end_frame` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_client_get_local_machine_index` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_client_local_player_quit` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_abort` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_client_all_local_players_have_quit` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_client_request_immediate_start` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_get_number_of_games_played` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_get_random_seed` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `create_global_network_game_server` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_invalidate_player` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_add_machine` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_update_machine` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `xbox_set_machine_name` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_generate_local_machine_name` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_end_and_load_ui` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_reset_for_next_round` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_assign_players_to_team` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_player_is_valid` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_invalidate_machine` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_add_player` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `compare_network_players` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_spawn_player` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_player_is_valid` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_invalidate` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_update_player` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_remove_player` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_create_game_objects` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_remove_machine` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_get_random_player_name` | networking/network_game_ui.c | Compiled |  | compiled into game.wasm |
| `initialize_network_game_packets` | networking/network_messages.c | Compiled |  | compiled into game.wasm |
| `network_event` | networking/network_messages.c | Compiled |  | compiled into game.wasm |
| `encode_network_game_message` | networking/network_messages.c | Compiled |  | compiled into game.wasm |
| `create_network_game_message` | networking/network_messages.c | Compiled |  | compiled into game.wasm |
| `decode_network_game_message` | networking/network_messages.c | Compiled |  | compiled into game.wasm |
| `countdown_timer_update` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `countdown_timer_get_time_remaining` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `countdown_timer_increment` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `countdown_timer_decrement` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `countdown_timer_set_time_remaining` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_set_game_name` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_game_name` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_state` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_open_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_game_is_open` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_game_is_valid` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_player_quit_messages_ingame` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_start_network_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_switch_to_postgame` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_graceful_shutdown` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_client_machine_is_joined_to_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_accept_client_machine_into_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `player_name_is_unique` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `get_unique_random_name` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `get_unique_random_color` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_add_player_to_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_remove_player_from_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_adjust_machine_settings` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_all_machines_have_loaded` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_client_machine_game_loading_complete` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_client_machine_is_precached` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_client_update_packet` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_switch_machine_from_postgame_to_pregame` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_update_ticks` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_queue_player_for_addition` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_begin_game_start_countdown` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `server_needs_more_teams` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `server_has_a_player_on_each_machine` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `server_has_enough_machines` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `server_ok_to_countdown` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_invalidate_network_machine` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_generate_join_game_token` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_client_machine` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_connection` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_client_connection` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_machine_connection` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_client_machine_at_index` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_client_machine_at_address` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_game_can_start` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_pause_countdown` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_change_map_name` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_change_game_variant` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_ingame_addition_count` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_kick_machine` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_rejection_message` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_reject_connection_game_is_full` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_machine_may_add_player_ingame` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_last_refusal_code` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_accepts_late_joins` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_get_client_machine_count` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `dump_network_game_data` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_dump` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_remove_client_machine_from_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_remove_machine_from_game` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_update_countdown` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_client_machine_is_loaded` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_machine_has_waiting_players` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_dispose` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_idle` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_reset_to_pregame` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_create` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_write` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_machine_is_loading_late` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_message_to_client_machine` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_game_settings_to_client_machine` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_distributed_server_machines` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_client_machine_in_game` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_distributed_server_send_to_machine` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_message_to_machine` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_message_to_all_machines` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_player_joined_info_ingame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_game_data_pregame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_distributed_server_send_to_machine_reliably` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_distributed_server_send_to_all_reliably` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_send_game_settings_to_all_machines` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_flush_game_data_pregame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_queue_client_player` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_broadcast_game_search` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_ping` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_join_game_request` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_add_player_request_pregame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_datagram` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_client_message` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `telnet_console_initialize` | networking/telnet_console.c | Compiled |  | compiled into game.wasm |
| `telnet_console_dispose` | networking/telnet_console.c | Compiled |  | compiled into game.wasm |
| `telnet_console_print` | networking/telnet_console.c | Compiled |  | compiled into game.wasm |
| `telnet_client_write` | networking/telnet_console.c | Compiled |  | compiled into game.wasm |
| `telnet_console_process` | networking/telnet_console.c | Compiled |  | compiled into game.wasm |
| `network_game_client_network_available` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_network_lost` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_map_name_is_valid` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_process_messages` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_process_last_messages` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_advertised_game_compatible` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_join_first_available_game` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_set_team` | networking/network_client_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_client_receive_game_settings_piece` | networking/network_client_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_connection_write_reliable` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_flush_reliable` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_connection_datagram_size` | networking/network_connection.c | Compiled |  | compiled into game.wasm |
| `network_distributed_client_send` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_distributed_client_send_reliably` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_distributed_client` | networking/network_game_globals.c | Compiled |  | compiled into game.wasm |
| `network_game_player_slot_reusable` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_player_slot_held` | networking/network_game_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_machine_has_players` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_refuse_late_joiner` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_same_player` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_player_among` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_start_late_joiner` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_restore_held_players` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_discard_held_players` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_queue_held_players` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_keep_late_joiners_alive` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_late_joiner_loaded` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_drop_client_machine` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_client_machine_heard` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_client_machine_is_local` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_countdown_started` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_setup_game_from_playlist` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_add_new_client` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_public_endpoint` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_client_machines` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_idle_postgame_tasks` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_have_all_machines_have_precached` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_idle_pregame_tasks` | networking/network_server_manager.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_remove_player_request_pregame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_settings_request` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_player_settings_request` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_game_start_request` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_graceful_game_exit_pregame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_map_is_precached_pregame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_loaded` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_add_player_request_ingame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_remove_player_request_ingame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_remove_player_request_postgame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `network_game_server_handle_message_client_switch_to_pregame` | networking/network_server_message_handler.c | Compiled |  | compiled into game.wasm |
| `telnet_client_disconnect` | networking/telnet_console.c | Compiled |  | compiled into game.wasm |
| `process_telnet_client_buffer` | networking/telnet_console.c | Compiled |  | compiled into game.wasm |

## `objects`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `antennas_initialize` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antennas_initialize_for_new_map` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antennas_dispose_from_old_map` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antennas_dispose` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antenna_new` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antenna_delete` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antenna_update_attachment` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antenna_render_proper` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antenna_update` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antenna_render` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `antennas_update` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `flag_datum_get_vertex` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_datum_get_cell` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flags_initialize` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flags_initialize_for_new_map` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flags_dispose_from_old_map` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flags_dispose` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_tesselate_region` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_delete` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_update_attachment` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_set_attachment_shape` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_set_trailing_shape` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_update` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_render_proper` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_new` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flag_render` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `flags_update` | objects/widgets/flags.c | Compiled |  | compiled into game.wasm |
| `glow_dispose` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_initialize_for_new_map` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_dispose_from_old_map` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_new` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_delete` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_trailing_particle_update_color` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_trailing_particle_update_size` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_trailing_particle_update_velocity` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_trailing_particle_update_position` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_trailing_particle_age` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_normal_particle_update_color` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_normal_particle_update_size` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_particle_new` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `point_from_parametric_line` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_render` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `nonuniform_cubic_spline` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `nonuniform_cubic_spline_vector3d` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_initialize` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_normal_particle_new` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `get_particle_world_position` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_normal_particle_update_position` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_particles_initialize` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_trailing_particle_new` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_update` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_submit` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `light_volumes_initialize` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volumes_dispose` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volumes_initialize_for_new_map` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volumes_dispose_from_old_map` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volume_new` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volume_delete` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volume_interpolate_frames` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `pow1` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volume_render` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `light_volume_submit` | objects/widgets/light_volumes.c | Compiled |  | compiled into game.wasm |
| `lightnings_initialize` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightnings_dispose` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightnings_initialize_for_new_map` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightnings_dispose_from_old_map` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightning_new` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightning_delete` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightning_render` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightning_offset_marker_position` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `lightning_submit` | objects/widgets/lightning.c | Compiled |  | compiled into game.wasm |
| `tag_group_to_widget_type` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widget_type_definition_get` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_initialize` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_initialize_for_new_map` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_dispose_from_old_map` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_dispose` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_new` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_delete` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_need_lighting` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_render` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `widgets_update` | objects/widgets/widgets.c | Compiled |  | compiled into game.wasm |
| `damage_initialize` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_dispose` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_initialize_for_new_map` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_dispose_from_old_map` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_render_debug` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_initialize_vitality` | objects/damage.c | Ported |  | `tools/engine_unit_vitality_reference.py` |
| `object_get_maximum_body_vitality` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_get_maximum_shield_vitality` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_data_new` | objects/damage.c | Ported |  | `tools/engine_unit_flaming_reference.py` |
| `object_restore_body` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_double_charge_shield` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_destroy_notify_children` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `get_player_index_from_object_or_parents` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_can_take_damage` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_cannot_take_damage` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_set_ranged_attack_inhibited` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_set_melee_attack_inhibited` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_effect_new_on_object` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_permutation_shield_regions` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_get_actual_body_vitality` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_get_actual_shield_vitality` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_deplete_shield` | objects/damage.c | Ported |  | `tools/engine_unit_vitality_reference.py` |
| `object_damage_shield` | objects/damage.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `object_damage_aftermath` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_effect_new_at_location` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `render_debug_object_damage` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_deplete_body` | objects/damage.c | Ported |  | `tools/engine_unit_vitality_reference.py` |
| `object_destroy` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_destroy_region` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_damage_body` | objects/damage.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `object_cause_damage` | objects/damage.c | Ported |  | `tools/engine_cause_damage_reference.py` |
| `object_damage_update` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `area_of_effect_cause_damage_to_object` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `area_of_effect_cause_damage` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `object_deleted_procs_call` | objects/object_deleted_procs.c | Compiled |  | compiled into game.wasm |
| `texture_cache_bitmap_load` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `shade_scalar` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `shade_vector2d` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `shade_vector3d` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `sample_lightmap` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `sample_diffuse_texture` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `should_render_lights` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_initialize` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_dispose` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_initialize_for_new_map` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_dispose_from_old_map` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_enable` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_delete` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_build_cluster_array` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `object_get_self_illumination` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_particle` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_disconnect_from_map` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_disconnect_from_structure_bsp` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_attenuation` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `brighten_real_rgb_color` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `cluster_get_first_light` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `cluster_get_next_light` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_marker_begin` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_unmarked` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_mark` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_marker_end` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `render_debug_light` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_queue_lens_flare` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `find_point_lights_for_object_in_cluster` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `build_distant_lights` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_compute_bounding_sphere` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_get_bounding_sphere` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_render_diffuse` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_render_specular` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_illumination_at_point` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_prepare_for_object_dynamic` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_distant_lighting_at_point` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_reconnect_to_map` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_reconnect_to_structure_bsp` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_new` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `light_new_unattached` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_preprocess_scene` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `lights_prepare_for_object_static` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `object_shadow_get_object` | objects/object_shadows.c | Compiled |  | compiled into game.wasm |
| `object_build_shadow_recursive` | objects/object_shadows.c | Compiled |  | compiled into game.wasm |
| `object_build_shadow` | objects/object_shadows.c | Compiled |  | compiled into game.wasm |
| `object_type_definition_get` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_get_datum_size` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_get_name` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_types_initialize` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_types_dispose` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_types_initialize_for_new_map` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_types_dispose_from_old_map` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_adjust_placement` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_new` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_place` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_delete` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_update` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_export_function_values` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_handle_deleted_object` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_handle_region_destroyed` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_handle_parent_destroyed` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_preprocess_node_orientations` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_postprocess_node_matrices` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_reset` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_disconnect_from_structure_bsp` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_render_debug` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_notify_impulse_sound` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_definition_index_to_object_type` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `scenario_get_object_type_scenario_datums` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `scenario_get_object_type_scenario_palette` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_types_disconnect_from_structure_bsp` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_types_place_objects` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_types_place_all` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_names_postprocess` | objects/object_types.c | Ported | content/src/campaign.rs:16 | `tools/engine_object_names_reference.py` |
| `object_types_reconnect_to_structure_bsp` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `object_type_synchronize` | objects/object_types.c | Compiled |  | compiled into game.wasm |
| `scenery_initialize` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_initialize_for_new_map` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_dispose_from_old_map` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_dispose` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_place` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_new` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_delete` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_update` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_animation_start_private` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_get_animation_time` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_animation_start` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `scenery_animation_start_at_frame` | objects/scenery.c | Compiled |  | compiled into game.wasm |
| `damage_set_network_state` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_replay_player_effect` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_replay_aftermath` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_replay_kill` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_replay_static` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_get_network_state` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `damage_kill_object_for_player` | objects/damage.c | Compiled |  | compiled into game.wasm |
| `find_point_light_for_object` | objects/object_lights.c | Compiled |  | compiled into game.wasm |
| `cluster_get_first_noncollideable_object` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `cluster_get_next_noncollideable_object` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `cluster_get_first_collideable_object` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `cluster_get_next_collideable_object` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_next_cluster` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_try_and_get_and_verify_type` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_and_verify_type` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_iterator_new` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_iterator_next` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_ultimate_parent` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_add_scenario_permutation` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_set_object_index_for_name_index` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_fix_for_deleted_object` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_set_garbage` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `garbage_collect_now` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_information_get` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_pvs_set_object` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_pvs_set_camera_point` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_pvs_clear` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_get_activating_cluster_index` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_definition_predict` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_predict` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_beautify` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_header_block_get` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_header_new` | objects/objects.c | Ported |  | `tools/engine_data_reference.py` |
| `object_header_delete` | objects/objects.c | Ported |  | `tools/engine_data_reference.py` |
| `object_header_block_allocate` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_scripting_set_collideable` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_marker_begin` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_marker_end` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_unmarked_function` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_mark_function` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_place` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `find_objects_from_point_vector` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_dump_memory` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_initialize` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_initialize_for_new_map` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_dispose_from_old_map` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_dispose` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_activate` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_deactivate` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_reset` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_placement_data_new` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_disconnect_from_map` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_first_cluster` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_node_matrices` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_attachment_marker_name` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_has_node` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_set_automatic_deactivation` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_set_visibility` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_velocities` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_location` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_start_interpolation` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_offset_interpolation` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_permute_region` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_function_value` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_in_clusters_by_indices` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_index_from_name_index` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_disconnect_from_structure_bsp` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_visible_to_any_player` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_pvs_activate` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_scripting_set_scale` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_delete_initial_recursive` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_delete` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_reconnect_to_map` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_node_matrix` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_marker_by_name` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_align_marker_to_matrix` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_detach` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_origin` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_orientation` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_get_world_matrix` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_inverse_kinematics` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_in_sphere` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_reconnect_to_structure_bsp` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_paparazzi` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_export_function_values` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_compute_node_matrices` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_postprocess_node_matrices` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_choose_random_change_colors` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_scripting_detach` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_render_debug` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `attachments_new` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_set_position` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_translate` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_new` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_attach_to_node` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_force_inside_bsp` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_update` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_compute_node_matrices_recursive` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_new_from_scenario` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_attach_to_marker` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_new_by_name` | objects/objects.c | Compiled | import/src/level.rs:1097 | compiled into game.wasm |
| `objects_scripting_attach` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_delete_immediately` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `active_garbage_limit` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_garbage_collection` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_update` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `objects_memory_compact` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_connect_lights` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_name_list_allocate` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_name_list_free` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_name_list_clear` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_name_list_new` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_claim_scenario_name` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_name_list_delete` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_name_list_lookup` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `recursive_object_adder` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `sort_dumps` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_add_to_dump` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_dump_write` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_find_region_permutations_available_with_variant` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_select_random_region_permutations_by_variant` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_determine_variant_number` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_choose_random_region_permutations` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_add_to_list` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_remove_from_list` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `attachments_delete` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_create_attachments` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_delete_attachments` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_delete_recursive` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_compute_function_values` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `object_compute_change_colors` | objects/objects.c | Compiled |  | compiled into game.wasm |
| `placeholder_initialize` | objects/placeholder_definitions.c | Compiled |  | compiled into game.wasm |
| `placeholder_initialize_for_new_map` | objects/placeholder_definitions.c | Compiled |  | compiled into game.wasm |
| `placeholder_dispose_from_old_map` | objects/placeholder_definitions.c | Compiled |  | compiled into game.wasm |
| `placeholder_dispose` | objects/placeholder_definitions.c | Compiled |  | compiled into game.wasm |
| `placeholder_place` | objects/placeholder_definitions.c | Compiled |  | compiled into game.wasm |
| `placeholder_new` | objects/placeholder_definitions.c | Compiled |  | compiled into game.wasm |
| `placeholder_delete` | objects/placeholder_definitions.c | Compiled |  | compiled into game.wasm |
| `antenna_record_shape` | objects/widgets/antenna.c | Compiled |  | compiled into game.wasm |
| `glow_time_wrap_down` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |
| `glow_time_wrap_up` | objects/widgets/glow.c | Compiled |  | compiled into game.wasm |

## `physics`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `bsp2d_test_point` | physics/bsp2d.c | Ported |  | `tools/engine_collision_reference.py` |
| `bsp3d_test_point` | physics/bsp3d.c | Ported | content/src/level.rs:908 | `tools/engine_collision_reference.py` |
| `bsp3d_clip_line_to_leaves` | physics/bsp3d.c | Compiled |  | compiled into game.wasm |
| `bsp3d_clip_polygon_to_leaves_recursive` | physics/bsp3d.c | Compiled |  | compiled into game.wasm |
| `bsp3d_clip_polygon_to_leaves` | physics/bsp3d.c | Compiled |  | compiled into game.wasm |
| `collision_surface_edge_count` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_polygon` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `add_feature` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `render_debug_collision_vertex` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `render_debug_collision_edge` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `render_debug_collision_surface` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `render_debug_collision_bsp` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_edge_length` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_perimeter` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_area` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_project_point2d` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_test_point2d` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_find_closest_point2d` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_test_line2d` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_test_sphere` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp_valid_surface_index` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp3d_enter_node` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_test_point` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `bsp2d_test_sphere_recursive` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `sphere_test_vector` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp_test_pill_new_recursive` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp_test_pill_new` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `bsp3d_test_sphere_recursive` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_leaf_test_vector` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `pill_test_vector` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp_test_sphere` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp_test_vector` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp_test_vector_recursive` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_surface_test_pill` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_bsp_test_pill` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_debug_render` | physics/collision_debug.c | Compiled |  | compiled into game.wasm |
| `collision_log_compare_overall_usage` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_initialize` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_enable` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_store_period` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_begin_period` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_continue_period` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_end_period` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_format_usage` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_render` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_get_current_user` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_start_time` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_end_time` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_usage` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_log_display` | physics/collision_usage.c | Compiled |  | compiled into game.wasm |
| `collision_model_get_material_type` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_test_sphere` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `object_test_point` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_test_point` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `object_test_vector` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_test_vector` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_test_vector_exit` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_test_pill` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_test_pill_new` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `object_get_features_in_sphere` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_get_features_in_sphere` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `clip_position_to_plane` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `clip_velocity_to_plane` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `clip_position_to_line` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `clip_velocity_to_line` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_fix_pill_nudge_collision` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_fix_pill` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_move_point` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_move_pill` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `collision_move_sphere` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `pin_fraction` | physics/physics.c | Ported |  | `tools/engine_physics_math_reference.py` |
| `physics_get_features_in_sphere` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `get_material_type` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `render_debug_physics` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_instance_new` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_test_point` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_test_vector` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `compute_ground_plane` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `friction_evaluate` | physics/physics.c | Ported |  | `tools/engine_physics_math_reference.py` |
| `physics_compute_new` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_compute_biped_collision` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_compute_vehicle_collision` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_compute_unit_collisions` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `rotate_vectors3d_by_angular_velocity` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_update_new` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_update_old` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_update` | physics/physics.c | Compiled |  | compiled into game.wasm |
| `physics_variable_position_update` | physics/physics_variables.c | Ported |  | `tools/engine_physics_variable_reference.py` |
| `physics_variable_speed_update` | physics/physics_variables.c | Ported |  | `tools/engine_physics_variable_reference.py` |
| `physics_variable_speed_update_seek` | physics/physics_variables.c | Ported |  | `tools/engine_physics_variable_reference.py` |
| `physics_variable_update` | physics/physics_variables.c | Ported |  | `tools/engine_physics_variable_reference.py` |
| `physics_variable_position_get_seek_direction` | physics/physics_variables.c | Ported |  | `tools/engine_physics_variable_reference.py` |
| `physics_variable_position_update_seek` | physics/physics_variables.c | Ported |  | `tools/engine_physics_variable_reference.py` |
| `physics_variable_update_seek` | physics/physics_variables.c | Ported |  | `tools/engine_physics_variable_reference.py` |
| `breakable_surface_plane_distance` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surface_get` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_initialize` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_dispose` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_initialize_for_new_map` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_dispose_from_old_map` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_enable` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surfaces_reset` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surface_flags_get` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surface_extant` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surface_damage` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surface_damage_area_of_effect` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `breakable_surface_effect` | physics/breakable_surfaces.c | Compiled |  | compiled into game.wasm |
| `bsp3d_plane_distance_to_point` | physics/bsp3d.c | Compiled |  | compiled into game.wasm |
| `bsp3d_polygon_plane_distance_to_point` | physics/bsp3d.c | Compiled |  | compiled into game.wasm |
| `bsp2d_test_pill_recursive` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `bsp3d_test_pill_recursive` | physics/collision_bsp.c | Ported |  | `tools/engine_collision_reference.py` |
| `collision_features_new` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_from_point` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_from_line` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_from_polygon` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_from_vertex` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_from_edge` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_from_surface` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_bsp_get_features_in_sphere` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_test_point` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_features_test_vector` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `render_debug_collision_features` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_sphere_test_point` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_cylinder_test_point` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_prism_test_point` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_sphere_test_vector` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_cylinder_test_vector` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_prism_test_vector` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `render_debug_collision_sphere` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `render_debug_collision_cylinder` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `render_debug_collision_prism` | physics/collision_features.c | Compiled |  | compiled into game.wasm |
| `collision_model_instance_new` | physics/collision_models.c | Compiled |  | compiled into game.wasm |
| `collision_model_test_sphere` | physics/collision_models.c | Compiled |  | compiled into game.wasm |
| `collision_model_test_point` | physics/collision_models.c | Compiled |  | compiled into game.wasm |
| `collision_model_test_vector` | physics/collision_models.c | Compiled |  | compiled into game.wasm |
| `collision_model_test_pill` | physics/collision_models.c | Compiled |  | compiled into game.wasm |
| `collision_model_get_features_in_sphere` | physics/collision_models.c | Compiled |  | compiled into game.wasm |
| `render_debug_collision_model` | physics/collision_models.c | Compiled |  | compiled into game.wasm |
| `collision_cluster_fog_plane` | physics/collisions.c | Compiled |  | compiled into game.wasm |
| `point_physics_initialize_for_new_map` | physics/point_physics.c | Compiled |  | compiled into game.wasm |
| `point_physics_dispose_from_old_map` | physics/point_physics.c | Compiled |  | compiled into game.wasm |
| `point_physics_definition_get_mass` | physics/point_physics.c | Ported |  | `tools/engine_physics_math_reference.py` |
| `point_physics_update` | physics/point_physics.c | Compiled |  | compiled into game.wasm |
| `point_physics_definition_interpolate` | physics/point_physics.c | Compiled |  | compiled into game.wasm |
| `render_debug_point_physics` | physics/point_physics.c | Compiled |  | compiled into game.wasm |

## `rasterizer`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `Direct3D_Release` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3D8_Release` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3D8_CreateDevice` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_BeginScene` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_EndScene` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_GetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_GetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Release` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_GetDeviceCaps` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Present` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_GetBackBuffer` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateTexture` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateVolumeTexture` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateCubeTexture` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreatePalette` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderTarget` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_preinitialize__fill_you_up_with_the_devils_cock` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_GetDepthStencilSurface` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_BeginScene` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_EndScene` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Clear` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetViewport` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_get_bitmap_default_hardware_format` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_GetRenderState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTexture` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetPalette` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_GetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShader` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetShaderConstantMode` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetStreamSource` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetIndices` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetPixelShaderProgram` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVerticalBlankCallback` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_reset_state` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_spin_begin` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_spin_end` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_windows_begin` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_window_get_fog` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_windows_end` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_frame_end` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DTexture8_GetSurfaceLevel` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DVolumeTexture_UnlockBox` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dispose` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_set_vblank_callback` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVolumeTexture8_LockBox` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVolumeTexture8_UnlockBox` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_texture_bitmap_data` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_texture_direct` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DCubeTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_texture_direct_non_blocking` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_texture` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DCubeTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DCubeTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_texture_non_blocking` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_get_target` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DPalette_Unlock` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DPalette8_Lock` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DPalette8_Unlock` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_vertex_shader` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_pixel_shader` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_model_skinning` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `D3DSurface_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DSurface8_Release` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_model_lighting_point_light` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DSurface8_GetDesc` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DSurface8_LockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `IDirect3DSurface8_UnlockRect` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_model_lighting_distant_light` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_model_lighting` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_frustum_z` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_get_stencil_mode` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `SetupSmartStates` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_filthy_bitmap_defaults_initialize` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_initialize` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_frame_begin` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_window_set_fog` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_present` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_framebuffer_blend_function` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_target` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_target_as_texture` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_secondary_render_target_debug` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_stencil_mode` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_window_begin` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_window_end` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `real_alpha_to_pixel32` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `rasterizer_active_camouflage_set_visibility` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `rasterizer_active_camouflage_cache_primary_render_target` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `rasterizer_active_camouflage_draw` | rasterizer/xbox/rasterizer_xbox_active_camouflage.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_purge_proc` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_locked_proc` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_update_function_pointers` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_initialize_for_new_map` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_dispose_from_old_map` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_flush` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_begin_update` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_end_update` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decal_vertices_new` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_DrawPrimitive` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decal_vertices_delete` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetStreamSource` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_end` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData4ub` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `D3DVertexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Register` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_initialize` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_dispose` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decal_vertices_lock` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decal_vertices_unlock` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_begin` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_decals_draw` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `detail_object_build_vertices` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014b990` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014b9e0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014ba00` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `rasterizer_detail_objects_initialize` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014bb90` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014bdb0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014be10` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014be20` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014be30` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_detail_objects_end` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014be50` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014be80` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014be90` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014bea0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `code_0014bec0` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `rasterizer_detail_objects_dispose` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_detail_objects_begin` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_detail_objects_rebuild_vertices` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_detail_objects_draw` | rasterizer/xbox/rasterizer_xbox_detail_objects.c | Compiled |  | compiled into game.wasm |
| `draw_primitives_vertex_buffer_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `draw_primitives_triangle_buffer_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_geometry_initialize` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_geometry_begin` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_geometry_end` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_triangles_new` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `draw_primitives_dynamic_triangle_count_valid` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `dynamic_vertex_group_get_d3d_vertex_buffer` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_triangles_delete` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_vertices_new` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014ceb0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014cec0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_vertices_get_type` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014cf80` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_vertices_delete` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_dynamic_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_dynamic_triangles_dynamic_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_dynamic_triangles_static_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_dynamic_triangles_static_vertices2` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_static_triangles_dynamic_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_static_triangles_static_vertices` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e180` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e190` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e1a0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e1c0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e1d0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e1e0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e1f0` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e200` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `code_0014e220` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_geometry_dispose` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_triangles_lock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_triangles_unlock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_vertices_lock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_vertices_unlock` | rasterizer/xbox/rasterizer_xbox_draw_primitives.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_hud_begin` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_hud_end` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_lit_geometry_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_screen_geometry_add_multitexture_params_to_base` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_screen_geometry_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexDataColor` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_dynamic_unlit_geometry_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `submit_screen_vertex` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_psuedo_dynamic_screen_quad_draw` | rasterizer/xbox/rasterizer_xbox_dynavobgeom.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTexture` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_lightmap_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_lightmaps_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_light_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_lights_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_textures_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_light_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_lights_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_lightmap_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_lightmaps_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_lightmap_mask_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_lightmap_mask_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_lightmap_masks_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_mirrors_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_mirrors_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflections_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflections_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_transparent_geometry_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_transparent_geometry_end` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_lightmaps_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_lightmap_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_lightmap_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_lights_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_light_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_light_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_textures_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_diffuse_texture_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_lights_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_spot_light_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_light_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_light_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_lightmaps_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_lightmap_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_specular_lightmap_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_lightmap_masks_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_lightmap_mask_begin` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_mirror_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_reflection_draw` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_transparent_geometry_submit` | rasterizer/xbox/rasterizer_xbox_environment.c | Compiled |  | compiled into game.wasm |
| `previous_camera_matrix_for_window` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_00155180` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_001551d0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_001551f0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_initialize` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_window_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_window_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_dispose` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_001554a0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_00155500` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_fog_draw` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_fog_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_fog_screen_wind_get_vector` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_001556c0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_001556d0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `code_001556e0` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_model_submit` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_model_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `set_real_vector4d` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `local_random_boolean` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_is_active` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_wind_update` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_fog_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_fog_screen_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_fog_screen_draw` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_fog_screen_end` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_model_begin` | rasterizer/xbox/rasterizer_xbox_environment_fog.c | Compiled |  | compiled into game.wasm |
| `rasterizer_error` | rasterizer/xbox/rasterizer_xbox_errors.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateTexture` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateVolumeTexture` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateCubeTexture` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `rasterizer_bitmap_new` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DBaseTexture8_Release` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `D3DTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `D3DVolumeTexture_UnlockBox` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVolumeTexture8_LockBox` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVolumeTexture8_UnlockBox` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `D3DCubeTexture_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DCubeTexture8_LockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DCubeTexture8_UnlockRect` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `rasterizer_bitmap_2d_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `rasterizer_bitmap_3d_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `rasterizer_bitmap_cm_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `rasterizer_bitmap_delete` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `rasterizer_bitmap_changed` | rasterizer/xbox/rasterizer_xbox_hardware_bitmaps.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateVertexBuffer` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateIndexBuffer` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00158450` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00158460` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `D3DVertexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `D3DIndexBuffer_Lock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `D3DIndexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DIndexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DIndexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DIndexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_vertex_buffer_new` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_vertex_buffer_delete` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_triangle_buffer_new` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_triangle_buffer_delete` | rasterizer/xbox/rasterizer_xbox_hardware_geometry.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_project_billboard` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData4f` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_sun_glow_copy_source` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_sun_glow_convolve` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_sun_glow_draw` | rasterizer/xbox/rasterizer_xbox_lights.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `set_environment_shader_pixel_shader` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_models_begin` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_model_end` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_models_end` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `rasterizer_model_ambient_reflection_tint` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `rasterizer_model_draw_environment_shader` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_model_begin` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_model_transparent_geometry_submit` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_model_draw` | rasterizer/xbox/rasterizer_xbox_models.c | Compiled |  | compiled into game.wasm |
| `code_0015d220` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d3d0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d420` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d640` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d6a0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d6b0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d6d0` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d700` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d710` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015d720` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_hud_motion_sensor_blip_begin` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_hud_motion_sensor_blip_draw` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_hud_motion_sensor_blip_end` | rasterizer/xbox/rasterizer_xbox_motion_sensor.c | Compiled |  | compiled into game.wasm |
| `code_0015e2b0` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Compiled |  | compiled into game.wasm |
| `code_0015e460` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Compiled |  | compiled into game.wasm |
| `code_0015e4b0` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Compiled |  | compiled into game.wasm |
| `code_0015e6d0` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Compiled |  | compiled into game.wasm |
| `code_0015e730` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Compiled |  | compiled into game.wasm |
| `rasterizer_plasma_energy_draw` | rasterizer/xbox/rasterizer_xbox_plasma_energy.c | Compiled |  | compiled into game.wasm |
| `profile_assert` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `callback_function` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `frame_callback_function` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_enabled` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_initialize` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_frame_begin` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_window_begin` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_profile_enable` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_begin` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_end` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_get_string` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_query` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_query_pushbuffer` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_frame_end` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_window_end` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_dispose` | rasterizer/xbox/rasterizer_xbox_profile.c | Compiled |  | compiled into game.wasm |
| `vector2d_reciprocal` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015f8d0` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015f920` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015f9b0` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015fbd0` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015fc30` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015fc40` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015fc60` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015fc70` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `code_0015fc80` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_texture_transforms` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_screen_effect` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_screen_flash` | rasterizer/xbox/rasterizer_xbox_screen_effect.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadows_begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadow_model_begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadow_model_end` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadow_end` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadows_end` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `rasterizer_shadow_convolve` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadow_begin` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadow_model_draw` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_environment_shadow_draw` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `code_00162ea0` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_00163050` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_001630a0` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_001632c0` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_00163320` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_text_end` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_00163340` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_00163360` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_00163370` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_00163380` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_text_begin` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_text_draw_character` | rasterizer/xbox/rasterizer_xbox_text.c | Compiled |  | compiled into game.wasm |
| `code_00163b40` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00163cf0` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_CreateVertexBuffer` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_group_draw__internal` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00163ee0` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00164100` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00164160` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00164170` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_BeginVisibilityTest` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_EndVisibilityTest` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_GetVisibilityTestResult` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `D3DVertexBuffer_Unlock` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Release` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Lock` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `IDirect3DVertexBuffer8_Unlock` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_groups_end` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `subtract_vectors4d` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `offset_vector4d` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_initialize_aux_buffer` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_dispose_aux_buffer` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_groups_begin` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_group_draw` | rasterizer/xbox/rasterizer_xbox_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `code_00168070` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Compiled |  | compiled into game.wasm |
| `code_00168090` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Compiled |  | compiled into game.wasm |
| `rasterizer_vertex_shaders_initialize` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Compiled |  | compiled into game.wasm |
| `rasterizer_vertex_shaders_dispose` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_initialize.c | Compiled |  | compiled into game.wasm |
| `code_00168350` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Compiled |  | compiled into game.wasm |
| `code_00168360` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Compiled |  | compiled into game.wasm |
| `code_00168370` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Compiled |  | compiled into game.wasm |
| `code_00168380` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_vertex_shader_permutation` | rasterizer/xbox/rasterizer_xbox_vertex_shaders_runtime.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `rasterizer_water_set_visibility_for_frame` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `rasterizer_water_set_visibility_for_window` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `rasterizer_water_get_visibility_for_window` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `rasterizer_water_build_bumpmap` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `rasterizer_water_draw` | rasterizer/xbox/rasterizer_xbox_water.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetRenderState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `D3DDevice_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_project_billboard` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetRenderState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetTextureStageState` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexShaderConstant` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_BeginVisibilityTest` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_EndVisibilityTest` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_GetVisibilityTestResult` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2f` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData4f` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexData2s` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_SetVertexDataColor` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_Begin` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `IDirect3DDevice8_End` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_end` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_get_occlusion_test_result` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_submit` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_begin` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_set_texture` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_set_tint_factor` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_set_zbuffer_enable` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_draw_sprite2d` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_draw_sprite3d` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_widget_submit_occlusion_test` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `shader_map_verify` | rasterizer/xbox/shader_transparent_chicago_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_transparent_chicago_create` | rasterizer/xbox/shader_transparent_chicago_preprocessor.c | Compiled |  | compiled into game.wasm |
| `rasterizer_initialize` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_reset_state` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_windows_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_window_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_window_get_fog` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_window_set_fog` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_window_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_windows_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_present` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dispose` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_vblank_callback` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_profile_enable` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_triangles_new` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_triangles_lock` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_triangles_unlock` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_triangles_delete` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_vertices_new` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_vertices_get_type` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_vertices_lock` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_vertices_unlock` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_vertices_delete` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_line` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_triangle` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_begin_screenspace` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_line_screenspace` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_linestrip_screenspace` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_end_screenspace` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_initialize` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_update_function_pointers` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_initialize_for_new_map` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_dispose_from_old_map` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_flush` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_dispose` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_new` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_lock` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_unlock` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_vertices_delete` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_detail_objects_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_detail_objects_rebuild_vertices` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_detail_objects_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_detail_objects_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_flash` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_model_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_model_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_model_transparent_geometry_submit` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_model_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_models_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_lightmaps_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_lightmap_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_lightmap_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_lightmap_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_lightmaps_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_lights_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_light_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_light_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_light_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_lights_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadows_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadow_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadow_model_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadow_model_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadow_model_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadow_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadow_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_shadows_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_textures_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_texture_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_diffuse_textures_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_lights_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_light_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_light_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_light_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_lights_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_lightmaps_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_lightmap_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_lightmap_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_lightmap_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_specular_lightmaps_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_lightmap_masks_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_lightmap_mask_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_lightmap_mask_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_lightmap_mask_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_lightmap_masks_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_mirrors_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_mirror_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_mirrors_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflections_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflection_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_reflections_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_transparent_geometry_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_transparent_geometry_submit` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_transparent_geometry_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_wind_get_vector` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_environment_fog_screen_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_hud_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_hud_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_unlit_geometry_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_lit_geometry_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_screen_geometry_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dynamic_screen_geometry_add_multitexture_params_to_base` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_psuedo_dynamic_screen_quad_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_submit` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_set_texture` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_set_tint_factor` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_set_zbuffer_enable` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_draw_sprite2d` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_draw_sprite3d` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_submit_occlusion_test` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_get_occlusion_test_result` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_hud_motion_sensor_blip_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_hud_motion_sensor_blip_draw` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_hud_motion_sensor_blip_end` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_point` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_immediate_vector` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_models_begin` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_model_vertices` | rasterizer/rasterizer.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effects_time` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effects_initialize` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effects_initialize_for_new_map` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effects_dispose_from_old_map` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effects_dispose` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_script_screen_effect_set_value` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_script_screen_effect_get_value` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_start` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_convolution` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_filter` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_filter_desaturation_tint` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_set_video` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_stop` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_effect_get_cinematic_parameters` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_set_near_clip_distance` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_get_near_clip_distance` | rasterizer/rasterizer_cinematics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_new_primitive` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_initialize` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_begin` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_end` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_dispose` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_test` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_compare_primitives` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_draw` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_line_shaded` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_triangle_shaded` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_line` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_triangle` | rasterizer/rasterizer_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_initialize` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_begin` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_get_fps` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_fps_accumulate` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `eat_my_shorts` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_count_static_vertices` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_count_dynamic_vertices` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_draw` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_end` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_statistics_dispose` | rasterizer/rasterizer_frame_statistics.c | Compiled |  | compiled into game.wasm |
| `uncompress_int8_to_real` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `uncompress_int16_to_real` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `uncompress_int32_to_real_vector3d` | rasterizer/rasterizer_geometry.c | Ported | import/src/level.rs:1943 | `tools/engine_math_extra_reference.py` |
| `rasterizer_geometry_get_vertex_size` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_geometry_byte_swap_vertices` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_geometry_uncompress_vertices` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_geometry_stripify` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `environment_vertex_compressed_get_point` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `environment_vertex_compressed_get_normal` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `environment_vertex_compressed_get_texcoord` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `environment_lightmap_vertex_compressed_get_incident_radiosity` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `environment_lightmap_vertex_compressed_get_texcoord` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `compress_real_to_int8` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `compress_real_to_int8_clamp` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `compress_real_to_int16` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `compress_real_to_int16_clamp` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `compress_real_vector3d_to_int32` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `compress_real_vector3d_to_int32_clamp` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_geometry_compress_vertices` | rasterizer/rasterizer_geometry.c | Compiled |  | compiled into game.wasm |
| `screenshot_in_progress` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `lens_flare_parameters_get` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `lens_flare_occlusion_test_results_get` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lights_reset_for_new_map` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lights_begin_for_new_frame` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lights_begin` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_light_submit` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lights_end` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `lens_flare_evaluate_corona_rotation_function` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lens_flare_submit` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lens_flare_submit_for_cluster` | rasterizer/rasterizer_lights.c | Compiled | import/src/level.rs:1679 | compiled into game.wasm |
| `rasterizer_lens_flares_submit_occlusion_tests` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `rasterizer_lens_flares_draw` | rasterizer/rasterizer_lights.c | Compiled |  | compiled into game.wasm |
| `lock_rasterizer_text_data` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `unlock_rasterizer_text_data` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_text_cache_initialize` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_text_set_shadow_color` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_text_cache_flush` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_text_cache_dispose` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_character` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_character_with_dropshadow` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_hires_glyph` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_hires_character` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_hires_character_with_dropshadow` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `hardware_character_cache_get_bitmap` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_string` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_draw_unicode_string` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_initialize` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_begin` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_new_group` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_new_group2` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_get_groups2` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_next_group` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_get_groups` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_get_group_from_presorted_index` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_get_group_presorted_index` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_get_group_pending_status` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_set_group_pending_status` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_get_primary_vertex_type` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_end` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_dispose` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_stop` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_sort_internal` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `group_sorted_indices_cmpfn` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_sort_external` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_geometry_draw` | rasterizer/rasterizer_transparent_geometry.c | Compiled |  | compiled into game.wasm |
| `rasterizer_initialize_for_new_map` | rasterizer/common/rasterizer_common.c | Compiled |  | compiled into game.wasm |
| `rasterizer_dispose_from_old_map` | rasterizer/common/rasterizer_common.c | Compiled |  | compiled into game.wasm |
| `rasterizer_frame_update` | rasterizer/common/rasterizer_common.c | Compiled |  | compiled into game.wasm |
| `rasterizer_memory_pool_initialize` | rasterizer/rasterizer_memory_pool.c | Compiled |  | compiled into game.wasm |
| `rasterizer_memory_pool_begin` | rasterizer/rasterizer_memory_pool.c | Compiled |  | compiled into game.wasm |
| `rasterizer_memory_alloc` | rasterizer/rasterizer_memory_pool.c | Compiled |  | compiled into game.wasm |
| `rasterizer_memory_alloc_const` | rasterizer/rasterizer_memory_pool.c | Compiled |  | compiled into game.wasm |
| `rasterizer_memory_pool_end` | rasterizer/rasterizer_memory_pool.c | Compiled |  | compiled into game.wasm |
| `rasterizer_memory_pool_dispose` | rasterizer/rasterizer_memory_pool.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_get_max_mipmap_count` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_get_pixel_data_size` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `bitmap_swizzle_vector2d` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_swizzle2d_byte` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_swizzle2d_word` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_swizzle2d_long` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_swizzle3d_byte` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_swizzle3d_word` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_swizzle3d_long` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_swizzle` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `rasterizer_xbox_bitmap_rebuild_hardware_format` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `bitmap_swizzle_vector3d` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `compute_swizzle_masks` | rasterizer/rasterizer_swizzle.c | Compiled |  | compiled into game.wasm |
| `hardware_character_cache_get_origin` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `hardware_character_padding` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `flush_hardware_character` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `cache_hardware_format_character` | rasterizer/rasterizer_text.c | Compiled |  | compiled into game.wasm |
| `rasterizer_transparent_encloses` | rasterizer/rasterizer_transparent_enclosure.h | Compiled |  | compiled into game.wasm |
| `rasterizer_screen_width_update` | rasterizer/xbox/rasterizer_xbox.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_drawing_begin` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_debug_drawing_end` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_begin` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_end` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_line` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_triangle` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_begin_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_end_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_line_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `_rasterizer_debug_immediate_linestrip_screenspace` | rasterizer/xbox/rasterizer_xbox_debug.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decal_blend_function_commutes` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_sort_run` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `rasterizer_decals_draw_order` | rasterizer/xbox/rasterizer_xbox_decals.c | Compiled |  | compiled into game.wasm |
| `rasterizer_shadow_convolve_pass` | rasterizer/xbox/rasterizer_xbox_shadows.c | Compiled |  | compiled into game.wasm |
| `rasterizer_widget_batch_sprites` | rasterizer/xbox/rasterizer_xbox_widgets.c | Compiled |  | compiled into game.wasm |
| `shader_stage_color_input` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_stage_color_output` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_stage_color_output_flags` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_stage_alpha_input` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_stage_alpha_output` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_stage_alpha_output_flags` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_stage_verify` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |
| `shader_transparent_generic_create` | rasterizer/xbox/shader_transparent_generic_preprocessor.c | Compiled |  | compiled into game.wasm |

## `render`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `render_effects` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_initialize` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_initialize_for_new_map` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_dispose_from_old_map` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_dispose` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_nonplayer_frame` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_frame_pregame` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_frame_present` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_location_visible` | render/render.c | Compiled |  | compiled into game.wasm |
| `rendered_cluster_get` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_window` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_player_frame` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_frame` | render/render.c | Compiled |  | compiled into game.wasm |
| `render_camera_warn_once` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_new` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_hack_frustum_z` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_build_frustum_bounds` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_frustum_sphere_diameter_in_pixels` | render/render_cameras.c | Ported |  | `tools/engine_render_size_reference.py` |
| `render_frustum_cube_view_fraction` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_frustum_get_projection_bounds` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_view_to_screen` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_screen_to_view` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_screen_to_world` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_get_adjusted_field_of_view_tangent` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_build_clipped_frustum_bounds` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_triangle_frontfacing` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_frustum_build_point_flags` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_frustum_triangle_visible` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_frustum_cube_visible` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_frustum_sphere_visible` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_world_to_screen` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_debug_frustum` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_mirror` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `render_camera_build_frustum` | render/render_cameras.c | Compiled |  | compiled into game.wasm |
| `contrail_fade` | render/render_contrails.c | Ported |  | `tools/engine_render_size_reference.py` |
| `render_contrail` | render/render_contrails.c | Compiled |  | compiled into game.wasm |
| `render_contrails` | render/render_contrails.c | Compiled |  | compiled into game.wasm |
| `render_contrails_ground_mapped` | render/render_contrails.c | Compiled |  | compiled into game.wasm |
| `render_contrails_media_mapped` | render/render_contrails.c | Compiled |  | compiled into game.wasm |
| `render_contrails_normal` | render/render_contrails.c | Compiled |  | compiled into game.wasm |
| `render_debug_triangle` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_quadrilateral` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_polygon` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_add_cache_string` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `build_circle_points` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `build_height_matrix` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `build_pill_points` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_add_cache_entry` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_point` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_line` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_vector` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_tick` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_line_offset` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_matrix` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_sphere` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_cylinder` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_pill` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_box` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_polygon_edges` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_string` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_string_at_point` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_camera` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_player` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_structure` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_bsp` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_input` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_structure_decals` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_point2d` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_line2d` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_vector2d` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_circle` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_vectors` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_quaternion` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_box2d_outline` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug_box_outline` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_debug` | render/render_debug.c | Compiled |  | compiled into game.wasm |
| `render_media_initialize_for_new_map` | render/render_media.c | Compiled |  | compiled into game.wasm |
| `render_media_dispose_from_old_map` | render/render_media.c | Compiled |  | compiled into game.wasm |
| `render_media_build_render_target` | render/render_media.c | Compiled |  | compiled into game.wasm |
| `render_media` | render/render_media.c | Compiled |  | compiled into game.wasm |
| `object_get_render_bounding_sphere` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_objects_initialize` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_objects_initialize_for_new_map` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_objects_dispose_from_old_map` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_objects_dispose` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `object_is_first_person_camera` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `find_rendered_objects` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `object_get_level_of_detail_pixels` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_object_list` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `interpolate_real_argb_color` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `interpolate_normal` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_object_shadow_begin` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `shadow_volume_plane_pair` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_object_shadow_end` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `object_render_state_refresh` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `object_get_cached_render_state` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_object` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `object_get_cached_render_lighting` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `process_rendered_objects` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `code_0017bbc0` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_objects` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `render_object_shadows` | render/render_objects.c | Compiled |  | compiled into game.wasm |
| `local_player_is_first_person` | render/render_particles.c | Compiled |  | compiled into game.wasm |
| `compare_rendered_particles` | render/render_particles.c | Compiled |  | compiled into game.wasm |
| `render_particles` | render/render_particles.c | Compiled |  | compiled into game.wasm |
| `render_sky` | render/render_sky.c | Compiled |  | compiled into game.wasm |
| `build_sprite_transform_origin_and_direction` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `build_sprite_compute_scale` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `build_sprite_prepare_for_window` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `build_sprite_get_group` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `build_sprites_begin` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `build_sprites_end` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `build_sprite_compute_basis` | render/render_sprite.c | Ported |  | `tools/engine_sprite_reference.py` |
| `build_sprite_compute_vertex_fade` | render/render_sprite.c | Ported |  | `tools/engine_sprite_reference.py` |
| `build_sprite` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `build_sprite_rotational` | render/render_sprite.c | Compiled |  | compiled into game.wasm |
| `triangle_strip_iterator_new` | render/triangle_strips.c | Compiled |  | compiled into game.wasm |
| `triangle_strip_iterator_get_triangle` | render/triangle_strips.c | Compiled |  | compiled into game.wasm |
| `render_detail_objects` | render/render_detail_objects.c | Compiled |  | compiled into game.wasm |

## `saved games`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `dummy` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_call_before_save_procs` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_call_before_load_procs` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_call_after_load_procs` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_dispose` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_initialize_for_new_map` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_dispose_from_old_map` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_save` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_revert` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_save_to_persistent_storage` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_test_persistent_storage` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_save_core` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_reverted` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_header_valid` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_allocation_record` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_note_event` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_malloc` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_gpu_malloc` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_data_new` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_memory_pool_new` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_lruv_cache_new` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_try_and_load_from_persistent_storage` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_load_core` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_initialize` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_allocate_buffer` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_free_buffer` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_create_or_open_file` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_close_file` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_write_to_file` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_read_from_file` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_write_core` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_read_core_header` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_read_core` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_get_persistent_storage_filename` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_get_persistent_storage_path` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `delete_persistent_storage` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_open_persistent_storage` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_read_header_from_persistent_storage` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_write_to_persistent_storage` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_read_from_persistent_storage` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `game_state_create_persistent_storage` | saved games/game_state_xbox.c | Compiled |  | compiled into game.wasm |
| `player_profiles_dispose` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profiles_enumerate_available_to_local_player_index` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_delete` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get_from_path` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_number_of_available_primary_colors` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get_rgb_color` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get_highest_completed_solo_level` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get_enclosing_directory_path` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `build_default_profile` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_read` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_write_thread_proc` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_new` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get_argb_color` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get_random_good_color` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_get_random_color` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_create_default_profiles_on_disk` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_write` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profiles_initialize` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_save` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_save_last_level_played` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `player_profile_save_level_completed` | saved games/player_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profiles_initialize` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profiles_dispose` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_new` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_delete` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_get_from_path` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_get_display_name` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_number_of_default_profiles_on_disk` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_create_default_profiles_on_disk` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_read` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_write_thread_proc` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profiles_enumerate_available_to_local_player_index` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_get` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_write` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_save` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_dispose` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_close` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_get_type` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_notify_memory_units_changed` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_get_useable_untitled_profile_name` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_take_mutex` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_release_mutex` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_perform_file_system_checks` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_name_unique` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_remember_player1_last_used_profile_directory` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_retrieve_player1_last_used_profile_directory` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_remember_last_used_multiplayer_variant_directory` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_retrieve_last_used_multiplayer_variant_directory` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_remember_last_used_multiplayer_map` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_retrieve_last_used_multiplayer_map` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_generate_checksum` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `find_and_create_directory_if_necessary` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_saved_game_files_start` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_saved_game_files_end` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_saved_game_file` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_mapfile_start` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_mapfile_end` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_saved_game_file_from_mapfile` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_take_mapfile_mutex` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_release_mapfile_mutex` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `set_nth_entry_in_mapfile` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_memory_units` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_initialize` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_find_profile_index_for_directory_path` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_default_playlist_profiles` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_default_player_profiles` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `count_enumerated_profiles_in_mapfile` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `build_saved_game_file_index` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `append_entry_to_mapfile` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `remove_nth_entry_in_mapfile` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_get_display_name` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `delete_enumerated_saved_game_file` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_open` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `synchronize_metadata_display_name_with_profile_name` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_file_get_path_to_enclosing_directory` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_default_profiles` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_delete_all_custom_profiles` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `get_nth_entry_in_mapfile` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `saved_game_files_enumerate_available_to_local_player_index` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `create_enumerated_saved_game_file` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `enumerate_memory_units_test` | saved games/saved_game_files.c | Compiled |  | compiled into game.wasm |
| `game_state_set_revert_time` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_allocation_new` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_contains` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_pointer` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_refuse` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_data_valid` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_reference_valid` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_memory_pool_valid` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_lruv_cache_valid` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_image_accept` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_data_arrays_new_map` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `game_state_check_data_arrays` | saved games/game_state.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_options_from_block` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |
| `playlist_profile_options_to_block` | saved games/playlist_profile.c | Compiled |  | compiled into game.wasm |

## `scenario`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `multiplayer_scenario_description_get_list` | scenario/multiplayer_scenario_description.c | Compiled |  | compiled into game.wasm |
| `map_name_from_multiplayer_scenario_description_item` | scenario/multiplayer_scenario_description.c | Compiled |  | compiled into game.wasm |
| `scenario_call_disconnect_from_structure_bsp_procs` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_call_reconnect_to_structure_bsp_procs` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_initialize` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_initialize_for_new_map` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_dispose_from_old_map` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_frame_update` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_unload` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `global_scenario_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `global_scenario_try_and_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `global_structure_bsp_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `global_collision_bsp_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `global_bsp3d_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_get_game_globals` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `global_structure_bsp_tag_index_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_location_award_bonus` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `default_material_definition_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_material_definition_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_location_deafening` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_fog_at_point` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_illumination_at_point` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_leaf_index_from_point` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_get_sky_definition_index` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_get_sky` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_test_pvs` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_test_pas` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_location_potentially_visible_local` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_location_potentially_visible` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_object_name_index_from_string` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_fog_region_get_fog_index` | scenario/scenario.c | Ported |  | `tools/engine_wind_weather_reference.py` |
| `scenario_switch_structure_bsp` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_reload_structure_bsp_if_necessary` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_get_structure_reference_index_from_tag_index` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_trigger_volume_test_point` | scenario/scenario.c | Ported | content/src/campaign.rs:143 | `tools/engine_scenario_reference.py` |
| `scenario_trigger_volume_test_object` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_debug_to_file` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `global_structure_bsp_index_get` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_load` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_location_from_point` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_location_from_line` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_ensure_point_within_world` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_get_fog_region_index` | scenario/scenario.c | Ported |  | `tools/engine_wind_weather_reference.py` |
| `scenario_location_underwater` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_location_water_depth` | scenario/scenario.c | Compiled | content/src/level.rs:752 | compiled into game.wasm |
| `scenario_get_sound_environment` | scenario/scenario.c | Ported |  | `tools/engine_sound_environment_reference.py` |
| `interpolate_real_rgb_color` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `scenario_get_atmospheric_fog` | scenario/scenario.c | Compiled |  | compiled into game.wasm |
| `wind_dispose_from_old_map` | scenario/wind.c | Ported |  | `tools/engine_wind_reference.py` |
| `wind_variance_get` | scenario/wind.c | Ported |  | `tools/engine_wind_reference.py` |
| `wind_update` | scenario/wind.c | Ported |  | `tools/engine_wind_reference.py` |
| `scenario_get_current_from_weather_palette` | scenario/wind.c | Ported |  | `tools/engine_wind_reference.py` |
| `wind_variance_initialize` | scenario/wind.c | Ported |  | `tools/engine_wind_reference.py` |
| `wind_initialize_for_new_map` | scenario/wind.c | Ported |  | `tools/engine_wind_reference.py` |
| `scenario_get_current` | scenario/wind.c | Ported | import/src/level/wind_weather_reference.rs:2 | `tools/engine_wind_weather_reference.py` |
| `scenario_get_wind` | scenario/wind.c | Ported | import/src/level/wind_weather_reference.rs:2 | `tools/engine_wind_weather_reference.py` |
| `scenario_get_water_current` | scenario/wind.c | Compiled |  | compiled into game.wasm |

## `shaders`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `shader_get_and_verify_type` | shaders/shader_definitions.c | Compiled |  | compiled into game.wasm |
| `shader_get_vertex_shader_permutation` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_is_mirror` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_is_destructable` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_is_double_sided` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_is_decal` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_is_water_decal` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_ignores_effect` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_type_is_transparent` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_type_is_lightmapped` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_type_is_vertex_lit` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_type_is_valid_for_environment` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_type_is_valid_for_model` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_type_is_valid_for_modifier` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_environment_texture_animation_evaluate` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_set` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_get` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_stop` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_restart` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `numeric_countdown_timer_update` | shaders/shaders.c | Compiled |  | compiled into game.wasm |
| `shader_texture_animation_evaluate` | shaders/shaders.c | Compiled |  | compiled into game.wasm |

## `shell`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `shell_platform_initialize` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `shell_platform_verify` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `shell_platform_dispose` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `shell_screen_pause` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `shell_get_command_line` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `shell_idle` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `fuck_code_in_the_eye` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `main` | shell/shell_xbox.c | Compiled |  | compiled into game.wasm |
| `shell_initialize` | shell/shell.c | Compiled |  | compiled into game.wasm |
| `shell_dispose` | shell/shell.c | Compiled |  | compiled into game.wasm |
| `shell_application_is_paused` | shell/shell.c | Compiled |  | compiled into game.wasm |
| `shell_application_pause` | shell/shell.c | Compiled |  | compiled into game.wasm |
| `shell_running_import_tool` | shell/shell.c | Compiled |  | compiled into game.wasm |

## `sound`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `sound_file_is_aiff` | sound/sound_import/sound_aiff.c | Compiled |  | compiled into game.wasm |
| `sound_file_aiff_info_get` | sound/sound_import/sound_aiff.c | Compiled |  | compiled into game.wasm |
| `sound_file_aiff_raw_data_get` | sound/sound_import/sound_aiff.c | Compiled |  | compiled into game.wasm |
| `sound_file_aiff_format` | sound/sound_import/sound_aiff.c | Compiled |  | compiled into game.wasm |
| `sound_file_info_get` | sound/sound_import/sound_import.c | Compiled |  | compiled into game.wasm |
| `sound_raw_sample_data_get` | sound/sound_import/sound_import.c | Compiled |  | compiled into game.wasm |
| `sound_file_is_wave` | sound/sound_import/sound_wave.c | Compiled |  | compiled into game.wasm |
| `sound_file_wave_info_get` | sound/sound_import/sound_wave.c | Compiled |  | compiled into game.wasm |
| `sound_file_wave_raw_data_get` | sound/sound_import/sound_wave.c | Compiled |  | compiled into game.wasm |
| `sound_file_wave_format` | sound/sound_import/sound_wave.c | Compiled |  | compiled into game.wasm |
| `game_sound_initialize` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_sound_dispose` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_sound_initialize_for_new_map` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_sound_clear` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_sound_restore` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_looping_sound_new` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_looping_sound_delete` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `unattached_impulse_sound_new` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `unspatialized_impulse_sound_new` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_sound_time` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_sound_stop` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_foley_predict` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_set_scale` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_set_alternate` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `unattached_looping_sound_start` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `unattached_looping_sound_stop` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_stop_internal` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `track_object_impulse_sound` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_sound_set_mouth_aperture` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `obstruction_cache_slot` | sound/game_sound.c | Ported |  | `tools/engine_sound_obstruction_reference.py` |
| `compute_combined_pas` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `location_potentially_audible` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `looping_sound_definition_is_music` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `update_potentially_audible_looping_sound` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_sound_dispose_from_old_map` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `object_impulse_sound_new` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_sound_new` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `scripted_looping_sound_stop` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `game_sound_update` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `compute_sound_obstruction` | sound/game_sound.c | Ported |  | `tools/engine_sound_obstruction_reference.py` |
| `scripted_looping_sound_start` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `compress_ima_adpcm_audio_data` | sound/ima_adpcm.c | Compiled |  | compiled into game.wasm |
| `decompress_ima_adpcm_audio_data` | sound/ima_adpcm.c | Compiled |  | compiled into game.wasm |
| `byte_swap_bungie_ima_adpcm_header` | sound/ima_adpcm.c | Compiled |  | compiled into game.wasm |
| `sound_class_get` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_classes_initialize` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_classes_dispose_from_old_map` | sound/sound_classes.c | Compiled |  | compiled into game.wasm |
| `sound_classes_dispose` | sound/sound_classes.c | Compiled |  | compiled into game.wasm |
| `sound_class_datum_get` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `debug_sound_classes_enable` | sound/sound_classes.c | Compiled |  | compiled into game.wasm |
| `debug_sound_classes_set_distances` | sound/sound_classes.c | Compiled |  | compiled into game.wasm |
| `debug_sound_classes_set_wet` | sound/sound_classes.c | Compiled |  | compiled into game.wasm |
| `sound_classes_initialize_for_new_map` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_classes_update` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_class_get_gain` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_class_set_gain` | sound/sound_classes.c | Ported |  | `tools/engine_sound_class_reference.py` |
| `sound_definition_get_maximum_distance` | sound/sound_definitions.c | Compiled |  | compiled into game.wasm |
| `sound_definition_get_minimum_distance` | sound/sound_definitions.c | Compiled |  | compiled into game.wasm |
| `sound_permutation_get_mouth_aperture` | sound/sound_definitions.c | Compiled |  | compiled into game.wasm |
| `sound_definition_find_pitch_range_by_pitch` | sound/sound_definitions.c | Ported |  | `tools/engine_sound_choice_reference.py` |
| `try_to_reset_permutations` | sound/sound_definitions.c | Ported |  | `tools/engine_sound_choice_reference.py` |
| `sound_permutation_get_real_mouth_aperture` | sound/sound_definitions.c | Compiled |  | compiled into game.wasm |
| `sound_definition_next_permutation` | sound/sound_definitions.c | Ported |  | `tools/engine_sound_choice_reference.py` |
| `sound_valid_for_channel` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `current_platform_definition` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_initialize_for_new_map` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_dispose` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_is_active` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_pause` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_render_time` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_reconnect_to_structure_bsp` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_try_and_get` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_enable` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_scripted_dialog_is_playing` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_manager_set_sound_environment` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_update_time` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_scale_value` | sound/sound_manager.c | Ported |  | `tools/engine_sound_choice_reference.py` |
| `sound_definition_is_playable` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `listener_get` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_definition_promote` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_manager_master_gain` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_delete` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_travel_milliseconds` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `refresh_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_channel_summary_build` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `channel_queue_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `channel_set_properties_hardware` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `track_loop_track_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `track_loop_impulse_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_set_definition_begin` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `looping_sound_find` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `looping_sound_new` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_set_definition_end` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `limit_pitch` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `render_debug_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `render_debug_looping_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_scale_random_value` | sound/sound_manager.c | Ported |  | `tools/engine_sound_choice_reference.py` |
| `sound_calculate_fade` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_initialize` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_start_fade` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_stop` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `source_distance` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `source_distance_squared` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_preempts_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_find_best_channel` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_find_channel` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_find_like_channel` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `update_channel_for_impulse_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `update_channel_for_looping_sound` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `process_looping_sounds` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_stop_impulse` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_stop_impulse_by_source_and_definition` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_stop_all` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `source_audible` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `refresh_sounds` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `update_channels` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `prioritize_sounds` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `refresh_listener` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_new_impulse` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_refresh_looping` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `detail_sound_random_offset` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `code_001be6a0` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `code_001be890` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `code_001bebf0` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_idle` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `code_001bee50` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_dispose_from_old_map` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `sound_render` | sound/sound_manager.c | Compiled |  | compiled into game.wasm |
| `read_sound_preferences` | sound/sound_preferences.c | Compiled |  | compiled into game.wasm |
| `write_sound_preferences` | sound/sound_preferences.c | Compiled |  | compiled into game.wasm |
| `sound_scenery_new` | sound/sound_scenery.c | Compiled |  | compiled into game.wasm |
| `sound_scenery_delete` | sound/sound_scenery.c | Compiled |  | compiled into game.wasm |
| `scripted_music_stop_all` | sound/game_sound.c | Compiled |  | compiled into game.wasm |
| `sound_samples_per_second` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_volume_from_gain` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_frequency_from_pitch` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_angle_from_angle` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_occlusion_from_occlusion` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_obstruction_from_obstruction` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_get` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_virtual_get_state` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_virtual_stop` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_update` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_begin_scene` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_dispose` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_end_scene` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_set_paused` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_flush` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_set_listener_properties` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_initialize` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_virtual_set_location` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_virtual_set_properties` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_virtual_queue_sound` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_set_I3DL2_properties` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `virtual_channel_new` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_fix_rear_speakers` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_initialize_channel` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_callback` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_set_location` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_set_properties` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_queue_packet` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_fill` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_queue_sound` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `interrupt_time_error` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_virtual_remap` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_virtual_touch` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_channel_stopped` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `dsound_error` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `channel_stop` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `channel_get` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `virtual_channel_get` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |
| `channel_get_state` | sound/sound_dsound_xbox.c | Compiled |  | compiled into game.wasm |

## `structures`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `reference_list_remove` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `reference_list_copy` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_new` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_make_valid` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_make_invalid` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_delete` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_get_next_datum` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_get_first_cluster` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_get_next_cluster` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_copy` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_get_first_reference` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_reconnect` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_disconnect` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `cluster_partition_get_first_datum` | structures/cluster_partitions.c | Compiled |  | compiled into game.wasm |
| `node_stack_push` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `node_stack_pop` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `node_stack_read` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_delete` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `find_like_crossing` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `map_leaf_find_face_on_node` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_close_portal` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_leaf_is_closed` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_family_mark` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `render_debug_leaf_portal` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `render_debug_leaf_portals` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_build_portal_from_leaves` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_get_leaf_bounds` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_closure` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_leaf_spans_polygon` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_face_get_vertex3d` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `render_debug_leaf_faces` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `normalize_three_dee` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `intersect_planes3d` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_build_portals_from_leaf` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_build_leaf_face_for_leaf_on_node` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_build_portals` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_build_leaf_faces_for_leaf` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_build_leaf_faces` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `leaf_map_initialize_from_bsp` | structures/leaf_map.c | Compiled |  | compiled into game.wasm |
| `calculate_world_from_cell_index_and_offset` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `get_local_player_datum` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `structure_detail_objects_initialize` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `structure_detail_objects_dispose_from_old_map` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `structure_detail_objects_dispose` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `structure_detail_objects_flush` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `detail_object_offset` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `key_compare_cells_lower_bound` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `key_compare_cells_upper_bound` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `get_lower_bound_cell` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `get_upper_bound_cell` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `dot_product4d` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `structure_detail_objects_initialize_for_new_map` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `structure_render_detail_objects` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `render_debug_detail_objects` | structures/structure_detail_objects.c | Compiled |  | compiled into game.wasm |
| `compare_temp_markers` | structures/structure_lens_flares.c | Compiled |  | compiled into game.wasm |
| `cluster_index_from_point` | structures/structure_lens_flares.c | Compiled |  | compiled into game.wasm |
| `build_structure_lens_flares` | structures/structure_lens_flares.c | Compiled |  | compiled into game.wasm |
| `structure_lens_flares_place` | structures/structure_lens_flares.c | Compiled |  | compiled into game.wasm |
| `structure_render_set_fog_offset` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `compare_surface_indices` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_dynamic_triangles_from_bitvector` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_dynamic_triangles_from_indices` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_build_dynamic_triangles` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_pass` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_preprocess` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_lightmaps` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_diffuse_texture` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_specular_lightmaps` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_reflection_lightmap_masks` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_reflection_mirrors` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_reflections` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_transparent_geometry` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_fog` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_fog_screen` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_specular_light` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_diffuse_light` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_render_shadow` | structures/structure_render.c | Compiled |  | compiled into game.wasm |
| `structure_decals_initialize` | structures/structure_runtime_decals.c | Compiled |  | compiled into game.wasm |
| `structure_decals_initialize_for_new_map` | structures/structure_runtime_decals.c | Compiled |  | compiled into game.wasm |
| `structure_decals_reconnect_to_structure_bsp` | structures/structure_runtime_decals.c | Compiled |  | compiled into game.wasm |
| `structure_decals_disconnect_from_structure_bsp` | structures/structure_runtime_decals.c | Compiled |  | compiled into game.wasm |
| `structure_decals_dispose_from_old_map` | structures/structure_runtime_decals.c | Compiled |  | compiled into game.wasm |
| `structure_decals_dispose` | structures/structure_runtime_decals.c | Compiled |  | compiled into game.wasm |
| `structure_decals_update` | structures/structure_runtime_decals.c | Compiled |  | compiled into game.wasm |
| `debug_pvs` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_find_camera` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_traverse_subclusters` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_traverse_surface_lists` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `bounding_rectangles_intersect` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `planes_intersect_rectangle` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_find_objects` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `grow_clipping_rectangle_by_portal_hull` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_render_debug_portal_hull` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `dequantize_byte_to_real_rectangle3d` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_build_surfaces_traverse_clusters` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_build_surfaces_traverse_leaf` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `portal_hull_from_points` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `portal_hull_from_portal` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `points_within_distance` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_find_mirror` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_build_surfaces_traverse_node` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_traverse_cluster` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_build_surfaces` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_find_clusters` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structure_visibility_compute` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |
| `structures_initialize` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structures_initialize_for_new_map` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structures_dispose_from_old_map` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structures_dispose` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_cluster_marker_begin` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_cluster_unmarked` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_cluster_mark` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_cluster_marker_end` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_render_surface_from_point_and_leaf` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_get_planar_fog_definition_index` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `sphere_intersects_cluster_portal` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_clusters_in_sphere_recursive` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_clusters_in_cone` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_test_vector` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_get_planar_fog` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `render_debug_fog_planes` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_clusters_in_sphere` | structures/structures.c | Compiled |  | compiled into game.wasm |
| `structure_bsp_get_cluster_pvs` | structures/structure_bsp_definitions.c | Ported | content/src/level.rs:799 | `tools/engine_sound_obstruction_reference.py` |
| `vertex_type_from_shader_tag` | structures/structure_bsp_definitions.c | Compiled |  | compiled into game.wasm |
| `structure_bsp_get_cluster_encoded_sound_distance` | structures/structure_bsp_definitions.c | Ported | content/src/level.rs:777 | `tools/engine_sound_environment_reference.py` |
| `structure_visibility_dequantize_byte_to_real` | structures/structure_visibility.c | Compiled |  | compiled into game.wasm |

## `tag_files`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `file_location_is_valid` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_compare_last_modification_dates` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `find_files_start` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_path_add_name` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_path_add_extension` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_path_remove_name` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_path_split` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_location_get_full_path` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_read_only` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_error` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_create` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_delete` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_exists` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_rename` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_open` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_close` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_get_position` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_set_position` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_get_eof` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_set_eof` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_read` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_write` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_read_from_position` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_write_to_position` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_get_last_modification_date` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_get_size` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `find_files_next` | tag_files/files_windows.c | Compiled |  | compiled into game.wasm |
| `file_location_set_volume` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_create` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `find_files` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_read_into_memory` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_printf` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_get_info` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_copy` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_add_directory` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_set_name` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_get_location` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_get_name` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_references_equal` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `file_reference_create_from_path` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `directory_create_or_delete_contents` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `datastore_read` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `datastore_write` | tag_files/files.c | Compiled |  | compiled into game.wasm |
| `tag_name_strip_path` | tag_files/tag_files.c | Compiled |  | compiled into game.wasm |
| `tag_empty_data` | tag_files/tag_groups.c | Compiled |  | compiled into game.wasm |
| `tag_data_get_pointer` | tag_files/tag_groups.c | Compiled |  | compiled into game.wasm |
| `tag_block_get_element_with_size` | tag_files/tag_groups.c | Compiled |  | compiled into game.wasm |
| `tag_block_get_element_fast` | tag_files/tag_groups.h | Compiled |  | compiled into game.wasm |

## `text`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `draw_string_initialize` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_initialize_for_new_map` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_dispose_from_old_map` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_dispose` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `text_bounds_draw_character` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `text_pick_draw_character` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_get_string` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_set_tab_stops` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_set_indents` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_set_color` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_get_color` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_set_font` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_set_format` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_set_draw_mode` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_set_highlight` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `bitmap_draw_character` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `styled_font_get` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `parse_string_new` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `parse_string` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `parse_unicode_string` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_partial` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_unicode_string_partial` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string` | text/draw_string.c | Compiled | ui/src/draw.rs:200 | compiled into game.wasm |
| `draw_unicode_string` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_compute_bounds` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_unicode_string_compute_bounds` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `draw_string_pick` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `bitmap_draw_string` | text/draw_string.c | Compiled |  | compiled into game.wasm |
| `font_get_character_by_ascii_code` | text/font_group.c | Compiled |  | compiled into game.wasm |
| `set_language_code` | text/international_strings.c | Compiled |  | compiled into game.wasm |
| `double_byte_character` | text/international_strings.c | Compiled |  | compiled into game.wasm |
| `get_next_character` | text/international_strings.c | Compiled |  | compiled into game.wasm |
| `get_previous_character` | text/international_strings.c | Compiled |  | compiled into game.wasm |
| `align_to_character` | text/international_strings.c | Compiled |  | compiled into game.wasm |
| `character_in_pattern` | text/international_strings.c | Compiled |  | compiled into game.wasm |
| `string_list_get_string` | text/text_group.c | Compiled |  | compiled into game.wasm |
| `unicode_string_list_get_string` | text/text_group.c | Compiled |  | compiled into game.wasm |
| `umemchr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `umemcpy` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `umemcmp` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `umemmove` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `umemset` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrcpy` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrcat` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrcmp` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrlen` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrnlen` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrchr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrcoll` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrcspn` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrncat` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrncmp` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrncpy` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrpbrk` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrrchr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrspn` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrstr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrtok` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrxfrm` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrlwr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrupr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrnlwr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrnupr` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrcasecmp` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrncasecmp` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisalpha` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisupper` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uislower` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisdigit` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisxdigit` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisspace` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uispunct` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisalnum` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisprint` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uisgraph` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uiscntrl` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `utoupper` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `utolower` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufgetc` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufputc` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uungetc` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufgets` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufputs` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ugets` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uputs` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `usnprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `usprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uvfprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uvprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uvsnprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uvsprintf` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufdopen` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufopen` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufclose` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ufreopen` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uperror` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `upopen` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uremove` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `utmpnam` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrtol` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrtoul` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrtod` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uatoi` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uctime` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `uasctime` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `wide_to_ascii` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ascii_to_wide` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `ustrerror` | text/unicode.c | Compiled |  | compiled into game.wasm |
| `fallback_string` | text/text_group.c | Compiled |  | compiled into game.wasm |
| `ustring_format_takes` | text/unicode.c | Compiled |  | compiled into game.wasm |

## `tool`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `connected_geometry_new` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `connected_geometry_delete` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `connected_geometry_group_recursive` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `connected_geometry_find_or_add_vertex` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `connected_geometry_find_or_add_edge` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `plane3d_from_points` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `connected_geometry_add_triangle` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `triangle_coplanar` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `connected_geometry_add_intermediate_triangle` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `connected_geometry_group_coplanar` | tool/connected_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_file_is_open` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_initialize` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_dispose` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_set_name` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_line` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_triangle` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_polygon` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_polygon_list` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_polygon_mesh__textured_with_no_import_scale` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_comment` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_rectangle3d` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_bounded_line` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_bounded_triangle` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_bounded_polygon` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_set_transform` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_point` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |
| `error_geometry_bounded_point` | tool/error_geometry.c | Compiled |  | compiled into game.wasm |

## `units`

| Function | File | Class | Engine | Evidence |
| --- | --- | --- | --- | --- |
| `biped_limp_noodle_get_max_relaxation_iterations` | units/biped_limp_noodle.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `biped_limp_noodle_valid_joint_rotation` | units/biped_limp_noodle.c | Compiled |  | compiled into game.wasm |
| `biped_limp_noodle_move_relax_and_constrain_positions` | units/biped_limp_noodle.c | Compiled |  | compiled into game.wasm |
| `validate_real_vector3d_axes3` | units/biped_limp_noodle.c | Compiled |  | compiled into game.wasm |
| `biped_limp_noodle_adjust_orientations` | units/biped_limp_noodle.c | Compiled |  | compiled into game.wasm |
| `biped_limp_noodle_relax_nodes_onto_environment` | units/biped_limp_noodle.c | Compiled |  | compiled into game.wasm |
| `animation_left_foot_frame_index` | units/bipeds.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `animation_right_foot_frame_index` | units/bipeds.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `bipeds_initialize` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `bipeds_initialize_for_new_map` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `bipeds_dispose_from_old_map` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `bipeds_dispose` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `biped_place` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_delete` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `biped_reset` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_disconnect_from_structure_bsp` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `biped_get_physics_pill` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_stop_melee_attack` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `biped_start_limp_body_physics` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_stop_limp_body_physics` | units/bipeds.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `biped_bumped_object` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_falling_damage` | units/bipeds.c | Ported |  | `tools/engine_biped_fall_reference.py` |
| `biped_start_landing` | units/bipeds.c | Ported |  | `tools/engine_biped_landing_reference.py` |
| `biped_flying_through_air` | units/bipeds.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `biped_apply_rotation` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_falling_danger` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_adjust_placement` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_export_function_values` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_get_sight_position` | units/bipeds.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `biped_get_autoaim_pill` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_fix_position` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_render_debug` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_find_ground_surface` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_approximate_surface_index` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_find_pathfinding_surface_index` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_exit_seat_end` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_vehicle_speech` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_jump` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_dead` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_make_footstep` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_try_to_make_footsteps` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_find_nearby_support_surface` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_verify_object_vectors` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_airborne` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_landing` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_slipping` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_jumping` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_build_flying_axes` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_physics` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_snap_facing` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_new` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_preprocess_node_orientations` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_accelerate` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_turning` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update_moving` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_check_discard` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `biped_update` | units/bipeds.c | Compiled |  | compiled into game.wasm |
| `dialogue_get_vocalization_name` | units/dialogue_definitions.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `dialogue_get_vocalization_type_by_name` | units/dialogue_definitions.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_definition_get_active_hud_index` | units/unit_definitions.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_definition_get_seat_active_hud_index` | units/unit_definitions.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_test_speech` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_is_speaking` | units/unit_dialogue.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `unit_dialogue_determine_variant` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_get_speech_priority_name` | units/unit_dialogue.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_get_speech_priority_by_name` | units/unit_dialogue.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_describe_speech` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_lose_speech` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_speak` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_notify_impulse_sound` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_make_damage_sound` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_scream` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_find_dialogue_variant` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_dialogue_setup` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_dialogue_update` | units/unit_dialogue.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_set_maximum_vitality` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_vitality_reference.py` |
| `units_scripting_set_maximum_vitality` | units/unit_scripting_commands.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_set_current_vitality` | units/unit_scripting_commands.c | Compiled |  | compiled into game.wasm |
| `units_scripting_set_current_vitality` | units/unit_scripting_commands.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_get_health` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_vitality_reference.py` |
| `unit_scripting_get_shield` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_vitality_reference.py` |
| `unit_scripting_get_grenade_count` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_commands_reference.py` |
| `unit_scripting_impervious` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_commands_reference.py` |
| `unit_scripting_start_user_animation_list` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_vitality_reference.py` |
| `unit_scripting_has_weapon` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_scripting_has_weapon_readied` | units/unit_scripting_commands.c | Ported |  | `tools/engine_unit_commands_reference.py` |
| `units_initialize` | units/units.c | Compiled |  | compiled into game.wasm |
| `units_initialize_for_new_map` | units/units.c | Compiled |  | compiled into game.wasm |
| `units_dispose_from_old_map` | units/units.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `units_dispose` | units/units.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `unit_kill` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_kill_silent` | units/units.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `unit_kill_no_statistics` | units/units.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `unit_delete` | units/units.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `units_update` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_export_function_values` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_persistent_control` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_seat_entrance_point` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_melee_range_and_ticks` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_animation_frames_remaining` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_euler_axis_doplan` | units/units.c | Ported |  | `tools/engine_unit_aiming_plan_reference.py` |
| `unit_get_zoom_level` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_animation_state_interruptable` | units/units.c | Ported |  | `tools/engine_unit_state_reference.py` |
| `unit_animation_busy` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_animation_overlay_action_loops` | units/units.c | Ported |  | `tools/engine_unit_state_reference.py` |
| `unit_animation_state_loops` | units/units.c | Ported |  | `tools/engine_unit_state_reference.py` |
| `unit_animation_weapon_ik` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_animation_vehicle_ik` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_animation_aiming_screen` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_animation_state_get_aiming_screen_index` | units/units.c | Ported |  | `tools/engine_unit_state_reference.py` |
| `unit_animation_state_can_be_entered_without_animation` | units/units.c | Ported |  | `tools/engine_unit_state_reference.py` |
| `unit_animation_compute_interpolation_frame_count` | units/units.c | Ported |  | `tools/engine_unit_state_reference.py` |
| `unit_animation_start_action` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_animation_start_overlay_action` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_can_enter_seat` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_impulse` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_handle_weapon_state_change` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_record_damage` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_test_spawning` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_destroy` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_head_position` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_get_camera_position` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_estimate_position` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_center_of_mass` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_animation_impulse_get_index` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_can_play_animation_impulse` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_test_animation_impulse` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_aiming_unit_index` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_get_aiming_vector` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_get_looking_vector` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_get_facing_vector` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_inventory_next_grenade` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_controllable` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_set_controllable` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_set_possessed` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_is_busy` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_scripting_set_emotion_animation` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_suspended` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_can_blink` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_doesnt_drop_items` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_scripting_vehicle_test_seat_list` | units/units.c | Ported |  | `tools/engine_unit_commands_reference.py` |
| `unit_scripting_vehicle_test_seat` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_unit_riders` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_unit_driver` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_scripting_unit_gunner` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_shield_sapping_update` | units/units.c | Compiled |  | compiled into game.wasm |
| `units_debug_get_next_unit` | units/units.c | Compiled |  | compiled into game.wasm |
| `units_debug_get_closest_unit` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_debug_ninja_rope` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_set_user_animation` | units/units.c | Compiled |  | compiled into game.wasm |
| `any_unit_is_dangerous` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_can_see_point` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_driven_by_ai` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_gunned_by_ai` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_set_desired_flashlight_state` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_get_current_flashlight_state` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_detach_from_parent` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_seat_filled` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_seat_is_driver` | units/units.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_seat_is_gunner` | units/units.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_seat_allow_noncombatants` | units/units.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_update_driver_and_gunner` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_current_equipment` | units/units.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `unit_add_grenade_to_inventory` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_add_grenade_type_to_inventory` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_add_equipment_to_inventory` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_delete_current_equipment` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_delete_all_weapons` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_has_weapon_definition_index` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_first_free_weapon_index` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_get_weapon_count` | units/units.c | Ported |  | `tools/engine_pickup_reference.py` |
| `unit_approve_weapon_pickup` | units/units.c | Ported |  | `tools/engine_pickup_reference.py` |
| `unit_get_grenade_count` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_get_current_grenade_type` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_throw_grenade_move_to_hand` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_throw_grenade_release` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_seat_update` | units/units.c | Compiled |  | compiled into game.wasm |
| `base_seat_label_get` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `seat_label_to_base_seat_index` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `base_weapon_label_get` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_set_animation` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_animation_update` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_refresh_illumination` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_weapon` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_drop_item` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_drop_grenades` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_drop_inventory_weapons` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_handle_region_destroyed` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_melee_sound` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_cause_continuous_melee_damage` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_set_enterable_by_player` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_set_emotion` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_aim_without_turning` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_get_custom_animation_time` | units/units.c | Ported |  | `tools/engine_unit_commands_reference.py` |
| `unit_is_playing_custom_animation` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_start_user_animation` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_set_mouth_aperture` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_inventory_get_must_be_readied_weapon` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_has_weapon` | units/units.c | Ported |  | `tools/engine_unit_tables_reference.py` |
| `unit_has_weapon_with_flag` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_start_running_blindly` | units/units.c | Ported |  | `tools/engine_unit_flaming_reference.py` |
| `unit_stop_running_blindly` | units/units.c | Ported |  | `tools/engine_unit_control_reference.py` |
| `unit_flame_to_death` | units/units.c | Ported |  | `tools/engine_unit_flaming_reference.py` |
| `unit_flying_through_air` | units/units.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `unit_euler_axis_buildplan` | units/units.c | Ported |  | `tools/engine_unit_aiming_plan_reference.py` |
| `unit_euler_axis_couple` | units/units.c | Ported |  | `tools/engine_unit_aiming_plan_reference.py` |
| `unit_set_or_test_seat_and_weapon_label` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_adjust_projectile_ray` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_render_debug` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_animation_set_state` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_abort_animation` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_find_nearby_seat` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_clip_to_aiming_bounds` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_inventory_get_weapon` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_set_actively_controlled` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `vehicle_scripting_find_available_seats` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_open` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_close` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_overcharged` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_set_seat` | units/units.c | Compiled |  | compiled into game.wasm |
| `units_set_desired_flashlight_state` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_get_seat_label` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_drop_current_equipment` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_can_use_weapon` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_approve_weapon_swap` | units/units.c | Ported |  | `tools/engine_pickup_reference.py` |
| `unit_weapon_next_index` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_drop_current_weapon` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_get_current_weapon_label` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `scripting_set_magic_base_seat` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_scripting_set_seat` | units/units.c | Ported |  | `tools/engine_unit_requests_reference.py` |
| `unit_handle_deleted_object` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_cause_melee_damage` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_cause_player_melee_damage` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_stop_custom_animation` | units/units.c | Ported |  | `tools/engine_unit_commands_reference.py` |
| `unit_custom_animation_at_frame` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_align_facing` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_start_flaming_to_death` | units/units.c | Ported |  | `tools/engine_unit_flaming_reference.py` |
| `unit_running_blind` | units/units.c | Ported |  | `tools/engine_unit_flaming_reference.py` |
| `unit_vectors_are_valid` | units/units.c | Ported |  | `tools/engine_unit_aiming_bounds_reference.py` |
| `unit_verify_vectors` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_control` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_preprocess_node_orientations` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_postprocess_node_matrices` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_euler_aiming_update` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_has_animation_to_enter_seat` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_update_animation` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_unzoom` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_get_zoom_magnification` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_integrated_night_vision_is_active` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_ping_animation` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_start_animation_impulse` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_inventory_next_weapon` | units/units.c | Ported |  | `tools/engine_unit_inventory_reference.py` |
| `unit_melee_attack_begin` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_leap_begin` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_unsuspecting` | units/units.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `unit_add_weapon_to_inventory` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_ready_desired_weapon` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_throw_grenade_begin` | units/units.c | Compiled |  | compiled into game.wasm |
| `scripting_magic_melee_attack` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_impact_melee_damage` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_place` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_solo_player_integrated_night_vision_is_active` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_add_initial_weapons` | units/units.c | Ported |  | `tools/engine_unit_items_reference.py` |
| `unit_adjust_for_seat_change` | units/units.c | Ported |  | `tools/engine_unit_hands_reference.py` |
| `unit_new` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_enter_seat` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_exit_seat_end` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_died` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_enter_vehicle` | units/units.c | Compiled |  | compiled into game.wasm |
| `vehicle_scripting_load_magic` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_try_and_exit_seat` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_update` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_damage_aftermath` | units/units.c | Compiled |  | compiled into game.wasm |
| `vehicle_scripting_unload` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_scripting_exit_vehicle` | units/units.c | Compiled |  | compiled into game.wasm |
| `vehicles_initialize` | units/vehicles.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `vehicles_initialize_for_new_map` | units/vehicles.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `vehicles_dispose_from_old_map` | units/vehicles.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `vehicles_dispose` | units/vehicles.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `vehicle_place` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_delete` | units/vehicles.c | Ported |  | `tools/engine_unit_life_reference.py` |
| `vehicle_causes_collision_damage` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_build_update` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_parse_update` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_hover` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `compute_airborne_ticks` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_is_flipped` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `update_alien_fighter_physics_new` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `set_real_quaternion` | units/vehicles.c | Ported |  | `tools/engine_animation_reference.py` |
| `vehicle_reset` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_new` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_preprocess_node_orientations` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_accelerate` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_render_debug` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_find_pathfinding_surface_index` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `update_alien_fighter_physics_old` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `slowly_stop_vehicle` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `compute_acceleration` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `create_ghost_effect` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `create_pelican_effect` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_dot_product3d_test` | units/vehicles.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `update_human_plane_physics` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `update_suspension` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `create_crashing_effects` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `update_turret_physics` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `create_slipping_effects` | units/vehicles.c | Ported |  | `tools/engine_vehicle_slip_reference.py` |
| `update_human_tank_physics` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_export_function_values` | units/vehicles.c | Ported |  | `tools/engine_vehicle_function_reference.py` |
| `vehicle_moving_near_any_player` | units/vehicles.c | Ported |  | `tools/engine_unit_questions_reference.py` |
| `vehicle_stuck` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `vehicle_cross_product3d_target` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_dot_product3d_target` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_triple_product3d_target` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_update` | units/vehicles.c | Compiled |  | compiled into game.wasm |
| `unit_animation_enhanced` | units/units.c | Compiled |  | compiled into game.wasm |
| `unarmed_melee_weapon_definition_index` | units/units.c | Ported |  | `tools/engine_unit_seats_reference.py` |
| `unit_grenade_throw_ease` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_weapon_class_base_animation` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_animation_pose_at_phase` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_node_orientations_blend` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_grenade_throw_keep_legs_moving` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_network_add_weapon` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_network_drop_weapon` | units/units.c | Compiled |  | compiled into game.wasm |
| `unit_network_forget_weapon` | units/units.c | Compiled |  | compiled into game.wasm |
| `update_human_boat_physics` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `update_human_jeep_physics` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `update_alien_scout_physics` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `update_alien_fighter_physics` | units/vehicles.c | Ported |  | `tools/engine_vehicle_physics_reference.py` |
| `vehicle_mass_points_fit` | units/vehicles.c | Compiled |  | compiled into game.wasm |
