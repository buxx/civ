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
    name: String,
    physics: ClientPhysicalContext,
}

impl ClientCity {
    pub fn new(id: Uuid, name: String, physics: ClientPhysicalContext) -> Self {
        Self { id, name, physics }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn physics(&self) -> &ClientPhysicalContext {
        &self.physics
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn physics(&self) -> &ClientPhysicalContext {
        &self.physics
    }
}
