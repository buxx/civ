use std::path::PathBuf;

use crate::config::ServerConfig;
use crate::context::Context;
use crate::runner::{Runner, RunnerContext};
use crate::snapshot::{Snapshot, SnapshotError};
use crate::state::State;
use crate::task::snapshot::SnapshotTask;
use crate::task::{TaskContext, TaskId};
use async_std::channel::Sender;
use bon::{builder, Builder};
use bridge::{Bridge, BridgeBuilder};
use clap::Parser;
use common::game::unit::{SystemTaskType, TaskType};
use common::game::GameFrame;
use common::rules::std1::Std1RuleSet;
use common::utils::Progress;
use common::world::reader::{WorldReader, WorldReaderError};
use log::info;
use std::io;
use std::{
    sync::{Arc, RwLock},
    thread,
};
use thiserror::Error;

pub mod bridge;
pub mod config;
pub mod context;
pub mod effect;
pub mod game;
pub mod reflect;
pub mod request;
pub mod runner;
pub mod snapshot;
pub mod state;
pub mod task;
pub mod test;
pub mod utils;

pub const TICK_BASE_PERIOD: u64 = 60;

#[derive(Parser, Debug, Builder)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path where load and save server snapshot
    #[arg(short, long)]
    snapshot: Option<PathBuf>,
    /// Game frame interval count between two snapshot
    #[arg(long, default_value = "120000")]
    snapshot_interval: u64,
    /// TCP listen address
    #[arg(short, long, default_value = "127.0.0.1:9876")]
    tcp_listen_address: String,
    /// WebSocket listen address
    #[arg(short, long, default_value = "127.0.0.1:9877")]
    ws_listen_address: String,
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Snapshot load/save error: {0}")]
    Snapshot(#[from] SnapshotError),
    #[error("Network prepare error: {0}")]
    PrepareBridge(String),
    #[error("World error: {0}")]
    World(#[from] WorldReaderError),
}

#[builder]
pub fn start<B: Bridge + 'static>(
    args: Args,
    bridge_builder: &dyn BridgeBuilder<B>,
    progress: Option<Sender<Progress<WorldReaderError>>>,
) -> Result<(), Error> {
    progress
        .as_ref()
        .map(|s| s.send_blocking(Progress::InProgress(0.)));
    let config = ServerConfig::from(args);

    let rules = Std1RuleSet;
    // TODO: move this code ?
    let state = match config.snapshot() {
        Some(snapshot_path) => {
            let snapshot_task = Box::new(SnapshotTask::new(
                TaskContext::builder()
                    .id(TaskId::default())
                    .start(GameFrame(0))
                    .end(*config.snapshot_interval())
                    .build(),
                snapshot_path.clone(),
            ));

            match Snapshot::try_from(snapshot_path) {
                Ok(snapshot) => State::from(snapshot),
                Err(SnapshotError::Io(error)) => match error {
                    io::ErrorKind::NotFound => {
                        info!(
                            "No snapshot found at {}: create empty state",
                            snapshot_path.display()
                        );
                        State::default()
                    }
                    _ => return Err(Error::from(SnapshotError::Io(error))),
                },
                Err(error) => return Err(Error::from(error)),
            }
            .with_replaced_task_type(TaskType::System(SystemTaskType::Snapshot), snapshot_task)
        }
        None => State::default(),
    };
    let world_source = PathBuf::from("./world");

    info!("Read world ...");
    let world = WorldReader::from(world_source, &progress)?;
    info!("Read world ... OK ({} tiles)", world.shape());

    let context = Context::new(Box::new(rules), config.clone());
    let state = Arc::new(RwLock::new(state));
    let world = Arc::new(RwLock::new(world));
    // let (from_clients_sender, from_clients_receiver): FromClientsChannels = unbounded();
    // let (to_clients_sender, to_clients_receiver): ToClientsChannels = unbounded();

    let (mut bridge, from_clients_receiver, to_clients_sender) = bridge_builder
        .build(context.clone(), Arc::clone(&state), &config)
        .map_err(|e| Error::PrepareBridge(e.to_string()))?;

    // let bridge = Network::new(
    //     context.clone(),
    //     Arc::clone(&state),
    //     config.tcp_listen_address(),
    //     config.ws_listen_address(),
    //     from_clients_sender,
    //     to_clients_receiver,
    // )
    // .map_err(|e| Error::PrepareNetwork(e.to_string()))?;

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

    let network = thread::spawn(move || bridge.run());
    let runner = thread::spawn(move || runner.run());

    network.join().unwrap();
    runner.join().unwrap();

    Ok(())
}
