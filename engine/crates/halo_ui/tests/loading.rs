//! The loading screen the plugin draws while the game loads a map: the
//! band of light sweeping with the load, then its fade into the mission.

use opence_halo_ui::loading::{self, Loading, LOADING_EXPORT};
use opence_halo_ui::scene::{HEIGHT, WIDTH};
use opence_runtime::plugins::View;
use opence_ui::loading::{CROSSFADE_SECONDS, SETTLE_SECONDS};

/// the ground: the first shape, covering the screen
fn ground_alpha(scene: &opence_halo_ui::Scene) -> f32 {
    let ground = &scene.shapes[0];
    assert_eq!(ground.rect, [0.0, 0.0, WIDTH, HEIGHT]);
    ground.fill[3]
}

/// the band's lit shapes' horizontal extent
fn band_span(scene: &opence_halo_ui::Scene) -> (f32, f32) {
    let lit: Vec<_> = scene.shapes.iter().filter(|s| s.additive).collect();
    assert!(!lit.is_empty(), "the band is drawn");
    let left = lit.iter().map(|s| s.rect[0]).fold(f32::MAX, f32::min);
    let right = lit
        .iter()
        .map(|s| s.rect[0] + s.rect[2])
        .fold(f32::MIN, f32::max);
    (left, right)
}

#[test]
fn the_band_follows_the_load_across_the_screen() {
    let mut screen = Loading::default();
    let Some(first) = screen.step(Some(0.0), 1.0 / 30.0) else {
        panic!("drawn while loading");
    };
    assert_eq!(ground_alpha(&first), 1.0);
    let (start, _) = band_span(&first);
    let mut last = first;
    for _ in 0..120 {
        let Some(scene) = screen.step(Some(0.6), 1.0 / 30.0) else {
            panic!("still loading");
        };
        last = scene;
    }
    let (moved, right) = band_span(&last);
    assert!(
        moved > start + 1000.0,
        "the band moved right: {start} to {moved}"
    );
    // the band is 128 of the screen's 640 wide
    assert!((right - moved - 128.0 * WIDTH / 640.0).abs() < 1.0);
    assert!(
        last.shapes.iter().any(|s| s.additive && s.fill[3] > 0.2),
        "the band is lit mid-load"
    );
}

#[test]
fn the_screen_fades_into_the_mission_once_the_map_is_ready() {
    let mut screen = Loading::default();
    for _ in 0..30 {
        screen.step(Some(0.5), 1.0 / 30.0);
    }
    let mut alphas = Vec::new();
    let mut frames = 0;
    while let Some(scene) = screen.step(None, 1.0 / 60.0) {
        alphas.push(ground_alpha(&scene));
        frames += 1;
        assert!(frames < 600, "the screen ends");
    }
    let seconds = frames as f32 / 60.0;
    assert!(
        seconds <= SETTLE_SECONDS + CROSSFADE_SECONDS + 0.1,
        "{seconds} s"
    );
    assert!(seconds >= CROSSFADE_SECONDS, "it fades, over {seconds} s");
    assert!(
        alphas.windows(2).all(|w| w[1] <= w[0] + 1e-6),
        "the fade only falls"
    );
    assert!(
        alphas.iter().any(|&a| a > 0.0 && a < 1.0),
        "the fade has steps"
    );
    // and nothing is drawn when no map loads
    assert!(screen.step(None, 1.0 / 60.0).is_none());
}

#[test]
fn the_progress_is_read_from_the_games_view() {
    // struct game_loading_port_view: long active; real progress
    let mut memory = vec![0u8; 64];
    memory[16..20].copy_from_slice(&1u32.to_le_bytes());
    memory[20..24].copy_from_slice(&0.25f32.to_le_bytes());
    let exports = [(LOADING_EXPORT.to_owned(), 16)];
    let read = View::over_with(&mut memory, &exports, loading::progress);
    assert_eq!(read, Some(0.25));
    memory[16..20].copy_from_slice(&0u32.to_le_bytes());
    let read = View::over_with(&mut memory, &exports, loading::progress);
    assert_eq!(read, None);
    // a game without the export loads nothing
    assert_eq!(View::over(&mut memory, loading::progress), None);
}
