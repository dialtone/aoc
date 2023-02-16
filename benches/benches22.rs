use criterion::{criterion_group, criterion_main, Criterion};

fn bench_22day05(c: &mut Criterion) {
    use aoc::solutions::year2022::day05::part1;
    let raw_input = aoc::input::get_input(2022, 5).unwrap();
    c.bench_function("year 22 day05 part 1", |b| b.iter(|| part1(&raw_input)));
}
criterion_group!(benches22, bench_22day05);

criterion_main!(benches22);
