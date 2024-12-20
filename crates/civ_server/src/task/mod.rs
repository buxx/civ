pub mod create;

use common::game::{slice::ClientTask, unit::TaskType, GameFrame};
use context::TaskContext;
use core::fmt::Debug;
use dyn_clone::DynClone;
use effect::{Effect, StateEffect, TaskEffect};
use uuid::Uuid;

pub mod context;
pub mod effect;

pub type TaskBox = Box<dyn Task + Send + Sync>;

pub trait Task: DynClone {
    fn type_(&self) -> TaskType;
    fn tick(&self, frame: GameFrame) -> Vec<Effect> {
        let mut effects = self.tick_(frame);

        if self.context().is_finished(frame) {
            effects.push(Effect::State(StateEffect::Task(
                self.context().id(),
                TaskEffect::Finished(self.context().id()),
            )));

            let (then_effects, then_tasks) = self.then();
            effects.extend(then_effects);

            for task in then_tasks {
                effects.push(Effect::State(StateEffect::Task(
                    task.context().id(),
                    TaskEffect::Push(task),
                )));
            }
        }

        effects
    }
    fn concern(&self) -> Concern;
    fn tick_(&self, frame: GameFrame) -> Vec<Effect>;
    fn context(&self) -> &TaskContext;
    fn then(&self) -> (Vec<Effect>, Vec<TaskBox>) {
        (vec![], vec![])
    }
}
dyn_clone::clone_trait_object!(Task);

pub enum Concern {
    Nothing,
    Unit(Uuid),
    City(Uuid),
}

pub trait IntoClientTask {
    fn into_client(&self) -> ClientTask;
}

impl IntoClientTask for TaskBox {
    fn into_client(&self) -> ClientTask {
        ClientTask::new(
            self.context().id(),
            self.type_().clone(),
            self.context().start(),
            self.context().end(),
        )
    }
}

impl Debug for TaskBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TaskBox")
            .field(&self.type_())
            .field(&self.context())
            .finish()
    }
}
