use civ_server::{
    effect::{CityEffect, Effect, StateEffect, UnitEffect},
    game::{city::City, unit::Unit},
    state::index::Index,
    test::{city::build_city, unit::build_unit},
};
use common::{geo::GeoVec, space::D2Size, utils::Vec2d};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_units(count: usize) -> Vec2d<Vec<Unit>> {
    let mut units = vec![];

    for i in 0..count {
        let unit = build_unit(i);
        units.push(GeoVec::new(unit.geo, vec![unit]));
    }

    Vec2d::from(D2Size::new(count, count), units)
}

fn build_cities(count: usize) -> Vec2d<Box<City>> {
    let mut cities = vec![];

    for i in 0..count {
        let city = build_city(i);
        cities.push(city);
    }

    Vec2d::from(D2Size::new(count, count), cities)
}

fn inject_units(index: &mut Index, units: &Vec2d<Vec<Unit>>, cities: &Vec2d<Box<City>>) {
    for units_ in units.iter().flatten() {
        for unit in units_ as &Vec<Unit> {
            index.apply(
                &vec![Effect::State(StateEffect::Unit(
                    *unit.id(),
                    UnitEffect::New(unit.clone()),
                ))],
                cities,
                units,
            );
        }
    }
}

fn inject_cities(index: &mut Index, cities: &Vec2d<Box<City>>, units: &Vec2d<Vec<Unit>>) {
    for city in cities.iter().flatten() {
        index.apply(
            &vec![Effect::State(StateEffect::City(
                *city.id(),
                CityEffect::New(*city.clone()),
            ))],
            cities,
            units,
        );
    }
}

pub fn bench_index_write_unit(c: &mut Criterion) {
    let mut group = c.benchmark_group("index_write");
    group.sample_size(10);

    let mut index = Index::default();
    let units = build_units(1);
    let cities = build_cities(1);
    group.bench_function("index_write_unit 1🚹", |b| {
        b.iter(|| inject_units(black_box(&mut index), black_box(&units), black_box(&cities)))
    });

    let mut index = Index::default();
    let units = build_units(1_000);
    let cities = build_cities(1_000);
    group.bench_function("index_write_unit 1k🚹", |b| {
        b.iter(|| inject_units(black_box(&mut index), black_box(&units), black_box(&cities)))
    });

    let mut index = Index::default();
    let units = build_units(10_000);
    let cities = build_cities(10_000);
    group.bench_function("index_write_unit 10k🚹", |b| {
        b.iter(|| inject_units(black_box(&mut index), black_box(&units), black_box(&cities)))
    });

    let mut index = Index::default();
    let units = build_units(1);
    let cities = build_cities(1);
    group.bench_function("index_write_city 1🏠", |b| {
        b.iter(|| inject_cities(black_box(&mut index), black_box(&cities), black_box(&units)))
    });

    let mut index = Index::default();
    let units = build_units(1);
    let cities = build_cities(1);
    group.bench_function("index_write_city 1k🏠", |b| {
        b.iter(|| inject_cities(black_box(&mut index), black_box(&cities), black_box(&units)))
    });

    let mut index = Index::default();
    let units = build_units(1);
    let cities = build_cities(1);
    group.bench_function("index_write_city 10k🏠", |b| {
        b.iter(|| inject_cities(black_box(&mut index), black_box(&cities), black_box(&units)))
    });
}

criterion_group!(benches, bench_index_write_unit);
criterion_main!(benches);
