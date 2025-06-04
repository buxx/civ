use bevy::prelude::*;
use bevy::window::Window;
use bevy_egui::egui::Context;
use common::game::{
    slice::ClientUnit,
    unit::{UnitCan, UnitId},
};

use crate::{
    ingame::{interact::unit::settle::SetupSettleCityName, DrawUiComponent, EGUI_DISPLAY_FACTOR},
    utils::gui::layout::fixed_window,
};

#[derive(Debug)]
pub struct UnitMenu {
    pub unit_id: UnitId,
    pub can: Vec<UnitCan>,
}

impl UnitMenu {
    pub fn from_unit(unit: &ClientUnit) -> Self {
        Self {
            unit_id: *unit.id(),
            can: unit.can().to_vec(),
        }
    }
}

impl DrawUiComponent for UnitMenu {
    fn draw(&mut self, ctx: &Context, window: &Window, commands: &mut Commands) -> bool {
        let mut close = false;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical_centered(|ui| {
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
