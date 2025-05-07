use bevy::{prelude::*, utils::HashMap};
use derive_more::Constructor;
use hexx::{Hex, HexLayout};

use super::tile::HexMeta;

#[derive(Debug, Resource, Constructor)]
pub struct HexGridResource<T: Send + Sync> {
    // TODO: Vec for perf (with xy position as index)
    pub entities: HashMap<Hex, HexMeta<T>>,
    pub layout: HexLayout,
}

impl<T: Send + Sync> Default for HexGridResource<T> {
    fn default() -> Self {
        Self {
            entities: Default::default(),
            layout: Default::default(),
        }
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CurrentCursorHex(pub Option<Hex>);
