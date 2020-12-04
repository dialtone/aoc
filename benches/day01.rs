use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day01::{parse, part1, part2};

fn bench_day1(c: &mut Criterion) {
    let expenses = parse(aoc::input::get_input(2020, 1).unwrap());
    c.bench_function("day1 part 1", |b| b.iter(|| part1(&expenses)));
    c.bench_function("day1 part 2", |b| b.iter(|| part2(&expenses)));
}

criterion_group!(benches, bench_day1);
criterion_main!(benches);