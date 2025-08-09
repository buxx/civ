use bon::Builder;
use civ_derive::Geo;
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

#[derive(Debug, Builder, Clone, Serialize, Deserialize, Geo)]
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

impl IntoClientModel<ClientCity> for City {
    fn into_client(self, _state: &State) -> ClientCity {
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
