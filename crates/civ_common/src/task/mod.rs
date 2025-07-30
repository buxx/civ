pub mod snapshot;
use bon::Builder;
use city::{BuildCityFrom, CityGenerator};
use common::{
    game::{
        city::{City, CityId},
        unit::{TaskType, UnitId},
        GameFrame,
    },
    geo::Geo,
};
use core::fmt::Debug;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    effect::{self, Effect},
    game::{task::settle::Settle, unit::Unit},
    runner::RunnerContext,
    state::StateError,
};

pub mod city;
pub mod unit;

#[derive(Error, Debug)]
pub enum CreateTaskError {
    #[error("Action is not possible: {0}")]
    GamePlay(GamePlayReason),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

#[derive(Error, Debug)]
pub enum GamePlayReason {
    #[error("Cant settle: {0}")]
    CantSettle(CantSettleReason),
    #[error("City no longer exist")]
    CityNoLongerExist,
    #[error("Unit no longer exist")]
    UnitNoLongerExist,
    #[error("Player no longer exist")]
    PlayerNoLongerExist,
}

#[derive(Error, Debug)]
pub enum CantSettleReason {
    #[error("{0} can't settle")]
    WrongUnitType(UnitType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

impl Default for TaskId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

pub type TaskBox = Box<dyn Task + Send + Sync>;

#[typetag::serde(tag = "type")]
pub trait Task: DynClone + Then {
    fn type_(&self) -> TaskType;
    fn concern(&self) -> Concern;
    fn tick(&self, _frame: GameFrame) -> Vec<Effect> {
        vec![]
    }
    fn context(&self) -> &TaskContext;
    fn boxed(&self) -> TaskBox;
}
dyn_clone::clone_trait_object!(Task);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Concern {
    Nothing,
    Unit(UnitId),
    City(CityId),
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
                *self.unit().flag(),
                *self.unit().geo(),
            ))
            .build()
            .generate()
    }
}
#[derive(Debug, Builder, Clone, PartialEq, Serialize, Deserialize)]

pub struct TaskContext {
    id: TaskId,
    start: GameFrame,
    end: GameFrame,
}

impl TaskContext {
    pub fn is_finished(&self, frame: GameFrame) -> bool {
        frame >= self.end
    }

    pub fn id(&self) -> &TaskId {
        &self.id
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

pub enum TaskContainer {
    Unit(UnitTaskContainer),
    City(CityTaskContainer),
    Empty,
}
pub enum UnitTaskContainer {
    Settle(Settle),
}

pub enum CityTaskContainer {
    Production(CityProductionTask),
}
