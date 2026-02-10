use std::path::Path;

use bevy::math::Vec3;
use bevy::reflect::GetPath;
use sprinkles::asset::{
    ColliderData, EmitterCollisionMode, EmitterData, EmissionShape, ParticleFlags,
    ParticlesColliderShape3D,
};
use sprinkles_editor::project::load_project_from_path;

fn read_f32<T: GetPath>(target: &T, path: &str) -> f32 {
    *target.path::<f32>(path).unwrap()
}

fn write_f32<T: GetPath>(target: &mut T, path: &str, value: f32) {
    *target.path_mut::<f32>(path).unwrap() = value;
}

fn read_bool<T: GetPath>(target: &T, path: &str) -> bool {
    *target.path::<bool>(path).unwrap()
}

fn write_bool<T: GetPath>(target: &mut T, path: &str, value: bool) {
    *target.path_mut::<bool>(path).unwrap() = value;
}

fn read_u32<T: GetPath>(target: &T, path: &str) -> u32 {
    *target.path::<u32>(path).unwrap()
}

fn write_u32<T: GetPath>(target: &mut T, path: &str, value: u32) {
    *target.path_mut::<u32>(path).unwrap() = value;
}

fn read_vec3<T: GetPath>(target: &T, path: &str) -> Vec3 {
    *target.path::<Vec3>(path).unwrap()
}

fn write_vec3<T: GetPath>(target: &mut T, path: &str, value: Vec3) {
    *target.path_mut::<Vec3>(path).unwrap() = value;
}

// --- f32 fields ---

#[test]
fn test_inspect_f32_lifetime() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_f32(&emitter, "time.lifetime"), 1.0);

    write_f32(&mut emitter, "time.lifetime", 3.0);
    assert_eq!(emitter.time.lifetime, 3.0);
}

#[test]
fn test_inspect_f32_noise_strength() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_f32(&emitter, "turbulence.noise_strength"), 1.0);

    write_f32(&mut emitter, "turbulence.noise_strength", 2.0);
    assert_eq!(emitter.turbulence.noise_strength, 2.0);
}

#[test]
fn test_inspect_f32_spread() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_f32(&emitter, "velocities.spread"), 45.0);

    write_f32(&mut emitter, "velocities.spread", 90.0);
    assert_eq!(emitter.velocities.spread, 90.0);
}

#[test]
fn test_inspect_f32_base_size() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_f32(&emitter, "collision.base_size"), 0.01);

    write_f32(&mut emitter, "collision.base_size", 0.1);
    assert_eq!(emitter.collision.base_size, 0.1);
}

#[test]
fn test_inspect_f32_explosiveness() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_f32(&emitter, "time.explosiveness"), 0.0);

    write_f32(&mut emitter, "time.explosiveness", 0.5);
    assert_eq!(emitter.time.explosiveness, 0.5);
}

// --- bool fields ---

#[test]
fn test_inspect_bool_one_shot() {
    let mut emitter = EmitterData::default();
    assert!(!read_bool(&emitter, "time.one_shot"));

    write_bool(&mut emitter, "time.one_shot", true);
    assert!(emitter.time.one_shot);
}

#[test]
fn test_inspect_bool_turbulence_enabled() {
    let mut emitter = EmitterData::default();
    assert!(!read_bool(&emitter, "turbulence.enabled"));

    write_bool(&mut emitter, "turbulence.enabled", true);
    assert!(emitter.turbulence.enabled);
}

#[test]
fn test_inspect_bool_use_scale() {
    let mut emitter = EmitterData::default();
    assert!(!read_bool(&emitter, "collision.use_scale"));

    write_bool(&mut emitter, "collision.use_scale", true);
    assert!(emitter.collision.use_scale);
}

// --- u32 field ---

#[test]
fn test_inspect_u32_particles_amount() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_u32(&emitter, "emission.particles_amount"), 8);

    write_u32(&mut emitter, "emission.particles_amount", 100);
    assert_eq!(emitter.emission.particles_amount, 100);
}

// --- optional u32 fields ---

#[test]
fn test_inspect_u32_fixed_fps() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_u32(&emitter, "time.fixed_fps"), 0);

    write_u32(&mut emitter, "time.fixed_fps", 60);
    assert_eq!(emitter.time.fixed_fps, 60);
}

