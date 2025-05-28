use bevy::prelude::*;
use bevy::window::Window;
use bevy_egui::egui::Context;
use common::game::unit::UnitCan;

use crate::{
    ingame::{
        menu::{unit::UnitMenu, MENU_DISPLAY_FACTOR},
        user::interact::settle::SetupSettle,
    },
    utils::gui::layout::fixed_window,
};

use super::DrawMenu;

impl DrawMenu for UnitMenu {
    fn draw(&mut self, ctx: &Context, window: &Window, commands: &mut Commands) -> bool {
        let mut close = false;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .factor(MENU_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical_centered(|ui| {
                    for can in &self.can {
                        if ui.button(can.name()).clicked() {
                            let event = match can {
                                UnitCan::Settle => SetupSettle(self.unit_id),
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
