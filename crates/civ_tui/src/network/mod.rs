use std::thread;

use bon::Builder;
use common::network::message::ClientToServerMessage;
use crossbeam::channel::Receiver;
use uuid::Uuid;

#[derive(Builder)]
pub struct Network {
    client_id: Uuid,
    to_server_address: String,
    to_server_receiver: Receiver<ClientToServerMessage>,
}

// TODO: heartbeat
impl Network {
    pub fn run(&self) {
        self.start_server_sender();
    }

    fn start_server_sender(&self) {
        let to_server_receiver = self.to_server_receiver.clone();
        let address = self.to_server_address.clone();
        let zmq_context = zmq::Context::new();
        let socket = zmq_context.socket(zmq::REQ).unwrap();
        socket.connect(&address).unwrap();

        let hello: Vec<u8> =
            bincode::serialize(&ClientToServerMessage::Hello(self.client_id)).unwrap();
        socket.send(hello, 0).unwrap();
        // TODO: timeout ?
        if socket.recv_bytes(0).is_err() {
            panic!("TODO");
        }

        for message in to_server_receiver {}
    }
}
