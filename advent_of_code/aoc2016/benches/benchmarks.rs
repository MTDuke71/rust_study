use criterion::{criterion_group, criterion_main, Criterion};
use aoc2016::solver::{day01, day02, day03, day04, day05};

fn bench_day01(c: &mut Criterion) {
    let input = include_str!("../inputs/day01.txt");
    c.bench_function("day01_combined", |b| b.iter(|| day01::solve(input)));
    c.bench_function("day01_part1",   |b| b.iter(|| day01::solve_part1(input)));
    c.bench_function("day01_part2",   |b| b.iter(|| day01::solve_part2(input)));
}

fn bench_day02(c: &mut Criterion) {
    let input = include_str!("../inputs/day02.txt");
    c.bench_function("day02_combined", |b| b.iter(|| day02::solve(input)));
    c.bench_function("day02_part1",   |b| b.iter(|| day02::solve_part1(input)));
    c.bench_function("day02_part2",   |b| b.iter(|| day02::solve_part2(input)));
}

fn bench_day03(c: &mut Criterion) {
    let input = include_str!("../inputs/day03.txt");
    c.bench_function("day03_combined",   |b| b.iter(|| day03::solve(input)));
    c.bench_function("day03_part1",      |b| b.iter(|| day03::solve_part1(input)));
    c.bench_function("day03_part2",      |b| b.iter(|| day03::solve_part2(input)));
    c.bench_function("day03_part1_sort", |b| b.iter(|| day03::solve_part1_sort(input)));
}

fn bench_day04(c: &mut Criterion) {
    let input = include_str!("../inputs/day04.txt");
    c.bench_function("day04_combined", |b| b.iter(|| day04::solve(input)));
    c.bench_function("day04_part1",   |b| b.iter(|| day04::solve_part1(input)));
    c.bench_function("day04_part2",   |b| b.iter(|| day04::solve_part2(input)));
}

fn bench_day05(c: &mut Criterion) {
    let input = include_str!("../inputs/day05.txt");
    let mut group = c.benchmark_group("day05");
    group.sample_size(10);
    group.bench_function("combined", |b| b.iter(|| day05::solve(input)));
    group.bench_function("part1",   |b| b.iter(|| day05::solve_part1(input)));
    group.bench_function("part2",   |b| b.iter(|| day05::solve_part2(input)));
    group.finish();
}

criterion_group!(benches, bench_day01, bench_day02, bench_day03, bench_day04, bench_day05);
criterion_main!(benches);
