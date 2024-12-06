use bon::Builder;

use super::{Action, ActionContext, Effect};

#[derive(Builder)]
pub struct Settle {
    context: ActionContext,
}

impl Settle {}

impl Action for Settle {
    fn tick_(&self, _frame: crate::state::GameFrame) -> Vec<Effect> {
        vec![]
    }

    fn context(&self) -> &ActionContext {
        &self.context
    }
}
