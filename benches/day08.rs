use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day08::{parse, part1, part2};

fn bench_day8(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 8).unwrap();
    c.bench_function("day8 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day8 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day8 part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_day8);
criterion_main!(benches);
