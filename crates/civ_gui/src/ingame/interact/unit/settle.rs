use bevy::prelude::*;
use common::{
    game::{slice::ClientUnit, unit::UnitId},
    network::message::{ClientToServerInGameMessage, ClientToServerUnitMessage},
};
use derive_more::Constructor;

use crate::{
    core::GameSlicePropagated,
    impl_ui_component_resource,
    ingame::{DrawUiComponent, GameSliceResource, EGUI_DISPLAY_FACTOR},
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

impl SettleCityName {
    fn from_unit(unit: &ClientUnit) -> Self {
        Self::new(*unit.id(), String::new())
    }
}

#[derive(Debug, Resource, Default)]
pub struct SettleCityNameResource(pub Option<SettleCityName>);
impl_ui_component_resource!(SettleCityNameResource, SettleCityName);

pub fn on_setup_settle_city_name(
    trigger: Trigger<SetupSettleCityName>,
    slice: Res<GameSliceResource>,
    mut modal: ResMut<SettleCityNameResource>,
) {
    if let Some(slice) = &slice.0 {
        if let Some(unit) = slice.unit(&trigger.event().0) {
            modal.0 = Some(SettleCityName::from_unit(unit));
        }
    }
}

pub fn settle_city_name_on_slice_propagated(
    _trigger: Trigger<GameSlicePropagated>,
    slice: Res<GameSliceResource>,
    mut modal: ResMut<SettleCityNameResource>,
) {
    if let (Some(component), Some(slice)) = (&modal.0, &slice.0) {
        if let Some(unit) = slice.unit(&component.unit_id) {
            modal.0 = Some(SettleCityName::from_unit(unit));
        } else {
            modal.0 = None;
        }
    }
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
