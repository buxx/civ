use bon::Builder;
use common::{
    game::{
        city::{CityProduct, CityProductionTons},
        slice::ClientCity,
        unit::UnitType,
    },
    geo::Geo,
};
use uuid::Uuid;

use common::geo::GeoContext;

use crate::runner::RunnerContext;

#[derive(Debug, Builder, Clone)]
pub struct City {
    id: Uuid,
    name: String,
    geo: GeoContext,
    production: CityProduction,
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
}

impl Geo for City {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

pub trait IntoClientCity {
    fn into_client(&self) -> ClientCity;
}

impl IntoClientCity for City {
    fn into_client(&self) -> ClientCity {
        ClientCity::new(self.id(), self.name().to_string(), self.geo().clone())
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
