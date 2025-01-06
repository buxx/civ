use common::game::tasks::client::ClientTask;

use crate::game::task::settle::Settle as SettleTask;

use super::Task;

#[derive(Debug, Clone)]
pub enum UnitTaskWrapper {
    Idle,
    Settle(SettleTask),
}

impl From<UnitTaskWrapper> for ClientTask {
    fn from(value: UnitTaskWrapper) -> Self {
        match value {
            UnitTaskWrapper::Idle => todo!(),
            UnitTaskWrapper::Settle(task) => {
                let context = task.context().clone();
                ClientTask::new(task.into(), context.start(), context.end())
            }
        }
    }
}
