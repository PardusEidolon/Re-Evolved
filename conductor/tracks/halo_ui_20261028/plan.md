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
- [ ] Task: Signed distance field fonts from the font tags, built behind the loading screen. 3327ed7d (reopened 2026-10-31: the game draws its text from text_tags.c, not these fields)
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Reading the UI' (Protocol in workflow.md) (automated) (reopened 2026-10-31: the fonts are reopened)

## Phase 3: Drawing and input [checkpoint: 12d45c9c]
- [ ] Task: Draw the layout at 4K over the background scene; golden frames of the main menu. de843758 (reopened 2026-10-31: no redraw plugin draws the layout; the app's 4K letter edges are soft)
  - Deviation: the original's own code draws every screen over its own background scene; the track's Rust redraw plugin was not needed. Its text is sharpened instead: each glyph is made from its font tag's own character at the display's pixels (`port/wasmtime/text_tags.c`, `display.high_res_text`, on by default). `ui_main_menu_3840x2160.png` is approved.
  - Measured (2026-10-31): in the app's 4K audit the menu's letter edges ramp over about 10 pixels with the setting on and off alike, while the runtime's and the FFI's tests see the atlas made and filled at 4K. The app's offscreen path likely picks its screen scale before its drawable is 4K; left recorded, not fixed.
- [x] Task: Keyboard navigation and the mouse mapped onto it, driven by tests through the control socket. d4998cee
- [ ] Task: Every other screen, each against its approved prototype. 908377aa (each main menu item opens its own screen, the original's) (reopened 2026-10-31: no screen is drawn from its prototype; the test only opens the original's)
- [x] Task: The pause menu, the post-game screens and the HUD's text. de843758 (`tests/text.rs`: the pause menu draws from the atlas; the ui map starts on the main menu in the app)
- [x] Task: No version tag on the main menu (the user's finding, 2026-10-31); the golden frame holds it. e9082658
  - The tag is the original's own textbox, filled by `set_textbox_to_build_number` with "01.01.14.2342". A `port:` guard fills it only while `display.build_number` is on (off by default); `tests/menus.rs` reads every text the main menu's widgets are given, with the setting off and on.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Drawing and input' (Protocol in workflow.md) (automated) (reopened 2026-10-31: its tasks are reopened)

## Phase 4: The loading screen [checkpoint: e3c631aa]
- [x] Task: Draw the Xbox loading screen at 4K from the original's code. 325cd5ff (verifies the C device's existing drawing)
- [x] Task: The chant from the user's xbe, silent without it. cfdcf9b2
- [x] Task: No hitch or black frame into the first frame of the next map. 325cd5ff
- [x] Task: Conductor - User Manual Verification 'Phase 4: The loading screen' (Protocol in workflow.md) (automated)
