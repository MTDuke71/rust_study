//! # Mission 10: Union-Find Performance Benchmarks
//!
//! Comprehensive benchmark suite for Union-Find operations covering:
//! - Various data sizes (1K, 10K, 100K, 1M elements)
//! - Different tree structures (chains, balanced, random)
//! - All core operations (find, union, connected, size, count)
//! - Edge cases and pathological scenarios
//!
//! Run with: `cargo bench`
//! Generate flamegraph: `cargo bench --bench performance -- --profile-time=5`

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use mission10::UnionFind;

// ============================================================================
// Benchmark Parameters
// ============================================================================

const SIZES: &[usize] = &[1_000, 10_000, 100_000, 1_000_000];
const SMALL_SIZES: &[usize] = &[100, 500, 1_000, 5_000];

// ============================================================================
// Setup Functions for Different Tree Structures
// ============================================================================

/// Creates a worst-case chain structure: 0 -> 1 -> 2 -> ... -> n-1
fn setup_chain_structure(n: usize) -> UnionFind {
    let mut uf = UnionFind::new(n);
    for i in 1..n {
        uf.union(0, i).unwrap();
    }
    uf
}

/// Creates a balanced tree structure with good rank distribution
fn setup_balanced_structure(n: usize) -> UnionFind {
    let mut uf = UnionFind::new(n);

    // Build balanced binary tree structure
    let mut step = 1;
    while step < n {
        for i in (step..n).step_by(step * 2) {
            if i >= step {
                uf.union(i - step, i).unwrap();
            }
        }
        step *= 2;
    }
    uf
}

/// Creates random union structure
fn setup_random_structure(n: usize, seed: u64) -> UnionFind {
    let mut uf = UnionFind::new(n);
    let mut rng_state = seed;

    // Simple LCG for reproducible randomness
    for _ in 0..(n / 2) {
        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let a = (rng_state % n as u64) as usize;

        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
        let b = (rng_state % n as u64) as usize;

        if a != b {
            uf.union(a, b).unwrap();
        }
    }
    uf
}

/// Creates a flat structure where all elements have same root
fn setup_flat_structure(n: usize) -> UnionFind {
    let mut uf = UnionFind::new(n);
    for i in 1..n {
        uf.union(0, i).unwrap();
    }
    // Apply path compression by finding all elements
    for i in 0..n {
        uf.find(i).unwrap();
    }
    uf
}

// ============================================================================
// Find Operation Benchmarks
// ============================================================================

fn bench_find_various_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_operations");

    for &size in SMALL_SIZES {
        group.throughput(Throughput::Elements(size as u64));

        // Chain structure (worst case)
        let mut chain_uf = setup_chain_structure(size);
        group.bench_with_input(BenchmarkId::new("chain_structure", size), &size, |b, &s| {
            b.iter(|| {
                for i in 0..s {
                    black_box(chain_uf.find(i).unwrap());
                }
            });
        });

        // Balanced structure
        let mut balanced_uf = setup_balanced_structure(size);
        group.bench_with_input(
            BenchmarkId::new("balanced_structure", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..s {
                        black_box(balanced_uf.find(i).unwrap());
                    }
                });
            },
        );

        // Flat structure (best case after path compression)
        let mut flat_uf = setup_flat_structure(size);
        group.bench_with_input(BenchmarkId::new("flat_structure", size), &size, |b, &s| {
            b.iter(|| {
                for i in 0..s {
                    black_box(flat_uf.find(i).unwrap());
                }
            });
        });

        // Random structure
        let mut random_uf = setup_random_structure(size, 42);
        group.bench_with_input(
            BenchmarkId::new("random_structure", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..s {
                        black_box(random_uf.find(i).unwrap());
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_find_with_path_compression_benefit(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_path_compression_benefit");

    for &size in &[1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64));

        // First find (no path compression benefit)
        group.bench_with_input(BenchmarkId::new("first_find", size), &size, |b, &s| {
            b.iter(|| {
                let mut uf = setup_chain_structure(s);
                black_box(uf.find(s - 1).unwrap()); // Deepest element
            });
        });

        // Second find (with path compression benefit)
        group.bench_with_input(BenchmarkId::new("second_find", size), &size, |b, &s| {
            b.iter(|| {
                let mut uf = setup_chain_structure(s);
                uf.find(s - 1).unwrap(); // First find applies path compression
                black_box(uf.find(s - 1).unwrap()); // Second find benefits
            });
        });
    }

    group.finish();
}

// ============================================================================
// Union Operation Benchmarks
// ============================================================================

fn bench_union_operations_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("union_operations");

    for &size in SIZES {
        group.throughput(Throughput::Elements(size as u64));

        // Sequential unions (creates chain)
        group.bench_with_input(BenchmarkId::new("sequential", size), &size, |b, &s| {
            b.iter(|| {
                let mut uf = UnionFind::new(s);
                for i in 1..s {
                    black_box(uf.union(i - 1, i).unwrap());
                }
            });
        });

        // Random unions
        group.bench_with_input(BenchmarkId::new("random", size), &size, |b, &s| {
            b.iter(|| {
                let mut uf = UnionFind::new(s);
                let mut rng_state = 12345u64;

                for _ in 0..(s / 2) {
                    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                    let a = (rng_state % s as u64) as usize;

                    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                    let b = (rng_state % s as u64) as usize;

                    if a != b {
                        black_box(uf.union(a, b).unwrap());
                    }
                }
            });
        });

        // Balanced unions (union by powers of 2)
        group.bench_with_input(BenchmarkId::new("balanced", size), &size, |b, &s| {
            b.iter(|| {
                let mut uf = UnionFind::new(s);
                let mut step = 1;
                while step < s {
                    for i in (step..s).step_by(step * 2) {
                        if i >= step {
                            black_box(uf.union(i - step, i).unwrap());
                        }
                    }
                    step *= 2;
                }
            });
        });
    }

    group.finish();
}

