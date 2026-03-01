use criterion::{criterion_group, criterion_main, Criterion};
use aoc2016::solver::day01;

fn bench_day01(c: &mut Criterion) {
    let input = include_str!("../inputs/day01.txt");
    c.bench_function("day01_combined", |b| b.iter(|| day01::solve(input)));
    c.bench_function("day01_part1",   |b| b.iter(|| day01::solve_part1(input)));
    c.bench_function("day01_part2",   |b| b.iter(|| day01::solve_part2(input)));
}

criterion_group!(benches, bench_day01);
criterion_main!(benches);
