use bon::Builder;

use super::{Task, TaskContext, Effect};

#[derive(Builder)]
pub struct Settle {
    context: TaskContext,
}

impl Settle {}

impl Task for Settle {
    fn tick_(&self, _frame: crate::state::GameFrame) -> Vec<Effect> {
        vec![]
    }

    fn context(&self) -> &TaskContext {
        &self.context
    }
}
