use super::clients::Clients;
use super::network::NetworkBridge;
use super::{Bridge, BridgeBuildError, BridgeBuilder, FromClientsChannels, ToClientsChannels};
use common::network::message::{
    ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage,
};
use common::network::{Client, ClientId};
use crossbeam::channel::{unbounded, Receiver, Sender};
use derive_more::Constructor;
use log::info;
use std::sync::{Arc, RwLock};
use std::{io, thread};

use crate::config::ServerConfig;
use crate::context::Context;
use crate::state::State;

#[derive(Debug, Constructor)]
pub struct DirectBridgeBuilder {
    client_to_server_sender: Sender<(Client, ClientToServerMessage)>,
    client_to_server_receiver: Receiver<(Client, ClientToServerMessage)>,
    server_to_client_sender: Sender<(ClientId, ServerToClientMessage)>,
    server_to_client_receiver: Receiver<(ClientId, ServerToClientMessage)>,
}

impl BridgeBuilder<NetworkBridge> for DirectBridgeBuilder {
    fn build(
        &self,
        context: Context,
        state: Arc<RwLock<State>>,
        config: &ServerConfig,
    ) -> Result<
        (
            NetworkBridge,
            Receiver<(Client, ClientToServerMessage)>,
            Sender<(ClientId, ServerToClientMessage)>,
        ),
        BridgeBuildError,
    > {
        let (from_clients_sender, from_clients_receiver): FromClientsChannels = unbounded();
        let (to_clients_sender, to_clients_receiver): ToClientsChannels = unbounded();
        let bridge = NetworkBridge::new(
            context.clone(),
            Arc::clone(&state),
            config.tcp_listen_address().to_string(),
            config.ws_listen_address().to_string(),
            from_clients_sender,
            to_clients_receiver,
        )
        .map_err(|e| BridgeBuildError::Io(e.kind()))?;
        Ok((bridge, from_clients_receiver, to_clients_sender))
    }
}

enum Signal {
    SendServerToClientsMessages,
    CheckStopRequired,
}

pub struct DirectBridge {
    state: Arc<RwLock<State>>,
    from_clients_sender: Sender<(Client, ClientToServerMessage)>,
    to_client_receiver: Receiver<(ClientId, ServerToClientMessage)>,
    clients: Clients,
}

// TODO: unwraps
// TODO: stop required
impl DirectBridge {
    pub fn new(
        state: Arc<RwLock<State>>,
        from_clients_sender: Sender<(Client, ClientToServerMessage)>,
        to_client_receiver: Receiver<(ClientId, ServerToClientMessage)>,
    ) -> io::Result<Self> {
        Ok(Self {
            state,
            from_clients_sender,
            to_client_receiver,
            clients: Clients::default(),
        })
    }
}

impl Bridge for DirectBridge {
    fn run(&mut self) {
        let from_clients_sender = self.from_clients_sender.clone();

        thread::spawn(move || {
            // while let Ok(message) = x {
            //     //
            // }
        });

        info!("Direct finished running");
    }
}
