pub mod std1;
use crate::game::{unit::UnitType, GameFrame};

pub trait RuleSet {
    fn settle_duration(&self, unit_type: &UnitType) -> GameFrame;
    fn can_settle(&self, unit: &UnitType) -> bool;
}
