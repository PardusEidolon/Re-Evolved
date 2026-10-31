//! The contract between the engine and the Swift app.
//!
//! The app plays game.wasm, the original's own code, through [`game`]
//! (UniFFI calls; each frame's render commands handed to the app's
//! renderer), shows the loading screen through [`loading`], sums up
//! `--benchmark`'s frame times through [`benchmark`], records a map
//! audit's run through [`audit`], and draws the menus anew from
//! [`menus`].
// (the UniFFI scaffolding is unsafe code; this crate is the only one
// allowed it)
#![allow(unsafe_code)]

pub mod audit;
pub mod bakes;
pub mod benchmark;
pub mod game;
pub mod loading;
pub mod menus;

use mimalloc::MiMalloc;

// (the app's allocator, as the engine's always was)
#[global_allocator]
static ALLOCATOR: MiMalloc = MiMalloc;

uniffi::setup_scaffolding!();

/// The engine's version, as the app shows it.
#[uniffi::export]
pub fn engine_version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// The control socket's path when none is named: the `opence` tool's
/// (`OPENCE_CONTROL_SOCKET`, else `opence/control.sock` in the runtime
/// folder), so the tool finds the app with no `--socket`.
#[uniffi::export]
pub fn default_control_socket() -> String {
    opence_dev::control::default_socket_path()
        .to_string_lossy()
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_socket_is_the_tools() {
        let tools = opence_dev::control::default_socket_path();
        assert_eq!(default_control_socket(), tools.to_string_lossy());
        assert!(
            default_control_socket().ends_with("opence/control.sock")
                || std::env::var_os("OPENCE_CONTROL_SOCKET").is_some()
        );
    }

    #[test]
    fn the_version_is_the_crates() {
        assert_eq!(engine_version(), env!("CARGO_PKG_VERSION"));
    }
}
