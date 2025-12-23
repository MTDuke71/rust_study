use criterion::{criterion_group, criterion_main, Criterion};

// Benchmark template - uncomment as days are implemented
// use aoc2023::solver::day01;

fn benchmark_day01(_c: &mut Criterion) {
    // let input = include_str!("../inputs/day01.txt");
    // c.bench_function("day01_part1", |b| {
    //     b.iter(|| day01::solve_part1(black_box(input)))
    // });
    // c.bench_function("day01_part2", |b| {
    //     b.iter(|| day01::solve_part2(black_box(input)))
    // });
}

// Add more benchmark functions as needed:
// fn benchmark_day02(c: &mut Criterion) { ... }

criterion_group!(benches, benchmark_day01);
criterion_main!(benches);
