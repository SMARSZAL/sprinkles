use bevy_egui::egui::{self, Color32, CornerRadius, FontId, Pos2, Rect, Response, Sense, Stroke, Vec2};
use egui_remixicon::icons;

use super::styles::{colors, TEXT_BASE, TEXT_SM};

const POPOVER_WIDTH: f32 = 256.0;
const SPACING: f32 = 4.0;
const HUE_BAR_WIDTH: f32 = 16.0;
const CHANNEL_BAR_HEIGHT: f32 = 20.0;
const SELECTOR_CIRCLE_RADIUS: f32 = 4.0;
const SELECTOR_RECT_HEIGHT: f32 = 4.0;
const SELECTOR_RECT_OVERFLOW: f32 = 2.0;
const CHECKER_SIZE: f32 = 4.0;
const CORNER_RADIUS: f32 = 2.0;
const VALUE_INPUT_WIDTH: f32 = 36.0;

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r + m, g + m, b + m)
}

fn hue_to_rgb(h: f32) -> Color32 {
    let (r, g, b) = hsv_to_rgb(h, 1.0, 1.0);
    Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn draw_selector_circle(ui: &mut egui::Ui, center: Pos2, color: Color32) {
    let painter = ui.painter();
    // fill with current color
    painter.circle_filled(center, SELECTOR_CIRCLE_RADIUS, color);
    // outer white border
    painter.circle_stroke(center, SELECTOR_CIRCLE_RADIUS + 1.0, Stroke::new(1.0, Color32::WHITE));
    // inner black border
    painter.circle_stroke(center, SELECTOR_CIRCLE_RADIUS, Stroke::new(1.0, Color32::BLACK));
}

fn draw_selector_rect_horizontal(ui: &mut egui::Ui, center_y: f32, rect: Rect, color: Color32) {
    let painter = ui.painter();
    let corner_radius = CornerRadius::same(CORNER_RADIUS as u8);
    let selector_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, center_y),
        Vec2::new(rect.width() + SELECTOR_RECT_OVERFLOW * 2.0, SELECTOR_RECT_HEIGHT),
    );
    // fill with current color
    painter.rect_filled(selector_rect, corner_radius, color);
    // outer white border
    painter.rect_stroke(selector_rect, corner_radius, Stroke::new(1.0, Color32::WHITE), egui::StrokeKind::Outside);
    // inner black border
    painter.rect_stroke(selector_rect, corner_radius, Stroke::new(1.0, Color32::BLACK), egui::StrokeKind::Inside);
}

fn draw_selector_rect_vertical(ui: &mut egui::Ui, center_x: f32, rect: Rect, color: Color32) {
    let painter = ui.painter();
    let corner_radius = CornerRadius::same(CORNER_RADIUS as u8);
    let selector_rect = Rect::from_center_size(
        Pos2::new(center_x, rect.center().y),
        Vec2::new(SELECTOR_RECT_HEIGHT, rect.height() + SELECTOR_RECT_OVERFLOW * 2.0),
    );
    // fill with current color
    painter.rect_filled(selector_rect, corner_radius, color);
    // outer white border
    painter.rect_stroke(selector_rect, corner_radius, Stroke::new(1.0, Color32::WHITE), egui::StrokeKind::Outside);
    // inner black border
    painter.rect_stroke(selector_rect, corner_radius, Stroke::new(1.0, Color32::BLACK), egui::StrokeKind::Inside);
}

fn draw_checkerboard(ui: &mut egui::Ui, rect: Rect) {
    let painter = ui.painter();
    let cols = (rect.width() / CHECKER_SIZE).ceil() as i32;
    let rows = (rect.height() / CHECKER_SIZE).ceil() as i32;

    for row in 0..rows {
        for col in 0..cols {
            let is_light = (row + col) % 2 == 0;
            let color = if is_light {
                Color32::from_gray(180)
            } else {
                Color32::from_gray(120)
            };

            let cell_rect = Rect::from_min_size(
                Pos2::new(
                    rect.min.x + col as f32 * CHECKER_SIZE,
                    rect.min.y + row as f32 * CHECKER_SIZE,
                ),
                Vec2::splat(CHECKER_SIZE),
            );

            // clip to parent rect
            let clipped = cell_rect.intersect(rect);
            if clipped.width() > 0.0 && clipped.height() > 0.0 {
                painter.rect_filled(clipped, CornerRadius::ZERO, color);
            }
        }
    }
}

