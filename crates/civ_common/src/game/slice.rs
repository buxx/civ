use std::fmt::Display;

use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::space::context::ClientGeoContext;

use super::unit::{UnitTask, UnitType};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSlice {
    cities: Vec<ClientCity>,
    units: Vec<ClientUnit>,
}

impl GameSlice {
    pub fn new(cities: Vec<ClientCity>, units: Vec<ClientUnit>) -> Self {
        Self { cities, units }
    }

    pub fn cities(&self) -> &[ClientCity] {
        &self.cities
    }

    pub fn units(&self) -> &[ClientUnit] {
        &self.units
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientCity {
    id: Uuid,
    name: String,
    geo: ClientGeoContext,
}

impl ClientCity {
    pub fn new(id: Uuid, name: String, physics: ClientGeoContext) -> Self {
        Self {
            id,
            name,
            geo: physics,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn geo(&self) -> &ClientGeoContext {
        &self.geo
    }
}

#[derive(Serialize, Deserialize, Clone, Builder)]
pub struct ClientUnit {
    id: Uuid,
    type_: UnitType,
    tasks: ClientUnitTasks,
    physics: ClientGeoContext,
}

impl ClientUnit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn physics(&self) -> &ClientGeoContext {
        &self.physics
    }

    pub fn physics_mut(&mut self) -> &mut ClientGeoContext {
        &mut self.physics
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn tasks(&self) -> &ClientUnitTasks {
        &self.tasks
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientUnitTasks {
    stack: Vec<(Uuid, UnitTask)>,
}

impl ClientUnitTasks {
    pub fn new(stack: Vec<(Uuid, UnitTask)>) -> Self {
        Self { stack }
    }
}

impl Display for ClientUnitTasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.stack.is_empty() {
            return f.write_str("Idle");
        }

        let sentence = self
            .stack
            .iter()
            .map(|(_, task)| format!("{} (?%)", task))
            .collect::<Vec<String>>()
            .join(", ");
        f.write_str(&sentence)
    }
}
