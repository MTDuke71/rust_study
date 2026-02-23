use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import solvers as you implement them
use aoc2022::solver::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day23};

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

fn benchmark_day14(c: &mut Criterion) {
    let input = include_str!("../inputs/day14.txt");

    c.bench_function("day14_parse", |b| {
        b.iter(|| day14::parse(black_box(input)))
    });

    let data = day14::parse(input);

    c.bench_function("day14_part1", |b| {
        b.iter(|| day14::part1(black_box(&data)))
    });

    c.bench_function("day14_part2", |b| {
        b.iter(|| day14::part2(black_box(&data)))
    });
}

fn benchmark_day15(c: &mut Criterion) {
    let input = include_str!("../inputs/day15.txt");

    c.bench_function("day15_parse", |b| {
        b.iter(|| day15::parse(black_box(input)))
    });

    let sensors = day15::parse(input);

    c.bench_function("day15_part1", |b| {
        b.iter(|| day15::part1(black_box(&sensors)))
    });

    c.bench_function("day15_part2", |b| {
        b.iter(|| day15::part2(black_box(&sensors)))
    });

    c.bench_function("day15_part2_lines", |b| {
        b.iter(|| day15::part2_lines(black_box(&sensors), 4_000_000))
    });

    c.bench_function("day15_part2_perimeter", |b| {
        b.iter(|| day15::part2_perimeter(black_box(&sensors), 4_000_000))
    });

    c.bench_function("day15_part2_row_scan", |b| {
        b.iter(|| day15::part2_row_scan(black_box(&sensors), 4_000_000))
    });

    c.bench_function("day15_combined", |b| {
        b.iter(|| day15::solve(black_box(input)))
    });
}

fn benchmark_day16(c: &mut Criterion) {
    let input = include_str!("../inputs/day16.txt");

    c.bench_function("day16_part1", |b| {
        b.iter(|| day16::solve_part1(black_box(input)))
    });

    c.bench_function("day16_part2", |b| {
        b.iter(|| day16::solve_part2(black_box(input)))
    });

    c.bench_function("day16_combined", |b| {
        b.iter(|| day16::solve(black_box(input)))
    });
}

fn benchmark_day17(c: &mut Criterion) {
    let input = include_str!("../inputs/day17.txt");

    c.bench_function("day17_parse", |b| {
        b.iter(|| day17::parse_input(black_box(input)))
    });

    c.bench_function("day17_part1", |b| {
        b.iter(|| day17::solve_part1(black_box(input)))
    });

    c.bench_function("day17_part2", |b| {
        b.iter(|| day17::solve_part2(black_box(input)))
    });

    c.bench_function("day17_combined", |b| {
        b.iter(|| day17::solve(black_box(input)))
    });
}

fn benchmark_day18(c: &mut Criterion) {
    let input = include_str!("../inputs/day18.txt");

    c.bench_function("day18_parse", |b| {
        b.iter(|| day18::parse_input(black_box(input)))
    });

    let cubes = day18::parse_input(input);

    c.bench_function("day18_part1", |b| {
        b.iter(|| day18::solve_part1_with_data(black_box(&cubes)))
    });

    c.bench_function("day18_part2", |b| {
        b.iter(|| day18::solve_part2_with_data(black_box(&cubes)))
    });

    c.bench_function("day18_combined", |b| {
        b.iter(|| day18::solve(black_box(input)))
    });
}

fn benchmark_day19(c: &mut Criterion) {
    let input = include_str!("../inputs/day19.txt");

    c.bench_function("day19_parse", |b| {
        b.iter(|| day19::parse_input(black_box(input)))
    });

    let data = day19::parse_input(input);

    c.bench_function("day19_part1", |b| {
        b.iter(|| day19::solve_part1_with_data(black_box(&data)))
    });

    c.bench_function("day19_part2", |b| {
        b.iter(|| day19::solve_part2_with_data(black_box(&data)))
    });

    c.bench_function("day19_combined", |b| {
        b.iter(|| day19::solve(black_box(input)))
    });
}

fn benchmark_day20(c: &mut Criterion) {
    let input = include_str!("../inputs/day20.txt");

    c.bench_function("day20_parse", |b| {
        b.iter(|| day20::parse_input(black_box(input)))
    });

    let values = day20::parse_input(input);

    c.bench_function("day20_part1", |b| {
        b.iter(|| day20::solve_part1(black_box(&values)))
    });

    c.bench_function("day20_part2", |b| {
        b.iter(|| day20::solve_part2(black_box(&values)))
    });

    c.bench_function("day20_combined", |b| {
        b.iter(|| day20::solve(black_box(input)))
    });
}

fn benchmark_day23(c: &mut Criterion) {
    let input = include_str!("../inputs/day23.txt");

    c.bench_function("day23_combined", |b| {
        b.iter(|| day23::solve(black_box(input)))
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
    benchmark_day14,
    benchmark_day15,
    benchmark_day16,
    benchmark_day17,
    benchmark_day18,
    benchmark_day19,
    benchmark_day20,
    benchmark_day23,
);

criterion_main!(benches);
