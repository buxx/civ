use crate::state::GameFrame;

pub mod settle;

pub trait Action {
    fn tick(&self, frame: GameFrame) -> Vec<Effect>;
}

pub enum Effect {}
