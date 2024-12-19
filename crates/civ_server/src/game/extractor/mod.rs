use std::sync::MutexGuard;

use common::{
    game::slice::{ClientCity, ClientUnit, ClientUnitTask, ClientUnitTasks, GameSlice},
    geo::Geo,
    space::window::Window,
};
use uuid::Uuid;

use crate::{state::State, task::Task};

use super::{city::City, unit::Unit};

pub struct Extractor<'a> {
    state: &'a MutexGuard<'a, State>,
}

impl<'a> Extractor<'a> {
    pub fn new(state: &'a MutexGuard<'a, State>) -> Self {
        Self { state }
    }

    pub fn game_slice(&self, _client_id: &Uuid, window: &Window) -> GameSlice {
        let index = self.state.index();

        let cities: Vec<ClientCity> = index
            .xy_cities(window)
            .iter()
            .map(|uuid| {
                (
                    *uuid,
                    index
                        .uuid_cities()
                        .get(uuid)
                        .expect("Index must respect cities integrity"),
                )
            })
            .map(|(uuid, index)| self.state.city(*index, &uuid).unwrap())
            .map(|city| self.city_into_client(city))
            .collect::<Vec<ClientCity>>();
        let units: Vec<ClientUnit> = index
            .xy_units(window)
            .iter()
            .map(|uuid| {
                (
                    *uuid,
                    index
                        .uuid_units()
                        .get(uuid)
                        .expect("Index must respect units integrity"),
                )
            })
            .map(|(uuid, index)| self.state.unit(*index, &uuid).unwrap())
            .map(|unit| self.unit_into_client(unit))
            .collect::<Vec<ClientUnit>>();
        GameSlice::new(cities, units)
    }

    pub fn unit_into_client(&self, unit: &Unit) -> ClientUnit {
        let stack = unit
            .tasks()
            .stack()
            .iter()
            .filter_map(|(uuid, _)| {
                // FIXME: use task index by uuid to avoid performance bottleneck here; REF PERF_TASK
                self.state
                    .tasks()
                    .iter()
                    .find(|t| t.context().id() == *uuid)
                    .map(|task| self.task_into_client(task))
            })
            .collect();
        let tasks = ClientUnitTasks::new(stack);

        ClientUnit::builder()
            .id(unit.id())
            .type_(unit.type_().clone())
            .tasks(tasks)
            .geo(unit.geo().clone())
            .build()
    }

    pub fn city_into_client(&self, city: &City) -> ClientCity {
        ClientCity::new(city.id(), city.name().to_string(), city.geo().clone())
    }

    pub fn task_into_client(&self, task: &Box<dyn Task + Send>) -> ClientUnitTask {
        ClientUnitTask::new(
            task.context().id(),
            task.type_().clone(),
            task.context().start(),
            task.context().end(),
        )
    }
}
