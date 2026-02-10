use bevy::prelude::*;
use sprinkles::asset::*;

#[test]
fn emitter_data_default_name() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.name, "Emitter");
}

#[test]
fn emitter_data_default_enabled() {
    let emitter = EmitterData::default();
    assert!(emitter.enabled);
}

#[test]
fn emitter_data_default_gravity() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.accelerations.gravity, Vec3::new(0.0, -9.8, 0.0));
}

#[test]
fn emitter_data_default_lifetime() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.time.lifetime, 1.0);
}

#[test]
fn emitter_data_default_particles_amount() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.emission.particles_amount, 8);
}

#[test]
fn emitter_data_default_emission_shape() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.emission.shape, EmissionShape::Point);
}

#[test]
fn emitter_data_default_direction() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.velocities.initial_direction, Vec3::X);
}

#[test]
fn emitter_data_default_spread() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.velocities.spread, 45.0);
}

#[test]
fn emitter_data_default_draw_order() {
    let emitter = EmitterData::default();
    assert_eq!(emitter.draw_pass.draw_order, DrawOrder::Index);
}

#[test]
fn emitter_data_default_shadow_caster() {
    let emitter = EmitterData::default();
    assert!(emitter.draw_pass.shadow_caster);
}

#[test]
fn emitter_data_default_collision() {
    let emitter = EmitterData::default();
    assert!(emitter.collision.mode.is_none());
    assert!(!emitter.collision.use_scale);
}

#[test]
fn emitter_data_default_turbulence() {
    let emitter = EmitterData::default();
    assert!(!emitter.turbulence.enabled);
}

#[test]
fn emitter_data_default_particle_flags() {
    let emitter = EmitterData::default();
    assert!(emitter.particle_flags.is_empty());
}

#[test]
fn collider_data_default() {
    let collider = ColliderData::default();
    assert_eq!(collider.name, "Collider");
    assert!(collider.enabled);
    assert_eq!(collider.position, Vec3::ZERO);
}

#[test]
fn standard_particle_material_default() {
    let mat = StandardParticleMaterial::default();
    assert_eq!(mat.base_color, [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(mat.alpha_mode, SerializableAlphaMode::Opaque);
    assert_eq!(mat.perceptual_roughness, 0.5);
    assert_eq!(mat.metallic, 0.0);
    assert_eq!(mat.reflectance, 0.5);
    assert!(!mat.double_sided);
    assert!(!mat.unlit);
    assert!(mat.fog_enabled);
}
