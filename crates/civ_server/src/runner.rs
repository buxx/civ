use bon::Builder;
use common::{
    game::{
        city::{CityProduct, CityProduction},
        unit::UnitType,
        GameFrame, GAME_FRAMES_PER_SECOND,
    },
    network::message::{
        ClientToServerCityMessage, ClientToServerMessage, ClientToServerUnitMessage,
        NotificationLevel, ServerToClientMessage,
    },
    task::{CreateTaskError, GamePlayReason},
};
use crossbeam::channel::{unbounded, Receiver, Sender};
use log::{error, info};
use rayon::{Scope, ThreadPoolBuilder};
use std::{
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
    time::{Duration, Instant},
};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    context::Context,
    game::task::settle::Settle,
    request::SetWindowRequestDealer,
    state::{NoLongerExist, State, StateError},
    task::{
        city::{BuildCityFrom, BuildCityFromChange, CityGenerator},
        effect::{self, Effect, StateEffect, TaskEffect},
        Concern, TaskError,
    },
    world::reader::WorldReader,
};
use crate::{task::TaskBox, utils::collection::slices};

pub struct RunnerContext {
    pub context: Context,
    pub state: Arc<RwLock<State>>,
    pub world: Arc<RwLock<WorldReader>>,
    pub from_clients_receiver: Receiver<(Uuid, ClientToServerMessage)>,
    pub to_client_sender: Sender<(Uuid, ServerToClientMessage)>,
}

impl RunnerContext {
    pub fn new(
        context: Context,
        state: Arc<RwLock<State>>,
        world: Arc<RwLock<WorldReader>>,
        from_clients_receiver: Receiver<(Uuid, ClientToServerMessage)>,
        to_client_sender: Sender<(Uuid, ServerToClientMessage)>,
    ) -> Self {
        Self {
            context,
            state,
            world,
            from_clients_receiver,
            to_client_sender,
        }
    }

    pub fn state(&self) -> RwLockReadGuard<State> {
        self.state
            .read()
            .expect("Assume state is always accessible")
    }

    pub fn default_production(&self) -> CityProduction {
        // Default according to context (warrior, then phalanx, etc) and tons
        CityProduction::new(vec![CityProduct::Unit(UnitType::Warriors)])
    }
}

impl Clone for RunnerContext {
    fn clone(&self) -> Self {
        Self::new(
            self.context.clone(),
            Arc::clone(&self.state),
            Arc::clone(&self.world),
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

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Deal with client request error: {0}")]
    DealClientRequest(DealClientRequestError),
}

#[derive(Debug, Error)]
pub enum DealClientRequestError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Unfeasible: {0 }")]
    Unfeasible(String),
}

impl Runner {
    pub(super) fn state(&self) -> RwLockReadGuard<State> {
        self.context
            .state
            .read()
            .expect("Assume state is always accessible")
    }

    pub fn state_mut(&self) -> RwLockWriteGuard<State> {
        self.context
            .state
            .write()
            .expect("Assume state is always accessible")
    }

    pub fn run(&mut self) {
        while !self.context.context.stop_is_required() {
            self.do_one_iteration();
        }
    }

    fn do_one_iteration(&mut self) {
        let tick_start = Instant::now();

        // TODO: do client requests in thread pool to not block task tick
        // and solve all effects here by reading channel
        let effects = self.clients();
        self.apply_effects(effects);

        let effects = self.tick();
        self.apply_effects(effects);

        self.fps_target(tick_start);
        self.game_frame_increment();
        self.stats_log();
    }

    fn clients(&mut self) -> Vec<Effect> {
        let mut effects = vec![];

        while let Ok((client_id, message)) = self.context.from_clients_receiver.try_recv() {
            match self.client(client_id, message) {
                Ok(effects_) => effects.extend(effects_),
                Err(error) => match error {
                    RunnerError::DealClientRequest(error) => match error {
                        DealClientRequestError::Unfeasible(message) => {
                            self.context
                                .to_client_sender
                                .send((
                                    client_id,
                                    ServerToClientMessage::Notification(
                                        NotificationLevel::Error,
                                        message,
                                    ),
                                ))
                                .unwrap();
                        }
                        DealClientRequestError::Unexpected(message) => {
                            error!("Error during processing client request: {}", message)
                        }
                    },
                },
            };
        }

        effects
    }

    fn game_frame_increment(&mut self) {
        let increment_each = self.tick_base_period / GAME_FRAMES_PER_SECOND;
        if self.ticks_since_last_increment >= increment_each {
            self.ticks_since_last_increment = 0;
            self.apply_effects(vec![Effect::State(StateEffect::IncrementGameFrame)])
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

            let context = self.context.context.clone();
            scope.spawn(move |_| {
                let state = state.read().expect("Assume state is always accessible");
                let tasks = state.tasks();
                for task in &tasks[start..end] {
                    let effects_ = match self.tick_task(task, &frame) {
                        Ok(effects_) => effects_,
                        Err(e) => {
                            eprintln!("Error when tasks execution: {}. Abort.", e);
                            context.require_stop();
                            return;
                        }
                    };
                    if tx.send(effects_).is_err() {
                        error!("Channel closed in tasks scope: abort");
                        return;
                    }
                }
            })
        }
    }

