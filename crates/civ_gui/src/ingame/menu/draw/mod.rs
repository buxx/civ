pub mod unit;
use bevy::prelude::*;
use bevy_egui::{EguiContextSettings, EguiContexts};

use super::{unit::UnitMenu, Menu, MenuResource};

pub trait DrawMenu {
    fn draw(&mut self, contexts: EguiContexts, window: &Window);
}

pub fn draw_menu(
    commands: Commands,
    egui: Query<(&mut EguiContextSettings, &Window)>,
    mut menu: ResMut<MenuResource>,
    mut contexts: EguiContexts,
    windows: Query<&Window>,
) {
    if let Some(menu) = &mut menu.0 {
        let window = windows.single();
        // TODO: impl something to avoid match usage here
        match menu {
            Menu::UnitMenu(menu) => {
                menu.draw(contexts, window);
            }
        }
    }
}
