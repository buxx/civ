use bon::Builder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{geo::GeoContext, world::partial::PartialWorld};

use super::{
    city::{CityExploitation, CityProduction},
    tasks::client::{city::production::ClientCityProductionTask, ClientTask},
    unit::UnitType,
    GameFrame,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GameSlice {
    world: PartialWorld,
    cities: Vec<ClientCity>,
    units: Vec<ClientUnit>,
}

impl GameSlice {
    pub fn new(world: PartialWorld, cities: Vec<ClientCity>, units: Vec<ClientUnit>) -> Self {
        Self {
            world,
            cities,
            units,
        }
    }

    pub fn world(&self) -> &PartialWorld {
        &self.world
    }

    pub fn cities(&self) -> &[ClientCity] {
        &self.cities
    }

    pub fn units(&self) -> &[ClientUnit] {
        &self.units
    }
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientCity {
    id: Uuid,
    name: String,
    geo: GeoContext,
    production: CityProduction,
    exploitation: CityExploitation,
    tasks: ClientCityTasks,
}

impl ClientCity {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn geo(&self) -> &GeoContext {
        &self.geo
    }

    pub fn production_str(&self, frame: &GameFrame) -> String {
        format!(
            "{} ({}%)",
            self.production.current(),
            self.tasks.production.progress(frame)
        )
    }
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientCityTasks {
    production: ClientCityProductionTask,
}

impl ClientCityTasks {
    pub fn new(production: ClientCityProductionTask) -> Self {
        Self { production }
    }
}

#[derive(Serialize, Deserialize, Clone, Builder, Debug, PartialEq)]
pub struct ClientUnit {
    id: Uuid,
    type_: UnitType,
    geo: GeoContext,
    task: Option<ClientTask>,
}

impl ClientUnit {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn geo(&self) -> &GeoContext {
        &self.geo
    }

    pub fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }

    pub fn type_(&self) -> &UnitType {
        &self.type_
    }

    pub fn task(&self) -> &Option<ClientTask> {
        &self.task
    }
}
