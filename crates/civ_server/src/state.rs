use std::ops::{Add, AddAssign};

use uuid::Uuid;

use crate::action::{settle::Settle, Action, ActionContext, Effect};

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
            for _ in 0..1_000 {
                self.actions.push(Box::new(
                    Settle::builder()
                        .context(
                            ActionContext::builder()
                                .id(Uuid::new_v4())
                                .start(self.frame_i)
                                .end(self.frame_i + GAME_FRAMES_PER_SECOND * 5)
                                .build(),
                        )
                        .build(),
                ))
            }
        }
    }

    pub fn actions(&self) -> &Vec<Box<dyn Action + Send>> {
        &self.actions
    }

    pub fn apply(&mut self, effects: Vec<Effect>) {
        let mut remove_ids = vec![];

        for effect in effects {
            match effect {
                Effect::ActionFinished(uuid) => remove_ids.push(uuid),
            }
        }

        if !remove_ids.is_empty() {
            // TODO: this is not a good performance way (idea: transport actions index in tick)
            self.actions
                .retain(|action| !remove_ids.contains(&action.context().id()));
        }
    }
}
