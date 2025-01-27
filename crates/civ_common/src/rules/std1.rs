use crate::{
    game::{
        city::{CityProduct, CityProductionTons},
        unit::{TaskType, UnitTaskType, UnitType},
        GameFrame, GAME_FRAMES_PER_SECOND,
    },
    world::{TerrainType, Tile},
};

use super::{RuleSet, RuleSetType};

#[derive(Clone)]
pub struct Std1RuleSet;

impl RuleSet for Std1RuleSet {
    fn type_(&self) -> RuleSetType {
        RuleSetType::Std1
    }

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

    fn can_be_startup(&self, tile: &Tile) -> bool {
        match tile.type_() {
            TerrainType::GrassLand | TerrainType::Plain => true,
        }
    }
}

impl From<Std1RuleSet> for RuleSetType {
    fn from(_: Std1RuleSet) -> Self {
        RuleSetType::Std1
    }
}
