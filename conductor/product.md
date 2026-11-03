# Product Guide: OpenCE (macOS)

## Vision
Bring Halo: Combat Evolved to modern macOS as a native game that is the
original's own code. **From 2026-10-27** the decompilation (`source/`) is
compiled unchanged as wasm32 and run in Wasmtime inside a native macOS app
(track `native_port_20261028`), so the game plays as the Xbox original
does. Around it the app is modern: native resolution at any aspect ratio,
4K, a keyboard-and-mouse interface built as a plugin, and lighting
extended from the original's own. The reasons are in
`docs/retrospective_fork_in_the_road.md`.

## Target Platform
- **macOS only** (Apple Silicon).
- Run through the CLI and Nix (`nix run .#engine`); there is no launcher.

## Target Users
- **Mac players** wanting Halo: CE running natively and fast on modern
  Apple hardware, as the original plays.
- **Contributors and preservationists** interested in a clean, CC0 port
  of the decompilation.

## Core Value
- **The original's game:** movement, weapons, vehicles, AI, scripts and
  sound are the original's code, checked against the natively compiled
  reference by traces.
- **Native and fast:** Metal on Apple Silicon; p99 ≤ 8.33 ms at 4K ultra.
- **Extended, switchably:** each extension (lighting, the HUD and menus at
  4K) can be switched back to stock and is measured against it. The
  Gearbox PC port's losses are fixed against the Xbox original.
- **Free and open:** CC0; game data from the user's own copy; no
  commercial assets shipped.

## Architecture
- **The game:** `source/` as `game.wasm`, run in Wasmtime, compiled ahead
  of time with Cranelift.
- **The import list (Rust):** the macOS platform layer (memory, files,
  threads, input, sound, the render calls) and the plugin hooks. Plugins
  are Rust and read the game through read-only, bounds-checked views.
- **The app (Swift):** the window, keyboard and mouse, sound output, and
  Metal drawing the rasterizer's buffer.

## Roadmap
1. `native_port_20261028`: the original running in Wasmtime, the platform
   layer, the plugin interface, stock rendering on Metal, the presentation
   as plugins, the PC port's losses, then the old engine removed. A1's
   go/no-go decides whether it carries on.
2. `halo_ui_20261028`: the Halo menus and HUD rewritten as a plugin, at 4K,
   for keyboard and mouse.
3. `map_audit_20261028`: every map and cutscene compared with stock.
4. `lighting_original_20261028`: placed lights, bounce and probes, the sky
   and atmosphere, and ray tracing, from the original's lighting.
5. Later: the living world, kev-4b and the netcode rework.

## Non-Goals
- Shipping game data or commercial fonts or art.
- OpenSauce feature support.
- Reading from or copying the Xbox SDK headers.
- Linux, Windows, Android, a browser build, or gamepads.
