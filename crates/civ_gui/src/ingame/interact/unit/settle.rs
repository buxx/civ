use bevy::prelude::*;
use common::{
    game::unit::UnitId,
    network::message::{ClientToServerInGameMessage, ClientToServerUnitMessage},
};
use derive_more::Constructor;

use crate::{
    impl_ui_component_resource,
    ingame::{DrawUiComponent, EGUI_DISPLAY_FACTOR},
    to_server,
    utils::gui::layout::fixed_window,
};

use super::super::UiComponentResource;

#[derive(Debug, Event, Deref)]
pub struct SetupSettleCityName(pub UnitId);

#[derive(Debug, Event)]
pub struct SetupSettle(pub UnitId, pub String);

#[derive(Debug, Constructor)]
pub struct SettleCityName {
    unit_id: UnitId,
    name: String,
}

#[derive(Debug, Resource, Default)]
pub struct SettleCityNameResource(pub Option<SettleCityName>);
impl_ui_component_resource!(SettleCityNameResource, SettleCityName);

pub fn on_setup_settle_city_name(
    trigger: Trigger<SetupSettleCityName>,
    mut modal: ResMut<SettleCityNameResource>,
) {
    let event = trigger.event();
    modal.0 = Some(SettleCityName::new(event.0, String::new()));
}

impl DrawUiComponent for SettleCityName {
    fn draw(
        &mut self,
        ctx: &bevy_egui::egui::Context,
        window: &Window,
        commands: &mut Commands,
    ) -> bool {
        let mut close = false;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.text_edit_singleline(&mut self.name);
                    if ui.button("Ok").clicked() {
                        close = true;
                        commands.trigger(SetupSettle(self.unit_id, self.name.clone()));
                    }
                });
            })
            .call();

        close
    }
}

pub fn on_setup_settle(trigger: Trigger<SetupSettle>, mut commands: Commands) {
    let event = trigger.event();
    to_server!(
        commands,
        ClientToServerInGameMessage::Unit(
            event.0,
            ClientToServerUnitMessage::Settle(event.1.clone())
        )
    );
}
