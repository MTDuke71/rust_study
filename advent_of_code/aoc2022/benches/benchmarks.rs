use criterion::{criterion_group, criterion_main, Criterion};

// Import solvers as you implement them
// use aoc2022::solver::day01;
// use aoc2022::solver::day02;

// Benchmark template - uncomment and adapt as you implement days
/*
fn benchmark_day01(c: &mut Criterion) {
    let input = include_str!("../inputs/day01.txt");

    c.bench_function("day01_part1", |b| {
        b.iter(|| day01::solve_part1(input))
    });

    c.bench_function("day01_part2", |b| {
        b.iter(|| day01::solve_part2(input))
    });
}
*/

criterion_group!(
    benches,
    // benchmark_day01,
    // benchmark_day02,
);

criterion_main!(benches);
