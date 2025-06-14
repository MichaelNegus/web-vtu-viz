use bevy::{
    prelude::*,
    render::{
        mesh::MeshTag, render_resource::{AsBindGroup, ShaderRef, ShaderType}, view::NoFrustumCulling
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
    // let points = parse_vtu_pts(VTU_DEMO_FILE).expect("Failed to read pts");

    let size = 50;
    let mut points = Vec::new();
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                let pos = Vec3::new(i as f32, j as f32, k as f32);
                points.push(pos);
            }
        }   
    }

    let mesh_cube = Mesh::from(Cuboid::default());
        let mesh_handle = meshes.add(mesh_cube);
        let material_handle = materials.add(CustomMaterial { scale: 0.25 });

    for (idx, pt) in points.iter().enumerate(){
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
