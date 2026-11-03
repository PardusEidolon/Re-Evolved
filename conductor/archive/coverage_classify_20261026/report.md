# Report: Coverage 1 of 11, Classify and cite

The census before this track is `docs/census.md` at 530eef73^; after it, at
the track's last commit. `engine/census/ratchet.tsv` now holds the counts
below, so no later commit can raise a system's Missing, Partial or owed
count without failing `census --check`.

## The counts

| | Before | After |
| --- | ---: | ---: |
| Functions of the original | 8,501 | 7,627 |
| Ported | 81 (16,832 B) | 130 (25,120 B) |
| Matched | 581 (146,624 B) | 581 (188,384 B) |
| Native | 296 (32,032 B) | 332 (38,896 B) |
| Partial | 94 (36,000 B) | 117 (72,000 B) |
| Replaced | 2,687 (519,888 B) | 2,771 (569,992 B) |
| **Missing** | **4,762 (859,746 B)** | **3,696 (716,730 B)** |
| Owe evidence | 675 | 698 |

By system (only the systems whose counts moved):

| System | Missing | Partial | Owed | Ported | Native | Replaced |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `ai` | 951 → 798 | 6 → 12 | 25 → 31 | 0 → 39 | 0 | 103 → 91 |
| `bink` | 0 | 0 | 0 | 0 | 0 | 33 → 24 |
| `bitmaps` | 3 → 0 | 0 | 1 | 0 | 0 | 209 |
| `bungie_net` | 0 | 0 | 0 | 0 | 0 | 109 → 105 |
| `cache` | 37 → 0 | 0 | 0 | 0 | 0 | 187 |
| `camera` | 108 → 81 | 2 → 3 | 21 → 22 | 0 → 1 | 0 | 17 |
| `cseries` | 0 | 0 | 0 | 0 | 29 → 28 | 122 → 112 |
| `cutscene` | 72 → 48 | 1 → 2 | 9 → 10 | 0 | 0 | 0 → 7 |
| `devices` | 43 → 42 | 0 | 17 | 0 | 0 | 0 → 1 |
| `dialogs` | 1 → 0 | 0 | 0 | 0 | 0 | 5 |
| `effects` | 88 → 87 | 0 | 89 | 0 → 1 | 0 | 13 |
| `game` | 675 → 610 | 10 → 16 | 33 → 39 | 0 → 6 | 0 | 12 → 61 |
| `hs` | 692 → 578 | 2 → 3 | 49 → 50 | 0 | 0 | 2 → 11 |
| `input` | 5 → 1 | 0 | 0 | 0 | 0 | 51 → 46 |
| `interface` | 734 → 452 | 16 → 19 | 112 → 115 | 6 | 0 | 62 → 104 |
| `items` | 151 → 103 | 0 → 1 | 24 → 25 | 7 → 8 | 0 | 0 → 1 |
| `main` | 138 → 91 | 0 → 1 | 0 → 1 | 0 | 0 | 18 → 34 |
| `math` | 84 → 73 | 0 | 5 | 36 | 72 → 70 | 19 → 25 |
| `memory` | 0 | 0 | 2 | 0 | 151 → 128 | 106 → 102 |
| `models` | 16 → 11 | 1 | 13 | 17 | 0 | 7 |
| `networking` | 0 | 0 | 0 | 0 | 0 | 313 → 288 |
| `objects` | 241 → 229 | 12 | 59 | 0 | 0 | 49 → 59 |
| `physics` | 98 → 45 | 26 | 40 | 0 | 0 | 27 → 60 |
| `rasterizer` | 0 | 0 | 3 | 0 | 0 | 793 → 777 |
| `render` | 0 | 0 | 2 | 2 | 0 | 134 → 115 |
| `saved games` | 116 → 104 | 2 | 2 | 0 | 0 | 31 |
| `shell` | 3 → 2 | 0 | 0 | 0 | 0 | 10 → 11 |
| `sound` | 93 → 46 | 2 | 65 | 5 | 0 | 55 → 56 |
| `structures` | 28 → 26 | 0 | 1 | 0 | 0 | 98 → 99 |
| `tag_files` | 0 | 0 | 0 | 0 | 44 → 43 | 5 |
| `text` | 86 → 21 | 0 | 9 | 0 | 0 → 63 | 17 → 19 |
| `units` | 278 → 227 | 14 → 17 | 81 → 84 | 8 → 9 | 0 | 0 → 9 |
| **all** | 4762 → 3696 | 94 → 117 | 675 → 698 | 81 → 130 | 296 → 332 | 2687 → 2771 |

