use criterion::{criterion_group, criterion_main, Criterion};
use aoc2016::solver::{day01, day02, day03, day04};

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

criterion_group!(benches, bench_day01, bench_day02, bench_day03, bench_day04);
criterion_main!(benches);
