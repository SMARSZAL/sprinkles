mod curve;
mod gradient;
mod particle_material;
pub(crate) mod serde_helpers;

pub use curve::{CurveEasing, CurveMode, CurvePoint, CurveTexture};
pub use gradient::{Gradient, GradientInterpolation, GradientStop, SolidOrGradientColor};
pub use particle_material::{DrawPassMaterial, SerializableAlphaMode, StandardParticleMaterial};

use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use serde_helpers::*;

#[derive(Default, TypePath)]
pub struct ParticleSystemAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ParticleSystemAssetLoaderError {
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse RON: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for ParticleSystemAssetLoader {
    type Asset = ParticleSystemAsset;
    type Settings = ();
    type Error = ParticleSystemAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = ron::de::from_bytes::<ParticleSystemAsset>(&bytes)?;
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ParticleFlags: u32 {
        const ROTATE_Y = 1 << 1;
        const DISABLE_Z = 1 << 2;

        // TODO: requires implementing damping
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Reflect)]
pub enum ParticleSystemDimension {
    #[default]
    D3,
    D2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, Reflect)]
pub enum DrawOrder {
    #[default]
    Index,
    Lifetime,
    ReverseLifetime,
    ViewDepth,
}

impl DrawOrder {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterTime {
    #[serde(default = "default_lifetime")]
    pub lifetime: f32,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub lifetime_randomness: f32,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub delay: f32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub one_shot: bool,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub explosiveness: f32,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub spawn_time_randomness: f32,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub fixed_fps: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_seed: Option<u32>,
}

fn default_lifetime() -> f32 {
    1.0
}

impl Default for EmitterTime {
    fn default() -> Self {
        Self {
            lifetime: 1.0,
            lifetime_randomness: 0.0,
            delay: 0.0,
            one_shot: false,
            explosiveness: 0.0,
            spawn_time_randomness: 0.0,
            fixed_fps: 0,
            fixed_seed: None,
        }
    }
}

impl EmitterTime {
    pub fn total_duration(&self) -> f32 {
        self.delay + self.lifetime
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterData {
    pub name: String,
    #[serde(default = "default_enabled", skip_serializing_if = "is_true")]
    pub enabled: bool,

    #[serde(default, skip_serializing_if = "is_zero_vec3")]
    pub position: Vec3,

    #[serde(default)]
    pub time: EmitterTime,

    #[serde(default)]
    pub draw_pass: EmitterDrawPass,

    #[serde(default)]
    pub emission: EmitterEmission,

    #[serde(default)]
    pub scale: EmitterScale,

    #[serde(default, skip_serializing_if = "EmitterAngle::should_skip")]
    pub angle: EmitterAngle,

    #[serde(default)]
    pub colors: EmitterColors,

    #[serde(default)]
    pub velocities: EmitterVelocities,

    #[serde(default)]
    pub accelerations: EmitterAccelerations,

    #[serde(default, skip_serializing_if = "EmitterTurbulence::should_skip")]
    pub turbulence: EmitterTurbulence,

    #[serde(default)]
    pub collision: EmitterCollision,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_emitter: Option<SubEmitterConfig>,

    #[serde(default)]
    #[reflect(ignore)]
    pub particle_flags: ParticleFlags,
}

fn default_enabled() -> bool {
    true
}

impl Default for EmitterData {
    fn default() -> Self {
        Self {
            name: "Emitter".to_string(),
            enabled: true,
            position: Vec3::ZERO,
            time: EmitterTime::default(),
            draw_pass: EmitterDrawPass::default(),
            emission: EmitterEmission::default(),
            scale: EmitterScale::default(),
            angle: EmitterAngle::default(),
            colors: EmitterColors::default(),
            velocities: EmitterVelocities::default(),
            accelerations: EmitterAccelerations::default(),
            turbulence: EmitterTurbulence::default(),
            collision: EmitterCollision::default(),
            sub_emitter: None,
            particle_flags: ParticleFlags::empty(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default, Reflect)]
pub enum TransformAlign {
    #[default]
    Billboard,
    YToVelocity,
    BillboardYToVelocity,
    BillboardFixedY,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterDrawPass {
    #[serde(default, skip_serializing_if = "DrawOrder::is_default")]
    pub draw_order: DrawOrder,
    pub mesh: ParticleMesh,
    #[serde(default)]
    pub material: DrawPassMaterial,
    #[serde(default = "default_shadow_caster", skip_serializing_if = "is_true")]
    pub shadow_caster: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_align: Option<TransformAlign>,
}

fn default_shadow_caster() -> bool {
    true
}

impl Default for EmitterDrawPass {
    fn default() -> Self {
        Self {
            draw_order: DrawOrder::default(),
            mesh: ParticleMesh::default(),
            material: DrawPassMaterial::default(),
            shadow_caster: true,
            transform_align: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub enum QuadOrientation {
    FaceX,
    FaceY,
    #[default]
    FaceZ,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum ParticleMesh {
    Quad {
        #[serde(default)]
        orientation: QuadOrientation,
        #[serde(default = "default_quad_size")]
        size: Vec2,
        #[serde(default, skip_serializing_if = "is_zero_vec2")]
        subdivide: Vec2,
    },
    Sphere {
        #[serde(default = "default_sphere_radius")]
        radius: f32,
    },
    Cuboid {
        half_size: Vec3,
    },
    Cylinder {
        top_radius: f32,
        bottom_radius: f32,
        height: f32,
        radial_segments: u32,
        rings: u32,
        cap_top: bool,
        cap_bottom: bool,
    },
    Prism {
        #[serde(default = "default_prism_left_to_right")]
        left_to_right: f32,
        #[serde(default = "default_prism_size")]
        size: Vec3,
        #[serde(default, skip_serializing_if = "is_zero_vec3")]
        subdivide: Vec3,
    },
}

fn default_quad_size() -> Vec2 {
    Vec2::ONE
}

fn default_sphere_radius() -> f32 {
    1.0
}

fn default_prism_left_to_right() -> f32 {
    0.5
}

fn default_prism_size() -> Vec3 {
    Vec3::splat(1.0)
}

impl Default for ParticleMesh {
    fn default() -> Self {
        Self::Sphere { radius: 1.0 }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Reflect)]
pub struct Range {
    #[serde(default)]
    pub min: f32,
    #[serde(default = "default_one_f32")]
    pub max: f32,
}

fn default_one_f32() -> f32 {
    1.0
}

impl Default for Range {
    fn default() -> Self {
        Self { min: 0.0, max: 1.0 }
    }
}

impl Range {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn span(&self) -> f32 {
        let span = self.max - self.min;
        if span.abs() < f32::EPSILON { 1.0 } else { span }
    }

    fn is_zero(&self) -> bool {
        self.min == 0.0 && self.max == 0.0
    }

    fn zero() -> Self {
        Self { min: 0.0, max: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub enum EmissionShape {
    #[default]
    Point,
    Sphere {
        radius: f32,
    },
    SphereSurface {
        radius: f32,
    },
    Box {
        extents: Vec3,
    },
    Ring {
        axis: Vec3,
        height: f32,
        radius: f32,
        inner_radius: f32,
    },
}

impl EmissionShape {
    fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

fn default_emission_scale() -> Vec3 {
    Vec3::ONE
}

fn default_particles_amount() -> u32 {
    8
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterEmission {
    #[serde(default, skip_serializing_if = "is_zero_vec3")]
    pub offset: Vec3,
    #[serde(
        default = "default_emission_scale",
        skip_serializing_if = "is_one_vec3"
    )]
    pub scale: Vec3,
    #[serde(default, skip_serializing_if = "EmissionShape::is_default")]
    pub shape: EmissionShape,
    #[serde(default = "default_particles_amount")]
    pub particles_amount: u32,
}

impl Default for EmitterEmission {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            scale: Vec3::ONE,
            shape: EmissionShape::default(),
            particles_amount: 8,
        }
    }
}

fn default_scale_range() -> Range {
    Range { min: 1.0, max: 1.0 }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterScale {
    #[serde(default = "default_scale_range")]
    pub range: Range,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_over_lifetime: Option<CurveTexture>,
}

impl Default for EmitterScale {
    fn default() -> Self {
        Self {
            range: default_scale_range(),
            scale_over_lifetime: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterColors {
    #[serde(default)]
    pub initial_color: SolidOrGradientColor,
    #[serde(default = "Gradient::white")]
    pub color_over_lifetime: Gradient,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha_over_lifetime: Option<CurveTexture>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emission_over_lifetime: Option<CurveTexture>,
}

impl Default for EmitterColors {
    fn default() -> Self {
        Self {
            initial_color: SolidOrGradientColor::default(),
            color_over_lifetime: Gradient::white(),
            alpha_over_lifetime: None,
            emission_over_lifetime: None,
        }
    }
}

fn default_direction() -> Vec3 {
    Vec3::X
}

fn default_spread() -> f32 {
    45.0
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct AnimatedVelocity {
    #[serde(default = "Range::zero", skip_serializing_if = "Range::is_zero")]
    pub velocity: Range,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub velocity_over_lifetime: Option<CurveTexture>,
}

impl Default for AnimatedVelocity {
    fn default() -> Self {
        Self {
            velocity: Range::zero(),
            velocity_over_lifetime: None,
        }
    }
}

impl AnimatedVelocity {
    fn is_default(&self) -> bool {
        self.velocity.is_zero() && self.velocity_over_lifetime.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterAngle {
    #[serde(default = "Range::zero", skip_serializing_if = "Range::is_zero")]
    pub range: Range,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle_over_lifetime: Option<CurveTexture>,
}

impl Default for EmitterAngle {
    fn default() -> Self {
        Self {
            range: Range::zero(),
            angle_over_lifetime: None,
        }
    }
}

impl EmitterAngle {
    fn should_skip(&self) -> bool {
        self.range.is_zero() && self.angle_over_lifetime.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterVelocities {
    #[serde(default = "default_direction")]
    pub initial_direction: Vec3,
    #[serde(default = "default_spread")]
    pub spread: f32,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub flatness: f32,
    #[serde(default = "Range::zero", skip_serializing_if = "Range::is_zero")]
    pub initial_velocity: Range,
    #[serde(default)]
    pub radial_velocity: AnimatedVelocity,
    #[serde(default)]
    pub angular_velocity: AnimatedVelocity,
    #[serde(default, skip_serializing_if = "is_zero_vec3")]
    pub pivot: Vec3,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub inherit_ratio: f32,
}

impl Default for EmitterVelocities {
    fn default() -> Self {
        Self {
            initial_direction: Vec3::X,
            spread: 45.0,
            flatness: 0.0,
            initial_velocity: Range::zero(),
            radial_velocity: AnimatedVelocity::default(),
            angular_velocity: AnimatedVelocity::default(),
            pivot: Vec3::ZERO,
            inherit_ratio: 0.0,
        }
    }
}

fn default_gravity() -> Vec3 {
    Vec3::new(0.0, -9.8, 0.0)
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterAccelerations {
    #[serde(default = "default_gravity")]
    pub gravity: Vec3,
}

impl Default for EmitterAccelerations {
    fn default() -> Self {
        Self {
            gravity: Vec3::new(0.0, -9.8, 0.0),
        }
    }
}

fn default_turbulence_noise_strength() -> f32 {
    1.0
}

fn default_turbulence_noise_scale() -> f32 {
    2.5
}

fn default_turbulence_influence() -> Range {
    Range { min: 0.0, max: 0.1 }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterTurbulence {
    #[serde(default, skip_serializing_if = "is_false")]
    pub enabled: bool,
    #[serde(default = "default_turbulence_noise_strength")]
    pub noise_strength: f32,
    #[serde(default = "default_turbulence_noise_scale")]
    pub noise_scale: f32,
    #[serde(default, skip_serializing_if = "is_zero_vec3")]
    pub noise_speed: Vec3,
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub noise_speed_random: f32,
    #[serde(default = "default_turbulence_influence")]
    pub influence: Range,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub influence_over_lifetime: Option<CurveTexture>,
}

impl Default for EmitterTurbulence {
    fn default() -> Self {
        Self {
            enabled: false,
            noise_strength: default_turbulence_noise_strength(),
            noise_scale: default_turbulence_noise_scale(),
            noise_speed: Vec3::ZERO,
            noise_speed_random: 0.0,
            influence: default_turbulence_influence(),
            influence_over_lifetime: None,
        }
    }
}

impl EmitterTurbulence {
    fn should_skip(&self) -> bool {
        if self.enabled {
            return false;
        }
        let d = Self::default();
        self.noise_strength == d.noise_strength
            && self.noise_scale == d.noise_scale
            && self.noise_speed == d.noise_speed
            && self.noise_speed_random == d.noise_speed_random
            && self.influence.min == d.influence.min
            && self.influence.max == d.influence.max
            && self.influence_over_lifetime.is_none()
    }
}

fn default_collision_base_size() -> f32 {
    0.01
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum EmitterCollisionMode {
    Rigid { friction: f32, bounce: f32 },
    HideOnContact,
}

impl Default for EmitterCollisionMode {
    fn default() -> Self {
        Self::Rigid {
            friction: 0.0,
            bounce: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct EmitterCollision {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<EmitterCollisionMode>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub use_scale: bool,
    #[serde(default = "default_collision_base_size")]
    pub base_size: f32,
}

impl Default for EmitterCollision {
    fn default() -> Self {
        Self {
            mode: None,
            base_size: default_collision_base_size(),
            use_scale: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum SubEmitterMode {
    Constant,
    AtEnd,
    AtCollision,
    AtStart,
}

fn default_sub_emitter_frequency() -> f32 {
    4.0
}

fn default_sub_emitter_amount() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct SubEmitterConfig {
    pub mode: SubEmitterMode,
    pub target_emitter: usize,
    #[serde(default = "default_sub_emitter_frequency")]
    pub frequency: f32,
    #[serde(default = "default_sub_emitter_amount")]
    pub amount: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub keep_velocity: bool,
}

impl Default for SubEmitterConfig {
    fn default() -> Self {
        Self {
            mode: SubEmitterMode::Constant,
            target_emitter: 0,
            frequency: default_sub_emitter_frequency(),
            amount: default_sub_emitter_amount(),
            keep_velocity: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum ParticlesColliderShape3D {
    Box { size: Vec3 },
    Sphere { radius: f32 },
}

impl Default for ParticlesColliderShape3D {
    fn default() -> Self {
        Self::Sphere { radius: 1.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct ColliderData {
    pub name: String,
    #[serde(default = "default_enabled", skip_serializing_if = "is_true")]
    pub enabled: bool,
    pub shape: ParticlesColliderShape3D,
    #[serde(default)]
    pub position: Vec3,
}

impl Default for ColliderData {
    fn default() -> Self {
        Self {
            name: "Collider".to_string(),
            enabled: true,
            shape: ParticlesColliderShape3D::default(),
            position: Vec3::ZERO,
        }
    }
}

#[derive(Asset, TypePath, Debug, Clone, Serialize, Deserialize)]
pub struct ParticleSystemAsset {
    pub name: String,
    pub dimension: ParticleSystemDimension,
    pub emitters: Vec<EmitterData>,
    #[serde(default)]
    pub colliders: Vec<ColliderData>,
}
