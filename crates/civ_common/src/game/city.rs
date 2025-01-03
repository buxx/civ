use std::ops::{Add, AddAssign, SubAssign};

use super::unit::UnitType;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CityProduct {
    Unit(UnitType),
}
