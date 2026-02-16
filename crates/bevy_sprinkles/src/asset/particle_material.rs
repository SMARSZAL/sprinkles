use bevy::{prelude::*, render::alpha::AlphaMode};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

use super::serde_helpers::{is_false, is_true, is_zero_f32};
use crate::textures::preset::TextureRef;

/// Sets how a material's base color alpha channel is used for transparency, copied from Bevy's [`AlphaMode`](bevy::render::alpha::AlphaMode).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub enum SerializableAlphaMode {
    /// Base color alpha values are overridden to be fully opaque (1.0).
    Opaque,
    /// Reduce transparency to fully opaque or fully transparent based on a threshold.
    ///
    /// Compares the base color alpha value to the specified threshold.
    /// If the value is below the threshold, considers the color to be fully transparent
    /// (alpha is set to 0.0). If it is equal to or above the threshold, considers the
    /// color to be fully opaque (alpha is set to 1.0).
    Mask {
        /// The alpha threshold below which pixels are discarded.
        cutoff: f32,
    },
    /// The base color alpha value defines the opacity of the color.
    /// Standard alpha-blending is used to blend the fragment's color
    /// with the color behind it.
    #[default]
    Blend,
    /// Similar to [`AlphaMode::Blend`](bevy::render::alpha::AlphaMode::Blend), however
    /// assumes RGB channel values are premultiplied.
    ///
    /// For otherwise constant RGB values, behaves more like `Blend` for alpha values
    /// closer to 1.0, and more like `Add` for alpha values closer to 0.0.
    ///
    /// Can be used to avoid "border" or "outline" artifacts that can occur when using
    /// plain alpha-blended textures.
    Premultiplied,
    /// Combines the color of the fragments with the colors behind them in an
    /// additive process, (i.e. like light) producing lighter results.
    ///
    /// Black produces no effect. Alpha values can be used to modulate the result.
    ///
    /// Useful for effects like holograms, ghosts, lasers and other energy beams.
    Add,
    /// Combines the color of the fragments with the colors behind them in a
    /// multiplicative process, (i.e. like pigments) producing darker results.
    ///
    /// White produces no effect. Alpha values can be used to modulate the result.
    ///
    /// Useful for effects like stained glass, window tint film and some colored liquids.
    Multiply,
    /// Spreads the fragment out over a hardware-dependent number of sample locations
    /// proportional to the alpha value. This requires multisample antialiasing; if MSAA
    /// isn't on, this is identical to `Mask` with a value of 0.5.
    ///
    /// Alpha to coverage provides improved performance and better visual fidelity over
    /// `Blend`, as Bevy doesn't have to sort objects when it's in use. It's especially
    /// useful for complex transparent objects like foliage.
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

/// A serializable PBR material for particles, copied from Bevy's [`StandardMaterial`](bevy::pbr::StandardMaterial).
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
#[reflect(Clone)]
pub struct StandardParticleMaterial {
    /// The color of the surface of the material before lighting.
    ///
    /// Doubles as diffuse albedo for non-metallic, specular for metallic and a mix
    /// for everything in between. If used together with a `base_color_texture`, this
    /// is factored into the final base color as `base_color * base_color_texture_value`.
    ///
    /// Defaults to white `[1.0, 1.0, 1.0, 1.0]`.
    #[serde(default = "default_base_color")]
    pub base_color: [f32; 4],

    /// The actual pre-lighting color is `base_color * this_texture`.
    ///
    /// You should set `base_color` to white (the default) if you want the texture
    /// to show as-is. Setting `base_color` to something else will tint the texture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<TextureRef>,

    /// Color the material "emits" to the camera.
    ///
    /// This is typically used for monitor screens or LED lights. Anything that can
    /// be visible even in darkness.
    ///
    /// The default emissive color is black `[0.0, 0.0, 0.0, 1.0]`, which doesn't
    /// add anything to the material color.
    #[serde(default, skip_serializing_if = "is_default_emissive")]
    pub emissive: [f32; 4],

