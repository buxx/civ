use bon::Builder;

use crate::state::GameFrame;

use super::{Effect, PhysicalContext, Task, TaskContext, TaskType};

#[derive(Builder)]
pub struct Settle {
    context: TaskContext,
    physic: PhysicalContext,
}

impl Settle {}

impl Task for Settle {
    fn tick_(&self, _frame: GameFrame) -> Vec<Effect> {
        vec![]
    }

    fn context(&self) -> &TaskContext {
        &self.context
    }

    fn type_(&self) -> TaskType {
        TaskType::Physical(self.physic.clone())
    }
}
