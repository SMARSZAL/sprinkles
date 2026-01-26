//! Prelude module for convenient imports.
//!
//! ```rust,ignore
//! use bevy_starling::prelude::*;
//! ```

// core plugin
pub use crate::StarlingPlugin;

// asset types
pub use crate::asset::{
    DrawOrder, DrawPassMaterial, EmissionShape, EmitterData, EmitterDrawPass, EmitterDrawing,
    EmitterTime, Gradient as ParticleGradient, GradientInterpolation, GradientStop, ParticleFlags,
    ParticleMesh, ParticleProcessConfig, ParticleProcessDisplay, ParticleProcessDisplayColor,
    ParticleProcessDisplayScale, ParticleProcessSpawnAccelerations, ParticleProcessSpawnPosition,
    ParticleProcessSpawnVelocity, ParticleProcessTurbulence, ParticleSystemAsset,
    ParticleSystemDimension, QuadOrientation, Range as ParticleRange, SerializableAlphaMode,
    SolidOrGradientColor, SplineCurve, StandardParticleMaterial,
};

// runtime types
pub use crate::runtime::{
    EmitterEntity, EmitterRuntime, ParticleMaterial, ParticleMaterialHandle, ParticleSystem2D,
    ParticleSystem3D, ParticleSystemRuntime,
};
