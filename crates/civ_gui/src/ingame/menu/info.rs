use bevy::prelude::*;
use bevy_egui::egui::Context;
use civ_derive::Geo;
use common::{
    game::{
        slice::{ClientCity, ClientUnit},
        GameFrame,
    },
    geo::{Geo, GeoContext},
    world::{CtxTile, Tile},
};
use thiserror::Error;

use crate::{
    impl_ui_component_resource,
    ingame::{DrawUiComponent, EGUI_DISPLAY_FACTOR},
    utils::gui::layout::fixed_window,
};

#[derive(Debug, Event)]
pub struct SetupTileInfoMenu(pub GeoContext);

impl Geo for SetupTileInfoMenu {
    fn geo(&self) -> &GeoContext {
        &self.0
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.0
    }
}

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct TileInfoMenuResource(pub Option<TileInfoMenu>);
impl_ui_component_resource!(TileInfoMenuResource, TileInfoMenu, SetupTileInfoMenu);

#[derive(Debug, Geo)]
pub struct TileInfoMenu {
    geo: GeoContext,
    _tile: Tile,
    _city: Option<ClientCity>,
    _units: Option<Vec<ClientUnit>>,
}

impl DrawUiComponent for TileInfoMenu {
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
            .title("TODO")
            .factor(EGUI_DISPLAY_FACTOR)
            .ui(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("TODO");
                });
            })
            .call();

        close
    }
}

impl TryFrom<(GeoContext, &CtxTile<Tile>)> for TileInfoMenu {
    type Error = TryTileInfoMenuFromCtxTileError;

    fn try_from(value: (GeoContext, &CtxTile<Tile>)) -> Result<Self, Self::Error> {
        if let CtxTile::Visible(tile) = value.1 {
            return Ok(Self {
                geo: value.0,
                _tile: tile.clone(),
                _city: None,
                _units: None,
            });
        }

        Err(TryTileInfoMenuFromCtxTileError::Outside)
    }
}

#[derive(Debug, Error)]
pub enum TryTileInfoMenuFromCtxTileError {
    #[error("Outside")]
    Outside,
}
