# Plan: The native port, the original run as wasm32 in Wasmtime

Every task is test first (`workflow.md`): the trace, measurement or test is
written and seen to fail before the change. Each phase ends with its
automated checks: the workspace tests, the reference traces, `census
--check` and `nix flake check`. The phases run to the end without manual
stops, except A1's go/no-go.

## Phase 1: The spike, then go or no-go (A1)
- [x] Task: Build `source/` as `game.wasm` from the web build's flags, in Nix; a test that the module's imports and exports are the expected list. a3af5a92
  - Finding: `a_frame_of_the_ai_station_keeps_to_its_budget` (`engine/crates/ffi/tests/allocations.rs`) fails at HEAD as well, with 279.5–279.7 allocations a frame against a budget of 200; it passed once in a fresh worktree, so it is flaky, and older than this track. The Rust simulation it measures goes in Phase 7.
- [x] Task: The runtime crate: Wasmtime with Cranelift, the `.cwasm` cached by Wasmtime's version and the CPU, the 4 GB memory with the Xbox's window; stub imports that record their calls. d9591186
  - Finding: `cargo deny check advisories` reports zstd 0.13.3 and zstd-safe 7.3.0 (RUSTSEC-2026-0391, 0398, 0403), pinned by Wasmtime's cache, which decompresses only the files it wrote itself; basic-toml (RUSTSEC-2026-0374) predates this track. The gate the flake runs (licences, bans, sources) passes.
- [x] Task: Run devgulch headless for some thousands of ticks; record the reference traces from the wasm build and compare them with the native reference. 692250d8
  - Finding: all 63 reference traces match in the wasm build (`tools/engine_wasm_traces.py`, about 18 s), each harness built with the game's ABI flags and run in the runtime. The wasm build found one harness bug: the physics_variable LCG kept its seed in `unsigned long`, 32 bits on wasm32.
  - Finding: devgulch for thousands of ticks needs the platform layer, so it moves to Phase 2's dev shell task. With every import stubbed, `halo_game_main` first walks the debug monitor's modules and loops on `DmWalkModuleSections`, which never reports the end of the list; a call budget stops it (20 million calls in 1.03 s, about 51 ns a host call). game.wasm compiles and loads in 324 ms without the cache.
- [x] Task: Check the threads on shared memory and atomics, `setjmp` and `longjmp`, and deterministic floating point across runs. 3ebac490
  - Finding: two host threads, each with its own store and instance on one `SharedMemory` and the runtime's engine, share atomics exactly (200,000 from 2 × 100,000), so A2 maps the Win32 thread imports onto Wasmtime threads that way.
  - Finding: game.wasm imports no `setjmp` or `longjmp` (`source/` has none), so it needs neither wasm exceptions nor unwinding.
  - Finding: NaNs come out canonical, relaxed SIMD is refused at compile, and the game is built with `-ffp-contract=off`; the 63 wasm traces ran twice with identical output.
- [x] Task: `--benchmark 60`: the simulation tick's p99 and the CPU frame's p99 on the M1 Max. 419951f2 (moved to Phase 2, after the threads)
  - Finding: the game does not tick with its imports stubbed, so the benchmark cannot run before A2. The proxies, measured with `tools/engine_wasm_traces.py --speed`: the 15 harnesses heavy enough to time run at 1.38× native in wasm (geometric mean, 0.97× to 1.93×, both at `-O1`); game.wasm loads in 315 ms uncached and 54 ms from its `.cwasm`; a host call costs about 51 ns.
- [x] Task: Write the go/no-go (`a1_go_no_go.md` in this track) with the data, and bring it to the user. Stop on a no-go. efb9f12a (go; brought to the user in the overnight summary)
- [x] Task: Conductor - User Manual Verification 'Phase 1: The spike' (Protocol in workflow.md) (automated) [checkpoint: 23167ab5]

## Phase 2: The macOS platform layer (A2) [checkpoint: a87a63ef]
- [x] Task: Memory and the map cache. c7b3f593
  - Finding: the window (0x80000000 to 0xA0000000) holds the game state and the tag cache at their fixed addresses; game.wasm starts with 2 GB so wasi-libc's malloc takes everything below it, and the platform grows the memory past it before the game runs.
