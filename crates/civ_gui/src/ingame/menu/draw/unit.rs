use bevy::window::Window;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

use crate::ingame::menu::unit::UnitMenu;

use super::DrawMenu;

impl DrawMenu for UnitMenu {
    fn draw(&mut self, mut contexts: EguiContexts, window: &Window) {
        let window_size = &window.resolution;
        let screen_width = window_size.width();
        let screen_height = window_size.height();

        // Fixed size of the egui window
        let desired_width = 300.0;
        let desired_height = 200.0;

        let center_x = (screen_width - desired_width) / 2.0;
        let center_y = (screen_height - desired_height) / 2.0;

        egui::Window::new("Centered Window")
            .fixed_size(egui::vec2(desired_width, desired_height))
            .fixed_pos(egui::pos2(center_x, center_y))
            .show(contexts.ctx_mut(), |ui| {
                ui.label("This is a centered, fixed-size window.");
            });
    }
}
