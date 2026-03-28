//! # Sort Algorithm Comparison Benchmarks
//!
//! Compares three sorting algorithms across increasing input sizes to demonstrate
//! how complexity class determines real-world performance:
//!
//! | Algorithm     | Complexity     | Notes                                    |
//! |---------------|----------------|------------------------------------------|
//! | `std::sort`   | O(n log n)     | TimSort — highly optimized, adaptive     |
//! | Circle Sort   | O(n log n)*    | Forum-invented, in-place, recursive      |
//! | Bubble Sort   | O(n²)          | Classic quadratic baseline                |
//!
//! *Circle Sort worst case is O(n log² n), but average is O(n log n).
//!
//! ## Benchmark Results (2026-03-28)
//!
//! | n       | std::sort   | Circle Sort | Bubble Sort | Bubble/Circle |
//! |--------:|------------:|------------:|------------:|--------------:|
//! |     100 |      453 ns |      2.7 µs |      2.6 µs |          1.0× |
//! |     500 |      2.7 µs |     22.9 µs |     61.6 µs |          2.7× |
//! |   1,000 |      6.3 µs |     52.6 µs |      286 µs |          5.4× |
//! |   5,000 |     37.6 µs |      680 µs |     6.67 ms |          9.8× |
//! |  10,000 |     80.6 µs |     1.52 ms |     26.1 ms |         17.2× |
//!
//! Key takeaway: At n=100 bubble and circle are tied. By n=10,000 the O(n²)
//! vs O(n log n) divergence is 17×. std::sort (TimSort) dominates both due
//! to decades of optimization, but scales the same class as circle sort.
//!
//! Run: `cargo bench -p circle_sort`

use circle_sort::{bubble_sort, circle_sort};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

/// Generate a deterministic shuffled array of given size.
///
/// Uses a simple LCG (linear congruential generator) to produce
/// reproducible "random" permutations without pulling in rand.
fn shuffled_vec(n: usize) -> Vec<i64> {
    let mut v: Vec<i64> = (0..n as i64).collect();
    // Fisher-Yates shuffle with LCG
    let mut seed: u64 = 42;
    for i in (1..n).rev() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (seed >> 33) as usize % (i + 1);
        v.swap(i, j);
    }
    v
}

fn bench_sorting(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort_comparison");

    // Sizes chosen to show the O(n²) vs O(n log n) divergence clearly.
    // Bubble sort gets painful fast, so we cap at 10_000.
    let sizes = [100, 500, 1_000, 5_000, 10_000];

    for &size in &sizes {
        let input = shuffled_vec(size);

        group.bench_with_input(
            BenchmarkId::new("std_sort", size),
            &input,
            |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |mut v| {
                        v.sort();
                        v
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("circle_sort", size),
            &input,
            |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |mut v| {
                        circle_sort(&mut v);
                        v
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bubble_sort", size),
            &input,
            |b, data| {
                b.iter_batched(
                    || data.clone(),
                    |mut v| {
                        bubble_sort(&mut v);
                        v
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_sorting);
criterion_main!(benches);
