use clap::Parser;
use std::{
    str::FromStr,
    sync::{atomic::AtomicBool, Arc, RwLock},
    thread,
};

use common::{
    game::PlayerId,
    network::{
        client::NetworkClient,
        message::{ClientToServerMessage, ServerToClientMessage},
        ClientId,
    },
    rules::std1::Std1RuleSet,
};
use context::Context;
use crossbeam::channel::{unbounded, Receiver, Sender};
use runner::Runner;
use state::State;
use thiserror::Error;

mod command;
mod context;
mod error;
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

    let stop = Arc::new(AtomicBool::new(false));
    let connected = Arc::new(AtomicBool::new(false));
    let client_id = ClientId::default();
    let player_id = PlayerId::from_str(&args.player.unwrap_or(PlayerId::default().to_string()))
        .map_err(Error::PlayerId)?;
    let context = Context::new(stop.clone(), connected.clone(), Box::new(Std1RuleSet));
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
        stop.clone(),
        connected.clone(),
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
