use std::sync::MutexGuard;

use bon::Builder;
use common::{
    game::{
        slice::{ClientUnit, ClientUnitTasks},
        unit::{TaskType, UnitType},
    },
    geo::Geo,
};
use uuid::Uuid;

use common::geo::GeoContext;

use crate::{state::State, task::IntoClientTask};

#[derive(Debug, Builder, Clone)]
pub struct Unit {
    id: Uuid,
    type_: UnitType,
    #[builder(default)]
    tasks: UnitTasks,
    geo: GeoContext,
}

impl Unit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn tasks(&self) -> &UnitTasks {
        &self.tasks
    }
}

impl Geo for Unit {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

#[derive(Debug, Default, Clone)]
pub struct UnitTasks {
    stack: Vec<(Uuid, TaskType)>,
}

impl UnitTasks {
    pub fn stack(&self) -> &[(Uuid, TaskType)] {
        &self.stack
    }
}

pub trait IntoClientUnit {
    fn into_client(&self, state: &MutexGuard<State>) -> ClientUnit;
}

impl IntoClientUnit for Unit {
    fn into_client(&self, state: &MutexGuard<State>) -> ClientUnit {
        let stack = self
            .tasks()
            .stack()
            .iter()
            .filter_map(|(uuid, _)| {
                // FIXME: use task index by uuid to avoid performance bottleneck here; REF PERF_TASK
                state
                    .tasks()
                    .iter()
                    .find(|t| t.context().id() == *uuid)
                    .map(|task| task.into_client())
            })
            .collect();
        let tasks = ClientUnitTasks::new(stack);

        ClientUnit::builder()
            .id(self.id())
            .type_(self.type_().clone())
            .tasks(tasks)
            .geo(self.geo().clone())
            .build()
    }
}
