use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use async_std::channel::{unbounded, Sender};
use civ_server::{
    config::ServerConfig,
    context::Context,
    game::placer::RandomPlacer,
    runner::{worker::setup_task_workers, Runner, RunnerContext},
    state::State,
    world::reader::WorldReader,
};
use common::{
    network::{
        message::{ClientToServerMessage, ClientToServerNetworkMessage, ServerToClientMessage},
        Client, ClientId,
    },
    rules::std1::Std1RuleSet,
    space::{window::Resolution, D2Size},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_messages(count: usize) -> Vec<ClientToServerMessage> {
    let mut messages = vec![];

    for _ in 0..count {
        messages.push(ClientToServerMessage::Network(
            ClientToServerNetworkMessage::Hello(Client::default(), Resolution::new(127, 128)),
        ))
    }

    messages
}

fn build_runner() -> (Runner, Sender<(Client, ClientToServerMessage)>) {
    let context = Context::new(Box::new(Std1RuleSet), ServerConfig::default());
    let state = Arc::new(RwLock::new(State::empty(D2Size::new(1, 1))));
    let world = WorldReader::new(PathBuf::new(), 0, 0, vec![]);
    let (from_clients_sender, from_clients_receiver) =
        unbounded::<(Client, ClientToServerMessage)>();
    let (to_clients_sender, _) = unbounded::<(ClientId, ServerToClientMessage)>();
    let runner_context = RunnerContext::new(
        context,
        state,
        Arc::new(RwLock::new(world)),
        from_clients_receiver,
        to_clients_sender,
        Box::new(RandomPlacer),
    );
    let mut runner = Runner::builder()
        .context(runner_context)
        .tick_base_period(1_000_000_000) // To ensure no wait before ticks
        .build();
    runner.task_workers = setup_task_workers(&runner.context);

    (runner, from_clients_sender)
}

fn send_messages(
    messages: Vec<ClientToServerMessage>,
    sender: Sender<(Client, ClientToServerMessage)>,
) {
    for message in messages {
        let client = match &message {
            ClientToServerMessage::Network(ClientToServerNetworkMessage::Hello(client, _)) => {
                *client
            }
            _ => unreachable!(),
        };
        sender.send_blocking((client, message)).unwrap();
    }
}

fn runner_client_messages(runner: &mut Runner, expected: usize) {
    runner.do_one_iteration();

    let client_count = runner.context.state().clients().clients_count();
    assert_eq!(client_count, expected);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("runner_client_messages");

    group.bench_function("runner_client_messages 1✉️", |b| {
        b.iter_with_setup(
            || {
                let messages = build_messages(1);
                let (runner, sender) = build_runner();
                send_messages(messages.clone(), sender.clone());
                runner
            },
            |mut runner| runner_client_messages(black_box(&mut runner), black_box(1)),
        )
    });

    group.bench_function("runner_client_messages 10k✉️", |b| {
        b.iter_with_setup(
            || {
                let messages = build_messages(10_000);
                let (runner, sender) = build_runner();
                send_messages(messages.clone(), sender.clone());
                runner
            },
            |mut runner| runner_client_messages(black_box(&mut runner), black_box(10_000)),
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
