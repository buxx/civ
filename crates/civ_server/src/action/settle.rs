use super::{Action, Effect};

pub struct Settle {}

impl Action for Settle {
    fn tick(&self, frame: crate::state::GameFrame) -> Vec<Effect> {
        todo!()
    }
}
