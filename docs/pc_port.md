# What the PC port left out

Gearbox's Halo PC (2003) and Custom Edition (2004) were ported from the
Xbox original onto Direct3D 9 and a PC sound card, and lost or changed
effects the Xbox drew and played. OpenCE does not port them back: it runs
the Xbox original's own code (`source/`, compiled as `game.wasm`), so each
effect the Xbox had is in it by construction. What remains to show is that
each one runs here and draws, and that what it falls to the presentation to
do (the device that draws the game's commands, the mixer, the screen's
size) does it as the Xbox did.

Each entry below names the effect, the original's function that draws it,
and how it is measured. The losses are taken against the Xbox original by
measurement, not from the PC port's notes: a PC behaviour is listed only
where the PC's own maps or tags show it (`docs/custom_edition_caches.md`).

## How the effects are measured

`engine/crates/runtime/tests/pc_port.rs` plays Blood Gulch at 640×480, the
time stepped a 60th a frame (`debug.frame_clock`) so that frame N is the
same moment of the game in every run, once as it is and once with each
effect switched off by the original's own console global (the
`rasterizer_*` globals of `hs_globals_external.c`, which set
`rasterizer_debug_options`, `rasterizer/rasterizer_console_vars.h`). A
plugin counts the draws each frame's commands make and the vertices (or
indices) they draw. An effect Blood Gulch draws draws less with its switch
off; the table it writes (`build/engine/pc_port.tsv`) is the measurement,
and the effects it found drawn are held by the test.

`rasterizer_environment` is left at 2, its starting value
(`rasterizer.c`): at 1 or 0 `rasterizer_frame_begin` would set every
environment part from it each frame and undo the switch under test.

## The effects

