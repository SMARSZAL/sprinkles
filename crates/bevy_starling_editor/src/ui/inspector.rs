use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::ui::styles::colors;
use crate::viewport::ViewportLayout;

pub fn draw_inspector(mut contexts: EguiContexts, mut layout: ResMut<ViewportLayout>) -> Result {
    let ctx = contexts.ctx_mut()?;

    let response = egui::SidePanel::left("inspector")
        .resizable(true)
        .default_width(384.0)
        .min_width(200.0)
        .frame(
            egui::Frame::NONE
                .fill(colors::PANEL_BG)
                .inner_margin(egui::Margin::same(8)),
        )
        .show(ctx, |ui| {
            ui.heading("Inspector");
            ui.separator();
        });

    layout.left_panel_width = response.response.rect.width();

    Ok(())
}
