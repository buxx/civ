use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::{
        tasks::client::{settle::ClientSettle, ClientTaskType},
        unit::{TaskType, UnitTaskType},
    },
    geo::{Geo, GeoContext},
    task::{CantSettleReason, CreateTaskError, GamePlayReason},
};

use crate::{
    context::Context,
    effect::Effect,
    game::unit::Unit,
    runner::RunnerContext,
    state::State,
    task::{
        unit::UnitTaskWrapper, CityName, Concern, Task, TaskBox, TaskContext, TaskError, TaskId,
        Then, ThenTransformUnitIntoCity, WithUnit,
    },
};

#[derive(Debug, Builder, Clone)]
pub struct Settle {
    context: TaskContext,
    geo: GeoContext,
    settler: Box<Unit>,
    city_name: String,
}

impl Settle {
    pub fn new(
        task_id: TaskId,
        context: Context,
        state: RwLockReadGuard<State>,
        settler: Unit,
        city_name: String,
    ) -> Result<Self, CreateTaskError> {
        if !context.rules().can_settle(settler.type_()) {
            return Err(CreateTaskError::GamePlay(GamePlayReason::CantSettle(
                CantSettleReason::WrongUnitType(*settler.type_()),
            )));
        }

        let end = *state.frame() + context.rules().settle_duration(settler.type_()).0;
        let task = Settle::builder()
            .geo(*settler.geo())
            .settler(Box::new(settler))
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

impl Geo for Settle {
    fn geo(&self) -> &GeoContext {
        &self.geo
    }

    fn geo_mut(&mut self) -> &mut GeoContext {
        &mut self.geo
    }
}

impl WithUnit for Settle {
    fn unit(&self) -> &Unit {
        &self.settler
    }
}

impl CityName for Settle {
    fn city_name(&self) -> &str {
        &self.city_name
    }
}

impl ThenTransformUnitIntoCity for Settle {}

impl Then for Settle {
    fn then(&self, context: &RunnerContext) -> Result<(Vec<Effect>, Vec<TaskBox>), TaskError> {
        self.transform_unit_into_city(context)
    }
}

impl Task for Settle {
    fn type_(&self) -> TaskType {
        TaskType::Unit(UnitTaskType::Settle)
    }

    fn context(&self) -> &TaskContext {
        &self.context
    }

    fn concern(&self) -> Concern {
        Concern::Unit(*self.settler.id())
    }
}

impl From<Settle> for ClientTaskType {
    fn from(value: Settle) -> Self {
        ClientTaskType::Settle(ClientSettle::new(value.city_name.to_string()))
    }
}

impl From<Settle> for UnitTaskWrapper {
    fn from(value: Settle) -> Self {
        UnitTaskWrapper::Settle(value)
    }
}
