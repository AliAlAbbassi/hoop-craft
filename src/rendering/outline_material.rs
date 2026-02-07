use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{Material, MaterialPipeline, MaterialPipelineKey};
use bevy::shader::ShaderRef;

/// Material for inverted-hull outlines.
/// Uses front-face culling so only the extruded back faces are visible as outlines.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct OutlineMaterial {
    #[uniform(0)]
    pub params: OutlineParams,
}

#[derive(ShaderType, Reflect, Debug, Clone, Copy)]
pub struct OutlineParams {
    pub outline_width: f32,
    pub outline_color: Vec4,
}

impl Default for OutlineMaterial {
    fn default() -> Self {
        Self {
            params: OutlineParams {
                outline_width: 0.003,
                outline_color: Vec4::new(0.1, 0.05, 0.1, 1.0),
            },
        }
    }
}

impl Material for OutlineMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/cel_vertex.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/outline_frag.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // Front-face culling: renders only the back faces (the outlines)
        descriptor.primitive.cull_mode = Some(Face::Front);
        Ok(())
    }
}

pub struct OutlineMaterialPlugin;

impl Plugin for OutlineMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<OutlineMaterial>::default());
    }
}
