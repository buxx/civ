use common::space::window::Window;
use uuid::Uuid;

use crate::game::{city::City, unit::Unit};

use super::{Concern, TaskBox};

// FIXME: Move this mod into state
#[derive(Debug, Clone)]
pub enum Effect {
    State(StateEffect),
}

#[derive(Debug, Clone)]
pub enum StateEffect {
    Client(Uuid, ClientEffect),
    Tasks(TasksEffect),
    Task(Uuid, TaskEffect),
    City(Uuid, CityEffect),
    Unit(Uuid, UnitEffect),
}

#[derive(Debug, Clone)]
pub enum TaskEffect {
    Push(TaskBox),
    Finished(TaskBox),
    Remove(Uuid, Concern),
}

#[derive(Debug, Clone)]
pub enum TasksEffect {
    Remove(Vec<(Uuid, Concern)>),
    Add(Vec<TaskBox>),
}

#[derive(Debug, Clone)]
pub enum ClientEffect {
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
    Effect::State(StateEffect::Unit(unit.id(), UnitEffect::New(unit)))
}

pub fn replace_unit(unit: Unit) -> Effect {
    Effect::State(StateEffect::Unit(unit.id(), UnitEffect::Replace(unit)))
}

pub fn remove_unit(unit: Unit) -> Effect {
    Effect::State(StateEffect::Unit(unit.id(), UnitEffect::Remove(unit)))
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
        task.context().id(),
        TaskEffect::Push(task),
    ))
}

pub fn remove_tasks(tasks: Vec<(Uuid, Concern)>) -> Effect {
    Effect::State(StateEffect::Tasks(TasksEffect::Remove(tasks)))
}

pub fn remove_task(task: TaskBox) -> Effect {
    Effect::State(StateEffect::Task(
        task.context().id(),
        TaskEffect::Remove(task.context().id(), task.concern()),
    ))
}
