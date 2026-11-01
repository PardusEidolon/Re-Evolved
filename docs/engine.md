# The engine

OpenCE Engine is a macOS app that plays the original game's own code.
`source/` is compiled unchanged, as wasm32, into `game.wasm`
(`tools/engine_wasm.py`), and Wasmtime runs it inside the app. The game
calls only the Rust import list: the macOS platform layer and the plugin
hooks. Swift and Metal draw what the game's Direct3D 8 device asks for.
Why the project took this direction is in
`docs/retrospective_fork_in_the_road.md`; the plan is
`conductor/tracks/native_port_20261028`.

## Building and testing

```sh
nix build .#engine             # OpenCE Engine.app
nix run .#engine               # build it and start the game on its main menu, from the folder holding assets/
nix run .#engine -- --map bloodgulch   # or start on a map
python3 tools/engine_wasm.py   # game.wasm alone
python3 tools/engine_build.py --out build/engine --test   # the app and its Swift tests
cd engine && cargo test --release --workspace             # the Rust tests
cd engine && cargo clippy --release --workspace --all-targets -- -D warnings
python3 tools/engine_wasm_traces.py --check   # game.wasm against every reference trace
```

Run the commands after the `nix` ones in `nix develop`. Tests that need
the user's data (`assets/maps`, `Haloce_dump`) skip themselves without it.
`unsafe` code is refused, every public item needs a document comment, and
`unwrap` and `expect` are refused outside tests.

## The architecture

`game.wasm` is the original's code with its 32-bit pointers, so every tag
layout and every pointer in a map stays as the original has it. The Rust
crates (`engine/crates/`) run it and answer its imports; the Swift app
(`engine/app/`) gives it a window, input, sound and Metal.

