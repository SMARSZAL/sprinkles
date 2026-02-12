use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use super::Range;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default, Reflect)]
pub enum CurveMode {
    SingleCurve,
    #[default]
    DoubleCurve,
    Hold,
    Stairs,
    SmoothStairs,
}

impl FromStr for CurveMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SingleCurve" => Ok(Self::SingleCurve),
            "DoubleCurve" => Ok(Self::DoubleCurve),
            "Hold" => Ok(Self::Hold),
            "Stairs" => Ok(Self::Stairs),
            "SmoothStairs" => Ok(Self::SmoothStairs),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default, Reflect)]
pub enum CurveEasing {
    #[default]
    Power,
    Sine,
    Expo,
    Circ,
}

impl FromStr for CurveEasing {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Power" => Ok(Self::Power),
            "Sine" => Ok(Self::Sine),
            "Expo" => Ok(Self::Expo),
            "Circ" => Ok(Self::Circ),
            _ => Err(()),
        }
    }
}

fn default_tension() -> f64 {
    0.0
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Reflect)]
pub struct CurvePoint {
    pub position: f32,
    pub value: f64,
    #[serde(default)]
    pub mode: CurveMode,
    #[serde(default = "default_tension")]
    pub tension: f64,
    #[serde(default)]
    pub easing: CurveEasing,
}

impl CurvePoint {
    pub fn new(position: f32, value: f64) -> Self {
        Self {
            position,
            value,
            mode: CurveMode::default(),
            tension: 0.0,
            easing: CurveEasing::default(),
        }
    }