fn bench_union_redundant_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("union_redundant");

    for &size in &[1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64));

        // Many redundant unions (already connected)
        group.bench_with_input(
            BenchmarkId::new("already_connected", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    let mut uf = UnionFind::new(s);
                    // Connect all to root
                    for i in 1..s {
                        uf.union(0, i).unwrap();
                    }
                    // Now try redundant unions
                    for i in 1..s {
                        black_box(uf.union(0, i).unwrap()); // Should return false
                    }
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Connected Query Benchmarks
// ============================================================================

fn bench_connected_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("connected_queries");

    for &size in &[1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size as u64));

        // All connected (one big component)
        let mut all_connected_uf = UnionFind::new(size);
        for i in 1..size {
            all_connected_uf.union(0, i).unwrap();
        }

        group.bench_with_input(BenchmarkId::new("all_connected", size), &size, |b, &s| {
            b.iter(|| {
                for i in 0..s.min(1000) {
                    // Limit queries for large sizes
                    for j in (i + 1)..s.min(1000) {
                        black_box(all_connected_uf.connected(i, j).unwrap());
                    }
                }
            });
        });

        // None connected (all singletons)
        let mut none_connected_uf = UnionFind::new(size);

        group.bench_with_input(BenchmarkId::new("none_connected", size), &size, |b, &s| {
            b.iter(|| {
                for i in 0..s.min(1000) {
                    for j in (i + 1)..s.min(1000) {
                        black_box(none_connected_uf.connected(i, j).unwrap());
                    }
                }
            });
        });

        // Random connectivity
        let mut random_uf = setup_random_structure(size, 54321);

        group.bench_with_input(
            BenchmarkId::new("random_connectivity", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..s.min(1000) {
                        for j in (i + 1)..s.min(1000) {
                            black_box(random_uf.connected(i, j).unwrap());
                        }
                    }
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Size and Count Benchmarks
// ============================================================================

fn bench_size_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("size_operations");

    for &size in &[1_000, 10_000, 100_000] {
        group.throughput(Throughput::Elements(size as u64));

        // One big component
        let mut big_component_uf = UnionFind::new(size);
        for i in 1..size {
            big_component_uf.union(0, i).unwrap();
        }

        group.bench_with_input(BenchmarkId::new("big_component", size), &size, |b, &s| {
            b.iter(|| {
                for i in 0..s {
                    black_box(big_component_uf.size(i).unwrap());
                }
            });
        });

        // Many small components
        let mut small_components_uf = UnionFind::new(size);
        for i in (0..size).step_by(10) {
            for j in 1..10.min(size - i) {
                if i + j < size {
                    small_components_uf.union(i, i + j).unwrap();
                }
            }
        }

        group.bench_with_input(
            BenchmarkId::new("small_components", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    for i in 0..s {
                        black_box(small_components_uf.size(i).unwrap());
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_count_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_operations");

    for &size in SIZES {
        group.throughput(Throughput::Elements(1)); // Count is O(1)

        let uf = setup_random_structure(size, 98765);

        group.bench_with_input(BenchmarkId::new("count", size), &size, |b, _| {
            b.iter(|| {
                black_box(uf.count());
            });
        });
    }

    group.finish();
}

// ============================================================================
// Mixed Workload Benchmarks
// ============================================================================

fn bench_mixed_workload(c: &mut Criterion) {
    let mut group = c.benchmark_group("mixed_workload");

    for &size in &[1_000, 10_000] {
        group.throughput(Throughput::Elements(size as u64));

        // Realistic workload: 40% union, 50% connected, 10% size
        group.bench_with_input(BenchmarkId::new("realistic", size), &size, |b, &s| {
            b.iter(|| {
                let mut uf = UnionFind::new(s);
                let mut rng_state = 13579u64;

                for _ in 0..s {
                    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                    let op_type = rng_state % 10;

                    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                    let a = (rng_state % s as u64) as usize;

                    rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                    let b = (rng_state % s as u64) as usize;

                    if a != b {
                        match op_type {
                            0..=3 => {
                                black_box(uf.union(a, b).unwrap());
                            } // 40%
                            4..=8 => {
                                black_box(uf.connected(a, b).unwrap());
                            } // 50%
                            9 => {
                                black_box(uf.size(a).unwrap());
                            } // 10%
                            _ => unreachable!(),
                        }
                    }
                }
            });
        });
    }

    group.finish();
}

// ============================================================================
// Memory Allocation Benchmarks
// ============================================================================

fn bench_creation_and_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("creation_setup");

    for &size in SIZES {
        group.throughput(Throughput::Elements(size as u64));

        // Just creation
        group.bench_with_input(BenchmarkId::new("new", size), &size, |b, &s| {
            b.iter(|| {
                black_box(UnionFind::new(s));
            });
        });

        // Creation + full connectivity
        group.bench_with_input(
            BenchmarkId::new("new_plus_full_connect", size),
            &size,
            |b, &s| {
                b.iter(|| {
                    let mut uf = UnionFind::new(s);
                    for i in 1..s {
                        uf.union(0, i).unwrap();
                    }
                    black_box(uf);
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Pathological Cases
// ============================================================================

fn bench_pathological_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathological_cases");

    // Extremely deep tree before path compression
    group.bench_function("deep_tree_first_find", |b| {
        b.iter(|| {
            let mut uf = UnionFind::new(1000);
            // Create a deep chain
            for i in 1..1000 {
                uf.union(i - 1, i).unwrap();
            }
            // Find the deepest element (triggers path compression)
            black_box(uf.find(999).unwrap());
        });
    });

    // Many small components
    group.bench_function("many_small_components", |b| {
        b.iter(|| {
            let mut uf = UnionFind::new(10000);
            // Create 1000 components of size 10 each
            for i in 0..1000 {
                let base = i * 10;
                for j in 1..10 {
                    if base + j < 10000 {
                        uf.union(base, base + j).unwrap();
                    }
                }
            }

            // Query connectivity across different components
            for i in 0..1000 {
                let base1 = (i * 10) % 10000;
                let base2 = ((i + 1) * 10) % 10000;
                black_box(uf.connected(base1, base2).unwrap());
            }
        });
    });

    group.finish();
}

// ============================================================================
// Benchmark Groups
// ============================================================================

criterion_group!(
    find_benches,
    bench_find_various_structures,
    bench_find_with_path_compression_benefit
);

criterion_group!(
    union_benches,
    bench_union_operations_scaling,
    bench_union_redundant_operations
);

criterion_group!(
    query_benches,
    bench_connected_queries,
    bench_size_operations,
    bench_count_operations
);

criterion_group!(
    workload_benches,
    bench_mixed_workload,
    bench_creation_and_setup
);

criterion_group!(pathological_benches, bench_pathological_cases);

criterion_main!(
    find_benches,
    union_benches,
    query_benches,
    workload_benches,
    pathological_benches
);
