use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};

use crate::{
    game::{
        city::{CityProduct, CityProductionTons},
        unit::{TaskType, UnitType},
        GameFrame,
    },
    world::Tile,
};

pub mod std1;

pub type RuleSetBox = Box<dyn RuleSet + Send + Sync>;

pub trait RuleSet: DynClone {
    fn type_(&self) -> RuleSetType;
    fn tasks(&self) -> Vec<TaskType>;
    fn unit_can(&self, type_: &UnitType) -> Vec<TaskType>;
    fn settle_duration(&self, unit_type: &UnitType) -> GameFrame;
    fn can_settle(&self, unit: &UnitType) -> bool;
    fn required_tons(&self, product: &CityProduct) -> CityProductionTons;
    fn can_be_startup(&self, tile: &Tile) -> bool;
}

dyn_clone::clone_trait_object!(RuleSet);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum RuleSetType {
    Testing,
    Std1,
}

impl From<RuleSetBox> for RuleSetType {
    fn from(value: RuleSetBox) -> Self {
        value.type_()
    }
}
