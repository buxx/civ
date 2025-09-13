use bevy::prelude::*;
use common::{
    game::slice::{ClientCity, ClientUnit},
    geo::{ImaginaryWorldPoint, WorldPoint},
    world::{CtxTile, Tile},
};
use derive_more::Constructor;
use rustc_hash::FxHashMap;

#[derive(Debug, Resource, Constructor, Default)]
pub struct GridResource(pub Option<Grid>);

#[derive(Debug, Constructor)]
pub struct Grid {
    pub grid: FxHashMap<WorldPoint, GridHex>,
    pub center: ImaginaryWorldPoint,
}

impl std::ops::Deref for Grid {
    type Target = FxHashMap<WorldPoint, GridHex>;

    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

#[derive(Debug, Constructor)]
pub struct GridHex {
    pub point: WorldPoint,
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
pub struct CurrentCursorHex(pub Option<ImaginaryWorldPoint>);
