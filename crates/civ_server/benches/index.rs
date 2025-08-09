use civ_server::{
    effect::{CityEffect, Effect, StateEffect, UnitEffect},
    game::{city::City, unit::Unit},
    state::index::Index,
    test::{city::build_city, unit::build_unit},
};
use common::{geo::GeoVec, space::D2Size, utils::Vec2d};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn inject_units(index: &mut Index, unit_count: usize, cities: &Vec2d<City>) {
    let mut units = vec![];

    for i in 0..unit_count {
        let unit = build_unit(i);
        units.push(GeoVec::new(unit.geo, vec![unit]));
    }

    let units = Vec2d::from(D2Size::new(unit_count, unit_count), units);

    for units_ in units.iter().flatten() {
        for unit in units_ as &Vec<Unit> {
            index.apply(
                &vec![Effect::State(StateEffect::Unit(
                    *unit.id(),
                    UnitEffect::New(unit.clone()),
                ))],
                cities,
                &units,
            );
        }
    }
}

fn inject_cities(index: &mut Index, city_count: usize, units: &Vec2d<Vec<Unit>>) {
    let mut cities = vec![];

    for i in 0..city_count {
        let city = build_city(i);
        cities.push(city);
    }

    let cities: Vec2d<City> = Vec2d::from(D2Size::new(city_count, city_count), cities);

    for city in cities.iter().flatten() {
        index.apply(
            &vec![Effect::State(StateEffect::City(
                *city.id(),
                CityEffect::New(city.clone()),
            ))],
            &cities,
            units,
        );
    }
}

fn index_write_unit(unit_count: usize, cities: &Vec2d<City>) {
    let mut index = Index::default();
    inject_units(&mut index, unit_count, cities);
}

fn index_write_city(city_count: usize, units: &Vec2d<Vec<Unit>>) {
    let mut index = Index::default();
    inject_cities(&mut index, city_count, units);
}

// FIXME BS NOW: unit/city builds are done during test instead of before
pub fn bench_index_write_unit(c: &mut Criterion) {
    let mut group = c.benchmark_group("index_write");
    group.sample_size(10);

    group.bench_function("index_write_unit 1🚹", |b| {
        b.iter(|| {
            index_write_unit(
                black_box(1),
                &Vec2d::from(D2Size::new(1, 1), Vec::<City>::new()),
            )
        })
    });

    group.bench_function("index_write_unit 10k🚹", |b| {
        b.iter(|| {
            index_write_unit(
                black_box(10_000),
                &Vec2d::from(D2Size::new(10_000, 10_000), Vec::<City>::new()),
            )
        })
    });

    group.bench_function("index_write_city 1🏠", |b| {
        b.iter(|| {
            index_write_city(
                black_box(1),
                &Vec2d::from(D2Size::new(1, 1), Vec::<GeoVec<Unit>>::new()),
            )
        })
    });
    group.bench_function("index_write_city 1k🏠", |b| {
        b.iter(|| {
            index_write_city(
                black_box(1_000),
                &Vec2d::from(D2Size::new(1_000, 1_000), Vec::<GeoVec<Unit>>::new()),
            )
        })
    });
}

criterion_group!(benches, bench_index_write_unit);
criterion_main!(benches);
