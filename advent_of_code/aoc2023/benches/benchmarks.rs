use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc2023::solver::{day01, day02};

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

criterion_group!(benches, benchmark_day01, benchmark_day02);
criterion_main!(benches);
