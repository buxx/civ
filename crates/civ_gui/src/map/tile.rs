use bevy::prelude::*;

use common::geo::{ImaginaryWorldPoint, WorldPoint};
use derive_more::Constructor;

use super::AtlasIndex;

#[allow(unused)]
#[derive(Debug, Constructor)]
pub struct HexMeta<T: Send + Sync> {
    pub entity: Entity,
    pub imaginary: ImaginaryWorldPoint,
    pub point: Option<WorldPoint>,
    pub tile: T, // TODO: rename
    pub atlas: AtlasIndex,
}

impl<T: Send + Sync> HexMeta<T> {
    pub fn imaginary(&self) -> ImaginaryWorldPoint {
        self.imaginary
    }
}
