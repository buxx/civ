use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{geo::GeoContext, world::partial::PartialWorld};

use super::{
    unit::{TaskType, UnitType},
    ClientTask, ClientTasks, GameFrame,
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

#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientCity {
    id: Uuid,
    name: String,
    tasks: ClientTasks<ClientConcreteTask>,
    geo: GeoContext,
}

impl ClientCity {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn geo(&self) -> &GeoContext {
        &self.geo
    }

    pub fn tasks(&self) -> &ClientTasks<ClientConcreteTask> {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut ClientTasks<ClientConcreteTask> {
        &mut self.tasks
    }
}

#[derive(Serialize, Deserialize, Clone, Builder, Debug, PartialEq)]
pub struct ClientUnit {
    id: Uuid,
    type_: UnitType,
    tasks: ClientTasks<ClientConcreteTask>,
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

    pub fn tasks(&self) -> &ClientTasks<ClientConcreteTask> {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut ClientTasks<ClientConcreteTask> {
        &mut self.tasks
    }
}

// FIXME BS NOW: revoir archi pour task client unit/city
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientConcreteTask {
    id: Uuid,
    task: TaskType,
    start: GameFrame,
    end: GameFrame,
}

impl ClientConcreteTask {
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

impl ClientTask for ClientConcreteTask {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn display(&self, frame: &GameFrame) -> String {
        format!("{} ({}%)", self.task, (self.progress(frame) * 100.0) as u8)
    }
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct ClientCityTask {
//     id: Uuid,
//     task: TaskType,
//     start: GameFrame,
//     end: GameFrame,
// }

// impl ClientCityTask {
//     pub fn new(id: Uuid, task: TaskType, start: GameFrame, end: GameFrame) -> Self {
//         Self {
//             id,
//             task,
//             start,
//             end,
//         }
//     }

//     pub fn id(&self) -> Uuid {
//         self.id
//     }

//     pub fn progress(&self, frame: &GameFrame) -> f32 {
//         let total = self.end.0 - self.start.0;
//         let current = frame.0 - self.start.0;
//         current as f32 / total as f32
//     }
// }

// impl ClientTask for ClientCityTask {
//     fn id(&self) -> &Uuid {
//         &self.id
//     }

//     fn display(&self, frame: &GameFrame) -> String {
//         format!("{} ({}%)", self.task, (self.progress(frame) * 100.0) as u8)
//     }
// }
