pub mod clients;

use clients::Clients;
use common::{
    game::{
        city::CityId, nation::flag::Flag, server::ServerResume, unit::UnitId, GameFrame, PlayerId,
    },
    network::Client,
    rules::RuleSetBox,
    space::window::{DisplayStep, Window},
};
use index::Index;
use thiserror::Error;

use crate::{
    effect::{
        Action, CityEffect, ClientEffect, Effect, StateEffect, TaskEffect, TasksEffect, UnitEffect,
    },
    game::{city::City, unit::Unit},
    snapshot::Snapshot,
    task::{TaskBox, TaskId},
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
    testing: u64,
}

impl State {
    pub fn new(
        frame_i: GameFrame,
        clients: Clients,
        index: Index,
        tasks: Vec<TaskBox>,
        cities: Vec<City>,
        units: Vec<Unit>,
        testing: u64,
    ) -> Self {
        Self {
            frame_i,
            clients,
            index,
            tasks,
            cities,
            units,
            testing,
        }
    }

    pub fn with_tasks(mut self, tasks: Vec<TaskBox>) -> Self {
        self.tasks = tasks;
        self
    }

    pub fn frame(&self) -> &GameFrame {
        &self.frame_i
    }

    pub fn tasks(&self) -> &Vec<TaskBox> {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut Vec<TaskBox> {
        &mut self.tasks
    }

    pub fn clients(&self) -> &Clients {
        &self.clients
    }

    pub fn clients_mut(&mut self) -> &mut Clients {
        &mut self.clients
    }

    pub fn increment_frame(&mut self) {
        self.frame_i += GameFrame(1);
    }

    pub fn apply(&mut self, effects: &Vec<Effect>) {
        let mut remove_tasks = vec![];

        for effect in effects {
            match effect {
                Effect::State(effect) => match effect {
                    StateEffect::IncrementGameFrame => {
                        self.increment_frame();
                    }
                    StateEffect::Client(client, effect) => {
                        self.clients.apply(client, effect).unwrap();
                    }
                    StateEffect::Task(uuid, effect) => match effect {
                        TaskEffect::Push(task) => self.tasks.push(task.clone()),
                        TaskEffect::Finished(_) => remove_tasks.push(uuid),
                        TaskEffect::Remove(_, _) => remove_tasks.push(uuid),
                    },
                    StateEffect::Tasks(effect) => match effect {
                        TasksEffect::Remove(tasks) => {
                            remove_tasks
                                .extend(tasks.iter().map(|(i, _)| i).collect::<Vec<&TaskId>>());
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
                    StateEffect::Unit(unit_id, effect) => match effect {
                        UnitEffect::New(unit) => {
                            self.units.push(unit.clone());
                        }
                        UnitEffect::Remove(_) => {
                            self.units.retain(|unit| unit.id() != unit_id);
                        }
                        UnitEffect::Replace(unit) => {
                            *self.find_unit_mut(unit_id).unwrap() = unit.clone();
                        }
                    },
                    StateEffect::Testing => {
                        self.testing += 1;
                    }
                },
                Effect::Action(action) => match action {
                    Action::UpdateClientWindow(client, set_window) => {
                        let window = Window::new(
                            set_window.start_x(),
                            set_window.start_y(),
                            set_window.end_x(),
                            set_window.end_y(),
                            DisplayStep::from_shape(set_window.shape()),
                        );
                        self.clients
                            .apply(client, &ClientEffect::SetWindow(window))
                            .unwrap();
                    }
                },
                Effect::Shines(_) => {}
            }
        }

        if !remove_tasks.is_empty() {
            // TODO: this is not a good performance way (idea: transport tasks index in tick)
            self.tasks
                .retain(|task| !remove_tasks.contains(&task.context().id()));
        }

        // Update index must be after because based on &self.cities and &self.units
        self.index.apply(effects, &self.cities, &self.units);
    }

    pub fn cities(&self) -> &[City] {
        &self.cities
    }

    pub fn city(&self, index: usize, city_id: &CityId) -> Result<&City, StateError> {
        if let Some(city) = self.cities.get(index) {
            if city.id() == city_id {
                return Ok(city);
            }
        }

        Err(StateError::NotFound(NotFound::City(index, *city_id)))
    }

    pub fn city_mut(&mut self, index: usize, city_id: &CityId) -> Result<&mut City, StateError> {
        if let Some(city) = self.cities.get_mut(index) {
            if city.id() == city_id {
                return Ok(city);
            }
        }

        Err(StateError::NotFound(NotFound::City(index, *city_id)))
    }

    pub fn find_city(&self, city_id: &CityId) -> Result<&City, StateError> {
        let unit_index = self
            .index()
            .uuid_cities()
            .get(city_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::City(*city_id)))?;
        self.city(*unit_index, city_id)
    }

    pub fn find_city_mut(&mut self, city_id: &CityId) -> Result<&mut City, StateError> {
        let unit_index = self
            .index()
            .uuid_cities()
            .get(city_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::City(*city_id)))?;
        self.city_mut(*unit_index, city_id)
    }

    pub fn unit(&self, index: usize, unit_id: &UnitId) -> Result<&Unit, StateError> {
        if let Some(unit) = self.units.get(index) {
            if unit.id() == unit_id {
                return Ok(unit);
            }
        }

        Err(StateError::NotFound(NotFound::Unit(index, *unit_id)))
    }

    pub fn unit_mut(&mut self, index: usize, unit_id: &UnitId) -> Result<&mut Unit, StateError> {
        if let Some(unit) = self.units.get_mut(index) {
            if unit.id() == unit_id {
                return Ok(unit);
            }
        }

        Err(StateError::NotFound(NotFound::Unit(index, *unit_id)))
    }

    pub fn find_unit(&self, unit_id: &UnitId) -> Result<&Unit, StateError> {
        let unit_index = self
            .index()
            .uuid_units()
            .get(unit_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::Unit(*unit_id)))?;
        self.unit(*unit_index, unit_id)
    }

    pub fn find_unit_mut(&mut self, unit_id: &UnitId) -> Result<&mut Unit, StateError> {
        let unit_index = self
            .index()
            .uuid_units()
            .get(unit_id)
            .ok_or(StateError::NoLongerExist(NoLongerExist::Unit(*unit_id)))?;
        self.unit_mut(*unit_index, unit_id)
    }

    pub fn units(&self) -> &[Unit] {
        &self.units
    }

    pub fn units_mut(&mut self) -> &mut Vec<Unit> {
        &mut self.units
    }

    pub fn index(&self) -> &Index {
        &self.index
    }

    pub fn index_mut(&mut self) -> &mut Index {
        &mut self.index
    }

    pub fn testing(&self) -> u64 {
        self.testing
    }

    pub fn client_flag(&self, client: &Client) -> Result<&Flag, StateError> {
        Ok(self
            .clients
            .player_state(client.player_id())
            .ok_or(StateError::NoLongerExist(NoLongerExist::Player(
                *client.player_id(),
            )))?
            .flag())
    }

    pub fn server_resume(&self, rules: &RuleSetBox) -> ServerResume {
        let flags = self.clients.flags();
        ServerResume::new(rules.clone().into(), flags)
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot::from(self)
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
    City(CityId),
    #[error("No unit for uuid {0}")]
    Unit(UnitId),
    #[error("No player state for uuid {0}")]
    Player(PlayerId),
}

#[derive(Error, Debug)]
pub enum NotFound {
    #[error("No city for index {0} and uuid {1}")]
    City(usize, CityId),
    #[error("No unit for index {0} and uuid {1}")]
    Unit(usize, UnitId),
}
