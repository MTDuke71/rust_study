use criterion::{criterion_group, criterion_main, Criterion};
use aoc2016::solver::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13};

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

fn bench_day06(c: &mut Criterion) {
    let input = include_str!("../inputs/day06.txt");
    c.bench_function("day06_combined", |b| b.iter(|| day06::solve(input)));
    c.bench_function("day06_part1",   |b| b.iter(|| day06::solve_part1(input)));
    c.bench_function("day06_part2",   |b| b.iter(|| day06::solve_part2(input)));
}

fn bench_day07(c: &mut Criterion) {
    let input = include_str!("../inputs/day07.txt");
    c.bench_function("day07_combined", |b| b.iter(|| day07::solve(input)));
    c.bench_function("day07_part1",   |b| b.iter(|| day07::solve_part1(input)));
    c.bench_function("day07_part2",   |b| b.iter(|| day07::solve_part2(input)));
}

fn bench_day08(c: &mut Criterion) {
    let input = include_str!("../inputs/day08.txt");
    c.bench_function("day08_combined", |b| b.iter(|| day08::solve(input)));
    c.bench_function("day08_part1",   |b| b.iter(|| day08::solve_part1(input)));
    c.bench_function("day08_part2",   |b| b.iter(|| day08::solve_part2(input)));
}

fn bench_day09(c: &mut Criterion) {
    let input = include_str!("../inputs/day09.txt");
    c.bench_function("day09_combined", |b| b.iter(|| day09::solve(input)));
    c.bench_function("day09_part1",   |b| b.iter(|| day09::solve_part1(input)));
    c.bench_function("day09_part2",   |b| b.iter(|| day09::solve_part2(input)));
}

fn bench_day10(c: &mut Criterion) {
    let input = include_str!("../inputs/day10.txt");
    c.bench_function("day10_combined", |b| b.iter(|| day10::solve(input)));
    c.bench_function("day10_part1",   |b| b.iter(|| day10::solve_part1(input)));
    c.bench_function("day10_part2",   |b| b.iter(|| day10::solve_part2(input)));
}

fn bench_day11(c: &mut Criterion) {
    let input = include_str!("../inputs/day11.txt");
    let mut group = c.benchmark_group("day11");
    group.sample_size(20);
    group.bench_function("combined", |b| b.iter(|| day11::solve(input)));
    group.bench_function("part1",   |b| b.iter(|| day11::solve_part1(input)));
    group.bench_function("part2",   |b| b.iter(|| day11::solve_part2(input)));
    group.finish();
}

fn bench_day12(c: &mut Criterion) {
    let input = include_str!("../inputs/day12.txt");
    let mut group = c.benchmark_group("day12");
    group.sample_size(20);
    group.bench_function("combined", |b| b.iter(|| day12::solve(input)));
    group.bench_function("part1",   |b| b.iter(|| day12::solve_part1(input)));
    group.bench_function("part2",   |b| b.iter(|| day12::solve_part2(input)));
    group.finish();
}

fn bench_day13(c: &mut Criterion) {
    let input = include_str!("../inputs/day13.txt");
    c.bench_function("day13_combined", |b| b.iter(|| day13::solve(input)));
    c.bench_function("day13_part1",   |b| b.iter(|| day13::solve_part1(input)));
    c.bench_function("day13_part2",   |b| b.iter(|| day13::solve_part2(input)));
}

criterion_group!(benches, bench_day01, bench_day02, bench_day03, bench_day04, bench_day05, bench_day06, bench_day07, bench_day08, bench_day09, bench_day10, bench_day11, bench_day12, bench_day13);
criterion_main!(benches);
