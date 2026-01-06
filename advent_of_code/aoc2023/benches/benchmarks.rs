use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc2023::solver::{day01, day02, day03, day04, day05, day06};

fn benchmark_day01(c: &mut Criterion) {
    let input = include_str!("../inputs/day01.txt");
    
    c.bench_function("day01_part1", |b| {
        b.iter(|| day01::solve_part1(black_box(input)))
    });
    
    c.bench_function("day01_part2", |b| {
        b.iter(|| day01::solve_part2(black_box(input)))
    });
}

fn benchmark_day02(c: &mut Criterion) {
    let input = include_str!("../inputs/day02.txt");
    
    c.bench_function("day02_part1", |b| {
        b.iter(|| day02::solve_part1(black_box(input)))
    });
    
    c.bench_function("day02_part2", |b| {
        b.iter(|| day02::solve_part2(black_box(input)))
    });
}

fn benchmark_day03(c: &mut Criterion) {
    let input = include_str!("../inputs/day03.txt");
    
    c.bench_function("day03_part1", |b| {
        b.iter(|| day03::solve_part1(black_box(input)))
    });
    
    c.bench_function("day03_part2", |b| {
        b.iter(|| day03::solve_part2(black_box(input)))
    });
}

fn benchmark_day04(c: &mut Criterion) {
    let input = include_str!("../inputs/day04.txt");
    
    c.bench_function("day04_part1", |b| {
        b.iter(|| day04::solve_part1(black_box(input)))
    });
    
    c.bench_function("day04_part2", |b| {
        b.iter(|| day04::solve_part2(black_box(input)))
    });
}

fn benchmark_day05(c: &mut Criterion) {
    let input = include_str!("../inputs/day05.txt");
    
    c.bench_function("day05_part1", |b| {
        b.iter(|| day05::solve_part1(black_box(input)))
    });
    
    c.bench_function("day05_part2", |b| {
        b.iter(|| day05::solve_part2(black_box(input)))
    });
}

fn benchmark_day06(c: &mut Criterion) {
    let input = include_str!("../inputs/day06.txt");
    
    c.bench_function("day06_part1", |b| {
        b.iter(|| day06::solve_part1(black_box(input)))
    });
    
    c.bench_function("day06_part2", |b| {
        b.iter(|| day06::solve_part2(black_box(input)))
    });
}

criterion_group!(benches, benchmark_day01, benchmark_day02, benchmark_day03, benchmark_day04, benchmark_day05, benchmark_day06);
criterion_main!(benches);
