use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::{
    game::PlayerId,
    network::{
        message::{ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage},
        Client, ClientId,
    },
};
use crossbeam::channel::{Receiver, Sender};
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeHandler, NodeListener},
};

const SEND_INTERVAL: Duration = Duration::from_millis(25);
const CHECK_STOP_INTERVAL: Duration = Duration::from_millis(250);

enum Signal {
    SendClientToServerMessages,
    CheckStopIsRequired,
}

pub struct NetworkClient {
    client_id: ClientId,
    player_id: PlayerId,
    stop: Arc<AtomicBool>,
    connected: Arc<AtomicBool>,
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
        stop: Arc<AtomicBool>,
        connected: Arc<AtomicBool>,
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
            stop,
            connected,
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
                    self.connected.swap(established, Ordering::Relaxed);

                    // Inform server about our uuid
                    let message = bincode::serialize(&ClientToServerMessage::Network(
                        ClientToServerNetworkMessage::Hello(Client::new(
                            self.client_id,
                            self.player_id,
                        )),
                    ))
                    .unwrap();
                    self.handler.network().send(endpoint, &message);
                }
                NetEvent::Accepted(_, _) => {}
                NetEvent::Message(_endpoint, input_data) => {
                    let message: ServerToClientMessage = bincode::deserialize(input_data).unwrap();
                    self.from_server_sender.send(message);
                }
                NetEvent::Disconnected(_) => {
                    self.connected.swap(false, Ordering::Relaxed);
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
                        if self.stop.load(Ordering::Relaxed) {
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
