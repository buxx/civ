use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use async_std::channel::unbounded;
use civ_server::{
    config::ServerConfig,
    context::Context,
    game::placer::RandomPlacer,
    runner::{worker::setup_task_workers, Runner, RunnerContext},
    state::State,
    task::{TaskContext, TaskId},
    test::task::{fibonacci, FibonacciTask},
    world::reader::WorldReader,
};
use common::{
    game::GameFrame,
    network::{
        message::{ClientToServerMessage, ServerToClientMessage},
        Client, ClientId,
    },
    rules::std1::Std1RuleSet,
    space::D2Size,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_runner(tasks_count: usize, complexity: u64) -> Runner {
    let context = Context::new(Box::new(Std1RuleSet), ServerConfig::default());
    let mut state = State::empty(D2Size::new(1, 1));
    for _ in 0..tasks_count {
        state.tasks_mut().push(Box::new(FibonacciTask::new(
            TaskContext::builder()
                .id(TaskId::default())
                .start(GameFrame(0))
                .end(GameFrame(1_000_000_000))
                .build(),
            complexity,
        )));
    }

    let state = Arc::new(RwLock::new(state));
    let world = WorldReader::new(PathBuf::new(), 0, 0, vec![]);
    let (_, from_clients_receiver) = unbounded::<(Client, ClientToServerMessage)>();
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
    runner
}

fn run(runner: &mut Runner, tasks_count: usize, iterations: usize) {
    for _ in 0..iterations {
        runner.do_one_iteration();
    }

    let expected_testing_value = (tasks_count * iterations) as u64;
    let testing_value = runner.context.state().testing();
    assert_eq!(testing_value, expected_testing_value);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // fibonacci
    c.bench_function("fibonacci 1🧠 ", |b| b.iter(|| fibonacci(black_box(1))));
    c.bench_function("fibonacci 1M🧠 ", |b| {
        b.iter(|| fibonacci(black_box(1_000_000)))
    });
    c.bench_function("fibonacci 1G🧠 ", |b| {
        b.iter(|| fibonacci(black_box(1_000_000_000)))
    });

    // runner_with_fibonacci_tasks
    c.bench_function("runner_with_fibonacci_tasks 8🎯 1🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(8, 1),
            |mut runner| run(black_box(&mut runner), black_box(8), black_box(1_000)),
        )
    });
    c.bench_function("runner_with_fibonacci_tasks 1k🎯 1🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(1_000, 1),
            |mut runner| run(black_box(&mut runner), black_box(1_000), black_box(1_000)),
        )
    });
    c.bench_function("runner_with_fibonacci_tasks 10k🎯 1🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(10_000, 1),
            |mut runner| run(black_box(&mut runner), black_box(10_000), black_box(1_000)),
        )
    });

    c.bench_function("runner_with_fibonacci_tasks 8🎯 1M🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(8, 1_000_000),
            |mut runner| run(black_box(&mut runner), black_box(8), black_box(1_000)),
        )
    });
    c.bench_function("runner_with_fibonacci_tasks 1k🎯 1M🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(1_000, 1_000_000),
            |mut runner| run(black_box(&mut runner), black_box(1_000), black_box(1_000)),
        )
    });
    c.bench_function("runner_with_fibonacci_tasks 10k🎯 1M🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(10_000, 1_000_000),
            |mut runner| run(black_box(&mut runner), black_box(10_000), black_box(1_000)),
        )
    });

    c.bench_function("runner_with_fibonacci_tasks 8🎯 1G🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(8, 1_000_000_000),
            |mut runner| run(black_box(&mut runner), black_box(8), black_box(1_000)),
        )
    });
    c.bench_function("runner_with_fibonacci_tasks 1k🎯 1G🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(1_000, 1_000_000_000),
            |mut runner| run(black_box(&mut runner), black_box(1_000), black_box(1_000)),
        )
    });
    c.bench_function("runner_with_fibonacci_tasks 10k🎯 1G🧠 1k➰", |b| {
        b.iter_with_setup(
            || build_runner(10_000, 1_000_000_000),
            |mut runner| run(black_box(&mut runner), black_box(10_000), black_box(1_000)),
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
