use bevy::prelude::*;
use bevy_egui::egui::Ui;

use crate::bridge::single::SingleConfiguration;

#[derive(Debug, Default)]
pub struct SingleState {}

#[derive(Event, Deref)]
pub struct StartSingleEvent(pub SingleConfiguration);

pub fn draw(ui: &mut Ui, state: &mut SingleState, mut commands: Commands) {
    ui.vertical_centered(|ui| {
        if ui.button("Start new game (hardcoded for tests").clicked() {
            commands.trigger(StartSingleEvent(SingleConfiguration::from_state(state)));
        }
    });
}
