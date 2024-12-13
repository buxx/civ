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
    Remove,
}
pub enum UnitEffect {
    New(Unit),
    Remove,
    Move(Unit),
}

#[derive(Clone)]
pub enum IndexEffect {
    RefreshCityIndexes,
    RefreshUnitIndexes,
    NewlyCity(City),
    RemovedCity(Uuid),
    NewlyUnit(Unit),
    RemovedUnit(Uuid),
    MovedUnit(Unit),
}

impl Effect {
    pub fn index_effect(&self) -> Option<IndexEffect> {
        match self {
            Effect::State(effect) => match effect {
                StateEffect::Task(_, _) => None,
                StateEffect::Client(_, _) => None,
                StateEffect::City(uuid, effect) => match effect {
                    CityEffect::New(city) => Some(IndexEffect::NewlyCity(city.clone())),
                    CityEffect::Remove => Some(IndexEffect::RemovedCity(*uuid)),
                },
                StateEffect::Unit(uuid, effect) => match effect {
                    UnitEffect::New(unit) => Some(IndexEffect::NewlyUnit(unit.clone())),
                    UnitEffect::Remove => Some(IndexEffect::RemovedUnit(*uuid)),
                    UnitEffect::Move(unit) => Some(IndexEffect::MovedUnit(unit.clone())),
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
        let mut refresh_cities_index = false;
        let mut refresh_units_index = false;
        let mut index_effects = vec![];

        for effect in self.iter() {
            if let Some(index_effect) = effect.index_effect() {
                match index_effect {
                    IndexEffect::RefreshCityIndexes => refresh_cities_index = true,
                    IndexEffect::RefreshUnitIndexes => refresh_units_index = true,
                    IndexEffect::NewlyCity(_) => index_effects.push(index_effect.clone()),
                    IndexEffect::RemovedCity(_) => index_effects.push(index_effect.clone()),
                    IndexEffect::NewlyUnit(_) => index_effects.push(index_effect.clone()),
                    IndexEffect::RemovedUnit(_) => index_effects.push(index_effect.clone()),
                    IndexEffect::MovedUnit(_) => index_effects.push(index_effect.clone()),
                }
            }
        }

        if refresh_cities_index {
            index_effects.push(IndexEffect::RefreshCityIndexes)
        }

        if refresh_units_index {
            index_effects.push(IndexEffect::RefreshUnitIndexes)
        }

        index_effects
    }
}
