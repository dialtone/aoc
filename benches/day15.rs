use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day15::{parse, part1, part2};

fn bench_day15(c: &mut Criterion) {
    let raw_input = "0,5,4,1,10,14,7".to_owned();
    c.bench_function("day15 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day15 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day15 part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_day15);
criterion_main!(benches);
