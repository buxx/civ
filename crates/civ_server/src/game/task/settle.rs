use std::sync::MutexGuard;

use bon::Builder;
use common::{
    game::GameFrame,
    task::{CreateTaskError, GamePlayError},
};
use uuid::Uuid;

use crate::{
    context::Context,
    game::{city::City, physics::Geo},
    state::State,
    task::{
        context::{GeoContext, TaskContext},
        effect::{CityEffect, Effect, StateEffect, UnitEffect},
        Task, TaskType,
    },
};

#[derive(Builder)]
pub struct Settle {
    context: TaskContext,
    physic: GeoContext,
    settler: Uuid,
    city_name: String,
}

impl Settle {
    pub fn new(
        context: Context,
        state: MutexGuard<State>,
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

        let task_id = Uuid::new_v4();
        let end = *state.frame() + context.rules().settle_duration(unit.type_()).0;
        let task = Settle::builder()
            .settler(*unit_uuid)
            .city_name(city_name)
            .physic(unit.geo().clone())
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
    fn tick_(&self, _frame: GameFrame) -> Vec<Effect> {
        vec![]
    }

    fn context(&self) -> &TaskContext {
        &self.context
    }

    fn type_(&self) -> TaskType {
        TaskType::Physical(&self.physic)
    }

    fn then(&self) -> (Vec<Effect>, Vec<Box<dyn Task + Send>>) {
        let city_id = Uuid::new_v4();
        let city = City::builder()
            .id(city_id)
            .name(self.city_name.clone())
            .geo(self.physic.clone())
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
}
