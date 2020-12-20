use criterion::{criterion_group, criterion_main, Criterion};

fn bench_day1(c: &mut Criterion) {
    use aoc::solutions::year2020::day01::{parse, part1, part2};
    let expenses = parse(aoc::input::get_input(2020, 1).unwrap());
    c.bench_function("day1 part 1", |b| b.iter(|| part1(&expenses)));
    c.bench_function("day1 part 2", |b| b.iter(|| part2(&expenses)));
}

fn bench_day2(c: &mut Criterion) {
    use aoc::solutions::year2020::day02::{part1, part2};
    let raw_input = aoc::input::get_input(2020, 2).unwrap();
    c.bench_function("day2 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day2 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_day3(c: &mut Criterion) {
    use aoc::solutions::year2020::day03::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 3).unwrap();
    c.bench_function("day3 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day3 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day3 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day4(c: &mut Criterion) {
    use aoc::solutions::year2020::day04::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 4).unwrap();
    c.bench_function("day4 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day4 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day4 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day5(c: &mut Criterion) {
    use aoc::solutions::year2020::day05::{part1, part2};
    let input = aoc::input::get_input(2020, 5).unwrap();
    c.bench_function("day5 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day5 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day6(c: &mut Criterion) {
    use aoc::solutions::year2020::day06::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 6).unwrap();
    c.bench_function("day6 parse", |b| b.iter(|| parse(&raw_input)));
    c.bench_function("day6 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day6 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_day7(c: &mut Criterion) {
    use aoc::solutions::year2020::day07::{parse, parse_right, part1, part2};
    let raw_input = aoc::input::get_input(2020, 7).unwrap();
    c.bench_function("day7 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day7 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day7 parse_right", |b| b.iter(|| parse_right(&raw_input)));
    let input_right = parse_right(&raw_input);
    c.bench_function("day7 part 2", |b| b.iter(|| part2(&input_right)));
}

fn bench_day7_graphlib(c: &mut Criterion) {
    use aoc::solutions::year2020::day07_graphlib::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 7).unwrap();
    c.bench_function("day7 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day7 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day7 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day8(c: &mut Criterion) {
    use aoc::solutions::year2020::day08::{parse, part1, part2};

    let raw_input = aoc::input::get_input(2020, 8).unwrap();
    c.bench_function("day8 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day8 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day8 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day9(c: &mut Criterion) {
    use aoc::solutions::year2020::day09::{parse, part1, part2};

    let raw_input = aoc::input::get_input(2020, 9).unwrap();
    c.bench_function("day9 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day9 part 1", |b| b.iter(|| part1(&input, 25)));
    c.bench_function("day9 part 2", |b| b.iter(|| part2(&input, 3199139634)));
}

fn bench_day10(c: &mut Criterion) {
    use aoc::solutions::year2020::day10::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 10).unwrap();
    c.bench_function("day10 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day10 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day10 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day11(c: &mut Criterion) {
    use aoc::solutions::year2020::day11::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 11).unwrap();
    c.bench_function("day11 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day11 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day11 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day12(c: &mut Criterion) {
    use aoc::solutions::year2020::day12::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 12).unwrap();
    c.bench_function("day12 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day12 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day12 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day13(c: &mut Criterion) {
    use aoc::solutions::year2020::day13::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 13).unwrap();
    c.bench_function("day13 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    // c.bench_function("day13 part 1", |b| b.iter(|| part1(&input)));
    // c.bench_function("day13 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day14(c: &mut Criterion) {
    use aoc::solutions::year2020::day14::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 14).unwrap();
    c.bench_function("day14 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day14 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day14 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day14b(c: &mut Criterion) {
    use aoc::solutions::year2020::day14b::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 14).unwrap();
    c.bench_function("day14b parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day14b part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day14b part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day15(c: &mut Criterion) {
    use aoc::solutions::year2020::day15::{parse, part1, part2};
    let raw_input = "0,5,4,1,10,14,7".to_owned();
    c.bench_function("day15 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day15 part 1", |b| b.iter(|| part1(&input)));
    c.bench_function("day15 part 2", |b| b.iter(|| part2(&input)));
}

fn bench_day16(c: &mut Criterion) {
    use aoc::solutions::year2020::day16::{parse, part1, part2};
    let raw_input = aoc::input::get_input(2020, 16).unwrap();
    c.bench_function("day16 parse", |b| b.iter(|| parse(&raw_input)));
    let input = parse(&raw_input);
    c.bench_function("day16 part 1", |b| {
        b.iter(|| part1(&input.0, &input.1, &input.2))
    });
    c.bench_function("day16 part 2", |b| {
        b.iter(|| part2(&input.0, &input.1, &input.2))
    });
}

fn bench_day17(c: &mut Criterion) {
    use aoc::solutions::year2020::day17::{part1, part2};

    let raw_input = aoc::input::get_input(2020, 17).unwrap();
    c.bench_function("day17 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day17 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_day17b(c: &mut Criterion) {
    use aoc::solutions::year2020::day17b::{part1 as part1b, part2 as part2b};
    let raw_input = aoc::input::get_input(2020, 17).unwrap();
    c.bench_function("day17b part 1", |b| b.iter(|| part1b(&raw_input)));
    c.bench_function("day17b part 2", |b| b.iter(|| part2b(&raw_input)));
}

fn bench_day18(c: &mut Criterion) {
    use aoc::solutions::year2020::day18::{part1, part2};
    let raw_input = aoc::input::get_input(2020, 18).unwrap();
    c.bench_function("day18 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day18 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_day19(c: &mut Criterion) {
    use aoc::solutions::year2020::day19::{part1, part2};
    let raw_input = aoc::input::get_input(2020, 19).unwrap();
    c.bench_function("day19 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day19 part 2", |b| b.iter(|| part2(&raw_input)));
}

fn bench_day20(c: &mut Criterion) {
    use aoc::solutions::year2020::day20::{part1, part2};
    let raw_input = aoc::input::get_input(2020, 20).unwrap();
    c.bench_function("day20 part 1", |b| b.iter(|| part1(&raw_input)));
    c.bench_function("day20 part 2", |b| b.iter(|| part2(&raw_input)));
}

// criterion_group! {
//     name = benches;
//     config = Criterion::default().sample_size(10);
//     targets = bench_day15
// }

criterion_group!(
    benches,
    bench_day1,
    bench_day2,
    bench_day3,
    bench_day4,
    bench_day5,
    bench_day6,
    bench_day7,
    bench_day7_graphlib,
    bench_day8,
    bench_day9,
    bench_day10,
    bench_day11,
    bench_day12,
    bench_day13,
    bench_day14,
    bench_day14b,
    bench_day15,
    bench_day16,
    bench_day17,
    bench_day17b,
    bench_day18,
    bench_day19,
    bench_day20
);

criterion_main!(benches);