fn hsv_square(ui: &mut egui::Ui, hue: f32, saturation: &mut f32, value: &mut f32, size: f32) -> bool {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click_and_drag());
    let mut changed = false;
    let corner_radius = CornerRadius::same(CORNER_RADIUS as u8);

    if ui.input(|i| i.pointer.any_down()) && response.contains_pointer() {
        if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
            let local_pos = pos - rect.min;
            *saturation = (local_pos.x / size).clamp(0.0, 1.0);
            *value = 1.0 - (local_pos.y / size).clamp(0.0, 1.0);
            changed = true;
        }
    }

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let r = CORNER_RADIUS;

        // helper to get color at normalized position
        let color_at = |sx: f32, sy: f32| -> Color32 {
            let (rv, gv, bv) = hsv_to_rgb(hue, sx, 1.0 - sy);
            Color32::from_rgb((rv * 255.0) as u8, (gv * 255.0) as u8, (bv * 255.0) as u8)
        };

        // draw the HSV square using a mesh for smooth gradients
        let mut mesh = egui::Mesh::default();

        let steps = 32;
        for y in 0..steps {
            for x in 0..steps {
                let x0 = rect.min.x + (x as f32 / steps as f32) * size;
                let x1 = rect.min.x + ((x + 1) as f32 / steps as f32) * size;
                let y0 = rect.min.y + (y as f32 / steps as f32) * size;
                let y1 = rect.min.y + ((y + 1) as f32 / steps as f32) * size;

                let s0 = x as f32 / steps as f32;
                let s1 = (x + 1) as f32 / steps as f32;
                let v0 = y as f32 / steps as f32;
                let v1 = (y + 1) as f32 / steps as f32;

                let c00 = color_at(s0, v0);
                let c10 = color_at(s1, v0);
                let c01 = color_at(s0, v1);
                let c11 = color_at(s1, v1);

                let idx = mesh.vertices.len() as u32;
                mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x0, y0), uv: egui::epaint::WHITE_UV, color: c00 });
                mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x1, y0), uv: egui::epaint::WHITE_UV, color: c10 });
                mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x0, y1), uv: egui::epaint::WHITE_UV, color: c01 });
                mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x1, y1), uv: egui::epaint::WHITE_UV, color: c11 });

                mesh.indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx + 1, idx + 3, idx + 2]);
            }
        }

        painter.add(egui::Shape::mesh(mesh));

        // draw corner covers with background color, then quarter circles with corner colors
        // mask size is corner radius + 1px offset to fully cover overflow
        let bg = colors::WINDOW_BG;
        let mask_r = r + 1.0;

        // corner centers and colors
        let corners = [
            (Pos2::new(rect.min.x + mask_r, rect.min.y + mask_r), color_at(0.0, 0.0)),  // top-left
            (Pos2::new(rect.max.x - mask_r, rect.min.y + mask_r), color_at(1.0, 0.0)),  // top-right
            (Pos2::new(rect.min.x + mask_r, rect.max.y - mask_r), color_at(0.0, 1.0)),  // bottom-left
            (Pos2::new(rect.max.x - mask_r, rect.max.y - mask_r), color_at(1.0, 1.0)),  // bottom-right
        ];

        let corner_rects = [
            Rect::from_min_size(rect.min, Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.max.x - mask_r, rect.min.y), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.min.x, rect.max.y - mask_r), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.max.x - mask_r, rect.max.y - mask_r), Vec2::splat(mask_r)),
        ];

        for i in 0..4 {
            painter.rect_filled(corner_rects[i], CornerRadius::ZERO, bg);
            painter.circle_filled(corners[i].0, mask_r, corners[i].1);
        }

        // draw border
        painter.rect_stroke(rect, corner_radius, Stroke::new(1.0, colors::BORDER), egui::StrokeKind::Inside);

        // draw selector circle
        let circle_pos = Pos2::new(
            rect.min.x + *saturation * size,
            rect.min.y + (1.0 - *value) * size,
        );
        let (r, g, b) = hsv_to_rgb(hue, *saturation, *value);
        let selector_color = Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
        draw_selector_circle(ui, circle_pos, selector_color);
    }

    changed
}

