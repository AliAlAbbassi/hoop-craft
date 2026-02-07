// Sobel edge detection post-process shader
// Detects edges in the scene for silhouette outlines

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct SobelParams {
    edge_threshold: f32,
    edge_color_r: f32,
    edge_color_g: f32,
    edge_color_b: f32,
    edge_width: f32,
}

@group(0) @binding(2) var<uniform> params: SobelParams;

// Sample luminance at offset
fn sample_luma(uv: vec2<f32>, offset: vec2<f32>) -> f32 {
    let color = textureSample(screen_texture, texture_sampler, uv + offset);
    return dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let tex_size = vec2<f32>(textureDimensions(screen_texture));
    let texel = params.edge_width / tex_size;

    // Sobel kernel sampling (3x3)
    let tl = sample_luma(in.uv, vec2<f32>(-texel.x,  texel.y));
    let tm = sample_luma(in.uv, vec2<f32>( 0.0,      texel.y));
    let tr = sample_luma(in.uv, vec2<f32>( texel.x,  texel.y));
    let ml = sample_luma(in.uv, vec2<f32>(-texel.x,  0.0));
    let mr = sample_luma(in.uv, vec2<f32>( texel.x,  0.0));
    let bl = sample_luma(in.uv, vec2<f32>(-texel.x, -texel.y));
    let bm = sample_luma(in.uv, vec2<f32>( 0.0,     -texel.y));
    let br = sample_luma(in.uv, vec2<f32>( texel.x, -texel.y));

    // Sobel operators
    let gx = -tl - 2.0 * ml - bl + tr + 2.0 * mr + br;
    let gy = -tl - 2.0 * tm - tr + bl + 2.0 * bm + br;
    let edge = sqrt(gx * gx + gy * gy);

    // Original color
    let original = textureSample(screen_texture, texture_sampler, in.uv);

    // Blend edge onto original
    let edge_factor = smoothstep(params.edge_threshold, params.edge_threshold + 0.05, edge);
    let edge_color = vec4<f32>(params.edge_color_r, params.edge_color_g, params.edge_color_b, 1.0);

    return mix(original, edge_color, edge_factor);
}
