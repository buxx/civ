use clap::Parser;
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

use common::{
    game::PlayerId,
    network::{
        message::{ClientToServerMessage, ServerToClientMessage},
        ClientId,
    },
    rules::std1::Std1RuleSet,
};
use context::Context;
use crossbeam::channel::{unbounded, Receiver, Sender};
use network::NetworkClient;
use runner::Runner;
use state::State;
use thiserror::Error;

mod command;
mod context;
mod error;
mod network;
mod runner;
mod state;

#[derive(Error, Debug)]
enum Error {
    #[error("Invalid player id: {0}")]
    PlayerId(uuid::Error),
    #[error("Network prepare error: {0}")]
    PrepareNetwork(String),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Server address
    #[arg(short, long, default_value = "127.0.0.1:9876")]
    address: String,

    /// Player unique id
    #[arg(short, long)]
    player: Option<String>,
}

fn main() -> Result<(), Error> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);
    let args = Arguments::parse();

    let client_id = ClientId::default();
    let player_id = PlayerId::from_str(&args.player.unwrap_or(PlayerId::default().to_string()))
        .map_err(Error::PlayerId)?;
    let context = Context::new(Box::new(Std1RuleSet));
    let state = Arc::new(RwLock::new(State::new(client_id)));
    let (to_server_sender, to_server_receiver): (
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    ) = unbounded();
    let (from_server_sender, from_server_receiver): (
        Sender<ServerToClientMessage>,
        Receiver<ServerToClientMessage>,
    ) = unbounded();

    let network = NetworkClient::new(
        client_id,
        player_id,
        &args.address,
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
