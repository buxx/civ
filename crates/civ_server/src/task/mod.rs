use bon::Builder;
use city::{BuildCityFrom, CityGenerator};
use common::{
    game::{unit::TaskType, GameFrame},
    geo::Geo,
};
use core::fmt::Debug;
use dyn_clone::DynClone;
use effect::Effect;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    game::{city::City, unit::Unit},
    runner::RunnerContext,
    state::StateError,
};

pub mod city;
pub mod effect;
pub mod unit;

pub type TaskBox = Box<dyn Task + Send + Sync>;

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

pub trait Then {
    fn then(&self, context: &RunnerContext) -> Result<(Vec<Effect>, Vec<TaskBox>), TaskError>;
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
    ) -> Result<(Vec<Effect>, Vec<TaskBox>), TaskError> {
        let city = self.city(context)?;
        let tasks = city.tasks().clone().into();
        let effects = vec![
            effect::remove_unit(self.unit().clone()),
            effect::new_city(city),
        ];

        Ok((effects, tasks))
    }

    fn city(&self, context: &RunnerContext) -> Result<City, TaskError> {
        CityGenerator::builder()
            .context(context)
            .game_frame(context.state().frame())
            .from(BuildCityFrom::Scratch(
                self.city_name().to_string(),
                *self.unit().geo(),
            ))
            .build()
            .generate()
    }
}

// pub trait CityTask: DynClone + Task {
//     fn city_task_type(&self) -> CityTaskType;
//     // See https://users.rust-lang.org/t/reconsider-trait-as-another/123488/5
//     fn into_task(&self) -> TaskBox;
// }
// dyn_clone::clone_trait_object!(CityTask);

// pub trait UnitTask: DynClone + Task {
//     fn unit_task_type(&self) -> UnitTaskType;
//     // See https://users.rust-lang.org/t/reconsider-trait-as-another/123488/5
//     fn into_task(&self) -> TaskBox;
// }
// dyn_clone::clone_trait_object!(UnitTask);

#[derive(Debug, Builder, Clone, PartialEq)]
pub struct TaskContext {
    id: Uuid,
    start: GameFrame,
    end: GameFrame,
}

impl TaskContext {
    pub fn is_finished(&self, frame: GameFrame) -> bool {
        frame >= self.end
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn start(&self) -> GameFrame {
        self.start
    }

    pub fn end(&self) -> GameFrame {
        self.end
    }

    pub fn progress(&self, frame: &GameFrame) -> f32 {
        let total = self.end.0 - self.start.0;
        let current = frame.0 - self.start.0;
        current as f32 / total as f32
    }
}
