use bevy::{prelude::*, utils::HashMap};
use common::{
    game::slice::{ClientCity, ClientUnit},
    geo::{ImaginaryWorldPoint, WorldPoint},
    world::{CtxTile, Tile},
};
use derive_more::Constructor;
use hexx::{Hex, HexLayout};

#[derive(Debug, Resource, Constructor, Default)]
pub struct GridResource {
    pub grid: HashMap<Hex, GridHex>,
    pub center: ImaginaryWorldPoint,
    pub layout: HexLayout,
}

impl std::ops::Deref for GridResource {
    type Target = HashMap<Hex, GridHex>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

#[derive(Debug, Constructor)]
pub struct GridHex {
    pub imaginary: ImaginaryWorldPoint,
    pub point: WorldPoint,
    pub tile: GridHexResource<CtxTile<Tile>>,
    pub city: Option<GridHexResource<ClientCity>>,
    pub units: Option<GridHexResource<Vec<ClientUnit>>>,
}

#[derive(Debug, Constructor)]
pub struct GridHexResource<T: Send + Sync> {
    pub entity: Entity,
    pub item: T,
}

impl<T: Send + Sync> std::ops::Deref for GridHexResource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentCursorHex(pub Option<Hex>);
