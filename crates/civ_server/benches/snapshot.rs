use std::path::PathBuf;

use civ_server::{
    snapshot::Snapshot,
    state::{clients::Clients, index::Index, State},
    test::{city::build_city, task::build_task, unit::build_unit},
};
use common::game::GameFrame;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_snapshot(units_count: usize, cities_count: usize, tasks_count: usize) -> State {
    let units = (0..units_count).map(build_unit).collect();
    let cities = (0..cities_count).map(build_city).collect();
    let tasks = (0..tasks_count).map(|_| build_task()).collect();

    let index = Index::default();
    State::new(
        GameFrame(0),
        Clients::default(),
        index,
        tasks,
        cities,
        units,
        0,
    )
}

fn snapshot_state(state: &State) -> Snapshot {
    Snapshot::from(state)
}

fn dump_state(snapshot: &Snapshot) {
    snapshot.dump(&PathBuf::from("/dev/null")).unwrap();
}

pub fn bench_dump_snapshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("index_write");
    group.sample_size(100);

    let state = build_snapshot(100_000, 5_000, 1_000_000);
    group.bench_function("snapshot_state 100k🚹 5k🏠 1M🎯", |b| {
        b.iter(|| snapshot_state(black_box(&state)))
    });

    let snapshot = snapshot_state(&state);
    group.bench_function("write_snapshot 100k🚹 5k🏠 1M🎯", |b| {
        b.iter(|| dump_state(black_box(&snapshot)))
    });
}

criterion_group!(benches, bench_dump_snapshot);
criterion_main!(benches);
