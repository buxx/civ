use dyn_clone::DynClone;

use crate::game::{unit::UnitType, GameFrame};

pub mod std1;

pub trait RuleSet: DynClone {
    fn settle_duration(&self, unit_type: &UnitType) -> GameFrame;
    fn can_settle(&self, unit: &UnitType) -> bool;
}

dyn_clone::clone_trait_object!(RuleSet);