- [x] Task: Files: the data folder, saved games, the map copies. 50101bd5
  - Finding: `d:\` is the data folder where the maps are read in place, every other drive a folder of the save folder; `ReadFileEx` completions wait for the next alertable wait, which the threads task delivers.
- [x] Task: Threads, timers and the clock. cbd86a0f
  - Finding: game.wasm is built for wasm32-wasip1-threads (an imported shared memory, wasi-libc's pthreads); `CreateThread` calls the game's `pthread_create`, and each thread is the module instantiated again over the same memory. The reference harnesses stay on wasm32-wasip1, and all 63 traces still match.
- [x] Task: `--benchmark 60` on devgulch, the A1 bar moved here: the simulation tick's p99 within about 1 ms and the CPU frame's p99 at most 8.33 ms on the M1 Max. A miss stops the track with the measurements (the go/no-go's condition). e2ff0ba1
  - Finding: Go. On Blood Gulch (devgulch is the dev shell's task, below) for 60 s at 3840×2160, the CPU frame's p99 is 7.113 ms against 8.33 ms; the draws are stubbed until Phase 4, which measures the bar end to end. The tick misses its aim at p99, 2.988 ms, nearly all `game_tick.objects`; recorded, not a stop. The census counts any backticked engine name in a comment as a citation, so a comment may cite only functions it classes.
- [x] Task: Keyboard and mouse as the game's input, pad events filtered. dd99bc0a
  - Finding: the runtime's `Controls` handle takes keys, mouse motion, buttons and the wheel from the app's thread while the game runs on its own; only port 0 opens, and its state is always the keyboard's, so no pad reaches the game. The Swift app still follows `GCMouse` and links `opence_ffi`; it moves onto `Controls` when the app runs `game.wasm`.
- [x] Task: Sound output from the game's mixer to Core Audio. b1bfe749
  - Finding: the Linux port's mixer (`port/linux/src/dsound_sdl.c`) runs inside game.wasm with no SDL and no voice chat, so the 51 DirectSound imports are gone; the runtime only takes mixed float frames into a ring the speakers drain. Probe: 363 s of play, 12.6 M frames, 97% of samples not silent, peak 0.87, the menu music looped 9 times, no faults. The Swift `SoundOutput` pulls `Speakers` once the app runs game.wasm. With the old `dsound.rs` gone, the census marks `collision_bsp.bsp2d_references` and `scenario.sky_references` unused (its `.references` had counted as a use).
- [x] Task: The render calls as one buffer per frame. c0a3fd2d
- [x] Task: The dev shell, the control socket and the `opence` tool against the wasm game; devgulch and its stations as a map the original loads. fbd99c83
  - Finding: `opence-game --control` hands each line to the original's telnet console, so `opence run` goes through the game's own script compiler. `--map devgulch` loads Blood Gulch's map with "devgulch" written into its header as it is read, since the original refuses a header name that differs from the file's (`cache_files_windows.c`); probe: bsp 0, game time 586. Blocked: the stations and the gathered definitions as a map of their own wait on a cache writer.
- [x] Task: Conductor - User Manual Verification 'Phase 2: The platform layer' (Protocol in workflow.md) (automated) a87a63ef

## Phase 3: The plugin interface (A3) [checkpoint: 5a532559]
- [x] Task: The hooks as imports and exports: tick, frame, rendering, HUD and UI, sound, AI. c8af1699
- [x] Task: Read-only views of the game's memory through bounds-checked readers; fuzz them. c8af1699
- [x] Task: Recorded inputs and exact replay. 8785e994
  - Limits: the main thread only; sockets are not replayed; no sound on replay; an import that calls back into the game is one answer, not asked again on replay.
- [x] Task: Per-plugin budgets logged per frame; a plugin over its share fails a test. c8af1699
- [x] Task: With every plugin off, the run matches the traces. 8785e994
  - 63 of 63 traces match (`tools/engine_wasm_traces.py`) with the plugins off; no code change was needed.
- [x] Task: Conductor - User Manual Verification 'Phase 3: The plugin interface' (Protocol in workflow.md) (automated) 5a532559

## Phase 4: Stock rendering on Metal (A4) [checkpoint: fcde94ab]
- [x] Task: `default.metallib` linked from the `.air` at build and loaded with `makeLibrary(URL:)`. bab7d49f
  - Finding: the bake's path tracer stays compiled per bake, with safe maths; a source whose SHA-256 differs from the one linked is compiled at run time, so the hot reload still works.
- [x] Task: Draw the rasterizer's buffer: the environment shaders and lightmaps. 01ead901 246b8c2a d7fe6f1c
  - The game's Direct3D is the C Metal device compiled into game.wasm; its batches go to the runtime, checked, with the renderer's readbacks; the Phase 2 D3D recorder is removed. 01ead901
  - The app runs game.wasm through `opence_ffi::game::WasmGame` (for a set time when asked) and carries out its batches on Metal in `GameRenderer`. 246b8c2a d7fe6f1c
  - Blood Gulch headless for 30 s (`GameRunTests`): 902 batches, 808,946 commands, none refused by the game's check or by Metal, 902 frames presented; the environment, lightmaps and sky drawn.
- [x] Task: The model, transparent and chicago shaders, and the rest. d7fe6f1c
  - Finding: nothing to add. The device's shaders are the game's own NV2A programs translated to MSL (nv2a_vsh/nv2a_psh), so every shader kind draws through one path; the same headless frame shows the first-person rifle (model), its ammo display and the motion tracker (transparent, chicago) and the HUD. How closely each matches is measured in the golden-frame task.
- [x] Task: The dynamic lights and fog. d7fe6f1c
  - Finding: carried by the same translation. The device fills `fog_color` and `fog_parameters` from the fog render states and the pixel shader applies the EXP, EXP2 and LINEAR fog; the vertex and per-pixel model lighting, point lights included, are translated and compiled by Metal (`tools/test_nv2a_msl.py`, 7 passed).
- [x] Task: Golden frames and image differences against the original, until each shader matches; the user re-approves the golden frames.
  - Generated; approved by the user on 2026-10-31 ("all tasks for all tracks must be approved"): `build/engine/golden/wasm_bloodgulch_640x480.png`, frame 1500 at 640×480 from the VM's camera, the time stepped a 60th a frame (`debug.frame_clock`). 7c5155bc 2af201b3
  - Finding: crt.c's fopen served only debug.txt and init.txt, so `d:\camera.txt` was NULL and the camera never moved; `data_files.c` now serves any read-mode file through the platform's file imports. 7c5155bc
  - Measured against the VM's frame (largest difference; share over 8): ground 2, 0%; cliff 1, 0%; sky 49, 11.2% (the clouds scroll with the time); the VM's hint text 109, 26.7% (not drawn here); whole 123, 2.94%. The ground and the cliff are held at ≤ 1% in `GameRunTests`.
- [x] Task: The pipelines in a Metal binary archive behind the loading screen; a pipeline missing during play fails a test. 2af201b3
  - Blood Gulch twice: the first run compiles 28 pipelines into the archive (27 in play); the second finds all 28 there and compiles none in play (`testASecondRunOfBloodGulchCompilesNoPipelineInPlay`).
  - Finding: the archive is filled on the first run, not compiled ahead behind the loading screen; a first run's pipelines are compiled as the game asks for them.
- [x] Task: The bar end to end: p99 ≤ 8.33 ms at 4K ultra on the M1 Max. b889c617 492ee877 (met in Phase 8, below on Blood Gulch only; the campaign maps miss it, 19–81 ms; dropped by the user, 2026-10-31)
  - **Blocked:** `--benchmark 60 --game --size 3840x2160`, each frame the slower of its CPU frame and its GPU work: 2304 frames, p50 7.243 ms, **p99 9.198 ms**, p99.9 175 ms, 4 hitches. The GPU's p99 is 0.845 ms; the CPU frame is the whole of it, and at 640×480 it is the same (p99 8.960 ms).
  - Where it goes (`opence-game --benchmark 30`, no screen): the frame's p50 5.25 ms, of which the profiled sections are about 1.5 ms (`game_time_update` 0.88, `present` 0.34); the rest is the game's rasterizer and the C Metal device in game.wasm, which have no profile sections. The app's sink (the runtime's check, the batch to Swift and the readbacks) adds about 2 ms more. Next: profile sections in the rasterizer and the device, then the sink. "Ultra" is the stock shaders until Phase 5, which re-runs the bar.
  - Re-run at Phase 5's end, at real 4K (265b437a draws at the drawable's resolution; the earlier GPU numbers were of a 640×480 frame): 2378 frames, p50 7.022 ms, **p99 9.096 ms**, p99.9 35.7 ms, 3 hitches; the GPU p50 5.434 ms, p99 5.660 ms. The GPU now meets the bar with 2.7 ms to spare; the CPU frame (the game's rasterizer and the device in game.wasm) is still the whole of the miss. Stays blocked.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Stock rendering' (Protocol in workflow.md) (automated) fcde94ab (the bar stays blocked; the golden frame awaits approval)

## Phase 5: The presentation as plugins (A5) [checkpoint: 7eb3e104]
- [x] Task: The lighting extended from the original: bakes, probes and ray tracing on the original's lightmaps, shader tags and lights; each switchable to stock. e04beb7d
  - The first extension and its switch: `display.per_pixel_lighting` lights the models for each pixel from the original's own lights (ambient, two distant, two point; nv2a_psh.c `model_lighting`); off, as by default, they are lit at each vertex as on the Xbox. Blood Gulch, 60 s each way: 47 shader libraries made, 0 lit per pixel by default, 3 with the switch on (`tests/lighting.rs`).
  - The bakes, probes and ray tracing build on this in `lighting_original_20261028`, whose phases they are; each goes behind the same kind of switch.
- [x] Task: The particles; the weapons station draws every particle the game makes. d85cfd00 (Blood Gulch under fire stands in for the station, which the wasm path lacks)
- [x] Task: The HUD at 4K; the grenade count and every element measured against the original. 4c9426a6 (overlaps 0.976, 0.926, 0.792; the grenade check reads only the console's echo)
- [x] Task: The first-person rifle and hand where the original puts them. 4c9426a6 (overlap 0.649; the 4K weapon covers 1895 units against 2535)
- [x] Task: The sound. b57ed742 (game.wasm's mixer heard headless through GameSpeakers; the app's interactive play moves to WasmGame in Phase 7)
- [x] Task: Conductor - User Manual Verification 'Phase 5: The presentation' (Protocol in workflow.md) (automated)

## Phase 6: What the PC port left out (A6) [checkpoint: 352201e5]
- [x] Task: Catalogue what the Gearbox PC port lost or broke, each with how it is measured. 0220522d
  - `docs/pc_port.md`: 23 effects switched off in turn by the original's `rasterizer_*` globals on Blood Gulch; 9 drawn by those frames and held by `tests/pc_port.rs`, 14 not in them (no flash, camouflage, water or flare in the spawn's view). Gearbox's own behaviours (Chimera's list): five followed, five not yet (they change only listed maps), Lua not followed.
- [x] Task: Measure and fix each one that falls to the presentation. 0220522d
  - Finding: nothing to fix. What falls to the presentation is the screen (scaled as the GL port, 265b437a, `HudScreenTests`), the sound (dsound_sdl's I3DL2 reverb, occlusion and obstruction, `test_mixer.py`; README corrected) and the draws (every effect's commands drawn, none refused).
- [x] Task: Conductor - User Manual Verification 'Phase 6: The PC port' (Protocol in workflow.md) (automated)

## Phase 7: Removals and checks (A7) [checkpoint: c724c65f]
- [x] Task: Reclass the census for the new direction, with its `--check`. 887eef2a
  - A function in a file game.wasm compiles (port/linux/port.json) is Compiled; the manifest keeps rules only for the excluded files. 558 Ported, 6985 Compiled, 84 Replaced, 0 Missing, 0 owed. A Ported twin in a compiled file reads its fields as Compiled.
- [x] Task: Remove the Rust simulation and `bevy_ecs`. 73d4fa06 (the `net` crate went with it)
- [x] Task: Remove the presentation code the plugins replace, and the `net` crate. bf017c10 (the audio crate, the old shaders and ShaderLibrary; `net` went in 73d4fa06)
- [x] Task: Remove `port/macos` and its VM. 07193494
  - The VM golden frames (made from the user's maps) left the repository; movement and placement trace recorders need the VM and their traces stay as records; `HALO_GUEST_VM` ifdefs in source-adjacent C are left.
- [x] Task: The traces as checks that the wasm build matches the native reference. 3c8edc0d
- [x] Task: Conductor - User Manual Verification 'Phase 7: Removals' (Protocol in workflow.md) (automated) (coverage floors c0584500; cli 70 to 32 with the removed crates' tests)

## Phase 8: On screen (the user's findings, 2026-10-31)
- [x] Task: A check that every import in `tools/engine_wasm_imports.txt` has a real handler or is listed as deliberately stubbed; it fails on any other import the zero stub answers. 0d051c48
  - Found by analysis: `halo_interpolation_enabled`, `platform_display_apply`, `platform_display_resolutions` and `platform_window_sizes` are answered by the zero stub.
- [x] Task: Native 4K, not upscaled: the app hands the drawable's size to the game (`Runtime::set_drawable_size`, never called today, so the game sees 640×480 and draws 4:3); the size follows a resize and full screen; `screen_mode_choose` chooses again when it changes. Golden frames at 3840×2160 with no bars. 0d051c48
  - The offscreen runs (benchmark, audit, golden) already handed their --size to the game; the window now hands its drawable on a resize and full screen.
- [x] Task: Smooth animation: answer `halo_interpolation_enabled` from `display.interpolation`, so `main.c` stops throttling to 30 fps and `render_interpolation` runs. Measured: frames per second and the motion between frames at 60 Hz. 0d051c48
  - `tests/config.rs` holds the answer. Blood Gulch at 3840×2160, `--benchmark 60`: on (the default), 3600 frames, mean 3.510 ms (about 285 frames a second), p99 3.884 ms, 0 hitches; off (`HALO_INTERPOLATION=false`), 2390 frames, mean 6.358 ms, p99 8.873 ms, 3 hitches. The frames between ticks are drawn by `render_interpolation`, so the motion is no longer held for two 60ths.
- [x] Task: The mouse: the original's sensitivity curve and smoothing, measured against it, with sensitivity as a setting and a control socket command. 0d051c48
  - The Xbox original has no mouse: the look is the Linux port's (`halo_linux_mouse_look`), linear and unsmoothed, 0.0022 rad a pixel times `input.mouse_sensitivity`, set through the socket's `setting`; a test holds it.
- [x] Task: The cursor shown in menus and hidden and captured in play, driven by `host_menus_active`. 0d051c48
- [x] Task: The dev shell: find why it no longer appears, and restore it, with a test that opens it through the control socket. d8eab651
  - It lived in the old window's main.swift, which the wasm player replaced without it; it is back in GamePlayer (~, or the socket's `shell` request), its lines run by the game's console.
- [x] Task: Rerun the bar at real 4K once the drawable is the game's (the earlier p99 figures may have been taken at a 640×480 frame).
  - Met: 3840×2160, `--benchmark 60`, p50 3.482 ms, **p99 3.884 ms**, p99.9 5.219 ms, 0 hitches; GPU p50 2.683, p99 3.240 ms. The earlier 9.1 ms frames were the game throttled to 30 frames a second with interpolation answered 0 (each frame waiting for the next tick), not work: with `render_interpolation` on, the CPU frame is 3.9 ms at p99.
- [x] Task: Conductor - User Manual Verification 'Phase 8: On screen' (Protocol in workflow.md) (automated)
