use clients::Clients;
use common::network::message::{
    ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage,
};
use common::network::{Client, ClientId};
use crossbeam::channel::{Receiver, Sender};
use log::info;
use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeHandler, NodeListener};
use std::io;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use crate::context::Context;
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
    from_clients_sender: Sender<(Client, ClientToServerMessage)>,
    to_client_receiver: Receiver<(ClientId, ServerToClientMessage)>,
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
        tcp_listen_addr: &str,
        ws_listen_addr: &str,
        from_clients_sender: Sender<(Client, ClientToServerMessage)>,
        to_client_receiver: Receiver<(ClientId, ServerToClientMessage)>,
    ) -> io::Result<Self> {
        let (handler, node_listener) = node::split::<Signal>();

        info!(
            "Server will try to listent TCP {} and Ws {}",
            tcp_listen_addr, ws_listen_addr
        );
        handler
            .network()
            .listen(Transport::FramedTcp, tcp_listen_addr)?;
        handler.network().listen(Transport::Ws, ws_listen_addr)?;

        info!(
            "Server running and listening TCP {} and Ws {}",
            tcp_listen_addr, ws_listen_addr
        );
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
                    match &message {
                        ClientToServerMessage::Network(message_) => match &message_ {
                            ClientToServerNetworkMessage::Hello(client, _) => {
                                info!("DEBUG: client hello");
                                self.clients.insert(*client, endpoint);
                                self.from_clients_sender
                                    .send((*client, message.clone()))
                                    .unwrap();
                            }
                            ClientToServerNetworkMessage::Goodbye => {
                                info!("DEBUG: client goodbye");
                                self.clients.remove(&endpoint);
                                self.state
                                    .write()
                                    .expect("Assume state is always accessible")
                                    .clients_mut()
                                    .set_count(self.clients.length());
                            }
                        },
                        ClientToServerMessage::Game(_message) => {
                            info!("DEBUG game clients: {:?}", self.clients);
                            let client = self.clients.client_for_endpoint(&endpoint).unwrap();
                            self.from_clients_sender
                                .send((*client, message.clone()))
                                .unwrap();
                        }
                    }
                }
                NetEvent::Disconnected(endpoint) => {
                    info!("DEBUG: client disconnected");
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
