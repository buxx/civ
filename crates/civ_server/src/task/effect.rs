use common::space::window::Window;
use uuid::Uuid;

use crate::game::{city::City, unit::Unit};

use super::Task;

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
    Push(Box<dyn Task + Send>),
    Finished,
}

pub enum ClientEffect {
    SetWindow(Window),
}
pub enum CityEffect {
    New(City),
    Remove,
}
pub enum UnitEffect {
    New(Unit),
    Remove,
    Move((u64, u64)),
}

#[derive(Clone)]
pub enum IndexEffect {
    NewCity(City),
    RemovedCity(Uuid),
    NewUnit(Unit),
    RemovedUnit(Uuid),
    MovedUnit(Uuid, (u64, u64)),
}

impl Effect {
    pub fn index_effect(&self) -> Option<IndexEffect> {
        match self {
            Effect::State(effect) => match effect {
                StateEffect::Task(_, _) => None,
                StateEffect::Client(_, _) => None,
                StateEffect::City(uuid, effect) => match effect {
                    CityEffect::New(city) => Some(IndexEffect::NewCity(city.clone())),
                    CityEffect::Remove => Some(IndexEffect::RemovedCity(*uuid)),
                },
                StateEffect::Unit(uuid, effect) => match effect {
                    UnitEffect::New(unit) => Some(IndexEffect::NewUnit(unit.clone())),
                    UnitEffect::Remove => Some(IndexEffect::RemovedUnit(*uuid)),
                    UnitEffect::Move(to_) => Some(IndexEffect::MovedUnit(*uuid, *to_)),
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
