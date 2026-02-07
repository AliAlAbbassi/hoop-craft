use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;
use bevy::pbr::MaterialExtension;

/// Cel-shading parameters passed to the GPU.
/// Binding starts at slot 100 to avoid conflicts with StandardMaterial.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct CelExtension {
    #[uniform(100)]
    pub params: CelParams,
}

/// GPU-side cel-shading parameters.
/// Must match the struct layout in cel_material.wgsl.
#[derive(ShaderType, Reflect, Debug, Clone, Copy)]
pub struct CelParams {
    pub shadow_threshold: f32,
    pub shadow_smoothness: f32,
    pub shadow_color: Vec4,

    pub rim_power: f32,
    pub rim_intensity: f32,
    pub rim_color: Vec4,

    pub specular_threshold: f32,
    pub specular_smoothness: f32,
    pub specular_intensity: f32,

    pub sss_intensity: f32,
    pub sss_light_color: Vec4,
    pub sss_shadow_color: Vec4,

    pub material_type: u32,
    pub _padding: u32,
}

impl Default for CelParams {
    fn default() -> Self {
        Self {
            shadow_threshold: 0.5,
            shadow_smoothness: 0.02,
            shadow_color: Vec4::new(0.55, 0.45, 0.55, 1.0),

            rim_power: 3.0,
            rim_intensity: 0.4,
            rim_color: Vec4::new(1.0, 1.0, 1.0, 1.0),

            specular_threshold: 0.9,
            specular_smoothness: 0.02,
            specular_intensity: 0.5,

            sss_intensity: 0.3,
            sss_light_color: Vec4::new(1.0, 0.9, 0.7, 1.0),   // warm yellow
            sss_shadow_color: Vec4::new(0.8, 0.3, 0.3, 1.0),  // warm red

            material_type: 0,
            _padding: 0,
        }
    }
}

impl Default for CelExtension {
    fn default() -> Self {
        Self {
            params: CelParams::default(),
        }
    }
}

impl MaterialExtension for CelExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/cel_material.wgsl".into()
    }
}
