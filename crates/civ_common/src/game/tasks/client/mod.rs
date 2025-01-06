pub mod city;
pub mod settle;

use serde::{Deserialize, Serialize};
use settle::ClientSettle;

use crate::game::GameFrame;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientTask {
    type_: ClientTaskType,
    start: GameFrame,
    end: GameFrame,
}

impl ClientTask {
    pub fn new(type_: ClientTaskType, start: GameFrame, end: GameFrame) -> Self {
        Self { type_, start, end }
    }

    pub fn progress(&self, frame: &GameFrame) -> f32 {
        let total = self.end.0 - self.start.0;
        let current = frame.0 - self.start.0;
        current as f32 / total as f32
    }

    pub fn type_(&self) -> &ClientTaskType {
        &self.type_
    }

    pub fn to_string(&self, frame: &GameFrame) -> String {
        match &self.type_ {
            ClientTaskType::Settle(task) => {
                format!("{} ({}%)", task, (self.progress(frame) * 100.0) as u8)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClientTaskType {
    Settle(ClientSettle),
}
