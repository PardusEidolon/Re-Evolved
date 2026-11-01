//! The Halo UI plugin: the menus drawn anew at the display's resolution,
//! as the design artifact's prototypes draw them, from the game's live
//! widget tree.
//!
//! The game keeps running its own UI (interface/ui_widget.c): its
//! screens, their focus, their input and their strings are the
//! original's. With `display.redraw_menus` on, the game stops drawing the
//! menus' text, their lists and the pictures below a screen's own
//! children; this plugin reads the tree at each `ui_update` and lays the
//! screen out as a [`Scene`], which the app draws over the frame. While
//! the game loads a map it draws the loading screen ([`loading`]), each
//! frame of the load and of its fade into the mission.

pub mod loading;
pub mod scene;
pub mod screens;

use std::sync::{Arc, Mutex, PoisonError};
use std::time::Instant;

use opence_runtime::keyboard::KEYBOARD_EXPORT;
use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::widgets::WIDGET_GLOBALS_EXPORT;
use opence_runtime::{Controls, Tuning};

pub use scene::Scene;

/// The setting that turns the plugin's drawing on.
pub const SETTING: &str = "display.redraw_menus";

/// The plugin: each `ui_update`, the first player's screen laid out into
/// the scene it shares with the app.
pub struct Redraw {
    scene: Arc<Mutex<Scene>>,
    tuning: Tuning,
    controls: Controls,
    /// the open screen, as the last `ui_update` laid it out
    menus: Scene,
    /// the loading screen, and when it was last stepped
    loading: loading::Loading,
    stepped: Option<Instant>,
    /// the loading screen's last frame, and whether the map still loads
    screen: Option<(Scene, bool)>,
    /// whether the menus have been drawn, so that the first map loaded
    /// from them is paced
    paced: bool,
}

impl Redraw {
    /// The plugin, laying the screens out into `scene` while the game's
    /// setting [`SETTING`] (read through `tuning`) is on, and telling the
    /// pointer (through `controls`) where it drew each item.
    #[must_use]
    pub fn new(scene: Arc<Mutex<Scene>>, tuning: Tuning, controls: Controls) -> Self {
        Self {
            scene,
            tuning,
            controls,
            menus: Scene::default(),
            loading: loading::Loading::default(),
            stepped: None,
            screen: None,
            paced: false,
        }
    }
}

impl Plugin for Redraw {
    fn name(&self) -> &str {
        "the Halo UI"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Ui, Hook::Frame]
    }

    fn on(&mut self, event: &Event<'_>, view: &View<'_>) {
        let on = self.tuning.get(SETTING).as_deref() == Some("true");
        match event {
            Event::Ui => {
                self.menus = if on { menus(view) } else { Scene::default() };
                // (with nothing drawn the pointer finds the game's own layout)
                self.controls.set_redrawn(self.menus.targets.clone());
                self.controls.set_placed(self.menus.pictures.clone());
                if !self.paced && !self.menus.shapes.is_empty() {
                    self.loading.pace_next(loading::FIRST_SWEEP_SECONDS);
                    self.paced = true;
                }
            }
            // (the game's frame ends: each frame of a map's load, whose
            // loop updates no menus, and each frame after it)
            Event::Frame => {
                let now = Instant::now();
                let seconds = self
                    .stepped
                    .map_or(0.0, |then| now.duration_since(then).as_secs_f32());
                self.stepped = Some(now);
                let progress = if on { loading::progress(view) } else { None };
                self.screen = self
                    .loading
                    .step(progress, seconds)
                    .map(|screen| (screen, progress.is_some()));
                self.controls.set_loading_held(self.loading.held());
            }
            _ => return,
        }
        // (both hooks publish the same scene: the loading screen lies over
        // the menus until it has faded, whichever hook ran last)
        *self.scene.lock().unwrap_or_else(PoisonError::into_inner) = self.composed();
    }
}

impl Redraw {
    /// The menus with the loading screen over them; while a map loads, the
    /// screen alone (the menus the load left open would draw over it).
    fn composed(&self) -> Scene {
        match &self.screen {
            Some((screen, true)) => screen.clone(),
            Some((screen, false)) => {
                let mut scene = self.menus.clone();
                scene.shapes.extend(screen.shapes.iter().cloned());
                scene.texts.extend(screen.texts.iter().cloned());
                scene
            }
            None => self.menus.clone(),
        }
    }
}

/// The open screen laid out, from the game's widget tree and keyboard.
fn menus(view: &View<'_>) -> Scene {
    view.export(WIDGET_GLOBALS_EXPORT)
        .and_then(|globals| {
            let tree = view.widget_tree(globals, 0)?;
            let mut scene = screens::scene(&tree);
            if let Some(keyboard) = view
                .export(KEYBOARD_EXPORT)
                .and_then(|at| view.keyboard(at))
            {
                screens::keyboard(&keyboard, &mut scene);
            }
            Some(scene)
        })
        .unwrap_or_default()
}
