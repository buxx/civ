use log::error;
use log::info;
use std::{
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
    time::{Duration, Instant},
};

use crate::{context::Context, state::State};

pub struct Runner {
    tick_base_period: u64,
    lag: Duration,
    context: Arc<RwLock<Context>>,
    state: Arc<RwLock<State>>,
}

impl Runner {
    pub fn new(
        tick_base_period: u64,
        context: Arc<RwLock<Context>>,
        state: Arc<RwLock<State>>,
    ) -> Self {
        Self {
            tick_base_period,
            lag: Duration::ZERO,
            context,
            state,
        }
    }

    fn context(&self) -> RwLockReadGuard<Context> {
        self.context
            .read()
            .expect("Context must be readable or we crash")
    }

    fn context_mut(&self) -> RwLockWriteGuard<Context> {
        self.context
            .write()
            .expect("Context must be writeable or we crash")
    }

    fn state_mut(&self) -> RwLockWriteGuard<State> {
        // FIXME: stop all by Context
        self.state.write().unwrap()
    }

    pub fn run(&mut self) {
        self.start_stats();
        while !self.context().stop_is_required() {
            self.tick();
            self.state_mut().increment();
        }
    }

    fn tick(&mut self) {
        let tick_start = Instant::now();

        // FPS target
        let tick_duration = Instant::now() - tick_start;
        let sleep_target_ns: u64 = 1_000_000_000 / self.tick_base_period;
        let sleep_target = Duration::from_nanos(sleep_target_ns);
        let need_sleep = sleep_target
            - Duration::from_nanos(
                (tick_duration.as_nanos() as u64).min(sleep_target.as_nanos() as u64),
            );
        self.lag += (tick_duration.max(sleep_target) - sleep_target).min(Duration::ZERO);
        let can_catch_lag = self.lag.min(need_sleep);
        self.lag -= can_catch_lag;
        thread::sleep(need_sleep - can_catch_lag);
    }

    fn start_stats(&self) {
        let context = Arc::clone(&self.context);
        let state = Arc::clone(&self.state);
        let sleet_time = Duration::from_secs(1);

        thread::spawn(move || {
            while !context
                .read()
                .expect("Context must be readable or we crash")
                .stop_is_required()
            {
                let previous_frame_i = *state.read().unwrap().frame_i();
                thread::sleep(sleet_time);

                let frame_count = match state.read() {
                    Ok(state) => state.frame_i().0 - previous_frame_i.0,
                    Err(error) => {
                        error!("Error from runner stats: {}", error);
                        context
                            .write()
                            .expect("Context must be writeable or we crash")
                            .require_stop();
                        break;
                    }
                };
                info!("{} tick/s", frame_count);
            }
        });
    }
}
