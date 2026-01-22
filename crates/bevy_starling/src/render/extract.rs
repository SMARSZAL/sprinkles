use bevy::{
    prelude::*,
    render::{Extract, storage::ShaderStorageBuffer},
};

use crate::{
    asset::ParticleSystemAsset,
    core::ParticleSystem3D,
    runtime::{ParticleBufferHandle, ParticleSystemRuntime},
};

use super::EmitterUniforms;

#[derive(Resource, Default)]
pub struct ExtractedParticleSystem {
    pub emitters: Vec<(Entity, ExtractedEmitterData)>,
}

pub struct ExtractedEmitterData {
    pub uniforms: EmitterUniforms,
    pub particle_buffer_handle: Handle<ShaderStorageBuffer>,
    pub amount: u32,
}

pub fn extract_particle_systems(
    mut commands: Commands,
    query: Extract<
        Query<(
            Entity,
            &ParticleSystemRuntime,
            &ParticleBufferHandle,
            &ParticleSystem3D,
        )>,
    >,
    assets: Extract<Res<Assets<ParticleSystemAsset>>>,
    time: Extract<Res<Time>>,
) {
    let mut extracted = ExtractedParticleSystem::default();

    for (entity, runtime, buffer_handle, particle_system) in query.iter() {
        let Some(asset) = assets.get(&particle_system.handle) else {
            continue;
        };

        let Some(emitter) = asset.emitters.first() else {
            continue;
        };

        if !emitter.enabled {
            continue;
        }

        let lifetime = emitter.lifetime;
        // always use actual frame delta for physics simulation
        // fixed_fps only affects emission timing via system_phase
        let delta_time = time.delta_secs();

        let uniforms = EmitterUniforms {
            delta_time,
            system_phase: runtime.system_phase(lifetime),
            prev_system_phase: runtime.prev_system_phase(lifetime),
            cycle: runtime.cycle,

            amount: emitter.amount,
            lifetime: emitter.lifetime,
            lifetime_randomness: emitter.lifetime_randomness,
            emitting: if runtime.emitting { 1 } else { 0 },

            gravity: emitter.process.gravity.into(),
            random_seed: runtime.random_seed,

            initial_velocity: emitter.process.initial_velocity.into(),
            _pad1: 0.0,
            initial_velocity_randomness: emitter.process.initial_velocity_randomness.into(),
            _pad2: 0.0,

            initial_scale: emitter.process.initial_scale,
            initial_scale_randomness: emitter.process.initial_scale_randomness,
            explosiveness: emitter.explosiveness,
            randomness: emitter.randomness,
        };

        extracted.emitters.push((
            entity,
            ExtractedEmitterData {
                uniforms,
                particle_buffer_handle: buffer_handle.particle_buffer.clone(),
                amount: emitter.amount,
            },
        ));
    }

    commands.insert_resource(extracted);
}
