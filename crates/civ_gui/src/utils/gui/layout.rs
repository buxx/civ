use bevy::window::Window;
use bevy_egui::egui::{self, Context, Ui};
use bon::builder;

#[builder]
pub fn fixed_window(
    ctx: &Context,
    window: &Window,
    ui: impl FnOnce(&mut Ui),
    width: Option<f32>,
    height: Option<f32>,
    factor: Option<f32>,
) {
    let window_size = &window.resolution;
    let screen_width = window_size.width() / factor.unwrap_or(1.0);
    let screen_height = window_size.height() / factor.unwrap_or(1.0);

    // Fixed size of the egui window
    let desired_width = width.unwrap_or(300.0) * factor.unwrap_or(1.0);
    let desired_height = height.unwrap_or(200.0) * factor.unwrap_or(1.0);

    let center_x = (screen_width - desired_width) / 2.0;
    let center_y = (screen_height - desired_height) / 2.0;

    egui::Window::new("Centered Window")
        .fixed_size(egui::vec2(desired_width, desired_height))
        .fixed_pos(egui::pos2(center_x, center_y))
        .show(ctx, |ui_| ui(ui_));
}
