use common::game::tasks::client::ClientTask;
use serde::{Deserialize, Serialize};

use crate::{game::task::settle::Settle as SettleTask, task::WithContext};

use super::{TaskBox, TaskContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[macro_export]
macro_rules! impl_into_unit_task_wrapper {
    ($type:ty, $variant:expr) => {
        impl From<$type> for $crate::task::unit::UnitTaskWrapper {
            fn from(value: $type) -> Self {
                $variant(value)
            }
        }
    };
}
