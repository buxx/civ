use clients::Clients;
use common::network::message::{
    ClientToServerGameMessage, ClientToServerMessage, ClientToServerNetworkMessage,
    ServerToClientEstablishmentMessage, ServerToClientMessage,
};
use common::network::Client;
use crossbeam::channel::{Receiver, Sender};
use log::info;
use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeHandler, NodeListener};
use std::io;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use uuid::Uuid;

use crate::context::Context;
use crate::effect::ClientEffect;
use crate::state::State;

mod clients;

const SEND_INTERVAL: Duration = Duration::from_millis(25);
const CHECK_STOP_INTERVAL: Duration = Duration::from_millis(250);

enum Signal {
    SendServerToClientsMessages,
    CheckStopRequired,
}

pub struct Network {
    context: Context,
    state: Arc<RwLock<State>>,
    from_clients_sender: Sender<(Client, ClientToServerGameMessage)>,
    to_client_receiver: Receiver<(Uuid, ServerToClientMessage)>,
    handler: NodeHandler<Signal>,
    node_listener: NodeListener<Signal>,
    clients: Clients,
}

// TODO: unwraps
// TODO: stop required
impl Network {
    pub fn new(
        context: Context,
        state: Arc<RwLock<State>>,
        listen_addr: &str,
        from_clients_sender: Sender<(Client, ClientToServerGameMessage)>,
        to_client_receiver: Receiver<(Uuid, ServerToClientMessage)>,
    ) -> io::Result<Self> {
        let (handler, node_listener) = node::split::<Signal>();
        handler
            .network()
            .listen(Transport::FramedTcp, listen_addr)?;

        info!("Network server running at {}", listen_addr);
        Ok(Self {
            context,
            state,
            from_clients_sender,
            to_client_receiver,
            handler,
            node_listener,
            clients: Clients::default(),
        })
    }

    pub fn run(mut self) {
        let node_listener = self.node_listener;

        // TODO : Trigger signal to start the signal loop of sending messages to clients
        // This could probably be enhanced for better performances. To check ...
        self.handler
            .signals()
            .send(Signal::SendServerToClientsMessages);
        self.handler.signals().send(Signal::CheckStopRequired);

        node_listener.for_each(move |event| match event {
            node::NodeEvent::Network(event) => match event {
                NetEvent::Connected(_, _) => unreachable!(), // There is no connect() calls.
                NetEvent::Accepted(_, _) => {}
                NetEvent::Message(endpoint, input_data) => {
                    let message: ClientToServerMessage = bincode::deserialize(input_data).unwrap();
                    match message {
                        ClientToServerMessage::Network(message) => match message {
                            // FIXME: this Hello show we must write into state, probably we should
                            // send message through from_clients_sender (bad name so) ?
                            ClientToServerNetworkMessage::Hello(client) => {
                                self.clients.insert(client, endpoint);
                                self.state
                                    .write()
                                    .expect("Assume state is always accessible")
                                    .clients_mut()
                                    .set_count(self.clients.length());

                                let state = self
                                    .state
                                    .read()
                                    .expect("Consider state is always accessible");
                                let server_resume = state.server_resume(self.context.rules());
                                let player_flag = state
                                    .clients()
                                    .player_state(client.player_id())
                                    .map(|s| s.flag())
                                    .cloned();

                                let message = ServerToClientMessage::Establishment(
                                    ServerToClientEstablishmentMessage::ServerResume(
                                        server_resume,
                                        player_flag,
                                    ),
                                );
                                let data = bincode::serialize(&message).unwrap();
                                self.handler.network().send(endpoint, &data);
                            }
                            ClientToServerNetworkMessage::Goodbye => {
                                self.clients.remove(&endpoint);
                                self.state
                                    .write()
                                    .expect("Assume state is always accessible")
                                    .clients_mut()
                                    .set_count(self.clients.length());
                            }
                        },
                        ClientToServerMessage::Game(message) => {
                            let client = self.clients.client_for_endpoint(&endpoint).unwrap();
                            self.from_clients_sender.send((*client, message)).unwrap();
                        }
                    }
                }
                NetEvent::Disconnected(endpoint) => {
                    self.clients.remove(&endpoint);
                    self.state
                        .write()
                        .expect("Assume state is always accessible")
                        .clients_mut()
                        .set_count(self.clients.length());
                }
            },
            node::NodeEvent::Signal(signal) => {
                match signal {
                    Signal::SendServerToClientsMessages => {
                        while let Ok((client_id, message)) = self.to_client_receiver.try_recv() {
                            if let Some(endpoint) = self.clients.endpoint(&client_id) {
                                let data = bincode::serialize(&message).unwrap();
                                self.handler.network().send(*endpoint, &data);
                            }
                        }
                        self.handler
                            .signals()
                            .send_with_timer(Signal::SendServerToClientsMessages, SEND_INTERVAL);
                    }
                    Signal::CheckStopRequired => {
                        if self.context.stop_is_required() {
                            self.handler.stop();
                        }
                        self.handler.signals().send_with_timer(
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
