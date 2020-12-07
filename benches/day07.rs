use criterion::{criterion_group, criterion_main, Criterion};

use aoc::solutions::year2020::day07::{parse, parse_right, part1, part2};

fn bench_day7(c: &mut Criterion) {
    let raw_input = aoc::input::get_input(2020, 7).unwrap();
    c.bench_function("day7 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day7 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day7 parse_right", |b| b.iter(|| parse_right(&raw_input)));
    let input_right = parse_right(&raw_input);
    c.bench_function("day7 part 2", |b| b.iter(|| part2(&input_right)));
}

criterion_group!(benches, bench_day7);
criterion_main!(benches);
