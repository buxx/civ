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
