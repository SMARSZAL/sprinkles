use bevy::{prelude::*, render::alpha::AlphaMode};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use super::serde_helpers::{is_false, is_true, is_zero_f32};
use crate::textures::preset::TextureRef;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub enum SerializableAlphaMode {
    Opaque,
    Mask {
        cutoff: f32,
    },
    #[default]
    Blend,
    Premultiplied,
    Add,
    Multiply,
    AlphaToCoverage,
}

impl From<SerializableAlphaMode> for AlphaMode {
    fn from(mode: SerializableAlphaMode) -> Self {
        match mode {
            SerializableAlphaMode::Opaque => AlphaMode::Opaque,
            SerializableAlphaMode::Mask { cutoff } => AlphaMode::Mask(cutoff),
            SerializableAlphaMode::Blend => AlphaMode::Blend,
            SerializableAlphaMode::Premultiplied => AlphaMode::Premultiplied,
            SerializableAlphaMode::Add => AlphaMode::Add,
            SerializableAlphaMode::Multiply => AlphaMode::Multiply,
            SerializableAlphaMode::AlphaToCoverage => AlphaMode::AlphaToCoverage,
        }
    }
}

impl From<AlphaMode> for SerializableAlphaMode {
    fn from(mode: AlphaMode) -> Self {
        match mode {
            AlphaMode::Opaque => SerializableAlphaMode::Opaque,
            AlphaMode::Mask(cutoff) => SerializableAlphaMode::Mask { cutoff },
            AlphaMode::Blend => SerializableAlphaMode::Blend,
            AlphaMode::Premultiplied => SerializableAlphaMode::Premultiplied,
            AlphaMode::Add => SerializableAlphaMode::Add,
            AlphaMode::Multiply => SerializableAlphaMode::Multiply,
            AlphaMode::AlphaToCoverage => SerializableAlphaMode::AlphaToCoverage,
        }
    }
}

fn default_base_color() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

fn default_perceptual_roughness() -> f32 {
    0.5
}

fn default_alpha_mode() -> SerializableAlphaMode {
    SerializableAlphaMode::Opaque
}

fn default_reflectance() -> f32 {
    0.5
}

fn default_fog_enabled() -> bool {
    true
}

fn is_default_emissive(v: &[f32; 4]) -> bool {
    *v == [0.0, 0.0, 0.0, 1.0]
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Clone)]
pub struct StandardParticleMaterial {
    #[serde(default = "default_base_color")]
    pub base_color: [f32; 4],

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<TextureRef>,

    #[serde(default, skip_serializing_if = "is_default_emissive")]
    pub emissive: [f32; 4],

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<TextureRef>,

    #[serde(default = "default_alpha_mode")]
    pub alpha_mode: SerializableAlphaMode,

    #[serde(default = "default_perceptual_roughness")]
    pub perceptual_roughness: f32,

    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub metallic: f32,

    #[serde(default = "default_reflectance")]
    pub reflectance: f32,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<TextureRef>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_map_texture: Option<TextureRef>,

    #[serde(default, skip_serializing_if = "is_false")]
    pub double_sided: bool,

    #[serde(default, skip_serializing_if = "is_false")]
    pub unlit: bool,

    #[serde(default = "default_fog_enabled", skip_serializing_if = "is_true")]
    pub fog_enabled: bool,
}

impl Default for StandardParticleMaterial {
    fn default() -> Self {
        Self {
            base_color: default_base_color(),
            base_color_texture: None,
            emissive: [0.0, 0.0, 0.0, 1.0],
            emissive_texture: None,
            perceptual_roughness: default_perceptual_roughness(),
            metallic: 0.0,
            metallic_roughness_texture: None,
            normal_map_texture: None,
            alpha_mode: default_alpha_mode(),
            double_sided: false,
            unlit: false,
            fog_enabled: true,
            reflectance: default_reflectance(),
        }
    }
}

