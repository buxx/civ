use serde::{Deserialize, Serialize};

use crate::world::Tile;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldItem<C, U> {
    pub tile: Tile,
    pub city: Option<C>,
    pub units: Vec<U>,
}
