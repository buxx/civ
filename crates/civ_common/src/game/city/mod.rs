use std::fmt::Display;

use crate::{
    game::{
        city::{
            production::{tons::CityProductionTons, CityProduction},
            task::CityTasks,
        },
        nation::flag::Flag,
    },
    geo::{Geo, GeoContext},
};

use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod production;
pub mod task;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CityId(pub Uuid);

impl CityId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Default for CityId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for CityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct City {
    id: CityId,
    flag: Flag,
    name: String,
    geo: GeoContext,
    production: CityProduction,
    exploitation: CityExploitation,
    tasks: CityTasks,
}

impl City {
    pub fn id(&self) -> &CityId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
    }

    pub fn production(&self) -> &CityProduction {
        &self.production
    }

    pub fn tasks(&self) -> &CityTasks {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut CityTasks {
        &mut self.tasks
    }

    pub fn exploitation(&self) -> &CityExploitation {
        &self.exploitation
    }

    pub fn exploitation_mut(&mut self) -> &mut CityExploitation {
        &mut self.exploitation
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CityExploitation {
    tons: CityProductionTons,
}

impl CityExploitation {
    pub fn new(tons: CityProductionTons) -> Self {
        Self { tons }
    }

    pub fn production_tons(&self) -> &CityProductionTons {
        &self.tons
    }
}
