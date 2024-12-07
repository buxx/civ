use bon::Builder;
use uuid::Uuid;

use crate::state::GameFrame;

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

#[derive(Builder, Clone)]
pub struct PhysicalContext {
    x: u64,
    y: u64,
}
