# Plan: Coverage 3 of 11, Data fidelity

Every task is test first (`workflow.md`): the trace, measurement or census test is written and seen to fail before the change. Each phase ends with an automated check: the workspace tests, `census --check` with the ratchet, and `nix flake check`. The phases run to the end without manual stops.

## Phase 1: Strides and fix-ups
- [x] Task: Write the failing test comparing every importer stride with `tag_layout.tsv`. 31ff985d
- [x] Task: Fix the strides, starting with the `game_globals` difficulty block. 31ff985d
- [x] Task: Reference traces for the four differing fix-ups, then match them. 13cb5ef9 68c1fd7d (the script fix-up is checked against every stock map's stored nodes rather than a compiled trace)
- [x] Task: Conductor - User Manual Verification 'Phase 1: Strides and fix-ups' (Protocol in workflow.md) (automated): 1410 tests pass, census --check passes, treefmt clean

## Phase 2: Layouts from a second source
- [x] Task: Write tests for `vehicle_definition` and each shader subtype's layout against a synthetic tag.
- [x] Task: Fill the layouts from format documentation and check them on the user's maps where present.
- [x] Task: Fuzz targets for the new readers.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Layouts from a second source' (Protocol in workflow.md) (automated)

## Phase 3: Fields
- [x] Task: Wire or class each of the 516 kept-but-unused fields, one commit per tag group. f17c00e7 9f6dffb3 (once the layouts were filled there were 597; 280 are traced through locals or by hand, the 13 finished functions reading 26 dropped fields are Partial, and the 291 left are classed by reason in `unused_fields.tsv`; in two commits, since the census checks every group at once)
- [x] Task: Trace each of the 151 untraced fields. f17c00e7 (198 once the layouts were filled; 0 now)
- [x] Task: Conductor - User Manual Verification 'Phase 3: Fields' (Protocol in workflow.md) (automated) 9f6dffb3: 1432 tests pass, census --check clean, clippy and treefmt clean

## Phase 4: Cache and bitmaps
- [x] Task: Port, match or class the cache and bitmaps functions. (already done by the evidence track at 1c398cf4: cache 187 Replaced, bitmaps 1 Matched and 209 Replaced, nothing owed; the ratchet leaves both out, which counts as zero)
- [x] Task: Conductor - User Manual Verification 'Phase 4: Cache and bitmaps' (Protocol in workflow.md) (automated): census --check clean

## Phase 5: The report
- [x] Task: Run `census` and record the counts before and after this track, per system and per class. (report.md)
- [x] Task: Write the data-backed report and lower the ratchet to the new counts. (report.md; the ratchet already holds the counts after, raised by hand once in f17c00e7, and `census --check` fails on any count below it)
- [x] Task: Send the user the report; it sets up the next track.
- [x] Task: Conductor - User Manual Verification 'Phase 5: The report' (Protocol in workflow.md) (automated): census --check, 1432 tests and nix flake check pass at 9f6dffb3