## What moved them

1. **A double count (874 functions).** The symbol headers know 1,351
   functions only by address (`code_000ad290`), and the census counted each
   of them again under its C name, as an inlined function of size 0. Each
   file's address-only symbols, in address order, are its definitions
   without a symbol, in source order: 1,090 are named by order, 143 by order
   with the sizes rising with the bodies' lengths (rank correlation ≥ 0.8),
   and 118 that the C defines nothing for are known by their place ("after
   X"). The pairing is borne out by the manifest: 36 entries written by hand
   under a `code_` name were identical to the entries for the names it now
   gives them. `engine/census/names.tsv` holds the table, and `--check`
   fails when it is stale.
2. **Citations (Phase 2).** 23 of the engine's own versions of original
   functions (`unit_update`, `main_loop`, `weapon_ready` and others) are
   cited and classed Partial, with what they leave out. That is why Partial
   and owed rose: these functions were counted Missing before.
3. **Native helpers.** real_math.h's one-operation helpers are traced
   against the engine's primitives (49 more Ported), and unicode.c's 63
   string helpers are Native (`String`).
4. **The library reason.** The collision BSP walk (bsp3d, bsp2d,
   collision_bsp) is Replaced by Jolt, with the player-visible behaviour it
   must keep: physics' Missing fell from 98 to 45.
5. **The tools reason.** The console, the cheats and the debug draws are
   the dev shell's.
6. **Unreached functions.** All 336 functions no root of the game reaches
   were reviewed. 168 are Replaced or Native: network state and
   prediction (the engine sends snapshots), debug checks and editor hooks,
   byte swaps (settled at import), the progress bar's Direct3D drawing, the
   pad and the Bink movie. The other 168 stay Missing with a note naming the
   track that owns them (a test requires it): AI 14, units 24, game loop 26,
   animation and world 54, interface 19, cameras 20, scripts and sounds 11.

## Findings for later tracks

- **Single-precision trig.** The engine's crates make 145 calls to
  `libm::sinf`, `cosf`, `atan2f` and the other single-precision functions.
  The original's x87 code works out trig at extended precision and rounds
  to float once, so the two can differ in the last bit. The evidence track
  should trace these calls against the original and, where they differ,
  compute at double precision and round.
- **`clamp_length` (physics/src/movement.rs)** compares lengths and
  scales `normalize() * maximum`. The original's `limit2d` (ported as
  sim/src/real_math.rs) compares squared lengths and scales by
  `length / sqrtf(dot)`. These round differently, and they differ at
  exactly the limit. The units track should use the ported `limit2d`.
- **Owed rose to 698.** The most-called owed functions are still the datum
  allocators (`data_next_index`, `datum_new`, `object_new`,
  `object_delete`), `hs_return`, `hs_evaluate`, `real_random_range` and
  `sound_class_get`. By system: interface 115, effects 89, units 84,
  sound 65, objects 59, hs 50, physics 40, game 39.
- **The largest Missing systems** are ai (798, 269 KB), game (610),
  hs (578), interface (452), objects (229) and units (227). The gap heads
  from the census's ranking still stand: `actor_update`, the
  `*_decide_action` tables, `unit_damage_aftermath` and
  `main_loop_iteration`.

## The next track

Coverage 2, evidence, starts from 698 owed functions. It builds the
reference harness's fake world (objects, tag instances, a small map) and
then works down the owed list by caller count, starting with the
allocators above. The trig finding is the first divergence it should test.
