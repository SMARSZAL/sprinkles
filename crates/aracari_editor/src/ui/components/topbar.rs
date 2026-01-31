use bevy::prelude::*;

use crate::state::DirtyState;
use crate::ui::components::playback_controls::playback_controls;
use crate::ui::components::seekbar::seekbar;
use crate::ui::tokens::{BACKGROUND_COLOR, BORDER_COLOR, FONT_PATH, TEXT_BODY_COLOR, TEXT_SIZE};
use crate::ui::widgets::button::{ButtonProps, ButtonVariant, button};
use crate::ui::widgets::separator::EditorSeparator;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_save_button_text);
}

#[derive(Component)]
pub struct EditorTopbar;

#[derive(Component)]
struct SaveButton;

pub fn topbar(asset_server: &AssetServer) -> impl Bundle {
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    (
        EditorTopbar,
        Node {
            width: percent(100),
            height: px(52),
            padding: UiRect::all(px(12)),
            border: UiRect::bottom(px(1)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR.into()),
        BorderColor::all(BORDER_COLOR),
        children![
            (
                Text::new("TODO: current project"),
                TextFont {
                    font: font.clone().into(),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(TEXT_BODY_COLOR.into()),
            ),
            (
                Node {
                    column_gap: px(12),
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    seekbar(asset_server),
                    playback_controls(asset_server),
                    EditorSeparator::vertical(),
                    (
                        SaveButton,
                        button(
                            ButtonProps::new("Save").with_variant(ButtonVariant::Primary),
                            asset_server,
                        ),
                    ),
                ],
            ),
        ],
    )
}

fn update_save_button_text(
    dirty_state: Res<DirtyState>,
    save_buttons: Query<&Children, With<SaveButton>>,
    mut texts: Query<&mut Text>,
) {
    if !dirty_state.is_changed() {
        return;
    }

    let label = if dirty_state.has_unsaved_changes {
        "Save*"
    } else {
        "Save"
    };

    for children in &save_buttons {
        for child in children.iter() {
            if let Ok(mut text) = texts.get_mut(child) {
                **text = label.to_string();
            }
        }
    }
}