    fn tick_task(&self, task: &TaskBox, frame: &GameFrame) -> Result<Vec<Effect>, TaskError> {
        let mut effects = task.tick(*frame);

        if task.context().is_finished(*frame) {
            effects.push(Effect::State(StateEffect::Task(
                task.context().id(),
                TaskEffect::Finished(task.clone()),
            )));

            let (then_effects, then_tasks) = task.then(&self.context)?;
            effects.extend(then_effects);

            for task in then_tasks {
                effects.push(Effect::State(StateEffect::Task(
                    task.context().id(),
                    TaskEffect::Push(task),
                )));
            }
        }

        Ok(effects)
    }

    fn apply_effects(&mut self, effects: Vec<Effect>) {
        self.state_mut().apply(&effects);
        self.reflects(&effects);
    }

    fn client(
        &self,
        client_id: Uuid,
        message: ClientToServerMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        match message {
            ClientToServerMessage::SetWindow(window) => {
                //
                SetWindowRequestDealer::new(self.context.clone(), client_id).deal(&window)
            }
            ClientToServerMessage::Unit(uuid, message) => {
                //
                self.refresh_unit_on(&uuid, message)
            }
            ClientToServerMessage::City(uuid, message) => {
                //
                self.refresh_city_on(&uuid, message)
            }
        }
    }
    fn refresh_unit_on(
        &self,
        uuid: &Uuid,
        message: ClientToServerUnitMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        let state = self.state();
        let unit = state.find_unit(uuid).unwrap(); // TODO: unwrap -> same error management than crate_task
        let old_task = unit.task();

        let task = match message {
            ClientToServerUnitMessage::Settle(city_name) => Settle::new(
                Uuid::new_v4(),
                self.context.context.clone(),
                self.state(),
                unit.clone(),
                city_name.clone(),
            )?,
        };
        let mut unit = unit.clone();
        unit.set_task(Some(task.clone().into()));

        let mut effects = vec![effect::replace_unit(unit), effect::add_task(Box::new(task))];
        if let Some(old_task) = old_task {
            effects.push(effect::remove_task(old_task.clone().into()));
        }
        Ok(effects)
    }

