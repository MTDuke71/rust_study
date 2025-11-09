# Performance Benchmarking

*Created: 2025-11-08*
*Tags: #rust-benchmarking #criterion #performance-measurement #optimization #profiling*

## Overview

Performance benchmarking in Rust involves **systematic measurement** of code execution time, memory usage, and throughput to validate optimizations and compare algorithms. Rust's ecosystem provides sophisticated tools for **accurate**, **reproducible** benchmarking.

## Core Tools

### Criterion.rs - Gold Standard
```rust
// Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmarks"
harness = false
```

```rust
// benches/my_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_recursive(n-1) + fibonacci_recursive(n-2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

### Built-in Benchmarking (Nightly)
```rust
#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_algorithm(b: &mut Bencher) {
        b.iter(|| {
            // Algorithm to benchmark
            test::black_box(expensive_computation())
        });
    }
}
```

## Mission Benchmarking Patterns

### Mission 1: Stack Performance
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mission1::Stack;

fn benchmark_stack_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack_operations");
    
    // Benchmark push operations
    group.bench_function("push_1000", |b| {
        b.iter(|| {
            let mut stack = Stack::new();
            for i in 0..1000 {
                stack.push(black_box(i));
            }
        });
    });
    
    // Benchmark pop operations
    group.bench_function("pop_1000", |b| {
        b.iter_with_setup(
            || {
                let mut stack = Stack::new();
                for i in 0..1000 {
                    stack.push(i);
                }
                stack
            },
            |mut stack| {
                while stack.pop().is_some() {
                    // Pop all elements
                }
            }
        );
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_stack_operations);
criterion_main!(benches);
```

### Mission 5: HashMap Performance
```rust
fn benchmark_hashmap_vs_btreemap(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_comparison");
    
    let data: Vec<(String, i32)> = (0..1000)
        .map(|i| (format!("key_{}", i), i))
        .collect();
    
    group.bench_function("hashmap_insert", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for (k, v) in &data {
                map.insert(k.clone(), black_box(*v));
            }
        });
    });
    
    group.bench_function("btreemap_insert", |b| {
        b.iter(|| {
            let mut map = BTreeMap::new();
            for (k, v) in &data {
                map.insert(k.clone(), black_box(*v));
            }
        });
    });
    
    group.finish();
}
```

### Mission 6: Grid Operations
```rust
fn benchmark_grid_access_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_access");
    let grid = Grid::new(1000, 1000, 0i32);
    
    // Row-major access (cache-friendly)
    group.bench_function("row_major", |b| {
        b.iter(|| {
            let mut sum = 0;
            for row in 0..grid.height() {
                for col in 0..grid.width() {
                    sum += grid.get(row, col).unwrap_or(&0);
                }
            }
            black_box(sum)
        });
    });
    
    // Column-major access (cache-unfriendly)
    group.bench_function("column_major", |b| {
        b.iter(|| {
            let mut sum = 0;
            for col in 0..grid.width() {
                for row in 0..grid.height() {
                    sum += grid.get(row, col).unwrap_or(&0);
                }
            }
            black_box(sum)
        });
    });
    
    group.finish();
}
```

## Advanced Benchmarking Techniques

### Input Size Scaling
```rust
fn benchmark_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("algorithm_scaling");
    
    for size in [100, 1000, 10000, 100000].iter() {
        group.bench_with_input(BenchmarkId::new("sort", size), size, |b, &size| {
            b.iter_with_setup(
                || (0..size).rev().collect::<Vec<_>>(), // Worst case: reverse sorted
                |mut data| data.sort()
            );
        });
    }
    
    group.finish();
}
```

### Memory Usage Profiling
```rust
use criterion::{measurement::WallTime, BenchmarkGroup};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Custom allocator to track memory usage
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Ordering::SeqCst);
        }
        ret
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

fn benchmark_with_memory_tracking(c: &mut Criterion) {
    c.bench_function("memory_usage", |b| {
        b.iter(|| {
            let start_mem = ALLOCATED.load(Ordering::SeqCst);
            let result = expensive_operation();
            let end_mem = ALLOCATED.load(Ordering::SeqCst);
            println!("Memory used: {} bytes", end_mem - start_mem);
            black_box(result)
        });
    });
}
```

## AoC Benchmarking Patterns

### String Processing Optimization
```rust
fn benchmark_aoc_day11(c: &mut Criterion) {
    let mut group = c.benchmark_group("password_generation");
    let input = "abcdefgh";
    
    // Naive string approach
    group.bench_function("string_based", |b| {
        b.iter(|| {
            find_next_password_string(black_box(input))
        });
    });
    
    // Optimized byte array approach
    group.bench_function("byte_array", |b| {
        b.iter(|| {
            find_next_password_bytes(black_box(input.as_bytes()))
        });
    });
    
    group.finish();
}
```

### Algorithm Comparison
```rust
fn benchmark_pathfinding(c: &mut Criterion) {
    let mut group = c.benchmark_group("pathfinding_algorithms");
    let grid = generate_test_grid(100, 100);
    
    group.bench_function("dijkstra", |b| {
        b.iter(|| dijkstra_pathfind(&grid, (0, 0), (99, 99)));
    });
    
    group.bench_function("a_star", |b| {
        b.iter(|| a_star_pathfind(&grid, (0, 0), (99, 99)));
    });
    
    group.bench_function("bfs", |b| {
        b.iter(|| bfs_pathfind(&grid, (0, 0), (99, 99)));
    });
    
    group.finish();
}
```

