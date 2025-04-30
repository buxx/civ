use bevy::prelude::*;
use bevy_egui::egui::Ui;

#[derive(Debug, Default)]
pub struct SingleState {}

#[derive(Event)]
pub struct StartSingleEvent;

pub fn draw(ui: &mut Ui, _state: &mut SingleState, mut commands: Commands) {
    ui.vertical_centered(|ui| {
        if ui.button("Start new game (hardcoded for tests)").clicked() {
            commands.trigger(StartSingleEvent);
        }
    });
}
