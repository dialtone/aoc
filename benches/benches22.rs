use criterion::{criterion_group, criterion_main, Criterion};

fn bench_22day05(c: &mut Criterion) {
    use aoc::solutions::year2022::day05::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 5).unwrap();
    c.bench_function("year 22 day05 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day05 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_22day05b(c: &mut Criterion) {
    use aoc::solutions::year2022::day05b::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 5).unwrap();
    c.bench_function("year 22 day05b part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day05b part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_22day06(c: &mut Criterion) {
    use aoc::solutions::year2022::day06::{part1, part2, part2b, part2c};
    let raw_input = aoc::input::get_input(2022, 6).unwrap();
    c.bench_function("year 22 day06 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day06 part 2", |b| b.iter(|| part2(&raw_input)));
    c.bench_function("year 22 day06 part 2b", |b| b.iter(|| part2b(&raw_input)));
    c.bench_function("year 22 day06 part 2c", |b| b.iter(|| part2c(&raw_input)));
}
criterion_group!(benches22, bench_22day05, bench_22day05b, bench_22day06);

criterion_main!(benches22);
