use bon::Builder;
use common::game::{
    city::CityProductionTons,
    unit::{CityTaskType, TaskType},
};
use uuid::Uuid;

use crate::{
    runner::RunnerContext,
    task::{context::TaskContext, effect::Effect, Concern, Task, TaskBox, TaskError, Then},
};

#[derive(Builder, Clone)]
pub struct CityProductionTask {
    context: TaskContext,
    city: Uuid,
    tons: CityProductionTons,
}

impl CityProductionTask {
    pub fn tons(&self) -> CityProductionTons {
        self.tons
    }
}

impl Task for CityProductionTask {
    fn type_(&self) -> TaskType {
        TaskType::City(CityTaskType::Production(self.tons))
    }

    fn concern(&self) -> Concern {
        Concern::City(self.city)
    }

    fn context(&self) -> &TaskContext {
        &self.context
    }
}

impl Then for CityProductionTask {
    fn then(&self, _context: &RunnerContext) -> Result<(Vec<Effect>, Vec<TaskBox>), TaskError> {
        todo!()
    }
}
