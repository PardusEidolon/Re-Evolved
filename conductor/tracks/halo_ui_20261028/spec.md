# Spec: The Halo UI rewrite, a plugin drawn at 4K from `ui.map`

## Overview
After the native port, the original's own UI code runs in `game.wasm`: its
widgets, screens, flow and event handlers, read from the user's `ui.map`.
The game still decides everything the menus do. This track redraws them as
a Rust plugin on the HUD and UI hook, at native resolution (4K): the same
screens, the same flow and the same design language, with crisp text, high
quality bitmaps and mouse support. It is a modern rendering of the
original, not a new identity.

It carries on two archived tracks: `halo_ui_20261021` (the widgets from
`ui.map`) and `menu_redesign_20261024` (the mockups in the design artifact
https://claude.ai/artifact/MqqSyPQ8J4pcW7i9LwRbk1). Their launcher part is
dropped: the game starts through the CLI and Nix.

## Functional Requirements

### Design first
1. Every screen is drawn first as a static prototype in a Claude design
   artifact, starting from the existing mockups without their launcher
   boards: the main menu over its scene, campaign and difficulty, saved
   games, multiplayer, settings, profiles, the pause menu, the post-game
   screens and the loading screen. It is built only once the user approves
   it.
2. The prototypes hold only our own drawings and placeholders. Reference
   captures of the original stay in the gitignored `assets/` folder.

### Reading the game's UI
3. The plugin reads the live widget tree from the game's memory through the
   read-only views (`native_port_20261028`, A3): each widget's type,
   bounds, focus, text, bitmap and animation state.
4. Fonts are drawn from the font tags as signed distance fields built
   behind the loading screen, so text is sharp at 4K. Bitmaps are drawn
   with high quality filtering.

### Drawing and input
5. The 640×480 layout is scaled to the window's height and centred, with
   the background scene filling the width. The background scene is drawn
   by the renderer.
6. The keyboard drives the game's own navigation: the arrow keys, Enter,
   Escape and Tab.
7. The mouse is mapped onto it: hovering focuses a widget, clicking selects
   it, the wheel scrolls lists, and right-click goes back. Hit boxes come
   from the widgets' bounds.
8. The pause menu, the post-game screens and the HUD's own text use the
   same drawing.

### The loading screen
9. The Xbox loading screen (`interface/progress_bar.c`, now running as the
   original's code) is drawn at 4K: its background, the panning light, its
   timing and its fade.
10. The monk chant is read from the user's own Xbox `default.xbe` when the
    data folder has it, and played under the screen; without it the screen
    is silent. Bungie's audio is never shipped or written outside the cache.
11. The pipeline archive and the fonts are built behind this screen
    (`native_port_20261028`, A4), with no hitch when it fades into the
    first frame.

## Non-Functional Requirements
- Every task is test first (`conductor/workflow.md`); where a test cannot
  come first (Metal output), the check is a golden frame.
- The plugin keeps to its budget; p99 ≤ 8.33 ms at 4K holds in the menus.
- Every read of the game's memory and of `ui.map` goes through the
  bounds-checked readers, fuzzed.
- Fonts: Overpass and OpenCE only for anything of ours; the game's text is
  drawn from its own font tags. Bungie's and Microsoft's branding is not
  imitated in our own drawings.
- Nothing from the user's data enters the repository, the bundle or an
  artifact.

## Acceptance Criteria
- The user has approved every screen's prototype.
- Every screen the original reaches is drawn at 4K by the plugin, with the
  game deciding its flow; a golden frame of each matches the approved
  design.
- Keyboard and mouse reach everything; the plugin's tests drive each screen
  through the control socket.
- The loading screen plays between campaign maps, with the chant when the
  xbe is present, and fades into the next map without black frames.

## Out of Scope
- A launcher, and any screen the original doesn't have.
- Gamepads.
