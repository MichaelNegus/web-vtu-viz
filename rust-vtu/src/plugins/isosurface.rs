use bevy::{app::{App, Plugin, Startup}, asset::{Assets, RenderAssetUsages}, color::{palettes::css::{ORANGE_RED, RED, WHITE_SMOKE}, ColorToComponents, LinearRgba}, ecs::system::{Commands, ResMut}, math::{primitives::Cuboid, NormedVectorSpace, Vec3}, pbr::{AmbientLight, MeshMaterial3d, StandardMaterial}, render::mesh::{Indices, Mesh, Mesh3d, PrimitiveTopology}, utils::default};

use crate::plugins::{marching_cubes::algorithm::{iso_surface, Triangle}, util};

pub struct IsosurfacePlugin;

impl Plugin for IsosurfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_iso_srf);
    }
}

fn generate_iso_srf(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    // Generate iso surface from cell data
    let points = util::pts_from_csv("assets/csv/points.csv");
    let cells = util::cells_from_csv("assets/csv/connectivity.csv");
    let velocity = util::pts_from_csv("assets/csv/velocity.csv");
    let (min, max) = util::min_max_norm(&velocity);

    let cell_data: Vec<f32> = velocity.iter().map(|&v| Vec3::from_array(v).norm()).collect();
    let iso_val = (min + max) * 0.25;

    let triangles = iso_surface(&points, &cells, &cell_data, iso_val);

    let mesh = create_mesh(&triangles);

    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: LinearRgba::from_f32_array([0.0,1.0,1.0,1.0]).into(),
            alpha_mode:bevy::render::alpha::AlphaMode::Opaque,
            cull_mode: None,
            ..Default::default()
        })),
    ));

    commands.insert_resource(AmbientLight {
        color: WHITE_SMOKE.into(),
        brightness: 1000.,
        ..default()
    });
}

fn create_mesh(tris: &[Triangle]) -> Mesh {

    let mut coords = Vec::with_capacity(tris.len() * 3);
    let mut indices = Vec::with_capacity(tris.len() * 3);

    let mut index = 0;
    for tri in tris{
        coords.push([tri.p1[0],tri.p1[1],tri.p1[2]]);
        coords.push([tri.p2[0],tri.p2[1],tri.p2[2]]);
        coords.push([tri.p3[0],tri.p3[1],tri.p3[2]]);
        indices.push(index);
        indices.push(index+1);
        indices.push(index+2);
        index+=3;
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        coords,
    )
    .with_inserted_indices(Indices::U32(indices))
    .with_computed_smooth_normals()
}