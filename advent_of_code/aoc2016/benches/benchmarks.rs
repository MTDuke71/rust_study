use criterion::{criterion_group, criterion_main, Criterion};
use aoc2016::solver::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day20};

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

fn bench_day14(c: &mut Criterion) {
    let input = include_str!("../inputs/day14.txt");
    let mut group = c.benchmark_group("day14");
    group.sample_size(10);
    group.bench_function("combined", |b| b.iter(|| day14::solve(input)));
    group.bench_function("part1",   |b| b.iter(|| day14::solve_part1(input)));
    group.bench_function("part2",   |b| b.iter(|| day14::solve_part2(input)));
    group.finish();
}

fn bench_day15(c: &mut Criterion) {
    let input = include_str!("../inputs/day15.txt");
    c.bench_function("day15_crt_combined",   |b| b.iter(|| day15::solve(input)));
    c.bench_function("day15_crt_part1",      |b| b.iter(|| day15::solve_part1(input)));
    c.bench_function("day15_crt_part2",      |b| b.iter(|| day15::solve_part2(input)));
    c.bench_function("day15_brute_combined", |b| b.iter(|| day15::solve_brute_force(input)));
}

fn bench_day16(c: &mut Criterion) {
    let input = include_str!("../inputs/day16.txt");
    c.bench_function("day16_combined", |b| b.iter(|| day16::solve(input)));
    c.bench_function("day16_part1",    |b| b.iter(|| day16::solve_part1(input)));
    c.bench_function("day16_part2",    |b| b.iter(|| day16::solve_part2(input)));
}

fn bench_day17(c: &mut Criterion) {
    let input = include_str!("../inputs/day17.txt");
    let mut group = c.benchmark_group("day17");
    group.sample_size(20);
    group.bench_function("combined", |b| b.iter(|| day17::solve(input)));
    group.bench_function("part1",   |b| b.iter(|| day17::solve_part1(input)));
    group.bench_function("part2",   |b| b.iter(|| day17::solve_part2(input)));
    group.finish();
}

fn bench_day18(c: &mut Criterion) {
    let input = include_str!("../inputs/day18.txt");
    let mut group = c.benchmark_group("day18");
    group.bench_function("combined_vector", |b| b.iter(|| day18::solve(input)));
    group.bench_function("part1",           |b| b.iter(|| day18::solve_part1(input)));
    group.bench_function("part2_vector",    |b| b.iter(|| day18::solve_part2(input)));
    group.bench_function("part2_bitset",    |b| b.iter(|| day18::solve_part2_bitset(input)));
    group.finish();
}

fn bench_day20(c: &mut Criterion) {
    let input = include_str!("../inputs/day20.txt");
    c.bench_function("day20_combined", |b| b.iter(|| day20::solve(input)));
    c.bench_function("day20_part1",   |b| b.iter(|| day20::solve_part1(input)));
    c.bench_function("day20_part2",   |b| b.iter(|| day20::solve_part2(input)));
}

criterion_group!(benches, bench_day01, bench_day02, bench_day03, bench_day04, bench_day05, bench_day06, bench_day07, bench_day08, bench_day09, bench_day10, bench_day11, bench_day12, bench_day13, bench_day14, bench_day15, bench_day16, bench_day17, bench_day18, bench_day20);
criterion_main!(benches);
