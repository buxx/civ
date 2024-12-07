use common::network::message::ClientToServerMessage;
use crossbeam::channel::{unbounded, Receiver, Sender};
use network::Network;
use uuid::Uuid;

mod network;

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let (to_server_sender, to_server_receiver): (
        Sender<ClientToServerMessage>,
        Receiver<ClientToServerMessage>,
    ) = unbounded();

    let client_id = Uuid::new_v4();
    let runner = Network::builder()
        .client_id(client_id)
        .to_server_address("tcp://127.0.0.1:9876".into())
        .to_server_receiver(to_server_receiver)
        .build();

    runner.run();
}
