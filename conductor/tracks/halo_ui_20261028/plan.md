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

## Phase 5: Every option the game's own (the user, 2026-10-31: "make this feature complete and the options work")
- [x] Task: Each function the PC version's screens run is the Xbox's or answered by `menu_functions.c`; `tools/menu_functions_check.py` writes `port/assets/menus/UNWIRED.md` from the files and `--check` (a flake check) fails when it is stale. None is left on a screen a player reaches; four are on screens nothing opens (Direct IP, the browser's Filters).
- [x] Task: Load Game's list disposed with its screen ("load game menu dispose").
- [x] Task: LAN and the browser without a network show the game's own error (5), not an assert (`port:` in `ui_widget_event_handler_functions.c`).
- [x] Task: The server browser's column titles sort its games.
- [x] Task: Video Setup's OK saves the mode, keeps it, and asks the app to apply it (`platform_display_apply`, `runtime/src/display.rs`): full screen or a window of the sizes that fit the screen, and vsync.
- [x] Task: A headless walk at 4K of every main menu item and the screen under it, and the campaign's chain to its game (`tests/menus.rs` drives the same over the socket).
- [ ] Task: The user's review of the whole UI at the end (Phase 1's approval).

## Phase 6: The second pass (the user, 2026-11-01: "Include the cursor sweep perform and second pass")
- [x] Task: The cursor lands on what it points at at any window size: the plugin gives the input the rectangles it drew each item in, and the game's menus (`port:` in ui_widget.c) ask for the one under the pointer rather than the 640×480 place its tags give (`runtime/src/redrawn.rs`, `input.rs`, `halo_ui_pointer.h`; `the_pointer_finds_what_a_plugin_drew_under_it`).
- [x] Task: Settings as tabs down the left and a pane on the right, its rows ◀ value ▶, each value the game's own (`screens.rs`; `tests/screens.rs` reads Audio Setup's device and master volume).
- [x] Task: The campaign as a carousel of its missions, each with its picture from the user's ui.map, which the game draws where the plugin placed it, at run time; nothing of it is written to the repository.
- [x] Task: The loading screen: the original's band of light (progress_bar.c's timing, mask and brightness) sweeps with the game's own load, read from `game_loading_port_view` (`port:` in game.c's precache loop, which also runs the frame hook), then a 0.5 s crossfade into the mission (`loading.rs`, `tests/loading.rs`; a headless clip of a10's load).
- [x] Task: Replay Level's map kind row, and the ABOUT pane.
- [x] Task: Conductor - User Manual Verification 'Phase 6: The second pass' (Protocol in workflow.md) (automated) (workspace tests, clippy, census, menu_functions_check, `nix build .#engine`, `nix flake check`; walks at 3840×2160 and 2560×1600)

## Phase 7: The third pass (the user, 2026-11-01: "I still feel we need to make a third pass"; run through conductor, the Halo UI track alone)
- [x] Task: Find why choosing a mission from the campaign crashes the game, and fix it; a test drives the chain from Replay Level's mission to its game over the socket. 51751b1d
- [x] Task: Enter takes the focused item: it no longer goes back a menu and then forward again; OK and BACK in the footer take a click. 51751b1d
- [x] Task: The settings belong to no profile: Settings opens without one, and the game's own values are kept apart from the profiles. 8aecefa4
- [x] Task: Video as one tab, its rows the design artifact's (mode, window size, vsync, the drawing's settings). 741b9d7f
- [~] Task: No dummy switch: each row changes what it names in the game (physics, mouse sensitivity, video and the rest), or is taken off; a test sets each and reads the game's value back.
- [ ] Task: No colon after a label; the menus' text wraps inside its box.
- [ ] Task: The carousel's pictures aligned inside their cards.
- [ ] Task: The loading screen blue; the first campaign map loaded from the menu sweeps smoothly across for at least 5 s.
- [ ] Task: Conductor - User Manual Verification 'Phase 7: The third pass' (Protocol in workflow.md) (automated) (workspace tests, clippy, census, menu_functions_check, `nix build .#engine`, `nix flake check`; a headless walk at 4K into the review pack)
