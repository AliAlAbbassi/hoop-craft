use bevy::prelude::*;
use bevy::core_pipeline::core_3d::graph::Node3d;
use bevy::core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin};
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_graph::{InternedRenderLabel, RenderLabel};
use bevy::render::render_resource::ShaderType;
use bevy::shader::ShaderRef;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenMaterialPlugin::<SobelOutlineEffect>::default());
    }
}

/// Sobel edge detection post-processing effect.
/// Attach to a Camera3d entity to enable silhouette outlines.
#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
pub struct SobelOutlineEffect {
    /// Edge detection sensitivity (lower = more edges).
    pub edge_threshold: f32,
    /// Outline color RGB packed as vec3.
    pub edge_color_r: f32,
    pub edge_color_g: f32,
    pub edge_color_b: f32,
    /// Outline width in texels.
    pub edge_width: f32,
}

impl SobelOutlineEffect {
    pub fn new(threshold: f32, color: Color, width: f32) -> Self {
        let linear = color.to_linear();
        Self {
            edge_threshold: threshold,
            edge_color_r: linear.red,
            edge_color_g: linear.green,
            edge_color_b: linear.blue,
            edge_width: width,
        }
    }
}

impl FullscreenMaterial for SobelOutlineEffect {
    fn fragment_shader() -> ShaderRef {
        "shaders/outline_post.wgsl".into()
    }

    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node3d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node3d::EndMainPassPostProcessing.intern(),
        ]
    }
}
