pub mod city;
pub mod settle;

use derive_more::Constructor;
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

    pub fn progress(&self, frame: &GameFrame) -> ClientTaskProgress {
        let total = self.end.0 - self.start.0;
        let current = frame.0 - self.start.0;
        ClientTaskProgress::new(current as f32 / total as f32, self.start, self.end)
    }

    pub fn type_(&self) -> &ClientTaskType {
        &self.type_
    }

    pub fn to_string(&self, frame: &GameFrame) -> String {
        match &self.type_ {
            ClientTaskType::Idle => "Idle".to_string(),
            ClientTaskType::Settle(task) => {
                format!(
                    "{} ({}%)",
                    task,
                    (self.progress(frame).current * 100.0) as u8
                )
            }
        }
    }
}

#[derive(Debug, Constructor, Clone)]
pub struct ClientTaskProgress {
    pub current: f32,
    pub start: GameFrame,
    pub end: GameFrame,
}

impl std::ops::Deref for ClientTaskProgress {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl std::ops::DerefMut for ClientTaskProgress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.current
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClientTaskType {
    Idle,
    Settle(ClientSettle),
}
