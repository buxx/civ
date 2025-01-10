use civ_server::{
    effect::{CityEffect, Effect, StateEffect, UnitEffect},
    game::{city::City, task::production::CityProductionTask, unit::Unit},
    state::index::Index,
    task::{city::CityTasks, TaskContext},
};
use common::{
    game::{
        city::{CityExploitation, CityProduction, CityProductionTons},
        unit::UnitType,
        GameFrame,
    },
    geo::{GeoContext, WorldPoint},
    space::window::{DisplayStep, Window},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use uuid::Uuid;

fn build_unit(i: usize) -> Unit {
    Unit::builder()
        .id(Uuid::new_v4())
        .geo(
            GeoContext::builder()
                .point(WorldPoint::new(i as u64, i as u64))
                .build(),
        )
        .type_(UnitType::Warriors)
        .build()
}

fn inject_units(index: &mut Index, unit_count: usize, cities: &[City]) -> Vec<Unit> {
    let mut units = vec![];

    for i in 0..unit_count {
        let unit = build_unit(i);
        units.push(unit.clone());

        index.apply(
            &vec![Effect::State(StateEffect::Unit(
                unit.id(),
                UnitEffect::New(unit.clone()),
            ))],
            cities,
            &units,
        );
    }

    units
}

fn build_city(i: usize) -> City {
    let city_uuid = Uuid::new_v4();
    City::builder()
        .id(city_uuid)
        .name("CityName".to_string())
        .geo(
            GeoContext::builder()
                .point(WorldPoint::new(i as u64, i as u64))
                .build(),
        )
        .production(CityProduction::new(vec![]))
        .exploitation(CityExploitation::new(CityProductionTons(1)))
        .tasks(
            CityTasks::builder()
                .production(
                    CityProductionTask::builder()
                        .city(city_uuid)
                        .context(
                            TaskContext::builder()
                                .id(Uuid::new_v4())
                                .start(GameFrame(0))
                                .end(GameFrame(1))
                                .build(),
                        )
                        .tons(CityProductionTons(1))
                        .build(),
                )
                .build(),
        )
        .build()
}

fn inject_cities(index: &mut Index, city_count: usize, units: &[Unit]) -> Vec<City> {
    let mut cities = vec![];

    for i in 0..city_count {
        let city = build_city(i);
        cities.push(city.clone());

        index.apply(
            &vec![Effect::State(StateEffect::City(
                *city.id(),
                CityEffect::New(city.clone()),
            ))],
            &cities,
            units,
        );
    }

    cities
}

fn inject_massive(index: &mut Index, count: usize) {
    let mut cities = vec![];
    let mut units = vec![];

    for i in 0..count {
        cities.push(build_city(i));
        units.push(build_unit(i));
    }

    index.reindex_cities(&cities);
    index.reindex_units(&units);
}

fn index_write_unit(unit_count: usize, cities: &[City]) {
    let mut index = Index::default();
    inject_units(&mut index, unit_count, cities);
}

fn index_write_city(city_count: usize, units: &[Unit]) {
    let mut index = Index::default();
    inject_cities(&mut index, city_count, units);
}

fn index_xy_window(index: &Index, xy_end: u64) {
    let mut city_counter = 0;
    let mut unit_counter = 0;

    for _city in index.xy_cities(&Window::new(0, 0, xy_end, xy_end, DisplayStep::Close)) {
        city_counter += 1;
    }
    for _unit in index.xy_units(&Window::new(0, 0, xy_end, xy_end, DisplayStep::Close)) {
        unit_counter += 1;
    }

    assert_eq!(city_counter, xy_end);
    assert_eq!(unit_counter, xy_end);
}

pub fn bench_index_write_unit(c: &mut Criterion) {
    let mut group = c.benchmark_group("index_write");
    group.sample_size(10);

    group.bench_function("index_write_unit 1🚹", |b| {
        b.iter(|| index_write_unit(black_box(1), &[]))
    });
    group.bench_function("index_write_unit 1k🚹", |b| {
        b.iter(|| index_write_unit(black_box(1_000), &[]))
    });
    group.bench_function("index_write_unit 10k🚹", |b| {
        b.iter(|| index_write_unit(black_box(10_000), &[]))
    });

    group.bench_function("index_write_city 1🏠", |b| {
        b.iter(|| index_write_city(black_box(1), &[]))
    });
    group.bench_function("index_write_city 1k🏠", |b| {
        b.iter(|| index_write_city(black_box(1_000), &[]))
    });
    group.bench_function("index_write_city 10k🏠", |b| {
        b.iter(|| index_write_city(black_box(10_000), &[]))
    });
}
pub fn bench_index_xy_window(c: &mut Criterion) {
    let mut group = c.benchmark_group("index_xy_window");
    group.sample_size(10);

    let mut index = Index::default();
    inject_massive(&mut index, 1);
    group.bench_function("index_xy_window 1🚹 1🏠", |b| {
        b.iter(|| index_xy_window(black_box(&index), black_box(1)))
    });

    let mut index = Index::default();
    inject_massive(&mut index, 1_000);
    group.bench_function("index_xy_window 1k🚹 1k🏠", |b| {
        b.iter(|| index_xy_window(black_box(&index), black_box(1_000)))
    });

    let mut index = Index::default();
    inject_massive(&mut index, 10_000);
    group.bench_function("index_xy_window 10k🚹 10k🏠", |b| {
        b.iter(|| index_xy_window(black_box(&index), black_box(10_000)))
    });
}

criterion_group!(benches, bench_index_write_unit, bench_index_xy_window);
criterion_main!(benches);
