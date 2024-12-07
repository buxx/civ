use std::thread;

use bon::Builder;
use common::network::message::FromClientMessage;
use crossbeam::channel::Sender;

pub mod message;

#[derive(Builder)]
pub struct Network {
    clients_listener_address: String,
    from_clients_sender: Sender<FromClientMessage>,
}

// TODO: unwraps
// TODO: stop required
impl Network {
    pub fn run(&self) {
        self.start_clients_listener();
    }

    fn start_clients_listener(&self) {
        let from_clients_sender = self.from_clients_sender.clone();
        let address = self.clients_listener_address.clone();
        let zmq_context = zmq::Context::new();
        let socket = zmq_context.socket(zmq::REP).unwrap();
        socket.bind(&address).unwrap();
        let ok = bincode::serialize(&1).unwrap();

        let clients_listener = thread::spawn(move || {
            loop {
                // Receive client REQ messages bytes
                let messages_bytes = match socket.recv_bytes(0) {
                    Ok(message_bytes) => message_bytes,
                    Err(_) => {
                        // TODO
                        continue;
                    }
                };

                // Decode received bytes into collection of messages
                let message: FromClientMessage = match bincode::deserialize(&messages_bytes) {
                    Ok(messages) => messages,
                    Err(error) => {
                        // TODO
                        continue;
                    }
                };

                // Send client expected acknowledgement
                socket.send(&ok, 0).unwrap();

                // Send through channel the decoded messages
                from_clients_sender.send(message).unwrap();
            }
        });

        clients_listener.join().unwrap();
    }
}
