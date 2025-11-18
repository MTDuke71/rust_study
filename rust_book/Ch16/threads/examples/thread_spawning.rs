// Example: Various Thread Spawning Patterns
// Different ways to create and manage threads

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Thread Spawning Patterns ===\n");

    // Pattern 1: Simple computation in background
    println!("Pattern 1: Background computation");
    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=1000 {
            sum += i;
        }
        sum
    });

    println!("Main thread continues while background computes...");
    let result = handle.join().unwrap();
    println!("Background result: {}\n", result);

    // Pattern 2: Named thread for debugging
    println!("Pattern 2: Named thread");
    thread::Builder::new()
        .name("worker".to_string())
        .spawn(|| {
            println!("Worker thread: {:?}", thread::current().name());
        })
        .unwrap()
        .join()
        .unwrap();
    println!();

    // Pattern 3: Thread with custom stack size
    println!("Pattern 3: Custom stack size");
    thread::Builder::new()
        .stack_size(32 * 1024) // 32 KB stack
        .spawn(|| {
            println!("Thread with custom stack running");
        })
        .unwrap()
        .join()
        .unwrap();
    println!();

    // Pattern 4: Concurrent task execution
    println!("Pattern 4: Concurrent tasks");
    let task_count = 4;
    let handles: Vec<_> = (0..task_count)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(100 * (i + 1)));
                println!("Task {} complete", i);
                i
            })
        })
        .collect();

    println!("Waiting for all tasks...");
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("All tasks complete: {:?}\n", results);

    // Pattern 5: Scoped threads (requires std::thread::scope - Rust 1.63+)
    println!("Pattern 5: Scoped threads (safe borrowing)");
    let mut data = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        s.spawn(|| {
            println!("Scoped thread can borrow: {:?}", data);
        });
    }); // All scoped threads joined here

    println!("Data after scoped thread: {:?}", data);

    // Pattern 6: Scoped threads with mutation
    println!("\nPattern 6: Scoped threads with mutation");
    thread::scope(|s| {
        s.spawn(|| {
            data[0] = 100; // Can mutate safely in scope
            println!("Scoped thread mutated first element");
        });
    }); // Thread joined here before we use data again

    println!("Modified data: {:?}", data);
}
