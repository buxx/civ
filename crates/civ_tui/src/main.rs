use std::{
    sync::{Arc, Mutex},
    thread,
};

use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use context::Context;
use crossbeam::channel::{unbounded, Receiver, Sender};
use network::Network;
use runner::Runner;
use state::State;
use thiserror::Error;
use uuid::Uuid;

mod command;
mod context;
mod error;
mod network;
mod runner;
mod state;

#[derive(Error, Debug)]
enum Error {
    #[error("Network prepare error: {0}")]
    PrepareNetwork(String),
}

fn main() -> Result<(), Error> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let client_id = Uuid::new_v4();
    let context = Context::new();
    let state = Arc::new(Mutex::new(State::new(client_id)));
    let (to_server_sender, to_server_receiver): (
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    ) = unbounded();
    let (from_server_sender, from_server_receiver): (
        Sender<ServerToClientMessage>,
        Receiver<ServerToClientMessage>,
    ) = unbounded();

    let network = Network::new(
        client_id,
        "127.0.0.1:9876",
        context.clone(),
        Arc::clone(&state),
        to_server_receiver,
        from_server_sender,
    )
    .map_err(|e| Error::PrepareNetwork(e.to_string()))?;
    let mut runner = Runner::builder()
        .context(context)
        .state(Arc::clone(&state))
        .from_server_receiver(from_server_receiver)
        .to_server_sender(to_server_sender)
        .build();

    let network = thread::spawn(move || network.run());
    let runner = thread::spawn(move || runner.run());

    network.join().unwrap();
    runner.join().unwrap();

    Ok(())
}
