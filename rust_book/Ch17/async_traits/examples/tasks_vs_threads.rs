// Example: Tasks vs Threads - Choosing the right concurrency model

use tokio::time::{sleep, Duration, Instant};

/// CPU-bound work - best for threads
fn cpu_intensive_work(n: u64) -> u64 {
    (0..n).fold(0, |acc, x| acc.wrapping_add(x * x))
}

/// I/O-bound work - best for async tasks
async fn io_intensive_work(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("I/O result {}", id)
}

#[tokio::main]
async fn main() {
    println!("=== Tasks vs Threads Decision Guide ===\n");

    // Example 1: I/O-bound workload - async wins
    println!("Example 1: I/O-bound (async tasks)");
    let start = Instant::now();

    let tasks: Vec<_> = (0..100)
        .map(|i| tokio::spawn(io_intensive_work(i)))
        .collect();

    for task in tasks {
        task.await.unwrap();
    }

    println!("100 I/O tasks completed in {:?}", start.elapsed());
    println!("Memory: ~few hundred KB\n");

    // Example 2: CPU-bound workload - threads win
    println!("Example 2: CPU-bound (thread pool)");
    let start = Instant::now();

    let threads: Vec<_> = (0..4)
        .map(|i| std::thread::spawn(move || cpu_intensive_work(10_000_000 * (i + 1))))
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }

    println!("4 CPU-intensive threads completed in {:?}", start.elapsed());
    println!("Used available cores efficiently\n");

    // Example 3: Mixed workload - hybrid approach
    println!("Example 3: Mixed workload (hybrid)");
    let start = Instant::now();

    // Async for I/O coordination
    let data = io_intensive_work(1).await;

    // Spawn thread for CPU work
    let processed = tokio::task::spawn_blocking(move || {
        cpu_intensive_work(5_000_000);
        format!("Processed: {}", data)
    })
    .await
    .unwrap();

    // Back to async for final I/O
    println!("{}", processed);
    sleep(Duration::from_millis(50)).await;

    println!("Hybrid completed in {:?}\n", start.elapsed());

    // Example 4: Scalability comparison
    println!("Example 4: Scalability");
    println!("┌─────────────────┬─────────────┬─────────────┐");
    println!("│ Scenario        │ Threads     │ Async Tasks │");
    println!("├─────────────────┼─────────────┼─────────────┤");
    println!("│ 10 concurrent   │ ~10 MB      │ ~100 KB     │");
    println!("│ 1000 concurrent │ ~1 GB       │ ~10 MB      │");
    println!("│ 10000 concurrent│ Crashes     │ ~100 MB     │");
    println!("└─────────────────┴─────────────┴─────────────┘\n");

    // Example 5: Real-world decision matrix
    println!("Example 5: When to use what");
    println!("\n✅ Use Async Tasks When:");
    println!("  - Waiting on I/O (network, disk, timers)");
    println!("  - High concurrency needed (1000s of connections)");
    println!("  - Non-blocking is critical");
    println!("  - Examples: web servers, API clients, chat apps");

    println!("\n✅ Use Threads When:");
    println!("  - CPU-intensive computation");
    println!("  - Blocking operations (no async alternative)");
    println!("  - Parallel data processing");
    println!("  - Examples: image processing, crypto, simulations");

    println!("\n✅ Use Both (Hybrid) When:");
    println!("  - I/O coordination + CPU processing");
    println!("  - Web server with compute endpoints");
    println!("  - Data pipeline: async fetch → thread process → async store");

    // Example 6: Common mistakes
    println!("\n\n❌ Common Mistakes:");

    println!("\nMistake 1: Blocking in async context");
    println!("// BAD");
    println!("async fn bad() {{");
    println!("    std::thread::sleep(Duration::from_secs(1));  // Blocks executor!");
    println!("}}");
    println!("\n// GOOD");
    println!("async fn good() {{");
    println!("    tokio::time::sleep(Duration::from_secs(1)).await;  // Non-blocking");
    println!("}}");

    println!("\nMistake 2: Using async for CPU work");
    println!("// BAD - async overhead without I/O benefit");
    println!("async fn bad() {{");
    println!("    expensive_cpu_work();  // Blocks entire executor thread");
    println!("}}");
    println!("\n// GOOD - offload to thread pool");
    println!("async fn good() {{");
    println!("    tokio::task::spawn_blocking(|| expensive_cpu_work()).await");
    println!("}}");

    println!("\n\n🎯 Key Takeaways:");
    println!("- I/O-bound → async tasks (scales to millions)");
    println!("- CPU-bound → threads (limited by cores)");
    println!("- Mixed → hybrid with spawn_blocking");
    println!("- Never block the async executor");
}
