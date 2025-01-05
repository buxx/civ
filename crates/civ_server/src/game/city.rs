use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        city::{CityProduct, CityProductionTons},
        slice::{ClientCity, ClientConcreteTask},
        unit::{CityTaskType, TaskType, UnitType},
    },
    geo::Geo,
};
use uuid::Uuid;

use common::geo::GeoContext;

use crate::{
    runner::RunnerContext,
    state::State,
    task::{Concern, IntoClientConcreteTask, Tasks},
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
        // FIXME BS NOW: ce bout de code montre qu'il y a un problème de rattachement entre les tâches et les city/unit
        // trouver autre chose ...
        let x = state
            .tasks()
            .iter()
            .filter_map(|t| match (t.type_(), t.concern()) {
                (TaskType::City(CityTaskType::Production(_)), Concern::City(city_id)) => {
                    if city_id == self.id() {
                        Some(t.into_client())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<ClientConcreteTask>>();
        // FIXME BS NOW: fuck, les state.tasks pas encore mises à jour ?
        let production_task = x.first().unwrap();
        let product = self.production.current();

        ClientCity::builder()
            .id(self.id())
            .geo(*self.geo())
            .name(self.name.clone())
            .production((product.clone(), production_task.clone()))
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
            stack: vec![CityProduct::Unit(UnitType::Warriors)],
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
