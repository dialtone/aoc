use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day17::{part1, part2};
use aoc::solutions::year2020::day17b::{part1 as part1b, part2 as part2b};

fn bench_day17(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 17).unwrap();
    c.bench_function("day17 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day17 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_day17b(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 17).unwrap();
    c.bench_function("day17b part 1", |b| b.iter(|| part1b(&raw_input)));
    c.bench_function("day17b part 2", |b| b.iter(|| part2b(&raw_input)));
}

criterion_group!(benches, bench_day17, bench_day17b);
criterion_main!(benches);
