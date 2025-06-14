#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip
}

struct CustomMaterial {
    scale: f32,
};
@group(2) @binding(0) var<uniform> material: CustomMaterial;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let scaled = vertex.position * material.scale;
    out.clip_position = mesh_functions::mesh_position_local_to_clip(world_from_local, vec4(scaled, 1.0));
    out.color = vec4<f32>(1,0,0,1);
    return out;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return mesh.color;
}