    fn refresh_city_on(
        &self,
        uuid: &Uuid,
        message: ClientToServerCityMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        let state = self.state();
        let city = state.find_city(uuid).unwrap(); // TODO: unwrap -> same error management than crate_task
        let from = match message {
            ClientToServerCityMessage::SetProduction(production) => {
                BuildCityFrom::Change(city, BuildCityFromChange::Production(production))
            }
            ClientToServerCityMessage::SetExploitation(exploitation) => {
                BuildCityFrom::Change(city, BuildCityFromChange::Exploitation(exploitation))
            }
        };
        let old_tasks = state
            .index()
            .city_tasks(uuid)
            .iter()
            .map(|i| (*i, Concern::City(*i)))
            .collect::<Vec<(Uuid, Concern)>>();
        let city = CityGenerator::builder()
            .context(&self.context)
            .game_frame(self.context.state().frame())
            .from(from)
            .build()
            .generate()
            // TODO: unwrap -> same error management than crate_task
            .unwrap();
        let new_tasks = city.tasks().clone().into();

        Ok(vec![
            effect::replace_city(city),
            effect::remove_tasks(old_tasks),
            effect::add_tasks(new_tasks),
        ])
    }
}

impl From<CreateTaskError> for RunnerError {
    fn from(value: CreateTaskError) -> Self {
        match &value {
            CreateTaskError::GamePlay(reason) => {
                //
                RunnerError::DealClientRequest(DealClientRequestError::Unfeasible(
                    reason.to_string(),
                ))
            }
            CreateTaskError::Unexpected(message) => {
                //
                RunnerError::DealClientRequest(DealClientRequestError::Unexpected(message.clone()))
            }
        }
    }
}

impl From<StateError> for CreateTaskError {
    fn from(value: StateError) -> Self {
        match value {
            StateError::NotFound(error) => CreateTaskError::Unexpected(error.to_string()),
            StateError::NoLongerExist(error) => match error {
                NoLongerExist::City(_) => {
                    CreateTaskError::GamePlay(GamePlayReason::CityNoLongerExist)
                }
                NoLongerExist::Unit(_) => {
                    CreateTaskError::GamePlay(GamePlayReason::UnitNoLongerExist)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use common::{
        game::{
            slice::{ClientUnit, GameSlice},
            tasks::client::{settle::ClientSettle, ClientTask, ClientTaskType},
            unit::UnitType,
            GameFrame,
        },
        geo::{Geo, GeoContext, WorldPoint},
        network::message::ClientStateMessage,
        rules::std1::Std1RuleSet,
        space::window::{DisplayStep, SetWindow, Window},
        world::{partial::PartialWorld, TerrainType, Tile},
    };

    use crate::{
        game::unit::Unit,
        task::effect::{self},
        FromClientsChannels, ToClientsChannels,
    };

    use super::*;
    use rstest::*;

    struct TestingRunnerContext {
        from_clients_sender: Sender<(Uuid, ClientToServerMessage)>,
        from_clients_receiver: Receiver<(Uuid, ClientToServerMessage)>,
        to_clients_sender: Sender<(Uuid, ServerToClientMessage)>,
        to_clients_receiver: Receiver<(Uuid, ServerToClientMessage)>,
        units: Vec<Unit>,
        rule_set: Std1RuleSet,
    }

    impl TestingRunnerContext {
        fn new() -> Self {
            let (from_clients_sender, from_clients_receiver): FromClientsChannels = unbounded();
            let (to_clients_sender, to_clients_receiver): ToClientsChannels = unbounded();

            Self {
                from_clients_sender,
                from_clients_receiver,
                to_clients_sender,
                to_clients_receiver,
                units: vec![],
                rule_set: Std1RuleSet,
            }
        }

        fn units(mut self, value: Vec<Unit>) -> Self {
            self.units = value;
            self
        }

        fn build(&mut self) -> Runner {
            let mut state = State::default();
            let world = WorldReader::new(
                PathBuf::new(),
                1,
                1,
                vec![
                    Tile::new(TerrainType::GrassLand),
                    Tile::new(TerrainType::GrassLand),
                    Tile::new(TerrainType::GrassLand),
                    Tile::new(TerrainType::GrassLand),
                ],
            );

            while let Some(unit) = self.units.pop() {
                state.apply(&vec![effect::new_unit(unit)]);
            }

            let context = Context::new(Box::new(self.rule_set.clone()));
            let state = Arc::new(RwLock::new(state));

            let context = RunnerContext::new(
                context,
                state,
                Arc::new(RwLock::new(world)),
                self.from_clients_receiver.clone(),
                self.to_clients_sender.clone(),
            );

            Runner::builder()
                .tick_base_period(9999)
                .context(context)
                .build()
        }
    }

    #[fixture]
    fn settler() -> Unit {
        Unit::builder()
            .geo(GeoContext::builder().point(WorldPoint::new(0, 0)).build())
            .id(Uuid::new_v4())
            .type_(UnitType::Settlers)
            .build()
    }

    #[rstest]
    fn test_settle(settler: Unit) {
        // GIVEN
        let mut testing = TestingRunnerContext::new().units(vec![settler.clone()]);
        let client_id = Uuid::new_v4();
        let settler_id = settler.id();
        let city_name = "CityName".to_string();
        let client_settler = ClientUnit::builder()
            .id(settler_id)
            .geo(*settler.geo())
            .type_(*settler.type_())
            .build();
        let expected_client_unit = ClientUnit::builder()
            .id(settler_id)
            .geo(*settler.geo())
            .type_(*settler.type_())
            .task(ClientTask::new(
                ClientTaskType::Settle(ClientSettle::new(city_name.clone())),
                GameFrame(0),
                GameFrame(100),
            ))
            .build();
        let mut runner = testing.build();

        let set_window = ClientToServerMessage::SetWindow(SetWindow::new(0, 0, 1, 1));
        let create_task = ClientToServerMessage::Unit(
            settler_id,
            ClientToServerUnitMessage::Settle(city_name.clone()),
        );

        let expected_set_window = ServerToClientMessage::State(ClientStateMessage::SetWindow(
            Window::new(0, 0, 1, 1, DisplayStep::Close),
        ));
        let expected_game_set_slice =
            ServerToClientMessage::State(ClientStateMessage::SetGameSlice(GameSlice::new(
                PartialWorld::new(
                    WorldPoint::new(0, 0),
                    1,
                    1,
                    vec![
                        Tile::new(TerrainType::GrassLand),
                        Tile::new(TerrainType::GrassLand),
                        Tile::new(TerrainType::GrassLand),
                        Tile::new(TerrainType::GrassLand),
                    ],
                ),
                vec![],
                vec![client_settler],
            )));
        let expected_set_unit =
            ServerToClientMessage::State(ClientStateMessage::SetUnit(expected_client_unit));

        // WHEN
        testing
            .from_clients_sender
            .send((client_id, set_window))
            .unwrap();
        runner.do_one_iteration();
        testing
            .from_clients_sender
            .send((client_id, create_task))
            .unwrap();
        runner.do_one_iteration();

        // THEN
        assert_eq!(testing.to_clients_receiver.len(), 3);

        let message1 = testing.to_clients_receiver.try_recv();
        assert_eq!(message1, Ok((client_id, expected_set_window)));

        let message2 = testing.to_clients_receiver.try_recv();
        assert_eq!(message2, Ok((client_id, expected_game_set_slice)));

        let message3 = testing.to_clients_receiver.try_recv();
        assert_eq!(message3, Ok((client_id, expected_set_unit)));
    }
}
