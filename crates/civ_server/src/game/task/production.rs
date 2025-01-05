use bon::Builder;
use common::game::{
    city::CityProductionTons,
    tasks::client::city::production::ClientCityProductionTask,
    unit::{CityTaskType, TaskType},
};
use uuid::Uuid;

use crate::{
    runner::RunnerContext,
    task::{effect::Effect, Concern, Task, TaskBox, TaskContext, TaskError, Then},
};

#[derive(Debug, Builder, Clone)]
pub struct CityProductionTask {
    context: TaskContext,
    city: Uuid,
    tons: CityProductionTons,
}

impl CityProductionTask {
    pub fn _tons(&self) -> CityProductionTons {
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

// impl CityTask for CityProductionTask {
//     fn city_task_type(&self) -> CityTaskType {
//         CityTaskType::Production(self.tons)
//     }

//     fn into_task(&self) -> TaskBox {
//         Box::new(self.clone())
//     }
// }

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
