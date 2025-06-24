use bevy::prelude::*;
use bevy_egui::egui::Context;
use common::game::{city::CityId, slice::ClientCity, GameFrame};

use crate::{
    impl_ui_component_resource,
    ingame::{interact::WithCityId, DrawUiComponent, EGUI_DISPLAY_FACTOR},
    utils::gui::layout::fixed_window,
};

#[derive(Debug, Event)]
pub struct SetupCityMenu(pub CityId);

impl WithCityId for SetupCityMenu {
    fn city_id(&self) -> &CityId {
        &self.0
    }
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct CityMenuResource(pub Option<CityMenu>);
impl_ui_component_resource!(CityMenuResource, CityMenu, SetupCityMenu);

#[derive(Debug)]
pub struct CityMenu {
    pub city_id: CityId,
    pub city_name: String,
}

impl From<ClientCity> for CityMenu {
    fn from(city: ClientCity) -> Self {
        Self {
            city_id: *city.id(),
            city_name: city.name().to_string(),
        }
    }
}

impl DrawUiComponent for CityMenu {
    fn draw(
        &mut self,
        ctx: &Context,
        window: &Window,
        _commands: &mut Commands,
        _frame: GameFrame,
    ) -> bool {
        let close = false;

        fixed_window()
            .ctx(ctx)
            .window(window)
            .title(&self.city_name)
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("todo");
                });
            })
            .call();

        close
    }
}

// TODO: Derive on attribute
impl WithCityId for CityMenu {
    fn city_id(&self) -> &CityId {
        &self.city_id
    }
}
