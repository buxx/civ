pub mod clients;
use std::ops::{Add, AddAssign};

use clients::Clients;
use index::Index;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    game::{city::City, task::settle::Settle, unit::Unit},
    task::{
        context::{PhysicalContext, TaskContext},
        effect::{CityEffect, Effect, IntoIndexEffects, StateEffect, TaskEffect, UnitEffect},
        Task,
    },
};

pub mod index;

pub const GAME_FRAMES_PER_SECOND: u64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GameFrame(pub u64);

impl Add<u64> for GameFrame {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign for GameFrame {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(Default)]
pub struct State {
    frame_i: GameFrame,
    clients: Clients,
    index: Index,
    tasks: Vec<Box<dyn Task + Send>>,
    cities: Vec<City>,
    units: Vec<Unit>,
}

impl State {
    pub fn frame(&self) -> &GameFrame {
        &self.frame_i
    }

    pub fn tasks(&self) -> &Vec<Box<dyn Task + Send>> {
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

        // HACK
        if self.frame_i.0 == 0 {
            for x in 0..100 {
                for y in 0..100 {
                    self.cities.push(
                        City::builder()
                            .id(Uuid::new_v4())
                            .physics(PhysicalContext::builder().x(x * 5).y(y * 5).build())
                            .build(),
                    );
                }
            }
        }
        if self.frame_i.0 == 19 {
            for _ in 0..1_000 {
                self.tasks.push(Box::new(
                    Settle::builder()
                        .context(
                            TaskContext::builder()
                                .id(Uuid::new_v4())
                                .start(self.frame_i)
                                .end(self.frame_i + GAME_FRAMES_PER_SECOND * 5)
                                .build(),
                        )
                        .physic(PhysicalContext::builder().x(0).y(0).build())
                        .build(),
                ))
            }
        }
    }

    pub fn apply(&mut self, effects: Vec<Effect>) {
        let mut remove_ids = vec![];
        let index_effects = effects.index_effects();

        for effect in effects {
            match effect {
                Effect::State(effect) => match effect {
                    StateEffect::Client(uuid, effect) => {
                        self.clients.apply(uuid, effect);
                    }
                    StateEffect::Task(uuid, effect) => match effect {
                        TaskEffect::Finished => remove_ids.push(uuid),
                    },
                    StateEffect::City(uuid, effect) => match effect {
                        CityEffect::New(city) => {
                            self.cities.push(city);
                        }
                        CityEffect::Remove => {
                            self.cities.retain(|city| city.id() != uuid);
                        }
                    },
                    StateEffect::Unit(uuid, effect) => match effect {
                        UnitEffect::New(unit) => {
                            self.units.push(unit);
                        }
                        UnitEffect::Remove => {
                            self.units.retain(|unit| unit.id() != uuid);
                        }
                        UnitEffect::Move(_unit) => todo!(),
                    },
                },
            }
        }

        if !remove_ids.is_empty() {
            // TODO: this is not a good performance way (idea: transport tasks index in tick)
            self.tasks
                .retain(|task| !remove_ids.contains(&task.context().id()));
        }

        self.index.apply(index_effects, &self.cities, &self.units);
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

    pub fn unit(&self, index: usize, uuid: &Uuid) -> Result<&Unit, StateError> {
        if let Some(unit) = self.units.get(index) {
            if &unit.id() == uuid {
                return Ok(unit);
            }
        }

        Err(StateError::UnitNotFound(index, *uuid))
    }

    pub fn units(&self) -> &[Unit] {
        &self.units
    }

    pub fn index(&self) -> &Index {
        &self.index
    }
}

#[derive(Error, Debug)]
pub enum StateError {
    #[error("No city for index {0} and uuid {1}")]
    CityNotFound(usize, Uuid),
    #[error("No unit for index {0} and uuid {1}")]
    UnitNotFound(usize, Uuid),
}
