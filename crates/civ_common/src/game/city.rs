use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use super::unit::UnitType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CityId(pub Uuid);

impl CityId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Default for CityId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for CityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct CityProductionTons(pub u64);

impl Add<u64> for CityProductionTons {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign for CityProductionTons {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl SubAssign for CityProductionTons {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Sub for CityProductionTons {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CityExploitation {
    tons: CityProductionTons,
}

impl CityExploitation {
    pub fn new(tons: CityProductionTons) -> Self {
        Self { tons }
    }

    pub fn production_tons(&self) -> &CityProductionTons {
        &self.tons
    }
}
