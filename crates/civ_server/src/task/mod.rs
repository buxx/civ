pub mod create;

use common::{
    game::{
        slice::ClientTask,
        unit::{TaskType, UnitTaskType},
        GameFrame,
    },
    geo::Geo,
};
use context::TaskContext;
use core::fmt::Debug;
use dyn_clone::DynClone;
use effect::{CityEffect, Effect, StateEffect, UnitEffect};
use uuid::Uuid;

use crate::game::{city::City, unit::Unit};

pub mod context;
pub mod effect;

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

pub enum Concern {
    Nothing,
    Unit(Uuid),
    City(Uuid),
}

pub trait IntoClientTask {
    fn into_client(&self) -> ClientTask;
}

impl IntoClientTask for TaskBox {
    fn into_client(&self) -> ClientTask {
        ClientTask::new(
            self.context().id(),
            TaskType::Unit(UnitTaskType::Settle), // FIXME
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

pub trait Then {
    fn then(&self) -> (Vec<Effect>, Vec<TaskBox>);
}

pub trait WithUnit {
    fn unit(&self) -> &Unit;
}

pub trait CityName {
    fn city_name(&self) -> &str;
}

pub trait ThenTransformUnitIntoCity: WithUnit + CityName + Geo {
    fn transform_unit_into_city(&self) -> (Vec<Effect>, Vec<TaskBox>) {
        let unit = self.unit();
        let city_id = Uuid::new_v4();
        let city = City::builder()
            .id(city_id)
            .name(self.city_name().to_string())
            .geo(*self.geo())
            .build();

        (
            vec![
                Effect::State(StateEffect::Unit(
                    unit.id(),
                    UnitEffect::Remove(unit.clone()),
                )),
                Effect::State(StateEffect::City(city_id, CityEffect::New(city))),
            ],
            vec![],
        )
    }
}
