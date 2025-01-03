use crate::game::{
    city::{CityProduct, CityProductionTons},
    unit::{TaskType, UnitTaskType, UnitType},
    GameFrame, GAME_FRAMES_PER_SECOND,
};

use super::RuleSet;

#[derive(Clone)]
pub struct Std1RuleSet;

impl RuleSet for Std1RuleSet {
    fn tasks(&self) -> Vec<TaskType> {
        vec![TaskType::Unit(UnitTaskType::Settle)]
    }

    fn unit_can(&self, type_: &UnitType) -> Vec<TaskType> {
        match type_ {
            UnitType::Settlers => vec![TaskType::Unit(UnitTaskType::Settle)],
            UnitType::Warriors => vec![],
        }
    }

    fn settle_duration(&self, unit_type: &UnitType) -> GameFrame {
        GameFrame(match unit_type {
            UnitType::Settlers => GAME_FRAMES_PER_SECOND * 10,
            UnitType::Warriors => 0,
        })
    }

    fn can_settle(&self, unit_type: &UnitType) -> bool {
        match unit_type {
            UnitType::Settlers => true,
            UnitType::Warriors => false,
        }
    }

    fn required_tons(&self, product: &CityProduct) -> CityProductionTons {
        match product {
            CityProduct::Unit(unit_type) => match unit_type {
                UnitType::Settlers => CityProductionTons(40),
                UnitType::Warriors => CityProductionTons(8),
            },
        }
    }
}
