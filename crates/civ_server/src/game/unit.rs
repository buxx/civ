use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        slice::{ClientConcreteTask, ClientUnit},
        unit::{UnitTaskType, UnitType},
        ClientTasks,
    },
    geo::Geo,
};
use uuid::Uuid;

use common::geo::GeoContext;

use crate::{
    state::State,
    task::{IntoClientConcreteTask, Tasks},
};

use super::IntoClientModel;

#[derive(Debug, Builder, Clone)]
pub struct Unit {
    id: Uuid,
    type_: UnitType,
    #[builder(default = Tasks::empty())]
    tasks: Tasks<UnitTaskType>,
    geo: GeoContext,
}

impl Unit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn tasks(&self) -> &Tasks<UnitTaskType> {
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

impl IntoClientModel<ClientUnit> for Unit {
    fn into_client(self, state: &RwLockReadGuard<State>) -> ClientUnit {
        let stack: Vec<ClientConcreteTask> = self
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
        let tasks = ClientTasks::new(stack);

        ClientUnit::builder()
            .id(self.id())
            .type_(self.type_().clone())
            .tasks(tasks)
            .geo(self.geo().clone())
            .build()
    }
}
