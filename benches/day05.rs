use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day05::{part1, part2};

fn bench_day5(c: &mut Criterion) {
    let input = aoc::input::get_input(2020, 5).unwrap();
    c.bench_function("day5 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day5 part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_day5);
criterion_main!(benches);
