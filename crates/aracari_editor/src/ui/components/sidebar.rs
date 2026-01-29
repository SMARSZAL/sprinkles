use bevy::prelude::*;

use crate::ui::tokens::{BACKGROUND_COLOR, BORDER_COLOR, FONT_PATH, TEXT_BODY_COLOR, TEXT_SIZE};

#[derive(Component)]
pub struct EditorSidebar;

pub fn sidebar(asset_server: &AssetServer) -> impl Bundle {
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    (
        EditorSidebar,
        Node {
            width: px(72),
            height: percent(100),
            padding: UiRect::all(px(12)),
            border: UiRect::right(px(1)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(12),
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR.into()),
        BorderColor::all(BORDER_COLOR),
        children![
            (
                Text::new("TODO:"),
                TextFont {
                    font: font.clone().into(),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(TEXT_BODY_COLOR.into()),
            ),
            (
                Text::new("Project"),
                TextFont {
                    font: font.clone().into(),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(TEXT_BODY_COLOR.into()),
            ),
            (
                Text::new("Outline"),
                TextFont {
                    font: font.clone().into(),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(TEXT_BODY_COLOR.into()),
            ),
            (
                Text::new("Defs"),
                TextFont {
                    font: font.clone().into(),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(TEXT_BODY_COLOR.into()),
            ),
            (
                Text::new("Settings"),
                TextFont {
                    font: font.into(),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(TEXT_BODY_COLOR.into()),
            ),
        ],
    )
}
