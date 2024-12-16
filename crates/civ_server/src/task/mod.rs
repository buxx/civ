pub mod create;
use common::game::GameFrame;
use context::{PhysicalContext, TaskContext};
use effect::{Effect, StateEffect, TaskEffect};

pub mod context;
pub mod effect;

pub trait Task {
    fn tick(&self, frame: GameFrame) -> Vec<Effect> {
        let mut effects = self.tick_(frame);

        if self.context().is_finished(frame) {
            effects.push(Effect::State(StateEffect::Task(
                self.context().id(),
                TaskEffect::Finished,
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
    fn tick_(&self, frame: GameFrame) -> Vec<Effect>;
    fn context(&self) -> &TaskContext;
    fn type_(&self) -> TaskType;
    fn then(&self) -> (Vec<Effect>, Vec<Box<dyn Task + Send>>) {
        (vec![], vec![])
    }
}

pub enum TaskType<'a> {
    Physical(&'a PhysicalContext),
}
