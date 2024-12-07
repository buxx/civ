use std::ops::{Add, AddAssign};

use uuid::Uuid;

use crate::{
    game::{city::City, task::settle::Settle},
    task::{
        context::{PhysicalContext, TaskContext},
        effect::Effect,
        Task,
    },
};

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
    tasks: Vec<Box<dyn Task + Send>>,
    cities: Vec<City>,
}

impl State {
    pub fn frame(&self) -> &GameFrame {
        &self.frame_i
    }

    pub fn increment(&mut self) {
        self.frame_i += GameFrame(1);

        // HACK
        if self.frame_i.0 == 0 {
            for x in 0..100 {
                for y in 0..100 {
                    self.cities.push(
                        City::builder()
                            .id(Uuid::new_v4())
                            .physics(PhysicalContext::builder().x(x * 5).y(y * 5).build())
                            .build(),
                    );
                }
            }
        }
        if self.frame_i.0 == 19 {
            for _ in 0..1_000 {
                self.tasks.push(Box::new(
                    Settle::builder()
                        .context(
                            TaskContext::builder()
                                .id(Uuid::new_v4())
                                .start(self.frame_i)
                                .end(self.frame_i + GAME_FRAMES_PER_SECOND * 5)
                                .build(),
                        )
                        .physic(PhysicalContext::builder().x(0).y(0).build())
                        .build(),
                ))
            }
        }
    }

    pub fn tasks(&self) -> &Vec<Box<dyn Task + Send>> {
        &self.tasks
    }

    pub fn apply(&mut self, effects: Vec<Effect>) {
        let mut remove_ids = vec![];

        for effect in effects {
            match effect {
                Effect::TaskFinished(uuid) => remove_ids.push(uuid),
            }
        }

        if !remove_ids.is_empty() {
            // TODO: this is not a good performance way (idea: transport tasks index in tick)
            self.tasks
                .retain(|task| !remove_ids.contains(&task.context().id()));
        }
    }
}
