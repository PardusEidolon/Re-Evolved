# Spec: Coverage census

## Overview
The engine names 177 of the original's 6,962 functions. Many of the rest
need no line-for-line port: the engine does the same job its own way, uses
a native Rust primitive in its place, or replaced it on purpose. This track
sorts every function in `source/` into one class, sets what each class must
show, and builds a census in Rust whose check keeps it current. The census
covers the Rust crates and the Swift app, and the data the original reads:
scripts, every tag field and the tags' load-time fix-ups, and sounds. It
ends with a data-backed report that becomes the plan for the next long
track, coverage completion. This track ports nothing itself.

## The classes and what each must show
1. **Ported** (line for line). Must show a reference trace that compiles
   it, or a recorded reason plus a named reference trace of a caller that
   covers it.
2. **Matched** (rewritten; the behaviour must match, the code need not).
   Must show the engine item that does its job and a check against the
   original itself: a reference trace of its result, or a measurement or
   golden comparison with the stock game. A test of the engine alone is
   not enough.
3. **Native** (a helper the engine doesn't port: math, cseries, data
   arrays, lists, memory and strings). Must name the Rust or crate
   primitive used in its place and keep the determinism rules:
   - numeric helpers are checked by the math reference trace (libm, no
     fused multiply-add);
   - containers the simulation iterates keep a fixed order.
4. **Partial** (started, not finished). Must list the branches, cases or
   flags it leaves out, with Ported or Matched evidence for the part that
   is done.
5. **Replaced** (on purpose; no behaviour to match). Must give one reason
   from a fixed list: the enhanced lighting and Metal renderer, Xbox-only
   hardware, keyboard and mouse only, maps imported to a typed format, the
   editor and tools, networking and Xbox Live, Bink. Must name the Rust or
   Swift code that stands in for it, and note any behaviour visible to the
   player that has to live on elsewhere.
6. **Missing** (not started). Records its size, its system, and whether
   the played game reaches it.

## Functional Requirements
1. **The census crate** (`engine/crates/census`) and an `opence census`
   command:
   - Parses every `source/` file's symbol header (address, size, name).
   - Collects the citations from all Rust crates and the Swift app's
     Sources.
   - Builds a call graph from `source/`.
   - Reads the classes from `engine/census.toml`: rules by directory, file
     or name, plus entries for single functions.
   - Writes `docs/census.md` (every function and tag field, by system) and
     a summary table in `docs/engine.md`.
   - Replaces `tools/engine_reference_map.py`. The Python trace scripts
     stay, since they compile `source/` with clang. `opence census --check`
     runs their `--check`, and the cargo test checks that each cited trace
     exists.
   - Reads through bounds-checked readers, and passes the workspace's
     lints.
2. **Reachability.** Starting from the main loop of a campaign and a
   multiplayer game, the call graph marks what the played game reaches. A
   function reached only from Replaced code is Replaced through its caller.
3. **The check** is a cargo test, so nextest and `nix flake check` run it.
   It fails when:
   - a function has no class;
   - a class's evidence is missing, or a cited test, trace or primitive
     doesn't exist;
   - a Partial entry lists nothing left out;
   - a reason is left over for a function that is now traced;
   - a tag field has no marks;
   - the docs are stale.
4. **The walk through the systems:** AI, scripts, game,
   objects/units/items/devices, physics, effects, sound, models/animation,
   camera/cutscene, interface/HUD and UI, input, scenario/structures, saved
   games, main/shell, math/cseries/memory. For each one, read the Rust and
   Swift sides, add the citations they lack (today's count is a floor), and
   class every function with its evidence.
5. **Tag fields, every field of every tag group:**
   - The layout (name, type, offset) comes from `source/`'s
     `*_definitions.h`, laid out by clang (`offsetof`).
   - Each field gets three marks: read by the original (its functions
     access it), kept by the importer, and used by the engine (the typed
     field is read outside the importer).
   - The importer's readers register the offsets they read, labelled with
     field names, in place of bare offset constants. The census takes the
     "kept" mark from that registration.
   - **Load-time fix-ups:** what the original's tag loading fills in,
     clamps or computes (postprocess and defaults), and whether the
     importer does the same.
6. **Scripts and sounds:**
   - every script function and global, and whether the engine wires it;
   - every sound tag, and whether play asks for it (today 77 of 321), plus
     the per-object sound limit.
7. **The report:** an Artifact with:
   - numbers per system and class, in bytes as well as counts;
   - the tag fields read and dropped, kept and unused, and the fix-ups
     that differ;
   - the Missing, Partial and evidence-less items, ranked by how much of
     play depends on them;
   - the proposed phases of the coverage-completion track.

## Non-Functional Requirements
- Reads `source/` only, never `xbox/include`. Puts no game data in the
  repository. The tag fields come from `source/` and the importer's code,
  not from the user's maps.
- Deterministic output. The census check finishes in under 10 seconds.
- Tests are written first, against small fake source, Rust and Swift
  trees.

## Acceptance Criteria
- All 6,962 functions have one of the six classes, each with the evidence
  its class asks for.
- Every field of every tag group has its three marks, and every fix-up is
  compared.
- The census check passes under nextest and `nix flake check`. The old
  reference map's checks still hold.
- The script and sound lists are complete.
- The report is published, and the next track's phases are proposed from
  it.

## Out of Scope
- Porting or fixing anything. That is the completion track.
- The map plays and cutscene audit of `coverage_audit_20261021`.
- Line coverage of the crates, which `tools/engine_coverage.py` already
  measures.
