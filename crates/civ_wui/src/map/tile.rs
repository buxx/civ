use bevy::prelude::*;

use common::geo::{ImaginaryWorldPoint, WorldPoint};
use common::world::{CtxTile, Tile as BaseTile};

use super::AtlasIndex;

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

    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn tile(&self) -> &Option<CtxTile<BaseTile>> {
        &self.tile
    }

    pub fn atlas(&self) -> &AtlasIndex {
        &self.atlas
    }

    pub fn point(&self) -> Option<WorldPoint> {
        self.point
    }

    pub fn imaginary(&self) -> ImaginaryWorldPoint {
        self.imaginary
    }
}
