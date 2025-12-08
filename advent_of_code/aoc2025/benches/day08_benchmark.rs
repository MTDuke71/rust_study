use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;

// We need to expose the internal functions for benchmarking
// In a real scenario, we'd make them pub or use #[path] to include the module

fn benchmark_day08_distance_methods(c: &mut Criterion) {
    // Read the actual puzzle input
    let input = fs::read_to_string("inputs/day08.txt")
        .expect("Failed to read day08.txt");
    
    let mut group = c.benchmark_group("day08_distance_comparison");
    
    // Benchmark original f64 with sqrt
    group.bench_function("part1_with_sqrt_f64", |b| {
        b.iter(|| {
            aoc2025::solver::day08::solve_part1(black_box(&input))
        });
    });
    
    // Benchmark optimized i64 squared distance
    group.bench_function("part1_squared_i64", |b| {
        b.iter(|| {
            aoc2025::solver::day08::solve_part1_optimized(black_box(&input))
        });
    });
    
    // Benchmark Part 2 as well
    group.bench_function("part2_with_sqrt_f64", |b| {
        b.iter(|| {
            aoc2025::solver::day08::solve_part2(black_box(&input))
        });
    });
    
    group.bench_function("part2_squared_i64", |b| {
        b.iter(|| {
            aoc2025::solver::day08::solve_part2_optimized(black_box(&input))
        });
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_day08_distance_methods);
criterion_main!(benches);
