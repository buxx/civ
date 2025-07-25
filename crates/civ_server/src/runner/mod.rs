use async_std::channel::{unbounded, Receiver, Sender};
use bon::{builder, Builder};
use common::{
    game::{
        city::{CityId, CityProduct, CityProduction},
        unit::{UnitId, UnitType},
        GameFrame, GAME_FRAMES_PER_SECOND,
    },
    network::{
        message::{
            ClientToServerCityMessage, ClientToServerMessage, ClientToServerUnitMessage,
            NotificationLevel, ServerToClientInGameMessage, ServerToClientMessage,
        },
        Client, ClientId,
    },
    task::{CreateTaskError, GamePlayReason},
    world::reader::WorldReader,
};
use log::{debug, error, info};
use std::{
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    thread,
    time::{Duration, Instant},
};
use thiserror::Error;

use crate::{
    context::Context,
    effect::{self, Effect, StateEffect, TaskEffect},
    game::{
        placer::{PlacerBox, RandomPlacer},
        task::settle::Settle,
    },
    state::{NoLongerExist, State, StateError},
    task::{
        city::{BuildCityFrom, BuildCityFromChange, CityGenerator},
        Concern, TaskError, TaskId,
    },
};
use crate::{task::TaskBox, utils::collection::slices};

pub mod client;
pub struct RunnerContext {
    pub context: Context,
    pub state: Arc<RwLock<State>>,
    pub world: Arc<RwLock<WorldReader>>,
    pub from_clients_receiver: Receiver<(Client, ClientToServerMessage)>,
    pub to_client_sender: Sender<(ClientId, ServerToClientMessage)>,
}

impl RunnerContext {
    pub fn new(
        context: Context,
        state: Arc<RwLock<State>>,
        world: Arc<RwLock<WorldReader>>,
        from_clients_receiver: Receiver<(Client, ClientToServerMessage)>,
        to_client_sender: Sender<(ClientId, ServerToClientMessage)>,
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
    #[builder(default = vec![])]
    workers_channels: Vec<(Sender<()>, Receiver<Vec<Effect>>)>,
    #[builder(default = Box::new(RandomPlacer))]
    placer: PlacerBox,
}

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Deal with client request error: {0}")]
    DealClientRequest(DealClientRequestError),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Obsolete action: {0}")]
    NoLongerExist(String),
}

#[derive(Debug, Error)]
pub enum DealClientRequestError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Unfeasible: {0}")]
    Unfeasible(String),
    #[error("Unauthorized")]
    Unauthorized,
}

impl Runner {
    pub(super) fn state(&self) -> RwLockReadGuard<State> {
        self.context
            .state
            .read()
            .expect("Assume state is always accessible")
    }

    pub(super) fn world(&self) -> RwLockReadGuard<WorldReader> {
        self.context
            .world
            .read()
            .expect("Assume world is always accessible")
    }

    pub fn state_mut(&self) -> RwLockWriteGuard<State> {
        self.context
            .state
            .write()
            .expect("Assume state is always accessible")
    }

    pub fn run(&mut self) {
        self.setup_workers();

        while !self.context.context.stop_is_required() {
            self.do_one_iteration();
        }
    }

    pub fn setup_workers(&mut self) {
        let workers_count = num_cpus::get();

        for i in 0..workers_count {
            let (start_work_sender, start_work_receiver) = unbounded();
            let (results_sender, results_receiver) = unbounded();

            self.workers_channels
                .push((start_work_sender, results_receiver));

            let state = Arc::clone(&self.context.state);
            let context = self.context.clone();
            thread::spawn(move || {
                while start_work_receiver.recv_blocking().is_ok() {
                    let state = state.read().expect("Assume state is always accessible");
                    let frame = *state.frame();
                    let tasks_count = state.tasks().len();
                    let slices = slices(tasks_count, workers_count);
                    let tasks = state.tasks();
                    let (start, end) = slices[i];
                    let mut effects = vec![];

                    for task in &tasks[start..end] {
                        match tick_task(&context, task, &frame) {
                            Ok(effects_) => effects.extend(effects_),
                            Err(e) => {
                                eprintln!("Error when tasks execution: {}. Abort.", e);
                                context.context.require_stop();
                                return;
                            }
                        };
                    }

                    if results_sender.send_blocking(effects).is_err() {
                        error!("Channel closed in tasks scope: abort");
                        return;
                    }
                }
            });
        }
    }

