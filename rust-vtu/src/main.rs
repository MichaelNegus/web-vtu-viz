pub mod parse;
pub mod plugins;

use bevy::render::view::NoIndirectDrawing;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::plugins::{instanced_renderer::InstanceRenderPlugin, isosurface::IsosurfacePlugin};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() {
    // redirect panic messages to the browser console
    console_error_panic_hook::set_once();

    // optionally redirect logs to the browser console
    wasm_logger::init(wasm_logger::Config::default());

    app();
}

fn main() {
    app();
}

/// Main app entry point.
fn app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#wasm-app".into()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(InstanceRenderPlugin)
        .add_plugins(IsosurfacePlugin)
        .run();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn read_file(name: &str, _data: Vec<u8>) {
    info!("Loaded file {}, data length {}", name, _data.len());
}

/// Set up basic scene with a cube.
fn setup(mut commands: Commands) {
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        NoIndirectDrawing,
    ));
}
