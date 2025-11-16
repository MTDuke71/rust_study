// Chapter 16.1: Using Threads to Run Code Simultaneously
// Basic thread spawning and join handles

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Basic Thread Spawning ===\n");

    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread continues running
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();

    println!("\n=== Using move to Transfer Ownership ===\n");

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
