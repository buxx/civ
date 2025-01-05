use serde::{Deserialize, Serialize};

use crate::game::GameFrame;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientCityProductionTask {
    start: GameFrame,
    end: GameFrame,
}

impl ClientCityProductionTask {
    pub fn new(start: GameFrame, end: GameFrame) -> Self {
        Self { start, end }
    }

    pub fn progress(&self, frame: &GameFrame) -> f32 {
        let total = self.end.0 - self.start.0;
        let current = frame.0 - self.start.0;
        current as f32 / total as f32
    }

    pub fn start(&self) -> GameFrame {
        self.start
    }

    pub fn end(&self) -> GameFrame {
        self.end
    }
}
