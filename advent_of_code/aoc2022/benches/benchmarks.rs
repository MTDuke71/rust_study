use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import solvers as you implement them
use aoc2022::solver::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13};

fn benchmark_day01(c: &mut Criterion) {
    let input = include_str!("../inputs/day01.txt");

    c.bench_function("day01_parse", |b| {
        b.iter(|| day01::parse_input(black_box(input)))
    });

    c.bench_function("day01_part1", |b| {
        b.iter(|| day01::solve_part1(black_box(input)))
    });

    c.bench_function("day01_part2", |b| {
        b.iter(|| day01::solve_part2(black_box(input)))
    });

    c.bench_function("day01_combined", |b| {
        b.iter(|| day01::solve(black_box(input)))
    });
}

fn benchmark_day02(c: &mut Criterion) {
    let input = include_str!("../inputs/day02.txt");

    c.bench_function("day02_parse", |b| {
        b.iter(|| day02::parse_input(black_box(input)))
    });

    c.bench_function("day02_combined", |b| {
        b.iter(|| day02::solve(black_box(input)))
    });
}

fn benchmark_day03(c: &mut Criterion) {
    let input = include_str!("../inputs/day03.txt");

    c.bench_function("day03_parse", |b| {
        b.iter(|| day03::parse_input(black_box(input)))
    });

    c.bench_function("day03_combined_hashset", |b| {
        b.iter(|| day03::solve(black_box(input)))
    });

    c.bench_function("day03_combined_bitset", |b| {
        b.iter(|| day03::solve_bitset(black_box(input)))
    });
}

fn benchmark_day04(c: &mut Criterion) {
    let input = include_str!("../inputs/day04.txt");

    c.bench_function("day04_parse", |b| {
        b.iter(|| day04::parse_input(black_box(input)))
    });

    c.bench_function("day04_combined", |b| {
        b.iter(|| day04::solve(black_box(input)))
    });
}

fn benchmark_day05(c: &mut Criterion) {
    let input = include_str!("../inputs/day05.txt");

    c.bench_function("day05_parse", |b| {
        b.iter(|| day05::parse_input(black_box(input)))
    });

    c.bench_function("day05_part1", |b| {
        b.iter(|| day05::solve_part1(black_box(input)))
    });

    c.bench_function("day05_part2", |b| {
        b.iter(|| day05::solve_part2(black_box(input)))
    });

    c.bench_function("day05_combined", |b| {
        b.iter(|| day05::solve(black_box(input)))
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

    c.bench_function("day06_combined", |b| {
        b.iter(|| day06::solve(black_box(input)))
    });
}

fn benchmark_day07(c: &mut Criterion) {
    let input = include_str!("../inputs/day07.txt");

    c.bench_function("day07_combined", |b| {
        b.iter(|| day07::solve(black_box(input)))
    });
}

fn benchmark_day08(c: &mut Criterion) {
    let input = include_str!("../inputs/day08.txt");

    c.bench_function("day08_combined", |b| {
        b.iter(|| day08::solve(black_box(input)))
    });
    
    /*
    // SIMD comparison - commented out (no performance benefit)
    // Benchmark showed: 174µs SIMD vs 173µs scalar (no improvement)
    #[cfg(target_arch = "x86_64")]
    {
        c.bench_function("day08_simd", |b| {
            b.iter(|| day08::solve_simd(black_box(input)))
        });
    }
    */
}

fn benchmark_day09(c: &mut Criterion) {
    let input = include_str!("../inputs/day09.txt");

    c.bench_function("day09_combined", |b| {
        b.iter(|| day09::solve(black_box(input)))
    });

    c.bench_function("day09_part1", |b| {
        b.iter(|| day09::solve_part1(black_box(input)))
    });

    c.bench_function("day09_part2", |b| {
        b.iter(|| day09::solve_part2(black_box(input)))
    });
}

fn benchmark_day10(c: &mut Criterion) {
    let input = include_str!("../inputs/day10.txt");

    c.bench_function("day10_combined", |b| {
        b.iter(|| day10::solve(black_box(input)))
    });

    c.bench_function("day10_part1", |b| {
        b.iter(|| day10::solve_part1(black_box(input)))
    });

    c.bench_function("day10_part2", |b| {
        b.iter(|| day10::solve_part2(black_box(input)))
    });
}

fn benchmark_day11(c: &mut Criterion) {
    let input = include_str!("../inputs/day11.txt");

    c.bench_function("day11_combined", |b| {
        b.iter(|| day11::solve(black_box(input)))
    });

    c.bench_function("day11_part1", |b| {
        b.iter(|| day11::solve_part1(black_box(input)))
    });

    c.bench_function("day11_part2", |b| {
        b.iter(|| day11::solve_part2(black_box(input)))
    });
}

fn benchmark_day12(c: &mut Criterion) {
    let input = include_str!("../inputs/day12.txt");

    c.bench_function("day12_combined_backward", |b| {
        b.iter(|| day12::solve(black_box(input)))
    });

    // Compare all three approaches for Part 2
    let map = day12::parse_input(input);
    
    c.bench_function("day12_part2_sequential", |b| {
        b.iter(|| day12::solve_part2_with_data(black_box(&map)))
    });

    c.bench_function("day12_part2_parallel", |b| {
        b.iter(|| day12::solve_part2_parallel(black_box(&map)))
    });

    c.bench_function("day12_part2_backward", |b| {
        b.iter(|| day12::bfs_backward_to_any_a(black_box(&map)))
    });
}

fn benchmark_day13(c: &mut Criterion) {
    let input = include_str!("../inputs/day13.txt");

    c.bench_function("day13_parse", |b| {
        b.iter(|| day13::parse_input(black_box(input)))
    });

    let data = day13::parse_input(input);

    c.bench_function("day13_part1", |b| {
        b.iter(|| day13::solve_part1_with_data(black_box(&data)))
    });

    c.bench_function("day13_part2", |b| {
        b.iter(|| day13::solve_part2_with_data(black_box(&data)))
    });

    c.bench_function("day13_combined", |b| {
        b.iter(|| day13::solve(black_box(input)))
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
);

criterion_main!(benches);
