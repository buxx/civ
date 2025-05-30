use async_std::channel::{Receiver, Sender};
use bevy::prelude::*;
use common::network::message::{ClientToServerMessage, ServerToClientMessage};
use common::network::ServerAddress;
use std::sync::mpsc::{channel, Receiver as SyncReceiver, Sender as SyncSender};

use message_io::network::Endpoint;

use super::BridgeMessage;

use crate::bridge::InternalBridgeMessage;

use message_io::node;
use message_io::node::NodeHandler;
use message_io::{
    network::{NetEvent, Transport},
    node::NodeEvent,
};
use std::sync::{Arc, Mutex};
use std::thread;

enum Signal {
    Connected,
    Disconnected,
    ClientToServerMessageAvailable,
}

pub fn connect(
    address: ServerAddress,
    to_server_receiver: Receiver<ClientToServerMessage>,
    from_server_sender: Sender<BridgeMessage>,
) {
    let handler: Arc<Mutex<Option<NodeHandler<Signal>>>> = Arc::new(Mutex::new(None));
    let server: Arc<Mutex<Option<Endpoint>>> = Arc::new(Mutex::new(None));

    let handler_ = handler.clone();
    let server_ = server.clone();
    let from_server_sender_ = from_server_sender.clone();
    let (bridge_sender, bridge_receiver): (
        SyncSender<ClientToServerMessage>,
        SyncReceiver<ClientToServerMessage>,
    ) = channel();
    thread::spawn(move || {
        let (handler, listener) = node::split();

        let (server, _) = handler
            .network()
            .connect(Transport::FramedTcp, address.to_string())
            .unwrap();

        *handler_.lock().unwrap() = Some(handler.clone());
        *server_.lock().unwrap() = Some(server);

        listener.for_each(move |event| match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(_endpoint, _ok) => handler.signals().send(Signal::Connected),
                NetEvent::Accepted(_, _) => unreachable!(), // Only generated by listening
                NetEvent::Message(_endpoint, data) => {
                    let message: ServerToClientMessage = bincode::deserialize(data).unwrap();
                    from_server_sender_
                        .send_blocking(BridgeMessage::Server(message))
                        .unwrap();
                }
                NetEvent::Disconnected(_endpoint) => {
                    //
                    handler.signals().send(Signal::Disconnected)
                }
            },
            NodeEvent::Signal(signal) => match signal {
                Signal::Connected => {
                    info!("Connected to {}", &address);
                    from_server_sender_
                        .send_blocking(BridgeMessage::Internal(
                            InternalBridgeMessage::ConnectionEstablished(address.clone()),
                        ))
                        .unwrap();
                }
                Signal::Disconnected => {
                    info!("Disconnected from {}", &address)
                }
                Signal::ClientToServerMessageAvailable => {
                    if let Ok(message) = bridge_receiver.try_recv() {
                        let message = bincode::serialize(&message).unwrap();
                        handler.network().send(server, &message);
                    }
                }
            },
        });
    });

    let to_server_receiver_ = to_server_receiver.clone();
    thread::spawn(move || {
        while let Ok(message) = to_server_receiver_.recv_blocking() {
            let handler = handler.lock().unwrap();
            let handler = handler.as_ref().unwrap();
            bridge_sender.send(message).unwrap();
            handler
                .signals()
                .send(Signal::ClientToServerMessageAvailable);
        }
    });
}