    pub fn with_mode(mut self, mode: CurveMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_tension(mut self, tension: f64) -> Self {
        self.tension = tension;
        self
    }

    pub fn with_easing(mut self, easing: CurveEasing) -> Self {
        self.easing = easing;
        self
    }
}

fn is_empty_string(s: &Option<String>) -> bool {
    s.as_ref().is_none_or(|s| s.is_empty())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct CurveTexture {
    #[serde(default, skip_serializing_if = "is_empty_string")]
    pub name: Option<String>,
    pub points: Vec<CurvePoint>,
    #[serde(default)]
    pub range: Range,
}

impl Default for CurveTexture {
    fn default() -> Self {
        Self {
            name: Some("Constant".to_string()),
            points: vec![CurvePoint::new(0.0, 1.0), CurvePoint::new(1.0, 1.0)],
            range: Range::new(0.0, 1.0),
        }
    }
}

impl CurveTexture {
    pub fn new(points: Vec<CurvePoint>) -> Self {
        Self {
            name: None,
            points,
            range: Range::default(),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_range(mut self, range: Range) -> Self {
        self.range = range;
        self
    }

    pub fn cache_key(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for point in &self.points {
            point.position.to_bits().hash(&mut hasher);
            (point.value as f32).to_bits().hash(&mut hasher);
            std::mem::discriminant(&point.mode).hash(&mut hasher);
            (point.tension as f32).to_bits().hash(&mut hasher);
        }
        self.range.min.to_bits().hash(&mut hasher);
        self.range.max.to_bits().hash(&mut hasher);
        hasher.finish()
    }

    pub fn is_constant(&self) -> bool {
        if self.points.len() < 2 {
            return true;
        }
        let first_value = self.points[0].value;
        self.points
            .iter()
            .all(|p| (p.value - first_value).abs() < f64::EPSILON)
    }

    pub fn sample(&self, t: f32) -> f32 {
        if self.points.is_empty() {
            return 1.0;
        }
        if self.points.len() == 1 {
            return self.points[0].value as f32;
        }

        let t = t.clamp(0.0, 1.0);

        let mut left_idx = 0;
        let mut right_idx = self.points.len() - 1;

        for (i, point) in self.points.iter().enumerate() {
            if point.position <= t {
                left_idx = i;
            }
        }
        for (i, point) in self.points.iter().enumerate() {
            if point.position >= t {
                right_idx = i;
                break;
            }
        }

        let left = &self.points[left_idx];
        let right = &self.points[right_idx];

        if left_idx == right_idx {
            return left.value as f32;
        }

        let segment_range = right.position - left.position;
        if segment_range <= 0.0 {
            return left.value as f32;
        }

        let local_t = (t - left.position) / segment_range;

        let slope_sign = (right.value - left.value).signum() as f32;
        let effective_tension = right.tension as f32 * slope_sign;
        let curved_t = apply_curve(local_t, right.mode, right.easing, effective_tension);

        (left.value + (right.value - left.value) * curved_t as f64) as f32
    }
}

fn apply_curve(t: f32, mode: CurveMode, easing: CurveEasing, tension: f32) -> f32 {
    match mode {
        CurveMode::SingleCurve => apply_easing(t, easing, tension),
        CurveMode::DoubleCurve => {
            if t < 0.5 {
                let local_t = t * 2.0;
                apply_easing(local_t, easing, tension) * 0.5
            } else {
                let local_t = (t - 0.5) * 2.0;
                0.5 + apply_easing(local_t, easing, -tension) * 0.5
            }
        }
        CurveMode::Hold => 0.0,
        CurveMode::Stairs => {
            let steps = tension_to_steps(tension);
            (t * steps as f32).floor() / (steps - 1).max(1) as f32
        }
        CurveMode::SmoothStairs => {
            let steps = tension_to_steps(tension);
            let step_size = 1.0 / steps as f32;
            let current_step = (t / step_size).floor();
            let local_t = (t - current_step * step_size) / step_size;
            let smooth_t = local_t * local_t * (3.0 - 2.0 * local_t);
            let start = current_step / (steps - 1).max(1) as f32;
            let end = (current_step + 1.0).min(steps as f32 - 1.0) / (steps - 1).max(1) as f32;
            start + (end - start) * smooth_t
        }
    }
}

fn apply_easing(t: f32, easing: CurveEasing, tension: f32) -> f32 {
    match easing {
        CurveEasing::Power => apply_power(t, tension),
        CurveEasing::Sine => apply_sine(t, tension),
        CurveEasing::Expo => apply_expo(t, tension),
        CurveEasing::Circ => apply_circ(t, tension),
    }
}

fn apply_power(t: f32, tension: f32) -> f32 {
    if tension.abs() < f32::EPSILON {
        return t;
    }
    let exp = 1.0 / (1.0 - tension.abs() * 0.999);
    if tension > 0.0 {
        t.powf(exp)
    } else {
        1.0 - (1.0 - t).powf(exp)
    }
}

fn apply_sine(t: f32, tension: f32) -> f32 {
    use std::f32::consts::PI;
    let intensity = tension.abs();
    if intensity < f32::EPSILON {
        return t;
    }
    let eased = if tension >= 0.0 {
        1.0 - (t * PI * 0.5).cos()
    } else {
        (t * PI * 0.5).sin()
    };
    t + (eased - t) * intensity
}

fn apply_expo(t: f32, tension: f32) -> f32 {
    let intensity = tension.abs();
    if intensity < f32::EPSILON {
        return t;
    }
    let eased = if tension >= 0.0 {
        if t <= 0.0 {
            0.0
        } else {
            (2.0_f32).powf(10.0 * (t - 1.0))
        }
    } else {
        if t >= 1.0 {
            1.0
        } else {
            1.0 - (2.0_f32).powf(-10.0 * t)
        }
    };
    t + (eased - t) * intensity
}

fn apply_circ(t: f32, tension: f32) -> f32 {
    let intensity = tension.abs();
    if intensity < f32::EPSILON {
        return t;
    }
    let eased = if tension >= 0.0 {
        1.0 - (1.0 - t * t).sqrt()
    } else {
        (1.0 - (1.0 - t) * (1.0 - t)).sqrt()
    };
    t + (eased - t) * intensity
}

fn tension_to_steps(tension: f32) -> u32 {
    let tension = tension.clamp(0.0, 1.0);
    2 + (64.0 * tension) as u32
}
