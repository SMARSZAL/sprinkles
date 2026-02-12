use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, Hash, Reflect)]
pub enum GradientInterpolation {
    Steps,
    #[default]
    Linear,
    Smoothstep,
}

impl GradientInterpolation {
    pub(crate) fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct GradientStop {
    pub color: [f32; 4],
    pub position: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Gradient {
    pub stops: Vec<GradientStop>,
    #[serde(default, skip_serializing_if = "GradientInterpolation::is_default")]
    pub interpolation: GradientInterpolation,
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            stops: vec![
                GradientStop {
                    color: [0.0, 0.0, 0.0, 1.0],
                    position: 0.0,
                },
                GradientStop {
                    color: [1.0, 1.0, 1.0, 1.0],
                    position: 1.0,
                },
            ],
            interpolation: GradientInterpolation::Linear,
        }
    }
}

impl Gradient {
    pub fn white() -> Self {
        Self {
            stops: vec![
                GradientStop {
                    color: [1.0, 1.0, 1.0, 1.0],
                    position: 0.0,
                },
                GradientStop {
                    color: [1.0, 1.0, 1.0, 1.0],
                    position: 1.0,
                },
            ],
            interpolation: GradientInterpolation::Linear,
        }
    }

    pub fn cache_key(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for stop in &self.stops {
            for c in stop.color {
                c.to_bits().hash(&mut hasher);
            }
            stop.position.to_bits().hash(&mut hasher);
        }
        self.interpolation.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Clone)]
pub enum SolidOrGradientColor {
    Solid { color: [f32; 4] },
    Gradient { gradient: Gradient },
}

impl Default for SolidOrGradientColor {
    fn default() -> Self {
        Self::Solid {
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

impl SolidOrGradientColor {
    pub fn solid(color: [f32; 4]) -> Self {
        Self::Solid { color }
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, Self::Solid { .. })
    }

    pub fn is_gradient(&self) -> bool {
        matches!(self, Self::Gradient { .. })
    }

    pub fn as_solid_color(&self) -> Option<[f32; 4]> {
        match self {
            Self::Solid { color } => Some(*color),
            _ => None,
        }
    }
}
