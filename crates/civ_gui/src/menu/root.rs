use bevy::prelude::*;
use bevy_egui::egui::Ui;

use super::switch::SwitchMenuScreenEvent;

#[derive(Debug, Default)]
pub struct RootState {}

pub fn draw(ui: &mut Ui, _state: &mut RootState, mut commands: Commands) {
    ui.vertical_centered(|ui| {
        if ui.button("Local game").clicked() {
            commands.trigger(SwitchMenuScreenEvent::Single);
        }
        if ui.button("Join server").clicked() {
            commands.trigger(SwitchMenuScreenEvent::Join);
        }
    });
}
