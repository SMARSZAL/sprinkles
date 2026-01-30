use bevy::prelude::*;

use crate::ui::tokens::{BORDER_COLOR, FONT_PATH, TEXT_DISPLAY_COLOR, TEXT_SIZE};
use crate::ui::widgets::button::{ButtonVariant, IconButtonProps, icon_button};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (setup_panel_section_buttons, handle_collapse_click),
    );
}

#[derive(Component)]
pub struct EditorPanelSection;

#[derive(Component)]
struct PanelSectionHeader;

#[derive(Component)]
struct PanelSectionButtonsContainer;

#[derive(Component)]
struct PanelSectionCollapseButton(Entity);

#[derive(Component, Default)]
struct Collapsed(bool);

#[derive(Component)]
struct PanelSectionState {
    on_add: Option<fn()>,
    collapsible: bool,
}

const ICON_ADD: &str = "icons/ri-add-line.png";
const ICON_COLLAPSE: &str = "icons/ri-arrow-down-s-line.png";

#[derive(Default)]
pub struct PanelSectionProps {
    pub title: String,
    pub on_add: Option<fn()>,
    pub collapsible: bool,
}

impl PanelSectionProps {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn on_add(mut self, callback: fn()) -> Self {
        self.on_add = Some(callback);
        self
    }

    pub fn collapsible(mut self) -> Self {
        self.collapsible = true;
        self
    }
}

pub fn panel_section(props: PanelSectionProps, asset_server: &AssetServer) -> impl Bundle {
    let PanelSectionProps {
        title,
        on_add,
        collapsible,
    } = props;
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    (
        EditorPanelSection,
        Collapsed::default(),
        Node {
            width: percent(100),
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            padding: UiRect::all(px(12)),
            border: UiRect::bottom(px(1)),
            ..default()
        },
        BorderColor::all(BORDER_COLOR),
        PanelSectionState {
            on_add,
            collapsible,
        },
        children![(
            PanelSectionHeader,
            Node {
                width: percent(100),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Text::new(title),
                    TextFont {
                        font: font.into(),
                        font_size: TEXT_SIZE,
                        weight: FontWeight::SEMIBOLD,
                        ..default()
                    },
                    TextColor(TEXT_DISPLAY_COLOR.into()),
                ),
                (
                    PanelSectionButtonsContainer,
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ),
            ],
        )],
    )
}

fn setup_panel_section_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    new_sections: Query<(Entity, &PanelSectionState, &Children), Added<EditorPanelSection>>,
    headers: Query<&Children, With<PanelSectionHeader>>,
    containers: Query<Entity, With<PanelSectionButtonsContainer>>,
) {
    for (section_entity, state, section_children) in &new_sections {
        let Some(&header_entity) = section_children.first() else {
            continue;
        };
        let Ok(header_children) = headers.get(header_entity) else {
            continue;
        };
        let Some(&container_entity) = header_children.get(1) else {
            continue;
        };
        if containers.get(container_entity).is_err() {
            continue;
        }

        if let Some(callback) = state.on_add {
            let add_entity = commands
                .spawn(icon_button(
                    IconButtonProps::new(ICON_ADD)
                        .variant(ButtonVariant::Ghost)
                        .on_click(callback),
                    &asset_server,
                ))
                .id();
            commands.entity(container_entity).add_child(add_entity);
        }

        if state.collapsible {
            let collapse_entity = commands
                .spawn((
                    PanelSectionCollapseButton(section_entity),
                    UiTransform {
                        rotation: Rot2::degrees(180.0),
                        ..default()
                    },
                    icon_button(
                        IconButtonProps::new(ICON_COLLAPSE).variant(ButtonVariant::Ghost),
                        &asset_server,
                    ),
                ))
                .id();
            commands.entity(container_entity).add_child(collapse_entity);
        }
    }
}

fn handle_collapse_click(
    mut interactions: Query<
        (Entity, &Interaction, &PanelSectionCollapseButton),
        Changed<Interaction>,
    >,
    mut sections: Query<(&mut Collapsed, &Children), With<EditorPanelSection>>,
    mut nodes: Query<&mut Node, Without<PanelSectionHeader>>,
    headers: Query<Entity, With<PanelSectionHeader>>,
    mut button_transforms: Query<&mut UiTransform>,
) {
    for (button_entity, interaction, collapse_button) in &mut interactions {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok((mut collapsed, section_children)) = sections.get_mut(collapse_button.0) else {
            continue;
        };

        collapsed.0 = !collapsed.0;

        for child in section_children.iter() {
            if headers.get(child).is_ok() {
                continue;
            }
            if let Ok(mut node) = nodes.get_mut(child) {
                node.display = if collapsed.0 {
                    Display::None
                } else {
                    Display::Flex
                };
            }
        }

        if let Ok(mut transform) = button_transforms.get_mut(button_entity) {
            transform.rotation = if collapsed.0 {
                Rot2::degrees(0.0)
            } else {
                Rot2::degrees(180.0)
            };
        }
    }
}

