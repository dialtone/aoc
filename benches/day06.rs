use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day06::{part1, part2};

fn bench_day6(c: &mut Criterion) {
    let input = aoc::input::get_input(2020, 6).unwrap();
    c.bench_function("day6 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day6 part 2", |b| b.iter(|| part2(&input)));
}

criterion_group!(benches, bench_day6);
criterion_main!(benches);
