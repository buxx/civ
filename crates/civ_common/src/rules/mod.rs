use dyn_clone::DynClone;

use crate::game::{
    city::{CityProduct, CityProductionTons},
    unit::{TaskType, UnitType},
    GameFrame,
};

pub mod std1;

pub type RuleSetBox = Box<dyn RuleSet + Send + Sync>;

pub trait RuleSet: DynClone {
    fn tasks(&self) -> Vec<TaskType>;
    fn unit_can(&self, type_: &UnitType) -> Vec<TaskType>;
    fn settle_duration(&self, unit_type: &UnitType) -> GameFrame;
    fn can_settle(&self, unit: &UnitType) -> bool;
    fn required_tons(&self, product: &CityProduct) -> CityProductionTons;
}

dyn_clone::clone_trait_object!(RuleSet);
