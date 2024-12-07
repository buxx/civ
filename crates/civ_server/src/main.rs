use std::sync::Mutex;

use context::Context;
use runner::Runner;
use state::State;

mod context;
mod game;
mod runner;
mod state;
mod task;
mod utils;

pub const TICK_BASE_PERIOD: u64 = 60;

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let context = Mutex::new(Context::new());
    let state = Mutex::new(State::default());
    let mut runner = Runner::new(TICK_BASE_PERIOD, context, state);
    runner.run();
}
