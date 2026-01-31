use bevy::prelude::*;

use crate::ui::widgets::inspector_field::{InspectorFieldProps, fields_row, spawn_inspector_field};
use crate::ui::widgets::panel_section::{PanelSectionProps, PanelSectionSize, panel_section};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, setup_time_section_fields);
}

#[derive(Component)]
pub struct TimeSection;

#[derive(Component)]
struct TimeSectionInitialized;

pub fn time_section(asset_server: &AssetServer) -> impl Bundle {
    (
        TimeSection,
        panel_section(
            PanelSectionProps::new("Time")
                .collapsible()
                .with_size(PanelSectionSize::XL),
            asset_server,
        ),
    )
}

fn setup_time_section_fields(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sections: Query<Entity, (With<TimeSection>, Without<TimeSectionInitialized>)>,
) {
    for entity in &sections {
        commands
            .entity(entity)
            .insert(TimeSectionInitialized)
            .with_children(|parent| {
                parent.spawn(fields_row()).with_children(|row| {
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.lifetime")
                            .with_icon("icons/ri-time-line.png")
                            .with_suffix("s"),
                        &asset_server,
                    );
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.lifetime_randomness")
                            .percent()
                            .with_icon("icons/ri-time-line.png"),
                        &asset_server,
                    );
                });

                parent.spawn(fields_row()).with_children(|row| {
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.delay")
                            .with_min(0.)
                            .with_icon("icons/ri-time-line.png")
                            .with_suffix("s"),
                        &asset_server,
                    );
                });

                parent.spawn(fields_row()).with_children(|row| {
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.explosiveness").percent(),
                        &asset_server,
                    );
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.spawn_time_randomness").percent(),
                        &asset_server,
                    );
                });

                parent.spawn(fields_row()).with_children(|row| {
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.fixed_fps")
                            .u32_or_empty()
                            .with_placeholder("Unlimited"),
                        &asset_server,
                    );
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.fixed_seed")
                            .optional_u32()
                            .with_icon("icons/ri-seedling-fill.png")
                            .with_placeholder("Random"),
                        &asset_server,
                    );
                });

                parent.spawn(fields_row()).with_children(|row| {
                    spawn_inspector_field(
                        row,
                        InspectorFieldProps::new("time.one_shot").bool(),
                        &asset_server,
                    );
                });
            });
    }
}
