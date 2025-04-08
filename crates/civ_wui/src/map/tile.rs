use bevy::prelude::*;

use common::geo::{ImaginaryWorldPoint, WorldPoint};
use common::world::{CtxTile, Tile as BaseTile};

use super::AtlasIndex;

#[allow(unused)]
#[derive(Debug)]
pub struct HexTileMeta {
    entity: Entity,
    imaginary: ImaginaryWorldPoint,
    point: Option<WorldPoint>,
    tile: Option<CtxTile<BaseTile>>,
    atlas: AtlasIndex,
}

impl HexTileMeta {
    pub fn new(
        entity: Entity,
        imaginary: ImaginaryWorldPoint,
        point: Option<WorldPoint>,
        tile: Option<CtxTile<BaseTile>>,
        atlas: AtlasIndex,
    ) -> Self {
        Self {
            entity,
            imaginary,
            point,
            tile,
            atlas,
        }
    }

    pub fn imaginary(&self) -> ImaginaryWorldPoint {
        self.imaginary
    }
}
