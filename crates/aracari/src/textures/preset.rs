use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[cfg(feature = "preset-textures")]
use bevy::asset::embedded_asset;

#[cfg(feature = "preset-textures")]
#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Hash, PartialEq, Eq)]
pub enum PresetTexture {
    Circle1,
    Circle2,
    Circle3,
    Circle4,
    Circle5,
    Dirt1,
    Dirt2,
    Dirt3,
    Fire1,
    Fire2,
    Flame1,
    Flame2,
    Flame3,
    Flame4,
    Flame5,
    Flame6,
    Flare1,
    Light1,
    Light2,
    Light3,
    Magic1,
    Magic2,
    Magic3,
    Magic4,
    Magic5,
    Muzzle1,
    Muzzle2,
    Muzzle3,
    Muzzle4,
    Muzzle5,
    Scorch1,
    Scorch2,
    Scorch3,
    Scratch1,
    Slash1,
    Slash2,
    Slash3,
    Slash4,
    Smoke1,
    Smoke2,
    Smoke3,
    Smoke4,
    Smoke5,
    Smoke6,
    Smoke7,
    Smoke8,
    Smoke9,
    Smoke10,
    Spark1,
    Spark2,
    Spark3,
    Spark4,
    Spark5,
    Spark6,
    Spark7,
    Star1,
    Star2,
    Star3,
    Star4,
    Star5,
    Star6,
    Star7,
    Star8,
    Star9,
    Symbol1,
    Symbol2,
    Trace1,
    Trace2,
    Trace3,
    Trace4,
    Trace5,
    Trace6,
    Trace7,
    Twirl1,
    Twirl2,
    Twirl3,
    Window1,
    Window2,
    Window3,
    Window4,
}

