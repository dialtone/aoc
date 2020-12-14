use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day14b::{parse, part1, part2};

fn bench_day14b(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 14).unwrap();
    c.bench_function("day14b parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day14b part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day14b part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_day14b);
criterion_main!(benches);

