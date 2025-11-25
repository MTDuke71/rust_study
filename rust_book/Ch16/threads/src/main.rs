// Chapter 16.1: Using Threads to Run Code Simultaneously
// Basic thread spawning and join handles

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Without JoinHandle (thread may not complete) ===\n");

    // NOTE: This thread will continue running in the background even after
    // this example "finishes" - it becomes an orphaned thread that can
    // interfere with subsequent examples!

    // Spawn a thread WITHOUT saving the handle
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread (Example 1)!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread runs and may exit before spawned thread finishes
    for i in 1..5 {
        println!("hi number {} from the main thread (Example 1)!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("(Main thread finishing - spawned thread may be cut off!)\n");

    println!("=== With JoinHandle (blocks main thread immediately) ===\n");

    // Spawn a thread AND save the handle
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread (Example 2)!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for the spawned thread to finish BEFORE main thread work
    handle.join().unwrap();

    // Main thread runs AFTER spawned thread completes
    for i in 1..5 {
        println!("hi number {} from the main thread (Example 2)!", i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("(Spawned thread completed first, then main thread ran!)\n");

    println!("=== With JoinHandle (concurrent execution) ===\n");

    // Spawn a thread AND save the handle
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread (Example 3)!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread runs concurrently
    for i in 1..5 {
        println!("hi number {} from the main thread (Example 3)!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish AFTER main thread work
    handle.join().unwrap();

    println!("(Both threads ran concurrently, main waited for spawned to complete!)\n");

    println!("=== Using move to Transfer Ownership ===\n");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // v is no longer accessible here - ownership was moved

    handle.join().unwrap();

    println!("\nThread examples complete!");
    println!("Run examples with: cargo run --example <name>");
    println!("  - join_handles");
    println!("  - move_closures");
    println!("  - thread_spawning");
}
