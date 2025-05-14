use bevy::{prelude::*, utils::HashMap};
use common::geo::{ImaginaryWorldPoint, WorldPoint};
use derive_more::Constructor;
use hexx::{Hex, HexLayout};

#[derive(Debug, Resource, Constructor)]
pub struct HexGridResource<T: Send + Sync> {
    // TODO: Vec for perf (with xy position as index)
    pub grid: HashMap<Hex, GridTile<T>>,
    // FIXME BS NOW: in unique resource
    pub layout: HexLayout,
}

impl<T: Send + Sync> Default for HexGridResource<T> {
    fn default() -> Self {
        Self {
            grid: Default::default(),
            layout: Default::default(),
        }
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentCursorHex(pub Option<Hex>);

#[allow(unused)]
#[derive(Debug, Constructor)]
pub struct GridTile<T: Send + Sync> {
    pub entity: Entity,
    pub imaginary: ImaginaryWorldPoint,
    pub point: Option<WorldPoint>,
    pub item: T,
}

impl<T: Send + Sync> GridTile<T> {
    pub fn imaginary(&self) -> ImaginaryWorldPoint {
        self.imaginary
    }
}
