use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day17::{part1, part2};

fn bench_day17(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 17).unwrap();
    // c.bench_function("day17 parse", |b| b.iter(|| parse(&raw_input)));
    // let input = parse(&raw_input);
    c.bench_function("day17 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day17 part 2", |b| b.iter(|| part2(&raw_input)));
}

criterion_group!(benches, bench_day17);
criterion_main!(benches);
