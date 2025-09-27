use super::clients::Clients;
use super::{Bridge, BridgeBuildError, BridgeBuilder, FromClientsChannels, ToClientsChannels};
use async_std::channel::{unbounded, Receiver, Sender};
use common::network::message::{
    ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage,
};
use common::network::{Client, ClientId};
use log::{debug, info};
use message_io::network::{NetEvent, Transport};
use message_io::node::{self};
use std::io;
use std::sync::{Arc, RwLock};

use crate::bridge::{CHECK_STOP_INTERVAL, SEND_INTERVAL};
use crate::config::ServerConfig;
use crate::context::Context;
use crate::state::State;

#[derive(Debug, Default)]
pub struct NetworkBridgeBuilder;

impl BridgeBuilder<NetworkBridge> for NetworkBridgeBuilder {
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

pub struct NetworkBridge {
    context: Context,
    _state: Arc<RwLock<State>>,
    from_clients_sender: Sender<(Client, ClientToServerMessage)>,
    to_client_receiver: Receiver<(ClientId, ServerToClientMessage)>,
    tcp_listen_addr: String,
    ws_listen_addr: String,
    clients: Clients,
}

// TODO: unwraps
// TODO: stop required
impl NetworkBridge {
    pub fn new(
        context: Context,
        state: Arc<RwLock<State>>,
        tcp_listen_addr: String,
        ws_listen_addr: String,
        from_clients_sender: Sender<(Client, ClientToServerMessage)>,
        to_client_receiver: Receiver<(ClientId, ServerToClientMessage)>,
    ) -> io::Result<Self> {
        Ok(Self {
            context,
            _state: state,
            from_clients_sender,
            to_client_receiver,
            tcp_listen_addr,
            ws_listen_addr,
            clients: Clients::default(),
        })
    }
}

impl Bridge for NetworkBridge {
    fn run(&mut self) {
        let (handler, node_listener) = node::split::<Signal>();

        info!(
            "Starting TCP {} and Ws {}",
            &self.tcp_listen_addr, &self.ws_listen_addr
        );
        handler
            .network()
            .listen(Transport::FramedTcp, &self.tcp_listen_addr)
            .unwrap();
        handler
            .network()
            .listen(Transport::Ws, &self.ws_listen_addr)
            .unwrap();

        // TODO : Trigger signal to start the signal loop of sending messages to clients
        // This could probably be enhanced for better performances. To check ...
        handler.signals().send(Signal::SendServerToClientsMessages);
        handler.signals().send(Signal::CheckStopRequired);

        node_listener.for_each(move |event| match event {
            node::NodeEvent::Network(event) => match event {
                NetEvent::Connected(_, _) => unreachable!(), // There is no connect() calls.
                NetEvent::Accepted(_, _) => {}
                NetEvent::Message(endpoint, input_data) => {
                    let message = match bincode::deserialize::<ClientToServerMessage>(input_data) {
                        Ok(message) => message,
                        Err(error) => {
                            log::error!("Receive error: {error}");
                            return;
                        }
                    };
                    dbg!(&message);

                    match &message {
                        ClientToServerMessage::Network(message_) => match &message_ {
                            ClientToServerNetworkMessage::Hello(client, _) => {
                                debug!(
                                    "Client hello ({}, {})",
                                    client.client_id(),
                                    client.player_id()
                                );
                                self.clients.insert(*client, endpoint);
                                self.from_clients_sender
                                    .send_blocking((*client, message.clone()))
                                    .unwrap();
                            }
                            ClientToServerNetworkMessage::Goodbye => {
                                debug!("Client goodbye");
                                self.clients.remove(&endpoint);
                            }
                        },
                        ClientToServerMessage::Game(_message) => {
                            let client = self.clients.client_for_endpoint(&endpoint).unwrap();
                            self.from_clients_sender
                                .send_blocking((*client, message.clone()))
                                .unwrap();
                        }
                    }
                }
                NetEvent::Disconnected(endpoint) => {
                    debug!("Client disconnected");
                    self.clients.remove(&endpoint);
                }
            },
            node::NodeEvent::Signal(signal) => {
                match signal {
                    Signal::SendServerToClientsMessages => {
                        while let Ok((client_id, message)) = self.to_client_receiver.try_recv() {
                            if let Some(endpoint) = self.clients.endpoint(&client_id) {
                                let data = bincode::serialize(&message).unwrap();
                                handler.network().send(*endpoint, &data);
                            }
                        }
                        handler
                            .signals()
                            .send_with_timer(Signal::SendServerToClientsMessages, SEND_INTERVAL);
                    }
                    Signal::CheckStopRequired => {
                        if self.context.stop_is_required() {
                            handler.stop();
                        }
                        handler.signals().send_with_timer(
                            Signal::SendServerToClientsMessages,
                            CHECK_STOP_INTERVAL,
                        );
                    }
                };
            }
        });

        info!("Network server finished running");
    }
}
