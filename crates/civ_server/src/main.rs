use std::{sync::Mutex, thread};

use common::network::message::FromClientMessage;
use context::Context;
use crossbeam::channel::{unbounded, Receiver, Sender};
use network::Network;
use runner::Runner;
use state::State;

mod context;
mod game;
mod network;
mod runner;
mod state;
mod task;
mod utils;

pub const TICK_BASE_PERIOD: u64 = 60;

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    let context = Mutex::new(Context::new());
    let state = Mutex::new(State::default());
    let (from_clients_sender, from_clients_receiver): (
        Sender<FromClientMessage>,
        Receiver<FromClientMessage>,
    ) = unbounded();

    let network = Network::builder()
        .clients_listener_address("tcp://127.0.0.1:9876".into())
        .from_clients_sender(from_clients_sender)
        .build();
    let mut runner = Runner::builder()
        .tick_base_period(TICK_BASE_PERIOD)
        .context(context)
        .state(state)
        .from_clients_receiver(from_clients_receiver)
        .build();

    let network = thread::spawn(move || network.run());
    let runner = thread::spawn(move || runner.run());

    network.join().unwrap();
    runner.join().unwrap();
}
