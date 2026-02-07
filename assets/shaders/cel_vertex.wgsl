// Inverted hull outline vertex shader
// Extrudes vertices along normals for outline effect
// Used with front-face culling so only back faces (outlines) are visible

#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct OutlineParams {
    outline_width: f32,
    outline_color: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> outline: OutlineParams;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    #ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
    #endif
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // Extrude vertex along normal for outline
    let extruded = vertex.position + vertex.normal * outline.outline_width;

    // Transform to clip space
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(extruded, 1.0),
    );

    out.color = outline.outline_color;

    return out;
}
