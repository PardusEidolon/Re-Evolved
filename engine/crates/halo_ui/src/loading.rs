//! The loading screen, drawn while the game loads a map: the original's
//! band of light (interface/progress_bar.c) sweeping across the screen
//! with the load, then the screen's fade into the mission.
//!
//! The original draws its loading picture through the band, and the
//! picture is the Xbox disc's `loading.tga`, which a PC map set lacks: the
//! game's own screen is black without it. The light's timing, its
//! brightness, its mask and its fade are the original's, from
//! [`opence_ui::loading`]; the band is drawn as light over a deep blue
//! ground, the blue of the original's picture.
//!
//! The first map the player loads from the menus is paced: its band
//! sweeps across for [`FIRST_SWEEP_SECONDS`] however fast the map loads,
//! the game held on its loading loop until it has (`host_ui_loading_held`).

use opence_runtime::plugins::View;
use opence_ui::loading::{LoadingFrame, LoadingScreen, Stage, MASK_WIDTH};

use crate::scene::{hex, Align, Face, Scene, Shape, Text, HEIGHT, WIDTH};

/// The game's view of its load (source/game/game.c's
/// `game_loading_port_view`), which game.wasm exports.
pub const LOADING_EXPORT: &str = "game_loading_port_view";

/// `long active`, then `real progress`
const ACTIVE: u32 = 0;
const PROGRESS: u32 = 4;

/// the original's screen is 640 units wide
const UNIT: f32 = WIDTH / 640.0;

/// the band drawn as strips across and rows down its 128 by 16 mask
const STRIPS: u32 = 64;
const ROWS: u32 = 24;

/// the light's color: the original's loading picture is blue-white
const LIGHT: u32 = 0x9fd4ff;

/// the ground's: a deep blue, dark enough for the band to show over it
const GROUND: u32 = 0x0a2552;

/// How long the first map loaded from the menus sweeps for, at the least.
pub const FIRST_SWEEP_SECONDS: f32 = 5.0;

/// How far the game's map load has come (0 to 1), while it loads one.
#[must_use]
pub fn progress(view: &View<'_>) -> Option<f32> {
    let at = view.export(LOADING_EXPORT)?;
    if view.u32(at.checked_add(ACTIVE)?)? == 0 {
        return None;
    }
    view.f32(at.checked_add(PROGRESS)?)
}

/// The loading screen's state across frames.
#[derive(Debug, Default)]
pub struct Loading {
    screen: Option<LoadingScreen>,
    /// the seconds the next load is to sweep for, at the least
    next: Option<f32>,
    /// the load's pace: the seconds it sweeps for, and the seconds so far
    pace: Option<(f32, f32)>,
}

impl Loading {
    /// The screen `seconds` after the last step, with the load at
    /// `progress` (`None` once no map loads): its scene, or `None` when
    /// nothing is drawn.
    pub fn step(&mut self, progress: Option<f32>, seconds: f32) -> Option<Scene> {
        let screen = match progress {
            Some(progress) => {
                if self.screen.is_none() {
                    self.pace = self.next.take().map(|sweep| (sweep, 0.0));
                }
                let shown = match &mut self.pace {
                    Some((sweep, so_far)) => {
                        if seconds.is_finite() {
                            *so_far += seconds.clamp(0.0, opence_ui::loading::LONGEST_FRAME);
                        }
                        progress.min(*so_far / *sweep)
                    }
                    None => progress,
                };
                let screen = self.screen.get_or_insert_with(|| LoadingScreen::new(true));
                screen.set_progress(shown);
                screen
            }
            None => {
                let screen = self.screen.as_mut()?;
                screen.set_ready();
                screen
            }
        };
        let frame = screen.update(seconds);
        if frame.stage == Stage::Done {
            self.screen = None;
            self.pace = None;
            return None;
        }
        Some(scene(&frame))
    }

    /// The next map loaded sweeps for `seconds` at the least.
    pub fn pace_next(&mut self, seconds: f32) {
        if seconds.is_finite() && seconds > 0.0 {
            self.next = Some(seconds);
        }
    }

    /// Whether the game is to stay on its loading loop: a paced load's
    /// band has not yet swept across.
    #[must_use]
    pub fn held(&self) -> bool {
        let loading = self
            .screen
            .as_ref()
            .is_some_and(|screen| screen.stage() == Stage::Loading);
        loading && self.pace.is_some_and(|(sweep, so_far)| so_far < sweep)
    }
}

/// A frame of the screen laid out.
#[must_use]
pub fn scene(frame: &LoadingFrame) -> Scene {
    let alpha = frame.screen_alpha.clamp(0.0, 1.0);
    let mut scene = Scene::default();
    scene
        .shapes
        .push(Shape::fill([0.0, 0.0, WIDTH, HEIGHT], hex(GROUND, alpha)));
    // the mask: sin across its width, sin^0.75 down its height
    let left = frame.mask_position * UNIT;
    let width = MASK_WIDTH * UNIT / STRIPS as f32;
    let height = HEIGHT / ROWS as f32;
    for strip in 0..STRIPS {
        let x = (strip as f32 + 0.5) / STRIPS as f32;
        let across = half_sin(core::f32::consts::PI * x);
        for row in 0..ROWS {
            let y = (row as f32 + 0.5) / ROWS as f32;
            let down = libm::powf(half_sin(core::f32::consts::PI * y), 0.75);
            let lit = (across * down * (250.0 / 255.0) + 5.0 / 255.0) * frame.color * alpha;
            let mut shape = Shape::fill(
                [
                    left + strip as f32 * width,
                    row as f32 * height,
                    width,
                    height,
                ],
                hex(LIGHT, lit.clamp(0.0, 1.0)),
            );
            shape.additive = true;
            scene.shapes.push(shape);
        }
    }
    let mut caption = Text::new(
        "LOADING",
        [WIDTH / 2.0, HEIGHT * 0.86],
        64.0,
        Face::Heavy,
        hex(0xd8ecff, (0.35 + 0.65 * frame.color) * alpha),
    );
    caption.glow = hex(LIGHT, 0.6 * frame.color * alpha);
    caption.align = Align::Center;
    scene.texts.push(caption);
    scene
}

/// sin, never below 0 (the mask's halves)
fn half_sin(x: f32) -> f32 {
    libm::sinf(x).max(0.0)
}
