use bevy::window::Window;
use bevy_egui::egui::Context;

use crate::{
    ingame::menu::{
        unit::{UnitMenu, UnitMenuEffect},
        MENU_DISPLAY_FACTOR,
    },
    utils::gui::layout::fixed_window,
};

use super::DrawMenu;

impl DrawMenu<UnitMenuEffect> for UnitMenu {
    fn draw(&mut self, ctx: &Context, window: &Window) -> Vec<UnitMenuEffect> {
        let mut effects = vec![];

        fixed_window()
            .ctx(ctx)
            .window(window)
            .factor(MENU_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical_centered(|ui| {
                    for can in &self.can {
                        if ui.button(can.name()).clicked() {
                            effects.push(UnitMenuEffect::Do(can.clone()));
                        }
                    }
                });
            })
            .call();

        effects
    }
}
