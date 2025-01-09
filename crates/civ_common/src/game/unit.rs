use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::city::CityProductionTons;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum UnitType {
    Warriors,
    Settlers,
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitType::Warriors => f.write_str("Warriors"),
            UnitType::Settlers => f.write_str("Settlers"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskType {
    City(CityTaskType),
    Unit(UnitTaskType),
    Testing,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnitTaskType {
    Settle,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CityTaskType {
    Production(CityProductionTons),
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Unit(UnitTaskType::Settle) => f.write_str("Settle"),
            TaskType::City(CityTaskType::Production(_)) => f.write_str("Production"),
            TaskType::Testing => f.write_str("Testing"),
        }
    }
}

impl TaskType {
    pub fn is_city_production(&self) -> bool {
        matches!(self, TaskType::City(CityTaskType::Production(_)))
    }
}