    pub fn do_one_iteration(&mut self) {
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

        // TODO: parallel
        while let Ok((client, message)) = self.context.from_clients_receiver.try_recv() {
            match self.client(&client, message) {
                Ok(effects_) => effects.extend(effects_),
                Err(error) => match error {
                    RunnerError::DealClientRequest(error) => match error {
                        DealClientRequestError::Unfeasible(message) => {
                            self.context
                                .to_client_sender
                                .send_blocking((
                                    *client.client_id(),
                                    ServerToClientMessage::InGame(
                                        ServerToClientInGameMessage::Notification(
                                            NotificationLevel::Error,
                                            message,
                                        ),
                                    ),
                                ))
                                .unwrap();
                        }
                        DealClientRequestError::Unexpected(message) => {
                            error!("Error during processing client request: {}", message)
                        }
                        DealClientRequestError::Unauthorized => self
                            .context
                            .to_client_sender
                            .send_blocking((
                                *client.client_id(),
                                ServerToClientMessage::InGame(
                                    ServerToClientInGameMessage::Notification(
                                        NotificationLevel::Error,
                                        "Unauthorized".to_string(),
                                    ),
                                ),
                            ))
                            .unwrap(),
                    },
                    RunnerError::Unexpected(message) => {
                        error!("Error during processing client request: {}", message)
                    }
                    RunnerError::NoLongerExist(message) => {
                        debug!("Process client request about obsolete matter: {}", message)
                    }
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
        let clients_count = state.clients().clients_count();
        let players_count = state.clients().players_count();
        let cities_count = state.cities().len();
        let units_count = state.units().len();
        drop(state);

        if Instant::now().duration_since(self.last_stat).as_millis() >= 1000 {
            info!(
                "⏰{} 🌍{} 🎯{} 👥{} 🚩{} 🚹{} 🏠{}",
                self.ticks_since_last_stats,
                self.state().frame().0,
                tasks_length,
                clients_count,
                players_count,
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
        let sleep_target: Duration = Duration::from_nanos(sleep_target_ns);
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
        let mut effects = vec![];

        for (i, (start_sender, _)) in self.workers_channels.iter().enumerate() {
            if start_sender.send_blocking(()).is_err() {
                debug!("Worker {} start channel is closed", i)
            }
        }

        for (_, results_receiver) in &self.workers_channels {
            effects.extend(results_receiver.recv_blocking().unwrap_or_default());
        }

        effects
    }

    fn apply_effects(&mut self, effects: Vec<Effect>) {
        self.state_mut().apply(&effects);
        self.reflects(&effects);
    }

    // TODO: add tests here
    fn refresh_unit_on(
        &self,
        unit_id: &UnitId,
        message: ClientToServerUnitMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        debug!("Refresh unit on: {:?}", &message);

        let state = self.state();
        let unit = state.find_unit(unit_id).unwrap(); // TODO: unwrap -> same error management than crate_task
        let old_task = unit.task();

        let new_task = match message {
            ClientToServerUnitMessage::Settle(city_name) => Some(Settle::new(
                TaskId::default(),
                self.context.context.clone(),
                self.state(),
                unit.clone(),
                city_name.clone(),
            )?),
            ClientToServerUnitMessage::CancelCurrentTask => None,
        };
        let mut unit = unit.clone();
        unit.task = None;

        if let Some(new_task) = &new_task {
            unit.set_task(Some(new_task.clone().into()));
        }

        let mut effects = vec![effect::replace_unit(unit)];

        if let Some(new_task) = new_task {
            effects.push(effect::add_task(Box::new(new_task)));
        }

        if let Some(old_task) = old_task {
            effects.push(effect::remove_task(old_task.clone().into()));
        }

        Ok(effects)
    }

    fn refresh_city_on(
        &self,
        city_id: &CityId,
        message: ClientToServerCityMessage,
    ) -> Result<Vec<Effect>, RunnerError> {
        let state = self.state();
        let city = state.find_city(city_id).unwrap(); // TODO: unwrap -> same error management than crate_task
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
            .city_tasks(city_id)
            .iter()
            .map(|i| (*i, Concern::City(*city_id)))
            .collect::<Vec<(TaskId, Concern)>>();
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

fn tick_task(
    context: &RunnerContext,
    task: &TaskBox,
    frame: &GameFrame,
) -> Result<Vec<Effect>, TaskError> {
    let mut effects = task.tick(*frame);

    if task.context().is_finished(*frame) {
        effects.push(Effect::State(StateEffect::Task(
            *task.context().id(),
            TaskEffect::Finished(task.clone()),
        )));

        let (then_effects, then_tasks) = task.then(context)?;
        effects.extend(then_effects);

        for task in then_tasks {
            effects.push(Effect::State(StateEffect::Task(
                *task.context().id(),
                TaskEffect::Push(task),
            )));
        }
    }

    Ok(effects)
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
impl From<StateError> for RunnerError {
    fn from(value: StateError) -> Self {
        match value {
            StateError::NotFound(reason) => RunnerError::Unexpected(reason.to_string()),
            StateError::NoLongerExist(reason) => RunnerError::NoLongerExist(reason.to_string()),
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
                NoLongerExist::Player(_) => {
                    CreateTaskError::GamePlay(GamePlayReason::PlayerNoLongerExist)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use common::{
        game::{
            city::CityProductionTons,
            nation::flag::Flag,
            server::ServerResume,
            slice::{ClientUnit, GameSlice},
            tasks::client::{settle::ClientSettle, ClientTask, ClientTaskType},
            unit::{TaskType, UnitCan, UnitType},
            GameFrame, PlayerId,
        },
        geo::{ImaginaryWorldPoint, WorldPoint},
        network::message::{
            ClientStateMessage, ClientToServerEstablishmentMessage, ClientToServerGameMessage,
            ClientToServerInGameMessage, ServerToClientEstablishmentMessage,
        },
        rules::{RuleSet, RuleSetType},
        space::window::{DisplayStep, Resolution, Window},
        world::{partial::PartialWorld, CtxTile, TerrainType, Tile},
    };

    use crate::{
        config::ServerConfig,
        effect::{self},
        game::{
            placer::{Placer, PlacerError},
            unit::Unit,
        },
        state::clients::Clients,
    };

    use super::*;
    use pretty_assertions::{assert_eq, assert_matches};
    use rstest::*;

    #[derive(Clone)]
    struct TestRuleSet;

    impl RuleSet for TestRuleSet {
        fn tasks(&self) -> Vec<TaskType> {
            unreachable!()
        }

        fn unit_can(&self, _: &UnitType) -> Vec<TaskType> {
            unreachable!()
        }

        fn settle_duration(&self, _: &UnitType) -> GameFrame {
            GameFrame(100)
        }

        fn can_settle(&self, _: &UnitType) -> bool {
            true
        }

        fn required_tons(&self, product: &CityProduct) -> CityProductionTons {
            match product {
                CityProduct::Unit(unit_type) => match unit_type {
                    UnitType::Settlers => CityProductionTons(40),
                    UnitType::Warriors => CityProductionTons(8),
                },
            }
        }

        fn type_(&self) -> RuleSetType {
            RuleSetType::Testing
        }

        fn can_be_startup(&self, _tile: &common::world::Tile) -> bool {
            true
        }
    }

    struct TestPlacer;

    impl<'a> Placer<'a> for TestPlacer {
        fn startup(
            &self,
            _rules: &'a common::rules::RuleSetBox,
            _state: &'a RwLockReadGuard<State>,
            _world: &'a RwLockReadGuard<WorldReader>,
        ) -> Result<WorldPoint, PlacerError> {
            Ok(WorldPoint::new(0, 0))
        }
    }

    struct TestingRunnerContext {
        from_clients_sender: Sender<(Client, ClientToServerMessage)>,
        from_clients_receiver: Receiver<(Client, ClientToServerMessage)>,
        to_clients_sender: Sender<(ClientId, ServerToClientMessage)>,
        to_clients_receiver: Receiver<(ClientId, ServerToClientMessage)>,
        units: Vec<Unit>,
        rule_set: TestRuleSet,
        client_id: ClientId,
        player_id: PlayerId,
        resolution: Resolution,
    }

    impl TestingRunnerContext {
        fn new() -> Self {
            let (from_clients_sender, from_clients_receiver) = unbounded();
            let (to_clients_sender, to_clients_receiver) = unbounded();

            Self {
                from_clients_sender,
                from_clients_receiver,
                to_clients_sender,
                to_clients_receiver,
                units: vec![],
                rule_set: TestRuleSet,
                client_id: ClientId::default(),
                player_id: PlayerId::default(),
                resolution: Resolution::default(),
            }
        }

        fn client_id(mut self, value: ClientId) -> Self {
            self.client_id = value;
            self
        }

        fn player_id(mut self, value: PlayerId) -> Self {
            self.player_id = value;
            self
        }

        fn resolution(mut self, value: Resolution) -> Self {
            self.resolution = value;
            self
        }

        fn _units(mut self, value: Vec<Unit>) -> Self {
            self.units = value;
            self
        }

        fn build(&mut self) -> Runner {
            let mut state = State::default();
            let world = WorldReader::new(
                PathBuf::new(),
                2,
                2,
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

            let mut clients = HashMap::new();
            clients.insert(
                self.client_id,
                (
                    self.resolution,
                    Window::new((0, 0).into(), (1, 1).into(), DisplayStep::Close),
                ),
            );

            *state.clients_mut() = Clients::new(HashMap::new());

            let config = ServerConfig::new(None, GameFrame(0), "".to_string(), "".to_string());
            let context = Context::new(Box::new(self.rule_set.clone()), config);
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
                .placer(Box::new(TestPlacer))
                .build()
        }

        fn to_server(&self, client: Client, message: ClientToServerMessage) {
            self.from_clients_sender
                .send_blocking((client, message))
                .unwrap()
        }
    }

    #[rstest]
    fn test_settle() {
        // GIVEN
        let flag = Flag::Abkhazia;
        let player_id = PlayerId::default();
        let client_id = ClientId::default();
        let window_width = 3;
        let window_height = 3;
        let resolution = Resolution::new(window_width, window_height);
        let mut context = TestingRunnerContext::new()
            .player_id(player_id)
            .client_id(client_id)
            .resolution(resolution);
        let city_name = "CityName".to_string();
        let mut runner = context.build();
        let client = Client::new(client_id, player_id);

        let take_place = ClientToServerEstablishmentMessage::TakePlace(flag, resolution).into();

        let expected_window_start = ImaginaryWorldPoint::new(-1, -1);
        let expected_window_end = ImaginaryWorldPoint::new(1, 1);
        let expected_window_step = DisplayStep::Close;
        let expected_window = Window::new(
            expected_window_start,
            expected_window_end,
            expected_window_step,
        );
        let expected_game_slice_tiles = vec![
            CtxTile::Outside,
            CtxTile::Outside,
            CtxTile::Outside,
            CtxTile::Outside,
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Outside,
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
            CtxTile::Visible(Tile::new(TerrainType::GrassLand)),
        ];
        let expected_game_slice_world = PartialWorld::new(
            expected_window_start,
            window_width,
            window_height,
            expected_game_slice_tiles,
        );

        let expected_server_resume = ServerResume::new(RuleSetType::Testing, vec![]);

        // WHEN/THEN
        context.to_server(client, take_place);
        runner.do_one_iteration();

        assert_eq!(context.to_clients_receiver.len(), 4);
        let message1 = context.to_clients_receiver.try_recv();
        let message2 = context.to_clients_receiver.try_recv();
        let message3 = context.to_clients_receiver.try_recv();
        let message4 = context.to_clients_receiver.try_recv();

        assert_matches!(
            message1,
            Ok((
                _,
                ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                    ClientStateMessage::SetUnit(_)
                ))
            ))
        );
        assert_matches!(
            message2,
            Ok((
                _,
                ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                    ClientStateMessage::SetGameSlice(_)
                ))
            ))
        );
        assert_matches!(
            message3,
            Ok((
                _,
                ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
                    ClientStateMessage::SetWindow(_)
                ))
            ))
        );
        assert_matches!(
            message4,
            Ok((
                _,
                ServerToClientMessage::Establishment(
                    ServerToClientEstablishmentMessage::ServerResume(_, _)
                )
            ))
        );

        let ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
            ClientStateMessage::SetUnit(set_unit),
        )) = message1.unwrap().1
        else {
            unreachable!()
        };
        assert_eq!(set_unit.type_(), &UnitType::Settlers);
        assert_eq!(set_unit.geo().point(), &WorldPoint::new(0, 0));

        let ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
            ClientStateMessage::SetGameSlice(game_slice),
        )) = message2.unwrap().1
        else {
            unreachable!()
        };
        assert_eq!(game_slice.world(), &expected_game_slice_world);
        assert_eq!(game_slice.cities(), vec![]);
        assert_eq!(game_slice.units().len(), 0); // FIXME Unit will be added next in separated message ? it has been received ?

        let ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
            ClientStateMessage::SetWindow(set_window),
        )) = message3.unwrap().1
        else {
            unreachable!()
        };
        assert_eq!(set_window, expected_window);

        let ServerToClientMessage::Establishment(ServerToClientEstablishmentMessage::ServerResume(
            set_server_resume,
            set_flag,
        )) = message4.unwrap().1
        else {
            unreachable!()
        };
        assert_eq!(set_server_resume, expected_server_resume);
        assert_eq!(set_flag, Some(flag));

        // WHEN/THEN
        let task = ClientToServerInGameMessage::Unit(
            *set_unit.id(),
            ClientToServerUnitMessage::Settle(city_name.clone()),
        )
        .into();
        context.to_server(client, task);
        runner.do_one_iteration();

        let expected_client_unit = ClientUnit::builder()
            .id(*set_unit.id())
            .geo(*set_unit.geo())
            .flag(flag)
            .type_(*set_unit.type_())
            .task(ClientTask::new(
                ClientTaskType::Settle(ClientSettle::new(city_name.clone())),
                GameFrame(0),
                GameFrame(100), // Depends on SettleTask
            ))
            .can(vec![UnitCan::Settle])
            .build();
        let expected_set_unit = ServerToClientMessage::InGame(ServerToClientInGameMessage::State(
            ClientStateMessage::SetUnit(expected_client_unit),
        ));

        let message5 = context.to_clients_receiver.try_recv();
        assert_eq!(message5, Ok((client_id, expected_set_unit)));
    }
}
