use bon::Builder;
use serde::{Deserialize, Serialize};

pub mod partial;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TerrainType {
    GrassLand,
    Plain,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tile {
    pub type_: TerrainType,
}

impl Tile {
    pub fn new(type_: TerrainType) -> Self {
        Self { type_ }
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
