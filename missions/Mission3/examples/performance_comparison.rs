/// Performance comparison between Mission3 implementation and std library
///
/// This validates that our implementations meet performance requirements.
use mission3::*;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Performance Comparison ===\n");

    // Test with different data sizes
    let sizes = [100, 1000, 10000, 100000];

    for &size in &sizes {
        println!("📊 Testing with {} elements", size);

        // Generate sorted test data
        let data: Vec<i32> = (0..size).map(|i| i * 2).collect(); // Even numbers

        // Test basic search
        compare_basic_search(&data);

        // Test bounds search
        compare_bounds_search(&data);

        println!();
    }

    // Test iterator performance
    test_iterator_performance();

    // Test trait overhead
    test_trait_overhead();

    println!("Performance validation complete! ✅");
}

fn compare_basic_search(data: &[i32]) {
    let target = data[data.len() / 2]; // Middle element
    let iterations = 10000;

    // Warm up
    for _ in 0..100 {
        let _ = data.binary_search(&target);
        let _ = binary_search::search_slice(data, &target);
    }

    // Benchmark std library
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = data.binary_search(&target);
    }
    let std_time = start.elapsed();

    // Benchmark our implementation
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = binary_search::search_slice(data, &target);
    }
    let our_time = start.elapsed();

    let ratio = our_time.as_nanos() as f64 / std_time.as_nanos() as f64;

    println!("  Basic search:");
    println!(
        "    std library:      {:>8} ns/op",
        std_time.as_nanos() / iterations
    );
    println!(
        "    our implementation: {:>8} ns/op",
        our_time.as_nanos() / iterations
    );
    println!("    ratio (ours/std): {:.2}x", ratio);

    if ratio < 2.0 {
        println!("    ✅ Performance acceptable (< 2x overhead)");
    } else {
        println!("    ⚠️  Performance concern (>= 2x overhead)");
    }
}

fn compare_bounds_search(data: &[i32]) {
    let target = data[data.len() / 2];
    let iterations = 5000;

    // Benchmark our left/right bounds
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = binary_search::search_left_bound(data, &target);
        let _ = binary_search::search_right_bound(data, &target);
    }
    let bounds_time = start.elapsed();

    println!("  Bounds search:");
    println!(
        "    left+right bounds: {:>8} ns/op",
        bounds_time.as_nanos() / iterations
    );

    // Compare with manual implementation for reference
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = data.binary_search(&target);
        let _ = data.binary_search(&target);
    }
    let double_search_time = start.elapsed();

    println!(
        "    2x basic search:   {:>8} ns/op",
        double_search_time.as_nanos() / iterations
    );

    let efficiency = bounds_time.as_nanos() as f64 / double_search_time.as_nanos() as f64;
    println!("    efficiency ratio: {:.2}x", efficiency);
}

fn test_iterator_performance() {
    println!("🔄 Iterator Performance Test");

    // Test data with many duplicates
    let mut data = Vec::new();
    for i in 0..10000 {
        for _ in 0..5 {
            // 5 copies of each number
            data.push(i);
        }
    }

    let target = 5000;
    let iterations = 1000;

    // Benchmark our iterator-based range finding
    let start = Instant::now();
    for _ in 0..iterations {
        let count = search_iter::find_all_equal(&data, &target).count();
        assert_eq!(count, 5); // Should find 5 copies
    }
    let iter_time = start.elapsed();

    // Benchmark manual search for comparison
    let start = Instant::now();
    for _ in 0..iterations {
        let count = data.iter().filter(|&&x| x == target).count();
        assert_eq!(count, 5);
    }
    let linear_time = start.elapsed();

    println!("  Iterator vs Linear search:");
    println!(
        "    Binary search iter: {:>8} ns/op",
        iter_time.as_nanos() / iterations
    );
    println!(
        "    Linear filter:      {:>8} ns/op",
        linear_time.as_nanos() / iterations
    );

    let speedup = linear_time.as_nanos() as f64 / iter_time.as_nanos() as f64;
    println!("    speedup: {:.1}x faster", speedup);

    if speedup > 1.0 {
        println!("    ✅ Binary search iterator is more efficient");
    } else {
        println!("    ⚠️  Binary search iterator is slower (unexpected for large data)");
    }
}

fn test_trait_overhead() {
    println!("🏗️ Trait Abstraction Overhead Test");

    let data: Vec<i32> = (0..50000).collect();
    let target = 25000;
    let iterations = 5000;

    // Direct slice search
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = binary_search::search_slice(&data, &target);
    }
    let direct_time = start.elapsed();

    // Trait-based search on same data
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = searchable::search_in(&data, &target);
    }
    let trait_time = start.elapsed();

    let overhead = trait_time.as_nanos() as f64 / direct_time.as_nanos() as f64;

    println!("  Direct vs Trait-based search:");
    println!(
        "    direct slice:    {:>8} ns/op",
        direct_time.as_nanos() / iterations
    );
    println!(
        "    trait-based:     {:>8} ns/op",
        trait_time.as_nanos() / iterations
    );
    println!("    overhead ratio: {:.2}x", overhead);

    if overhead < 1.1 {
        println!("    ✅ Zero-cost abstraction achieved (< 10% overhead)");
    } else if overhead < 1.5 {
        println!("    ✅ Acceptable abstraction cost (< 50% overhead)");
    } else {
        println!("    ⚠️  High abstraction overhead (>= 50%)");
    }

    // Test custom key extraction
    let people: Vec<(i32, &str)> = (0..50000).map(|i| (i, "person")).collect();
    let target_id = 25000;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = binary_search::search_by_key(&people, &target_id, |p| p.0);
    }
    let key_time = start.elapsed();

    let key_overhead = key_time.as_nanos() as f64 / direct_time.as_nanos() as f64;

    println!("  Key extraction search:");
    println!(
        "    by-key search:   {:>8} ns/op",
        key_time.as_nanos() / iterations
    );
    println!("    vs direct ratio: {:.2}x", key_overhead);

    if key_overhead < 2.0 {
        println!("    ✅ Key extraction overhead acceptable");
    } else {
        println!("    ⚠️  Key extraction overhead high");
    }
}

// Helper function to format duration nicely
fn _format_duration(duration: Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos < 1000 {
        format!("{}ns", nanos)
    } else if nanos < 1_000_000 {
        format!("{:.1}μs", nanos as f64 / 1000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.1}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.1}s", nanos as f64 / 1_000_000_000.0)
    }
}
