use criterion::{criterion_group, criterion_main, Criterion};

fn bench22_day05(c: &mut Criterion) {
    use aoc::solutions::year2022::day05::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 5).unwrap();
    c.bench_function("year 22 day05 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day05 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench22_day05b(c: &mut Criterion) {
    use aoc::solutions::year2022::day05b::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 5).unwrap();
    c.bench_function("year 22 day05b part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day05b part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench22_day06(c: &mut Criterion) {
    use aoc::solutions::year2022::day06::{part1, part2, part2b, part2c};
    let raw_input = aoc::input::get_input(2022, 6).unwrap();
    c.bench_function("year 22 day06 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day06 part 2", |b| b.iter(|| part2(&raw_input)));
    c.bench_function("year 22 day06 part 2b", |b| b.iter(|| part2b(&raw_input)));
    c.bench_function("year 22 day06 part 2c", |b| b.iter(|| part2c(&raw_input)));
}

fn bench22_day07(c: &mut Criterion) {
    use aoc::solutions::year2022::day07::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2022, 7).unwrap();
    c.bench_function("year 22 day07 parse", |b| b.iter(|| parse(&raw_input)));
    c.bench_function("year 22 day07 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day07 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench22_day08(c: &mut Criterion) {
    use aoc::solutions::year2022::day08::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 8).unwrap();
    c.bench_function("year 22 day08 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day08 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench22_day09(c: &mut Criterion) {
    use aoc::solutions::year2022::day09::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 9).unwrap();
    c.bench_function("year 22 day09 part 1", |b| {
        b.iter(|| part1(raw_input.as_bytes()))
    });
    c.bench_function("year 22 day09 part 2", |b| {
        b.iter(|| part2(raw_input.as_bytes()))
    });
}

fn bench22_day10(c: &mut Criterion) {
    use aoc::solutions::year2022::day10::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 10).unwrap();
    c.bench_function("year 22 day10 part 1", |b| {
        b.iter(|| part1(raw_input.as_bytes()))
    });
    c.bench_function("year 22 day10 part 2", |b| {
        b.iter(|| part2(raw_input.as_bytes()))
    });
}

fn bench22_day11(c: &mut Criterion) {
    use aoc::solutions::year2022::day11::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2022, 11).unwrap();
    c.bench_function("year 22 day11 parse", |b| {
        b.iter(|| parse(raw_input.as_bytes()))
    });
    c.bench_function("year 22 day11 part 1", |b| {
        b.iter(|| part1(raw_input.as_bytes()))
    });
    c.bench_function("year 22 day11 part 2", |b| {
        b.iter(|| part2(raw_input.as_bytes()))
    });
}

fn bench22_day12(c: &mut Criterion) {
    use aoc::solutions::year2022::day12::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 12).unwrap();
    c.bench_function("year 22 day12 part 1", |b| {
        b.iter(|| part1(raw_input.as_bytes()))
    });
    c.bench_function("year 22 day12 part 2", |b| {
        b.iter(|| part2(raw_input.as_bytes()))
    });
}

fn bench22_day13(c: &mut Criterion) {
    use aoc::solutions::year2022::day13::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 13).unwrap();
    c.bench_function("year 22 day13 part 1", |b| {
        b.iter(|| part1(raw_input.as_bytes()))
    });
    c.bench_function("year 22 day13 part 2", |b| {
        b.iter(|| part2(raw_input.as_bytes()))
    });
}

fn bench22_day14(c: &mut Criterion) {
    use aoc::solutions::year2022::day14::{part1, part2};
    let raw_input = aoc::input::get_input(2022, 14).unwrap();
    c.bench_function("year 22 day14 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("year 22 day14 part 2", |b| b.iter(|| part2(&raw_input)));
}

criterion_group!(
    benches22,
    bench22_day05,
    bench22_day05b,
    bench22_day06,
    bench22_day07,
    bench22_day08,
    bench22_day09,
    bench22_day10,
    bench22_day11,
    bench22_day12,
    bench22_day13,
    bench22_day14,
);

criterion_main!(benches22);
