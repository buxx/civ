use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        unit::{TaskType, UnitTaskType},
        GameFrame,
    },
    geo::{Geo, GeoContext},
    task::{CreateTaskError, GamePlayError},
};
use uuid::Uuid;

use crate::{
    context::Context,
    game::{city::City, unit::Unit},
    state::State,
    task::{
        context::TaskContext,
        effect::{CityEffect, Effect, StateEffect, UnitEffect},
        Concern, Task, TaskBox,
    },
};

#[derive(Builder, Clone)]
pub struct Settle {
    context: TaskContext,
    geo: GeoContext,
    settler: Unit,
    city_name: String,
}

impl Settle {
    pub fn new(
        task_id: Uuid,
        context: Context,
        state: RwLockReadGuard<State>,
        settler: Unit,
        city_name: String,
    ) -> Result<Self, CreateTaskError> {
        if !context.rules().can_settle(settler.type_()) {
            return Err(CreateTaskError::GamePlay(GamePlayError::CantSettle(
                format!("{} cant do this action", settler.type_()),
            )));
        }

        let end = *state.frame() + context.rules().settle_duration(settler.type_()).0;
        let task = Settle::builder()
            .geo(*settler.geo())
            .settler(settler)
            .city_name(city_name)
            .context(
                TaskContext::builder()
                    .id(task_id)
                    .start(*state.frame())
                    .end(end)
                    .build(),
            )
            .build();
        Ok(task)
    }
}

impl Task for Settle {
    fn type_(&self) -> TaskType {
        TaskType::Unit(UnitTaskType::Settle)
    }

    fn tick(&self, _frame: GameFrame) -> Vec<Effect> {
        vec![]
    }

    fn context(&self) -> &TaskContext {
        &self.context
    }

    fn then(&self) -> (Vec<Effect>, Vec<TaskBox>) {
        let city_id = Uuid::new_v4();
        let city = City::builder()
            .id(city_id)
            .name(self.city_name.clone())
            .geo(self.geo)
            .build();

        (
            vec![
                Effect::State(StateEffect::Unit(
                    self.settler.id(),
                    UnitEffect::Remove(self.settler.clone()),
                )),
                Effect::State(StateEffect::City(city_id, CityEffect::New(city))),
            ],
            vec![],
        )
    }

    fn concern(&self) -> Concern {
        Concern::Unit(self.settler.id())
    }
}