fn hue_bar(ui: &mut egui::Ui, hue: &mut f32, height: f32) -> bool {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(HUE_BAR_WIDTH, height), Sense::click_and_drag());
    let mut changed = false;
    let corner_radius = CornerRadius::same(CORNER_RADIUS as u8);

    if ui.input(|i| i.pointer.any_down()) && response.contains_pointer() {
        if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
            let local_y = pos.y - rect.min.y;
            *hue = (local_y / height).clamp(0.0, 1.0) * 360.0;
            changed = true;
        }
    }

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let r = CORNER_RADIUS;

        // draw hue gradient
        let mut mesh = egui::Mesh::default();
        let steps = 36;
        for i in 0..steps {
            let y0 = rect.min.y + (i as f32 / steps as f32) * height;
            let y1 = rect.min.y + ((i + 1) as f32 / steps as f32) * height;

            let h0 = (i as f32 / steps as f32) * 360.0;
            let h1 = ((i + 1) as f32 / steps as f32) * 360.0;

            let c0 = hue_to_rgb(h0);
            let c1 = hue_to_rgb(h1);

            let idx = mesh.vertices.len() as u32;
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(rect.min.x, y0), uv: egui::epaint::WHITE_UV, color: c0 });
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(rect.max.x, y0), uv: egui::epaint::WHITE_UV, color: c0 });
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(rect.min.x, y1), uv: egui::epaint::WHITE_UV, color: c1 });
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(rect.max.x, y1), uv: egui::epaint::WHITE_UV, color: c1 });
            mesh.indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx + 1, idx + 3, idx + 2]);
        }
        painter.add(egui::Shape::mesh(mesh));

        // draw corner covers with background color, then quarter circles with corner colors
        // mask size is corner radius + 1px offset to fully cover overflow
        let bg = colors::WINDOW_BG;
        let mask_r = r + 1.0;
        let top_color = hue_to_rgb(0.0);
        let bottom_color = hue_to_rgb(360.0);

        let corners = [
            (Pos2::new(rect.min.x + mask_r, rect.min.y + mask_r), top_color),     // top-left
            (Pos2::new(rect.max.x - mask_r, rect.min.y + mask_r), top_color),     // top-right
            (Pos2::new(rect.min.x + mask_r, rect.max.y - mask_r), bottom_color),  // bottom-left
            (Pos2::new(rect.max.x - mask_r, rect.max.y - mask_r), bottom_color),  // bottom-right
        ];

        let corner_rects = [
            Rect::from_min_size(rect.min, Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.max.x - mask_r, rect.min.y), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.min.x, rect.max.y - mask_r), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.max.x - mask_r, rect.max.y - mask_r), Vec2::splat(mask_r)),
        ];

        for i in 0..4 {
            painter.rect_filled(corner_rects[i], CornerRadius::ZERO, bg);
            painter.circle_filled(corners[i].0, mask_r, corners[i].1);
        }

        // draw border
        painter.rect_stroke(rect, corner_radius, Stroke::new(1.0, colors::BORDER), egui::StrokeKind::Inside);

        // draw selector
        let selector_y = rect.min.y + (*hue / 360.0) * height;
        let selector_color = hue_to_rgb(*hue);
        draw_selector_rect_horizontal(ui, selector_y, rect, selector_color);
    }

    changed
}

