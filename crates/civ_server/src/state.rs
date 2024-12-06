use std::ops::{Add, AddAssign};

use crate::action::{settle::Settle, Action, Effect};

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
    actions: Vec<Box<dyn Action + Send>>,
}

impl State {
    pub fn frame(&self) -> &GameFrame {
        &self.frame_i
    }

    pub fn increment(&mut self) {
        self.frame_i += GameFrame(1);

        // HACK
        if self.frame_i.0 == 19 {
            self.actions.push(Box::new(Settle::new()))
        }
    }

    pub fn actions(&self) -> Vec<&Box<dyn Action>> {
        vec![]
    }

    pub fn apply(&mut self, effect: Effect) {}
}
