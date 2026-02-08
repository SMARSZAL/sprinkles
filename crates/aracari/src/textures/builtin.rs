use bevy::{asset::embedded_asset, prelude::*};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Hash, PartialEq, Eq)]
pub enum BuiltinTexture {
    Circle,
}

impl BuiltinTexture {
    pub fn embedded_path(&self) -> &'static str {
        match self {
            Self::Circle => "embedded://aracari/textures/circle_01_a.png",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Hash, PartialEq, Eq)]
pub enum TextureRef {
    Builtin(BuiltinTexture),
    Local(String),
    Asset(String),
}

impl TextureRef {
    pub fn load(&self, asset_server: &AssetServer) -> Handle<Image> {
        match self {
            Self::Builtin(builtin) => asset_server.load(builtin.embedded_path()),
            Self::Local(path) | Self::Asset(path) => asset_server.load(path),
        }
    }
}

pub fn register_builtin_textures(app: &mut App) {
    embedded_asset!(app, "circle_01_a.png");
}
