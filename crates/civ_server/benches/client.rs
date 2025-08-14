use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use async_std::channel::{unbounded, Sender};
use civ_server::{
    config::ServerConfig,
    context::Context,
    runner::{worker::setup_workers, Runner, RunnerContext},
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

fn build_waves(messages: usize, iterations: usize) -> Vec<Vec<ClientToServerMessage>> {
    let mut waves = vec![];

    for _ in 0..iterations {
        let mut messages_ = vec![];
        for _ in 0..messages {
            messages_.push(ClientToServerMessage::Network(
                ClientToServerNetworkMessage::Hello(Client::default(), Resolution::new(127, 128)),
            ))
        }
        waves.push(messages_.clone());
    }

    waves
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
    );
    let mut runner = Runner::builder()
        .context(runner_context)
        .tick_base_period(1_000_000_000) // To ensure no wait before ticks
        .build();
    runner.workers_channels = setup_workers(&runner.context);

    (runner, from_clients_sender)
}

fn runner_with_client_messages(
    runner: &mut Runner,
    waves: Vec<Vec<ClientToServerMessage>>,
    sender: Sender<(Client, ClientToServerMessage)>,
) {
    let client = Client::default();
    let mut counter = 0;
    let mut effects = vec![];

    for wave in waves {
        for message in wave {
            counter += 1;
            sender.send_blocking((client, message)).unwrap();
        }

        effects.extend(runner.clients());
    }

    assert_eq!(effects.len(), counter * 2); // Two effect per Hello expected
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let waves = build_waves(1, 1);
    let (mut runner, sender) = build_runner();
    c.bench_function("runner_client_messages 1✉️ 1➰", |b| {
        b.iter(|| {
            runner_with_client_messages(
                black_box(&mut runner),
                black_box(waves.clone()),
                black_box(sender.clone()),
            )
        })
    });

    let waves = build_waves(1_000, 1_000);
    let (mut runner, sender) = build_runner();
    c.bench_function("runner_client_messages 1k✉️ 1k➰", |b| {
        b.iter(|| {
            runner_with_client_messages(
                black_box(&mut runner),
                black_box(waves.clone()),
                black_box(sender.clone()),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
