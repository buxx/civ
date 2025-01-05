use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        city::{CityExploitation, CityProduction},
        slice::ClientCity,
    },
    geo::Geo,
};
use uuid::Uuid;

use common::geo::GeoContext;

use crate::{state::State, task::city::CityTasks};

use super::IntoClientModel;

#[derive(Debug, Builder, Clone)]
pub struct City {
    id: Uuid,
    name: String,
    geo: GeoContext,
    production: CityProduction,
    exploitation: CityExploitation,
    tasks: CityTasks,
}

impl City {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
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
            .id(*self.id())
            .geo(*self.geo())
            .name(self.name.clone())
            .production(self.production.clone())
            .exploitation(self.exploitation.clone())
            .tasks(self.tasks.clone().into())
            .build()
    }
}
