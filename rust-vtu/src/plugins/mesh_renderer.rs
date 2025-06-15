use bevy::{
    prelude::*,
    render::{
        mesh::MeshTag,
        render_resource::{AsBindGroup, ShaderRef},
    },
};

use crate::plugins::util;

pub struct CustomMeshPlugin;

impl Plugin for CustomMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default());
        app.add_systems(Startup, spawn_custom_mesh);
    }
}

fn spawn_custom_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // let points = parse_vtu_pts(VTU_DEMO_FILE).expect("Failed to read pts");

    let points = util::dummy_pts(50);

    let mesh_cube = Mesh::from(Cuboid::default());
    let mesh_handle = meshes.add(mesh_cube);
    let material_handle = materials.add(CustomMaterial { scale: 0.25 });

    for (idx, pt) in points.iter().enumerate() {
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
            MeshTag(idx as u32),
            Transform::from_xyz(pt[0], pt[1], pt[2]),
        ));
    }
}

const SHADER_ASSET_PATH: &str = "shaders/instance_shader.wgsl";

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    scale: f32,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
