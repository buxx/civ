use common::space::Window;

use crate::runner::RunnerContext;

pub struct SetWindowRequestDealer {
    // TODO: pas contexte mais que le necessaire a cette action ?
    context: RunnerContext,
}

impl SetWindowRequestDealer {
    pub fn new(context: RunnerContext) -> Self {
        Self { context }
    }

    pub fn deal(&self, window: &Window) {
        todo!()
    }
}