## Performance Analysis Techniques

### Statistical Analysis
```rust
fn benchmark_with_statistics(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistical_analysis");
    
    // Configure sample size and measurement time
    group.sample_size(1000);
    group.measurement_time(Duration::from_secs(10));
    
    // Warm-up iterations
    group.warm_up_time(Duration::from_secs(1));
    
    group.bench_function("algorithm", |b| {
        b.iter(|| expensive_computation())
    });
    
    group.finish();
}
```

### Regression Testing
```rust
// Save baseline performance
fn save_baseline(c: &mut Criterion) {
    c.bench_function("baseline_algorithm", |b| {
        b.iter(|| baseline_implementation())
    });
}

// Compare against baseline
fn compare_optimization(c: &mut Criterion) {
    c.bench_function("optimized_algorithm", |b| {
        b.iter(|| optimized_implementation())
    });
}
```

## Profiling Integration

### CPU Profiling with perf
```bash
# Build with debug symbols
cargo build --release --bin my_app

# Run with perf
perf record --call-graph=dwarf ./target/release/my_app

# Analyze results
perf report

# Generate flamegraph
perf script | stackcollapse-perf.pl | flamegraph.pl > profile.svg
```

### Memory Profiling with Valgrind
```bash
# Install valgrind tools
sudo apt install valgrind

# Run memory profiler
valgrind --tool=massif ./target/release/my_app

# Analyze memory usage
ms_print massif.out.*
```

## Common Pitfalls

### Optimizer Interference
```rust
// Wrong: optimizer might eliminate work
fn bad_benchmark(b: &mut Bencher) {
    b.iter(|| {
        expensive_computation() // Result not used, might be optimized away
    });
}

// Correct: use black_box to prevent optimization
fn good_benchmark(b: &mut Bencher) {
    b.iter(|| {
        black_box(expensive_computation()) // Forces evaluation
    });
}
```

### Input Bias
```rust
// Wrong: always same input
fn biased_benchmark(b: &mut Bencher) {
    let data = vec![1, 2, 3, 4, 5]; // Same every time
    b.iter(|| process_data(&data));
}

// Correct: varied inputs
fn unbiased_benchmark(b: &mut Bencher) {
    b.iter_with_setup(
        || generate_random_data(1000), // New data each iteration
        |data| process_data(&data)
    );
}
```

### Measurement Noise
```rust
// Configure for stable measurements
fn stable_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("stable_measurement");
    
    // Increase sample size for statistical significance
    group.sample_size(1000);
    
    // Longer measurement time reduces noise  
    group.measurement_time(Duration::from_secs(10));
    
    // Multiple measurements for confidence
    group.confidence_level(0.95);
    group.significance_level(0.05);
    
    group.bench_function("algorithm", |b| {
        b.iter(|| algorithm_under_test())
    });
}
```

## Integration with Other Concepts

- **[[zero-cost-abstractions]]**: Validating abstraction performance claims
- **[[Performance Patterns]]**: Measuring optimization effectiveness
- **[[Big-O Notation]]**: Empirical complexity validation
- **[[Memory Safety]]**: Performance cost of safety features
- **[[interior-mutability]]**: Runtime checking overhead measurement

## Daily Study Applications

### Week 2: Collections Performance
- Benchmark different collection types
- Measure iterator vs loop performance

### Week 5: Error Handling Costs
- Result<T, E> vs Option<T> performance
- Error propagation overhead measurement

### Week 6: Advanced Patterns
- Trait object vs generic dispatch costs
- Async runtime performance characteristics

## Mission Integration Examples

### Mission Reports
- **Mission 1**: Stack operation Big-O validation
- **Mission 4**: Rc<RefCell<T>> vs Box<T> comparison
- **Mission 5**: Hash function performance analysis
- **Mission 6**: Grid access pattern optimization
- **Mission 10**: Union-Find operation scaling

### Tutorial Validation
- Benchmark tutorial examples vs hand-optimized code
- Validate [[zero-cost-abstractions]] claims
- Measure learning progression efficiency gains

## Further Reading

- **Criterion.rs Documentation**: Comprehensive benchmarking guide
- **[[Performance Patterns]]**: Optimization techniques to measure
- **[[Big-O Notation]]**: Theoretical vs empirical complexity
- **[[zero-cost-abstractions]]**: Validating abstraction costs

---

*Performance Benchmarking Links:*
- [[zero-cost-abstractions]] - Validating abstraction costs
- [[Performance Patterns]] - Optimization measurement
- [[Big-O Notation]] - Complexity validation
- [[Memory Safety]] - Safety feature costs
- [[interior-mutability]] - Runtime overhead measurement
- [[benchmarking]] - General benchmarking concepts
- [[mission-1]] - Stack performance validation
- [[mission-5]] - HashMap performance analysis
- [[mission-6]] - Grid operation optimization