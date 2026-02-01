use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use mission11::{string_dp, MemoCache};
use std::collections::HashMap;

// ============================================================================
// Fibonacci: Classic Memoization Example
// ============================================================================

fn fibonacci_naive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_naive(n - 1) + fibonacci_naive(n - 2),
    }
}

fn fibonacci_memo(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n <= 1 {
        return n;
    }
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = fibonacci_memo(n - 1, cache) + fibonacci_memo(n - 2, cache);
    cache.insert(n, result);
    result
}

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    for n in [10, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::new("naive", n), n, |b, &n| {
            b.iter(|| fibonacci_naive(black_box(n)))
        });

        group.bench_with_input(BenchmarkId::new("memoized", n), n, |b, &n| {
            b.iter(|| {
                let mut cache = HashMap::new();
                fibonacci_memo(black_box(n), &mut cache)
            })
        });
    }

    group.finish();
}

// ============================================================================
// String Pattern Matching: AoC 2024 Day 19 Style
// ============================================================================

fn bench_string_pattern_matching(c: &mut Criterion) {
    let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

    let test_cases = vec![
        ("brwrr", "short_5"),
        ("bggr", "short_4"),
        ("brwrrbrwrr", "medium_10"),    // Doubled
        ("brwrrbrwrrbrwrr", "long_15"), // Tripled
    ];

    let mut group = c.benchmark_group("string_pattern_matching");

    for (design, label) in test_cases.iter() {
        group.bench_with_input(
            BenchmarkId::new("can_construct", label),
            design,
            |b, &design| {
                b.iter(|| {
                    let mut memo = HashMap::new();
                    string_dp::can_construct(black_box(design), black_box(&patterns), &mut memo)
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("count_constructions", label),
            design,
            |b, &design| {
                b.iter(|| {
                    let mut memo = HashMap::new();
                    string_dp::count_constructions(
                        black_box(design),
                        black_box(&patterns),
                        &mut memo,
                    )
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Cache Performance: Hit Ratio and Overhead
// ============================================================================

fn bench_cache_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_operations");

    // Benchmark cache insertion via memoize
    group.bench_function("insert_1000_entries", |b| {
        b.iter(|| {
            let mut cache: MemoCache<u64, u64> = MemoCache::new();
            for i in 0..1000 {
                cache.memoize(black_box(i), || black_box(i * 2));
            }
        })
    });

    // Benchmark cache lookup (hits)
    group.bench_function("lookup_1000_hits", |b| {
        let mut cache: MemoCache<u64, u64> = MemoCache::new();
        for i in 0..1000 {
            cache.memoize(i, || i * 2);
        }

        b.iter(|| {
            for i in 0..1000 {
                cache.memoize(black_box(i), || black_box(i * 2));
            }
        })
    });

    // Benchmark cache lookup (misses)
    group.bench_function("lookup_1000_misses", |b| {
        b.iter(|| {
            let mut cache: MemoCache<u64, u64> = MemoCache::new();
            for i in 0..1000 {
                cache.memoize(black_box(i), || black_box(i * 2));
            }
        })
    });

    group.finish();
}

// ============================================================================
// State Space Complexity: Counting Paths in Grid
// ============================================================================

fn count_paths_naive(row: usize, col: usize, rows: usize, cols: usize) -> u64 {
    if row == rows - 1 && col == cols - 1 {
        return 1;
    }
    if row >= rows || col >= cols {
        return 0;
    }

    let down = count_paths_naive(row + 1, col, rows, cols);
    let right = count_paths_naive(row, col + 1, rows, cols);
    down + right
}

fn count_paths_memo(
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if row == rows - 1 && col == cols - 1 {
        return 1;
    }
    if row >= rows || col >= cols {
        return 0;
    }

    if let Some(&result) = memo.get(&(row, col)) {
        return result;
    }

    let down = count_paths_memo(row + 1, col, rows, cols, memo);
    let right = count_paths_memo(row, col + 1, rows, cols, memo);
    let result = down + right;

    memo.insert((row, col), result);
    result
}

fn bench_grid_path_counting(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_path_counting");

    for size in [5, 8, 10].iter() {
        group.bench_with_input(BenchmarkId::new("naive", size), size, |b, &size| {
            b.iter(|| count_paths_naive(black_box(0), black_box(0), size, size))
        });

        group.bench_with_input(BenchmarkId::new("memoized", size), size, |b, &size| {
            b.iter(|| {
                let mut memo = HashMap::new();
                count_paths_memo(black_box(0), black_box(0), size, size, &mut memo)
            })
        });
    }

    group.finish();
}

// ============================================================================
// Zero-Copy vs Allocation: String Slice Keys
// ============================================================================

fn count_with_owned_strings(design: &str, patterns: &[&str]) -> u64 {
    fn helper(remaining: &str, patterns: &[&str], memo: &mut HashMap<String, u64>) -> u64 {
        if remaining.is_empty() {
            return 1;
        }

        if let Some(&count) = memo.get(remaining) {
            return count;
        }

        let mut total = 0;
        for pattern in patterns {
            if let Some(suffix) = remaining.strip_prefix(pattern) {
                total += helper(suffix, patterns, memo);
            }
        }

        memo.insert(remaining.to_string(), total); // ← Allocation!
        total
    }

    let mut memo = HashMap::new();
    helper(design, patterns, &mut memo)
}

fn count_with_borrowed_slices<'a>(design: &'a str, patterns: &[&str]) -> u64 {
    fn helper<'a>(remaining: &'a str, patterns: &[&str], memo: &mut HashMap<&'a str, u64>) -> u64 {
        if remaining.is_empty() {
            return 1;
        }

        if let Some(&count) = memo.get(remaining) {
            return count;
        }

        let mut total = 0;
        for pattern in patterns {
            if let Some(suffix) = remaining.strip_prefix(pattern) {
                total += helper(suffix, patterns, memo);
            }
        }

        memo.insert(remaining, total); // ← Zero-copy!
        total
    }

    let mut memo = HashMap::new();
    helper(design, patterns, &mut memo)
}

fn bench_zero_copy_vs_allocation(c: &mut Criterion) {
    let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
    let design = "brwrrbrwrrbrwrrbrwrr"; // Longer design

    let mut group = c.benchmark_group("zero_copy_vs_allocation");

    group.bench_function("owned_string_keys", |b| {
        b.iter(|| count_with_owned_strings(black_box(design), black_box(&patterns)))
    });

    group.bench_function("borrowed_slice_keys", |b| {
        b.iter(|| count_with_borrowed_slices(black_box(design), black_box(&patterns)))
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    bench_fibonacci,
    bench_string_pattern_matching,
    bench_cache_operations,
    bench_grid_path_counting,
    bench_zero_copy_vs_allocation,
);

criterion_main!(benches);