#[test]
fn test_inspect_optional_fixed_seed() {
    let mut emitter = EmitterData::default();
    assert!(emitter.time.fixed_seed.is_none());

    emitter.time.fixed_seed = Some(42);
    assert_eq!(emitter.time.fixed_seed, Some(42));

    let path_result = emitter.reflect_path("time.fixed_seed");
    assert!(path_result.is_ok(), "reflection path should resolve");
}

// --- Vec3 fields ---

#[test]
fn test_inspect_vec3_offset() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_vec3(&emitter, "emission.offset"), Vec3::ZERO);

    write_vec3(&mut emitter, "emission.offset", Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(emitter.emission.offset, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_inspect_vec3_gravity() {
    let mut emitter = EmitterData::default();
    let gravity = read_vec3(&emitter, "accelerations.gravity");
    assert_eq!(gravity, Vec3::new(0.0, -9.8, 0.0));

    write_vec3(
        &mut emitter,
        "accelerations.gravity",
        Vec3::new(0.0, -15.0, 0.0),
    );
    assert_eq!(emitter.accelerations.gravity, Vec3::new(0.0, -15.0, 0.0));
}

// --- Range fields ---

#[test]
fn test_inspect_range_scale() {
    let mut emitter = EmitterData::default();
    assert_eq!(read_f32(&emitter, "scale.range.min"), 1.0);
    assert_eq!(read_f32(&emitter, "scale.range.max"), 1.0);

    write_f32(&mut emitter, "scale.range.min", 0.5);
    write_f32(&mut emitter, "scale.range.max", 2.0);
    assert_eq!(emitter.scale.range.min, 0.5);
    assert_eq!(emitter.scale.range.max, 2.0);
}

#[test]
fn test_inspect_range_initial_velocity() {
    let mut emitter = EmitterData::default();

    write_f32(&mut emitter, "velocities.initial_velocity.min", 5.0);
    write_f32(&mut emitter, "velocities.initial_velocity.max", 10.0);
    assert_eq!(emitter.velocities.initial_velocity.min, 5.0);
    assert_eq!(emitter.velocities.initial_velocity.max, 10.0);
}

// --- Enum variants ---

#[test]
fn test_inspect_emission_shape_point_to_sphere() {
    let mut emitter = EmitterData::default();
    assert!(matches!(emitter.emission.shape, EmissionShape::Point));

    emitter.emission.shape = EmissionShape::Sphere { radius: 3.0 };
    assert!(matches!(
        emitter.emission.shape,
        EmissionShape::Sphere { radius } if radius == 3.0
    ));
}

#[test]
fn test_inspect_emission_shape_ring_fields() {
    let mut emitter = EmitterData::default();
    emitter.emission.shape = EmissionShape::Ring {
        axis: Vec3::Y,
        height: 0.5,
        radius: 3.0,
        inner_radius: 1.0,
    };

    if let EmissionShape::Ring {
        radius,
        inner_radius,
        height,
        axis,
    } = &emitter.emission.shape
    {
        assert_eq!(*radius, 3.0);
        assert_eq!(*inner_radius, 1.0);
        assert_eq!(*height, 0.5);
        assert_eq!(*axis, Vec3::Y);
    } else {
        panic!("expected Ring variant");
    }
}

#[test]
fn test_inspect_collision_mode_rigid() {
    let mut emitter = EmitterData::default();
    emitter.collision.mode = Some(EmitterCollisionMode::Rigid {
        friction: 0.5,
        bounce: 0.8,
    });

    if let Some(EmitterCollisionMode::Rigid { friction, bounce }) = &emitter.collision.mode {
        assert_eq!(*friction, 0.5);
        assert_eq!(*bounce, 0.8);
    } else {
        panic!("expected Rigid collision mode");
    }
}

#[test]
fn test_inspect_collision_mode_hide_on_contact() {
    let mut emitter = EmitterData::default();
    emitter.collision.mode = Some(EmitterCollisionMode::HideOnContact);

    assert!(matches!(
        emitter.collision.mode,
        Some(EmitterCollisionMode::HideOnContact)
    ));
}

#[test]
fn test_inspect_collision_mode_none() {
    let mut emitter = EmitterData::default();
    emitter.collision.mode = Some(EmitterCollisionMode::Rigid {
        friction: 0.5,
        bounce: 0.8,
    });

    emitter.collision.mode = None;
    assert!(emitter.collision.mode.is_none());
}

// --- Gradient fields ---

#[test]
fn test_inspect_color_over_lifetime_gradient() {
    let mut emitter = EmitterData::default();

    let stops = &emitter.colors.color_over_lifetime.stops;
    assert!(
        !stops.is_empty(),
        "default color_over_lifetime should have stops"
    );

    emitter.colors.color_over_lifetime.stops = vec![
        sprinkles::asset::GradientStop {
            color: [1.0, 0.0, 0.0, 1.0],
            position: 0.0,
        },
        sprinkles::asset::GradientStop {
            color: [0.0, 0.0, 0.0, 0.0],
            position: 1.0,
        },
    ];

    assert_eq!(emitter.colors.color_over_lifetime.stops.len(), 2);
    assert_eq!(
        emitter.colors.color_over_lifetime.stops[0].color,
        [1.0, 0.0, 0.0, 1.0]
    );
}

#[test]
fn test_inspect_initial_color_variant_switch() {
    let mut emitter = EmitterData::default();
    assert!(matches!(
        emitter.colors.initial_color,
        sprinkles::asset::SolidOrGradientColor::Solid { .. }
    ));

    emitter.colors.initial_color =
        sprinkles::asset::SolidOrGradientColor::Gradient {
            gradient: sprinkles::asset::Gradient {
                stops: vec![
                    sprinkles::asset::GradientStop {
                        color: [1.0, 0.0, 0.0, 1.0],
                        position: 0.0,
                    },
                    sprinkles::asset::GradientStop {
                        color: [0.0, 0.0, 1.0, 1.0],
                        position: 1.0,
                    },
                ],
                interpolation: sprinkles::asset::GradientInterpolation::Linear,
            },
        };

    assert!(matches!(
        emitter.colors.initial_color,
        sprinkles::asset::SolidOrGradientColor::Gradient { .. }
    ));
}

// --- Curve fields ---

#[test]
fn test_inspect_scale_over_lifetime_curve() {
    let mut emitter = EmitterData::default();
    assert!(emitter.scale.scale_over_lifetime.is_none());

    emitter.scale.scale_over_lifetime = Some(sprinkles::asset::CurveTexture {
        name: None,
        points: vec![
            sprinkles::asset::CurvePoint::new(0.0, 1.0),
            sprinkles::asset::CurvePoint::new(1.0, 0.0),
        ],
        range: sprinkles::asset::Range::new(0.0, 1.0),
    });

    assert!(emitter.scale.scale_over_lifetime.is_some());
    let curve = emitter.scale.scale_over_lifetime.as_ref().unwrap();
    assert_eq!(curve.points.len(), 2);
}

#[test]
fn test_inspect_angle_over_lifetime_curve() {
    let mut emitter = EmitterData::default();
    assert!(emitter.angle.angle_over_lifetime.is_none());

    emitter.angle.angle_over_lifetime = Some(sprinkles::asset::CurveTexture {
        name: None,
        points: vec![
            sprinkles::asset::CurvePoint::new(0.0, 0.0),
            sprinkles::asset::CurvePoint::new(1.0, 1.0),
        ],
        range: sprinkles::asset::Range::new(0.0, 360.0),
    });

    assert!(emitter.angle.angle_over_lifetime.is_some());
    let curve = emitter.angle.angle_over_lifetime.as_ref().unwrap();
    assert_eq!(curve.range.max, 360.0);
}

// --- Sub-emitter fields ---

#[test]
fn test_inspect_sub_emitter_constant() {
    let mut emitter = EmitterData::default();
    assert!(emitter.sub_emitter.is_none());

    emitter.sub_emitter = Some(sprinkles::asset::SubEmitterConfig {
        mode: sprinkles::asset::SubEmitterMode::Constant,
        target_emitter: 1,
        frequency: 4.0,
        amount: 2,
        keep_velocity: true,
    });

    let sub = emitter.sub_emitter.as_ref().unwrap();
    assert!(matches!(sub.mode, sprinkles::asset::SubEmitterMode::Constant));
    assert_eq!(sub.target_emitter, 1);
}

#[test]
fn test_inspect_sub_emitter_frequency() {
    let mut emitter = EmitterData::default();
    emitter.sub_emitter = Some(sprinkles::asset::SubEmitterConfig {
        mode: sprinkles::asset::SubEmitterMode::Constant,
        target_emitter: 1,
        frequency: 4.0,
        amount: 2,
        keep_velocity: false,
    });

    emitter.sub_emitter.as_mut().unwrap().frequency = 8.0;
    assert_eq!(emitter.sub_emitter.as_ref().unwrap().frequency, 8.0);
}

#[test]
fn test_inspect_sub_emitter_keep_velocity() {
    let mut emitter = EmitterData::default();
    emitter.sub_emitter = Some(sprinkles::asset::SubEmitterConfig {
        mode: sprinkles::asset::SubEmitterMode::Constant,
        target_emitter: 1,
        frequency: 4.0,
        amount: 2,
        keep_velocity: false,
    });

    assert!(!emitter.sub_emitter.as_ref().unwrap().keep_velocity);

    emitter.sub_emitter.as_mut().unwrap().keep_velocity = true;
    assert!(emitter.sub_emitter.as_ref().unwrap().keep_velocity);
}

// --- Particle flags ---

#[test]
fn test_inspect_particle_flags_rotate_y() {
    let mut emitter = EmitterData::default();
    assert!(emitter.particle_flags.is_empty());

    emitter.particle_flags = ParticleFlags::ROTATE_Y;
    assert!(emitter.particle_flags.contains(ParticleFlags::ROTATE_Y));
    assert!(!emitter.particle_flags.contains(ParticleFlags::DISABLE_Z));
}

#[test]
fn test_inspect_particle_flags_combined() {
    let mut emitter = EmitterData::default();

    emitter.particle_flags = ParticleFlags::ROTATE_Y | ParticleFlags::DISABLE_Z;
    assert!(emitter.particle_flags.contains(ParticleFlags::ROTATE_Y));
    assert!(emitter.particle_flags.contains(ParticleFlags::DISABLE_Z));
}

// --- Collider properties ---

#[test]
fn test_inspect_collider_box_shape_size() {
    let mut collider = ColliderData::default();
    collider.shape = ParticlesColliderShape3D::Box {
        size: Vec3::new(10.0, 1.0, 10.0),
    };

    if let ParticlesColliderShape3D::Box { size } = &collider.shape {
        assert_eq!(*size, Vec3::new(10.0, 1.0, 10.0));
    } else {
        panic!("expected Box shape");
    }
}

#[test]
fn test_inspect_collider_position() {
    let mut collider = ColliderData::default();
    assert_eq!(collider.position, Vec3::ZERO);

    collider.position = Vec3::new(0.0, -5.0, 0.0);
    assert_eq!(collider.position, Vec3::new(0.0, -5.0, 0.0));
}

// --- Maximal fixture roundtrip ---

#[test]
fn test_inspect_maximal_fixture_reflection_paths() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("maximal_emitter.ron");
    let asset = load_project_from_path(&path).expect("should load maximal_emitter.ron");
    let emitter = &asset.emitters[0];

    assert_eq!(read_f32(emitter, "time.lifetime"), 3.0);
    assert_eq!(read_f32(emitter, "time.explosiveness"), 0.3);
    assert_eq!(read_f32(emitter, "time.delay"), 0.5);
    assert!(!read_bool(emitter, "time.one_shot"));
    assert_eq!(read_u32(emitter, "time.fixed_fps"), 60);
    assert_eq!(read_u32(emitter, "emission.particles_amount"), 64);
    assert_eq!(read_f32(emitter, "velocities.spread"), 30.0);
    assert!(read_bool(emitter, "turbulence.enabled"));
    assert_eq!(read_f32(emitter, "turbulence.noise_strength"), 2.0);
    assert_eq!(read_f32(emitter, "collision.base_size"), 0.05);
    assert!(read_bool(emitter, "collision.use_scale"));
    assert_eq!(
        read_vec3(emitter, "accelerations.gravity"),
        Vec3::new(0.0, -15.0, 0.0)
    );
    assert_eq!(
        read_vec3(emitter, "emission.offset"),
        Vec3::new(0.5, 1.0, -0.5)
    );
    assert_eq!(read_f32(emitter, "scale.range.min"), 0.5);
    assert_eq!(read_f32(emitter, "scale.range.max"), 2.0);
}
