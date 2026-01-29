pub mod components;
pub mod tokens;
pub mod widgets;

use bevy::prelude::*;

use components::data_panel::{data_panel, setup_data_panel_resize};
use components::inspector_panel::{inspector_panel, setup_inspector_panel_resize};
use components::sidebar::sidebar;
use components::topbar::topbar;

pub struct EditorUiPlugin;

impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(widgets::button::plugin)
            .add_plugins(widgets::panel::plugin)
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (setup_data_panel_resize, setup_inspector_panel_resize),
            );
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            topbar(&asset_server),
            (
                Node {
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                children![
                    sidebar(&asset_server),
                    data_panel(&asset_server),
                    inspector_panel(&asset_server),
                ],
            ),
        ],
    ));
}
