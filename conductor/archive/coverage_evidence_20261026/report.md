# Report: Coverage 2 of 11, Evidence for the owed

The census before this track is `docs/census.md` at 5eb6a643 (the classify
track's last commit); after it, at a3d09543. `engine/census/ratchet.tsv`
holds the counts after, so no later commit can raise a system's Missing,
Partial or owed count without failing `census --check`.

## The counts

| | Before | After |
| --- | ---: | ---: |
| Functions of the original | 7,627 | 7,627 |
| Ported | 130 | 208 |
| Matched | 581 | 538 |
| Native | 332 | 332 |
| Partial | 117 | 161 |
| Replaced | 2,771 | 2,773 |
| **Missing** | **3,696** | **3,615** |
| **Owe evidence** | **698** | **0** |

By system (only the systems whose counts moved):

| System | Ported | Matched | Native | Partial | Replaced | Missing | Owed |
|---|---|---|---|---|---|---|---|
| ai | 39 → 42 | 19 → 14 | 0 | 12 → 17 | 91 | 798 → 795 | 31 → 0 |
| bitmaps | 0 | 1 | 0 | 0 | 209 | 0 | 1 → 0 |
| camera | 1 → 3 | 19 → 18 | 0 | 3 → 4 | 17 | 81 → 79 | 22 → 0 |
| cseries | 0 | 0 → 1 | 28 | 0 | 112 → 111 | 0 | 0 |
| cutscene | 0 | 8 | 0 | 2 | 7 | 48 | 10 → 0 |
| devices | 0 → 7 | 17 → 18 | 0 | 0 | 1 | 42 → 34 | 17 → 0 |
| effects | 1 | 89 → 88 | 0 | 0 → 1 | 13 | 87 | 89 → 0 |
| game | 6 → 47 | 23 → 15 | 0 | 16 → 28 | 61 | 610 → 565 | 39 → 0 |
| hs | 0 → 2 | 47 → 42 | 0 | 3 → 8 | 11 | 578 → 576 | 50 → 0 |
| interface | 6 | 96 | 0 | 19 | 104 | 452 | 115 → 0 |
| items | 8 → 9 | 24 | 0 | 1 | 1 | 103 → 102 | 25 → 0 |
| main | 0 | 0 | 0 | 1 | 34 | 91 | 1 → 0 |
| math | 36 → 44 | 5 → 6 | 70 | 0 | 25 | 73 → 64 | 5 → 0 |
| memory | 0 | 2 | 128 | 0 | 102 | 0 | 2 → 0 |
| models | 17 | 12 | 0 | 1 | 7 | 11 | 13 → 0 |
| objects | 0 → 2 | 47 → 42 | 0 | 12 → 17 | 59 | 229 → 227 | 59 → 0 |
| physics | 0 | 14 | 0 | 26 → 22 | 60 → 64 | 45 | 40 → 0 |
| rasterizer | 0 | 3 | 0 | 0 | 777 | 0 | 3 → 0 |
| render | 2 | 2 | 0 | 0 | 115 | 0 | 2 → 0 |
| saved games | 0 | 0 | 0 | 2 | 31 | 104 | 2 → 0 |
| scenario | 0 | 13 → 11 | 0 | 0 → 2 | 23 | 21 | 13 → 0 |
| sound | 5 → 8 | 63 → 62 | 0 | 2 → 3 | 56 | 46 → 43 | 65 → 0 |
| structures | 0 | 1 → 2 | 0 | 0 | 99 → 98 | 26 | 1 → 0 |
| text | 0 | 9 → 5 | 63 | 0 → 4 | 19 | 21 | 9 → 0 |
| units | 9 → 18 | 67 → 52 | 0 | 17 → 29 | 9 | 227 → 221 | 84 → 0 |
| all | 130 → 208 | 581 → 538 | 332 | 117 → 161 | 2771 → 2773 | 3696 → 3615 | 698 → 0 |

Every one of the 698 owed functions now has a check against the original
and is classed by it. They went four ways:

- **Ported (+78).** A reference trace compiles the game's own function
  from `source/` against the fake world and the engine matches it, so the
  entry moved up from Matched or Missing.
- **Matched.** The rest stayed Matched, now backed by a trace or by a
  behaviour test on the stock maps.
- **Partial (+44).** The check showed part of the behaviour is not there.
  Each entry's `leaves_out` names what is missing, its `check` names the
  test that covers the rest, and it is handed to its system's track.

Missing also fell by 81. The traces compiled helpers that had not been
ported, and the fixes in the next section ported them with their
evidence.

## The evidence

- **41 new reference scripts** (`tools/engine_*_reference.py`) and **42
  reference tests** (`engine/crates/*/tests/*_reference.rs`), in 40 code
  commits. Every script has a `--check` that fails when its trace is
  stale.
- **A shared fake world** (bb93668a) with an object table, tag instances
  and a small synthetic map, which the traces compile against. No game
  data is in it.
- **The AI** (a3d09543) is checked by mission tests on the stock maps
  rather than by fake-world traces, which the plan asked for. An actor's
  behaviour needs encounters, firing positions and a BSP that the fake
  world does not yet model, and the ten stock maps exercise them as
  scripts call them. Those tests skip themselves without game data. The
  parts they do not reach are Partial for Track 10, the AI, which can
  build the fake-world actors when it ports them.

## Divergences found and fixed

Each fix has a regression test, which is the trace or test that found it.

| Commit | What the engine did | What the game does |
| --- | --- | --- |
| a5636c13 | Slowed bullets linearly over their maximum range | Decelerates them at (v0² − v1²) / 2(upper − lower) over the air or water damage range, from when a timer reaches 1 |
| 74472a35 | Never hurt a unit for a fall; `unit_suspended` did nothing | Soft, hurt and lethal landing speeds; a suspended unit holds still (without this, a10's cryo player fell through BSP 7) |
| daec474c | Scaled the frame's time by the game's speed; capped at 8 engine ticks | Uses the speed as the clock's rate and caps at 7 game ticks (14 engine ticks), throwing away the time past the cap |
| a5188b6d | Had no multiplayer announcer; gave out teams by its own alternation | Uses a five-sound queue with a two-second start wait, and the `next_team_index` counter |
| 751df1e3 | Respawned at the start farthest from enemies | Rates every start by game type, team and distances, weighted by √random |
| 0f7945f7 | Stood netgame items a wait after taking; rolled a real; never purged | Stands them on the game clock's period, rolls an integer weighted choice, and purges untaken items |
| 4ef7b5f9 | Ended a game only on its time limit | Also ends it when fewer than two teams have a player |
| 72571cdf | Held the look at 85° pitch and the yaw to −π..π | 85.5° pitch, and the yaw kept in 0..2π |
| 6df92e41 | Clamped the UI's sequence frames; drew nothing for sprites or a missing first image | Wraps the frame through the run, uses the sprite's image, and falls back when the first is −1 |
| a3d09543 | `ai_kill` right after `ai_place` did nothing, since the unit had no vitality yet | `actor_kill` marks the unit to die whatever its state |

The vehicle controllers (fa30f388) match within a relative error of
2.3e-7. The physics maths (5c4be5af) and the device groups (f97d2efb, all
39,360 lines) match exactly.

## What this sets up

The next track, **Coverage 3, data**, starts from 3,615 Missing and 161
Partial with nothing owed, so the ratchet now only falls. The largest
Missing systems are the ones the later tracks own:

| System | Missing | Partial | Track |
| --- | ---: | ---: | --- |
| `ai` | 795 | 17 | 10, AI |
| `hs` | 576 | 8 | 9, scripts and sounds |
| `game` | 565 | 28 | 5, game loop |
| `interface` | 452 | 19 | 7, interface |
| `objects` | 227 | 17 | 4 and 6, units and world |
| `units` | 221 | 29 | 4, units |
| `saved games` | 104 | 2 | 3, data |
| `items` | 102 | 1 | 4, units |

The 44 new Partials are the first work for their tracks: each one names
what it leaves out and the check that will close it.

## Checks

- `census --check`: clean, with the ratchet at the counts above.
- `cargo clippy --workspace --tests -- -D warnings`: clean.
- `cargo nextest run --workspace`: 1,402 tests pass. The tests that need
  game data ran here, with the data present.
