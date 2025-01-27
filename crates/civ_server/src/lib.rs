use std::path::PathBuf;

use clap::Parser;
use common::network::{
    message::{ClientToServerMessage, ServerToClientMessage},
    Client, ClientId,
};
use crossbeam::channel::{Receiver, Sender};

pub mod config;
pub mod context;
pub mod effect;
pub mod game;
pub mod network;
pub mod reflect;
pub mod request;
pub mod runner;
pub mod snapshot;
pub mod state;
pub mod task;
pub mod test;
pub mod utils;

pub type FromClientsChannels = (
    Sender<(Client, ClientToServerMessage)>,
    Receiver<(Client, ClientToServerMessage)>,
);
pub type ToClientsChannels = (
    Sender<(ClientId, ServerToClientMessage)>,
    Receiver<(ClientId, ServerToClientMessage)>,
);

#[derive(Parser, Debug)]
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
