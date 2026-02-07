// Cel-shading fragment shader (Genshin Impact style)
// Used as a MaterialExtension on top of StandardMaterial

#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

// Cel-shading parameters (binding slot 100+ to avoid conflict with StandardMaterial)
struct CelParams {
    // Shadow band thresholds
    shadow_threshold: f32,        // Main light/shadow boundary
    shadow_smoothness: f32,       // Transition softness
    shadow_color: vec4<f32>,      // Tint applied in shadow regions

    // Rim lighting
    rim_power: f32,               // Fresnel exponent
    rim_intensity: f32,           // Brightness multiplier
    rim_color: vec4<f32>,         // Rim light color

    // Specular
    specular_threshold: f32,      // Specular cutoff
    specular_smoothness: f32,     // Specular edge softness
    specular_intensity: f32,      // Specular brightness

    // SSS approximation
    sss_intensity: f32,           // Subsurface scattering strength
    sss_light_color: vec4<f32>,   // Warm edge on lit side
    sss_shadow_color: vec4<f32>,  // Cool/red edge on shadow side

    // Material type flags (0=skin, 1=hair, 2=metal, 3=cloth)
    material_type: u32,
    _padding: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> cel: CelParams;

// Fresnel calculation for rim lighting
fn fresnel_factor(normal: vec3<f32>, view_dir: vec3<f32>, power: f32) -> f32 {
    return pow(1.0 - saturate(dot(normal, view_dir)), power);
}

// Quantized diffuse: discrete shadow bands
fn cel_diffuse(ndotl: f32) -> f32 {
    let threshold = cel.shadow_threshold;
    let smooth = cel.shadow_smoothness;
    return smoothstep(threshold - smooth, threshold + smooth, ndotl);
}

// Quantized specular highlight
fn cel_specular(ndoth: f32) -> f32 {
    let threshold = cel.specular_threshold;
    let smooth = cel.specular_smoothness;
    return smoothstep(threshold - smooth, threshold + smooth, ndoth) * cel.specular_intensity;
}

// SSS approximation: color bleeding at light/shadow boundary
fn sss_contribution(ndotl: f32) -> vec3<f32> {
    let boundary_width = 0.15;
    let boundary = smoothstep(cel.shadow_threshold - boundary_width, cel.shadow_threshold, ndotl)
                  - smoothstep(cel.shadow_threshold, cel.shadow_threshold + boundary_width, ndotl);

    // Warm on lit side, cool on shadow side
    let mix_factor = smoothstep(cel.shadow_threshold - 0.05, cel.shadow_threshold + 0.05, ndotl);
    let sss_color = mix(cel.sss_shadow_color.rgb, cel.sss_light_color.rgb, mix_factor);

    return sss_color * boundary * cel.sss_intensity;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;

    // Get base PBR result
    var lit_color = apply_pbr_lighting(pbr_input);

    let base_color = pbr_input.material.base_color.rgb;

    // Reconstruct basic lighting info from PBR
    let normal = pbr_input.world_normal;
    let view = normalize(pbr_input.V);

    // Approximate NdotL from PBR lighting ratio
    // We use the lit vs unlit comparison to determine shadow regions
    let luminance_lit = dot(lit_color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let luminance_base = dot(base_color, vec3<f32>(0.299, 0.587, 0.114));
    let light_ratio = saturate(luminance_lit / max(luminance_base, 0.001));

    // Apply cel-shading quantization
    let cel_factor = cel_diffuse(light_ratio);

    // Cel-shaded base: lit color or shadow-tinted color
    let shadow_tint = base_color * cel.shadow_color.rgb;
    var final_color = mix(shadow_tint, lit_color.rgb, cel_factor);

    // SSS contribution at boundary
    final_color += sss_contribution(light_ratio) * base_color;

    // Rim lighting (masked to lit areas for anime look)
    let rim = fresnel_factor(normal, view, cel.rim_power);
    let rim_mask = cel_factor; // Only show rim on lit side
    final_color += cel.rim_color.rgb * rim * rim_mask * cel.rim_intensity;

    // Specular highlight (quantized)
    let half_dir = normalize(view + normal); // Approximation
    let ndoth = saturate(dot(normal, half_dir));
    let spec = cel_specular(ndoth);
    final_color += vec3<f32>(spec);

    out.color = vec4<f32>(final_color, lit_color.a);

    // Post-processing (fog, tonemapping, etc.)
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
