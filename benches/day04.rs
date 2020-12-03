use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day04::{parse, part1, part2};

fn bench_day4(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 4).unwrap();
    c.bench_function("day4 parse", |b| b.iter(|| parse(&raw_input)));
    c.bench_function("day4 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day4 part 2", |b| b.iter(|| part2(&raw_input)));
}

criterion_group!(benches, bench_day4);
criterion_main!(benches);

