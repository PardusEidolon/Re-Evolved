# Notes for agents

This repository brings the Halo: Combat Evolved decompilation (Xbox build
01.01.14.2342, `cachebeta.exe`) to macOS as a native app. The game's C
sources are the decompilation. They are compiled unchanged as wasm32 and
run in Wasmtime inside the app. Under the game is a typed import list in
Rust, the macOS platform layer and the plugin hooks. Swift and Metal draw
what the game's rasterizer asks for. The project is published as OpenCE.

`CLAUDE.md` holds the project's working practices and comes first where
the two differ. Start with [README.md](README.md); the other documents are
listed at the end.

## Layout

| Path | Contents |
| --- | --- |
| `source/` | The game: 466 C files of the decompilation, compiled as they are with few changes (each marked, below) |
| `engine/crates/` | The Rust workspace: the importer (`import`, `content`), the census (`census`), the dev tools (`dev`, `cli`: the `opence` tool), the contract with Swift (`ffi`), and the Wasmtime runtime and import list once Track A adds them |
| `engine/app/` | The Swift app: the window, input, Metal, sound and the dev panel |
| `engine/census/` | The census's data: every function of the original and its class |
| `engine/fuzz/` | The readers' fuzz targets |
| `nix/` | The flake's cells (`repo`, `engine`) |
| `tools/` | `engine_build.py`, the reference scripts (`engine_*_reference.py`), the golden frames (`engine_golden.py`), the web build's flags (`web_build.py`, which `game.wasm` starts from) and tests |
| `port/` | The old ports, kept as reference for what the game expects of its host: `port/linux`, `port/android`, `port/web` and `port/windows`. `port/wasmtime` is `game.wasm`'s platform layer and Metal device. `port/third_party/` holds vendored libraries |
| `conductor/` | The tracks: plans, specs and the workflow |
| `docs/` | Design notes: `engine.md`, `census.md`, `retrospective_fork_in_the_road.md`, `custom_edition_caches.md` |
| `assets/` | Game data for local runs (gitignored) |

Removed by Track A's last phase: the Rust simulation (`sim`, `campaign`,
`script`, `physics`, `net`, `bevy_ecs`) and `port/macos`, the
Hypervisor.framework VM. Do not bring them back.

## Building

```sh
nix develop                          # the shell to work in
python tools/engine_build.py         # the Rust workspace and the Swift app
python tools/engine_build.py --test  # the Swift app's XCTests
nix build .#engine                   # the app, as it ships
nix run .#engine -- --map devgulch   # run it
nix flake check                      # keep this passing
```

- **wasm32.** `game.wasm` is ILP32, as the Xbox was: the tags' 4-byte
  pointers and the size asserts hold. A call must match its callee's type
  exactly (the link fails on a mismatch); the variadic prototypes header
  from the web build gives the game's variadic functions their real types.
- **Maths.** `game.wasm` is compiled with the vendored musl maths and
  without fused multiply-add, so every run computes the same results. Do
  not call the system's maths functions.
- **Ahead of time.** The metallib and `game.wasm` are built by Nix; the
  `.cwasm` (Cranelift) and the Metal binary archive are made behind the
  loading screen and cached in the user's cache folder.

## Testing

```sh
nix develop -c cargo test --manifest-path engine/Cargo.toml --workspace
nix develop -c python3 tools/engine_build.py --test --suite swift
nix develop -c cargo run -q --manifest-path engine/Cargo.toml -p opence_cli --bin opence -- census --check
python tools/engine_<name>_reference.py --check   # each trace is current
```

- Tests that need the user's data skip themselves when it is missing.
- Run the game headless only, never on the desktop of the person you work
  for, and stop only the processes you started.

## Rules of the project

### The game's sources

- Change `source/` as little as possible, and mark each change from the
  original with a comment that starts `port:`. Write like the surrounding
  code: its naming, its comment density, and the project's plain,
  descriptive comment style.
- The game reaches the platform only through the import list. A function
  the game calls that the platform provides is an import, declared once,
  with the type the game calls it with.
- Notes such as `BUG (original, preserved for exact matching)` mark the
  original's bugs. Fixing one is allowed; say so in a `port:` comment.

### The Xbox SDK

`port/include/xdk` was written without the SDK's headers, from the January
build's debug information and public documentation. Never read the SDK's
headers or anything derived from them, never copy from them, and do not
read `xbox/include`. To add a name, follow `port/include/xdk/README.md`.

### Plugins

- Plugins are Rust, on the hooks of the import list. They read the game's
  memory through read-only views with bounds-checked readers; the game
  still decides everything.
- Each plugin's time is logged per frame, and one over its share is a
  failing test. With every plugin off, a run matches the reference traces.

### Networking

- Only the host decides; clients predict their own players.
- Anything another machine sends is untrusted: check sizes, indices and
  rates.

### Art and credit

- Replacement HUD, title and menu art is made of hand-drawn redraws.
- `AUTHORS.md` credits art only.
- Fonts: Overpass and OpenCE (Newtown, respaced) only; see
  `port/assets/fonts/README.md`. Never ship the maps' commercial typefaces.

### Custom Edition maps

Custom Edition maps live in the data root's `custom_maps` folder with
`bitmaps.map`, `sounds.map` and `loc.map`. OpenSauce features are not
supported and must not be added. Map files are untrusted input: read them
through bounds-checked readers, and fuzz the readers. See
`docs/custom_edition_caches.md`.

### Settings

Settings live in `settings.toml`, with a default and a comment. Each is
also a dev panel control, a shell command and a control socket call.
Extensions to the original (the lighting's among them) can each be
switched back to stock.

### Third-party code and licences

The repository is CC0. Vendored code goes in `port/third_party/<name>`,
copied unchanged, with a README naming the upstream, the version or commit,
the checksum and what uses it. Crates are pinned in `engine/Cargo.lock`
and checked by `cargo deny`. GPL code (OpenSauce, Reclaimer) may be read
as documentation of file formats, never copied.

### Commits and pull requests

- Commit subjects are plain sentences saying what changed; the body says
  why and what was checked.
- A pull request is merged with a merge commit (`Merge pull request #N from
  <owner>/<branch>`), and follow-up changes go in separate commits whose
  subject ends with `(#N)`.
- A commit taken from a fork keeps its author and is made with
  `git cherry-pick -x`.

## Documents

| Document | About |
| --- | --- |
| [README.md](README.md) | Game data, building, running |
| [CLAUDE.md](CLAUDE.md) | The working practices |
| [docs/retrospective_fork_in_the_road.md](docs/retrospective_fork_in_the_road.md) | Why the game runs as wasm32 in Wasmtime |
| [docs/engine.md](docs/engine.md) | The app, the dev tools, the graphics tiers, the benchmark |
| [docs/census.md](docs/census.md) | Every function of the original and its class |
| [docs/custom_edition_caches.md](docs/custom_edition_caches.md) | Loading Custom Edition maps, and the evidence for each layout |
| [conductor/tracks.md](conductor/tracks.md) | The tracks, in order |
| [port/include/xdk/README.md](port/include/xdk/README.md) | How the SDK declarations were written and are checked |
| [port/assets/fonts/README.md](port/assets/fonts/README.md) | The fonts and their licences |
| `port/third_party/*/README.md` | Each vendored library |
| [AUTHORS.md](AUTHORS.md) | Art credits |
