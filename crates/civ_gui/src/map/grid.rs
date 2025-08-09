use bevy::{prelude::*, utils::HashMap};
use common::{
    game::slice::{ClientCity, ClientUnit},
    geo::{ImaginaryWorldPoint, WorldPoint},
    world::{CtxTile, Tile},
};
use derive_more::Constructor;
use hexx::{Hex, HexLayout};

#[derive(Debug, Resource, Constructor, Default)]
pub struct GridResource(pub Option<Grid>);

#[derive(Debug, Constructor)]
pub struct Grid {
    pub grid: HashMap<Hex, GridHex>,
    pub center: ImaginaryWorldPoint,
    pub relative_layout: HexLayout,
    pub absolute_layout: HexLayout,
}

impl std::ops::Deref for Grid {
    type Target = HashMap<Hex, GridHex>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

#[derive(Debug, Constructor)]
pub struct GridHex {
    pub _imaginary: ImaginaryWorldPoint,
    pub _point: WorldPoint,
    #[allow(unused)] // Only used in that feature for now
    pub tile: GridHexResource<CtxTile<Tile>>,
    pub city: Option<GridHexResource<ClientCity>>,
    pub units: Option<GridHexResource<Vec<ClientUnit>>>,
}

#[derive(Debug, Constructor)]
pub struct GridHexResource<T: Send + Sync> {
    #[allow(unused)] // Only used in that feature for now
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
