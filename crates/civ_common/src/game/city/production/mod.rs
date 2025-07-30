use serde::{Deserialize, Serialize};

use crate::game::city::production::product::CityProduct;

pub mod product;
pub mod task;
pub mod tons;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CityProduction {
    stack: Vec<CityProduct>,
}

impl CityProduction {
    pub fn new(stack: Vec<CityProduct>) -> Self {
        Self { stack }
    }

    pub fn current(&self) -> &CityProduct {
        self.stack.first().expect("One item is mandatory")
    }
}
