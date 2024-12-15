use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::space::context::ClientPhysicalContext;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    physics: ClientPhysicalContext,
}

impl ClientCity {
    pub fn new(id: Uuid, physics: ClientPhysicalContext) -> Self {
        Self { id, physics }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientUnit {
    id: Uuid,
    physics: ClientPhysicalContext,
}

impl ClientUnit {
    pub fn new(id: Uuid, physics: ClientPhysicalContext) -> Self {
        Self { id, physics }
    }
}
