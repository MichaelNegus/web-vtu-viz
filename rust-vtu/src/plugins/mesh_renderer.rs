use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

pub struct CustomMeshPlugin;

impl Plugin for CustomMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_custom_mesh);
    }
}

fn spawn_custom_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Define vertices and indices
    // let _: Vec<[f32; 3]> = vec![([0.0, 1.0, 0.0]), ([0.0, 1.0, 0.0]), ([0.0, 1.0, 0.0])];

    // let vertices: Vec<[f32; 3]> = vec![([0.0, 0.0, 0.0]), ([1.0, 0.0, 0.0]), ([0.5, 1.0, 0.0])];

    // let _custom_mesh =
    //     bevy::prelude::Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
    //         .with_inserted_attribute(bevy::prelude::Mesh::ATTRIBUTE_POSITION, vertices)
    //         .with_inserted_indices(Indices::U16(vec![0, 1, 2]));

    let dim = 0.25;
    let mesh_cube = Cuboid {
        half_size: [dim, dim, dim].into(),
    };

    let mesh_handle = meshes.add(mesh_cube);

    let num_y = 25;
    let num_x = 25;
    let num_z = 25;
    let spacing = 1.;
    for z in 0..num_y {
        for y in 0..num_z {
            for x in 0..num_x {
                let x01 = x as f32 * spacing;
                let y01 = y as f32 * spacing;
                let z01 = z as f32 * spacing;
                // sphere
                commands.spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: Srgba::rgb(x as f32 / num_x as f32, y as f32 / num_y as f32, z as f32 / num_z as f32).into(),
                        ..default()
                    })),
                    Transform::from_xyz(
                        x01, y01, z01
                    ),
                ));
            }
        }
    }
}
