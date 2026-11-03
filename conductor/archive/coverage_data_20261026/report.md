# Report: Coverage 3 of 11, Data fidelity

The census before this track is `docs/census.md` at 1c398cf4 (the evidence
track's last commit); after it, at 9f6dffb3.
`engine/census/ratchet.tsv` holds the counts after, so no later commit can
raise a system's Missing, Partial or owed count without failing
`census --check`.

## The counts

| | Before | After |
| --- | ---: | ---: |
| Functions of the original | 7,627 | 7,627 |
| Ported | 208 | 209 |
| Matched | 538 | 525 |
| Native | 332 | 332 |
| **Partial** | **161** | **174** |
| Replaced | 2,773 | 2,773 |
| **Missing** | **3,615** | **3,614** |
| Owe evidence | 0 | 0 |

By system (only the systems whose counts moved):

| System | Ported | Matched | Native | Partial | Replaced | Missing | Owed |
|---|---|---|---|---|---|---|---|
| effects | 1 | 88 → 87 | 0 | 1 → 2 | 13 | 87 | 0 |
| items | 9 | 24 → 17 | 0 | 1 → 8 | 1 | 102 | 0 |
| objects | 2 → 3 | 42 | 0 | 17 | 59 | 227 → 226 | 0 |
| rasterizer | 0 | 3 → 2 | 0 | 0 → 1 | 777 | 0 | 0 |
| sound | 8 | 62 → 60 | 0 | 3 → 5 | 56 | 43 | 0 |
| units | 18 | 52 → 50 | 0 | 29 → 31 | 9 | 221 | 0 |
| all | 208 → 209 | 538 → 525 | 332 | 161 → 174 | 2773 | 3615 → 3614 | 0 |

`cache` (187 Replaced) and `bitmaps` (1 Matched, 209 Replaced) have no
Missing, Partial or owed functions. That was already so when this track
began: the evidence track classed them, so the spec's 37 and 3 were out
of date. The ratchet leaves both systems out, which counts them as zero,
and a count can't rise from zero without a hand edit. The rasterizer's
first Partial below needed one.

Partial rose by 13 because the census now looks at the data each finished
function reads. Thirteen functions classed Matched read fields the engine
drops, so they are Partial now, each with a `leaves_out` naming the
fields. Missing fell by one with `object_new`'s creation effect
(fea9fc7d). The ratchet was raised by hand once, for those 13 functions
(f17c00e7).

## Strides and fix-ups

- **Strides.** A census test compares every block the importer steps
  through with the original's element size. Of its six mismatches, five
  came from the census matching a block to the wrong structure. The sixth
  was real: the importer stepped the `game_globals` difficulty block at
  0x294 bytes, not 0x284, and read the first word of padding as a 36th
  value (31ff985d).
- **Fix-ups.** All four differing load-time fix-ups now match the
  original:
  - `object_names_postprocess` is checked by a reference trace over 200
    made-up scenarios, and by the 1,964 placed names on the stock maps.
  - `postprocess_bitmap_group` is checked against every stock map's
    bitmap groups (13cb5ef9).
  - `hs_scenario_postprocess` and `hs_compile_postprocess` are ported as
    `import::script_fixup`, checked against every node of every stock
    map. Where the game would disable a scenario's scripts, the importer
    now does the same instead of refusing the level (68c1fd7d).
- **The stride, fix-up and tag-field sections of `census --check`** show
  no differences.

## Layouts from a second source

`vehicle_definition` and the shader classes are defined in their `.c`
files, not in headers, so before this track they had no layout. The
census now lays out every `.c` file's own tag structures. Where a
definition stops short of the tag, it is grown to the size given by
format documentation (read only, never copied). `layout_maps.rs` checks
each size on the user's maps:

- senv 836, sgla 480, smet 260, soso 440 and spla 332 match the gaps
  between tags in the maps exactly.
- schi, sotr, swat and vehi fit within theirs.

The script fix-up, which reads untrusted syntax data, has its own fuzz
target (d4b3fa72).

## The tag fields

| Tag fields (`tag_fields.tsv`) | Start of track | Layouts filled (d4b3fa72) | After |
| --- | ---: | ---: | ---: |
| Laid out | 6,753 | 9,230 | 9,230 |
| Used by the engine | 2,159 | 3,007 | 3,231 |
| Used, through a local or a hand trace | — | — | 280 |
| Kept, used nowhere | 502 | 597 | 291 |
| Kept, untraced | 150 | 198 | **0** |
| Not kept | 3,942 | 5,428 | 5,428 |

Every tag field is now read, traced or classed with a reason:

- **Traced (280).** The census follows a read through a local into the
  field where it is stored. For the 22 uses it cannot follow, such as an
  offset handed to a helper, `field_traces.tsv` names the use by hand.
- **Partial (26 fields).** These are read by a finished function but
  dropped by the engine, so that function is now Partial. The 13
  functions are listed below.
- **Classed with a reason (291).** `unused_fields.tsv` lists each kept
  field the engine uses nowhere, with why it owes nothing yet:

  | Reason | Fields |
  | --- | ---: |
  | Owed to the Missing or Partial functions that read it (each system's track) | 147 |
  | The original reads it nowhere | 94 |
  | Read only where the engine replaced the original | 48 |
  | The same, and also by the old port's own bounds checks | 2 |

  `--check` fails if the file is stale. It also fails on any kept field
  that is used nowhere and that only finished functions read.

The 13 functions that became Partial, and the tracks that will close
them:

| Function | System | Track |
| --- | --- | --- |
| `biped_update_turning`, `update_suspension` | units | 4, units |
| `projectile_detonate`, `projectile_update`, `projectile_collision`, `weapon_trigger_fire`, `weapon_update`, `trigger_create_projectiles`, `weapon_export_function_values` | items | 4, units |
| `effect_generate_particles` | effects | 4, units (effects) |
| `rasterizer_lens_flares_draw` | rasterizer | 11, the gate (no system track names the rasterizer) |
| `update_channel_for_impulse_sound`, `update_channel_for_looping_sound` | sound | 9, scripts and sounds |

Classing these fields found one census bug. `readers` named a function
after the return type of a function-pointer parameter, so every read in
`structure_render.c` and `hs_library_external.c` was credited to a
function called `void`. It now takes a name only outside parentheses.

## What this sets up

The next track, **Coverage 4, units and the object systems**, starts
from 3,614 Missing and 174 Partial, with nothing owed. Ten of the 13
new Partials are its own. The largest open systems:

| System | Missing | Partial | Track |
| --- | ---: | ---: | --- |
| `ai` | 795 | 17 | 10, AI |
| `hs` | 576 | 8 | 9, scripts and sounds |
| `game` | 565 | 28 | 5, game loop |
| `interface` | 452 | 19 | 7, interface |
| `objects` | 226 | 17 | 4, units |
| `units` | 221 | 31 | 4, units |
| `saved games` | 104 | 2 | 5, game loop |
| `items` | 102 | 8 | 4, units |

The 147 fields owed to Missing and Partial functions are the data those
tracks will start using. `unused_fields.tsv` names each field's reader,
so a track can see which fields it owes as its ports land.

## Checks

- `census --write` then `--check`: no problems, with the ratchet at the
  counts above.
- `cargo clippy --workspace --tests -- -D warnings`: clean.
- `cargo nextest run --workspace`: 1,432 tests pass, and the 3 marked
  `#[ignore]` are skipped. The tests that need game data ran here, with
  the data present.
- `nix flake check`: passes.