fn channel_bar(
    ui: &mut egui::Ui,
    value: &mut f32,
    width: f32,
    gradient_fn: impl Fn(f32) -> Color32,
    with_checkerboard: bool,
) -> bool {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, CHANNEL_BAR_HEIGHT), Sense::click_and_drag());
    let mut changed = false;
    let corner_radius = CornerRadius::same(CORNER_RADIUS as u8);

    if ui.input(|i| i.pointer.any_down()) && response.contains_pointer() {
        if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
            let local_x = pos.x - rect.min.x;
            *value = (local_x / width).clamp(0.0, 1.0);
            changed = true;
        }
    }

    if ui.is_rect_visible(rect) {
        let r = CORNER_RADIUS;

        // draw checkerboard for alpha
        if with_checkerboard {
            draw_checkerboard(ui, rect);
        }

        // draw gradient
        let mut mesh = egui::Mesh::default();
        let steps = 32;
        for i in 0..steps {
            let x0 = rect.min.x + (i as f32 / steps as f32) * width;
            let x1 = rect.min.x + ((i + 1) as f32 / steps as f32) * width;

            let t0 = i as f32 / steps as f32;
            let t1 = (i + 1) as f32 / steps as f32;

            let c0 = gradient_fn(t0);
            let c1 = gradient_fn(t1);

            let idx = mesh.vertices.len() as u32;
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x0, rect.min.y), uv: egui::epaint::WHITE_UV, color: c0 });
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x1, rect.min.y), uv: egui::epaint::WHITE_UV, color: c1 });
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x0, rect.max.y), uv: egui::epaint::WHITE_UV, color: c0 });
            mesh.vertices.push(egui::epaint::Vertex { pos: Pos2::new(x1, rect.max.y), uv: egui::epaint::WHITE_UV, color: c1 });
            mesh.indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx + 1, idx + 3, idx + 2]);
        }
        ui.painter().add(egui::Shape::mesh(mesh));

        // draw corner covers with background color, then quarter circles with corner colors
        // mask size is corner radius + 1px offset to fully cover overflow
        let bg = colors::WINDOW_BG;
        let mask_r = r + 1.0;
        let left_color = gradient_fn(0.0);
        let right_color = gradient_fn(1.0);

        let corners = [
            (Pos2::new(rect.min.x + mask_r, rect.min.y + mask_r), left_color),   // top-left
            (Pos2::new(rect.max.x - mask_r, rect.min.y + mask_r), right_color),  // top-right
            (Pos2::new(rect.min.x + mask_r, rect.max.y - mask_r), left_color),   // bottom-left
            (Pos2::new(rect.max.x - mask_r, rect.max.y - mask_r), right_color),  // bottom-right
        ];

        let corner_rects = [
            Rect::from_min_size(rect.min, Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.max.x - mask_r, rect.min.y), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.min.x, rect.max.y - mask_r), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(rect.max.x - mask_r, rect.max.y - mask_r), Vec2::splat(mask_r)),
        ];

        for i in 0..4 {
            ui.painter().rect_filled(corner_rects[i], CornerRadius::ZERO, bg);
            ui.painter().circle_filled(corners[i].0, mask_r, corners[i].1);
        }

        // draw border
        ui.painter().rect_stroke(rect, corner_radius, Stroke::new(1.0, colors::BORDER), egui::StrokeKind::Inside);

        // draw selector
        let selector_x = rect.min.x + *value * width;
        let selector_color = gradient_fn(*value);
        draw_selector_rect_vertical(ui, selector_x, rect, selector_color);
    }

    changed
}

fn channel_value_input(ui: &mut egui::Ui, value: &mut f32) -> bool {
    let mut changed = false;
    let corner_radius = CornerRadius::same(CORNER_RADIUS as u8);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(VALUE_INPUT_WIDTH, CHANNEL_BAR_HEIGHT), Sense::hover());

    // draw background
    ui.painter().rect_filled(rect, corner_radius, colors::INPUT_BG);

    let value_u8 = (*value * 255.0).round() as u8;
    let mut text = value_u8.to_string();
    let response = ui.put(
        rect,
        egui::TextEdit::singleline(&mut text)
            .horizontal_align(egui::Align::Center)
            .font(FontId::proportional(TEXT_BASE))
            .text_color(colors::TEXT_MUTED)
            .background_color(Color32::TRANSPARENT)
            .frame(false),
    );

    // draw border after TextEdit
    ui.painter().rect_stroke(rect, corner_radius, Stroke::new(1.0, colors::BORDER), egui::StrokeKind::Inside);

    if response.changed() {
        if let Ok(new_value) = text.parse::<u8>() {
            *value = new_value as f32 / 255.0;
            changed = true;
        }
    }

    changed
}

