//! # Chapter 17.6: Putting It All Together - Futures, Tasks, and Threads
//!
//! This module demonstrates the relationship between futures, tasks, and threads,
//! and when to choose each approach for concurrent programming.
//!
//! ## Key Concepts:
//! - **Threads**: OS-managed, good for parallelism and CPU-bound work
//! - **Tasks**: Runtime-managed, lightweight, good for I/O-bound concurrency
//! - **Futures**: The most granular unit of concurrency in Rust
//!
//! ## When to Use What:
//! - **Parallelizable work** (data processing) → Threads
//! - **Concurrent work** (handling many connections) → Async tasks
//! - **Both** → Combine threads and async freely!

use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};

/// Listing 17-40: Using async task with trpl::spawn_task (tokio equivalent)
///
/// Creates a stream that yields intervals using an async task.
/// The task runs on the async runtime and yields to other tasks during sleep.
pub fn get_intervals_async() -> impl Stream<Item = u32> {
    let (tx, rx) = mpsc::channel(16);

    // Spawn an async task (equivalent to trpl::spawn_task)
    tokio::spawn(async move {
        let mut count = 0;
        loop {
            // Async sleep - yields to runtime
            tokio::time::sleep(Duration::from_millis(1)).await;
            count += 1;

            if tx.send(count).await.is_err() {
                eprintln!("Could not send interval {count}: receiver dropped");
                break;
            }

            // Stop after 10 for demo purposes
            if count >= 10 {
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

/// Listing 17-41: Using std::thread instead of async task
///
/// Creates a stream that yields intervals using an OS thread.
/// This is *not* async - it uses blocking thread::sleep.
///
/// Notice how similar the API is to the async version above!
/// The calling code doesn't need to know whether it's async or threaded.
pub fn get_intervals_thread() -> impl Stream<Item = u32> {
    let (tx, rx) = mpsc::channel(16);

    // This is *not* tokio::spawn but std::thread::spawn!
    thread::spawn(move || {
        let mut count = 0;
        loop {
            // Likewise, this is *not* tokio::time::sleep but std::thread::sleep!
            thread::sleep(Duration::from_millis(1));
            count += 1;

            // Use blocking send since we're in a sync context.
            // NOTE: If the receiver is dropped (e.g., consumer stops early),
            // this will return an error - that's expected graceful shutdown!
            // The "Could not send interval X" message demonstrates proper
            // channel closure detection, not a bug.
            if tx.blocking_send(count).is_err() {
                // Expected when receiver drops - thread exits cleanly
                break;
            }

            // Stop after 10 for demo purposes
            if count >= 10 {
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

/// Listing 17-42: Mixing threads and async - sending from thread, receiving async
///
/// This demonstrates a common real-world pattern:
/// - Use a thread for blocking/CPU-intensive work
/// - Use async for I/O-bound message handling
///
/// Example use case: Video encoding (CPU-bound) notifying UI (async) when done.
pub async fn mixed_thread_and_async_demo() {
    println!("\n=== Listing 17-42: Mixing Threads and Async ===");
    println!("Spawning a thread to send messages...\n");

    let (tx, mut rx) = mpsc::channel(16);

    // Spawn a regular OS thread for the sending side
    thread::spawn(move || {
        for i in 1..=10 {
            // blocking_send works from sync context to async channel
            tx.blocking_send(i).unwrap();
            println!("  [Thread] Sent: {i}");
            thread::sleep(Duration::from_millis(100)); // Blocking sleep
        }
        println!("  [Thread] Done sending!");
    });

    // Receive messages asynchronously
    println!("Receiving messages asynchronously...");
    while let Some(message) = rx.recv().await {
        println!("  [Async] Received: {message}");
    }
    println!("All messages received!");
}

/// Demonstrates the key differences between tasks and threads
pub async fn tasks_vs_threads_comparison() {
    println!("\n=== Tasks vs Threads Comparison ===\n");

    // Key insight: Tasks are lightweight, threads are heavyweight
    println!("📊 Resource Comparison:");
    println!("   • Thread: ~2MB stack by default, OS-managed");
    println!("   • Task: ~few KB, runtime-managed");
    println!("   • You can spawn millions of tasks, but not threads!\n");

    println!("🔄 Concurrency Model:");
    println!("   • Threads: Concurrency BETWEEN threads only");
    println!("   • Tasks: Concurrency BETWEEN and WITHIN tasks (via futures)\n");

    println!("🎯 When to Use:");
    println!("   • Threads → Parallelizable work (data processing, computation)");
    println!("   • Async   → Concurrent work (network I/O, handling connections)");
    println!("   • Both    → Complex systems (video encoding + UI updates)\n");

    // Demonstrate both approaches produce identical results
    println!("Demo: Stream from ASYNC TASK (Listing 17-40 pattern):");
    let mut stream = get_intervals_async();
    let mut count = 0;
    while let Some(value) = stream.next().await {
        print!("{value} ");
        count += 1;
        if count >= 5 {
            println!("...(stopping early for demo)");
            break;
        }
    }

    println!("\nDemo: Stream from OS THREAD (Listing 17-41 pattern):");
    let mut stream = get_intervals_thread();
    let mut count = 0;
    while let Some(value) = stream.next().await {
        print!("{value} ");
        count += 1;
        if count >= 5 {
            println!("...(stopping early for demo)");
            break;
        }
    }

    println!("\n   ↑ Notice: Both produce identical output!");
    println!("   The calling code doesn't know (or care) which backs the stream.");
}

/// Demonstrates work-stealing concept (conceptual, not actual implementation)
pub fn explain_work_stealing() {
    println!("\n=== Work Stealing Explained ===\n");

    println!("Modern async runtimes (like Tokio) use WORK STEALING:");
    println!();
    println!("  ┌──────────────────────────────────────────────┐");
    println!("  │              Async Runtime                    │");
    println!("  │  ┌─────────┐  ┌─────────┐  ┌─────────┐       │");
    println!("  │  │Thread 1 │  │Thread 2 │  │Thread 3 │       │");
    println!("  │  │ Task A  │  │ Task C  │  │ (idle)  │       │");
    println!("  │  │ Task B  │  │ Task D  │  │    ↑    │       │");
    println!("  │  │   ↓     │  │         │  │ steals! │       │");
    println!("  │  │ Task E  │──┼─────────┼──│ Task E  │       │");
    println!("  │  └─────────┘  └─────────┘  └─────────┘       │");
    println!("  └──────────────────────────────────────────────┘");
    println!();
    println!("• Tasks can be moved between threads transparently");
    println!("• Idle threads 'steal' work from busy threads");
    println!("• This requires threads AND tasks working together");
    println!("• Tokio's #[tokio::main] sets this up automatically");
}

/// Summary of Chapter 17.6 concepts
pub fn chapter_summary() {
    println!("\n{}", "=".repeat(60));
    println!("          CHAPTER 17.6 SUMMARY");
    println!("{}", "=".repeat(60));

    println!("\n📚 Hierarchy of Concurrency:");
    println!("   Futures (finest) → Tasks → Threads (coarsest)");

    println!("\n🔑 Key Takeaways:");
    println!("   1. Threads and async are COMPLEMENTARY, not competing");
    println!("   2. Tasks are lightweight (millions possible)");
    println!("   3. Threads have no built-in cancellation");
    println!("   4. Futures compose naturally; threads don't");
    println!("   5. Use spawn_blocking for CPU work in async context");

    println!("\n📋 Decision Guide:");
    println!("   ┌─────────────────────┬─────────────────────┐");
    println!("   │ Parallelism needed? │ → Use threads       │");
    println!("   │ Concurrency needed? │ → Use async         │");
    println!("   │ Both needed?        │ → Combine them!     │");
    println!("   └─────────────────────┴─────────────────────┘");

    println!("\n🎯 Real-World Pattern (Listing 17-42):");
    println!("   Thread (CPU work) ──channel──▶ Async (I/O handling)");
    println!("   Example: Video encoder notifying UI when done");
}

/// Run all demonstrations
pub async fn run_all_demos() {
    println!("{}", "=".repeat(60));
    println!("  CHAPTER 17.6: Futures, Tasks, and Threads");
    println!("{}", "=".repeat(60));

    tasks_vs_threads_comparison().await;
    explain_work_stealing();
    mixed_thread_and_async_demo().await;
    chapter_summary();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_thread_stream_produces_values() {
        let mut stream = get_intervals_thread();
        let first = stream.next().await;
        assert!(first.is_some());
        assert_eq!(first.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_async_stream_produces_values() {
        let mut stream = get_intervals_async();
        let first = stream.next().await;
        assert!(first.is_some());
        assert_eq!(first.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_mixed_thread_async() {
        // Just verify it runs without panic
        let (tx, mut rx) = mpsc::channel(4);

        thread::spawn(move || {
            for i in 1..=3 {
                tx.blocking_send(i).unwrap();
            }
        });

        let mut received = Vec::new();
        while let Some(msg) = rx.recv().await {
            received.push(msg);
        }

        assert_eq!(received, vec![1, 2, 3]);
    }
}
