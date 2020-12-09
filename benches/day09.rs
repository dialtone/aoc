use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day09::{parse, part1, part2};

fn bench_day9(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 9).unwrap();
    c.bench_function("day9 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day9 part 1", |b| b.iter(|| part1(&input, 25)));
    c.bench_function("day9 part 2", |b| b.iter(|| part2(&input, 3199139634)));
}

criterion_group!(benches, bench_day9);
criterion_main!(benches);
