# Spec: The map and cutscene audit, every map played against stock

## Overview
Once the native port runs the original's own code and the Halo UI is
drawn, every map and every cutscene is played headless and compared with
the stock game, so that what the user sees anywhere in the game matches it
before the lighting is extended. It carries on the archived
`coverage_audit_20261021`: the coverage runs per map, the cutscene audit,
the fixes, and iterating until the review is clean.

With `source/` compiled as it is, most differences will be in the
presentation (the shaders, the plugins, the platform layer), not in the
gameplay; the audit finds them by measurement, wherever they are.

## Functional Requirements

### The runs
1. Each campaign map and each multiplayer map in the user's data folder is
   played headless from recorded inputs: through its encounters, its
   scripted events and its checkpoints, to its end.
2. Each cutscene is played in full, with its camera, animation, sound and
   subtitles.
3. A run records frames at fixed ticks, the sound mix, the script events,
   the frame times and any warning or fault, and saves them on the user's
   machine under the cache folder.

### Comparing with stock
4. The same inputs are played in the stock game (the original's code, drawn
   as the original draws it, from A4's stock rendering), and each frame and
   sound window is compared: image differences per region, the sound's
   loudness and spectrum, the script events in order.
5. A difference past its threshold is a finding, with its map, tick,
   frames, the region and the measurement.

### Fixing and iterating
6. Each finding is traced to its cause and fixed, test first, with a test
   that fails before the fix.
7. The runs are repeated until every map and every cutscene passes, or what
   remains is listed with why it is accepted.
8. A report, made from the data, sums up the runs: the findings, their
   causes and fixes, and what remains. It sets up the lighting track.

## Non-Functional Requirements
- The runs are headless and never use the user's desktop.
- The runs and comparisons are scripted through the `opence` tool, so any
  one can be run again alone.
- The recorded frames, sounds and reports stay on the user's machine; none
  enters the repository or an artifact.
- p99 ≤ 8.33 ms at 4K is measured on every map and listed in the report.

## Acceptance Criteria
- Every map and every cutscene in the user's data has a passing run, or an
  accepted difference with its reason.
- Each fix has its test, and the workspace tests pass.
- The report is written from the data.

## Out of Scope
- Extending the lighting, which is the next track.
- Changes to the gameplay beyond fixing what differs from stock.
