use bevy::{
    app::{App, Plugin, Startup},
    asset::{Asset, Assets, Handle},
    ecs::{
        resource::Resource,
        system::{Commands, ResMut},
    },
    math::{primitives::Cuboid, Vec3},
    pbr::{Material, MaterialPlugin, MeshMaterial3d},
    reflect::TypePath,
    render::{
        mesh::{Mesh, Mesh3d, MeshTag},
        render_resource::{AsBindGroup, ShaderRef},
        storage::ShaderStorageBuffer,
    },
    transform::components::Transform,
};

use crate::plugins::util::dummy_pts;

pub struct StorageBufferRenderer;

impl Plugin for StorageBufferRenderer {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<StorageMaterial>::default());
        app.add_systems(Startup, setup);
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<StorageMaterial>>,
) {
    let points = dummy_pts(50);

    // Example data for the storage buffer
    let color_data: Vec<[f32; 4]> = vec![
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [1.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0, 1.0],
        [0.5, 0.75, 0.3, 1.0],
    ];

    let num_cols = color_data.len();
    let colors = buffers.add(ShaderStorageBuffer::from(color_data));

    let mesh_handle = meshes.add(Cuboid::from_size(Vec3::splat(0.25)));
    // Create the custom material with the storage buffer
    let material_handle = materials.add(StorageMaterial {
        colors: colors.clone(),
    });

    commands.insert_resource(CustomMaterialHandle(material_handle.clone()));

    // Spawn cubes with the custom material
    for (idx, pt) in points.iter().enumerate() {
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
            MeshTag((idx % num_cols).try_into().unwrap()),
            Transform::from_xyz(pt[0], pt[1], pt[2]),
        ));
    }
}

const SHADER_ASSET_PATH: &str = "shaders/storage_shader.wgsl";

// Holds handles to the custom materials
#[derive(Resource)]
struct CustomMaterialHandle(Handle<StorageMaterial>);

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct StorageMaterial {
    #[storage(0, read_only)]
    colors: Handle<ShaderStorageBuffer>,
}

impl Material for StorageMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
