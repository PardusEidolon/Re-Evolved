# Plan: The map and cutscene audit, every map played against stock

Every task is test first (`workflow.md`). Each phase ends with its
automated checks: the workspace tests, the reference traces, `census
--check` and `nix flake check`. The phases run to the end without manual
stops.

## Phase 1: The runs [checkpoint: 8e50dfc9]
- [x] Task: Record inputs that play each map to its end, and play them headless through the `opence` tool. 37fe41b3
  - No scripted input plays a map to its end: each map is played by a fixed schedule of keys and mouse for a fixed time (120 s a campaign map, 40 s a multiplayer one), by `tools/engine_audit.py` through the app's `--audit` mode.
- [x] Task: Play each cutscene in full. 2919dc00
  - `opence scripts MAP` lists them; each is called (or woken) through the console with `--at`, and is over when its console thread ends.
- [x] Task: Record frames at fixed ticks, the sound mix, the script events, the frame times and the faults, under the cache folder. 2919dc00
  - Under `build/engine/audit` (local only): a frame each 300 ticks, `sound.tsv` from the first tick, `events.tsv`, `frame_ms.tsv`, `summary.json`.
- [x] Task: Conductor - User Manual Verification 'Phase 1: The runs' (Protocol in workflow.md) (automated)

## Phase 2: Comparing with stock [checkpoint: 262bbe06]
- [x] Task: Play the same inputs with the stock rendering and sound. 420fd7a7
    - Every map and cutscene played three times on the frame clock: stock, stock2 and shipped (76 runs each).
- [x] Task: Image differences per region, the sound's loudness and spectrum, the script events in order; thresholds set from repeated stock runs. 2919dc00
- [x] Task: List the findings with their map, tick, frames and measurement. 420fd7a7
    - 20 findings over 13 runs (build/engine/audit/findings.tsv, local only): 5 region, 9 loudness, 5 band, 1 event shift.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Comparing' (Protocol in workflow.md) (automated)

## Phase 3: Fixing and iterating [checkpoint: c3dc5c65]
- [x] Task: Trace each finding to its cause and fix it, each with a failing test first. 7c9b0d5d 97a7f349 46a606e6 (console lines on the game tick; frame clock on whole ms in threes; a run played again starts empty)
- [ ] Task: Run every map and cutscene again until they pass, or list what remains with its reason. (run 30 Oct: 228 runs, 0 failed; 76 compared, 93 findings) (reopened 2026-10-31: stock and stock2 part after about 40 s, so 59 of 76 runs have loose thresholds)
    - 70 `fault` findings are the audit bundle, not the game: it was built while the LIGHTS command (lighting Phase 2) was half in, so its host refused exactly one command a frame in every variant (refused = frames, nothing drawn differently). The current build refuses 0 (bloodgulch, 389 frames).
    - 23 findings are shipped against stock, which they should differ by: shipped turns on interpolation, the high-res HUD and text, per-pixel lighting, reverb and the enhanced animations. Sound (`dbfs`, `band`) is reverb; the regions are the HUD and lighting.
    - Remains: stock and stock2 repeat for about 40 s and then part. Worst region per kept frame: a10 0 throughout; a30 bridge ≤ 7 to tick 2100, then 17, 55 and 213 by tick 4200; c20 ≤ 4 to tick 2400, then 128 by 3300; b40 insertion_b spikes of 31–116 from tick 1500. The early differences are particles (`local_random`) and water. The later ones are effects, not play: `state.tsv` hashes the game state and each data array every tick, and two a30 stock runs keep objects, actors, players, encounters and scripts identical for all 2069 ticks (69 s), while particles and effects part at tick 426, contrails at 781, decals at 805 and the lights effects make at 1991. Those are stepped by rendered frames (`local_random`, frame time), as in the original, so the split is the original's own and is accepted (the c10 lighting runs' split after tick 2400 is the same). 59 of 76 runs have stock noise above the 4/255 floor, so their thresholds are loose.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Fixing' (Protocol in workflow.md) (automated) (reopened 2026-10-31: its run is reopened)

## Phase 4: The report [checkpoint: dd0f29c0]
- [x] Task: Write the report from the data: the findings, their causes and fixes, what remains, and p99 per map. bd7d4744
    - Written to `build/engine/audit/report.md` (local only, as the spec asks). 228 runs, 0 failed; 93 findings, all explained (70 the stale bundle, 23 the shipped enhancements); stock and stock2 part after ~40 s, cause not traced.
    - p99 at 3840×2160, 30 s a map: no map meets 8.33 ms. Multiplayer 8.79 (prisoner) to 11.29 ms (chillout), bloodgulch 9.11; campaign 11.13 (d40) to 34.33 ms (b30), a30 22.05, c10 19.51.
- [x] Task: Conductor - User Manual Verification 'Phase 4: The report' (Protocol in workflow.md) (automated)
