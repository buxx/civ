use std::{
    io,
    sync::{Arc, RwLock},
    time::Duration,
};

use common::{
    game::PlayerId,
    network::{
        message::{ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage},
        Client, ClientId,
    },
    space::window::Resolution,
};
use crossbeam::channel::{Receiver, Sender};
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeHandler, NodeListener},
};

use crate::{context::Context, state::State};

const SEND_INTERVAL: Duration = Duration::from_millis(25);
const CHECK_STOP_INTERVAL: Duration = Duration::from_millis(250);

enum Signal {
    SendClientToServerMessages,
    CheckStopIsRequired,
}

pub struct NetworkClient {
    client_id: ClientId,
    player_id: PlayerId,
    context: Context,
    state: Arc<RwLock<State>>,
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<ServerToClientMessage>,
    handler: NodeHandler<Signal>,
    node_listener: Option<NodeListener<Signal>>,
    server_endpoint: Endpoint,
}

// TODO: heartbeat
impl NetworkClient {
    pub fn new(
        client_id: ClientId,
        player_id: PlayerId,
        server_address: &str,
        context: Context,
        state: Arc<RwLock<State>>,
        to_server_receiver: Receiver<ClientToServerMessage>,
        from_server_sender: Sender<ServerToClientMessage>,
    ) -> io::Result<Self> {
        let (handler, node_listener) = node::split();

        let (server_endpoint, _) = handler
            .network()
            .connect(Transport::FramedTcp, server_address)?;

        Ok(Self {
            client_id,
            player_id,
            context,
            state,
            to_server_receiver,
            from_server_sender,
            handler,
            node_listener: Some(node_listener),
            server_endpoint,
        })
    }

    pub fn run(mut self) {
        let node_listener = self.node_listener.take().unwrap();

        // TODO : Trigger signal to start the signal loop of sending messages to server
        // This could probably be enhanced for better performances. To check ...
        self.handler
            .signals()
            .send(Signal::SendClientToServerMessages);
        self.handler.signals().send(Signal::CheckStopIsRequired);

        node_listener.for_each(move |event| match event {
            node::NodeEvent::Network(event) => match event {
                NetEvent::Connected(endpoint, established) => {
                    let mut state = self
                        .state
                        .write()
                        .expect("Assume state is always accessible");
                    state.set_connected(established);

                    // Inform server about our uuid
                    let message = bincode::serialize(&ClientToServerMessage::Network(
                        ClientToServerNetworkMessage::Hello(
                            Client::new(self.client_id, self.player_id),
                            Resolution::new(1, 1),
                        ),
                    ))
                    .unwrap();
                    self.handler.network().send(endpoint, &message);
                }
                NetEvent::Accepted(_, _) => {}
                NetEvent::Message(_endpoint, input_data) => {
                    let message: ServerToClientMessage = bincode::deserialize(input_data).unwrap();
                    self.from_server_sender.send(message).unwrap();
                }
                NetEvent::Disconnected(_) => {
                    let mut state = self
                        .state
                        .write()
                        .expect("Assume state is always accessible");
                    state.set_connected(false);
                    self.handler.stop();
                }
            },
            node::NodeEvent::Signal(signal) => {
                match signal {
                    Signal::SendClientToServerMessages => {
                        while let Ok(message) = self.to_server_receiver.try_recv() {
                            let data = bincode::serialize(&message).unwrap();
                            self.handler.network().send(self.server_endpoint, &data);
                        }
                        self.handler
                            .signals()
                            .send_with_timer(Signal::SendClientToServerMessages, SEND_INTERVAL);
                    }
                    Signal::CheckStopIsRequired => {
                        if self.context.stop_is_required() {
                            self.handler.stop();
                        }
                        self.handler
                            .signals()
                            .send_with_timer(Signal::CheckStopIsRequired, CHECK_STOP_INTERVAL);
                    }
                };
            }
        });
    }
}
