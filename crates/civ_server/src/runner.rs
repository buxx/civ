use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Runner {
    sleep_target: Duration,
    lag: Duration,
}

impl Runner {
    pub fn new(sleep_target: Duration) -> Self {
        Self {
            sleep_target,
            lag: Duration::ZERO,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let tick_start = Instant::now();

        // FPS target
        let tick_duration = Instant::now() - tick_start;
        let need_sleep = self.sleep_target
            - Duration::from_nanos(
                (tick_duration.as_nanos() as u64).min(self.sleep_target.as_nanos() as u64),
            );
        self.lag += (tick_duration.max(self.sleep_target) - self.sleep_target).min(Duration::ZERO);
        let can_catch_lag = self.lag.min(need_sleep);
        self.lag -= can_catch_lag;
        thread::sleep(need_sleep - can_catch_lag);
    }
}
