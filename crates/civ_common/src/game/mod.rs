pub mod nation;
pub mod server;
pub mod tasks;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};
use uuid::Uuid;

pub mod city;
pub mod slice;
pub mod unit;

pub const GAME_FRAMES_PER_SECOND: u64 = 10;
pub const PRODUCTION_FRAMES_PER_TONS: u64 = GAME_FRAMES_PER_SECOND * 10 * 60; // Number of frames to produce 1 prod ton
pub const FRAME_PRODUCTION_TONS_RATIO: f64 = 1.0 / PRODUCTION_FRAMES_PER_TONS as f64;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub Uuid);

impl FromStr for PlayerId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

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

impl Sub for GameFrame {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
