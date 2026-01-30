use bevy::input::mouse::MouseMotion;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;
use bevy::window::{CursorIcon, SystemCursorIcon};

use crate::ui::tokens::{BACKGROUND_COLOR, BORDER_COLOR};

const RESIZE_HANDLE_WIDTH: u32 = 16;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (handle_resize_hover, handle_resize_drag));
}

#[derive(Component)]
pub struct EditorPanel;

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum PanelDirection {
    #[default]
    Left,
    Right,
}

#[derive(Component)]
pub struct PanelWidth {
    pub current: u32,
    pub min: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct PanelResizeHandle {
    pub panel: Entity,
    pub direction: PanelDirection,
}

#[derive(Component, Default)]
pub struct ResizeDragState {
    pub dragging: bool,
    pub accumulated_delta: f32,
}

pub struct PanelProps {
    pub direction: PanelDirection,
    pub width: u32,
    pub min_width: u32,
    pub max_width: u32,
}

impl Default for PanelProps {
    fn default() -> Self {
        Self {
            direction: PanelDirection::default(),
            width: 250,
            min_width: 100,
            max_width: 500,
        }
    }
}

impl PanelProps {
    pub fn new(direction: PanelDirection) -> Self {
        Self {
            direction,
            ..default()
        }
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_min_width(mut self, min_width: u32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn with_max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }
}

pub fn panel(props: PanelProps) -> impl Bundle {
    let PanelProps {
        direction,
        width,
        min_width,
        max_width,
    } = props;

    let border = match direction {
        PanelDirection::Left => UiRect::right(px(1)),
        PanelDirection::Right => UiRect::left(px(1)),
    };
    let margin = match direction {
        PanelDirection::Left => UiRect::ZERO,
        PanelDirection::Right => UiRect::left(Val::Auto),
    };

    (
        EditorPanel,
        direction,
        PanelWidth {
            current: width,
            min: min_width,
            max: max_width,
        },
        Node {
            width: px(width),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            border,
            margin,
            position_type: PositionType::Relative,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR.into()),
        BorderColor::all(BORDER_COLOR),
    )
}

pub fn panel_resize_handle(panel: Entity, direction: PanelDirection) -> impl Bundle {
    let offset = px(-((RESIZE_HANDLE_WIDTH / 2) as f32));

    let (left, right) = match direction {
        PanelDirection::Left => (Val::Auto, offset),
        PanelDirection::Right => (offset, Val::Auto),
    };

    (
        PanelResizeHandle { panel, direction },
        ResizeDragState::default(),
        Hovered::default(),
        Node {
            position_type: PositionType::Absolute,
            width: px(RESIZE_HANDLE_WIDTH),
            height: percent(100),
            top: px(0),
            left,
            right,
            ..default()
        },
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
    )
}

fn handle_resize_hover(
    handles: Query<&Hovered, (Changed<Hovered>, With<PanelResizeHandle>)>,
    window: Single<Entity, With<Window>>,
    mut commands: Commands,
) {
    for hovered in &handles {
        if hovered.get() {
            commands
                .entity(*window)
                .insert(CursorIcon::from(SystemCursorIcon::ColResize));
        } else {
            commands.entity(*window).remove::<CursorIcon>();
        }
    }
}

fn handle_resize_drag(
    mut handles: Query<(&PanelResizeHandle, &mut ResizeDragState, &Hovered)>,
    mut panels: Query<(&mut Node, &mut PanelWidth), With<EditorPanel>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
) {
    let cursor_delta: f32 = mouse_motion.read().map(|e| e.delta.x).sum();

    for (handle, mut drag_state, hovered) in &mut handles {
        if mouse.just_pressed(MouseButton::Left) && hovered.get() {
            drag_state.dragging = true;
            drag_state.accumulated_delta = 0.0;
        }

        if mouse.just_released(MouseButton::Left) {
            drag_state.dragging = false;
        }

        if drag_state.dragging && cursor_delta != 0.0 {
            if let Ok((mut node, mut panel_width)) = panels.get_mut(handle.panel) {
                let delta = match handle.direction {
                    PanelDirection::Left => cursor_delta,
                    PanelDirection::Right => -cursor_delta,
                };

                drag_state.accumulated_delta += delta;
                let new_width = ((panel_width.current as f32) + drag_state.accumulated_delta)
                    .clamp(panel_width.min as f32, panel_width.max as f32)
                    as u32;

                if new_width != panel_width.current {
                    drag_state.accumulated_delta = 0.0;
                    panel_width.current = new_width;
                    node.width = px(new_width);
                }
            }
        }
    }
}
