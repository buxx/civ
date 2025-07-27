use bon::Builder;
use serde::{Deserialize, Serialize};

pub mod slice;
pub mod tile;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerrainType {
    GrassLand,
    Plain,
}

pub trait TileDetail {
    fn type_(&self) -> TerrainType;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    type_: TerrainType,
}

impl Tile {
    pub fn new(type_: TerrainType) -> Self {
        Self { type_ }
    }

    pub fn type_(&self) -> TerrainType {
        self.type_
    }
}

impl TileDetail for Tile {
    fn type_(&self) -> TerrainType {
        self.type_
    }
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub x: u64,
    pub y: u64,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Deserialize, Serialize, Builder)]
pub struct World {
    pub chunk_size: u64,
    pub width: u64,
    pub height: u64,
}

// FIXME BS NOW: not necessary anymore ? (Because Option<WorldTile> now)
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

impl<'a> From<Option<&'a Tile>> for CtxTile<Tile> {
    fn from(value: Option<&'a Tile>) -> Self {
        match value {
            Some(tile) => CtxTile::Visible(tile.clone()),
            None => CtxTile::Outside,
        }
    }
}
