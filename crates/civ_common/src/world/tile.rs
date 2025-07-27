use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::world::Tile;

#[derive(Serialize, Deserialize)]
pub struct Chunk<T> {
    pub x: u64,
    pub y: u64,
    pub item: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, Builder)]
pub struct World {
    pub chunk_size: u64,
    pub width: u64,
    pub height: u64,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtxTile<T> {
    Outside,
    Visible(T),
}

impl From<CtxTile<&Tile>> for CtxTile<Tile> {
    fn from(value: CtxTile<&Tile>) -> Self {
        match value {
            CtxTile::Outside => CtxTile::Outside,
            CtxTile::Visible(tile) => CtxTile::Visible(tile.clone()),
        }
    }
}
