pub mod clients;

use clients::Clients;
use common::{game::GameFrame, geo::Geo};
use index::Index;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    game::{city::City, unit::Unit},
    task::{
        effect::{CityEffect, Effect, StateEffect, TaskEffect, UnitEffect},
        TaskBox,
    },
};

pub mod index;

#[derive(Default)]
pub struct State {
    frame_i: GameFrame,
    clients: Clients,
    index: Index,
    tasks: Vec<TaskBox>,
    cities: Vec<City>,
    units: Vec<Unit>,
}

impl State {
    pub fn frame(&self) -> &GameFrame {
        &self.frame_i
    }

    pub fn tasks(&self) -> &Vec<TaskBox> {
        &self.tasks
    }

    pub fn clients(&self) -> &Clients {
        &self.clients
    }

    pub fn clients_mut(&mut self) -> &mut Clients {
        &mut self.clients
    }

    pub fn increment(&mut self) {
        self.frame_i += GameFrame(1);
    }

    pub fn apply(&mut self, effects: &Vec<Effect>) {
        let mut remove_ids = vec![];

        for effect in effects {
            match effect {
                Effect::State(effect) => match effect {
                    StateEffect::Client(uuid, effect) => {
                        self.clients.apply(*uuid, effect);
                    }
                    StateEffect::Task(uuid, effect) => match effect {
                        TaskEffect::Finished(_) => remove_ids.push(uuid),
                        TaskEffect::Push(task) => self.tasks.push(task.clone()),
                    },
                    StateEffect::City(uuid, effect) => match effect {
                        CityEffect::New(city) => {
                            self.cities.push(city.clone());
                        }
                        CityEffect::Remove(_) => {
                            self.cities.retain(|city| city.id() != *uuid);
                        }
                    },
                    StateEffect::Unit(uuid, effect) => match effect {
                        UnitEffect::New(unit) => {
                            self.units.push(unit.clone());
                        }
                        UnitEffect::Remove(_) => {
                            self.units.retain(|unit| unit.id() != *uuid);
                        }
                        UnitEffect::Move(unit_, to_) => {
                            if let Some(unit) = self.units.iter_mut().find(|u| u.id() == unit_.id())
                            {
                                unit.geo_mut().set_point(*to_)
                            }
                        }
                    },
                },
            }
        }

        if !remove_ids.is_empty() {
            // TODO: this is not a good performance way (idea: transport tasks index in tick)
            self.tasks
                .retain(|task| !remove_ids.contains(&&task.context().id()));
        }

        // Update index must be after because based on &self.cities and &self.units
        self.index.apply(&effects, &self.cities, &self.units);
    }

    pub fn cities(&self) -> &[City] {
        &self.cities
    }

    pub fn city(&self, index: usize, uuid: &Uuid) -> Result<&City, StateError> {
        if let Some(city) = self.cities.get(index) {
            if &city.id() == uuid {
                return Ok(city);
            }
        }

        Err(StateError::CityNotFound(index, *uuid))
    }

    pub fn find_city(&self, uuid: &Uuid) -> Result<&City, StateError> {
        let unit_index = self
            .index()
            .uuid_cities()
            .get(uuid)
            .ok_or(StateError::CityUuidFound(*uuid))?;
        self.city(*unit_index, uuid)
    }

    pub fn unit(&self, index: usize, uuid: &Uuid) -> Result<&Unit, StateError> {
        if let Some(unit) = self.units.get(index) {
            if &unit.id() == uuid {
                return Ok(unit);
            }
        }

        Err(StateError::UnitNotFound(index, *uuid))
    }

    pub fn find_unit(&self, uuid: &Uuid) -> Result<&Unit, StateError> {
        let unit_index = self
            .index()
            .uuid_units()
            .get(uuid)
            .ok_or(StateError::UnitUuidNotFound(*uuid))?;
        self.unit(*unit_index, uuid)
    }

    pub fn units(&self) -> &[Unit] {
        &self.units
    }

    pub fn index(&self) -> &Index {
        &self.index
    }

    pub fn city_tasks(&self, city_id: &Uuid) -> Result<Vec<&TaskBox>, StateError> {
        // CityTasks::from(
        //     self.index
        //         .city_tasks(city_id)
        //         .iter()
        //         // TODO: probably a performance issue here
        //         .filter_map(|id| self.tasks.iter().find(|t| t.context().id() == *id))
        //         .collect::<Vec<&TaskBox>>(),
        // )
        // .map_err(|e| StateError::CityTasksIntegrity(*city_id, e))
        Ok(self
            .index
            .city_tasks(city_id)
            .iter()
            // TODO: probably a performance issue here
            .filter_map(|id| self.tasks.iter().find(|t| t.context().id() == *id))
            .collect::<Vec<&TaskBox>>())
    }
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("No city for index {0} and uuid {1}")]
    CityNotFound(usize, Uuid),
    #[error("No city for uuid {0}")]
    CityUuidFound(Uuid),
    #[error("No unit for index {0} and uuid {1}")]
    UnitNotFound(usize, Uuid),
    #[error("No unit for uuid {0}")]
    UnitUuidNotFound(Uuid),
    // #[error("City integrity error ({0}): {1}")]
    // CityTasksIntegrity(Uuid, CityIntegrityError),
}
