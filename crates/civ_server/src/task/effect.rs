use common::{geo::WorldPoint, space::window::Window};
use uuid::Uuid;

use crate::game::{city::City, unit::Unit};

use super::TaskBox;

// FIXME: Move this mod into state
#[derive(Debug, Clone)]
pub enum Effect {
    State(StateEffect),
}

#[derive(Debug, Clone)]
pub enum StateEffect {
    Client(Uuid, ClientEffect),
    Task(Uuid, TaskEffect),
    City(Uuid, CityEffect),
    Unit(Uuid, UnitEffect),
}

#[derive(Debug, Clone)]
pub enum TaskEffect {
    Push(TaskBox),
    Finished(TaskBox),
}

#[derive(Debug, Clone)]
pub enum ClientEffect {
    SetWindow(Window),
}

#[derive(Debug, Clone)]
pub enum CityEffect {
    New(City),
    Remove(City),
}

#[derive(Debug, Clone)]
pub enum UnitEffect {
    New(Unit),
    Remove(Unit),
    Move(Unit, WorldPoint),
}
