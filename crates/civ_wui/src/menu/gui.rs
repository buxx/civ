use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn manage_gui(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
