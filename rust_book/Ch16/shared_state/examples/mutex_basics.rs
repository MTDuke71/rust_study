// Example: Mutex Basics
// Understanding Mutex for mutual exclusion

use std::sync::Mutex;
use std::thread;

fn main() {
    println!("=== Mutex Basics ===\n");

    // Example 1: Single-threaded mutex usage
    println!("Example 1: Basic mutex in single thread");
    let m = Mutex::new(5);

    println!("Initial value: {:?}", m);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("Modified to: {}", *num);
    } // lock is automatically released here

    println!("After modification: {:?}\n", m);

    // Example 2: Demonstrating lock scope
    println!("Example 2: Lock scope and automatic release");
    let counter = Mutex::new(0);

    // Lock is held only within this block
    {
        let mut count = counter.lock().unwrap();
        *count += 1;
        println!("Count incremented to: {}", *count);
        // Lock released at end of scope
    }

    // Can acquire lock again
    let final_count = counter.lock().unwrap();
    println!("Final count: {}\n", *final_count);

    // Example 3: Multiple threads (without Arc - for demonstration)
    println!("Example 3: Attempting to share Mutex (will fail without Arc)");
    println!("(See arc_mutex.rs example for correct multi-threaded usage)\n");

    // Example 4: Handling poisoned mutex
    println!("Example 4: Poisoned mutex handling");
    let data = Mutex::new(vec![1, 2, 3]);

    let handle = thread::spawn(move || {
        let mut v = data.lock().unwrap();
        v.push(4);

        // Simulate a panic while holding the lock
        if v.len() > 3 {
            panic!("Thread panicked!");
        }
    });

    // This will show the thread panicked
    let result = handle.join();
    println!("Thread result: {:?}", result);
    println!("(In real code, other threads would see poisoned mutex)");
}