#[cfg(feature = "preset-textures")]
impl PresetTexture {
    pub fn embedded_path(&self) -> &'static str {
        match self {
            Self::Circle1 => "embedded://aracari/textures/assets/circle_01.png",
            Self::Circle2 => "embedded://aracari/textures/assets/circle_02.png",
            Self::Circle3 => "embedded://aracari/textures/assets/circle_03.png",
            Self::Circle4 => "embedded://aracari/textures/assets/circle_04.png",
            Self::Circle5 => "embedded://aracari/textures/assets/circle_05.png",
            Self::Dirt1 => "embedded://aracari/textures/assets/dirt_01.png",
            Self::Dirt2 => "embedded://aracari/textures/assets/dirt_02.png",
            Self::Dirt3 => "embedded://aracari/textures/assets/dirt_03.png",
            Self::Fire1 => "embedded://aracari/textures/assets/fire_01.png",
            Self::Fire2 => "embedded://aracari/textures/assets/fire_02.png",
            Self::Flame1 => "embedded://aracari/textures/assets/flame_01.png",
            Self::Flame2 => "embedded://aracari/textures/assets/flame_02.png",
            Self::Flame3 => "embedded://aracari/textures/assets/flame_03.png",
            Self::Flame4 => "embedded://aracari/textures/assets/flame_04.png",
            Self::Flame5 => "embedded://aracari/textures/assets/flame_05.png",
            Self::Flame6 => "embedded://aracari/textures/assets/flame_06.png",
            Self::Flare1 => "embedded://aracari/textures/assets/flare_01.png",
            Self::Light1 => "embedded://aracari/textures/assets/light_01.png",
            Self::Light2 => "embedded://aracari/textures/assets/light_02.png",
            Self::Light3 => "embedded://aracari/textures/assets/light_03.png",
            Self::Magic1 => "embedded://aracari/textures/assets/magic_01.png",
            Self::Magic2 => "embedded://aracari/textures/assets/magic_02.png",
            Self::Magic3 => "embedded://aracari/textures/assets/magic_03.png",
            Self::Magic4 => "embedded://aracari/textures/assets/magic_04.png",
            Self::Magic5 => "embedded://aracari/textures/assets/magic_05.png",
            Self::Muzzle1 => "embedded://aracari/textures/assets/muzzle_01.png",
            Self::Muzzle2 => "embedded://aracari/textures/assets/muzzle_02.png",
            Self::Muzzle3 => "embedded://aracari/textures/assets/muzzle_03.png",
            Self::Muzzle4 => "embedded://aracari/textures/assets/muzzle_04.png",
            Self::Muzzle5 => "embedded://aracari/textures/assets/muzzle_05.png",
            Self::Scorch1 => "embedded://aracari/textures/assets/scorch_01.png",
            Self::Scorch2 => "embedded://aracari/textures/assets/scorch_02.png",
            Self::Scorch3 => "embedded://aracari/textures/assets/scorch_03.png",
            Self::Scratch1 => "embedded://aracari/textures/assets/scratch_01.png",
            Self::Slash1 => "embedded://aracari/textures/assets/slash_01.png",
            Self::Slash2 => "embedded://aracari/textures/assets/slash_02.png",
            Self::Slash3 => "embedded://aracari/textures/assets/slash_03.png",
            Self::Slash4 => "embedded://aracari/textures/assets/slash_04.png",
            Self::Smoke1 => "embedded://aracari/textures/assets/smoke_01.png",
            Self::Smoke2 => "embedded://aracari/textures/assets/smoke_02.png",
            Self::Smoke3 => "embedded://aracari/textures/assets/smoke_03.png",
            Self::Smoke4 => "embedded://aracari/textures/assets/smoke_04.png",
            Self::Smoke5 => "embedded://aracari/textures/assets/smoke_05.png",
            Self::Smoke6 => "embedded://aracari/textures/assets/smoke_06.png",
            Self::Smoke7 => "embedded://aracari/textures/assets/smoke_07.png",
            Self::Smoke8 => "embedded://aracari/textures/assets/smoke_08.png",
            Self::Smoke9 => "embedded://aracari/textures/assets/smoke_09.png",
            Self::Smoke10 => "embedded://aracari/textures/assets/smoke_10.png",
            Self::Spark1 => "embedded://aracari/textures/assets/spark_01.png",
            Self::Spark2 => "embedded://aracari/textures/assets/spark_02.png",
            Self::Spark3 => "embedded://aracari/textures/assets/spark_03.png",
            Self::Spark4 => "embedded://aracari/textures/assets/spark_04.png",
            Self::Spark5 => "embedded://aracari/textures/assets/spark_05.png",
            Self::Spark6 => "embedded://aracari/textures/assets/spark_06.png",
            Self::Spark7 => "embedded://aracari/textures/assets/spark_07.png",
            Self::Star1 => "embedded://aracari/textures/assets/star_01.png",
            Self::Star2 => "embedded://aracari/textures/assets/star_02.png",
            Self::Star3 => "embedded://aracari/textures/assets/star_03.png",
            Self::Star4 => "embedded://aracari/textures/assets/star_04.png",
            Self::Star5 => "embedded://aracari/textures/assets/star_05.png",
            Self::Star6 => "embedded://aracari/textures/assets/star_06.png",
            Self::Star7 => "embedded://aracari/textures/assets/star_07.png",
            Self::Star8 => "embedded://aracari/textures/assets/star_08.png",
            Self::Star9 => "embedded://aracari/textures/assets/star_09.png",
            Self::Symbol1 => "embedded://aracari/textures/assets/symbol_01.png",
            Self::Symbol2 => "embedded://aracari/textures/assets/symbol_02.png",
            Self::Trace1 => "embedded://aracari/textures/assets/trace_01.png",
            Self::Trace2 => "embedded://aracari/textures/assets/trace_02.png",
            Self::Trace3 => "embedded://aracari/textures/assets/trace_03.png",
            Self::Trace4 => "embedded://aracari/textures/assets/trace_04.png",
            Self::Trace5 => "embedded://aracari/textures/assets/trace_05.png",
            Self::Trace6 => "embedded://aracari/textures/assets/trace_06.png",
            Self::Trace7 => "embedded://aracari/textures/assets/trace_07.png",
            Self::Twirl1 => "embedded://aracari/textures/assets/twirl_01.png",
            Self::Twirl2 => "embedded://aracari/textures/assets/twirl_02.png",
            Self::Twirl3 => "embedded://aracari/textures/assets/twirl_03.png",
            Self::Window1 => "embedded://aracari/textures/assets/window_01.png",
            Self::Window2 => "embedded://aracari/textures/assets/window_02.png",
            Self::Window3 => "embedded://aracari/textures/assets/window_03.png",
            Self::Window4 => "embedded://aracari/textures/assets/window_04.png",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Reflect, Hash, PartialEq, Eq)]
pub enum TextureRef {
    #[cfg(feature = "preset-textures")]
    Preset(PresetTexture),
    Local(String),
    Asset(String),
}

impl TextureRef {
    pub fn load(&self, asset_server: &AssetServer) -> Handle<Image> {
        match self {
            #[cfg(feature = "preset-textures")]
            Self::Preset(preset) => asset_server.load(preset.embedded_path()),
            Self::Local(path) | Self::Asset(path) => asset_server.load(path),
        }
    }
}

