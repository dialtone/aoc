use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day11::{parse, part1, part2};

fn bench_day11(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 4).unwrap();
    c.bench_function("day11 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day11 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day11 part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_day11);
criterion_main!(benches);
