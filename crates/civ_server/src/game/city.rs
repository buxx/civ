use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        city::{CityProduct, CityProductionTons},
        slice::ClientCity,
        unit::{CityTaskType, UnitType},
        ClientTasks,
    },
    geo::Geo,
};
use uuid::Uuid;

use common::geo::GeoContext;

use crate::{
    runner::RunnerContext,
    state::State,
    task::{IntoClientConcreteTask, Tasks},
};

use super::IntoClientModel;

#[derive(Debug, Builder, Clone)]
pub struct City {
    id: Uuid,
    name: String,
    geo: GeoContext,
    production: CityProduction,
    #[builder(default = Tasks::empty())]
    tasks: Tasks<CityTaskType>,
}

impl City {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn production(&self) -> &CityProduction {
        &self.production
    }

    pub fn tasks(&self) -> &Tasks<CityTaskType> {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut Tasks<CityTaskType> {
        &mut self.tasks
    }
}

impl Geo for City {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

impl IntoClientModel<ClientCity> for City {
    fn into_client(self, state: &RwLockReadGuard<State>) -> ClientCity {
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
        let tasks = ClientTasks::new(stack);

        ClientCity::builder()
            .id(self.id())
            .tasks(tasks)
            .geo(*self.geo())
            .name(self.name.clone())
            .build()
    }
}

#[derive(Debug, Clone)]
pub struct CityProduction {
    stack: Vec<CityProduct>,
    tons: CityProductionTons,
}

impl CityProduction {
    pub fn new(stack: Vec<CityProduct>, tons: CityProductionTons) -> Self {
        Self { stack, tons }
    }

    pub fn default(_context: &RunnerContext) -> Self {
        // Default according to context (warrior, then phalanx, etc) and tons
        Self {
            stack: vec![CityProduct::Unit(UnitType::Settlers)],
            tons: CityProductionTons(1),
        }
    }

    pub fn current(&self) -> &CityProduct {
        self.stack.first().expect("One item is mandatory")
    }

    pub fn tons(&self) -> &CityProductionTons {
        &self.tons
    }
}

// pub struct CityTasks<'a> {
//     pub production: &'a TaskBox,
// }

// impl<'a> CityTasks<'a> {
//     pub fn from(tasks: Vec<&'a TaskBox>) -> Result<Self, CityIntegrityError> {
//         let mut production: Option<&TaskBox> = None;

//         for task in tasks {
//             match task.type_() {
//                 TaskType::Unit(_) => {}
//                 TaskType::City(type_) => match type_ {
//                     CityTaskType::Production => production = Some(task),
//                 },
//             }
//         }

//         let production = production.ok_or(CityIntegrityError::NoProductionTaskFound)?;
//         Ok(Self { production })
//     }
// }

// #[derive(Error, Debug)]
// pub enum CityIntegrityError {
//     #[error("No production task found")]
//     NoProductionTaskFound,
// }
