use std::sync::{Arc, RwLock};

use context::Context;
use runner::Runner;
use state::State;

mod context;
mod runner;
mod state;

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let tick_base_period: u64 = 100;
    let context = Arc::new(RwLock::new(Context::new()));
    let state = Arc::new(RwLock::new(State::default()));
    let mut runner = Runner::new(tick_base_period, context, state);
    runner.run();
}
