use common::game::city::CityId;
use common::game::nation::flag::Flag;
use common::game::unit::UnitId;
use common::network::message::ServerToClientMessage;
use common::network::{Client, ClientId};
use common::space::window::{SetWindow, Window, Resolution};

use crate::game::{city::City, unit::Unit};

use crate::task::{Concern, TaskBox, TaskId};

#[derive(Debug, Clone)]
pub enum Effect {
    /// Effect which will modify the state
    State(StateEffect),
    /// Effect which only product reflects
    Shines(Vec<(ServerToClientMessage, Vec<ClientId>)>),
    /// Effect which product immediate action
    // FIXME BS NOW: should not be a simple StateEffect or Shines?
    Action(Action),
}

#[derive(Debug, Clone)]
pub enum StateEffect {
    IncrementGameFrame,
    Clients(ClientsEffect),
    Client(Client, ClientEffect),
    Tasks(TasksEffect),
    Task(TaskId, TaskEffect),
    City(CityId, CityEffect),
    Unit(UnitId, UnitEffect),
    Testing,
}

#[derive(Debug, Clone)]
pub enum ClientsEffect {
    Count,
}

#[derive(Debug, Clone)]
pub enum Action {
    UpdateClientWindow(Client, SetWindow),
}

#[derive(Debug, Clone)]
pub enum TaskEffect {
    Push(TaskBox),
    Finished(TaskBox),
    Remove(TaskId, Concern),
}

#[derive(Debug, Clone)]
pub enum TasksEffect {
    Remove(Vec<(TaskId, Concern)>),
    Add(Vec<TaskBox>),
}

#[derive(Debug, Clone)]
pub enum ClientEffect {
    PlayerTookPlace(Flag, Window),
    SetResolution(Resolution),
    SetWindow(Window),
}

#[derive(Debug, Clone)]
pub enum CityEffect {
    New(City),
    Replace(City),
    Remove(City),
}

#[derive(Debug, Clone)]
pub enum UnitEffect {
    New(Unit),
    Replace(Unit),
    Remove(Unit),
}

pub fn new_unit(unit: Unit) -> Effect {
    Effect::State(StateEffect::Unit(*unit.id(), UnitEffect::New(unit)))
}

pub fn replace_unit(unit: Unit) -> Effect {
    Effect::State(StateEffect::Unit(*unit.id(), UnitEffect::Replace(unit)))
}

pub fn remove_unit(unit: Unit) -> Effect {
    Effect::State(StateEffect::Unit(*unit.id(), UnitEffect::Remove(unit)))
}

pub fn new_city(city: City) -> Effect {
    Effect::State(StateEffect::City(*city.id(), CityEffect::New(city)))
}

pub fn replace_city(city: City) -> Effect {
    Effect::State(StateEffect::City(*city.id(), CityEffect::Replace(city)))
}

pub fn add_tasks(tasks: Vec<TaskBox>) -> Effect {
    Effect::State(StateEffect::Tasks(TasksEffect::Add(tasks)))
}

pub fn add_task(task: TaskBox) -> Effect {
    Effect::State(StateEffect::Task(
        *task.context().id(),
        TaskEffect::Push(task),
    ))
}

pub fn remove_tasks(tasks: Vec<(TaskId, Concern)>) -> Effect {
    Effect::State(StateEffect::Tasks(TasksEffect::Remove(tasks)))
}

pub fn remove_task(task: TaskBox) -> Effect {
    Effect::State(StateEffect::Task(
        *task.context().id(),
        TaskEffect::Remove(*task.context().id(), task.concern()),
    ))
}
