use bevy::{
    prelude::*,
    render::{
        mesh::MeshTag,
        render_resource::{AsBindGroup, ShaderRef},
    },
};
use vtkio::Vtk;

use crate::parse::vtu_vertices;

pub struct CustomMeshPlugin;

impl Plugin for CustomMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_custom_mesh);
    }
}

const VTU_DEMO_FILE: &str = "assets/vtu/box.vtu";

fn parse_vtu_pts(path: &str) -> Result<Vec<[f64; 3]>, String> {
    let file_path = std::path::PathBuf::from(path);
    let vtk_file = Vtk::import(file_path).map_err(|e| e.to_string())?;
    vtu_vertices(vtk_file)
}

fn spawn_custom_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let points = parse_vtu_pts(VTU_DEMO_FILE).expect("Failed to read pts");

    let dim = 0.10;
    let mesh_cube = Cuboid {
        half_size: [dim, dim, dim].into(),
    };

    let mesh_handle = meshes.add(mesh_cube);
    let material_handle = materials.add(CustomMaterial {
        color: LinearRgba::GREEN,
    });

    for (idx, pt) in points.iter().enumerate() {
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
            MeshTag(idx as u32),
            Transform::from_xyz(pt[0] as f32, pt[1] as f32, pt[2] as f32),
        ));
    }
}

const SHADER_ASSET_PATH: &str = "shaders/instance_shader.wgsl";

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
