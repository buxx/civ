use std::path::PathBuf;

use super::{Concern, Task, TaskBox, TaskContext, TaskError, TaskId, Then};
use crate::{effect::Effect, runner::RunnerContext};
use bon::Builder;
use common::game::{
    unit::{SystemTaskType, TaskType},
    GameFrame,
};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct SnapshotTask {
    context: TaskContext,
    snapshot_to: PathBuf,
    each: GameFrame,
}

impl SnapshotTask {
    pub fn new(context: TaskContext, snapshot_to: PathBuf, each: GameFrame) -> Self {
        Self {
            context,
            snapshot_to,
            each,
        }
    }
}

#[typetag::serde]
impl Task for SnapshotTask {
    fn type_(&self) -> TaskType {
        TaskType::System(SystemTaskType::Snapshot)
    }

    fn concern(&self) -> Concern {
        Concern::Nothing
    }

    fn context(&self) -> &TaskContext {
        &self.context
    }

    fn boxed(&self) -> TaskBox {
        Box::new(self.clone())
    }
}

impl Then for SnapshotTask {
    fn then(&self, context: &RunnerContext) -> Result<(Vec<Effect>, Vec<TaskBox>), TaskError> {
        let state = context.state();
        let frame = state.frame();

        info!("Snapshot to {}", self.snapshot_to.display());
        state.snapshot().dump(&self.snapshot_to).unwrap();

        Ok((
            vec![],
            vec![Box::new(Self::new(
                TaskContext::builder()
                    .id(TaskId::default())
                    .start(*frame)
                    .end(*frame + self.each.0)
                    .build(),
                self.snapshot_to.clone(),
                self.each,
            ))],
        ))
    }
}
