use bevy::prelude::*;
use bevy_egui::egui::Ui;

use super::{state::MenuState, switch::Switch};

#[derive(Debug, Default)]
pub struct RootState {}

pub fn draw(ui: &mut Ui, _state: &mut RootState, mut commands: Commands) {
    ui.vertical_centered(|ui| {
        if ui.button("Local game").clicked() {
            commands.trigger(Switch::Single);
        }
        if ui.button("Join server").clicked() {
            commands.trigger(Switch::Join);
        }
    });
}
