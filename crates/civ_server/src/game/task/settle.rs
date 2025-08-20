use std::sync::RwLockReadGuard;

use bon::Builder;
use civ_derive::Geo;
use common::{
    game::{
        tasks::client::{settle::ClientSettle, ClientTaskType},
        unit::{TaskType, UnitTaskType},
        GameFrame,
    },
    geo::{Geo, GeoContext},
    task::{CantSettleReason, CreateTaskError, GamePlayReason},
};
use serde::{Deserialize, Serialize};

use crate::{
    context::Context,
    effect::Effect,
    game::unit::Unit,
    impl_boxed, impl_into_unit_task_wrapper, impl_then_transform_unit_into_city,
    impl_with_city_name, impl_with_context, impl_with_unit,
    runner::RunnerContext,
    state::State,
    task::{
        unit::UnitTaskWrapper, Concern, Task, TaskBox, TaskContext, TaskError, TaskId, Then,
        ThenTransformUnitIntoCity, WithCityName, WithUnit,
    },
};

#[derive(Debug, Builder, Clone, Serialize, Deserialize, Geo)]
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

impl_boxed!(Settle);
impl_with_context!(Settle);
impl_with_unit!(Settle, settler);
impl_with_city_name!(Settle, city_name);
impl_then_transform_unit_into_city!(Settle);
impl_into_unit_task_wrapper!(Settle, UnitTaskWrapper::Settle);

#[typetag::serde]
impl Task for Settle {
    fn type_(&self) -> TaskType {
        TaskType::Unit(UnitTaskType::Settle)
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
