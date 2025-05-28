use bevy::prelude::*;

use draw::DrawMenu;
use unit::UnitMenu;

pub mod draw;
pub mod unit;

pub const MENU_DISPLAY_FACTOR: f32 = 1.5;

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct MenuResource(pub Option<Menu>);

#[derive(Debug)]
pub enum Menu {
    UnitMenu(UnitMenu),
}

impl DrawMenu for Menu {
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
