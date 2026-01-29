use bevy::color::palettes::tailwind;
use bevy::prelude::*;

use crate::state::EditorState;
use crate::ui::tokens::{PRIMARY_COLOR, TEXT_BODY_COLOR};
use crate::ui::widgets::button::{
    ButtonSize, ButtonVariant, IconButtonProps, icon_button, set_button_variant,
};

const PLAY_ICON: &str = "icons/ri-play-fill.png";
const PAUSE_ICON: &str = "icons/ri-pause-fill.png";
const STOP_ICON: &str = "icons/ri-stop-fill.png";
const LOOP_ICON: &str = "icons/ri-repeat-fill.png";

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_play_pause_click,
            handle_stop_click,
            handle_loop_click,
            update_play_pause_icon,
            update_loop_button_style,
        ),
    );
}

#[derive(Component)]
pub struct EditorPlaybackControls;

#[derive(Component)]
pub struct PlayPauseButton;

#[derive(Component)]
pub struct StopButton;

#[derive(Component)]
pub struct LoopButton;

pub fn playback_controls(asset_server: &AssetServer) -> impl Bundle {
    (
        EditorPlaybackControls,
        Node {
            align_items: AlignItems::Center,
            column_gap: px(6),
            ..default()
        },
        children![
            play_pause_button(asset_server),
            stop_button(asset_server),
            loop_button(asset_server),
        ],
    )
}

fn play_pause_button(asset_server: &AssetServer) -> impl Bundle {
    (
        PlayPauseButton,
        icon_button(
            IconButtonProps::new(PAUSE_ICON)
                .color(tailwind::GREEN_500)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon),
            asset_server,
        ),
    )
}

fn stop_button(asset_server: &AssetServer) -> impl Bundle {
    (
        StopButton,
        icon_button(
            IconButtonProps::new(STOP_ICON)
                .color(TEXT_BODY_COLOR)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon),
            asset_server,
        ),
    )
}

fn loop_button(asset_server: &AssetServer) -> impl Bundle {
    (
        LoopButton,
        icon_button(
            IconButtonProps::new(LOOP_ICON)
                .color(PRIMARY_COLOR)
                .variant(ButtonVariant::Active)
                .size(ButtonSize::Icon),
            asset_server,
        ),
    )
}

fn handle_play_pause_click(
    mut editor_state: ResMut<EditorState>,
    query: Query<&Interaction, (Changed<Interaction>, With<PlayPauseButton>)>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            editor_state.is_playing = !editor_state.is_playing;
            if editor_state.is_playing {
                editor_state.play_requested = true;
            }
        }
    }
}

fn handle_stop_click(
    mut editor_state: ResMut<EditorState>,
    query: Query<&Interaction, (Changed<Interaction>, With<StopButton>)>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            editor_state.should_reset = true;
            editor_state.is_playing = false;
        }
    }
}

fn handle_loop_click(
    mut editor_state: ResMut<EditorState>,
    query: Query<&Interaction, (Changed<Interaction>, With<LoopButton>)>,
) {
    for interaction in &query {
        if *interaction == Interaction::Pressed {
            editor_state.is_looping = !editor_state.is_looping;
        }
    }
}

fn update_play_pause_icon(
    editor_state: Res<EditorState>,
    asset_server: Res<AssetServer>,
    button_query: Query<&Children, With<PlayPauseButton>>,
    mut image_query: Query<&mut ImageNode>,
) {
    if !editor_state.is_changed() {
        return;
    }

    let icon_path = if editor_state.is_playing {
        PAUSE_ICON
    } else {
        PLAY_ICON
    };

    for children in &button_query {
        for child in children.iter() {
            if let Ok(mut image) = image_query.get_mut(child) {
                image.image = asset_server.load(icon_path);
            }
        }
    }
}

fn update_loop_button_style(
    editor_state: Res<EditorState>,
    mut button_query: Query<
        (
            &Children,
            &mut ButtonVariant,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<LoopButton>,
    >,
    mut image_query: Query<&mut ImageNode>,
) {
    if !editor_state.is_changed() {
        return;
    }

    let variant = if editor_state.is_looping {
        ButtonVariant::Active
    } else {
        ButtonVariant::Ghost
    };

    for (children, mut current_variant, mut bg, mut border) in &mut button_query {
        if *current_variant != variant {
            *current_variant = variant;
            set_button_variant(variant, &mut bg, &mut border);
        }

        for child in children.iter() {
            if let Ok(mut image) = image_query.get_mut(child) {
                image.color = variant.text_color().into();
            }
        }
    }
}
