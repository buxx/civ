use bon::Builder;
use uuid::Uuid;

use crate::state::GameFrame;

pub mod settle;

pub trait Task {
    fn tick(&self, frame: GameFrame) -> Vec<Effect> {
        let mut effects = self.tick_(frame);

        if self.context().is_finished(frame) {
            effects.push(Effect::TaskFinished(self.context().id));
        }

        effects
    }
    fn tick_(&self, frame: GameFrame) -> Vec<Effect>;
    fn context(&self) -> &TaskContext;
}

#[derive(Builder)]
pub struct TaskContext {
    id: Uuid,
    start: GameFrame,
    end: GameFrame,
}

impl TaskContext {
    pub fn is_finished(&self, frame: GameFrame) -> bool {
        frame >= self.end
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn start(&self) -> GameFrame {
        self.start
    }

    pub fn end(&self) -> GameFrame {
        self.end
    }
}

pub enum Effect {
    TaskFinished(Uuid),
}
