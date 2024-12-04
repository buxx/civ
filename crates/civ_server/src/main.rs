use std::sync::{Arc, RwLock};

use runner::Runner;
use state::State;

mod runner;
mod state;

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let tick_base_period: u64 = 100;
    let state = Arc::new(RwLock::new(State::default()));
    let mut runner = Runner::new(tick_base_period, state);
    runner.run();
}
