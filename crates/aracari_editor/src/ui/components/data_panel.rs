use bevy::prelude::*;

use crate::ui::tokens::{FONT_PATH, TEXT_BODY_COLOR, TEXT_SIZE};
use crate::ui::widgets::panel::{PanelDirection, PanelProps, panel, panel_resize_handle};

#[derive(Component)]
pub struct EditorDataPanel;

pub fn data_panel(asset_server: &AssetServer) -> impl Bundle {
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    (
        EditorDataPanel,
        panel(
            PanelProps::new(PanelDirection::Left)
                .with_width(224)
                .with_min_width(160)
                .with_max_width(320),
        ),
        children![(
            Node {
                padding: UiRect::all(px(12)),
                ..default()
            },
            children![(
                Text::new("TODO: DataPanel"),
                TextFont {
                    font: font.into(),
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(TEXT_BODY_COLOR.into()),
            )],
        )],
    )
}

pub fn setup_data_panel_resize(
    mut commands: Commands,
    panels: Query<Entity, Added<EditorDataPanel>>,
) {
    for panel_entity in &panels {
        commands
            .entity(panel_entity)
            .with_child(panel_resize_handle(panel_entity, PanelDirection::Left));
    }
}
