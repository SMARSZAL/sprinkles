use bevy::prelude::*;

use crate::ui::tokens::{FONT_PATH, TEXT_BODY_COLOR, TEXT_SIZE};
use crate::ui::widgets::panel::{PanelDirection, PanelProps, panel, panel_resize_handle};
use crate::ui::widgets::panel_section::{PanelSectionProps, panel_section};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            setup_inspector_panel_resize,
            setup_inspector_panel_content,
        ),
    );
}

#[derive(Component)]
pub struct EditorInspectorPanel;

#[derive(Component)]
struct CollapsibleSection;

#[derive(Component)]
struct AddCollapsibleSection;

pub fn inspector_panel(asset_server: &AssetServer) -> impl Bundle {
    (
        EditorInspectorPanel,
        panel(
            PanelProps::new(PanelDirection::Right)
                .with_width(320)
                .with_min_width(320)
                .with_max_width(512),
        ),
        children![
            (
                CollapsibleSection,
                panel_section(
                    PanelSectionProps::new("Collapsible").collapsible(),
                    asset_server,
                ),
            ),
            (
                AddCollapsibleSection,
                panel_section(
                    PanelSectionProps::new("Add & Collapsible")
                        .on_add(|| println!("Add & Collapsible"))
                        .collapsible(),
                    asset_server,
                ),
            ),
        ],
    )
}

fn setup_inspector_panel_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    collapsible: Query<Entity, Added<CollapsibleSection>>,
    add_collapsible: Query<Entity, Added<AddCollapsibleSection>>,
) {
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    for entity in &collapsible {
        commands
            .entity(entity)
            .with_child(test_label("Content 1", font.clone()));
    }

    for entity in &add_collapsible {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(test_label("Content A", font.clone()));
            parent.spawn(test_label("Content B", font.clone()));
        });
    }
}

fn test_label(content: &str, font: Handle<Font>) -> impl Bundle {
    (
        Text::new(content),
        TextFont {
            font,
            font_size: TEXT_SIZE,
            ..default()
        },
        TextColor(TEXT_BODY_COLOR.into()),
    )
}

fn setup_inspector_panel_resize(
    mut commands: Commands,
    panels: Query<Entity, Added<EditorInspectorPanel>>,
) {
    for panel_entity in &panels {
        commands
            .entity(panel_entity)
            .with_child(panel_resize_handle(panel_entity, PanelDirection::Right));
    }
}
