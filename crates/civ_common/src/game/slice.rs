use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    geo::{GeoContext, ImaginaryWorldPoint, WorldPoint},
    world::partial::PartialWorld,
};

use super::{
    city::{CityExploitation, CityId, CityProduction},
    nation::flag::Flag,
    tasks::client::{city::production::ClientCityProductionTask, ClientTask},
    unit::{UnitCan, UnitId, UnitType},
    GameFrame,
};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
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

    pub fn center(&self) -> ImaginaryWorldPoint {
        self.world().imaginary_world_point_for_center_rel((0, 0))
    }

    pub fn cities(&self) -> &[ClientCity] {
        &self.cities
    }

    pub fn cities_mut(&mut self) -> &mut Vec<ClientCity> {
        &mut self.cities
    }

    pub fn units(&self) -> &[ClientUnit] {
        &self.units
    }

    pub fn units_mut(&mut self) -> &mut Vec<ClientUnit> {
        &mut self.units
    }

    // FIXME: cities by index like tiles
    pub fn city_at(&self, point: &WorldPoint) -> Option<&ClientCity> {
        self.cities.iter().find(|c| c.geo().point() == point)
    }

    // FIXME: cities by index like tiles
    pub fn units_at(&self, point: &WorldPoint) -> Option<Vec<&ClientUnit>> {
        let units: Vec<&ClientUnit> = self
            .units
            .iter()
            .filter(|c| c.geo().point() == point)
            .collect();

        if units.is_empty() {
            return None;
        }

        Some(units)
    }

    pub fn unit(&self, id: &UnitId) -> Option<&ClientUnit> {
        self.units.iter().find(|u| u.id() == id)
    }
}

impl std::fmt::Debug for GameSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GameSlice")
            .field(
                "world",
                &format!("{} world tiles", self.world.tiles().len()),
            )
            .field("cities", &format!("{} cities", self.cities.len()))
            .field("units", &format!("{} units", self.units.len()))
            .finish()
    }
}

#[derive(Builder, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientCity {
    id: CityId,
    flag: Flag,
    name: String,
    geo: GeoContext,
    production: CityProduction,
    exploitation: CityExploitation,
    tasks: ClientCityTasks,
}

impl ClientCity {
    pub fn id(&self) -> &CityId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
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
    id: UnitId,
    flag: Flag,
    type_: UnitType,
    geo: GeoContext,
    task: Option<ClientTask>,
    can: Vec<UnitCan>,
}

impl ClientUnit {
    pub fn id(&self) -> &UnitId {
        &self.id
    }

    pub fn flag(&self) -> &Flag {
        &self.flag
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

    pub fn can(&self) -> &[UnitCan] {
        &self.can
    }
}
