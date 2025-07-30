use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::world::Tile;

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct WorldItem {
    pub tile: Tile,
    pub city: Option<City>,
    pub units: Vec<Unit>,
}