impl StandardParticleMaterial {
    pub fn to_standard_material(&self, asset_server: &AssetServer) -> StandardMaterial {
        let base_color = Color::linear_rgba(
            self.base_color[0],
            self.base_color[1],
            self.base_color[2],
            self.base_color[3],
        );

        let emissive = Color::linear_rgba(
            self.emissive[0],
            self.emissive[1],
            self.emissive[2],
            self.emissive[3],
        );

        StandardMaterial {
            base_color,
            base_color_texture: self
                .base_color_texture
                .as_ref()
                .map(|tex_ref| tex_ref.load(asset_server)),
            emissive: emissive.into(),
            emissive_texture: self
                .emissive_texture
                .as_ref()
                .map(|tex_ref| tex_ref.load(asset_server)),
            perceptual_roughness: self.perceptual_roughness,
            metallic: self.metallic,
            metallic_roughness_texture: self
                .metallic_roughness_texture
                .as_ref()
                .map(|tex_ref| tex_ref.load(asset_server)),
            normal_map_texture: self
                .normal_map_texture
                .as_ref()
                .map(|tex_ref| tex_ref.load(asset_server)),
            alpha_mode: self.alpha_mode.into(),
            double_sided: self.double_sided,
            unlit: self.unlit,
            fog_enabled: self.fog_enabled,
            reflectance: self.reflectance,
            ..default()
        }
    }

    pub fn from_standard_material(material: &StandardMaterial) -> Self {
        let base_color = material.base_color.to_linear();
        let emissive = material.emissive;

        Self {
            base_color: [
                base_color.red,
                base_color.green,
                base_color.blue,
                base_color.alpha,
            ],
            base_color_texture: None,
            emissive: [emissive.red, emissive.green, emissive.blue, emissive.alpha],
            emissive_texture: None,
            perceptual_roughness: material.perceptual_roughness,
            metallic: material.metallic,
            metallic_roughness_texture: None,
            normal_map_texture: None,
            alpha_mode: material.alpha_mode.into(),
            double_sided: material.double_sided,
            unlit: material.unlit,
            fog_enabled: material.fog_enabled,
            reflectance: material.reflectance,
        }
    }

    pub fn cache_key(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for c in self.base_color {
            c.to_bits().hash(&mut hasher);
        }
        self.base_color_texture.hash(&mut hasher);
        for c in self.emissive {
            c.to_bits().hash(&mut hasher);
        }
        self.emissive_texture.hash(&mut hasher);
        self.perceptual_roughness.to_bits().hash(&mut hasher);
        self.metallic.to_bits().hash(&mut hasher);
        self.metallic_roughness_texture.hash(&mut hasher);
        self.normal_map_texture.hash(&mut hasher);
        std::mem::discriminant(&self.alpha_mode).hash(&mut hasher);
        if let SerializableAlphaMode::Mask { cutoff } = self.alpha_mode {
            cutoff.to_bits().hash(&mut hasher);
        }
        self.double_sided.hash(&mut hasher);
        self.unlit.hash(&mut hasher);
        self.fog_enabled.hash(&mut hasher);
        self.reflectance.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum DrawPassMaterial {
    Standard(StandardParticleMaterial),
    CustomShader {
        vertex_shader: Option<String>,
        fragment_shader: Option<String>,
    },
}

impl Default for DrawPassMaterial {
    fn default() -> Self {
        Self::Standard(StandardParticleMaterial::default())
    }
}

impl DrawPassMaterial {
    pub fn cache_key(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        match self {
            Self::Standard(mat) => {
                0u8.hash(&mut hasher);
                mat.cache_key().hash(&mut hasher);
            }
            Self::CustomShader {
                vertex_shader,
                fragment_shader,
            } => {
                1u8.hash(&mut hasher);
                vertex_shader.hash(&mut hasher);
                fragment_shader.hash(&mut hasher);
            }
        }
        hasher.finish()
    }
}