pub fn color_picker(ui: &mut egui::Ui, rgba: &mut [f32; 4], width: f32) -> Response {
    let color = Color32::from_rgba_unmultiplied(
        (rgba[0] * 255.0) as u8,
        (rgba[1] * 255.0) as u8,
        (rgba[2] * 255.0) as u8,
        (rgba[3] * 255.0) as u8,
    );

    let button_height = 24.0;
    let (button_rect, mut button_response) = ui.allocate_exact_size(Vec2::new(width, button_height), Sense::click());
    let corner_radius = CornerRadius::same(CORNER_RADIUS as u8);

    // draw button with checkerboard background and color
    if ui.is_rect_visible(button_rect) {
        let r = CORNER_RADIUS;
        let mask_r = r + 1.0;
        let bg = colors::PANEL_BG;

        ui.painter().rect_filled(button_rect, corner_radius, bg);
        draw_checkerboard(ui, button_rect);
        ui.painter().rect_filled(button_rect, corner_radius, color);

        // draw corner masks
        let corner_rects = [
            Rect::from_min_size(button_rect.min, Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(button_rect.max.x - mask_r, button_rect.min.y), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(button_rect.min.x, button_rect.max.y - mask_r), Vec2::splat(mask_r)),
            Rect::from_min_size(Pos2::new(button_rect.max.x - mask_r, button_rect.max.y - mask_r), Vec2::splat(mask_r)),
        ];

        let corner_centers = [
            Pos2::new(button_rect.min.x + mask_r, button_rect.min.y + mask_r),
            Pos2::new(button_rect.max.x - mask_r, button_rect.min.y + mask_r),
            Pos2::new(button_rect.min.x + mask_r, button_rect.max.y - mask_r),
            Pos2::new(button_rect.max.x - mask_r, button_rect.max.y - mask_r),
        ];

        for i in 0..4 {
            ui.painter().rect_filled(corner_rects[i], CornerRadius::ZERO, bg);
            ui.painter().circle_filled(corner_centers[i], mask_r, color);
        }

        ui.painter().rect_stroke(
            button_rect,
            corner_radius,
            Stroke::new(1.0, colors::BORDER),
            egui::StrokeKind::Inside,
        );
    }

    let popup_id = ui.make_persistent_id("color_picker_popup");
    let initial_color_id = popup_id.with("initial_color");
    let mut is_open = ui.data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

    if button_response.clicked() {
        is_open = !is_open;
        ui.data_mut(|d| d.insert_temp(popup_id, is_open));
        if is_open {
            ui.data_mut(|d| d.insert_temp(initial_color_id, *rgba));
        }
    }

    let initial_rgba = ui.data(|d| d.get_temp::<[f32; 4]>(initial_color_id).unwrap_or(*rgba));

    let mut changed = false;

    if is_open {
        let popup_pos = button_rect.left_bottom() + Vec2::new(0.0, 4.0);

        let area_response = egui::Area::new(popup_id)
            .order(egui::Order::Foreground)
            .fixed_pos(popup_pos)
            .show(ui.ctx(), |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.set_width(POPOVER_WIDTH);
                    ui.spacing_mut().item_spacing = Vec2::splat(SPACING);

                    // convert to HSV for editing
                    let (mut hue, mut saturation, mut value) = rgb_to_hsv(rgba[0], rgba[1], rgba[2]);

                    let square_size = POPOVER_WIDTH - HUE_BAR_WIDTH - SPACING;

                    // HSV square and hue bar
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = SPACING;

                        if hsv_square(ui, hue, &mut saturation, &mut value, square_size) {
                            let (r, g, b) = hsv_to_rgb(hue, saturation, value);
                            rgba[0] = r;
                            rgba[1] = g;
                            rgba[2] = b;
                            changed = true;
                        }

                        if hue_bar(ui, &mut hue, square_size) {
                            let (r, g, b) = hsv_to_rgb(hue, saturation, value);
                            rgba[0] = r;
                            rgba[1] = g;
                            rgba[2] = b;
                            changed = true;
                        }
                    });

                    // Previous/new color comparison
                    let color_box_height = 24.0;
                    let color_box_width = POPOVER_WIDTH / 2.0;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        // Previous color (left, rounded on left)
                        let (prev_rect, _) = ui.allocate_exact_size(Vec2::new(color_box_width, color_box_height), Sense::hover());
                        let prev_color = Color32::from_rgba_unmultiplied(
                            (initial_rgba[0] * 255.0) as u8,
                            (initial_rgba[1] * 255.0) as u8,
                            (initial_rgba[2] * 255.0) as u8,
                            (initial_rgba[3] * 255.0) as u8,
                        );
                        let left_radius = CornerRadius { nw: CORNER_RADIUS as u8, sw: CORNER_RADIUS as u8, ne: 0, se: 0 };
                        draw_checkerboard(ui, prev_rect);
                        ui.painter().rect_filled(prev_rect, left_radius, prev_color);

                        // New color (right, rounded on right)
                        let (new_rect, _) = ui.allocate_exact_size(Vec2::new(color_box_width, color_box_height), Sense::hover());
                        let new_color = Color32::from_rgba_unmultiplied(
                            (rgba[0] * 255.0) as u8,
                            (rgba[1] * 255.0) as u8,
                            (rgba[2] * 255.0) as u8,
                            (rgba[3] * 255.0) as u8,
                        );
                        let right_radius = CornerRadius { nw: 0, sw: 0, ne: CORNER_RADIUS as u8, se: CORNER_RADIUS as u8 };
                        draw_checkerboard(ui, new_rect);
                        ui.painter().rect_filled(new_rect, right_radius, new_color);
                    });

                    let label_width = 12.0;
                    let bar_width = POPOVER_WIDTH - label_width - SPACING - VALUE_INPUT_WIDTH - SPACING;

                    // R channel
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = SPACING;
                        ui.add_sized(Vec2::new(label_width, CHANNEL_BAR_HEIGHT), egui::Label::new(
                            egui::RichText::new("R").size(TEXT_SM).color(colors::AXIS_X)
                        ));
                        let base_g = rgba[1];
                        let base_b = rgba[2];
                        let base_a = rgba[3];
                        if channel_bar(ui, &mut rgba[0], bar_width, |t| {
                            Color32::from_rgba_unmultiplied(
                                (t * 255.0) as u8,
                                (base_g * 255.0) as u8,
                                (base_b * 255.0) as u8,
                                (base_a * 255.0) as u8,
                            )
                        }, false) {
                            changed = true;
                        }
                        if channel_value_input(ui, &mut rgba[0]) {
                            changed = true;
                        }
                    });

                    // G channel
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = SPACING;
                        ui.add_sized(Vec2::new(label_width, CHANNEL_BAR_HEIGHT), egui::Label::new(
                            egui::RichText::new("G").size(TEXT_SM).color(colors::AXIS_Y)
                        ));
                        let base_r = rgba[0];
                        let base_b = rgba[2];
                        let base_a = rgba[3];
                        if channel_bar(ui, &mut rgba[1], bar_width, |t| {
                            Color32::from_rgba_unmultiplied(
                                (base_r * 255.0) as u8,
                                (t * 255.0) as u8,
                                (base_b * 255.0) as u8,
                                (base_a * 255.0) as u8,
                            )
                        }, false) {
                            changed = true;
                        }
                        if channel_value_input(ui, &mut rgba[1]) {
                            changed = true;
                        }
                    });

                    // B channel
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = SPACING;
                        ui.add_sized(Vec2::new(label_width, CHANNEL_BAR_HEIGHT), egui::Label::new(
                            egui::RichText::new("B").size(TEXT_SM).color(colors::AXIS_Z)
                        ));
                        let base_r = rgba[0];
                        let base_g = rgba[1];
                        let base_a = rgba[3];
                        if channel_bar(ui, &mut rgba[2], bar_width, |t| {
                            Color32::from_rgba_unmultiplied(
                                (base_r * 255.0) as u8,
                                (base_g * 255.0) as u8,
                                (t * 255.0) as u8,
                                (base_a * 255.0) as u8,
                            )
                        }, false) {
                            changed = true;
                        }
                        if channel_value_input(ui, &mut rgba[2]) {
                            changed = true;
                        }
                    });

                    // A channel
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = SPACING;
                        ui.add_sized(Vec2::new(label_width, CHANNEL_BAR_HEIGHT), egui::Label::new(
                            egui::RichText::new("A").size(TEXT_SM).color(colors::TEXT_MUTED)
                        ));
                        let base_r = rgba[0];
                        let base_g = rgba[1];
                        let base_b = rgba[2];
                        if channel_bar(ui, &mut rgba[3], bar_width, |t| {
                            Color32::from_rgba_unmultiplied(
                                (base_r * 255.0) as u8,
                                (base_g * 255.0) as u8,
                                (base_b * 255.0) as u8,
                                (t * 255.0) as u8,
                            )
                        }, true) {
                            changed = true;
                        }
                        if channel_value_input(ui, &mut rgba[3]) {
                            changed = true;
                        }
                    });

                    // Confirm button
                    ui.add_space(SPACING);
                    if ui.add_sized(
                        Vec2::new(POPOVER_WIDTH, 24.0),
                        egui::Button::new(format!("{} Confirm", icons::CHECK_FILL))
                    ).clicked() {
                        ui.data_mut(|d| d.insert_temp(popup_id, false));
                    }
                });
            });

        // Close on click outside (check for new press, not release)
        let dominated = ui.input(|i| i.pointer.any_pressed());
        if dominated {
            if let Some(pos) = ui.input(|i| i.pointer.press_origin()) {
                let popup_rect = area_response.response.rect;
                if !popup_rect.contains(pos) && !button_rect.contains(pos) {
                    ui.data_mut(|d| d.insert_temp(popup_id, false));
                }
            }
        }
    }

    if changed {
        button_response.mark_changed();
    }

    button_response
}
