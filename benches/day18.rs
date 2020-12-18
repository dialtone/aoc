use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day18::{part1, part2};

fn bench_day18(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 18).unwrap();
    c.bench_function("day18 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day18 part 2", |b| b.iter(|| part2(&raw_input)));
}

criterion_group!(benches, bench_day18);
criterion_main!(benches);
