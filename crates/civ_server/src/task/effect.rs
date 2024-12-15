use common::space::window::Window;
use uuid::Uuid;

use crate::game::{city::City, unit::Unit};

// FIXME: Move this mod into state
pub enum Effect {
    State(StateEffect),
}

pub enum StateEffect {
    Client(Uuid, ClientEffect),
    Task(Uuid, TaskEffect),
    City(Uuid, CityEffect),
    Unit(Uuid, UnitEffect),
}
pub enum TaskEffect {
    Finished,
}

pub enum ClientEffect {
    SetWindow(Window),
}
pub enum CityEffect {
    New(City),
    Remove(City),
}
pub enum UnitEffect {
    New(Unit),
    Remove(Unit),
    Move(Unit, (u64, u64), (u64, u64)),
}

#[derive(Clone)]
pub enum IndexEffect {
    NewlyCity(City),
    RemovedCity(City),
    NewlyUnit(Unit),
    RemovedUnit(Unit),
    MovedUnit(Unit, (u64, u64), (u64, u64)),
}

impl Effect {
    pub fn index_effect(&self) -> Option<IndexEffect> {
        match self {
            Effect::State(effect) => match effect {
                StateEffect::Task(_, _) => None,
                StateEffect::Client(_, _) => None,
                StateEffect::City(_, effect) => match effect {
                    CityEffect::New(city) => Some(IndexEffect::NewlyCity(city.clone())),
                    CityEffect::Remove(city) => Some(IndexEffect::RemovedCity(city.clone())),
                },
                StateEffect::Unit(_, effect) => match effect {
                    UnitEffect::New(unit) => Some(IndexEffect::NewlyUnit(unit.clone())),
                    UnitEffect::Remove(unit) => Some(IndexEffect::RemovedUnit(unit.clone())),
                    UnitEffect::Move(unit, from_, to_) => {
                        Some(IndexEffect::MovedUnit(unit.clone(), *from_, *to_))
                    }
                },
            },
        }
    }
}

pub trait IntoIndexEffects {
    fn index_effects(&self) -> Vec<IndexEffect>;
}

impl IntoIndexEffects for Vec<Effect> {
    fn index_effects(&self) -> Vec<IndexEffect> {
        self.iter().filter_map(|e| e.index_effect()).collect()
    }
}
