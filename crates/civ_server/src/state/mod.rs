pub mod clients;

use clients::Clients;
use common::game::GameFrame;
use index::Index;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    game::{city::City, unit::Unit},
    task::{
        effect::{CityEffect, Effect, StateEffect, TaskEffect, TasksEffect, UnitEffect},
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
        let mut remove_tasks = vec![];

        for effect in effects {
            match effect {
                Effect::State(effect) => match effect {
                    StateEffect::Client(uuid, effect) => {
                        self.clients.apply(*uuid, effect);
                    }
                    StateEffect::Task(uuid, effect) => match effect {
                        TaskEffect::Push(task) => self.tasks.push(task.clone()),
                        TaskEffect::Finished(_) => remove_tasks.push(uuid),
                        TaskEffect::Remove(_, _) => remove_tasks.push(uuid),
                    },
                    StateEffect::Tasks(effect) => match effect {
                        TasksEffect::Remove(tasks) => {
                            remove_tasks
                                .extend(tasks.iter().map(|(i, _)| i).collect::<Vec<&Uuid>>());
                        }
                        TasksEffect::Add(tasks) => self.tasks.extend(tasks.clone()),
                    },
                    StateEffect::City(uuid, effect) => match effect {
                        CityEffect::New(city) => {
                            self.cities.push(city.clone());
                        }
                        CityEffect::Replace(city) => {
                            // TODO: unwrap (city can no longer exist)
                            *self.find_city_mut(city.id()).unwrap() = city.clone();
                        }
                        CityEffect::Remove(_) => {
                            // TODO: can use remove (by index) ? (state index is usable ?)
                            self.cities.retain(|city| city.id() != uuid);
                        }
                    },
                    StateEffect::Unit(uuid, effect) => match effect {
                        UnitEffect::New(unit) => {
                            self.units.push(unit.clone());
                        }
                        UnitEffect::Remove(_) => {
                            self.units.retain(|unit| unit.id() != *uuid);
                        }
                        UnitEffect::Replace(unit) => {
                            *self.find_unit_mut(uuid).unwrap() = unit.clone();
                        }
                    },
                },
            }
        }

        if !remove_tasks.is_empty() {
            // TODO: this is not a good performance way (idea: transport tasks index in tick)
            self.tasks
                .retain(|task| !remove_tasks.contains(&&task.context().id()));
        }

        // Update index must be after because based on &self.cities and &self.units
        self.index.apply(effects, &self.cities, &self.units);
    }

    pub fn cities(&self) -> &[City] {
        &self.cities
    }

    pub fn city(&self, index: usize, uuid: &Uuid) -> Result<&City, StateError> {
        if let Some(city) = self.cities.get(index) {
            if city.id() == uuid {
                return Ok(city);
            }
        }

        Err(StateError::NotFound(NotFound::City(index, *uuid)))
    }

    pub fn city_mut(&mut self, index: usize, uuid: &Uuid) -> Result<&mut City, StateError> {
        if let Some(city) = self.cities.get_mut(index) {
            if city.id() == uuid {
                return Ok(city);
            }
        }

        Err(StateError::NotFound(NotFound::City(index, *uuid)))
    }

    pub fn find_city(&self, uuid: &Uuid) -> Result<&City, StateError> {
        let unit_index = self
            .index()
            .uuid_cities()
            .get(uuid)
            .ok_or(StateError::NoLongerExist(NoLongerExist::City(*uuid)))?;
        self.city(*unit_index, uuid)
    }

    pub fn find_city_mut(&mut self, uuid: &Uuid) -> Result<&mut City, StateError> {
        let unit_index = self
            .index()
            .uuid_cities()
            .get(uuid)
            .ok_or(StateError::NoLongerExist(NoLongerExist::City(*uuid)))?;
        self.city_mut(*unit_index, uuid)
    }

    pub fn unit(&self, index: usize, uuid: &Uuid) -> Result<&Unit, StateError> {
        if let Some(unit) = self.units.get(index) {
            if &unit.id() == uuid {
                return Ok(unit);
            }
        }

        Err(StateError::NotFound(NotFound::Unit(index, *uuid)))
    }

    pub fn unit_mut(&mut self, index: usize, uuid: &Uuid) -> Result<&mut Unit, StateError> {
        if let Some(unit) = self.units.get_mut(index) {
            if &unit.id() == uuid {
                return Ok(unit);
            }
        }

        Err(StateError::NotFound(NotFound::Unit(index, *uuid)))
    }

    pub fn find_unit(&self, uuid: &Uuid) -> Result<&Unit, StateError> {
        let unit_index = self
            .index()
            .uuid_units()
            .get(uuid)
            .ok_or(StateError::NoLongerExist(NoLongerExist::Unit(*uuid)))?;
        self.unit(*unit_index, uuid)
    }

    pub fn find_unit_mut(&mut self, uuid: &Uuid) -> Result<&mut Unit, StateError> {
        let unit_index = self
            .index()
            .uuid_units()
            .get(uuid)
            .ok_or(StateError::NoLongerExist(NoLongerExist::Unit(*uuid)))?;
        self.unit_mut(*unit_index, uuid)
    }

    pub fn units(&self) -> &[Unit] {
        &self.units
    }

    pub fn index(&self) -> &Index {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut Index {
        &mut self.index
    }
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("Not found: {0}")]
    NotFound(NotFound),
    #[error("No longer exist: {0}")]
    NoLongerExist(NoLongerExist),
}

#[derive(Error, Debug)]
pub enum NoLongerExist {
    #[error("No city for uuid {0}")]
    City(Uuid),
    #[error("No unit for uuid {0}")]
    Unit(Uuid),
}

#[derive(Error, Debug)]
pub enum NotFound {
    #[error("No city for index {0} and uuid {1}")]
    City(usize, Uuid),
    #[error("No unit for index {0} and uuid {1}")]
    Unit(usize, Uuid),
}
