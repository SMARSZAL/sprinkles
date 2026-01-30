use bevy::prelude::*;

use crate::ui::tokens::BORDER_COLOR;

#[derive(Component)]
pub struct EditorSeparator;

impl EditorSeparator {
    pub fn horizontal() -> impl Bundle {
        (
            EditorSeparator,
            Node {
                width: px(24),
                height: px(1),
                ..default()
            },
            BackgroundColor(BORDER_COLOR.into()),
        )
    }

    pub fn vertical() -> impl Bundle {
        (
            EditorSeparator,
            Node {
                width: px(1),
                height: px(24),
                ..default()
            },
            BackgroundColor(BORDER_COLOR.into()),
        )
    }
}
