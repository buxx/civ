use bevy::prelude::*;

use unit::UnitMenu;

use super::DrawUiComponent;

pub mod draw;
pub mod unit;

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct MenuResource(pub Option<Menu>);

#[derive(Debug)]
pub enum Menu {
    UnitMenu(UnitMenu),
}

impl DrawUiComponent for Menu {
    fn draw(
        &mut self,
        ctx: &bevy_egui::egui::Context,
        window: &Window,
        commands: &mut Commands,
    ) -> bool {
        match self {
            Menu::UnitMenu(menu) => menu.draw(ctx, window, commands),
        }
    }
}
