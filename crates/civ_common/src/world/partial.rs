use crate::geo::WorldPoint;
use serde::{Deserialize, Serialize};

use super::Tile;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PartialWorld {
    original: WorldPoint,
    width: u64,
    height: u64,
    tiles: Vec<Tile>,
}

impl PartialWorld {
    pub fn new(original: WorldPoint, width: u64, height: u64, tiles: Vec<Tile>) -> Self {
        Self {
            original,
            width,
            height,
            tiles,
        }
    }

    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }
}
