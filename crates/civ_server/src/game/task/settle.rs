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
    game::city::City,
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
    settler: Uuid,
    city_name: String,
}

impl Settle {
    pub fn new(
        task_id: Uuid,
        context: Context,
        state: RwLockReadGuard<State>,
        unit_uuid: &Uuid,
        city_name: String,
    ) -> Result<Self, CreateTaskError> {
        let unit = state.find_unit(unit_uuid).map_err(|e| {
            CreateTaskError::IncoherentContext(
                "Unit not available anymore".to_string(),
                Some(Box::new(e)),
            )
        })?;

        if !context.rules().can_settle(unit.type_()) {
            return Err(CreateTaskError::GamePlay(GamePlayError::CantSettle(
                format!("{} cant do this action", unit.type_()),
            )));
        }

        let end = *state.frame() + context.rules().settle_duration(unit.type_()).0;
        let task = Settle::builder()
            .settler(*unit_uuid)
            .city_name(city_name)
            .geo(*unit.geo())
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
                    self.settler,
                    UnitEffect::Remove(self.settler),
                )),
                Effect::State(StateEffect::City(self.settler, CityEffect::New(city))),
            ],
            vec![],
        )
    }

    fn concern(&self) -> Concern {
        Concern::Unit(self.settler)
    }
}
