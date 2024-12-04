use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FrameI(pub u64);

impl Add<u64> for FrameI {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign for FrameI {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

#[derive(Default)]
pub struct State {
    frame_i: FrameI,
}

impl State {
    pub fn frame_i(&self) -> &FrameI {
        &self.frame_i
    }

    pub fn increment(&mut self) {
        self.frame_i += FrameI(1);
    }
}
