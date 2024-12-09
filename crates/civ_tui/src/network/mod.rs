use std::{io, thread, time::Duration};

use bon::Builder;
use common::{
    network::message::{ClientToServerEnveloppe, ClientToServerMessage, ServerToClientMessage},
    space::Window,
};
use crossbeam::channel::{Receiver, Sender};
use log::info;
use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeHandler, NodeListener},
};
use uuid::Uuid;

const SEND_INTERVAL: Duration = Duration::from_millis(25);

enum Signal {
    SendClientToServerMessages,
}

pub struct Network {
    client_id: Uuid,
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<ServerToClientMessage>,
    handler: NodeHandler<Signal>,
    node_listener: Option<NodeListener<Signal>>,
    server_endpoint: Endpoint,
}

// TODO: heartbeat
impl Network {
    pub fn new(
        client_id: Uuid,
        server_address: &str,
        to_server_receiver: Receiver<ClientToServerMessage>,
        from_server_sender: Sender<ServerToClientMessage>,
    ) -> io::Result<Self> {
        let (handler, node_listener) = node::split();

        let (server_endpoint, _) = handler
            .network()
            .connect(Transport::FramedTcp, server_address)?;

        Ok(Self {
            client_id,
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

        node_listener.for_each(move |event| match event {
            node::NodeEvent::Network(event) => match event {
                NetEvent::Connected(endpoint, _established) => {
                    info!("Connected");

                    //
                    let message =
                        bincode::serialize(&ClientToServerEnveloppe::Hello(self.client_id))
                            .unwrap();
                    self.handler.network().send(endpoint, &message);
                }
                NetEvent::Accepted(_, _) => {
                    info!("Accepted");
                }
                NetEvent::Message(_endpoint, input_data) => {
                    let message: ServerToClientMessage = bincode::deserialize(input_data).unwrap();
                    self.from_server_sender.send(message).unwrap();
                }
                NetEvent::Disconnected(_) => {
                    info!("Disconnected");
                    self.handler.stop();
                }
            },
            node::NodeEvent::Signal(signal) => {
                match signal {
                    Signal::SendClientToServerMessages => {
                        while let Ok(message) = self.to_server_receiver.try_recv() {
                            info!("Send message to server");
                            let data =
                                bincode::serialize(&ClientToServerEnveloppe::Message(message))
                                    .unwrap();
                            self.handler.network().send(self.server_endpoint, &data);
                        }
                        self.handler
                            .signals()
                            .send_with_timer(Signal::SendClientToServerMessages, SEND_INTERVAL);
                    }
                };
            }
        });
    }
}
