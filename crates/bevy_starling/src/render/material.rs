use bevy::{
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline},
    prelude::*,
    render::{
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
        },
        storage::ShaderStorageBuffer,
    },
    shader::ShaderRef,
};

const SHADER_ASSET_PATH: &str = "embedded://bevy_starling/shaders/particle_material.wgsl";

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct ParticleMaterialExtension {
    /// sorted particle data buffer (written in draw order by the sort compute shader)
    #[storage(100, read_only)]
    pub sorted_particles: Handle<ShaderStorageBuffer>,
    /// maximum number of particles
    #[uniform(101)]
    pub max_particles: u32,
}

impl MaterialExtension for ParticleMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        _descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // depth writing is kept enabled (default) because we use per-particle
        // depth bias in the vertex shader to establish consistent render order.
        // particles sorted later get smaller depth values, ensuring they "win"
        // the depth test and appear on top regardless of GPU processing order.
        Ok(())
    }
}
