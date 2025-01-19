use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        city::{CityExploitation, CityId, CityProduction},
        nation::flag::Flag,
        slice::ClientCity,
    },
    geo::Geo,
};

use common::geo::GeoContext;
use serde::{Deserialize, Serialize};

use crate::{state::State, task::city::CityTasks};

use super::IntoClientModel;

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

impl IntoClientModel<ClientCity> for City {
    fn into_client(self, _state: &RwLockReadGuard<State>) -> ClientCity {
        ClientCity::builder()
            .id(self.id)
            .geo(self.geo)
            .name(self.name.clone())
            .production(self.production.clone())
            .exploitation(self.exploitation.clone())
            .tasks(self.tasks.clone().into())
            .flag(self.flag)
            .build()
    }
}
