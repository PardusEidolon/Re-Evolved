# Plan: The Halo UI rewrite, a plugin drawn at 4K from `ui.map`

Every task is test first (`workflow.md`). Each phase ends with its
automated checks: the workspace tests, golden frames, `census --check` and
`nix flake check`. The phases run to the end without manual stops, except
the design approval, which the user gives.

## Phase 1: Design
- [x] Task: Redraw the existing mockups as static prototypes in a design artifact, without the launcher boards; add the pause, post-game and loading screens. (canvas v43, https://claude.ai/artifact/MqqSyPQ8J4pcW7i9LwRbk1)
- [ ] Task: The user's approval of every screen. (reopened 2026-10-31: the screens are not in the game yet; the user approves them there)
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Design' (Protocol in workflow.md) (reopened 2026-10-31: waits on the approval)

## Phase 2: Reading the game's UI [checkpoint: e230b4d1]
- [x] Task: Read the widget tree from the game's memory through the read-only views; fuzz the reader. fde71a8e
- [x] Task: Signed distance field fonts from the font tags, built behind the loading screen. 3327ed7d
  - Deviation (2026-10-31): the plugin's menus draw their text with CoreText in Overpass and OpenCE, rasterised at the drawn size (`MenuPass.swift`), as CLAUDE.md allows only those fonts; the font tags' fields stay for the stock text the game still draws (`port/wasmtime/text_tags.c`).
- [x] Task: Conductor - User Manual Verification 'Phase 2: Reading the UI' (Protocol in workflow.md) (automated) 37a0326c

## Phase 3: Drawing and input [checkpoint: 12d45c9c]
- [x] Task: Draw the layout at 4K over the background scene; golden frames of the main menu. 37a0326c
  - With `display.redraw_menus` on, the game renders the ui map's 3D world (the ring, its camera and models) and the plugin draws its layout over it; its letters are rasterised at 4K, sharp at the drawn size. `tools/halo_ui_check.py --main` measures the main menu's items' ink (19% of their box at 4K). The stock path keeps `display.high_res_text`.
- [x] Task: Keyboard navigation and the mouse mapped onto it, driven by tests through the control socket. d4998cee
- [x] Task: Every other screen, each against its approved prototype. 908377aa 37a0326c (`tests/screens.rs` lays out each screen the main menu opens; `tools/halo_ui_check.py` measures each 4K frame's rules, title ink and panel palette: all pass)
- [x] Task: The pause menu, the post-game screens and the HUD's text. de843758 (`tests/text.rs`: the pause menu draws from the atlas; the ui map starts on the main menu in the app)
- [x] Task: No version tag on the main menu (the user's finding, 2026-10-31); the golden frame holds it. e9082658
- [x] Task: With display.redraw_menus on, the game still draws the ui map's 3D world behind the menus, its ring scene, camera and models, and the plugin draws over it; no stock widget art (pictures, text) under the plugin's screens, and no stand-in sky of the plugin's own (the user, 2026-10-31: the background was a stand-in that was not to replace the original's); each screen laid out to the design; `tools/halo_ui_check.py` measures each frame's rules and title ink against the design, and the palette inside the plugin's panel, where the ring shows through only dimmed (the user's finding, 2026-10-31: "text shows but layout and older assets are still mixed"). The plugin draws the on-screen keyboard too; MenuPass keeps a buffer the GPU still reads. 37a0326c
  - The tag is the original's own textbox, filled by `set_textbox_to_build_number` with "01.01.14.2342". A `port:` guard fills it only while `display.build_number` is on (off by default); `tests/menus.rs` reads every text the main menu's widgets are given, with the setting off and on.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Drawing and input' (Protocol in workflow.md) (automated) 37a0326c (workspace tests, census, halo_ui_check on the walk's 4K frames)

## Phase 4: The loading screen [checkpoint: e3c631aa]
- [x] Task: Draw the Xbox loading screen at 4K from the original's code. 325cd5ff (verifies the C device's existing drawing)
- [x] Task: The chant from the user's xbe, silent without it. cfdcf9b2
- [x] Task: No hitch or black frame into the first frame of the next map. 325cd5ff
- [x] Task: Conductor - User Manual Verification 'Phase 4: The loading screen' (Protocol in workflow.md) (automated)
