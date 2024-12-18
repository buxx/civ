pub mod create;
use common::game::{unit::UnitTask, GameFrame};
use context::TaskContext;
use effect::{Effect, StateEffect, TaskEffect};
use uuid::Uuid;

pub mod context;
pub mod effect;

pub trait Task {
    fn type_(&self) -> UnitTask;
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
    fn concerned_unit(&self) -> Option<Uuid>;
    fn concerned_city(&self) -> Option<Uuid>;
    fn tick_(&self, frame: GameFrame) -> Vec<Effect>;
    fn context(&self) -> &TaskContext;
    fn then(&self) -> (Vec<Effect>, Vec<Box<dyn Task + Send>>) {
        (vec![], vec![])
    }
}
