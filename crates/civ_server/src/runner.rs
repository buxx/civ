use crossbeam::channel::{unbounded, Receiver, Sender};
use log::{error, info};
use rayon::{Scope, ThreadPoolBuilder};
use std::{
    error::Error,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
    time::{Duration, Instant},
};

use crate::{action::Effect, state::GAME_FRAMES_PER_SECOND, utils::collection::slices};
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
        self.state.read().expect("Assume state is always readable")
    }

    fn state_mut(&self) -> RwLockWriteGuard<State> {
        self.state
            .write()
            .expect("Assume state is always writeable")
    }

    pub fn run(&mut self) {
        while !self.context().stop_is_required() {
            let tick_start = Instant::now();
            let effects = self.tick();
            self.apply_effects(effects);
            self.fps_target(tick_start);
            self.game_frame_increment();
            self.stats_log();
        }
    }

    fn game_frame_increment(&mut self) {
        let increment_each = self.tick_base_period / GAME_FRAMES_PER_SECOND;
        if self.ticks_since_last_increment >= increment_each {
            self.ticks_since_last_increment = 0;
            self.state_mut().increment();
        }
        self.ticks_since_last_increment += 1;
    }

    fn stats_log(&mut self) {
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

    fn fps_target(&mut self, tick_start: Instant) {
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

    fn tick(&mut self) -> Vec<Effect> {
        let workers_count = num_cpus::get();
        let (tx, rx): (Sender<Vec<Effect>>, Receiver<Vec<Effect>>) = unbounded();
        ThreadPoolBuilder::new()
            .num_threads(workers_count)
            .build()
            .expect("Thread pool build must be stable")
            .scope(|scope| self.tick_actions_chunk(tx, scope, workers_count));

        rx.try_iter()
            .collect::<Vec<Vec<Effect>>>()
            .into_iter()
            .flatten()
            .collect()
    }

    fn tick_actions_chunk(&self, tx: Sender<Vec<Effect>>, scope: &Scope<'_>, workers_count: usize) {
        let state = self.state();
        let frame = *state.frame();
        let actions_count = state.actions().len();
        drop(state);

        for (start, end) in slices(actions_count, workers_count) {
            let state = Arc::clone(&self.state);
            let tx = tx.clone();

            scope.spawn(move |_| {
                let state = state.read().expect("State must be readable");
                let actions = state.actions();
                for action in &actions[start..end] {
                    let effects_ = action.tick(frame);
                    if tx.send(effects_).is_err() {
                        error!("Channel closed in actions scope: abort");
                        return;
                    }
                }
            })
        }
    }

    fn apply_effects(&mut self, effects: Vec<Effect>) {
        let mut state = self.state_mut();
        for effect in effects {
            state.apply(effect)
        }
    }
}
