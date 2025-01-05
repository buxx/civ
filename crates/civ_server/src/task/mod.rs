pub mod city;
pub mod create;

use city::{BuildCityFrom, CityBuilder};
use common::{
    game::{
        slice::ClientConcreteTask,
        unit::{CityTaskType, TaskType, UnitTaskType},
        GameFrame,
    },
    geo::Geo,
};
use context::TaskContext;
use core::fmt::Debug;
use dyn_clone::DynClone;
use effect::{CityEffect, Effect, StateEffect, UnitEffect};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    game::{
        city::{City, CityProduction},
        task::{CityTaskWrapper, TaskWrapper},
        unit::Unit,
    },
    runner::RunnerContext,
    state::StateError,
};

pub mod context;
pub mod effect;

pub type TaskBox = Box<dyn Task + Send + Sync>;
pub type CityTaskBox = Box<dyn CityTask + Send + Sync>;
// pub type UnitTaskBox = Box<dyn UnitTask + Send + Sync>;

pub trait Task: DynClone + Then {
    fn type_(&self) -> TaskType;
    fn concern(&self) -> Concern;
    fn tick(&self, _frame: GameFrame) -> Vec<Effect> {
        vec![]
    }
    fn context(&self) -> &TaskContext;
}
dyn_clone::clone_trait_object!(Task);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Concern {
    Unit(Uuid),
    City(Uuid),
}

pub trait IntoClientConcreteTask {
    fn into_client(&self) -> ClientConcreteTask;
}

impl IntoClientConcreteTask for TaskBox {
    fn into_client(&self) -> ClientConcreteTask {
        ClientConcreteTask::new(
            self.context().id(),
            self.type_(),
            self.context().start(),
            self.context().end(),
        )
    }
}

impl Debug for TaskBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TaskBox")
            // .field(&self.type_())
            .field(&self.context())
            .finish()
    }
}

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("State error: {0}")]
    State(#[from] StateError),
}

// #[derive(Debug, Default, Clone)]
// pub struct Tasks<T> {
//     stack: Vec<(Uuid, T)>,
// }

// impl<T> Tasks<T> {
//     pub fn empty() -> Self {
//         Self { stack: vec![] }
//     }

//     pub fn new(stack: Vec<(Uuid, T)>) -> Self {
//         Self { stack }
//     }

//     pub fn stack(&self) -> &[(Uuid, T)] {
//         &self.stack
//     }

//     fn replace(&mut self, tasks: Vec<(Uuid, T)>) {
//         self.stack = tasks;
//     }
// }

pub trait Then {
    fn then(&self, context: &RunnerContext) -> Result<(Vec<Effect>, Vec<TaskWrapper>), TaskError>;
}

pub trait WithUnit {
    fn unit(&self) -> &Unit;
}

// pub trait WithCity {
//     fn city(&self) -> &City;
// }

pub trait CityName {
    fn city_name(&self) -> &str;
}

pub trait ThenTransformUnitIntoCity: WithUnit + CityName + Geo {
    fn transform_unit_into_city(
        &self,
        context: &RunnerContext,
    ) -> Result<(Vec<Effect>, Vec<TaskWrapper>), TaskError> {
        let city = self.city(context)?;
        let tasks = city.tasks().clone().into();

        // let state = context.state();

        // let unit = self.unit();
        // let mut city = self.city(context);
        // let tasks = self.tasks(&city, context)?;
        // city.update(&tasks);
        // city.tasks_mut().replace(
        //     tasks
        //         .iter()
        //         .map(|t| (t.context().id(), t.city_task_type()))
        //         .collect::<Vec<(Uuid, CityTaskType)>>(),
        // );
        // let tasks: Vec<TaskBox> = tasks.into_iter().map(|t| t.into_task()).collect();

        Ok((
            vec![
                Effect::State(StateEffect::Unit(
                    self.unit().id(),
                    UnitEffect::Remove(self.unit().clone()),
                )),
                Effect::State(StateEffect::City(city.id(), CityEffect::New(city))),
            ],
            tasks,
        ))
    }

    fn city(&self, context: &RunnerContext) -> Result<City, TaskError> {
        CityBuilder::builder()
            .context(context)
            .game_frame(*context.state().frame())
            .from(BuildCityFrom::Scratch(
                self.city_name().to_string(),
                *self.unit().geo(),
            ))
            .build()
            .build()
    }
}

pub trait CityTask: DynClone + Task {
    fn city_task_type(&self) -> CityTaskType;
    // See https://users.rust-lang.org/t/reconsider-trait-as-another/123488/5
    fn into_task(&self) -> TaskBox;
}
dyn_clone::clone_trait_object!(CityTask);

pub trait UnitTask: DynClone + Task {
    fn unit_task_type(&self) -> UnitTaskType;
    // See https://users.rust-lang.org/t/reconsider-trait-as-another/123488/5
    fn into_task(&self) -> TaskBox;
}
dyn_clone::clone_trait_object!(UnitTask);
