use std::sync::{Arc, RwLock};

use context::Context;
use runner::Runner;
use state::State;

mod action;
mod context;
mod runner;
mod state;

pub const TICK_BASE_PERIOD: u64 = 60;

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let context = Arc::new(RwLock::new(Context::new()));
    let state = Arc::new(RwLock::new(State::default()));
    let mut runner = Runner::new(TICK_BASE_PERIOD, context, state);
    runner.run();
}