Blood Gulch played for ten seconds (300 ticks) after the load at 640×480,
the draws a frame averaged over the frames of play. A switched-off run
draws a little more a frame than the stock one (the console echoing the
switch's command), so each run is held against the median switched-off
run (110.44 draws a frame; the stock run draws 109.94), and an effect
counts as drawn when its switch saves at least half a draw a frame.

| Effect | Global | Draws a frame, median → off | Drawn here |
|---|---|---|---|
| the lightmaps | `rasterizer_environment_lightmaps` | 110.44 → 99.44 | yes |
| the environment's decals | `rasterizer_environment_decals` | 110.44 → 109.44 | yes |
| bumped reflections | `rasterizer_environment_reflections` | 110.44 → 100.45 | yes |
| transparent geometry | `rasterizer_environment_transparents` | 110.44 → 109.45 | yes |
| the environment's fog | `rasterizer_environment_fog` | 110.44 → 99.45 | yes |
| the models | `rasterizer_models` | 110.44 → 72.48 | yes |
| the models' transparents | `rasterizer_model_transparents` | 110.44 → 93.45 | yes |
| the shadows' softening | `rasterizer_shadows_convolution` | 110.44 → 109.45 | yes |
| the motion sensor | `rasterizer_hud_motion_sensor` | 110.44 → 107.46 | yes |
| specular lights | `rasterizer_environment_specular_lights` | 110.44 → 110.44 | not in these frames |
| specular lightmaps | `rasterizer_environment_specular_lightmaps` | 110.44 → 110.45 | not in these frames |
| the fog screen | `rasterizer_environment_fog_screen` | 110.44 → 110.44 | not in these frames |
| planar fog | `rasterizer_fog_plane` | 110.44 → 110.44 | not in these frames |
| atmospheric fog | `rasterizer_fog_atmosphere` | 110.44 → 110.44 | not in these frames |
| the water | `rasterizer_water` | 110.44 → 110.44 | not in these frames |
| lens flares | `rasterizer_lens_flares` | 110.44 → 110.44 | not in these frames |
| lens flare occlusion | `rasterizer_lens_flares_occlusion` | 110.44 → 110.44 | not in these frames |
| detail objects | `rasterizer_detail_objects` | 110.44 → 110.45 | not in these frames |
| bump mapping | `rasterizer_bump_mapping` | 110.44 → 110.45 | not in these frames |
| active camouflage | `rasterizer_active_camouflage` | 110.44 → 110.44 | not in these frames |
| plasma energy | `rasterizer_plasma_energy` | 110.44 → 110.44 | not in these frames |
| screen flashes | `rasterizer_screen_flashes` | 110.44 → 110.44 | not in these frames |
| screen effects | `rasterizer_screen_effects` | 110.44 → 110.44 | not in these frames |

The nine drawn are held by the test. The rest are not drawn by the frames
played, which are the spawn's view with no one moving: no camouflage,
plasma, flash or screen effect happens, no lens flare's source is in view,
and fog planes, water and specular parts are in other places of the map.
Counting draws does not see an effect that changes how a draw looks rather
than whether it is made (bump mapping changes a shader, not a draw); such
effects are measured by image instead (the golden frames, `HudScreenTests`).
Measuring the rest needs a scripted camera that visits each, which is left
to devgulch's stations once it has a map of its own.

## Gearbox's own behaviours

Custom Edition maps can be made around the PC's way of drawing rather than
the Xbox's. Chimera lists those maps and the behaviours each relies on
(`map_hacks_config.json`; here `port/linux/game/custom_edition_behaviours.inc`).
This build draws as the Xbox does and, for a listed map, follows these
(of the 248 maps listed, how many rely on each):

| Behaviour | Maps | Status |
|---|---|---|
| `gearbox_multitexture_blend_modes`: HUD multitexture overlays' blend functions in the PC's order | 54 | followed |
| `block_multitexture_overlays`: the overlays not drawn | 7 | followed |
| `hud_number_scale`: the HUD digits' metrics halved | 122 | followed |
| `disable_bitmap_hud_scale_flags`: bitmaps' HUD scale flags cleared | 33 | followed |
| `invert_detail_after_reflection`: model shaders' detail after reflection flipped | 15 | followed |
| `gearbox_meters`: the PC's fixed-function meters | 193 | not yet |
| `gearbox_chicago_multiply`: the PC's transparent chicago multiply | 44 | not yet |
| `alternate_bump_attenuation`, `gearbox_bump_attenuation` | 7 and 60 | not yet |
| `gearbox_shader_environment_types` | 10 | not yet |
| `old_widescreen_fix`: the old widescreen HUD | 42 | not yet |
| `embedded_lua` | 2 | not followed (no Lua) |

None of the five not yet followed changes a stock Xbox map; each changes
only a map Chimera lists. Blood Gulch is not listed.

## The sound

The Xbox mixed its sounds on the console's DSP with an I3DL2 reverb for the
place the listener was in (the sound environments of the BSP's clusters),
and occlusion and obstruction; the PC port handed 3D sound to the card
through DirectSound3D and EAX, so a card without EAX heard no reverb.
`game.wasm` keeps the Xbox's own sound code (`source/sound/sound_dsound_xbox.c`)
and mixes it in `port/linux/src/dsound_sdl.c`, which models the I3DL2
listener's reverb, the direct path's obstruction and occlusion and the mix
bins (`audio.reverb`, on by default). The mixer's tests
(`tools/harness/tests/mixer.c`, through `test_mixer.py`) check each of
these on known scenes, the reverb among them, and
`GameRunTests.testBloodGulchIsHeardHeadless` hears Blood Gulch through the
app's speakers. Not modelled: Doppler and cones.

`port/linux/README.md`'s table of the platform said the sound had no
reverb; that line predated the reverb and now names it.

## The screen

The Xbox drew at 640×480 and the PC port at the window's size with a
stretched HUD. `game.wasm` draws at the drawable's own resolution, its
640×480 screen scaled as the GL port scales it (`screen_mode_choose`;
`tests/screen.rs`, 265b437a), and the HUD and the first-person weapon are
measured at 4K against 640×480 (`HudScreenTests`).
