#[cfg(test)]
#[macro_use]
extern crate factori;

use std::path::PathBuf;

use crate::config::ServerConfig;
use crate::context::Context;
use crate::game::placer::RandomPlacer;
use crate::runner::{Runner, RunnerContext};
use crate::snapshot::{Snapshot, SnapshotError};
use crate::state::State;
use crate::task::snapshot::SnapshotTask;
use crate::task::{Boxed, TaskContext, TaskId};
use crate::world::reader::{WorldReader, WorldReaderError};
use async_std::channel::Sender;
use bon::{builder, Builder};
use bridge::{Bridge, BridgeBuilder};
use clap::Parser;
use common::game::GameFrame;
use common::rules::std1::Std1RuleSet;
use common::space::D2Size;
use common::utils::Progress;
use log::{info, warn};
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
pub mod runner;
pub mod snapshot;
pub mod state;
pub mod task;
pub mod test;
pub mod utils;
pub mod world;

pub const TICK_BASE_PERIOD: u64 = 60;

#[derive(Parser, Debug, Builder, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// World path to load
    world: PathBuf,
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
    state: Option<State>,
    progress: Option<Sender<Progress<WorldReaderError>>>,
) -> Result<(), Error> {
    progress
        .as_ref()
        .map(|s| s.send_blocking(Progress::InProgress(0.)));
    let config = ServerConfig::from(args.clone());
    let rules = Std1RuleSet;

    info!("Read world ...");
    let world = WorldReader::from(args.world.clone(), &progress)?;
    info!("Read world ... OK ({} tiles)", world.shape());

    info!("Read snapshot or create from scratch ...");
    let state = match state {
        Some(state) => state,
        None => build_state(&config, world.size())?,
    };
    info!("Read snapshot or create from scratch ... OK");

    let context = Context::new(Box::new(rules), config.clone());
    let state = Arc::new(RwLock::new(state));
    let world = Arc::new(RwLock::new(world));
    let (mut bridge, from_clients_receiver, to_clients_sender) = bridge_builder
        .build(context.clone(), Arc::clone(&state), &config)
        .map_err(|e| Error::PrepareBridge(e.to_string()))?;

    let tasks = config
        .snapshot()
        .map(|p| {
            vec![SnapshotTask::new(
                TaskContext::builder()
                    .id(TaskId::default())
                    .start(GameFrame(0))
                    .end(*config.snapshot_interval())
                    .build(),
                p.clone(),
            )
            .boxed()]
        })
        .unwrap_or_default();

    let mut runner = Runner::builder()
        .tick_base_period(TICK_BASE_PERIOD)
        .context(RunnerContext::new(
            context.clone(),
            world,
            Arc::new(RwLock::new(tasks)),
            Arc::clone(&state),
            from_clients_receiver,
            to_clients_sender,
            Box::new(RandomPlacer),
        ))
        .build();

    let network = thread::spawn(move || bridge.run());
    let runner = thread::spawn(move || runner.run());

    network.join().unwrap();
    runner.join().unwrap();

    Ok(())
}

fn build_state(config: &ServerConfig, world_size: D2Size) -> Result<State, Error> {
    let state = match config.snapshot() {
        Some(snapshot_path) => match Snapshot::try_from(snapshot_path) {
            Ok(snapshot) => State::from(snapshot),
            Err(SnapshotError::Io(io::ErrorKind::NotFound)) => {
                warn!("No snapshot found, create from scratch");
                State::empty(world_size)
            }
            Err(error) => {
                return Err(Error::from(error));
            }
        },
        None => State::empty(world_size),
    };
    Ok(state)
}
