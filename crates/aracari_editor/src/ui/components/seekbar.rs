use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::text::{FontFeatureTag, FontFeatures};

use crate::state::EditorState;
use crate::ui::tokens::{FONT_PATH, TEXT_MUTED_COLOR};

const SEEKBAR_HEIGHT: f32 = 4.0;
const SEEKBAR_WIDTH: f32 = 192.0;
const LABEL_SIZE: f32 = 12.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (update_seekbar, setup_seekbar_observers))
        .add_observer(on_seekbar_drag)
        .add_observer(on_seekbar_release);
}

#[derive(Component)]
pub struct EditorSeekbar;

#[derive(Component)]
pub struct SeekbarElapsed;

#[derive(Component)]
pub struct SeekbarDuration;

#[derive(Component)]
pub struct SeekbarHitbox;

#[derive(Component)]
pub struct SeekbarTrack;

#[derive(Component)]
pub struct SeekbarFill;

#[derive(Component, Default)]
pub struct SeekbarDragState {
    pub dragging: bool,
}

#[derive(EntityEvent)]
pub struct SeekbarDragEvent {
    pub entity: Entity,
    pub value: f32,
}

#[derive(EntityEvent)]
pub struct SeekbarReleaseEvent {
    pub entity: Entity,
}

pub fn seekbar(asset_server: &AssetServer) -> impl Bundle {
    let font: Handle<Font> = asset_server.load(FONT_PATH);
    let tabular_figures: FontFeatures = [FontFeatureTag::TABULAR_FIGURES].into();

    (
        EditorSeekbar,
        Node {
            align_items: AlignItems::Center,
            column_gap: px(6),
            ..default()
        },
        children![
            (
                SeekbarElapsed,
                Text::new("0.00"),
                TextFont {
                    font: font.clone().into(),
                    font_size: LABEL_SIZE,
                    font_features: tabular_figures.clone(),
                    weight: FontWeight::MEDIUM,
                    ..default()
                },
                TextColor(TEXT_MUTED_COLOR.into()),
            ),
            (
                Node {
                    width: px(SEEKBAR_WIDTH),
                    height: px(SEEKBAR_HEIGHT),
                    ..default()
                },
                children![
                    (
                        SeekbarTrack,
                        Node {
                            width: percent(100),
                            height: percent(100),
                            border_radius: BorderRadius::all(Val::Percent(100.0)),
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        BackgroundColor(tailwind::ZINC_700.into()),
                        children![(
                            SeekbarFill,
                            Node {
                                width: percent(0),
                                height: percent(100),
                                border_radius: BorderRadius::all(Val::Percent(100.0)),
                                ..default()
                            },
                            BackgroundColor(tailwind::ZINC_200.into()),
                        )],
                    ),
                    (
                        SeekbarHitbox,
                        SeekbarDragState::default(),
                        Node {
                            position_type: PositionType::Absolute,
                            width: px(SEEKBAR_WIDTH),
                            height: px(SEEKBAR_HEIGHT * 3.),
                            top: px(-SEEKBAR_HEIGHT),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ),
                ],
            ),
            (
                SeekbarDuration,
                Text::new("0.00s"),
                TextFont {
                    font: font.into(),
                    font_size: LABEL_SIZE,
                    font_features: tabular_figures,
                    weight: FontWeight::MEDIUM,
                    ..default()
                },
                TextColor(TEXT_MUTED_COLOR.into()),
            ),
        ],
    )
}

fn format_elapsed(ms: f32) -> String {
    let seconds = ms / 1000.0;
    format!("{:.2}", seconds)
}

fn format_duration(ms: f32) -> String {
    let seconds = ms / 1000.0;
    format!("{:.2}s", seconds)
}

fn setup_seekbar_observers(hitboxes: Query<Entity, Added<SeekbarHitbox>>, mut commands: Commands) {
    for entity in &hitboxes {
        commands
            .entity(entity)
            .observe(on_drag_start)
            .observe(on_drag)
            .observe(on_drag_end);
    }
}

fn update_seekbar(
    state: Res<EditorState>,
    mut elapsed_label: Query<&mut Text, (With<SeekbarElapsed>, Without<SeekbarDuration>)>,
    mut duration_label: Query<&mut Text, (With<SeekbarDuration>, Without<SeekbarElapsed>)>,
    mut fill: Query<&mut Node, With<SeekbarFill>>,
    drag_state: Query<&SeekbarDragState, With<SeekbarHitbox>>,
) {
    let Ok(drag) = drag_state.single() else {
        return;
    };

    for mut text in &mut elapsed_label {
        **text = format_elapsed(state.elapsed_ms);
    }

    for mut text in &mut duration_label {
        **text = format_duration(state.duration_ms);
    }

    // skip fill update while dragging since it's handled by the drag event
    if drag.dragging {
        return;
    }

    let progress = if state.duration_ms > 0.0 {
        (state.elapsed_ms / state.duration_ms).clamp(0.0, 1.0)
    } else {
        0.0
    };

    for mut node in &mut fill {
        node.width = Val::Percent(progress * 100.0);
    }
}

fn on_drag_start(
    event: On<Pointer<DragStart>>,
    mut hitboxes: Query<&mut SeekbarDragState, With<SeekbarHitbox>>,
) {
    let Ok(mut drag_state) = hitboxes.get_mut(event.entity) else {
        return;
    };
    drag_state.dragging = true;
}

fn on_drag(
    event: On<Pointer<Drag>>,
    hitboxes: Query<(&SeekbarDragState, &ComputedNode, &UiGlobalTransform), With<SeekbarHitbox>>,
    mut fill: Query<&mut Node, With<SeekbarFill>>,
    mut commands: Commands,
) {
    let entity = event.entity;
    let Ok((drag_state, computed, transform)) = hitboxes.get(entity) else {
        return;
    };

    if !drag_state.dragging {
        return;
    }

    let pointer_x = event.pointer_location.position.x;
    let scale = computed.inverse_scale_factor;
    let center_x = transform.translation.x * scale;
    let width = computed.size.x * scale;
    let left_x = center_x - width * 0.5;
    let value = ((pointer_x - left_x) / width).clamp(0.0, 1.0);

    for mut node in &mut fill {
        node.width = Val::Percent(value * 100.0);
    }

    commands.trigger(SeekbarDragEvent { entity, value });
}

fn on_drag_end(
    event: On<Pointer<DragEnd>>,
    mut hitboxes: Query<&mut SeekbarDragState, With<SeekbarHitbox>>,
    mut commands: Commands,
) {
    let entity = event.entity;
    let Ok(mut drag_state) = hitboxes.get_mut(entity) else {
        return;
    };

    drag_state.dragging = false;
    commands.trigger(SeekbarReleaseEvent { entity });
}

fn on_seekbar_drag(event: On<SeekbarDragEvent>, mut editor_state: ResMut<EditorState>) {
    let seek_time_ms = event.value * editor_state.duration_ms;
    editor_state.is_seeking = true;
    editor_state.seek_to_ms = Some(seek_time_ms);
    editor_state.elapsed_ms = seek_time_ms;
}

fn on_seekbar_release(_event: On<SeekbarReleaseEvent>, mut editor_state: ResMut<EditorState>) {
    editor_state.is_seeking = false;
}
