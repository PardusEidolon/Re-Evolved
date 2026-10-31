//! The Halo UI plugin: the menus drawn anew at the display's resolution,
//! as the design artifact's prototypes draw them, from the game's live
//! widget tree.
//!
//! The game keeps running its own UI (interface/ui_widget.c): its
//! screens, their focus, their input and their strings are the
//! original's. With `display.redraw_menus` on, the game stops drawing the
//! menus' text, their lists and the pictures below a screen's own
//! children; this plugin reads the tree at each `ui_update` and lays the
//! screen out as a [`Scene`], which the app draws over the frame.

pub mod scene;
pub mod screens;

use std::sync::{Arc, Mutex, PoisonError};

use opence_runtime::plugins::{Event, Hook, Plugin, View};
use opence_runtime::widgets::WIDGET_GLOBALS_EXPORT;
use opence_runtime::Tuning;

pub use scene::Scene;

/// The setting that turns the plugin's drawing on.
pub const SETTING: &str = "display.redraw_menus";

/// The plugin: each `ui_update`, the first player's screen laid out into
/// the scene it shares with the app.
pub struct Redraw {
    scene: Arc<Mutex<Scene>>,
    tuning: Tuning,
}

impl Redraw {
    /// The plugin, laying the screens out into `scene` while the game's
    /// setting [`SETTING`] (read through `tuning`) is on.
    #[must_use]
    pub fn new(scene: Arc<Mutex<Scene>>, tuning: Tuning) -> Self {
        Self { scene, tuning }
    }
}

impl Plugin for Redraw {
    fn name(&self) -> &str {
        "the Halo UI"
    }

    fn hooks(&self) -> &[Hook] {
        &[Hook::Ui]
    }

    fn on(&mut self, _event: &Event<'_>, view: &View<'_>) {
        let on = self.tuning.get(SETTING).as_deref() == Some("true");
        let scene = if on {
            view.export(WIDGET_GLOBALS_EXPORT)
                .and_then(|globals| view.widget_tree(globals, 0))
                .map(|tree| screens::scene(&tree))
                .unwrap_or_default()
        } else {
            Scene::default()
        };
        *self.scene.lock().unwrap_or_else(PoisonError::into_inner) = scene;
    }
}
