use crate::{
    effect::{Effect, StateEffect},
    impl_boxed, impl_with_context,
    runner::RunnerContext,
    task::{Concern, Task, TaskBox, TaskContext, TaskError, TaskId, Then},
};
use common::game::{unit::TaskType, GameFrame};
use serde::{Deserialize, Serialize};

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FibonacciTask {
    context: TaskContext,
    complexity: u64,
}

impl FibonacciTask {
    pub fn new(context: TaskContext, complexity: u64) -> Self {
        Self {
            context,
            complexity,
        }
    }
}

impl_boxed!(FibonacciTask);
impl_with_context!(FibonacciTask);

#[typetag::serde]
impl Task for FibonacciTask {
    fn type_(&self) -> TaskType {
        TaskType::Testing
    }

    fn concern(&self) -> Concern {
        Concern::Nothing
    }

    fn tick(&self, _frame: GameFrame) -> Vec<Effect> {
        fibonacci(self.complexity);
        vec![Effect::State(StateEffect::Testing)]
    }
}

impl Then for FibonacciTask {
    fn then(&self, _context: &RunnerContext) -> Result<(Vec<Effect>, Vec<TaskBox>), TaskError> {
        Ok((vec![], vec![]))
    }
}

pub fn build_task() -> Box<dyn Task> {
    Box::new(FibonacciTask::new(
        TaskContext::builder()
            .id(TaskId::default())
            .start(GameFrame(0))
            .end(GameFrame(1_000_000_000))
            .build(),
        1,
    ))
}
