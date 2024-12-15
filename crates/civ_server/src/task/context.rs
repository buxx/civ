use bon::Builder;
use common::space::context::ClientPhysicalContext;
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

impl PhysicalContext {
    pub fn xy(&self) -> (u64, u64) {
        (self.x, self.y)
    }

    pub fn set_xy(&mut self, value: (u64, u64)) {
        self.x = value.0;
        self.y = value.1;
    }
}

impl Into<ClientPhysicalContext> for PhysicalContext {
    fn into(self) -> ClientPhysicalContext {
        ClientPhysicalContext::new(self.x, self.y)
    }
}
