// Example: Arc<Mutex<T>> Pattern
// Safe shared mutable state across threads

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== Arc<Mutex<T>> Pattern ===\n");

    // Example 1: Shared counter
    println!("Example 1: Shared counter across threads");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}\n", *counter.lock().unwrap());

    // Example 2: Shared data structure
    println!("Example 2: Shared Vec with concurrent modifications");
    let shared_vec = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let shared_vec = Arc::clone(&shared_vec);
        let handle = thread::spawn(move || {
            let mut vec = shared_vec.lock().unwrap();
            vec.push(i);
            println!("Thread {} pushed value", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_vec = shared_vec.lock().unwrap();
    println!("Final vector: {:?}\n", *final_vec);

    // Example 3: Shared hashmap (cache pattern)
    println!("Example 3: Shared HashMap as cache");
    use std::collections::HashMap;

    let cache = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    // Multiple threads adding to cache
    for i in 0..3 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let mut cache = cache.lock().unwrap();
            cache.insert(format!("key_{}", i), i * 10);
            println!("Thread {} added to cache", i);
        });
        handles.push(handle);
    }

    // One thread reading from cache
    let cache_clone = Arc::clone(&cache);
    let reader = thread::spawn(move || {
        // Small delay to let other threads populate
        std::thread::sleep(std::time::Duration::from_millis(50));
        let cache = cache_clone.lock().unwrap();
        println!("Reader sees cache: {:?}", *cache);
    });

    for handle in handles {
        handle.join().unwrap();
    }
    reader.join().unwrap();

    println!("\nFinal cache state:");
    let cache = cache.lock().unwrap();
    for (key, value) in cache.iter() {
        println!("  {} -> {}", key, value);
    }
}
