use super::{Action, Effect};

pub struct Settle {}
impl Settle {
    pub fn new() -> Self {
        Self {}
    }
}

impl Action for Settle {
    fn tick(&self, frame: crate::state::GameFrame) -> Vec<Effect> {
        todo!()
    }
}
