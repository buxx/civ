use std::sync::MutexGuard;

use common::{
    network::message::{ClientStateMessage, ServerToClientMessage},
    space::window::Window,
};
use uuid::Uuid;

use crate::{
    game::{city::City, extractor::Extractor, unit::Unit},
    state::State,
};

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
    Finished(Uuid),
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

impl CityEffect {
    fn into_client_message(&self, state: &MutexGuard<State>) -> ServerToClientMessage {
        match self {
            CityEffect::New(city) => {
                let city = Extractor::new(state).city_into_client(city);
                ServerToClientMessage::State(ClientStateMessage::AddCity(city))
            }
            CityEffect::Remove(uuid) => {
                ServerToClientMessage::State(ClientStateMessage::RemoveCity(*uuid))
            }
        }
    }
}

impl UnitEffect {
    fn into_client_message(&self, state: &MutexGuard<State>) -> ServerToClientMessage {
        match self {
            UnitEffect::New(unit) => {
                let unit = Extractor::new(state).unit_into_client(unit);
                ServerToClientMessage::State(ClientStateMessage::AddUnit(unit))
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

impl TaskEffect {
    fn into_client_message(&self, state: &MutexGuard<State>) -> ServerToClientMessage {
        match self {
            TaskEffect::Push(task) => {
                // FIXME: how to be sure about unit_uuid ?
                let unit_uuid = task.concerned_unit().unwrap();
                let task = Extractor::new(state).task_into_client(task);
                ServerToClientMessage::State(ClientStateMessage::AddUnitTask(unit_uuid, task))
            }
            TaskEffect::Finished(uuid) => {
                // FIXME: not good, hopefully state is modified after ...
                let task = state
                    .tasks()
                    .iter()
                    .find(|t| t.context().id() == *uuid)
                    .unwrap();
                // FIXME: how to be sure about unit_uuid ?
                let unit_uuid = task.concerned_unit().unwrap();
                ServerToClientMessage::State(ClientStateMessage::RemoveUnitTask(unit_uuid, *uuid))
            }
        }
    }
}

impl Effect {
    pub fn reflect(&self, state: &MutexGuard<State>) -> Option<ServerToClientMessage> {
        match self {
            Effect::State(effect) => match effect {
                StateEffect::Client(_, _) => None,
                StateEffect::Task(_, effect) => Some(effect.into_client_message(state)),
                StateEffect::City(_, effect) => Some(effect.into_client_message(state)),
                StateEffect::Unit(_, effect) => Some(effect.into_client_message(state)),
            },
        }
    }
}
