use bevy::prelude::*;
use bevy_egui::egui::{Color32, RichText};
use common::{
    game::{slice::ClientUnit, unit::UnitId, GameFrame},
    network::message::{ClientToServerInGameMessage, ClientToServerUnitMessage},
};
use derive_more::Constructor;

use crate::{
    impl_ui_component_resource,
    ingame::{interact::WithUnitId, DrawUiComponent, EGUI_DISPLAY_FACTOR},
    to_server,
    utils::gui::layout::fixed_window,
};

#[derive(Debug, Event, Deref)]
pub struct SetupSettleCityName(pub UnitId);

impl WithUnitId for SetupSettleCityName {
    fn unit_id(&self) -> &UnitId {
        &self.0
    }
}

#[derive(Debug, Event)]
pub struct SetupSettle(pub UnitId, pub String);

#[derive(Debug, Constructor)]
pub struct SettleCityName {
    unit_id: UnitId,
    name: String,
    error: Option<String>,
}

impl From<ClientUnit> for SettleCityName {
    fn from(unit: ClientUnit) -> Self {
        Self::new(*unit.id(), String::new(), None)
    }
}
impl WithUnitId for SettleCityName {
    fn unit_id(&self) -> &UnitId {
        &self.unit_id
    }
}

#[derive(Debug, Resource, Default)]
pub struct SettleCityNameResource(pub Option<SettleCityName>);
impl_ui_component_resource!(SettleCityNameResource, SettleCityName, SetupSettleCityName);

impl DrawUiComponent for SettleCityName {
    fn draw(
        &mut self,
        ctx: &bevy_egui::egui::Context,
        window: &Window,
        commands: &mut Commands,
        _frame: GameFrame,
    ) -> bool {
        let mut close = false;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .title("City name")
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical(|ui| {
                    if let Some(error) = &self.error {
                        ui.label(RichText::new(error).color(Color32::RED));
                    }

                    ui.horizontal_wrapped(|ui| {
                        if ui.text_edit_singleline(&mut self.name).changed() {
                            self.error = None;
                        }

                        if ui.button("Ok").clicked() {
                            if self.name.is_empty() {
                                self.error = Some(String::from("Please fill a name"));
                            } else {
                                close = true;
                                commands.trigger(SetupSettle(self.unit_id, self.name.clone()));
                            }
                        }
                    });
                });
            })
            .call();

        close
    }
}

pub fn on_setup_settle(trigger: Trigger<SetupSettle>, mut commands: Commands) {
    let event = trigger.event();
    debug!("Send settle request to server ({}, {})", event.0, &event.1);

    to_server!(
        commands,
        ClientToServerInGameMessage::Unit(
            event.0,
            ClientToServerUnitMessage::Settle(event.1.clone())
        )
    );
}
