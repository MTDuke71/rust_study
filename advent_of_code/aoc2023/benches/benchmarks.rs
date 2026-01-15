use aoc2023::solver::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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

fn benchmark_day07(c: &mut Criterion) {
    let input = include_str!("../inputs/day07.txt");

    c.bench_function("day07_part1", |b| {
        b.iter(|| day07::solve_part1(black_box(input)))
    });

    c.bench_function("day07_part2", |b| {
        b.iter(|| day07::solve_part2(black_box(input)))
    });
}

fn benchmark_day08(c: &mut Criterion) {
    let input = include_str!("../inputs/day08.txt");

    c.bench_function("day08_part1", |b| {
        b.iter(|| day08::solve_part1(black_box(input)))
    });

    c.bench_function("day08_part2", |b| {
        b.iter(|| day08::solve_part2(black_box(input)))
    });
}

fn benchmark_day09(c: &mut Criterion) {
    let input = include_str!("../inputs/day09.txt");

    c.bench_function("day09_part1", |b| {
        b.iter(|| day09::solve_part1(black_box(input)))
    });

    c.bench_function("day09_part2", |b| {
        b.iter(|| day09::solve_part2(black_box(input)))
    });
}

fn benchmark_day10(c: &mut Criterion) {
    let input = include_str!("../inputs/day10.txt");

    c.bench_function("day10_part1", |b| {
        b.iter(|| day10::solve_part1(black_box(input)))
    });

    c.bench_function("day10_part2", |b| {
        b.iter(|| day10::solve_part2(black_box(input)))
    });

    c.bench_function("day10_both_parts", |b| {
        b.iter(|| day10::solve_both_parts(black_box(input)))
    });
}

fn benchmark_day11(c: &mut Criterion) {
    let input = include_str!("../inputs/day11.txt");

    c.bench_function("day11_part1", |b| {
        b.iter(|| day11::solve_part1(black_box(input)))
    });

    c.bench_function("day11_part2", |b| {
        b.iter(|| day11::solve_part2(black_box(input)))
    });
}

fn benchmark_day12(c: &mut Criterion) {
    let input = include_str!("../inputs/day12.txt");

    c.bench_function("day12_part1", |b| {
        b.iter(|| day12::solve_part1(black_box(input)))
    });

    c.bench_function("day12_part2", |b| {
        b.iter(|| day12::solve_part2(black_box(input)))
    });
}

fn benchmark_day13(c: &mut Criterion) {
    let input = include_str!("../inputs/day13.txt");

    c.bench_function("day13_part1", |b| {
        b.iter(|| day13::solve_part1(black_box(input)).unwrap())
    });

    c.bench_function("day13_part2", |b| {
        b.iter(|| day13::solve_part2(black_box(input)).unwrap())
    });
}

fn benchmark_day14(c: &mut Criterion) {
    let input = include_str!("../inputs/day14.txt");

    c.bench_function("day14_part1_simulation", |b| {
        b.iter(|| day14::solve_part1(black_box(input)).unwrap())
    });

    c.bench_function("day14_part2_simulation", |b| {
        b.iter(|| day14::solve_part2(black_box(input)).unwrap())
    });
    
    c.bench_function("day14_part2_sorted", |b| {
        b.iter(|| day14::solve_part2_sorted(black_box(input)).unwrap())
    });
    
    c.bench_function("day14_part2_reconstruction", |b| {
        b.iter(|| day14::solve_part2_reconstruction(black_box(input)).unwrap())
    });
}

criterion_group!(
    benches,
    benchmark_day01,
    benchmark_day02,
    benchmark_day03,
    benchmark_day04,
    benchmark_day05,
    benchmark_day06,
    benchmark_day07,
    benchmark_day08,
    benchmark_day09,
    benchmark_day10,
    benchmark_day11,
    benchmark_day12,
    benchmark_day13,
    benchmark_day14
);
criterion_main!(benches);
