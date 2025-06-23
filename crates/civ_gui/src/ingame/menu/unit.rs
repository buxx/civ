use bevy::prelude::*;
use bevy::window::Window;
use bevy_egui::egui::Context;
use common::game::{
    slice::ClientUnit,
    unit::{UnitCan, UnitId},
    GameFrame,
};

use crate::{
    impl_ui_component_resource,
    ingame::{
        interact::{
            unit::{info::SetupUnitInfo, settle::SetupSettleCityName},
            FromUnit, WithUnitId,
        },
        DrawUiComponent, EGUI_DISPLAY_FACTOR,
    },
    utils::gui::layout::fixed_window,
};

#[derive(Debug, Event)]
pub struct SetupUnitMenu(pub UnitId);

impl WithUnitId for SetupUnitMenu {
    fn unit_id(&self) -> &UnitId {
        &self.0
    }
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct UnitMenuResource(pub Option<UnitMenu>);
impl_ui_component_resource!(UnitMenuResource, UnitMenu, SetupUnitMenu);

#[derive(Debug)]
pub struct UnitMenu {
    pub unit_id: UnitId,
    pub can: Vec<UnitCan>,
}

impl FromUnit for UnitMenu {
    fn from_unit(unit: &ClientUnit) -> Self {
        Self {
            unit_id: *unit.id(),
            can: unit.can().to_vec(),
        }
    }
}

impl DrawUiComponent for UnitMenu {
    fn draw(
        &mut self,
        ctx: &Context,
        window: &Window,
        commands: &mut Commands,
        _frame: GameFrame,
    ) -> bool {
        let mut close = false;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .title("Unit menu")
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical_centered(|ui| {
                    if ui.button("Infos").clicked() {
                        commands.trigger(SetupUnitInfo(self.unit_id));
                        close = true;
                    }

                    for can in &self.can {
                        if ui.button(can.name()).clicked() {
                            let event = match can {
                                UnitCan::Settle => SetupSettleCityName(self.unit_id),
                            };

                            commands.trigger(event);
                            close = true;
                        }
                    }
                });
            })
            .call();

        close
    }
}

// TODO: Derive on attribute
impl WithUnitId for UnitMenu {
    fn unit_id(&self) -> &UnitId {
        &self.unit_id
    }
}
