pub mod plugins;

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::plugins::mesh_renderer::CustomMeshPlugin;

/// Main app entry point.
fn main() {
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
