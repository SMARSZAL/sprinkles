use bevy::render::render_resource::ShaderType;
use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy, Default, Pod, Zeroable, ShaderType)]
#[repr(C)]
pub struct ParticleData {
    pub position: [f32; 4], // xyz + scale
    pub velocity: [f32; 4], // xyz + lifetime_remaining
    pub color: [f32; 4],    // rgba
    pub custom: [f32; 4],   // age, phase, seed, flags
}

impl ParticleData {
    pub const FLAG_ACTIVE: u32 = 1;

    pub fn is_active(&self) -> bool {
        let flags = self.custom[3].to_bits();
        (flags & Self::FLAG_ACTIVE) != 0
    }
}
