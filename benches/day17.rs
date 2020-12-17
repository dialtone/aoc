use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day17::{parse, parse4, part1, part2};

fn bench_day17(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 17).unwrap();
    c.bench_function("day17 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    let input4 = parse4(&raw_input);
    c.bench_function("day17 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day17 part 2", |b| b.iter(|| part2(&input4)));
}

criterion_group!(benches, bench_day17);
criterion_main!(benches);
