use std::sync::{Arc, RwLock};

use runner::Runner;
use state::State;

mod runner;
mod state;

fn main() {
    let tick_base_period: u64 = 100;
    let state = Arc::new(RwLock::new(State::default()));
    let mut runner = Runner::new(tick_base_period, state);
    runner.run();
}
