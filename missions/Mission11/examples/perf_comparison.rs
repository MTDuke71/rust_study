//! Demonstration of Exponential → Linear Complexity Transformation
//!
//! **REQ-6**: Overlapping Subproblem Detection
//!
//! Shows concrete example of memoization transforming intractable exponential
//! algorithm into efficient linear algorithm using instrumented caches.
//!
//! ## Fibonacci Example
//! - Naive: O(2^n) - exponential branching
//! - Memoized: O(n) - each subproblem computed once
//!
//! Run with: `cargo run -p mission11 --example perf_comparison`

use mission11::instrumentation::InstrumentedCache;
use std::time::Instant;

/// Naive recursive Fibonacci - O(2^n) exponential complexity
///
/// Every call branches into two more calls creating exponential tree.
/// Massive redundant computation: fib(5) called many times.
fn fibonacci_naive(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci_naive(n - 1) + fibonacci_naive(n - 2)
    }
}

/// Memoized Fibonacci - O(n) linear complexity
///
/// Each unique n computed exactly once, result cached.
/// Transforms exponential → linear via memoization.
fn fibonacci_memoized(n: u64, cache: &mut InstrumentedCache<u64, u64>) -> u64 {
    // Track recursion depth
    cache.stats_mut().enter_recursion();

    // Base case
    if n <= 1 {
        cache.stats_mut().exit_recursion();
        return n;
    }

    // Check cache
    if let Some(&result) = cache.get(&n) {
        cache.stats_mut().exit_recursion();
        return result;
    }

    // Recursive computation
    let result = fibonacci_memoized(n - 1, cache) + fibonacci_memoized(n - 2, cache);

    // Cache result
    cache.insert(n, result);

    cache.stats_mut().exit_recursion();
    result
}

/// Count number of ways to climb stairs (1 or 2 steps at a time)
///
/// Classic DP problem demonstrating overlapping subproblems.
#[allow(dead_code)]
fn climb_stairs_naive(n: u64) -> u64 {
    if n <= 2 {
        n
    } else {
        climb_stairs_naive(n - 1) + climb_stairs_naive(n - 2)
    }
}

fn climb_stairs_memoized(n: u64, cache: &mut InstrumentedCache<u64, u64>) -> u64 {
    cache.stats_mut().enter_recursion();

    if n <= 2 {
        cache.stats_mut().exit_recursion();
        return n;
    }

    if let Some(&result) = cache.get(&n) {
        cache.stats_mut().exit_recursion();
        return result;
    }

    let result = climb_stairs_memoized(n - 1, cache) + climb_stairs_memoized(n - 2, cache);
    cache.insert(n, result);

    cache.stats_mut().exit_recursion();
    result
}

/// Demonstrate complexity transformation with timing
fn compare_fibonacci(n: u64) {
    println!("=== Fibonacci({}) Comparison ===\n", n);

    // Naive timing (only for small n to avoid timeout)
    let naive_time = if n <= 30 {
        let start = Instant::now();
        let result = fibonacci_naive(n);
        let elapsed = start.elapsed();
        println!("Naive Result: {}", result);
        println!("Naive Time: {:?}", elapsed);
        Some(elapsed.as_nanos())
    } else {
        println!("Naive: SKIPPED (would timeout for n={})", n);
        None
    };

    println!();

    // Memoized timing
    let mut cache = InstrumentedCache::new();
    let start = Instant::now();
    let result = fibonacci_memoized(n, &mut cache);
    let memoized_time = start.elapsed();

    println!("Memoized Result: {}", result);
    println!("Memoized Time: {:?}", memoized_time);
    println!();

    // Statistics
    println!("{}", cache.stats().report());
    println!();

    // Speedup
    if let Some(naive_ns) = naive_time {
        let speedup = naive_ns as f64 / memoized_time.as_nanos() as f64;
        println!("Speedup: {:.2}x faster", speedup);
    } else {
        println!("Speedup: ∞ (exponential → linear transformation)");
    }

    println!("\n{}", "=".repeat(50));
}

/// Demonstrate cache statistics analysis
fn analyze_cache_behavior(n: u64) {
    println!("=== Cache Behavior Analysis (Stairs n={}) ===\n", n);

    let mut cache = InstrumentedCache::new();
    let result = climb_stairs_memoized(n, &mut cache);

    println!("Result: {} ways to climb {} stairs\n", result, n);

    let stats = cache.stats();
    println!("{}", stats.report());
    println!();

    // Analysis
    println!("Analysis:");
    println!("- Theoretical state space: {} (0 to {})", n + 1, n);
    println!("- Actual cache size: {}", stats.max_size);
    println!(
        "- Coverage: {:.1}%",
        (stats.max_size as f64 / (n + 1) as f64) * 100.0
    );
    println!(
        "- Lookups per cache entry: {:.2}",
        stats.lookups as f64 / stats.max_size as f64
    );

    if stats.is_memoization_beneficial() {
        println!("\n✅ Memoization is HIGHLY beneficial!");
        println!(
            "   Hit ratio {:.1}% indicates significant subproblem overlap",
            stats.hit_ratio() * 100.0
        );
    } else {
        println!("\n⚠️  Memoization may not be beneficial for this problem");
    }

    println!("\n{}", "=".repeat(50));
}

/// Show complexity transformation with different problem sizes
fn scaling_demonstration() {
    println!("=== Scaling Demonstration ===\n");
    println!("| n  | Naive (ms) | Memoized (µs) | Speedup    | Cache Size |");
    println!("|----|-----------:|-------------:|------------|-----------|");

    for &n in &[10, 15, 20, 25, 30] {
        // Naive
        let naive_start = Instant::now();
        let _ = fibonacci_naive(n);
        let naive_time = naive_start.elapsed();

        // Memoized
        let mut cache = InstrumentedCache::new();
        let memo_start = Instant::now();
        let _ = fibonacci_memoized(n, &mut cache);
        let memo_time = memo_start.elapsed();

        let speedup = naive_time.as_nanos() as f64 / memo_time.as_nanos() as f64;

        println!(
            "| {:<2} | {:>10.3} | {:>12.1} | {:>10.1}x | {:>9} |",
            n,
            naive_time.as_secs_f64() * 1000.0,
            memo_time.as_micros(),
            speedup,
            cache.len()
        );
    }

    println!("\n{}", "=".repeat(50));
}

fn main() {
    println!("# Mission 11 REQ-6: Performance Comparison Demo\n");
    println!("Demonstrates exponential → linear complexity transformation\n");

    // Small problem - show both approaches
    compare_fibonacci(20);
    println!();

    // Large problem - naive would timeout
    compare_fibonacci(50);
    println!();

    // Cache behavior analysis
    analyze_cache_behavior(40);
    println!();

    // Scaling demonstration
    scaling_demonstration();
    println!();

    println!("## Key Takeaways\n");
    println!("1. **Complexity Transformation**: Memoization changes O(2^n) → O(n)");
    println!("2. **High Hit Ratio**: >50% indicates significant subproblem overlap");
    println!("3. **Cache Size**: Much smaller than total recursive calls");
    println!("4. **Speedup**: Exponential growth in speedup as n increases");
    println!("5. **Tractability**: Makes impossible problems instant (n=50+)");
}
