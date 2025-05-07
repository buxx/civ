use bevy::prelude::*;

use common::geo::{ImaginaryWorldPoint, WorldPoint};
use derive_more::Constructor;

use super::AtlasIndex;

#[allow(unused)]
#[derive(Debug, Constructor)]
pub struct HexMeta<T: Send + Sync> {
    entity: Entity,
    imaginary: ImaginaryWorldPoint,
    point: Option<WorldPoint>,
    tile: T,
    atlas: AtlasIndex,
}

impl<T: Send + Sync> HexMeta<T> {
    pub fn imaginary(&self) -> ImaginaryWorldPoint {
        self.imaginary
    }
}
