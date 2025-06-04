use bevy::prelude::*;

use bevy_egui::{EguiContextSettings, EguiContexts};
use unit::UnitMenu;

use crate::ingame::{DrawUiComponent, EGUI_DISPLAY_FACTOR};

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
            egui_settings.scale_factor = EGUI_DISPLAY_FACTOR;
        }

        let window = windows.single();
        disband = menu.draw(contexts.ctx_mut(), window, &mut commands);
    }

    if disband {
        menu.0 = None;
    }
}
