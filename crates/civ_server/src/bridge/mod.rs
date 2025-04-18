use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use common::network::{Client, ClientId};
use crossbeam::channel::{Receiver, Sender};
use std::io;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;

use crate::config::ServerConfig;
use crate::context::Context;
use crate::state::State;

mod clients;
pub mod network;

const SEND_INTERVAL: Duration = Duration::from_millis(25);
const CHECK_STOP_INTERVAL: Duration = Duration::from_millis(250);

pub type FromClientsChannels = (
    Sender<(Client, ClientToServerMessage)>,
    Receiver<(Client, ClientToServerMessage)>,
);
pub type ToClientsChannels = (
    Sender<(ClientId, ServerToClientMessage)>,
    Receiver<(ClientId, ServerToClientMessage)>,
);

pub trait BridgeBuilder<T> {
    fn build(
        &self,
        context: Context,
        state: Arc<RwLock<State>>,
        config: &ServerConfig,
    ) -> Result<
        (
            T,
            Receiver<(Client, ClientToServerMessage)>,
            Sender<(ClientId, ServerToClientMessage)>,
        ),
        BridgeBuildError,
    >;
}

#[derive(Debug, Error)]
pub enum BridgeBuildError {
    #[error("Io: {0}")]
    Io(io::ErrorKind),
}

pub trait Bridge: Send {
    fn run(&mut self);
}
