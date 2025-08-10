use bon::Builder;
use common::{
    game::{
        city::{CityExploitation, CityId, CityProduction, CityProductionTons},
        nation::flag::Flag,
        GameFrame,
    },
    geo::{Geo, GeoContext},
};

use crate::{
    game::city::City,
    runner::RunnerContext,
    task::city::{production::production_task, CityTasks},
};

use super::TaskError;

#[derive(Builder)]
pub struct CityGenerator<'a> {
    context: &'a RunnerContext,
    game_frame: &'a GameFrame,
    from: BuildCityFrom<'a>,
}

pub enum BuildCityFrom<'a> {
    Scratch(String, Flag, GeoContext),
    Change(&'a City, BuildCityFromChange),
}

pub enum BuildCityFromChange {
    Production(CityProduction),
    Exploitation(CityExploitation),
}

impl BuildCityFrom<'_> {
    pub fn id(&self) -> Option<&CityId> {
        match self {
            BuildCityFrom::Scratch(_, _, _) => None,
            BuildCityFrom::Change(city, _) => Some(city.id()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            BuildCityFrom::Scratch(city_name, _, _) => city_name,
            BuildCityFrom::Change(city, _) => city.name(),
        }
    }

    pub fn flag(&self) -> &Flag {
        match self {
            BuildCityFrom::Scratch(_, flag, _) => flag,
            BuildCityFrom::Change(city, _) => city.flag(),
        }
    }

    pub fn geo(&self) -> &GeoContext {
        match self {
            BuildCityFrom::Scratch(_, _, geo) => geo,
            BuildCityFrom::Change(city, _) => city.geo(),
        }
    }

    pub fn production(&self) -> Option<&CityProduction> {
        match self {
            BuildCityFrom::Scratch(_, _, _) => None,
            BuildCityFrom::Change(city, _) => Some(city.production()),
        }
    }
}

impl CityGenerator<'_> {
    pub fn generate(&self) -> Result<City, TaskError> {
        let default_production = self.context.default_production();
        let city_id = self.from.id().copied().unwrap_or(CityId::default());
        let tasks = CityTasks::new(production_task(
            self.context.context.rules(),
            self.game_frame,
            &self.from,
            &city_id,
            default_production.current(),
        ));
        // TODO: tons according to exploitation (according to geo ...)
        let exploitation = CityExploitation::new(CityProductionTons(1));

        Ok(City::builder()
            .id(city_id)
            .name(self.from.name().to_string())
            .flag(*self.from.flag())
            .geo(*self.from.geo())
            .production(
                self.from
                    .production()
                    .unwrap_or(&self.context.default_production())
                    .clone(),
            )
            .tasks(tasks)
            .exploitation(exploitation)
            .build())
    }
}
