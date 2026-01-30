use bevy::prelude::*;

use crate::ui::widgets::button::{ButtonProps, ButtonVariant, button};
use crate::ui::widgets::panel::{PanelDirection, PanelProps, panel, panel_resize_handle};
use crate::ui::widgets::panel_section::{PanelSectionProps, panel_section};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (setup_data_panel_resize, setup_emitters_section));
}

#[derive(Component)]
pub struct EditorDataPanel;

#[derive(Component)]
struct EmittersSection;

pub fn data_panel(asset_server: &AssetServer) -> impl Bundle {
    (
        EditorDataPanel,
        panel(
            PanelProps::new(PanelDirection::Left)
                .with_width(224)
                .with_min_width(160)
                .with_max_width(320),
        ),
        children![
            (
                EmittersSection,
                panel_section(
                    PanelSectionProps::new("Emitters").on_add(|| println!("TODO: add emitter")),
                    asset_server,
                ),
            ),
            panel_section(
                PanelSectionProps::new("Colliders").on_add(|| println!("TODO: add collider")),
                asset_server,
            ),
            panel_section(
                PanelSectionProps::new("Attractors").on_add(|| println!("TODO: add attractor")),
                asset_server,
            ),
        ],
    )
}

fn setup_data_panel_resize(
    mut commands: Commands,
    panels: Query<Entity, Added<EditorDataPanel>>,
) {
    for panel_entity in &panels {
        commands
            .entity(panel_entity)
            .with_child(panel_resize_handle(panel_entity, PanelDirection::Left));
    }
}

fn setup_emitters_section(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sections: Query<Entity, Added<EmittersSection>>,
) {
    for entity in &sections {
        commands.entity(entity).with_children(|parent| {
            parent.spawn(button(
                ButtonProps::new("Emitter 1")
                    .variant(ButtonVariant::Active)
                    .align_left()
                    .on_click(|| println!("Emitter 1"))
                    .on_more(|| println!("More Emitter 1...")),
                &asset_server,
            ));
            parent.spawn(button(
                ButtonProps::new("Emitter 2")
                    .variant(ButtonVariant::Ghost)
                    .align_left()
                    .on_click(|| println!("Emitter 2"))
                    .on_more(|| println!("More Emitter 2...")),
                &asset_server,
            ));
        });
    }
}
