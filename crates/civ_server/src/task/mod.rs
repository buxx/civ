use context::{PhysicalContext, TaskContext};
use effect::Effect;

use crate::state::GameFrame;

pub mod context;
pub mod effect;

pub trait Task {
    fn tick(&self, frame: GameFrame) -> Vec<Effect> {
        let mut effects = self.tick_(frame);

        if self.context().is_finished(frame) {
            effects.push(Effect::TaskFinished(self.context().id()));
        }

        effects
    }
    fn tick_(&self, frame: GameFrame) -> Vec<Effect>;
    fn context(&self) -> &TaskContext;
    fn type_(&self) -> TaskType;
}

pub enum TaskType<'a> {
    Physical(&'a PhysicalContext),
}
