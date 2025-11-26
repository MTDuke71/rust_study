use aoc2015::solver::day10;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use mission5::MemoCache;
use std::hint::black_box;

/// Non-memoized iterative approach (your implementation)
fn look_and_say_iterative(input: &str, iterations: usize) -> String {
    let mut current = input.to_string();
    for _ in 0..iterations {
        current = day10::look_and_say(&current);
    }
    current
}

/// Memoized recursive approach
fn look_and_say_with_memo(input: &str, iterations: usize) -> String {
    let mut cache = MemoCache::new();
    look_and_say_recursive(input.to_string(), iterations, &mut cache)
}

fn look_and_say_recursive(
    sequence: String,
    remaining: usize,
    cache: &mut MemoCache<(String, usize), String>,
) -> String {
    let key = (sequence.clone(), remaining);

    // Check cache first
    if let Some(cached) = cache.get(&key) {
        return cached.clone();
    }

    // Base case
    if remaining == 0 {
        cache.insert(key, sequence.clone());
        return sequence;
    }

    // Transform sequence
    let next_sequence = day10::look_and_say(&sequence);

    // Recursive call with memoization
    let result = look_and_say_recursive(next_sequence, remaining - 1, cache);

    // Cache result before returning
    cache.insert(key, result.clone());
    result
}

/// Benchmark both approaches for Part 2 (50 iterations)
fn benchmark_day10_part2(c: &mut Criterion) {
    let input = "1113122113"; // AoC 2015 Day 10 puzzle input

    let mut group = c.benchmark_group("day10_part2_50_iterations");

    // Benchmark iterative (no memoization)
    group.bench_with_input(
        BenchmarkId::new("iterative", "50_cycles"),
        &input,
        |b, &input| {
            b.iter(|| {
                let result = look_and_say_iterative(black_box(input), 50);
                black_box(result.len())
            });
        },
    );

    // Benchmark memoized (recursive with cache)
    group.bench_with_input(
        BenchmarkId::new("memoized", "50_cycles"),
        &input,
        |b, &input| {
            b.iter(|| {
                let result = look_and_say_with_memo(black_box(input), 50);
                black_box(result.len())
            });
        },
    );

    group.finish();
}

/// Benchmark both approaches at different iteration counts
fn benchmark_day10_scalability(c: &mut Criterion) {
    let input = "1113122113";

    let mut group = c.benchmark_group("day10_scalability");

    for iterations in [10, 20, 30, 40, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("iterative", iterations),
            iterations,
            |b, &iterations| {
                b.iter(|| {
                    let result = look_and_say_iterative(black_box(input), iterations);
                    black_box(result.len())
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("memoized", iterations),
            iterations,
            |b, &iterations| {
                b.iter(|| {
                    let result = look_and_say_with_memo(black_box(input), iterations);
                    black_box(result.len())
                });
            },
        );
    }

    group.finish();
}

/// Memory usage comparison (not a benchmark, just informative)
#[allow(dead_code)]
fn analyze_cache_effectiveness() {
    let input = "1113122113";
    let mut cache = MemoCache::new();

    println!("\n=== Cache Effectiveness Analysis ===");
    println!("Input: {}", input);

    let result = look_and_say_recursive(input.to_string(), 50, &mut cache);

    println!("After 50 iterations:");
    println!("- Final length: {}", result.len());
    println!("- Cache entries: {}", cache.len());

    println!("\nNote: Look-and-say sequences grow exponentially and rarely repeat,");
    println!("so memoization provides minimal benefit despite memory overhead.");
}

criterion_group!(benches, benchmark_day10_part2, benchmark_day10_scalability);
criterion_main!(benches);
