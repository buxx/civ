use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::game::unit::UnitType;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CityProduct {
    Unit(UnitType),
}

impl Display for CityProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CityProduct::Unit(unit_type) => f.write_str(&unit_type.to_string()),
        }
    }
}
