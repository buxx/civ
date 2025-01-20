use civ_server::config::ServerConfig;
use civ_server::context::Context;
use civ_server::network::Network;
use civ_server::runner::{Runner, RunnerContext};
use civ_server::snapshot::Snapshot;
use civ_server::state::State;
use civ_server::task::snapshot::SnapshotTask;
use civ_server::task::{TaskContext, TaskId};
use civ_server::world::reader::{WorldReader, WorldReaderError};
use civ_server::{Args, FromClientsChannels, ToClientsChannels};
use clap::Parser;
use common::game::unit::{SystemTaskType, TaskType};
use common::game::GameFrame;
use common::rules::std1::Std1RuleSet;
use crossbeam::channel::unbounded;
use log::info;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
};
use thiserror::Error;

pub const TICK_BASE_PERIOD: u64 = 60;

#[derive(Error, Debug)]
enum Error {
    #[error("Network prepare error: {0}")]
    PrepareNetwork(String),
    #[error("World error: {0}")]
    World(#[from] WorldReaderError),
}

fn main() -> Result<(), Error> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    let args = Args::parse();
    env_logger::init_from_env(env);

    let config = ServerConfig::from(args);

    let rules = Std1RuleSet;
    // TODO: move this code ?
    let state = match config.snapshot() {
        Some(snapshot) => {
            let snapshot_task = Box::new(SnapshotTask::new(
                TaskContext::builder()
                    .id(TaskId::default())
                    .start(GameFrame(0))
                    .end(*config.snapshot_interval())
                    .build(),
                snapshot.clone(),
            ));
            State::from(Snapshot::try_from(snapshot).unwrap())
                .with_replaced_task_type(TaskType::System(SystemTaskType::Snapshot), snapshot_task)
        }
        None => State::default(),
    };
    let world_source = PathBuf::from("./world");

    info!("Read world ...");
    let world = WorldReader::from(world_source)?;
    info!("Read world ... OK ({} tiles)", world.shape());

    let context = Context::new(Box::new(rules), config);
    let state = Arc::new(RwLock::new(state));
    let world = Arc::new(RwLock::new(world));
    let (from_clients_sender, from_clients_receiver): FromClientsChannels = unbounded();
    let (to_clients_sender, to_clients_receiver): ToClientsChannels = unbounded();

    let network = Network::new(
        context.clone(),
        Arc::clone(&state),
        "127.0.0.1:9876",
        from_clients_sender,
        to_clients_receiver,
    )
    .map_err(|e| Error::PrepareNetwork(e.to_string()))?;
    let mut runner = Runner::builder()
        .tick_base_period(TICK_BASE_PERIOD)
        .context(RunnerContext::new(
            context.clone(),
            Arc::clone(&state),
            world,
            from_clients_receiver,
            to_clients_sender,
        ))
        .build();

    let network = thread::spawn(move || network.run());
    let runner = thread::spawn(move || runner.run());

    network.join().unwrap();
    runner.join().unwrap();

    Ok(())
}
