// Chapter 16.4: Extensible Concurrency with Sync and Send
// Understanding thread safety guarantees

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Understanding Send and Sync Traits ===\n");

    // Arc<T> is both Send and Sync
    println!("Arc<T> is Send and Sync - can be used across threads:");
    let arc_value = Arc::new(42);
    let arc_clone = Arc::clone(&arc_value);

    let handle = thread::spawn(move || {
        println!("  Thread received: {}", arc_clone);
    });

    handle.join().unwrap();
    println!("  Main still has: {}\n", arc_value);

    // Rc<T> is neither Send nor Sync
    println!("Rc<T> is NOT Send or Sync - cannot cross thread boundaries:");
    let rc_value = Rc::new(42);
    println!("  Rc value in main thread: {}", rc_value);
    println!("  (Cannot move Rc to another thread - compile error if attempted)\n");

    // Demonstrating Send with regular types
    println!("Regular types like Vec<T> and String are Send:");
    let data = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        println!("  Thread received vec: {:?}", data);
        data.iter().sum::<i32>()
    });

    let sum = handle.join().unwrap();
    println!("  Sum computed in thread: {}\n", sum);

    // Demonstrating Sync with &T
    println!("Types that are Sync allow safe &T sharing:");
    let shared_data = vec![10, 20, 30];
    let shared_ref = &shared_data;

    // We can create references in different scopes (simulating thread sharing)
    let another_ref = shared_ref;
    println!("  Shared data: {:?}", another_ref);
    println!("  Original ref still valid: {:?}\n", shared_ref);

    println!("Key Points:");
    println!("  • Send: Safe to transfer ownership between threads");
    println!("  • Sync: Safe to share references (&T) between threads");
    println!("  • Arc<T> implements both Send and Sync");
    println!("  • Rc<T> implements neither (not thread-safe)");
    println!("  • Most types automatically implement Send and Sync");

    println!("\nRun examples with: cargo run --example <name>");
    println!("  - send_trait");
    println!("  - sync_trait");
    println!("  - custom_types");
}
