use common::{
    geo::ImaginaryWorldPoint,
    space::window::{DisplayStep, Window},
    utils::slice,
    world::{TerrainType, Tile},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn bench_tiles_from_window(c: &mut Criterion) {
    let world_tiles = vec![Tile::new(TerrainType::GrassLand); 1_000_000];
    let world_width = 1000;
    let world_height = 1000;
    let window_start = ImaginaryWorldPoint::new(450, 450);
    let window_end = ImaginaryWorldPoint::new(500, 500);
    let window = Window::new(window_start, window_end, DisplayStep::Close);

    c.bench_function("tiles_from_window", |b| {
        b.iter(|| {
            slice(
                black_box(&world_tiles),
                black_box(&window),
                black_box(world_width),
                black_box(world_height),
            )
        })
    });
}

criterion_group!(benches, bench_tiles_from_window);
criterion_main!(benches);
