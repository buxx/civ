use common::game::tasks::client::ClientTask;

use crate::game::task::settle::Settle as SettleTask;

use super::{Task, TaskBox, TaskContext};

#[derive(Debug, Clone)]
pub enum UnitTaskWrapper {
    Settle(SettleTask),
}

impl UnitTaskWrapper {
    pub fn context(&self) -> &TaskContext {
        match self {
            UnitTaskWrapper::Settle(settle) => settle.context(),
        }
    }
}

impl From<UnitTaskWrapper> for ClientTask {
    fn from(value: UnitTaskWrapper) -> Self {
        match value {
            UnitTaskWrapper::Settle(task) => {
                let context = task.context().clone();
                ClientTask::new(task.into(), context.start(), context.end())
            }
        }
    }
}

impl From<UnitTaskWrapper> for TaskBox {
    fn from(value: UnitTaskWrapper) -> Self {
        Box::new(match value {
            UnitTaskWrapper::Settle(task) => task,
        })
    }
}
