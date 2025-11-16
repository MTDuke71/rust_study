// Chapter 16.3: Shared-State Concurrency
// Using Mutex<T> and Arc<T> for safe shared state

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Basic Mutex Usage ===\n");

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("Changed value to: {}", *num);
    } // Lock is dropped here

    println!("m = {:?}\n", m);

    println!("=== Sharing Mutex Between Threads with Arc ===\n");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    println!("\nShared state examples complete!");
    println!("Run examples with: cargo run --example <name>");
    println!("  - mutex_basics");
    println!("  - arc_mutex");
    println!("  - rwlock");
    println!("  - deadlock (demonstrates deadlock scenario)");
}
