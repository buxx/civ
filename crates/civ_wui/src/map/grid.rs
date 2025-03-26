use bevy::{prelude::*, utils::HashMap};
use hexx::{Hex, HexLayout};

use super::tile::HexTileMeta;

#[derive(Debug, Resource, Default)]
pub struct HexGrid {
    // TODO: Vec for perf (with xy position as index)
    pub entities: HashMap<Hex, HexTileMeta>,
    pub layout: HexLayout,
}

impl HexGrid {
    pub fn new(entities: HashMap<Hex, HexTileMeta>, layout: HexLayout) -> Self {
        Self { entities, layout }
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentCursorHex(pub Option<Hex>);
