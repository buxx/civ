use std::time::Duration;

use runner::Runner;

mod runner;

fn main() {
    let tick_base_period: u64 = 50;
    let sleep_target_ns: u64 = 1_000_000_000 / tick_base_period;
    let sleep_target = Duration::from_nanos(sleep_target_ns);
    let mut runner = Runner::new(sleep_target);
    runner.run();
}
