use bon::Builder;
use common::game::{
    city::{CityId, CityProductionTons},
    tasks::client::city::production::ClientCityProductionTask,
    unit::{CityTaskType, TaskType},
};
use serde::{Deserialize, Serialize};

use crate::{
    effect::Effect,
    runner::RunnerContext,
    task::{Concern, Task, TaskBox, TaskContext, TaskError, Then},
};

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct CityProductionTask {
    context: TaskContext,
    city: CityId,
    tons: CityProductionTons,
}

impl CityProductionTask {
    pub fn _tons(&self) -> CityProductionTons {
        self.tons
    }
}

#[typetag::serde]
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

    fn boxed(&self) -> TaskBox {
        Box::new(self.clone())
    }
}

impl Then for CityProductionTask {
    fn then(&self, _context: &RunnerContext) -> Result<(Vec<Effect>, Vec<TaskBox>), TaskError> {
        todo!()
    }
}

impl From<CityProductionTask> for ClientCityProductionTask {
    fn from(value: CityProductionTask) -> Self {
        let context = value.context();
        ClientCityProductionTask::new(context.start(), context.end())
    }
}
