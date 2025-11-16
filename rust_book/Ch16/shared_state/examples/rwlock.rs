// Example: RwLock for Read-Write Locking
// Better performance when reads are more common than writes

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== RwLock Example ===\n");

    // Shared data with RwLock
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // Spawn multiple reader threads
    println!("Spawning reader threads...");
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // Acquire read lock (multiple readers can hold simultaneously)
            let read_guard = data.read().unwrap();
            println!("Reader {} sees: {:?}", i, *read_guard);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    // Spawn a writer thread
    println!("Spawning writer thread...");
    let data_clone = Arc::clone(&data);
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50)); // Let readers go first
        
        // Acquire write lock (exclusive access)
        let mut write_guard = data_clone.write().unwrap();
        println!("Writer modifying data...");
        write_guard.push(6);
        write_guard.push(7);
        thread::sleep(Duration::from_millis(100));
        println!("Writer done");
    });
    handles.push(writer);

    // Spawn more readers after writer
    println!("Spawning post-write readers...");
    for i in 5..8 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(200)); // Wait for writer
            let read_guard = data.read().unwrap();
            println!("Reader {} (after write) sees: {:?}", i, *read_guard);
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nFinal data: {:?}", *data.read().unwrap());

    // Example: Demonstrating concurrent reads
    println!("\n=== Concurrent Reads Demo ===");
    let counter = Arc::new(RwLock::new(0));
    let start = std::time::Instant::now();

    let mut read_handles = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let value = counter.read().unwrap();
            thread::sleep(Duration::from_millis(100));
            println!("Concurrent reader {} finished", i);
            *value
        });
        read_handles.push(handle);
    }

    for handle in read_handles {
        handle.join().unwrap();
    }

    println!("All concurrent reads completed in {:?}", start.elapsed());
    println!("(With Mutex, this would take ~1 second; with RwLock it's concurrent)");
}