    /// This color is multiplied by `emissive` to get the final emitted color.
    ///
    /// You should set `emissive` to white if you want to use the full range of
    /// color of the emissive texture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<TextureRef>,

    /// How to apply the alpha channel of the `base_color_texture`.
    ///
    /// See [`SerializableAlphaMode`] for details. Defaults to
    /// [`SerializableAlphaMode::Opaque`].
    #[serde(default = "default_alpha_mode")]
    pub alpha_mode: SerializableAlphaMode,

    /// Linear perceptual roughness, clamped to `[0.089, 1.0]` in the shader.
    ///
    /// Defaults to `0.5`. Low values result in a "glossy" material with specular
    /// highlights, while values close to `1.0` result in rough materials.
    ///
    /// If used together with a roughness/metallic texture, this is factored into
    /// the final base color as `roughness * roughness_texture_value`.
    #[serde(default = "default_perceptual_roughness")]
    pub perceptual_roughness: f32,

    /// How "metallic" the material appears, within `[0.0, 1.0]`.
    ///
    /// This should be set to `0.0` for dielectric materials or `1.0` for metallic
    /// materials. For a hybrid surface such as corroded metal, you may need to use
    /// in-between values.
    ///
    /// Defaults to `0.0`, for dielectric.
    #[serde(default, skip_serializing_if = "is_zero_f32")]
    pub metallic: f32,

    /// Specular intensity for non-metals on a linear scale of `[0.0, 1.0]`.
    ///
    /// Use the value as a way to control the intensity of the specular highlight
    /// of the material, i.e. how reflective the material is, rather than the
    /// physical property "reflectance."
    ///
    /// Defaults to `0.5` which is mapped to 4% reflectance in the shader.
    #[serde(default = "default_reflectance")]
    pub reflectance: f32,

    /// The blue channel contains metallic values, and the green channel contains
    /// the roughness values. Other channels are unused.
    ///
    /// Those values are multiplied by the scalar ones of the material, see
    /// [`metallic`](Self::metallic) and [`perceptual_roughness`](Self::perceptual_roughness)
    /// for details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<TextureRef>,

    /// A normal map texture for faking surface detail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normal_map_texture: Option<TextureRef>,

    /// Support two-sided lighting by automatically flipping the normals for
    /// "back" faces within the PBR lighting shader.
    ///
    /// Defaults to `false`.
    #[serde(default, skip_serializing_if = "is_false")]
    pub double_sided: bool,

    /// Whether to apply only the base color to this material.
    ///
    /// Normals, occlusion textures, roughness, metallic, reflectance, emissive,
    /// shadows, alpha mode and ambient light are ignored if this is set to `true`.
    #[serde(default, skip_serializing_if = "is_false")]
    pub unlit: bool,

    /// Whether to enable fog for this material.
    ///
    /// Defaults to `true`.
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
    /// Converts this serializable material into a Bevy [`StandardMaterial`],
    /// loading any referenced textures via the provided [`AssetServer`].
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

    /// Creates a [`StandardParticleMaterial`] from a Bevy [`StandardMaterial`].
    ///
    /// Texture references are not preserved. Only color and numeric properties are copied.
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

    /// Computes a hash key for material caching.
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

/// The material used for a draw pass, either a standard PBR material or custom shaders.
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub enum DrawPassMaterial {
    /// A standard PBR material for particles.
    Standard(StandardParticleMaterial),
    /// Custom vertex and/or fragment shaders.
    CustomShader {
        /// Optional path to a custom vertex shader.
        vertex_shader: Option<String>,
        /// Optional path to a custom fragment shader.
        fragment_shader: Option<String>,
    },
}

impl Default for DrawPassMaterial {
    fn default() -> Self {
        Self::Standard(StandardParticleMaterial::default())
    }
}

impl DrawPassMaterial {
    /// Computes a hash key for material caching.
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
