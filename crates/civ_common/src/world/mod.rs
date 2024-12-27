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
