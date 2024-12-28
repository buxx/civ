use bon::Builder;
use serde::{Deserialize, Serialize};

pub mod partial;
pub mod reader;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TerrainType {
    GrassLand,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Tile {
    pub type_: TerrainType,
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    pub x: usize,
    pub y: usize,
    pub tiles: Vec<Tile>,
}

#[derive(Debug, Deserialize, Serialize, Builder)]
pub struct World {
    pub chunk_size: u64,
    pub width: u64,
    pub height: u64,
}
