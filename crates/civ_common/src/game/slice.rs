use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::geo::GeoContext;

use super::{
    unit::{UnitTask, UnitType},
    GameFrame,
};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientCity {
    id: Uuid,
    name: String,
    geo: GeoContext,
}

impl ClientCity {
    pub fn new(id: Uuid, name: String, physics: GeoContext) -> Self {
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

    pub fn geo(&self) -> &GeoContext {
        &self.geo
    }
}

#[derive(Serialize, Deserialize, Clone, Builder)]
pub struct ClientUnit {
    id: Uuid,
    type_: UnitType,
    tasks: ClientUnitTasks,
    physics: GeoContext,
}

impl ClientUnit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn physics(&self) -> &GeoContext {
        &self.physics
    }

    pub fn physics_mut(&mut self) -> &mut GeoContext {
        &mut self.physics
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn tasks(&self) -> &ClientUnitTasks {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut ClientUnitTasks {
        &mut self.tasks
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientUnitTasks {
    stack: Vec<ClientUnitTask>,
}

impl ClientUnitTasks {
    pub fn new(stack: Vec<ClientUnitTask>) -> Self {
        Self { stack }
    }

    pub fn push(&mut self, task: ClientUnitTask) {
        self.stack.push(task);
    }

    pub fn remove(&mut self, uuid: Uuid) {
        self.stack.retain(|t| t.id() != uuid);
    }

    pub fn display(&self, frame: &GameFrame) -> String {
        if self.stack.is_empty() {
            return "Idle".into();
        }

        self.stack
            .iter()
            .map(|t| format!("{} ({}%)", t.task, (t.progress(frame) * 100.0) as u8))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClientUnitTask {
    id: Uuid,
    task: UnitTask,
    start: GameFrame,
    end: GameFrame,
}

impl ClientUnitTask {
    pub fn new(id: Uuid, task: UnitTask, start: GameFrame, end: GameFrame) -> Self {
        Self {
            id,
            task,
            start,
            end,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn progress(&self, frame: &GameFrame) -> f32 {
        let total = self.end.0 - self.start.0;
        let current = frame.0 - self.start.0;
        current as f32 / total as f32
    }
}
