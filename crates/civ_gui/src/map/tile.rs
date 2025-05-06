use bevy::prelude::*;

use common::geo::{ImaginaryWorldPoint, WorldPoint};
// use common::world::{CtxTile, Tile as BaseTile};
use derive_more::Constructor;

use super::AtlasIndex;

#[allow(unused)]
#[derive(Debug, Constructor)]
pub struct HexTileMeta<T> {
    entity: Entity,
    imaginary: ImaginaryWorldPoint,
    point: Option<WorldPoint>,
    // tile: Option<CtxTile<BaseTile>>,
    tile: Option<T>,
    atlas: AtlasIndex,
}

impl<T> HexTileMeta<T> {
    pub fn imaginary(&self) -> ImaginaryWorldPoint {
        self.imaginary
    }
}
