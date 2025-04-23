use bevy::prelude::*;
use bevy_egui::egui::Ui;

use super::switch::Switch;

#[derive(Debug, Default)]
pub struct SingleState {}

pub fn draw(ui: &mut Ui, _state: &mut SingleState, mut commands: Commands) {
    ui.vertical_centered(|ui| {
        if ui.button("Start new game").clicked() {
            commands.trigger(Switch::Single);
        }
    });
}
