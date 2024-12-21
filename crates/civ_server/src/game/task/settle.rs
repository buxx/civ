use std::sync::RwLockReadGuard;

use bon::Builder;
use common::{
    game::unit::{TaskType, UnitTaskType},
    geo::{Geo, GeoContext},
    task::{CreateTaskError, GamePlayError},
};
use uuid::Uuid;

use crate::{
    context::Context,
    game::unit::Unit,
    state::State,
    task::{
        context::TaskContext, effect::Effect, CityName, Concern, Task, TaskBox, Then,
        ThenTransformUnitIntoCity, WithUnit,
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
    fn then(&self) -> (Vec<Effect>, Vec<TaskBox>) {
        self.transform_unit_into_city()
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
        Concern::Unit(self.settler.id())
    }
}
