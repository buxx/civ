use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UnitType {
    Settlers,
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitType::Settlers => f.write_str("Settlers"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskType {
    Unit(UnitTaskType),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UnitTaskType {
    Settle,
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskType::Unit(UnitTaskType::Settle) => f.write_str("Settle"),
        }
    }
}
