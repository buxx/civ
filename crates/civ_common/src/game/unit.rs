use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::city::CityProductionTons;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UnitId(pub Uuid);

impl UnitId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Default for UnitId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for UnitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl From<Uuid> for UnitId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

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
    System(SystemTaskType),
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SystemTaskType {
    Snapshot,
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Unit(UnitTaskType::Settle) => f.write_str("Settle"),
            TaskType::City(CityTaskType::Production(_)) => f.write_str("Production"),
            TaskType::Testing => f.write_str("Testing"),
            TaskType::System(SystemTaskType::Snapshot) => f.write_str("Snapshot"),
        }
    }
}

impl TaskType {
    pub fn is_city_production(&self) -> bool {
        matches!(self, TaskType::City(CityTaskType::Production(_)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnitCan {
    Settle,
}

impl UnitCan {
    pub fn name(&self) -> &str {
        match self {
            UnitCan::Settle => "Settle",
        }
    }
}
