use bevy::{log::tracing_subscriber::reload::Handle, prelude::*};
use vtkio::Vtk;

use crate::parse::vtu_vertices;

pub struct CustomMeshPlugin;

impl Plugin for CustomMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_custom_mesh);
    }
}

const VTU_DEMO_FILE: &str = "assets/box.vtu";

fn parse_vtu_pts(path: &str) -> Result<Vec<[f64; 3]>, String> {
    let file_path = std::path::PathBuf::from(path);
    let vtk_file = Vtk::import(file_path).map_err(|e| e.to_string())?;
    vtu_vertices(vtk_file)
}

fn spawn_custom_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points = parse_vtu_pts(VTU_DEMO_FILE).expect("Failed to read pts");

    let dim = 0.10;
    let mesh_cube = Cuboid {
        half_size: [dim, dim, dim].into(),
    };

    let mesh_handle = meshes.add(mesh_cube);

    for pt in points {
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Srgba::new(1., 1., 0.5, 1.).into(),
                ..default()
            })),
            Transform::from_xyz(pt[0] as f32, pt[1] as f32, pt[2] as f32),
        ));
    }
}
