use log::error;
use log::info;
use std::{
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
    time::{Duration, Instant},
};

use crate::state::GAME_FRAMES_PER_SECOND;
use crate::{context::Context, state::State};

pub struct Runner {
    context: Arc<RwLock<Context>>,
    state: Arc<RwLock<State>>,
    tick_base_period: u64,
    lag: Duration,
    ticks_since_last_increment: u64,
    ticks_since_last_stats: u64,
    last_stat: Instant,
}

impl Runner {
    pub fn new(
        tick_base_period: u64,
        context: Arc<RwLock<Context>>,
        state: Arc<RwLock<State>>,
    ) -> Self {
        Self {
            context,
            state,
            tick_base_period,
            lag: Duration::ZERO,
            ticks_since_last_increment: 0,
            ticks_since_last_stats: 0,
            last_stat: Instant::now(),
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

    fn state(&self) -> RwLockReadGuard<State> {
        // FIXME: stop all by Context
        self.state.read().unwrap()
    }

    fn state_mut(&self) -> RwLockWriteGuard<State> {
        // FIXME: stop all by Context
        self.state.write().unwrap()
    }

    pub fn run(&mut self) {
        while !self.context().stop_is_required() {
            self.tick();

            // Game frame increment management
            let increment_each = self.tick_base_period / GAME_FRAMES_PER_SECOND;
            if self.ticks_since_last_increment >= increment_each {
                self.ticks_since_last_increment = 0;
                self.state_mut().increment();
            }
            self.ticks_since_last_increment += 1;

            // Stats print management
            if Instant::now().duration_since(self.last_stat).as_millis() >= 1000 {
                info!(
                    "{} tick/s, frame {}",
                    self.ticks_since_last_stats,
                    self.state().frame().0
                );

                self.ticks_since_last_stats = 0;
                self.last_stat = Instant::now();
            }
            self.ticks_since_last_stats += 1;
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
}
