use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use civ_server::{
    config::ServerConfig,
    context::Context,
    runner::{Runner, RunnerContext},
    state::State,
    task::{TaskContext, TaskId},
    test::task::{fibonacci, FibonacciTask},
    FromClientsChannels, ToClientsChannels,
};
use common::{game::GameFrame, rules::std1::Std1RuleSet, world::reader::WorldReader};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam::channel::unbounded;

fn runner(context: Context, state: Arc<RwLock<State>>) -> Runner {
    let world = WorldReader::new(PathBuf::new(), 0, 0, vec![]);
    let (_, from_clients_receiver): FromClientsChannels = unbounded();
    let (to_clients_sender, _): ToClientsChannels = unbounded();
    let runner_context = RunnerContext::new(
        context,
        state,
        Arc::new(RwLock::new(world)),
        from_clients_receiver,
        to_clients_sender,
    );

    Runner::builder()
        .context(runner_context)
        .tick_base_period(1_000_000_000) // To ensure no wait before ticks
        .build()
}

fn runner_with_fibonacci_tasks(tasks_count: usize, complexity: u64, iterations: usize) {
    let context = Context::new(Box::new(Std1RuleSet), ServerConfig::default());
    let mut state = State::default();
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
    let mut runner = runner(context.clone(), Arc::clone(&state));
    runner.setup_workers();

    for _ in 0..iterations {
        runner.do_one_iteration();
        assert!(!context.stop_is_required());
    }

    let expected_testing_value = (tasks_count * iterations) as u64;
    let testing_value = state.read().unwrap().testing();
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
        b.iter(|| runner_with_fibonacci_tasks(black_box(8), black_box(1), black_box(1000)))
    });
    c.bench_function("runner_with_fibonacci_tasks 1k🎯 1🧠 1k➰", |b| {
        b.iter(|| runner_with_fibonacci_tasks(black_box(1000), black_box(1), black_box(1000)))
    });
    c.bench_function("runner_with_fibonacci_tasks 10k🎯 1🧠 1k➰", |b| {
        b.iter(|| runner_with_fibonacci_tasks(black_box(10000), black_box(1), black_box(1000)))
    });

    c.bench_function("runner_with_fibonacci_tasks 8🎯 1M🧠 1k➰", |b| {
        b.iter(|| runner_with_fibonacci_tasks(black_box(8), black_box(1_000_000), black_box(1000)))
    });
    c.bench_function("runner_with_fibonacci_tasks 1k🎯 1M🧠 1k➰", |b| {
        b.iter(|| {
            runner_with_fibonacci_tasks(black_box(1000), black_box(1_000_000), black_box(1000))
        })
    });
    c.bench_function("runner_with_fibonacci_tasks 10k🎯 1M🧠 1k➰", |b| {
        b.iter(|| {
            runner_with_fibonacci_tasks(black_box(10000), black_box(1_000_000), black_box(1000))
        })
    });

    c.bench_function("runner_with_fibonacci_tasks 8🎯 1G🧠 1k➰", |b| {
        b.iter(|| {
            runner_with_fibonacci_tasks(black_box(8), black_box(1_000_000_000), black_box(1000))
        })
    });
    c.bench_function("runner_with_fibonacci_tasks 1k🎯 1G🧠 1k➰", |b| {
        b.iter(|| {
            runner_with_fibonacci_tasks(black_box(1000), black_box(1_000_000_000), black_box(1000))
        })
    });
    c.bench_function("runner_with_fibonacci_tasks 10k🎯 1G🧠 1k➰", |b| {
        b.iter(|| {
            runner_with_fibonacci_tasks(black_box(10000), black_box(1_000_000_000), black_box(1000))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
