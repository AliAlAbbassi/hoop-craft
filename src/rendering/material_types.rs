use bevy::prelude::*;

use super::cel_extension::{CelExtension, CelParams};
use super::cel_material::CelMaterial;

/// Material type identifier for different cel-shading presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialPreset {
    Skin,
    Hair,
    Metal,
    Cloth,
}

impl MaterialPreset {
    /// Create a CelMaterial with preset-appropriate parameters.
    pub fn create_material(&self, base_color: Color) -> CelMaterial {
        let params = match self {
            MaterialPreset::Skin => CelParams {
                shadow_threshold: 0.45,
                shadow_smoothness: 0.04,
                shadow_color: Vec4::new(0.65, 0.45, 0.50, 1.0),
                rim_power: 3.5,
                rim_intensity: 0.3,
                rim_color: Vec4::new(1.0, 0.95, 0.9, 1.0),
                specular_threshold: 0.95,
                specular_smoothness: 0.03,
                specular_intensity: 0.2,
                sss_intensity: 0.5,
                sss_light_color: Vec4::new(1.0, 0.85, 0.6, 1.0),
                sss_shadow_color: Vec4::new(0.9, 0.3, 0.25, 1.0),
                material_type: 0,
                _padding: 0,
            },
            MaterialPreset::Hair => CelParams {
                shadow_threshold: 0.5,
                shadow_smoothness: 0.03,
                shadow_color: Vec4::new(0.5, 0.4, 0.5, 1.0),
                rim_power: 2.5,
                rim_intensity: 0.5,
                rim_color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                specular_threshold: 0.85,
                specular_smoothness: 0.05,
                specular_intensity: 0.7,
                sss_intensity: 0.2,
                sss_light_color: Vec4::new(1.0, 0.9, 0.7, 1.0),
                sss_shadow_color: Vec4::new(0.6, 0.3, 0.4, 1.0),
                material_type: 1,
                _padding: 0,
            },
            MaterialPreset::Metal => CelParams {
                shadow_threshold: 0.55,
                shadow_smoothness: 0.01,
                shadow_color: Vec4::new(0.3, 0.3, 0.4, 1.0),
                rim_power: 4.0,
                rim_intensity: 0.6,
                rim_color: Vec4::new(0.8, 0.85, 1.0, 1.0),
                specular_threshold: 0.7,
                specular_smoothness: 0.02,
                specular_intensity: 1.0,
                sss_intensity: 0.0,
                sss_light_color: Vec4::ZERO,
                sss_shadow_color: Vec4::ZERO,
                material_type: 2,
                _padding: 0,
            },
            MaterialPreset::Cloth => CelParams {
                shadow_threshold: 0.48,
                shadow_smoothness: 0.05,
                shadow_color: Vec4::new(0.55, 0.45, 0.55, 1.0),
                rim_power: 3.0,
                rim_intensity: 0.35,
                rim_color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                specular_threshold: 0.92,
                specular_smoothness: 0.04,
                specular_intensity: 0.15,
                sss_intensity: 0.15,
                sss_light_color: Vec4::new(1.0, 0.9, 0.8, 1.0),
                sss_shadow_color: Vec4::new(0.7, 0.4, 0.5, 1.0),
                material_type: 3,
                _padding: 0,
            },
        };

        CelMaterial {
            base: StandardMaterial {
                base_color,
                ..default()
            },
            extension: CelExtension { params },
        }
    }
}
