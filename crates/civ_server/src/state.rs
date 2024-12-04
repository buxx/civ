use std::ops::{Add, AddAssign};

pub const GAME_FRAMES_PER_SECOND: u64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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

#[derive(Default)]
pub struct State {
    frame_i: GameFrame,
}

impl State {
    pub fn frame(&self) -> &GameFrame {
        &self.frame_i
    }

    pub fn increment(&mut self) {
        self.frame_i += GameFrame(1);
    }
}
