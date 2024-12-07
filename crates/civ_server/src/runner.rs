use bon::Builder;
use common::network::message::ClientToServerMessage;
use crossbeam::channel::{unbounded, Receiver, Sender};
use log::{error, info};
use rayon::{Scope, ThreadPoolBuilder};
use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
    time::{Duration, Instant},
};

use crate::{context::Context, state::State, task::effect::Effect};
use crate::{state::GAME_FRAMES_PER_SECOND, utils::collection::slices};

#[derive(Builder)]
pub struct Runner {
    context: Mutex<Context>,
    state: Mutex<State>,
    tick_base_period: u64,
    from_clients_receiver: Receiver<ClientToServerMessage>,
    #[builder(default = Duration::ZERO)]
    lag: Duration,
    #[builder(default = 0)]
    ticks_since_last_increment: u64,
    #[builder(default = 0)]
    ticks_since_last_stats: u64,
    #[builder(default = Instant::now())]
    last_stat: Instant,
}

impl Runner {
    fn context(&self) -> MutexGuard<Context> {
        self.context
            .lock()
            .expect("Assume context is always accessible")
    }

    fn state(&self) -> MutexGuard<State> {
        self.state
            .lock()
            .expect("Assume state is always accessible")
    }

    pub fn run(&mut self) {
        while !self.context().stop_is_required() {
            let tick_start = Instant::now();

            // FIXME: placer ecoute clients logiquement
            if let Ok(message) = self.from_clients_receiver.try_recv() {
                dbg!(message);
            }

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
            self.state().increment();
        }
        self.ticks_since_last_increment += 1;
    }

    fn stats_log(&mut self) {
        let state = self.state();
        let tasks_length = state.tasks().len();
        drop(state);

        if Instant::now().duration_since(self.last_stat).as_millis() >= 1000 {
            info!(
                "⏰{} 🌍{} 🎯{}",
                self.ticks_since_last_stats,
                self.state().frame().0,
                tasks_length,
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
            .scope(|scope| self.tick_tasks_chunk(tx, scope, workers_count));

        rx.try_iter()
            .collect::<Vec<Vec<Effect>>>()
            .into_iter()
            .flatten()
            .collect()
    }

    fn tick_tasks_chunk<'a>(
        &'a self,
        tx: Sender<Vec<Effect>>,
        scope: &Scope<'a>,
        workers_count: usize,
    ) {
        let state = self.state();
        let frame = *state.frame();
        let tasks_count = state.tasks().len();
        drop(state);

        let state = Arc::new(&self.state);
        for (start, end) in slices(tasks_count, workers_count) {
            let state = Arc::clone(&state);
            let tx = tx.clone();

            scope.spawn(move |_| {
                let state = state.lock().expect("Assume state is always accessible");
                let tasks = state.tasks();
                for task in &tasks[start..end] {
                    let effects_ = task.tick(frame);
                    if tx.send(effects_).is_err() {
                        error!("Channel closed in tasks scope: abort");
                        return;
                    }
                }
            })
        }
    }

    fn apply_effects(&mut self, effects: Vec<Effect>) {
        self.state().apply(effects)
    }
}
