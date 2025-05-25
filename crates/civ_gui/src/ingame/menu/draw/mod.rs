pub mod unit;
use bevy_egui::{egui::Context, EguiContextSettings, EguiContexts};

use super::{Menu, MenuResource, MENU_DISPLAY_FACTOR};

use bevy::prelude::*;

pub trait DrawMenu<T: Event> {
    fn draw(&mut self, ctx: &Context, window: &Window) -> Vec<T>;
}

pub fn draw_menu(
    mut commands: Commands,
    mut egui: Query<(&mut EguiContextSettings, &Window)>,
    mut menu: ResMut<MenuResource>,
    mut contexts: EguiContexts,
    windows: Query<&Window>,
) {
    let mut disband = false;

    if let Some(menu) = &mut menu.0 {
        if let Ok((mut egui_settings, _)) = egui.get_single_mut() {
            egui_settings.scale_factor = MENU_DISPLAY_FACTOR;
        }

        let window = windows.single();
        // TODO: impl something to avoid match usage here
        for event in match menu {
            Menu::UnitMenu(menu) => menu.draw(contexts.ctx_mut(), window),
        } {
            commands.trigger(event);
            disband = true;
        }
    }

    if disband {
        menu.0 = None;
    }
}
