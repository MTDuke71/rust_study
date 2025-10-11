// Performance Validation (Simple Version)
// Run with: cargo run --example performance_check

use mission1::Stack;
use std::time::Instant;

fn main() {
    println!("⚡ Performance Validation\n");

    // ==============================================
    // Test 1: Push performance scales linearly
    // ==============================================
    println!("📊 Test 1: Push Performance Scaling");

    let test_sizes = vec![1_000, 10_000, 100_000];

    for size in test_sizes {
        let start = Instant::now();

        let mut stack = Stack::new();
        for i in 0..size {
            stack.push(i);
        }

        let duration = start.elapsed();
        let ops_per_sec = size as f64 / duration.as_secs_f64();

        println!(
            "  {} pushes took {:?} ({:.0} ops/sec)",
            size, duration, ops_per_sec
        );
    }

    println!("✅ Push should scale roughly linearly (more items = proportionally more time)");

    // ==============================================
    // Test 2: Pop performance is constant
    // ==============================================
    println!("\n📊 Test 2: Pop Performance (should be constant time)");

    let mut stack = Stack::new();

    // Fill with lots of data
    for i in 0..100_000 {
        stack.push(i);
    }

    // Time individual pops
    let mut pop_times = Vec::new();
    for _ in 0..10 {
        let start = Instant::now();
        stack.pop();
        let duration = start.elapsed();
        pop_times.push(duration);
    }

    println!("Individual pop times:");
    for (i, time) in pop_times.iter().enumerate() {
        println!("  Pop {}: {:?}", i + 1, time);
    }

    println!("✅ Pop times should be roughly similar (O(1) - constant time)");

    // ==============================================
    // Test 3: Memory usage
    // ==============================================
    println!("\n💾 Test 3: Memory Efficiency");

    let mut stack = Stack::new();
    println!("Empty stack size: {} bytes", std::mem::size_of_val(&stack));

    // Add some items
    for i in 0..1000 {
        stack.push(i);
    }

    println!(
        "Stack with 1000 items: still {} bytes (items stored separately)",
        std::mem::size_of_val(&stack)
    );
    println!("✅ Stack struct itself is small (just holds a Vec)");

    println!("\n🏁 Performance validation complete!");
    println!("📝 For production code, use Criterion crate for precise benchmarks");
}
