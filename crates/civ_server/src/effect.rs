use common::game::city::CityId;
use common::game::nation::flag::Flag;
use common::game::unit::UnitId;
use common::game::PlayerId;
use common::network::message::ServerToClientMessage;
use common::network::{Client, ClientId};
use common::space::window::Window;

use crate::game::{city::City, unit::Unit};

use crate::task::{Concern, TaskBox, TaskId};

#[derive(Debug, Clone)]
pub enum Effect {
    /// Effect which will modify the state
    State(StateEffect),
    /// Effect which will modify the state
    Runner(RunnerEffect),
    // FIXME BS NOW: not simply "SendToClients" ?
    /// Effect which only product reflects
    Shines(Vec<(ServerToClientMessage, Vec<ClientId>)>),
}

#[derive(Debug, Clone)]
pub enum RunnerEffect {
    Tasks(TasksEffect),
    Task(TaskId, TaskEffect),
}

#[derive(Debug, Clone)]
pub enum StateEffect {
    IncrementGameFrame,
    Clients(ClientsEffect),
    Client(Client, ClientEffect),
    City(CityId, CityEffect),
    Unit(UnitId, UnitEffect),
    Testing,
}

impl From<StateEffect> for Effect {
    fn from(value: StateEffect) -> Self {
        Self::State(value)
    }
}

#[derive(Debug, Clone)]
pub enum ClientsEffect {
    // FIXME BS NOW: when disconnected, remove
    Insert(ClientId, PlayerId),
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

pub fn new_unit(unit: Unit) -> StateEffect {
    StateEffect::Unit(*unit.id(), UnitEffect::New(unit))
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
    Effect::Runner(RunnerEffect::Tasks(TasksEffect::Add(tasks)))
}

pub fn add_task(task: TaskBox) -> Effect {
    Effect::Runner(RunnerEffect::Task(
        *task.context().id(),
        TaskEffect::Push(task),
    ))
}

pub fn remove_tasks(tasks: Vec<(TaskId, Concern)>) -> Effect {
    Effect::Runner(RunnerEffect::Tasks(TasksEffect::Remove(tasks)))
}

pub fn remove_task(task: TaskBox) -> Effect {
    Effect::Runner(RunnerEffect::Task(
        *task.context().id(),
        TaskEffect::Remove(*task.context().id(), task.concern()),
    ))
}
