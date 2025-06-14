pub mod plugins;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::plugins::mesh_renderer::CustomMeshPlugin;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_js() {
    // redirect panic messages to the browser console
    console_error_panic_hook::set_once();

    // optionally redirect logs to the browser console
    wasm_logger::init(wasm_logger::Config::default());

    app();
}

// #[cfg(not(target_arch = "wasm32"))]
fn main() {
    app();
}

/// Main app entry point.
fn app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(CustomMeshPlugin)
        .run();
}

/// Set up basic scene with a cube.
fn setup(mut commands: Commands) {
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
