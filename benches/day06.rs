use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day06::{parse, part1, part2};

fn bench_day6(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 6).unwrap();
    c.bench_function("day6 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day6 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day6 part 2", |b| b.iter(|| part2(&raw_input)));
}

criterion_group!(benches, bench_day6);
criterion_main!(benches);
