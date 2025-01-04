use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};
use uuid::Uuid;

pub mod city;
pub mod slice;
pub mod unit;

pub const GAME_FRAMES_PER_SECOND: u64 = 10;
pub const PRODUCTION_TON_FRAMES: u64 = GAME_FRAMES_PER_SECOND * 10 * 60; // Number of frames to produce 1 prod ton
pub const FRAME_PRODUCTION_TONS_RATIO: f64 = 1.0 / PRODUCTION_TON_FRAMES as f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct GameFrame(pub u64);

impl Add<u64> for GameFrame {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign for GameFrame {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClientTasks<T: ClientTask> {
    stack: Vec<T>,
}

impl<T: ClientTask> ClientTasks<T> {
    pub fn new(stack: Vec<T>) -> Self {
        Self { stack }
    }

    pub fn push(&mut self, task: T) {
        self.stack.push(task);
    }

    pub fn remove(&mut self, uuid: &Uuid) {
        self.stack.retain(|t| t.id() != uuid);
    }

    pub fn display(&self, frame: &GameFrame) -> String {
        if self.stack.is_empty() {
            return "Idle".into();
        }

        self.stack
            .iter()
            .map(|t| t.display(frame))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

pub trait ClientTask {
    fn id(&self) -> &Uuid;
    fn display(&self, frame: &GameFrame) -> String;
}
