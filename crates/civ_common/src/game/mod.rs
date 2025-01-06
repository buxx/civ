pub mod tasks;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};

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
