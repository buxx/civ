use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use context::Context;
use crossbeam::channel::{unbounded, Receiver, Sender};
use network::Network;
use runner::Runner;
use state::State;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use thiserror::Error;
use uuid::Uuid;

mod context;
mod game;
mod network;
mod runner;
mod state;
mod task;
mod utils;

pub const TICK_BASE_PERIOD: u64 = 60;

#[derive(Error, Debug)]
enum Error {
    #[error("Network prepare error: {0}")]
    PrepareNetwork(String),
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

    let context = Arc::new(Mutex::new(Context::new()));
    let state = Arc::new(Mutex::new(State::default()));
    let (from_clients_sender, from_clients_receiver): FromClientsChannels = unbounded();
    let (to_clients_sender, to_clients_receiver): ToClientsChannels = unbounded();

    let network = Network::new(
        Arc::clone(&context),
        Arc::clone(&state),
        "127.0.0.1:9876",
        from_clients_sender,
        to_clients_receiver,
    )
    .map_err(|e| Error::PrepareNetwork(e.to_string()))?;
    let mut runner = Runner::builder()
        .tick_base_period(TICK_BASE_PERIOD)
        .context(Arc::clone(&context))
        .state(Arc::clone(&state))
        .from_clients_receiver(from_clients_receiver)
        .to_client_sender(to_clients_sender)
        .build();

    let network = thread::spawn(move || network.run());
    let runner = thread::spawn(move || runner.run());

    network.join().unwrap();
    runner.join().unwrap();

    Ok(())
}
