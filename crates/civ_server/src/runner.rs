use bon::Builder;
use common::{
    game::GAME_FRAMES_PER_SECOND,
    network::message::{
        ClientStateMessage, ClientToServerMessage, NotificationLevel, ServerToClientMessage,
    },
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use log::{error, info};
use rayon::{Scope, ThreadPoolBuilder};
use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
    time::{Duration, Instant},
};
use uuid::Uuid;

use crate::utils::collection::slices;
use crate::{
    context::Context,
    request::SetWindowRequestDealer,
    state::State,
    task::effect::{Effect, StateEffect, TaskEffect},
};

pub struct RunnerContext {
    pub context: Context,
    pub state: Arc<Mutex<State>>,
    pub from_clients_receiver: Receiver<(Uuid, ClientToServerMessage)>,
    pub to_client_sender: Sender<(Uuid, ServerToClientMessage)>,
}

impl RunnerContext {
    pub fn new(
        context: Context,
        state: Arc<Mutex<State>>,
        from_clients_receiver: Receiver<(Uuid, ClientToServerMessage)>,
        to_client_sender: Sender<(Uuid, ServerToClientMessage)>,
    ) -> Self {
        Self {
            context,
            state,
            from_clients_receiver,
            to_client_sender,
        }
    }

    pub fn state(&self) -> MutexGuard<State> {
        self.state
            .lock()
            .expect("Assume state is always accessible")
    }
}

impl Clone for RunnerContext {
    fn clone(&self) -> Self {
        Self::new(
            self.context.clone(),
            Arc::clone(&self.state),
            self.from_clients_receiver.clone(),
            self.to_client_sender.clone(),
        )
    }
}

#[derive(Builder)]
pub struct Runner {
    pub(super) context: RunnerContext,
    tick_base_period: u64,
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
    pub(super) fn state(&self) -> MutexGuard<State> {
        self.context
            .state
            .lock()
            .expect("Assume state is always accessible")
    }

    pub fn run(&mut self) {
        while !self.context.context.stop_is_required() {
            self.do_one_iteration();
        }
    }

    fn do_one_iteration(&mut self) {
        let tick_start = Instant::now();

        // FIXME: do client requests in thread pool to not block task tick
        // and solve all effects here by reading channel
        let effects = self.clients();
        self.apply_effects(effects);

        let effects = self.tick();
        self.apply_effects(effects);

        // FIXME: send new GameFrame to all clients
        // CLEAN
        // ONLY WHEN CHANGE (reflect)
        let client_ids = self.state().clients().client_ids();
        for client_id in client_ids {
            let frame = *self.state().frame();
            self.context
                .to_client_sender
                .send((
                    client_id,
                    ServerToClientMessage::State(ClientStateMessage::SetGameFrame(frame)),
                ))
                .unwrap();
        }

        self.fps_target(tick_start);
        self.game_frame_increment();
        self.stats_log();
    }

    fn clients(&mut self) -> Vec<Effect> {
        let mut effects = vec![];

        while let Ok((client_id, message)) = self.context.from_clients_receiver.try_recv() {
            effects.extend(self.client(client_id, message));
        }

        effects
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
        let clients_count = state.clients().count();
        let cities_count = state.cities().len();
        let units_count = state.units().len();
        drop(state);

        if Instant::now().duration_since(self.last_stat).as_millis() >= 1000 {
            info!(
                "⏰{} 🌍{} 🎯{} 👥{} 🚹{} 🏠{}",
                self.ticks_since_last_stats,
                self.state().frame().0,
                tasks_length,
                clients_count,
                units_count,
                cities_count
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

        let state = Arc::new(&self.context.state);
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
        self.reflect(&effects);
        self.state().apply(effects);
    }

    // FIXME reflechir a une producteur de Vec<(Uuid, ServerToClientMessage)> depuis un Effect
    // pour eviter le doublon de concerned/effect_point
    fn reflect(&self, effects: &Vec<Effect>) {
        let state = &self.state();
        for effect in effects {
            if let Some(message) = effect.reflect(state) {
                for client_id in self.concerned(effect, state) {
                    self.context
                        .to_client_sender
                        .send((client_id, message.clone()))
                        .unwrap()
                }
            }
        }
    }

    // FIXME: MutexGuard<State> not in self because mutex lock conflict
    fn concerned(&self, effect: &Effect, state: &MutexGuard<State>) -> Vec<Uuid> {
        if let Some(point) = state.effect_point(effect) {
            return state.clients().clients_displaying(&point);
        }

        vec![]
    }

    fn client(&self, client_id: Uuid, message: ClientToServerMessage) -> Vec<Effect> {
        match message {
            ClientToServerMessage::SetWindow(window) => {
                SetWindowRequestDealer::new(self.context.clone(), client_id).deal(&window)
            }
            ClientToServerMessage::CreateTask(message) => {
                match self.create_task(message) {
                    Ok(task) => {
                        // FIXME: is task to attach on unit (city, etc) do it here !!!
                        vec![Effect::State(StateEffect::Task(
                            task.context().id(),
                            TaskEffect::Push(task),
                        ))]
                    }
                    Err(error) => {
                        self.context
                            .to_client_sender
                            .send((
                                client_id,
                                ServerToClientMessage::Notification(
                                    NotificationLevel::Error,
                                    error.to_string(),
                                ),
                            ))
                            .unwrap();
                        vec![]
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use common::rules::std1::Std1RuleSet;

    use crate::{FromClientsChannels, ToClientsChannels};

    use super::*;
    use rstest::*;

    #[fixture]
    pub fn runner() -> Runner {
        let context = Context::new(Box::new(Std1RuleSet));
        let state = Arc::new(Mutex::new(State::default()));
        let (from_clients_sender, from_clients_receiver): FromClientsChannels = unbounded();
        let (to_clients_sender, to_clients_receiver): ToClientsChannels = unbounded();
        let context = RunnerContext::new(context, state, from_clients_receiver, to_clients_sender);
        Runner::builder()
            .tick_base_period(9999)
            .context(context)
            .build()
    }

    #[rstest]
    fn test_runner_one_iteration(mut runner: Runner) {
        runner.do_one_iteration();
    }
}
