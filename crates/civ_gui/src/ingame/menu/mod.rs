use bevy::prelude::*;

use bevy_egui::{EguiContextSettings, EguiContexts};
use common::game::unit::UnitId;
use unit::UnitMenu;

use crate::{
    core::GameSlicePropagated,
    ingame::{DrawUiComponent, GameSliceResource, EGUI_DISPLAY_FACTOR},
};

pub mod unit;

#[derive(Debug, Event)]
pub enum SetupMenu {
    Unit(UnitId),
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct MenuResource(pub Option<Menu>);

#[derive(Debug)]
pub enum Menu {
    UnitMenu(UnitMenu),
}

pub fn on_setup_menu(
    trigger: Trigger<SetupMenu>,
    slice: Res<GameSliceResource>,
    mut menu: ResMut<MenuResource>,
) {
    if let Some(slice) = &slice.0 {
        match trigger.event() {
            SetupMenu::Unit(unit_id) => {
                if let Some(unit) = slice.unit(unit_id) {
                    menu.0 = Some(Menu::UnitMenu(UnitMenu::from_unit(unit)));
                }
            }
        }
    }
}

pub fn menu_on_slice_propagated(
    _trigger: Trigger<GameSlicePropagated>,
    slice: Res<GameSliceResource>,
    mut resource: ResMut<MenuResource>,
) {
    if let (Some(slice), Some(menu)) = (&slice.0, &resource.0) {
        match menu {
            Menu::UnitMenu(unit_menu) => {
                if let Some(unit) = slice.unit(&unit_menu.unit_id) {
                    resource.0 = Some(Menu::UnitMenu(UnitMenu::from_unit(unit)));
                } else {
                    resource.0 = None;
                }
            }
        }
    }
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
