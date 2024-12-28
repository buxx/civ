use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{geo::GeoContext, world::partial::PartialWorld};

use super::{
    unit::{TaskType, UnitType},
    GameFrame,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameSlice {
    world: PartialWorld,
    cities: Vec<ClientCity>,
    units: Vec<ClientUnit>,
}

impl GameSlice {
    pub fn new(world: PartialWorld, cities: Vec<ClientCity>, units: Vec<ClientUnit>) -> Self {
        Self {
            world,
            cities,
            units,
        }
    }

    pub fn world(&self) -> &PartialWorld {
        &self.world
    }

    pub fn cities(&self) -> &[ClientCity] {
        &self.cities
    }

    pub fn units(&self) -> &[ClientUnit] {
        &self.units
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Builder, Debug, PartialEq)]
pub struct ClientUnit {
    id: Uuid,
    type_: UnitType,
    tasks: ClientUnitTasks,
    geo: GeoContext,
}

impl ClientUnit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn geo(&self) -> &GeoContext {
        &self.geo
    }

    pub fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientUnitTasks {
    stack: Vec<ClientTask>,
}

impl ClientUnitTasks {
    pub fn new(stack: Vec<ClientTask>) -> Self {
        Self { stack }
    }

    pub fn push(&mut self, task: ClientTask) {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientTask {
    id: Uuid,
    task: TaskType,
    start: GameFrame,
    end: GameFrame,
}

impl ClientTask {
    pub fn new(id: Uuid, task: TaskType, start: GameFrame, end: GameFrame) -> Self {
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
