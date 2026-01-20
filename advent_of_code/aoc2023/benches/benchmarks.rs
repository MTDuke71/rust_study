use aoc2023::solver::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19,
};
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

fn benchmark_day15(c: &mut Criterion) {
    let input = include_str!("../inputs/day15.txt");

    c.bench_function("day15_part1", |b| {
        b.iter(|| day15::solve_part1(black_box(input)).unwrap())
    });

    c.bench_function("day15_part2", |b| {
        b.iter(|| day15::solve_part2(black_box(input)).unwrap())
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
}

fn benchmark_day17(c: &mut Criterion) {
    let input = include_str!("../inputs/day17.txt");

    c.bench_function("day17_part1", |b| {
        b.iter(|| day17::solve_part1(black_box(input)))
    });

    c.bench_function("day17_part2", |b| {
        b.iter(|| day17::solve_part2(black_box(input)))
    });
}

fn benchmark_day18(c: &mut Criterion) {
    let input = include_str!("../inputs/day18.txt");

    c.bench_function("day18_part1", |b| {
        b.iter(|| day18::solve_part1(black_box(input)))
    });

    c.bench_function("day18_part2", |b| {
        b.iter(|| day18::solve_part2(black_box(input)))
    });
}

fn benchmark_day19(c: &mut Criterion) {
    let input = include_str!("../inputs/day19.txt");

    c.bench_function("day19_part1", |b| {
        b.iter(|| day19::solve_part1(black_box(input)))
    });

    c.bench_function("day19_part2", |b| {
        b.iter(|| day19::solve_part2(black_box(input)))
    });

    // Granular benchmarks to separate parsing from calculation
    c.bench_function("day19_part1_parse_only", |b| {
        b.iter(|| {
            use std::collections::HashMap;
            let sections: Vec<&str> = input.split("\n\n").collect();
            let workflows_text = sections[0];
            let parts_text = sections[1];
            
            // Parse workflows
            let mut workflows = HashMap::new();
            for line in workflows_text.lines() {
                let workflow = day19::parse_workflow(black_box(line));
                workflows.insert(workflow.name.clone(), workflow);
            }
            
            // Parse parts
            let parts: Vec<_> = parts_text.lines().map(|l| day19::parse_part(black_box(l))).collect();
            
            black_box((workflows, parts))
        })
    });

    c.bench_function("day19_part2_parse_only", |b| {
        b.iter(|| {
            use std::collections::HashMap;
            let sections: Vec<&str> = input.split("\n\n").collect();
            let workflows_text = sections[0];
            
            // Parse workflows only (Part 2 doesn't need parts)
            let mut workflows = HashMap::new();
            for line in workflows_text.lines() {
                let workflow = day19::parse_workflow(black_box(line));
                workflows.insert(workflow.name.clone(), workflow);
            }
            
            black_box(workflows)
        })
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
    benchmark_day19
);
criterion_main!(benches);