| Crate | What it does |
| --- | --- |
| `runtime` | Wasmtime with Cranelift: loads and caches `game.wasm`, answers its imports (files, threads, clocks, input, sound, the network, the console), runs the plugin hooks, profiles the game's own sections, records and replays a run |
| `render` | The render commands the game's Metal device writes (`opence_render.h`), decoded and checked batch by batch (`RenderValidator`) before Metal sees them |
| `ffi` | The contract with Swift, through UniFFI: `WasmGame` (start, input, the control socket, each frame's batches), the loading screen, the benchmark's report |
| `census` | The coverage census (below) |
| `content`, `import` | The importer's typed content and its bounds-checked readers of a stock cache |
| `bake` | The level's light baked again, for the lighting track |
| `ui` | The shell's widgets, for the Halo UI plugin |
| `dev` | The registry of commands and variables, and the control socket's limits |
| `cli` | The `opence` tool |

The Swift app has these modules:

| Module | What it does |
| --- | --- |
| `OpenCEEngineCore` | The UniFFI bindings (generated) |
| `OpenCEEngineKit` | Keyboard and mouse input, sound output, the map library |
| `OpenCEEngineRender` | `GameRenderer` (the game's batches on Metal), the pipeline archive, captures, GPU timings, the benchmark |
| `OpenCEEngineWindow` | The window and its view |
| `OpenCEEngine` | The app's entry and `GamePlayer` |

The game runs as the original does: its simulation at 60 Hz and its
scripts at 30 Hz. `game.wasm` uses musl's maths and no fused
multiply-add, so the same inputs from the same state give the same world.

### Plugins

A plugin is Rust, called at hooks the game's own code already marks: the
tick, AI, HUD, UI and frame hooks follow the exits of the game's profile
sections, the render hook follows each batch the Metal device submits,
and the sound hook follows the mixer's output (`runtime/src/plugins.rs`).
No line of `source/` changes for them. A plugin reads the game's memory
only through read-only, bounds-checked views of its data arrays, its tag
table and its tag blocks (`runtime/src/views.rs`), which answer `None`
wherever the game's own checks would refuse; the views are fuzzed. A
plugin's share of the frame is kept within its budget, and with every
plugin off the reference traces match as they do without plugins.

## Drawing

`game.wasm` keeps the original's rasterizer and draws through a Direct3D 8
device written for it in C (`port/wasmtime/metal/d3d8_metal_device.c`). The
device writes Metal-shaped commands into batches; the runtime decodes and
checks each batch, and `GameRenderer` carries it out.

The game's shaders are the original's, translated as they are met: the
vertex and pixel shader microcode becomes Metal Shading Language
(`nv2a_vsh.c`, `nv2a_psh.c`), per program and per map. So no shader can be
compiled when the app is built. Instead each pipeline is compiled once and
kept in a Metal binary archive (`pipelines.metallib` in the cache), and a
later run finds it there, so play never waits on Metal's compiler. The
Wasmtime cache keeps the compiled `game.wasm` the same way.

The lighting is the original's first. Each extension of it (the
`lighting_original` track) is switchable back to stock.

## The benchmark

`OpenCEEngine --benchmark SECONDS` plays `--map` (bloodgulch) offscreen
at `--size` (3840×2160) for `--load-seconds` (20) and then SECONDS, and
prints a JSON report. A frame's time is the slower of the game's CPU
frame (from its own profile sections, the 30 Hz throttle left out) and
its GPU work. `--baseline` with a report kept before fails on a
regression. The bar is p99 ≤ 8.33 ms on the M1 Max at 3840×2160, ultra.

## The control socket

With `--control` (beside the window) or `--headless` (with no window, drawn
offscreen at `--size`), the app listens on a Unix socket, `opence/control.sock`
in the user's runtime folder, or at `--socket PATH`. Each command line a
client sends is handed to the game's own telnet console, so `opence run
<line>` reaches the original's script compiler as a line typed at its
console does. A request is one JSON line and so is each reply.

```sh
opence run '(game_speed 2)'   # one line (--json for the data)
opence shell                  # an interactive shell
```

The game's keyboard and mouse are driven through the same socket, so a
test (or a script) works the menus as a player does:

```sh
opence key 40                 # a key pressed: down, held 50 ms, up (--down, --up)
opence pointer 320 200        # the menus' pointer, in the window's units
opence click 320 200          # a click there (--button 2 for the right one)
opence wheel -1               # the wheel, a notch down
opence focus                  # the screen shown and its focused widget, as JSON
```

A key request names its `down` explicitly: the menus read held keys, so a
press and release sent together, between two of the game's frames, would
not be seen. `runtime/tests/menus.rs` drives the real main menu this way.

## The coverage census

The census (`engine/crates/census`, `opence census`) gives every function
of the original (`source/`) a class and checks the evidence each class
needs:

- **Ported**: line for line, compiled into a reference trace by a
  `tools/engine_*_reference.py` script (listed in `engine/census/traced.tsv`),
  or with the reason it has none and a traced caller that covers it.
- **Compiled**: compiled unchanged into `game.wasm`: its file is one the
  game's build compiles (`port/linux/port.json`).
- **Matched**: rewritten, with a check against the original (a reference
  trace, a golden frame or a measurement).
- **Native**: a helper whose job a Rust primitive does, named.
- **Partial**: what is done has evidence, and what it leaves out is listed.
- **Replaced**: on purpose, for a listed reason (Xbox-only hardware,
  keyboard and mouse, the tools), with what stands in for it named.
- **Missing**: not started.

`engine/census/census.toml` holds the rules and entries. `opence census
--write` writes `traced.tsv`, `docs/census.md` (every function, by
system) and the table below; `opence census --check` fails when a class
lacks its evidence, when a doc or `traced.tsv` is stale, and when any
trace is stale. The census also marks each tag structure's fields
(`engine/census/tag_fields.tsv`) with whether the original reads them and
whether the importer keeps them.

<!-- coverage census: opence census --write writes this table -->
| System | Ported | Compiled | Matched | Native | Partial | Replaced | Missing | Owed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `ai` | 45 (1936 B) | 914 (294330 B) | | | | | | 0 |
| `bink` | | 24 (3968 B) | | | | | | 0 |
| `bitmaps` | 1 (224 B) | 137 (61392 B) | | | | 72 (2512 B) | | 0 |
| `bungie_net` | | 105 (18992 B) | | | | | | 0 |
| `cache` | | 187 (24784 B) | | | | | | 0 |
| `camera` | 5 (528 B) | 116 (36016 B) | | | | | | 0 |
| `cseries` | 1 (16 B) | 139 (21824 B) | | | | | | 0 |
| `cutscene` | | 65 (10528 B) | | | | | | 0 |
| `devices` | 21 (2176 B) | 39 (5008 B) | | | | | | 0 |
| `dialogs` | | 5 (1328 B) | | | | | | 0 |
| `editor` | | 9 (0 B) | | | | | | 0 |
| `effects` | 13 (2864 B) | 177 (53744 B) | | | | | | 0 |
| `game` | 81 (14432 B) | 635 (89648 B) | | | | | | 0 |
| `hs` | 15 (944 B) | 624 (66048 B) | | | | | | 0 |
| `input` | | 47 (7216 B) | | | | | | 0 |
| `interface` | 8 (1904 B) | 670 (154776 B) | | | | | | 0 |
| `items` | 12 (4256 B) | 125 (31792 B) | | | | | | 0 |
| `main` | | 126 (15840 B) | | | | | | 0 |
| `math` | 65 (8016 B) | 144 (15152 B) | | | | | | 0 |
| `memory` | 18 (2080 B) | 202 (33216 B) | | | | 12 (0 B) | | 0 |
| `models` | 21 (10000 B) | 27 (7568 B) | | | | | | 0 |
| `networking` | | 288 (49824 B) | | | | | | 0 |
| `objects` | 10 (5040 B) | 337 (46544 B) | | | | | | 0 |
| `physics` | 44 (12080 B) | 101 (35152 B) | | | | | | 0 |
| `rasterizer` | 1 (144 B) | 779 (185056 B) | | | | | | 0 |
| `render` | 4 (832 B) | 115 (37520 B) | | | | | | 0 |
| `saved games` | | 137 (24992 B) | | | | | | 0 |
| `scenario` | 12 (4048 B) | 45 (5536 B) | | | | | | 0 |
| `shaders` | | 21 (2624 B) | | | | | | 0 |
| `shell` | | 13 (528 B) | | | | | | 0 |
| `sound` | 14 (2416 B) | 158 (24544 B) | | | | | | 0 |
| `structures` | 2 (0 B) | 124 (31712 B) | | | | | | 0 |
| `tag_files` | | 48 (4288 B) | | | | | | 0 |
| `text` | | 112 (16912 B) | | | | | | 0 |
| `tool` | | 27 (8752 B) | | | | | | 0 |
| `units` | 165 (26944 B) | 164 (80576 B) | | | | | | 0 |
| **all** | 558 (100880 B) | 6986 (1507730 B) | | | | 84 (2512 B) | | 0 |
<!-- end of the coverage census -->
