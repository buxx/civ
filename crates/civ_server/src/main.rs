use common::{
    game::unit::UnitType,
    geo::{GeoContext, WorldPoint},
    network::message::{ClientToServerMessage, ServerToClientMessage},
    rules::std1::Std1RuleSet,
    world::reader::{
        FullMemoryWorldReader, FullMemoryWorldReaderError, WorldReader, WorldReaderError,
    },
};
use context::Context;
use crossbeam::channel::{unbounded, Receiver, Sender};
use game::unit::Unit;
use log::info;
use network::Network;
use runner::{Runner, RunnerContext};
use state::State;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
    thread,
};
use task::effect::{Effect, StateEffect, UnitEffect};
use thiserror::Error;
use uuid::Uuid;

mod context;
mod game;
mod network;
mod reflect;
mod request;
mod runner;
mod state;
mod task;
mod utils;

pub const TICK_BASE_PERIOD: u64 = 60;

#[derive(Error, Debug)]
enum Error {
    #[error("Network prepare error: {0}")]
    PrepareNetwork(String),
    #[error("World error: {0}")]
    World(#[from] WorldReaderError<FullMemoryWorldReaderError>),
}

type FromClientsChannels = (
    Sender<(Uuid, ClientToServerMessage)>,
    Receiver<(Uuid, ClientToServerMessage)>,
);
type ToClientsChannels = (
    Sender<(Uuid, ServerToClientMessage)>,
    Receiver<(Uuid, ServerToClientMessage)>,
);

fn main() -> Result<(), Error> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let rules = Std1RuleSet;
    let mut state = State::default();
    // HACK
    let world_source = PathBuf::from("./world");
    let uuid = Uuid::new_v4();
    state.apply(vec![Effect::State(StateEffect::Unit(
        uuid,
        UnitEffect::New(
            Unit::builder()
                .id(uuid)
                .type_(UnitType::Settlers)
                .geo(GeoContext::builder().point(WorldPoint::new(0, 0)).build())
                .build(),
        ),
    ))]);

    info!("Read world ...");
    let mut world = FullMemoryWorldReader::new(world_source);
    world.init()?;
    info!("Read world ... OK ({} tiles)", world.shape());

    let context = Context::new(Box::new(rules));
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
