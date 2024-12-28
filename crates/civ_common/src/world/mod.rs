pub mod reader;
use bon::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TerrainType {
    GrassLand,
}

#[derive(Serialize, Deserialize)]
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
    pub chunk_size: usize,
    pub width: usize,
    pub height: usize,
}
