use serde::{Deserialize, Serialize};

use crate::game::{city::City, unit::Unit};

pub mod reader;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldItem(common::world::item::WorldItem<City, Unit>);

impl std::ops::Deref for WorldItem {
    type Target = common::world::item::WorldItem<City, Unit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
