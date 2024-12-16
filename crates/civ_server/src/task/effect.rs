use common::{
    network::message::{ClientStateMessage, ServerToClientMessage},
    space::window::Window,
};
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

#[derive(Clone)]
pub enum ClientEffect {
    SetWindow(Window),
}

#[derive(Clone)]
pub enum CityEffect {
    New(City),
    Remove(Uuid),
}

#[derive(Clone)]
pub enum UnitEffect {
    New(Unit),
    Remove(Uuid),
    Move(Uuid, (u64, u64)),
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
                    CityEffect::Remove(_) => Some(IndexEffect::RemovedCity(*uuid)),
                },
                StateEffect::Unit(uuid, effect) => match effect {
                    UnitEffect::New(unit) => Some(IndexEffect::NewUnit(unit.clone())),
                    UnitEffect::Remove(_) => Some(IndexEffect::RemovedUnit(*uuid)),
                    UnitEffect::Move(_, to_) => Some(IndexEffect::MovedUnit(*uuid, *to_)),
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

impl Into<ServerToClientMessage> for &CityEffect {
    fn into(self) -> ServerToClientMessage {
        match self {
            CityEffect::New(city) => {
                ServerToClientMessage::State(ClientStateMessage::AddCity(city.into()))
            }
            CityEffect::Remove(uuid) => {
                ServerToClientMessage::State(ClientStateMessage::RemoveCity(*uuid))
            }
        }
    }
}

impl Into<ServerToClientMessage> for &UnitEffect {
    fn into(self) -> ServerToClientMessage {
        match self {
            UnitEffect::New(unit) => {
                ServerToClientMessage::State(ClientStateMessage::AddUnit(unit.into()))
            }
            UnitEffect::Remove(uuid) => {
                ServerToClientMessage::State(ClientStateMessage::RemoveUnit(*uuid))
            }
            UnitEffect::Move(uuid, to_) => {
                ServerToClientMessage::State(ClientStateMessage::MoveUnit(*uuid, *to_))
            }
        }
    }
}

impl Effect {
    pub fn reflect(&self) -> Option<ServerToClientMessage> {
        match self {
            Effect::State(effect) => match effect {
                StateEffect::Client(_, _) => None,
                StateEffect::Task(_, _) => None,
                StateEffect::City(_, city_effect) => Some(city_effect.into()),
                StateEffect::Unit(_, unit_effect) => Some(unit_effect.into()),
            },
        }
    }
}
