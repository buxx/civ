use bevy::prelude::*;
use bevy::window::Window;
use bevy_egui::egui::Context;
use common::game::unit::UnitCan;

use crate::{
    ingame::{
        interact::settle::SetupSettleCityName, menu::unit::UnitMenu, DrawUiComponent,
        EGUI_DISPLAY_FACTOR,
    },
    utils::gui::layout::fixed_window,
};

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
