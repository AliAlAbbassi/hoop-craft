// Shared utility functions for cel-shading pipeline

// Remap value from one range to another
fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    return to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min);
}

// Smooth step with adjustable edge
fn smooth_threshold(value: f32, threshold: f32, smoothness: f32) -> f32 {
    return smoothstep(threshold - smoothness, threshold + smoothness, value);
}

// Fresnel / rim factor
fn fresnel(normal: vec3<f32>, view_dir: vec3<f32>, power: f32) -> f32 {
    return pow(1.0 - saturate(dot(normal, view_dir)), power);
}

// Quantize a value to N discrete steps
fn quantize(value: f32, steps: f32) -> f32 {
    return floor(value * steps) / steps;
}

// Simple hash for noise
fn hash(p: vec2<f32>) -> f32 {
    let h = dot(p, vec2<f32>(127.1, 311.7));
    return fract(sin(h) * 43758.5453123);
}