#[cfg(feature = "preset-textures")]
pub fn register_preset_textures(app: &mut App) {
    embedded_asset!(app, "assets/circle_01.png");
    embedded_asset!(app, "assets/circle_02.png");
    embedded_asset!(app, "assets/circle_03.png");
    embedded_asset!(app, "assets/circle_04.png");
    embedded_asset!(app, "assets/circle_05.png");
    embedded_asset!(app, "assets/dirt_01.png");
    embedded_asset!(app, "assets/dirt_02.png");
    embedded_asset!(app, "assets/dirt_03.png");
    embedded_asset!(app, "assets/fire_01.png");
    embedded_asset!(app, "assets/fire_02.png");
    embedded_asset!(app, "assets/flame_01.png");
    embedded_asset!(app, "assets/flame_02.png");
    embedded_asset!(app, "assets/flame_03.png");
    embedded_asset!(app, "assets/flame_04.png");
    embedded_asset!(app, "assets/flame_05.png");
    embedded_asset!(app, "assets/flame_06.png");
    embedded_asset!(app, "assets/flare_01.png");
    embedded_asset!(app, "assets/light_01.png");
    embedded_asset!(app, "assets/light_02.png");
    embedded_asset!(app, "assets/light_03.png");
    embedded_asset!(app, "assets/magic_01.png");
    embedded_asset!(app, "assets/magic_02.png");
    embedded_asset!(app, "assets/magic_03.png");
    embedded_asset!(app, "assets/magic_04.png");
    embedded_asset!(app, "assets/magic_05.png");
    embedded_asset!(app, "assets/muzzle_01.png");
    embedded_asset!(app, "assets/muzzle_02.png");
    embedded_asset!(app, "assets/muzzle_03.png");
    embedded_asset!(app, "assets/muzzle_04.png");
    embedded_asset!(app, "assets/muzzle_05.png");
    embedded_asset!(app, "assets/scorch_01.png");
    embedded_asset!(app, "assets/scorch_02.png");
    embedded_asset!(app, "assets/scorch_03.png");
    embedded_asset!(app, "assets/scratch_01.png");
    embedded_asset!(app, "assets/slash_01.png");
    embedded_asset!(app, "assets/slash_02.png");
    embedded_asset!(app, "assets/slash_03.png");
    embedded_asset!(app, "assets/slash_04.png");
    embedded_asset!(app, "assets/smoke_01.png");
    embedded_asset!(app, "assets/smoke_02.png");
    embedded_asset!(app, "assets/smoke_03.png");
    embedded_asset!(app, "assets/smoke_04.png");
    embedded_asset!(app, "assets/smoke_05.png");
    embedded_asset!(app, "assets/smoke_06.png");
    embedded_asset!(app, "assets/smoke_07.png");
    embedded_asset!(app, "assets/smoke_08.png");
    embedded_asset!(app, "assets/smoke_09.png");
    embedded_asset!(app, "assets/smoke_10.png");
    embedded_asset!(app, "assets/spark_01.png");
    embedded_asset!(app, "assets/spark_02.png");
    embedded_asset!(app, "assets/spark_03.png");
    embedded_asset!(app, "assets/spark_04.png");
    embedded_asset!(app, "assets/spark_05.png");
    embedded_asset!(app, "assets/spark_06.png");
    embedded_asset!(app, "assets/spark_07.png");
    embedded_asset!(app, "assets/star_01.png");
    embedded_asset!(app, "assets/star_02.png");
    embedded_asset!(app, "assets/star_03.png");
    embedded_asset!(app, "assets/star_04.png");
    embedded_asset!(app, "assets/star_05.png");
    embedded_asset!(app, "assets/star_06.png");
    embedded_asset!(app, "assets/star_07.png");
    embedded_asset!(app, "assets/star_08.png");
    embedded_asset!(app, "assets/star_09.png");
    embedded_asset!(app, "assets/symbol_01.png");
    embedded_asset!(app, "assets/symbol_02.png");
    embedded_asset!(app, "assets/trace_01.png");
    embedded_asset!(app, "assets/trace_02.png");
    embedded_asset!(app, "assets/trace_03.png");
    embedded_asset!(app, "assets/trace_04.png");
    embedded_asset!(app, "assets/trace_05.png");
    embedded_asset!(app, "assets/trace_06.png");
    embedded_asset!(app, "assets/trace_07.png");
    embedded_asset!(app, "assets/twirl_01.png");
    embedded_asset!(app, "assets/twirl_02.png");
    embedded_asset!(app, "assets/twirl_03.png");
    embedded_asset!(app, "assets/window_01.png");
    embedded_asset!(app, "assets/window_02.png");
    embedded_asset!(app, "assets/window_03.png");
    embedded_asset!(app, "assets/window_04.png");
}
